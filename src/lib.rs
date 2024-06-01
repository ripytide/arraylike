#![doc = include_str!("../README.md")]
use sealed::Sealed;

/// A sealed trait for generically using arrays without const expressions which are unstable.
pub trait ArrayLike<T>: Sealed {
    /// A generic associated type for referring to the same array type as `Self` but with a
    /// different item type.
    type Array<U>;

    /// Returns a slice containing the entire array.
    fn as_slice(&self) -> &[T];
    /// Returns a mutable slice containing the entire array.
    fn as_mut_slice(&self) -> &[T];
    /// Returns an array of the same size as self, with function `f` applied to each element in order.
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
