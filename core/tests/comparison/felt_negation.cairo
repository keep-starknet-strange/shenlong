fn main() -> (felt, felt, felt) {
    return (
        neg(0),
        neg(1),
        neg(-1)
    );
}

fn neg(a: felt) -> (felt) {
    -a
}
