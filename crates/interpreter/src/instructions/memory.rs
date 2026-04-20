use crate::interpreter::resize_memory;
use crate::interpreter::Interpreter;
use crate::interpreter_types::{InterpreterTypes as IT, MemoryTr, RuntimeFlag, StackTr};
use crate::InstructionExecResult as Result;
use context_interface::Host;
use core::cmp::max;
use primitives::U256;

/// Implements the MLOAD instruction.
///
/// Loads a 32-byte word from memory.
pub fn mload<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    popn_top!([], top, interpreter.stack);
    let offset = as_usize_or_fail!(top);
    resize_memory(
        &mut interpreter.gas,
        &mut interpreter.memory,
        host.gas_params(),
        offset,
        32,
    )?;
    *top = U256::try_from_be_slice(interpreter.memory.slice_len(offset, 32).as_ref()).unwrap();
    Ok(())
}

/// Implements the MSTORE instruction.
///
/// Stores a 32-byte word to memory.
pub fn mstore<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    popn!([offset, value], interpreter.stack);
    let offset = as_usize_or_fail!(offset);
    interpreter.resize_memory(host.gas_params(), offset, 32)?;
    interpreter.memory.set(offset, &value.to_be_bytes::<32>());
    Ok(())
}

/// Implements the MSTORE8 instruction.
///
/// Stores a single byte to memory.
pub fn mstore8<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    popn!([offset, value], interpreter.stack);
    let offset = as_usize_or_fail!(offset);
    interpreter.resize_memory(host.gas_params(), offset, 1)?;
    interpreter.memory.set(offset, &[value.byte(0)]);
    Ok(())
}

/// Implements the MSIZE instruction.
///
/// Gets the size of active memory in bytes.
pub fn msize(stack: &mut impl StackTr, memory: &impl MemoryTr) -> Result {
    push!(stack, U256::from(memory.size()));
    Ok(())
}

/// Implements the MCOPY instruction.
///
/// EIP-5656: Memory copying instruction that copies memory from one location to another.
pub fn mcopy<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    check!(interpreter.runtime_flag, CANCUN);
    popn!([dst, src, len], interpreter.stack);

    // Into usize or fail
    let len = as_usize_or_fail!(len);
    // Deduce gas
    gas!(interpreter.gas, host.gas_params().mcopy_cost(len));

    if len == 0 {
        return Ok(());
    }

    let dst = as_usize_or_fail!(dst);
    let src = as_usize_or_fail!(src);
    // Resize memory
    interpreter.resize_memory(host.gas_params(), max(dst, src), len)?;
    // Copy memory in place
    interpreter.memory.copy(dst, src, len);
    Ok(())
}
