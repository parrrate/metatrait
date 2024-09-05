use either::Either;

use crate::{Impl, Trait};

pub struct Is<That, Tr: ?Sized>(That, Tr);

impl<That: Impl<Tr>, Tr: ?Sized + Trait> Trait for Is<That, Tr> {
    type Assocaited = Tr;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = That;
    type Sample = That;

    fn union(x: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {
        match x {
            Either::Left(x) => x.into_that(),
            Either::Right(x) => x.into_that(),
        }
    }
}

impl<That: Impl<Tr>, Tr: ?Sized + Trait> Impl<Is<Self, Tr>> for That {
    type Associated = Self;

    fn method<'out: 'tmp, 'tmp>(this: Self) -> Self
    where
        Self: 'tmp,
    {
        this
    }
}

pub trait IsExt<Tr: ?Sized + Trait> {
    fn into_that<That>(self) -> That
    where
        That: Impl<Tr>,
        Self: Impl<Is<That, Tr>>,
    {
        Self::method(self)
    }
}

impl<That, Tr: ?Sized + Trait> IsExt<Tr> for That {}
