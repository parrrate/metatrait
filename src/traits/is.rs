use either::Either;

use crate::{Free, Impl, Trait};

use super::empty::Empty;

pub struct Is<That>(That);

impl<That> Trait for Is<That> {
    type Assocaited<Imp: Impl<Self>> = Empty;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = That;
    type Sample = That;
    type Common<'a>
        = That
    where
        Self: 'a;

    fn union(x: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {
        match x {
            Either::Left(x) => x.t_into(),
            Either::Right(x) => x.t_into(),
        }
    }

    fn common<'a>(x: impl 'a + Impl<Self>) -> Self::Common<'a>
    where
        Self: 'a,
    {
        x.t_into()
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

pub trait Into2 {
    fn t_into<That>(self) -> That
    where
        Self: Impl<Is<That>>,
    {
        Self::method(self)
    }
}

impl<That> Into2 for That {}

impl<That> Free for Is<That> {
    type Free = That;

    fn free(x: impl Impl<Self>) -> Self::Free {
        x.t_into()
    }
}
