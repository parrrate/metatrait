use either::Either;

use crate::{
    traits::{
        either::IntoEither,
        is::{Is, IsExt},
    },
    Impl, Structural, Trait,
};

use super::functor::*;

pub trait MapFn<In: ?Sized + Trait> {
    type Out: ?Sized + Trait;
    fn run(self, _: impl Impl<In>) -> impl Impl<Self::Out>;
}

impl<F: FnOnce(In) -> Out, In, Out> MapFn<Is<In>> for F {
    type Out = Is<Out>;

    fn run(self, x: impl Impl<Is<In>>) -> impl Impl<Self::Out> {
        self(x.into_that())
    }
}

pub trait MapFn2<In0: ?Sized + Trait, In1: ?Sized + Trait> {
    type Out: ?Sized + Trait;
    fn run(self, _: impl Impl<In0>, _: impl Impl<In1>) -> impl Impl<Self::Out>;
}

pub trait TraitFn: Sized {
    type Out: ?Sized + Trait;
}

pub trait SelectFn<In0: ?Sized + Trait, In1: ?Sized + Trait>: TraitFn {
    type Tr0: ?Sized + Trait;
    type Tr1: ?Sized + Trait;

    fn run0(self, _: impl Impl<In0>) -> Either<impl Impl<Self::Out>, impl Impl<Self::Tr0>>;
    fn run1(self, _: impl Impl<In1>) -> Either<impl Impl<Self::Out>, impl Impl<Self::Tr1>>;
    fn run01(_: impl Impl<Self::Tr0>, _: impl Impl<In1>) -> impl Impl<Self::Out>;
    fn run10(_: impl Impl<Self::Tr1>, _: impl Impl<In0>) -> impl Impl<Self::Out>;
}

pub trait IterateFn<Wr: ?Sized + Wrap>: TraitFn {
    fn run(self) -> impl Impl<Wr::Wrap<IntoEither<Self, Self::Out>>>;
}

pub trait IterateFnExt<Wr: ?Sized + Iterate>: IterateFn<Wr> {
    fn run_iterate(self) -> impl Impl<Wr::Wrap<Self::Out>> {
        Wr::iterate(self)
    }
}

impl<Wr: ?Sized + Iterate, F: IterateFn<Wr>> IterateFnExt<Wr> for F {}

pub trait InspectFn<In: ?Sized + Trait, Wr: ?Sized + Wrap> {
    type Out: ?Sized + Structural;
    fn run(self, _: &mut impl Impl<In>) -> impl Impl<Wr::Wrap<Self::Out>>;
}
