extern crate static_cond;
use static_cond::static_cond;

fn main() {
    let x = {
        // Macro will expand to an expression
        static_cond!(if (+ 1 [2 3]) == (+ 1 [2 3]) {
            static_cond!(if black != white {
                "ok"
            } else {
                foobar
            })
        } else {
            blah
        })
    };
    assert_eq!(x, "ok");
}
