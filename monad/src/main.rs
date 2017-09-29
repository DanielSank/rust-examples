trait Comparable<T: Comparable<T>> {
    fn compare(&self, other: T) -> bool;
}

// C implements comparable
struct C {}

impl Comparable<C> for C {
    fn compare(&self, other: C) -> bool {true}
}

/*
trait Monad<M,A> {
    fn flatmap<B,C: Monad<M,B>>(&self, f: fn(A) -> C) -> C;
}
*/

fn main() {
    let c = C {};
    let cc = C {};
    println!("Comparison: {:?}", c.compare(cc));
}
