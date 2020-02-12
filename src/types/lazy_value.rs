use crate::traits::Lazy;

/// Structure that allows any strict value to accumulate thunks.
pub struct LazyValue<T> {
    val: T,
}

impl<T> LazyValue<T> {
    /// Produces a new `LazyValue` from any strict value.
    pub fn new(val: T) -> Self {
        LazyValue { val }
    }
}

impl<T> Lazy for LazyValue<T> {
    type Output = T;

    fn eval(self) -> Self::Output {
        self.val
    }
}

#[cfg(test)]
mod tests {
    use super::LazyValue;
    use crate::traits::Lazy;

    #[test]
    fn lazy_value_evals_correct_value() {
        let f = LazyValue::new(27);

        assert_eq!(f.eval(), 27);
    }
}
