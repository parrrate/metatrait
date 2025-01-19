use either::Either;

use crate::{Free, Impl, Trait};

pub struct IntoEither<L, Tr: ?Sized>(L, Tr);

impl<L, Tr: ?Sized + Trait> Trait for IntoEither<L, Tr> {
    type Assocaited = Tr;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = Either<L, Imp::Associated>;
    type Sample = Either<L, Tr::Sample>;
    type Common<'a>
        = Either<L, Tr::Common<'a>>
    where
        Self: 'a;

    fn union(x: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {
        match x {
            Either::Left(x) => x.into_either().map_right(Either::Left),
            Either::Right(x) => x.into_either().map_right(Either::Right),
        }
        .map_right(Trait::union)
    }

    fn common<'a>(x: impl 'a + Impl<Self>) -> Self::Common<'a>
    where
        Self: 'a,
    {
        x.into_either().map_right(Trait::common)
    }
}

impl<L, Tr: ?Sized + Trait, T: Impl<Tr>> Impl<IntoEither<L, Tr>> for Either<L, T> {
    type Associated = T;

    fn method<'out: 'tmp, 'tmp>(
        x: <IntoEither<L, Tr> as Trait>::In<'out, 'tmp, Self>,
    ) -> <IntoEither<L, Tr> as Trait>::Out<'out, Self>
    where
        Self: 'tmp,
    {
        x
    }
}

pub trait IntoEitherExt<L, Tr: ?Sized + Trait>: Impl<IntoEither<L, Tr>> {
    fn into_either(self) -> Either<L, impl Impl<Tr>> {
        Self::method(self)
    }
}

impl<L, Tr: ?Sized + Trait, T: Impl<IntoEither<L, Tr>>> IntoEitherExt<L, Tr> for T {}

impl<L, Tr: ?Sized + Free> Free for IntoEither<L, Tr> {
    type Free = Either<L, Tr::Free>;

    fn free(x: impl Impl<Self>) -> Self::Free {
        x.into_either().map_right(Free::free)
    }
}
