use crate::{
    gas,
    primitives::{Spec, B256, KECCAK_EMPTY, U256},
    InstructionResult, Interpreter,
};

pub fn keccak256(interpreter: &mut Interpreter) {
    pop!(interpreter, from, len);
    let len = as_usize_or_fail!(interpreter, len);
    gas_or_fail!(interpreter, gas::keccak256_cost(len as u64));
    let hash = if len == 0 {
        KECCAK_EMPTY
    } else {
        let from = as_usize_or_fail!(interpreter, from);
        shared_memory_resize!(interpreter, from, len);
        crate::primitives::keccak256(interpreter.shared_memory.slice(from, len))
    };

    push_b256!(interpreter, hash);
}

pub fn address(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::BASE);
    push_b256!(interpreter, interpreter.contract.address.into_word());
}

pub fn caller(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::BASE);
    push_b256!(interpreter, interpreter.contract.caller.into_word());
}

pub fn codesize(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, U256::from(interpreter.contract.bytecode.len()));
}

pub fn codecopy(interpreter: &mut Interpreter) {
    pop!(interpreter, memory_offset, code_offset, len);
    let len = as_usize_or_fail!(interpreter, len);
    gas_or_fail!(interpreter, gas::verylowcopy_cost(len as u64));
    if len == 0 {
        return;
    }
    let memory_offset = as_usize_or_fail!(interpreter, memory_offset);
    let code_offset = as_usize_saturated!(code_offset);
    shared_memory_resize!(interpreter, memory_offset, len);

    // Note: this can't panic because we resized memory to fit.
    interpreter.shared_memory.set_data(
        memory_offset,
        code_offset,
        len,
        interpreter.contract.bytecode.original_bytecode_slice(),
    );
}

pub fn calldataload(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index);
    let index = as_usize_saturated!(index);
    let load = if index < interpreter.contract.input.len() {
        let have_bytes = 32.min(interpreter.contract.input.len() - index);
        let mut bytes = [0u8; 32];
        bytes[..have_bytes].copy_from_slice(&interpreter.contract.input[index..index + have_bytes]);
        B256::new(bytes)
    } else {
        B256::ZERO
    };

    push_b256!(interpreter, load);
}

pub fn calldatasize(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, U256::from(interpreter.contract.input.len()));
}

pub fn callvalue(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, interpreter.contract.value);
}

pub fn calldatacopy(interpreter: &mut Interpreter) {
    pop!(interpreter, memory_offset, data_offset, len);
    let len = as_usize_or_fail!(interpreter, len);
    gas_or_fail!(interpreter, gas::verylowcopy_cost(len as u64));
    if len == 0 {
        return;
    }
    let memory_offset = as_usize_or_fail!(interpreter, memory_offset);
    let data_offset = as_usize_saturated!(data_offset);
    shared_memory_resize!(interpreter, memory_offset, len);

    // Note: this can't panic because we resized memory to fit.
    interpreter.shared_memory.set_data(
        memory_offset,
        data_offset,
        len,
        &interpreter.contract.input,
    );
}

/// EIP-211: New opcodes: RETURNDATASIZE and RETURNDATACOPY
pub fn returndatasize<SPEC: Spec>(interpreter: &mut Interpreter) {
    check!(interpreter, BYZANTIUM);
    gas!(interpreter, gas::BASE);
    push!(
        interpreter,
        U256::from(interpreter.return_data_buffer.len())
    );
}

/// EIP-211: New opcodes: RETURNDATASIZE and RETURNDATACOPY
pub fn returndatacopy<SPEC: Spec>(interpreter: &mut Interpreter) {
    check!(interpreter, BYZANTIUM);
    pop!(interpreter, memory_offset, offset, len);
    let len = as_usize_or_fail!(interpreter, len);
    gas_or_fail!(interpreter, gas::verylowcopy_cost(len as u64));
    let data_offset = as_usize_saturated!(offset);
    let (data_end, overflow) = data_offset.overflowing_add(len);
    if overflow || data_end > interpreter.return_data_buffer.len() {
        interpreter.instruction_result = InstructionResult::OutOfOffset;
        return;
    }
    if len != 0 {
        let memory_offset = as_usize_or_fail!(interpreter, memory_offset);
        shared_memory_resize!(interpreter, memory_offset, len);
        interpreter.shared_memory.set(
            memory_offset,
            &interpreter.return_data_buffer[data_offset..data_end],
        );
    }
}

pub fn gas(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, U256::from(interpreter.gas.remaining()));
}
