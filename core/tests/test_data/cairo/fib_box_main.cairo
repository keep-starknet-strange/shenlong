// Calculates fib, but all variables are boxes.
fn fib(a: Box::<felt>, b: Box::<felt>, n: Box::<felt>) -> Box::<felt> {
    let unboxed_n = unbox::<felt>(n);
    if unboxed_n == 0 {
        a
    } else {
        fib(
            b,
            into_box::<felt>(unbox::<felt>(a) + unbox::<felt>(b)),
            into_box::<felt>(unboxed_n - 1),
        )
    }
}

fn main() -> felt {
    let a = into_box::<felt>(0);
    let b = into_box::<felt>(1);
    let n = into_box::<felt>(30);
    unbox::<felt>(fib(a, b, n))
}