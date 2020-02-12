use crate::traits::Lazy;

/// The `Thunk` structure contains some lazily evaluated expression
/// `thunk` and a function `f` to apply on value produced by it.
pub struct Thunk<T, F> {
    thunk: T,
    f: F,
}

impl<T, F> Thunk<T, F> {
    /// Generates a new thunk given a lazy value `thunk`
    /// and a function `f` to apply to it.
    pub fn new(thunk: T, f: F) -> Thunk<T, F> {
        Thunk { thunk, f }
    }
}

impl<B, T: Lazy, F> Lazy for Thunk<T, F>
where
    F: FnOnce(T::Output) -> B,
{
    type Output = B;

    fn eval(self) -> Self::Output {
        // Evaluate the thunk, then apply the function.
        (self.f)(self.thunk.eval())
    }
}
