mod call_helpers;

pub use call_helpers::{
    get_memory_input_and_out_ranges, load_acc_and_calc_gas, load_account_delegated,
    load_account_delegated_handle_error, resize_memory,
};

use crate::{
    instructions::utility::IntoAddress,
    interpreter::Interpreter,
    interpreter_action::FrameInput,
    interpreter_types::{
        InputsTr, InterpreterTypes as IT, LoopControl, MemoryTr, RuntimeFlag, StackTr,
    },
    CallInput, CallInputs, CallScheme, CallValue, CreateInputs, Host,
    InstructionExecResult as Result, InstructionResult, InterpreterAction,
};
use context_interface::CreateScheme;
use primitives::{hardfork::SpecId, Address, Bytes, B256, U256};
use std::boxed::Box;

/// Implements the CREATE/CREATE2 instruction.
///
/// Creates a new contract with provided bytecode.
pub fn create<const IS_CREATE2: bool, I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    // Static call check is before gas charging (unlike execution-specs where it's
    // inside generic_create). This is safe because CREATE in a static context is
    // always an error regardless of gas accounting.
    require_non_staticcall!(interpreter.runtime_flag);

    // EIP-1014: Skinny CREATE2
    if IS_CREATE2 {
        check!(interpreter.runtime_flag, PETERSBURG);
    }

    popn!([value, code_offset, len], interpreter.stack);
    let len = as_usize_or_fail!(len);

    let mut code = Bytes::new();
    if len != 0 {
        // EIP-3860: Limit and meter initcode
        if interpreter
            .runtime_flag
            .spec_id()
            .is_enabled_in(SpecId::SHANGHAI)
        {
            // Limit is set as double of max contract bytecode size
            if len > host.max_initcode_size() {
                return Err(InstructionResult::CreateInitCodeSizeLimit);
            }
            gas!(interpreter.gas, host.gas_params().initcode_cost(len));
        }

        let code_offset = as_usize_or_fail!(code_offset);
        interpreter.resize_memory(host.gas_params(), code_offset, len)?;

        code = Bytes::copy_from_slice(interpreter.memory.slice_len(code_offset, len).as_ref());
    }

    // EIP-1014: Skinny CREATE2
    let scheme = if IS_CREATE2 {
        popn!([salt], interpreter.stack);
        // SAFETY: `len` is reasonable in size as gas for it is already deducted.
        gas!(interpreter.gas, host.gas_params().create2_cost(len));
        CreateScheme::Create2 { salt }
    } else {
        gas!(interpreter.gas, host.gas_params().create_cost());
        CreateScheme::Create
    };

    // State gas for account creation + contract metadata (EIP-8037)
    if host.is_amsterdam_eip8037_enabled() {
        state_gas!(interpreter.gas, host.gas_params().create_state_gas());
    }

    let mut gas_limit = interpreter.gas.remaining();

    // EIP-150: Gas cost changes for IO-heavy operations
    if interpreter
        .runtime_flag
        .spec_id()
        .is_enabled_in(SpecId::TANGERINE)
    {
        // Take remaining gas and deduce l64 part of it.
        gas_limit = host.gas_params().call_stipend_reduction(gas_limit);
    }
    gas!(interpreter.gas, gas_limit);

    // Call host to interact with target contract
    let create_inputs = CreateInputs::new(
        interpreter.input.target_address(),
        scheme,
        value,
        code,
        gas_limit,
        interpreter.gas.reservoir(),
    );
    interpreter
        .bytecode
        .set_action(InterpreterAction::NewFrame(FrameInput::Create(Box::new(
            create_inputs,
        ))));
    Err(InstructionResult::Suspend)
}

/// Implements the CALL instruction.
///
/// Message call with value transfer to another account.
pub fn call<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    popn!([local_gas_limit, to, value], interpreter.stack);
    let to = to.into_address();
    // Max gas limit is not possible in real ethereum situation.
    let local_gas_limit = u64::try_from(local_gas_limit).unwrap_or(u64::MAX);
    let has_transfer = !value.is_zero();

    if interpreter.runtime_flag.is_static() && has_transfer {
        return Err(InstructionResult::CallNotAllowedInsideStatic);
    }

    let (input, return_memory_offset) =
        get_memory_input_and_out_ranges(interpreter, host.gas_params())?;

    let (gas_limit, bytecode, bytecode_hash) =
        load_acc_and_calc_gas(interpreter, host, to, has_transfer, true, local_gas_limit)?;

    // Call host to interact with target contract
    interpreter
        .bytecode
        .set_action(InterpreterAction::NewFrame(FrameInput::Call(Box::new(
            CallInputs {
                input: CallInput::SharedBuffer(input),
                gas_limit,
                target_address: to,
                caller: interpreter.input.target_address(),
                bytecode_address: to,
                known_bytecode: (bytecode_hash, bytecode),
                value: CallValue::Transfer(value),
                scheme: CallScheme::Call,
                is_static: interpreter.runtime_flag.is_static(),
                return_memory_offset,
                reservoir: interpreter.gas.reservoir(),
            },
        ))));
    Err(InstructionResult::Suspend)
}

