func fib(a: felt, b: felt, n: felt) -> (felt, felt) {
    if (n == 0) {
        return (a, 0);
    } else {
        let (v, count) = fib(b, a + b, n - 1);
        return (v, count + 1);
    }
}
func main() {
    fib(1, 2, 500);
    return ();
}
