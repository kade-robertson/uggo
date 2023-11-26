pub(crate) trait Transposable {
    type Transposed;
    fn transpose(self) -> Self::Transposed;
}

impl<T, U> Transposable for Option<(T, U)> {
    type Transposed = (Option<T>, Option<U>);
    fn transpose(self) -> Self::Transposed {
        match self {
            Some((a, b)) => (Some(a), Some(b)),
            None => (None, None),
        }
    }
}
