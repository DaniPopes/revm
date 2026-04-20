use crate::{
    interpreter::Interpreter,
    interpreter_types::{InterpreterTypes as IT, RuntimeFlag, StackTr},
    Host, InstructionExecResult as Result,
};

/// Implements the GASPRICE instruction.
///
/// Gets the gas price of the originating transaction.
pub fn gasprice<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    push!(interpreter.stack, host.effective_gas_price());
    Ok(())
}

/// Implements the ORIGIN instruction.
///
/// Gets the execution origination address.
pub fn origin<I: IT, H: Host + ?Sized>(interpreter: &mut Interpreter<I>, host: &mut H) -> Result {
    push!(interpreter.stack, host.caller().into_word().into());
    Ok(())
}

/// Implements the BLOBHASH instruction.
///
/// EIP-4844: Shard Blob Transactions - gets the hash of a transaction blob.
pub fn blob_hash<I: IT, H: Host + ?Sized>(
    interpreter: &mut Interpreter<I>,
    host: &mut H,
) -> Result {
    check!(interpreter.runtime_flag, CANCUN);
    popn_top!([], index, interpreter.stack);
    let i = as_usize_saturated!(*index);
    *index = host.blob_hash(i).unwrap_or_default();
    Ok(())
}
