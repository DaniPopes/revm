/// Maps opcode names to their dispatch shape marker types.
///
/// When called with just an opcode name, expands to the marker type.
/// When called with `(OP, $($rest:tt)*)`, expands `opcode_dispatch_cb!(MARKER, $($rest)*)`.
macro_rules! opcode_shape {
    (STOP) => {
        NoArgs
    };
    (JUMPDEST) => {
        NoArgs
    };
    (INVALID) => {
        NoArgs
    };

    (ADD) => {
        S
    };
    (MUL) => {
        S
    };
    (SUB) => {
        S
    };
    (DIV) => {
        S
    };
    (SDIV) => {
        S
    };
    (MOD) => {
        S
    };
    (SMOD) => {
        S
    };
    (ADDMOD) => {
        S
    };
    (MULMOD) => {
        S
    };
    (SIGNEXTEND) => {
        S
    };
    (LT) => {
        S
    };
    (GT) => {
        S
    };
    (SLT) => {
        S
    };
    (SGT) => {
        S
    };
    (EQ) => {
        S
    };
    (ISZERO) => {
        S
    };
    (AND) => {
        S
    };
    (OR) => {
        S
    };
    (XOR) => {
        S
    };
    (NOT) => {
        S
    };
    (BYTE) => {
        S
    };
    (POP) => {
        S
    };
    (DUP1) => {
        S
    };
    (DUP2) => {
        S
    };
    (DUP3) => {
        S
    };
    (DUP4) => {
        S
    };
    (DUP5) => {
        S
    };
    (DUP6) => {
        S
    };
    (DUP7) => {
        S
    };
    (DUP8) => {
        S
    };
    (DUP9) => {
        S
    };
    (DUP10) => {
        S
    };
    (DUP11) => {
        S
    };
    (DUP12) => {
        S
    };
    (DUP13) => {
        S
    };
    (DUP14) => {
        S
    };
    (DUP15) => {
        S
    };
    (DUP16) => {
        S
    };
    (SWAP1) => {
        S
    };
    (SWAP2) => {
        S
    };
    (SWAP3) => {
        S
    };
    (SWAP4) => {
        S
    };
    (SWAP5) => {
        S
    };
    (SWAP6) => {
        S
    };
    (SWAP7) => {
        S
    };
    (SWAP8) => {
        S
    };
    (SWAP9) => {
        S
    };
    (SWAP10) => {
        S
    };
    (SWAP11) => {
        S
    };
    (SWAP12) => {
        S
    };
    (SWAP13) => {
        S
    };
    (SWAP14) => {
        S
    };
    (SWAP15) => {
        S
    };
    (SWAP16) => {
        S
    };

    (SHL) => {
        SRf
    };
    (SHR) => {
        SRf
    };
    (SAR) => {
        SRf
    };
    (CLZ) => {
        SRf
    };
    (PUSH0) => {
        SRf
    };

    (CODESIZE) => {
        SBr
    };
    (PC) => {
        SBr
    };

    (JUMP) => {
        SB
    };
    (JUMPI) => {
        SB
    };
    (PUSH1) => {
        SB
    };
    (PUSH2) => {
        SB
    };
    (PUSH3) => {
        SB
    };
    (PUSH4) => {
        SB
    };
    (PUSH5) => {
        SB
    };
    (PUSH6) => {
        SB
    };
    (PUSH7) => {
        SB
    };
    (PUSH8) => {
        SB
    };
    (PUSH9) => {
        SB
    };
    (PUSH10) => {
        SB
    };
    (PUSH11) => {
        SB
    };
    (PUSH12) => {
        SB
    };
    (PUSH13) => {
        SB
    };
    (PUSH14) => {
        SB
    };
    (PUSH15) => {
        SB
    };
    (PUSH16) => {
        SB
    };
    (PUSH17) => {
        SB
    };
    (PUSH18) => {
        SB
    };
    (PUSH19) => {
        SB
    };
    (PUSH20) => {
        SB
    };
    (PUSH21) => {
        SB
    };
    (PUSH22) => {
        SB
    };
    (PUSH23) => {
        SB
    };
    (PUSH24) => {
        SB
    };
    (PUSH25) => {
        SB
    };
    (PUSH26) => {
        SB
    };
    (PUSH27) => {
        SB
    };
    (PUSH28) => {
        SB
    };
    (PUSH29) => {
        SB
    };
    (PUSH30) => {
        SB
    };
    (PUSH31) => {
        SB
    };
    (PUSH32) => {
        SB
    };

    (DUPN) => {
        SBRf
    };
    (SWAPN) => {
        SBRf
    };
    (EXCHANGE) => {
        SBRf
    };

    (ADDRESS) => {
        SI
    };
    (CALLER) => {
        SI
    };
    (CALLVALUE) => {
        SI
    };
    (CALLDATASIZE) => {
        SI
    };

    (CALLDATALOAD) => {
        SIM
    };

    (GAS) => {
        SG
    };

    (MSIZE) => {
        SM
    };

    (RETURNDATASIZE) => {
        SRfRd
    };

    // Everything else: full interpreter + host access.
    ($op:ident) => {
        IH
    };
}

