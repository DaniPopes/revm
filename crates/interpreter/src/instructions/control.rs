use crate::{
    interpreter::Interpreter,
    interpreter_types::{
        InterpreterTypes as IT, Jumps, LoopControl, MemoryTr, RuntimeFlag, StackTr,
    },
    InstructionExecResult as Result, InstructionResult, InterpreterAction,
};
use context_interface::{cfg::GasParams, Host};
use primitives::{hints_util::cold_path, Bytes, U256};

/// Implements the JUMP instruction.
///
/// Unconditional jump to a valid destination.
pub fn jump(stack: &mut impl StackTr, bytecode: &mut impl Jumps) -> Result {
    popn!([target], stack);
    jump_inner(bytecode, target)
}

/// Implements the JUMPI instruction.
///
/// Conditional jump to a valid destination if condition is true.
pub fn jumpi(stack: &mut impl StackTr, bytecode: &mut impl Jumps) -> Result {
    popn!([target, cond], stack);
    if !cond.is_zero() {
        jump_inner(bytecode, target)?;
    }
    Ok(())
}

/// Internal helper function for jump operations.
///
/// Validates jump target and performs the actual jump.
#[inline(always)]
fn jump_inner(bytecode: &mut impl Jumps, target: U256) -> Result<(), InstructionResult> {
    let target = as_usize_saturated!(target);
    if !bytecode.is_valid_legacy_jump(target) {
        cold_path();
        return Err(InstructionResult::InvalidJump);
    }
    // SAFETY: `is_valid_jump` ensures that `dest` is in bounds.
    bytecode.absolute_jump(target);
    Ok(())
}

/// Implements the JUMPDEST instruction.
///
/// Marks a valid destination for jump operations.
pub fn jumpdest() -> Result {
    Ok(())
}

/// Implements the PC instruction.
///
/// Pushes the current program counter onto the stack.
pub fn pc(stack: &mut impl StackTr, bytecode: &impl Jumps) -> Result {
    // - 1 because we have already advanced the instruction pointer in `Interpreter::step`
    push!(stack, U256::from(bytecode.pc() - 1));
    Ok(())
}

/// Internal helper function for return operations.
///
/// Handles memory data retrieval and sets the return action.
#[inline]
fn return_inner(
    interpreter: &mut Interpreter<impl IT>,
    gas_params: &GasParams,
    instruction_result: InstructionResult,
) -> Result<(), InstructionResult> {
    popn!([offset, len], interpreter.stack);
    let len = as_usize_or_fail!(len);
    // Important: Offset must be ignored if len is zeros
    let mut output = Bytes::default();
    if len != 0 {
        let offset = as_usize_or_fail!(offset);
        interpreter.resize_memory(gas_params, offset, len)?;
        output = interpreter.memory.slice_len(offset, len).to_vec().into()
    }

    interpreter
        .bytecode
        .set_action(InterpreterAction::new_return(
            instruction_result,
            output,
            interpreter.gas,
        ));
    Err(instruction_result)
}

/// Implements the RETURN instruction.
///
/// Halts execution and returns data from memory.
pub fn ret<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    return_inner(interpreter, host.gas_params(), InstructionResult::Return)
}

/// EIP-140: REVERT instruction
pub fn revert<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    check!(interpreter.runtime_flag, BYZANTIUM);
    return_inner(interpreter, host.gas_params(), InstructionResult::Revert)
}

/// Stop opcode. This opcode halts the execution.
pub fn stop() -> Result {
    Err(InstructionResult::Stop)
}

/// Invalid opcode. This opcode halts the execution.
pub fn invalid() -> Result {
    Err(InstructionResult::InvalidFEOpcode)
}

/// Unknown opcode. This opcode halts the execution.
pub fn unknown() -> Result {
    Err(InstructionResult::OpcodeNotFound)
}
