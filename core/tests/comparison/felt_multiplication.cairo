fn main() -> (felt, felt, felt, felt, felt, felt, felt, felt) {
    return (
        mul(0,1),
        mul(1,0),
        mul(20,42),
        mul(2903857290345, 209384570293845702938457),
        mul(-1, 2),
        mul(2, -1),
        mul(-3, 3),
        mul(3, -3)
    );
}

fn mul(a: felt, b: felt) -> felt {
    a * b
}