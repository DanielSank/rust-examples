trait Monad<M,A> {
    fn flatmap<B,C: Monad<M,B>>(&self, f: fn(A) -> C) -> C;
}
