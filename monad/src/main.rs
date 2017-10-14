trait Monad<A> {
    fn flatmap<B, C: Monad<B>>(&self, f: fn(A) -> C) -> C;
}


fn main() {
    println!("It compiled!");
}
