use either::Either;

use crate::{Impl, Trait};

impl Trait for () {
    type Assocaited = Self;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = ();
    type Out<'out, Imp: Impl<Self>> = ();
    type Sample = ();

    fn union(_: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {}
}

impl Impl<()> for () {
    type Associated = Self;

    fn method<'out: 'tmp, 'tmp>(_: ())
    where
        Self: 'tmp,
    {
    }
}
