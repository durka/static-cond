#![no_std]

/// Evaluates a conditional during macro expansion.
/// 
/// Currently limited to equality comparison. Can compare any two token trees. Can be nested.
///
/// This currently is suitable for use in expression context only (because it creates a {} scope).
/// That could be changed, but then it wouldn't be usable in expression context.
///
/// Examples
/// =======
///
/// ```
/// # #[macro_use] extern crate static_cond;
/// # fn main() {
/// let x = static_cond!(if (+ 1 [2 3]) == (+ 1 [2 3]) {
///     static_cond!(if black != white {
///         "ok"
///     } else {
///         the compiler will never even try to interpret this
///     })
/// } else {
///     blah blah blah blah blah unreachable
/// });
/// assert_eq!(x, "ok");
/// # }
/// ```
///
/// The actual conditional and the code provided for the branches not followed is eliminated after
/// macro expansion (check `rustc --pretty=expanded`).
#[macro_export]
macro_rules! static_cond {
    // private rule to define and call the local macro
    (@go $lhs:tt $rhs:tt $arm1:tt $arm2:tt) => {{
        // note that the inner macro has no captures (it can't, because there's no way to escape `$`)
        macro_rules! __static_cond {
            ($lhs $lhs) => $arm1;
            ($lhs $rhs) => $arm2
        }
        
        __static_cond!($lhs $rhs)
    }};

    // no else condition provided: fall through with empty else
    (if $lhs:tt == $rhs:tt $then:tt) => {
        static_cond!(if $lhs == $rhs $then else { })
    };
    (if $lhs:tt != $rhs:tt $then:tt) => {
        static_cond!(if $lhs != $rhs $then else { })
    };
    
    // we evaluate a conditional by generating a new macro (in an inner scope, so name shadowing is
    // not a big concern) and calling it
    (if $lhs:tt == $rhs:tt $then:tt else $els:tt) => {
        static_cond!(@go $lhs $rhs $then $els)
    };
    (if $lhs:tt != $rhs:tt $then:tt else $els:tt) => {
        static_cond!(@go $lhs $rhs $els $then)
    };
}

