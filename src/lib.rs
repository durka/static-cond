#![no_std]

/// Evaluates a conditional during macro expansion as expression.
///
/// Currently limited to equality comparison. Can compare any two token trees. Can be nested.
///
/// This macro only works in expression-position, see [static_cond_item] for
/// a macro that works in item-position.
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
    // we evaluate a conditional by generating a new macro (in an inner scope, so name shadowing is
    // not a big concern) and calling it
    (@expr $lhs:tt $rhs:tt $arm1:tt $arm2:tt) => {{
        // note that the inner macro has no captures (it can't, because there's no way to escape `$`)
        macro_rules! __static_cond {
            ($lhs $lhs) => $arm1;
            ($lhs $rhs) => $arm2;
        }

        __static_cond!($lhs $rhs)
    }};
    (@item $lhs:tt $rhs:tt $arm1:tt $arm2:tt) => {
        // note that the inner macro has no captures (it can't, because there's no way to escape `$`)
        macro_rules! __static_cond {
            ($lhs $lhs) => $arm1;
            ($lhs $rhs) => $arm2;
        }

        __static_cond! { $lhs $rhs }
    };

    // no else condition provided: fall through with empty else
    (if $lhs:tt == $rhs:tt $then:tt) => {
        $crate::static_cond!(if $lhs == $rhs $then else { })
    };
    (if $lhs:tt != $rhs:tt $then:tt) => {
        $crate::static_cond!(if $lhs != $rhs $then else { })
    };

    // main entry point with then and else arms
    (if $lhs:tt == $rhs:tt $then:tt else $els:tt) => {
        $crate::static_cond!(@expr $lhs $rhs $then $els)
	};
    (if $lhs:tt != $rhs:tt $then:tt else $els:tt) => {
        $crate::static_cond!(@expr $lhs $rhs $els $then)
    };
}

/// Evaluates a conditional during macro expansion as item.
///
/// Currently limited to equality comparison. Can compare any two token trees. Can be nested.
///
/// This macro only works in item-position, see [static_cond] for
/// a macro that works in expression-position.
///
/// Examples
/// =======
///
/// ```
/// # #[macro_use] extern crate static_cond;
/// # fn main() {
/// static_cond_item!{
///     if (+ 1 [2 3]) == (+ 1 [2 3]) {
///         static_cond_item!{if black != white {
///             fn foo() -> &'static str {"ok"}
///         } else {
///             the compiler will never even try to interpret this
///         }}
///     } else {
///         blah blah blah blah blah unreachable
///     }
/// }
/// assert_eq!(foo(), "ok");
/// # }
/// ```
///
/// The actual conditional and the code provided for the branches not followed is eliminated after
/// macro expansion (check `rustc --pretty=expanded`).
#[macro_export]
macro_rules! static_cond_item {
    // no else condition provided: fall through with empty else
    (if $lhs:tt == $rhs:tt $then:tt) => {
        $crate::static_cond_item! { if $lhs == $rhs $then else { } }
    };
    (if $lhs:tt != $rhs:tt $then:tt) => {
        $crate::static_cond_item! { if $lhs != $rhs $then else { } }
    };

    // main entry point with then and else arms
    (if $lhs:tt == $rhs:tt $then:tt else $els:tt) => {
        $crate::static_cond! { @item $lhs $rhs $then $els }
    };
    (if $lhs:tt != $rhs:tt $then:tt else $els:tt) => {
        $crate::static_cond! { @item $lhs $rhs $els $then }
    };
}
