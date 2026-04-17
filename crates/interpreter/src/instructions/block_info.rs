use crate::{
    interpreter::Interpreter,
    interpreter_types::{InterpreterTypes as IT, RuntimeFlag, StackTr},
    Host, InstructionExecResult as Result,
};
use primitives::hardfork::SpecId::*;

/// EIP-1344: ChainID opcode
pub fn chainid<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    check!(interpreter, ISTANBUL);
    push!(interpreter, host.chain_id());
    Ok(())
}

/// Implements the COINBASE instruction.
///
/// Pushes the current block's beneficiary address onto the stack.
pub fn coinbase<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    push!(interpreter, host.beneficiary().into_word().into());
    Ok(())
}

/// Implements the TIMESTAMP instruction.
///
/// Pushes the current block's timestamp onto the stack.
pub fn timestamp<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    push!(interpreter, host.timestamp());
    Ok(())
}

/// Implements the NUMBER instruction.
///
/// Pushes the current block number onto the stack.
pub fn block_number<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    push!(interpreter, host.block_number());
    Ok(())
}

/// Implements the DIFFICULTY/PREVRANDAO instruction.
///
/// Pushes the block difficulty (pre-merge) or prevrandao (post-merge) onto the stack.
pub fn difficulty<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    if interpreter.runtime_flag.spec_id().is_enabled_in(MERGE) {
        // Unwrap is safe as this fields is checked in validation handler.
        push!(interpreter, host.prevrandao().unwrap());
    } else {
        push!(interpreter, host.difficulty());
    }
    Ok(())
}

/// Implements the GASLIMIT instruction.
///
/// Pushes the current block's gas limit onto the stack.
pub fn gaslimit<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    push!(interpreter, host.gas_limit());
    Ok(())
}

/// EIP-3198: BASEFEE opcode
pub fn basefee<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    check!(interpreter, LONDON);
    push!(interpreter, host.basefee());
    Ok(())
}

/// EIP-7516: BLOBBASEFEE opcode
pub fn blob_basefee<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    check!(interpreter, CANCUN);
    push!(interpreter, host.blob_gasprice());
    Ok(())
}

/// EIP-7843: SLOTNUM opcode
pub fn slot_num<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    check!(interpreter, AMSTERDAM);
    push!(interpreter, host.slot_num());
    Ok(())
}
