// Calculates fib...
fn fib(a: felt, b: felt, n: felt) -> felt {
    match n {
        0 => a,
        _ => fib(b, a + b, n - 1),
    }
}
fn empty(a: felt) {}
fn main(a:felt) -> felt {
    empty(1);
    fib(1, 2, 3)
}