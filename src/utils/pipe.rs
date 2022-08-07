pub trait PipeExt: Sized {
    fn pipe<F, B>(self, f: F) -> B
    where
        F: FnOnce(Self) -> B,
    {
        f(self)
    }
}

impl<A> PipeExt for A {}
