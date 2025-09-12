use either::Either;

use crate::{
    traits::{
        empty::Empty,
        is::{Is, IsExt},
    },
    Impl, Trait,
};

use super::functor::Wrap;

pub trait MapFn<In: ?Sized + Trait> {
    type Out: ?Sized + Trait;
    fn run(self, _: impl Impl<In>) -> impl Impl<Self::Out>;
}

impl<F: FnOnce(In) -> Out, In, Out> MapFn<Is<In, Empty>> for F {
    type Out = Is<Out, Empty>;

    fn run(self, x: impl Impl<Is<In, Empty>>) -> impl Impl<Self::Out> {
        self(x.into_that())
    }
}

pub trait MapFn2<In0: ?Sized + Trait, In1: ?Sized + Trait> {
    type Out: ?Sized + Trait;
    fn run(self, _: impl Impl<In0>, _: impl Impl<In1>) -> impl Impl<Self::Out>;
}

pub trait BaseFn: Sized {
    type Out: ?Sized + Trait;
}

pub trait SelectFn<In0: ?Sized + Trait, In1: ?Sized + Trait>: BaseFn {
    type Tr0: ?Sized + Trait;
    type Tr1: ?Sized + Trait;

    fn run0(self, _: impl Impl<In0>) -> Either<impl Impl<Self::Out>, impl Impl<Self::Tr0>>;
    fn run1(self, _: impl Impl<In1>) -> Either<impl Impl<Self::Out>, impl Impl<Self::Tr1>>;
    fn run01(_: impl Impl<Self::Tr0>, _: impl Impl<In1>) -> impl Impl<Self::Out>;
    fn run10(_: impl Impl<Self::Tr1>, _: impl Impl<In0>) -> impl Impl<Self::Out>;
}

pub trait IterateFn<Wr: ?Sized + Wrap>: BaseFn {
    fn done(self) -> Either<impl Impl<Self::Out>, impl Impl<Wr::Wrap<Is<Self, Empty>>>>;
}
