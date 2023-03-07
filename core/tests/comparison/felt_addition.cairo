fn main() -> (felt, felt, felt, felt, felt, felt, felt, felt, felt, felt) {
    return (
        add(0,1),
        add(1,0),
        add(20,42),
        add(2903857290345, 209384570293845702938457),
        add(-1, 2),
        add(2, -1),
        add(-3, 3),
        add(3, -3),
        add(10,-21),
        add(0,-100)
    );
}

fn add(a: felt, b: felt) -> felt {
    a + b
}