/// Dispatches an opcode by name, calling the function with the right interpreter fields.
macro_rules! opcode_dispatch {
    (STOP, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*() };
    (JUMPDEST, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*() };
    (INVALID, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*() };
    // Stack-only
    (ADD, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (MUL, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SUB, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DIV, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SDIV, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (MOD, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SMOD, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (ADDMOD, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (MULMOD, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SIGNEXTEND, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (LT, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (GT, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SLT, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SGT, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (EQ, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (ISZERO, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (AND, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (OR, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (XOR, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (NOT, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (BYTE, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (POP, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP1, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP2, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP3, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP4, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP5, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP6, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP7, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP8, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP9, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP10, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP11, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP12, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP13, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP14, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP15, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (DUP16, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP1, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP2, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP3, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP4, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP5, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP6, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP7, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP8, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP9, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP10, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP11, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP12, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP13, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP14, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP15, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    (SWAP16, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack) };
    // Stack + RuntimeFlag
    (SHL, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.runtime_flag) };
    (SHR, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.runtime_flag) };
    (SAR, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.runtime_flag) };
    (CLZ, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.runtime_flag) };
    (PUSH0, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.runtime_flag) };
    // Stack + &Bytecode
    (CODESIZE, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.bytecode) };
    (PC, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.bytecode) };
    // Stack + &mut Bytecode
    (JUMP, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (JUMPI, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH1, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH2, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH3, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH4, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH5, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH6, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH7, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH8, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH9, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH10, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH11, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH12, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH13, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH14, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH15, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH16, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH17, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH18, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH19, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH20, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH21, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH22, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH23, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH24, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH25, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH26, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH27, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH28, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH29, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH30, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH31, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    (PUSH32, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode) };
    // Stack + &mut Bytecode + RuntimeFlag
    (DUPN, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode, &$s.runtime_flag) };
    (SWAPN, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode, &$s.runtime_flag) };
    (EXCHANGE, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &mut $s.bytecode, &$s.runtime_flag) };
    // Stack + Input
    (ADDRESS, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.input) };
    (CALLER, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.input) };
    (CALLVALUE, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.input) };
    (CALLDATASIZE, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.input) };
    // Stack + Input + Memory
    (CALLDATALOAD, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.input, &$s.memory) };
    // Stack + Gas
    (GAS, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.gas) };
    // Stack + Memory
    (MSIZE, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.memory) };
    // Stack + RuntimeFlag + ReturnData
    (RETURNDATASIZE, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*(&mut $s.stack, &$s.runtime_flag, &$s.return_data) };
    // Everything else: full interpreter + host access
    ($op:ident, $s:ident, $h:ident, ($($f:tt)*)) => { $($f)*($s, $h) };
}

/// Macro that triggers `unreachable!` in debug builds but uses unchecked unreachable in release builds.
/// This provides better error messages during development while optimizing for performance in release.
#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! debug_unreachable {
    ($($t:tt)*) => {
        if cfg!(debug_assertions) {
            unreachable!($($t)*);
        } else {
            unsafe { core::hint::unreachable_unchecked() };
        }
    };
}

/// Macro for asserting assumptions in debug builds.
/// In debug builds, this will trigger unreachable code if the assumption is false.
/// In release builds, this serves as an optimization hint.
#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! assume {
    ($e:expr $(,)?) => {
        if !$e {
            debug_unreachable!(stringify!($e));
        }
    };

    ($e:expr, $($t:tt)+) => {
        if !$e {
            debug_unreachable!($($t)+);
        }
    };
}
