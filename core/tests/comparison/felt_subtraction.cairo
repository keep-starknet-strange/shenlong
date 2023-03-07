fn main() -> (felt, felt, felt, felt, felt, felt, felt, felt) {
    return (
        sub(0,1),
        sub(1,0),
        sub(20,42),
        sub(2903857290345, 209384570293845702938457),
        sub(-1, 2),
        sub(2, -1),
        sub(-3, 3),
        sub(3, -3)
    );
}

fn sub(a: felt, b: felt) -> felt {
    a - b
}