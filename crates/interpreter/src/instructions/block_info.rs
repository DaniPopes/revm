use crate::{
    interpreter::Interpreter,
    interpreter_types::{InterpreterTypes as IT, RuntimeFlag, StackTr},
    Host, InstructionExecResult as Result,
};
use primitives::hardfork::SpecId::*;

/// EIP-1344: ChainID opcode
pub fn chainid<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    check!(interpreter.runtime_flag, ISTANBUL);
    push!(interpreter.stack, host.chain_id());
    Ok(())
}

/// Implements the COINBASE instruction.
///
/// Pushes the current block's beneficiary address onto the stack.
pub fn coinbase<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    push!(interpreter.stack, host.beneficiary().into_word().into());
    Ok(())
}

/// Implements the TIMESTAMP instruction.
///
/// Pushes the current block's timestamp onto the stack.
pub fn timestamp<I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    push!(interpreter.stack, host.timestamp());
    Ok(())
}

/// Implements the NUMBER instruction.
///
/// Pushes the current block number onto the stack.
pub fn block_number<I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    push!(interpreter.stack, host.block_number());
    Ok(())
}

/// Implements the DIFFICULTY/PREVRANDAO instruction.
///
/// Pushes the block difficulty (pre-merge) or prevrandao (post-merge) onto the stack.
pub fn difficulty<I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    if interpreter.runtime_flag.spec_id().is_enabled_in(MERGE) {
        // Unwrap is safe as this fields is checked in validation handler.
        push!(interpreter.stack, host.prevrandao().unwrap());
    } else {
        push!(interpreter.stack, host.difficulty());
    }
    Ok(())
}

/// Implements the GASLIMIT instruction.
///
/// Pushes the current block's gas limit onto the stack.
pub fn gaslimit<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    push!(interpreter.stack, host.gas_limit());
    Ok(())
}

/// EIP-3198: BASEFEE opcode
pub fn basefee<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    check!(interpreter.runtime_flag, LONDON);
    push!(interpreter.stack, host.basefee());
    Ok(())
}

/// EIP-7516: BLOBBASEFEE opcode
pub fn blob_basefee<I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    check!(interpreter.runtime_flag, CANCUN);
    push!(interpreter.stack, host.blob_gasprice());
    Ok(())
}

/// EIP-7843: SLOTNUM opcode
pub fn slot_num<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    check!(interpreter.runtime_flag, AMSTERDAM);
    push!(interpreter.stack, host.slot_num());
    Ok(())
}
