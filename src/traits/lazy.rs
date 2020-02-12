use crate::types::Thunk;

pub trait Lazy {
    /// The type of the value produced when evalutated.
    type Output;

    /// Adds a new thunk given a function mapping the output type
    /// of the current expression to a new output type.
    fn op<NewOutput, F>(self, f: F) -> Thunk<Self, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> NewOutput,
    {
        Thunk::new(self, f)
    }

    /// Evaluates the lazy structure, returning the computed value.
    fn eval(self) -> Self::Output;
}
