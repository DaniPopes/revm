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
pub fn mload<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    popn_top!([], top, interpreter);
    let offset = as_usize_or_fail!(interpreter, top);
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
pub fn mstore<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    popn!([offset, value], interpreter);
    let offset = as_usize_or_fail!(interpreter, offset);
    interpreter.resize_memory(host.gas_params(), offset, 32)?;
    interpreter.memory.set(offset, &value.to_be_bytes::<32>());
    Ok(())
}

/// Implements the MSTORE8 instruction.
///
/// Stores a single byte to memory.
pub fn mstore8<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    popn!([offset, value], interpreter);
    let offset = as_usize_or_fail!(interpreter, offset);
    interpreter.resize_memory(host.gas_params(), offset, 1)?;
    interpreter.memory.set(offset, &[value.byte(0)]);
    Ok(())
}

/// Implements the MSIZE instruction.
///
/// Gets the size of active memory in bytes.
pub fn msize<WIRE: IT>(interpreter: &mut Interpreter<WIRE>) -> Result {
    push!(interpreter, U256::from(interpreter.memory.size()));
    Ok(())
}

/// Implements the MCOPY instruction.
///
/// EIP-5656: Memory copying instruction that copies memory from one location to another.
pub fn mcopy<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    check!(interpreter, CANCUN);
    popn!([dst, src, len], interpreter);

    // Into usize or fail
    let len = as_usize_or_fail!(interpreter, len);
    // Deduce gas
    gas!(interpreter, host.gas_params().mcopy_cost(len));

    if len == 0 {
        return Ok(());
    }

    let dst = as_usize_or_fail!(interpreter, dst);
    let src = as_usize_or_fail!(interpreter, src);
    // Resize memory
    interpreter.resize_memory(host.gas_params(), max(dst, src), len)?;
    // Copy memory in place
    interpreter.memory.copy(dst, src, len);
    Ok(())
}