/// Implements the CALLCODE instruction.
///
/// Message call with alternative account's code.
pub fn call_code<I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    popn!([local_gas_limit, to, value], interpreter.stack);
    let to = Address::from_word(B256::from(to));
    // Max gas limit is not possible in real ethereum situation.
    let local_gas_limit = u64::try_from(local_gas_limit).unwrap_or(u64::MAX);
    let has_transfer = !value.is_zero();

    let (input, return_memory_offset) =
        get_memory_input_and_out_ranges(interpreter, host.gas_params())?;

    let (gas_limit, bytecode, bytecode_hash) =
        load_acc_and_calc_gas(interpreter, host, to, has_transfer, false, local_gas_limit)?;

    // Call host to interact with target contract
    interpreter
        .bytecode
        .set_action(InterpreterAction::NewFrame(FrameInput::Call(Box::new(
            CallInputs {
                input: CallInput::SharedBuffer(input),
                gas_limit,
                target_address: interpreter.input.target_address(),
                caller: interpreter.input.target_address(),
                bytecode_address: to,
                known_bytecode: (bytecode_hash, bytecode),
                value: CallValue::Transfer(value),
                scheme: CallScheme::CallCode,
                is_static: interpreter.runtime_flag.is_static(),
                return_memory_offset,
                reservoir: interpreter.gas.reservoir(),
            },
        ))));
    Err(InstructionResult::Suspend)
}

/// Implements the DELEGATECALL instruction.
///
/// Message call with alternative account's code but same sender and value.
pub fn delegate_call<I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    check!(interpreter.runtime_flag, HOMESTEAD);
    popn!([local_gas_limit, to], interpreter.stack);
    let to = Address::from_word(B256::from(to));
    // Max gas limit is not possible in real ethereum situation.
    let local_gas_limit = u64::try_from(local_gas_limit).unwrap_or(u64::MAX);

    let (input, return_memory_offset) =
        get_memory_input_and_out_ranges(interpreter, host.gas_params())?;

    let (gas_limit, bytecode, bytecode_hash) =
        load_acc_and_calc_gas(interpreter, host, to, false, false, local_gas_limit)?;

    // Call host to interact with target contract
    interpreter
        .bytecode
        .set_action(InterpreterAction::NewFrame(FrameInput::Call(Box::new(
            CallInputs {
                input: CallInput::SharedBuffer(input),
                gas_limit,
                target_address: interpreter.input.target_address(),
                caller: interpreter.input.caller_address(),
                bytecode_address: to,
                known_bytecode: (bytecode_hash, bytecode),
                value: CallValue::Apparent(interpreter.input.call_value()),
                scheme: CallScheme::DelegateCall,
                is_static: interpreter.runtime_flag.is_static(),
                return_memory_offset,
                reservoir: interpreter.gas.reservoir(),
            },
        ))));
    Err(InstructionResult::Suspend)
}

/// Implements the STATICCALL instruction.
///
/// Static message call (cannot modify state).
pub fn static_call<I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    check!(interpreter.runtime_flag, BYZANTIUM);
    popn!([local_gas_limit, to], interpreter.stack);
    let to = Address::from_word(B256::from(to));
    // Max gas limit is not possible in real ethereum situation.
    let local_gas_limit = u64::try_from(local_gas_limit).unwrap_or(u64::MAX);

    let (input, return_memory_offset) =
        get_memory_input_and_out_ranges(interpreter, host.gas_params())?;

    let (gas_limit, bytecode, bytecode_hash) =
        load_acc_and_calc_gas(interpreter, host, to, false, false, local_gas_limit)?;

    // Call host to interact with target contract
    interpreter
        .bytecode
        .set_action(InterpreterAction::NewFrame(FrameInput::Call(Box::new(
            CallInputs {
                input: CallInput::SharedBuffer(input),
                gas_limit,
                target_address: to,
                caller: interpreter.input.target_address(),
                bytecode_address: to,
                known_bytecode: (bytecode_hash, bytecode),
                value: CallValue::Transfer(U256::ZERO),
                scheme: CallScheme::StaticCall,
                is_static: true,
                return_memory_offset,
                reservoir: interpreter.gas.reservoir(),
            },
        ))));
    Err(InstructionResult::Suspend)
}
