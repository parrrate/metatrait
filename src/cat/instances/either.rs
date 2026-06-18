use either::Either;

use crate::{
    cat::{functor::*, morphism::MapFn},
    traits::either::{IntoEither, IntoEitherExt},
    Impl, Trait,
};

pub struct Eithers<L>(L);

impl<L: Send, Tr: ?Sized + Trait> Unwrap<IntoEither<L, Tr>> for Eithers<L> {
    type Tr = Tr;
}

impl<L: Send> Wrap for Eithers<L> {
    type Wrap<Tr: ?Sized + Trait> = IntoEither<L, Tr>;
}

impl<L: Send> Pure for Eithers<L> {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>> {
        Either::Right(x)
    }
}

impl<L: Send> Map for Eithers<L> {
    fn map<F: MapFn<In>, In: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        x.into_either().map_right(|x| f.run(x))
    }
}
