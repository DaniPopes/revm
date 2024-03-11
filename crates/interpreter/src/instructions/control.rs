use crate::{
    gas,
    primitives::{Bytes, Spec, U256},
    InstructionResult, Interpreter, InterpreterResult,
};

pub fn jump(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::MID);
    pop!(interpreter, dest);
    jump_inner(interpreter, dest);
}

pub fn jumpi(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::HIGH);
    pop!(interpreter, dest, value);
    if value != U256::ZERO {
        jump_inner(interpreter, dest);
    }
}

#[inline(always)]
fn jump_inner(interpreter: &mut Interpreter, dest: U256) {
    let dest = as_usize_or_fail!(interpreter, dest, InstructionResult::InvalidJump);
    if !interpreter.contract.is_valid_jump(dest) {
        interpreter.instruction_result = InstructionResult::InvalidJump;
        return;
    }
    // SAFETY: `is_valid_jump` ensures that `dest` is in bounds.
    interpreter.instruction_pointer = unsafe { interpreter.contract.bytecode.as_ptr().add(dest) };
}

pub fn jumpdest(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::JUMPDEST);
}

pub fn pc(interpreter: &mut Interpreter) {
    gas!(interpreter, gas::BASE);
    // - 1 because we have already advanced the instruction pointer in `Interpreter::step`
    push!(interpreter, U256::from(interpreter.program_counter() - 1));
}

#[inline(always)]
fn return_inner(interpreter: &mut Interpreter, instruction_result: InstructionResult) {
    // zero gas cost
    // gas!(interpreter, gas::ZERO);
    pop!(interpreter, offset, len);
    let len = as_usize_or_fail!(interpreter, len);
    // important: offset must be ignored if len is zeros
    let mut output = Bytes::default();
    if len != 0 {
        let offset = as_usize_or_fail!(interpreter, offset);
        shared_memory_resize!(interpreter, offset, len);

        output = interpreter.shared_memory.slice(offset, len).to_vec().into()
    }
    interpreter.instruction_result = instruction_result;
    interpreter.next_action = crate::InterpreterAction::Return {
        result: InterpreterResult {
            output,
            gas: interpreter.gas,
            result: instruction_result,
        },
    };
}

pub fn ret(interpreter: &mut Interpreter) {
    return_inner(interpreter, InstructionResult::Return);
}

/// EIP-140: REVERT instruction
pub fn revert<SPEC: Spec>(interpreter: &mut Interpreter) {
    check!(interpreter, BYZANTIUM);
    return_inner(interpreter, InstructionResult::Revert);
}

/// Stop opcode. This opcode halts the execution.
pub fn stop(interpreter: &mut Interpreter) {
    interpreter.instruction_result = InstructionResult::Stop;
}

/// Invalid opcode. This opcode halts the execution.
pub fn invalid(interpreter: &mut Interpreter) {
    interpreter.instruction_result = InstructionResult::InvalidFEOpcode;
}

/// Unknown opcode. This opcode halts the execution.
pub fn unknown(interpreter: &mut Interpreter) {
    interpreter.instruction_result = InstructionResult::OpcodeNotFound;
}
