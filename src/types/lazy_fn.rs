use crate::traits::Lazy;

/// Structure for holding functions that map from the unit type
/// `()` to some value.
///
/// For example `|| 64` is a valid `LazyFn`, but `|x| x + 5` should
/// be added as a thunk with the `op` function defined by the `Lazy`
/// trait.
pub struct LazyFn<F> {
    f: F,
}

impl<F> LazyFn<F> {
    /// Creates a new `lazyFn` given a function `f: F` satisfying the
    /// trait `F: FnOnce() -> B`.
    pub fn new(f: F) -> Self {
        LazyFn { f }
    }
}

impl<B, F> Lazy for LazyFn<F>
where
    F: FnOnce() -> B,
{
    type Output = B;

    fn eval(self) -> Self::Output {
        (self.f)()
    }
}

#[cfg(test)]
mod tests {
    use super::LazyFn;
    use crate::traits::Lazy;

    #[test]
    fn lazy_fn_evals_correct_value() {
        let f = LazyFn::new(|| 27);

        assert_eq!(f.eval(), 27);
    }
}
