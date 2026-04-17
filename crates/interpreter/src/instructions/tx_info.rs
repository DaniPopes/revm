use crate::{
    interpreter::Interpreter,
    interpreter_types::{InterpreterTypes as IT, RuntimeFlag, StackTr},
    Host, InstructionExecResult as Result,
};

/// Implements the GASPRICE instruction.
///
/// Gets the gas price of the originating transaction.
pub fn gasprice<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    push!(interpreter, host.effective_gas_price());
    Ok(())
}

/// Implements the ORIGIN instruction.
///
/// Gets the execution origination address.
pub fn origin<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    push!(interpreter, host.caller().into_word().into());
    Ok(())
}

/// Implements the BLOBHASH instruction.
///
/// EIP-4844: Shard Blob Transactions - gets the hash of a transaction blob.
pub fn blob_hash<WIRE: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<WIRE>,
    host: &mut H,
) -> Result {
    check!(interpreter, CANCUN);
    popn_top!([], index, interpreter);
    let i = as_usize_saturated!(*index);
    *index = host.blob_hash(i).unwrap_or_default();
    Ok(())
}
