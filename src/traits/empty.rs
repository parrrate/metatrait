use crate::{Free, Impl, Trait};

pub enum Empty {}

impl Trait for Empty {
    type Assocaited = Self;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Self;
    type Out<'out, Imp: Impl<Self>> = Self;
    type Sample = Empty;
    type Common<'a> = ();

    fn union(_: either::Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {}
    fn common<'a>(_: impl 'a + Impl<Self>) -> Self::Common<'a> {}
}

impl<I> Impl<Empty> for I {
    type Associated = Self;

    fn method<'out: 'tmp, 'tmp>(empty: Empty) -> Empty
    where
        Self: 'tmp,
    {
        empty
    }
}

impl Free for Empty {
    type Free = ();

    fn free(_: impl Impl<Self>) -> Self::Free {}
}
