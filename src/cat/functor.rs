use either::Either;

use crate::{Impl, Trait};

use super::morphism::*;

pub trait Unwrap<Tr: ?Sized + Trait>: Wrap<Wrap<Self::Tr> = Tr> {
    type Tr: ?Sized + Trait;
}

pub trait Wrapped<Wr: ?Sized + Wrap>: Trait
where
    Wr: Wrap<Wrap<Self::Tr> = Self>,
{
    type Tr: ?Sized + Trait;
}

impl<Wr: ?Sized + Unwrap<Tr>, Tr: ?Sized + Trait> Wrapped<Wr> for Tr {
    type Tr = <Wr as Unwrap<Tr>>::Tr;
}

pub trait Wrap {
    type Wrap<Tr: ?Sized + Trait>: ?Sized + Wrapped<Self, Tr = Tr>;
}

pub trait Pure: Wrap {
    fn pure<Tr: ?Sized + Trait>(_: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>>;
}

pub trait Map: Wrap {
    fn map<F: MapFn<In>, In: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<In>>,
        _: F,
    ) -> impl Impl<Self::Wrap<F::Out>>;
}

pub trait Map2: Wrap {
    fn map2<F: MapFn2<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<In0>>,
        _: impl Impl<Self::Wrap<In1>>,
        _: F,
    ) -> impl Impl<Self::Wrap<F::Out>>;
}

pub trait Select: Wrap {
    fn select<F: SelectFn<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<In0>>,
        _: impl Impl<Self::Wrap<In1>>,
        _: F,
    ) -> impl Impl<Self::Wrap<F::Out>>;
}

pub trait Flatten: Wrap {
    fn flatten<Tr: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>>;
}

pub trait Iterate: Wrap {
    fn iterate<F: IterateFn<Self>>(_: F) -> impl Impl<Self::Wrap<F::Out>>;
}

pub trait ToEither: Wrap {
    fn either<In: ?Sized + Trait, Out: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<In>>,
    ) -> Either<impl Impl<In>, impl Impl<Self::Wrap<Out>>>;
}

pub trait Transpose: Wrap {
    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>>;
}

pub trait Functor: Map {}

impl<Wr: ?Sized + Map> Functor for Wr {}

pub trait Applicative: Functor + Map2 + Pure {}

impl<Wr: ?Sized + Functor + Map2 + Pure> Applicative for Wr {}

pub trait Monad: Applicative + Flatten {}

impl<Wr: ?Sized + Applicative + Flatten> Monad for Wr {}

pub trait WrappedMapExt<Wr: ?Sized + Map<Wrap<Tr::Tr> = Tr>, Tr: ?Sized + Wrapped<Wr>>:
    Impl<Tr>
{
    fn w_map<F: MapFn<Tr::Tr>>(self, f: F) -> impl Impl<Wr::Wrap<F::Out>> {
        Wr::map(self, f)
    }
}

impl<Wr: ?Sized + Map<Wrap<Tr::Tr> = Tr>, Tr: ?Sized + Wrapped<Wr>, TrO: ?Sized + Impl<Tr>>
    WrappedMapExt<Wr, Tr> for TrO
{
}

// fn funny_map<Wr: ?Sized + Map, Tr: ?, F: MapFn
