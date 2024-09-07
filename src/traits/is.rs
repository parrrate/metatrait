use either::Either;

use crate::{Free, Impl, Trait};

use super::empty::Empty;

pub struct Is<That>(That);

impl<That> Trait for Is<That> {
    type Assocaited = Empty;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = That;
    type Sample = That;
    type Common<'a> = That where Self: 'a;

    fn union(x: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {
        match x {
            Either::Left(x) => x.into_that(),
            Either::Right(x) => x.into_that(),
        }
    }

    fn common<'a>(x: impl 'a + Impl<Self>) -> Self::Common<'a>
    where
        Self: 'a,
    {
        x.into_that()
    }
}

impl<That> Impl<Is<Self>> for That {
    type Associated = Self;

    fn method<'out: 'tmp, 'tmp>(this: Self) -> Self
    where
        Self: 'tmp,
    {
        this
    }
}

pub trait IsExt {
    fn into_that<That>(self) -> That
    where
        Self: Impl<Is<That>>,
    {
        Self::method(self)
    }
}

impl<That> IsExt for That {}

impl<That> Free for Is<That> {
    type Free = That;

    fn free(x: impl Impl<Self>) -> Self::Free {
        x.into_that()
    }
}
