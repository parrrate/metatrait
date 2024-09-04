use crate::linked::{Impl, Trait};

pub enum Empty {}

impl Trait for Empty {
    type Assocaited = Self;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Self;
    type Out<'out, Imp: Impl<Self>> = Self;
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
