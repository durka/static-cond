extern crate static_cond;
use static_cond::static_cond_item;

// Macro will expand to an item
static_cond_item! {
    if (+ 1 [2 3]) == (+ 1 [2 3]) {
        static_cond_item!{
            if black != white {
                // The item to output
                fn foo() -> &'static str {
                    "ok"
                }
            } else {
                the compiler will never even try to interpret this
            }
        }
    } else {
        blah blah blah blah blah unreachable
    }
}

fn main() {
    foo();
}
