use either::Either;

use crate::{Impl, Trait};

impl Trait for () {
    type Assocaited = Self;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = ();
    type Sample = ();
    type Common<'a> = ();

    fn union(_: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {}
    fn common<'a>(_: impl 'a + Impl<Self>) -> Self::Common<'a> {}
}

impl Impl<()> for () {
    type Associated = Self;

    fn method<'out: 'tmp, 'tmp>(_: ())
    where
        Self: 'tmp,
    {
    }
}
