use sealed::Sealed;

pub trait ArrayLike<T>: Sealed {
    type Array<U>;

    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&self) -> &[T];
    fn map<F, U>(self, f: F) -> Self::Array<U>
    where
        F: FnMut(T) -> U;
}

mod sealed {
    pub trait Sealed {}

    impl<T, const N: usize> Sealed for [T; N] {}
}

impl<T, const N: usize> ArrayLike<T> for [T; N] {
    type Array<U> = [U; N];

    fn as_mut_slice(&self) -> &[T] {
        self.as_slice()
    }
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }

    fn map<F, U>(self, f: F) -> Self::Array<U>
    where
        F: FnMut(T) -> U,
    {
        self.map(f)
    }
}
