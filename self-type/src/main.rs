trait Comparable {
    fn compare(self: &Self, other: &Self) -> bool;
}

struct C {}

impl Comparable for C {
    fn compare(&self, other: &C) -> bool {true}
}

fn main() {
    let c = C {};
    let cc = C {};
    println!("Comparison: {:?}", c.compare(&cc));
}
