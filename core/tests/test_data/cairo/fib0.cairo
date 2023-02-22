func fib(a: felt, b: felt, n: felt) -> (felt, felt) {
    if (n == 0) {
        return (a, 0);
    } else {
        let (v, count) = fib(b, a + b, n - 1);
        return (v, count + 1);
    }
}
func fib_mid(n: felt) -> () {
    if (n == 0) {
        return ();
    } else {
        fib(0, 1, 500);
        return fib_mid(n - 1);
    }
}
func main() {
    fib_mid(100);
    return ();
}
