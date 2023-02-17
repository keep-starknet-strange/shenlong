// Calculates fib...
fn fib(a: felt, b: felt, n: felt) -> felt {
    match n {
        0 => a,
        _ => fib(b, a + b, n - 1),
    }
}

fn main(a:felt) -> felt {
    fib(1, 2, 150)
}