use either::Either;

use crate::{Impl, Trait};

pub struct AsEither<L, Tr: ?Sized>(L, Tr);

impl<L, Tr: ?Sized + Trait> Trait for AsEither<L, Tr> {
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

impl<L, Tr: ?Sized + Trait, T: Impl<Tr>> Impl<AsEither<L, Tr>> for Either<L, T> {
    type Associated = T;

    fn method<'out: 'tmp, 'tmp>(
        x: <AsEither<L, Tr> as Trait>::In<'out, 'tmp, Self>,
    ) -> <AsEither<L, Tr> as Trait>::Out<'out, Self>
    where
        Self: 'tmp,
    {
        x
    }
}

pub trait ToEitherExt<L, Tr: ?Sized + Trait>: Impl<AsEither<L, Tr>> {
    fn to_either(self) -> Either<L, impl Impl<Tr>> {
        Self::method(self)
    }
}

impl<L, Tr: ?Sized + Trait, T: Impl<AsEither<L, Tr>>> ToEitherExt<L, Tr> for T {}
