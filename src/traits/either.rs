use either::Either;

use crate::{Impl, Trait};

pub struct ToEither<L, Tr: ?Sized>(L, Tr);

impl<L, Tr: ?Sized + Trait> Trait for ToEither<L, Tr> {
    type Assocaited = Tr;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = Either<L, Imp::Associated>;
    type Sample = Either<L, Tr::Sample>;
    type Common<'a> = Either<L, Tr::Common<'a>>
    where
        Self: 'a;

    fn union(x: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {
        match x {
            Either::Left(x) => x.to_either().map_right(Either::Left),
            Either::Right(x) => x.to_either().map_right(Either::Right),
        }
        .map_right(Trait::union)
    }

    fn common<'a>(x: impl 'a + Impl<Self>) -> Self::Common<'a>
    where
        Self: 'a,
    {
        x.to_either().map_right(Trait::common)
    }
}

impl<L, Tr: ?Sized + Trait, T: Impl<Tr>> Impl<ToEither<L, Tr>> for Either<L, T> {
    type Associated = T;

    fn method<'out: 'tmp, 'tmp>(
        x: <ToEither<L, Tr> as Trait>::In<'out, 'tmp, Self>,
    ) -> <ToEither<L, Tr> as Trait>::Out<'out, Self>
    where
        Self: 'tmp,
    {
        x
    }
}

pub trait ToEitherExt<L, Tr: ?Sized + Trait>: Impl<ToEither<L, Tr>> {
    fn to_either(self) -> Either<L, impl Impl<Tr>> {
        Self::method(self)
    }
}

impl<L, Tr: ?Sized + Trait, T: Impl<ToEither<L, Tr>>> ToEitherExt<L, Tr> for T {}
