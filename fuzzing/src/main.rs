use fuzzing::math;
fn main() {
    let case = &std::env::args().collect::<Vec<String>>()[1];
    loop {
        math::operation(case)
    }
}
