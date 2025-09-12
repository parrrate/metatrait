use std::{convert::Infallible, marker::PhantomData};

use ghost::phantom;

use crate::{Impl, Trait};

use super::{functor::*, morphism::*};

#[phantom]
pub struct PureFn<Wr: ?Sized, Tr: ?Sized>;

impl<Wr: ?Sized + Pure, Tr: ?Sized + Trait> MapFn<Tr> for PureFn<Wr, Tr> {
    type Out = Wr::Wrap<Tr>;

    fn run(self, x: impl Impl<Tr>) -> impl Impl<Self::Out> {
        Wr::pure(x)
    }
}

#[phantom]
pub struct FlattenFn<Wr: ?Sized, Tr: ?Sized>;

impl<Wr: ?Sized + Flatten, Tr: ?Sized + Trait> MapFn<Wr::Wrap<Wr::Wrap<Tr>>> for FlattenFn<Wr, Tr> {
    type Out = Wr::Wrap<Tr>;

    fn run(self, x: impl Impl<Wr::Wrap<Wr::Wrap<Tr>>>) -> impl Impl<Self::Out> {
        x.w_flatten()
    }
}

#[phantom]
pub(crate) struct TransposeFn<WrO: ?Sized, WrI: ?Sized, Tr: ?Sized>;

impl<WrO: ?Sized + Pure + Map, WrI: ?Sized + Transpose, Tr: ?Sized + Trait>
    MapFn<WrI::Wrap<WrO::Wrap<Tr>>> for TransposeFn<WrO, WrI, Tr>
{
    type Out = WrO::Wrap<WrI::Wrap<Tr>>;

    fn run(self, x: impl Impl<WrI::Wrap<WrO::Wrap<Tr>>>) -> impl Impl<Self::Out> {
        WrI::transpose::<WrO, Tr>(x)
    }
}

pub enum SelectMap<Wr: ?Sized, Tr: ?Sized> {
    __Phantom(PhantomData<Wr>, PhantomData<Tr>, Infallible),
}

impl<Wr: ?Sized + Wrap, Tr: ?Sized + Trait> SelectMap<Wr, Tr> {
    fn run0<F, T: Impl<Wr::Wrap<Tr>>>(x: T, f: F) -> SelectMap0<F, T, Wr, Tr> {
        SelectMap0(f, x, PhantomData, PhantomData)
    }

    fn run1<F, T: Impl<Wr::Wrap<Tr>>>(x: T, f: F) -> SelectMap1<F, T, Wr, Tr> {
        SelectMap1(f, x, PhantomData, PhantomData)
    }
}

pub struct SelectMap01<T, F, In: ?Sized>(T, PhantomData<F>, PhantomData<In>);

impl<T, F, In: ?Sized> SelectMap01<T, F, In> {
    pub fn new(x: T) -> Self {
        Self(x, PhantomData, PhantomData)
    }
}

impl<T: Impl<F::Tr0>, F: SelectFn<In, Tr>, Tr: ?Sized + Trait, In: ?Sized + Trait> MapFn<Tr>
    for SelectMap01<T, F, In>
{
    type Out = F::Out;

    fn run(self, x: impl Impl<Tr>) -> impl Impl<Self::Out> {
        F::run01(self.0, x)
    }
}

pub struct SelectMap10<T, F, In: ?Sized>(T, PhantomData<F>, PhantomData<In>);

impl<T, F, In: ?Sized> SelectMap10<T, F, In> {
    pub fn new(x: T) -> Self {
        Self(x, PhantomData, PhantomData)
    }
}

impl<T: Impl<F::Tr1>, F: SelectFn<Tr, In>, Tr: ?Sized + Trait, In: ?Sized + Trait> MapFn<Tr>
    for SelectMap10<T, F, In>
{
    type Out = F::Out;

    fn run(self, x: impl Impl<Tr>) -> impl Impl<Self::Out> {
        F::run10(self.0, x)
    }
}

pub struct SelectMap0<F, T, Wr: ?Sized, Tr: ?Sized>(F, T, PhantomData<Wr>, PhantomData<Tr>);

impl<
        F: SelectFn<In, Tr>,
        T: Impl<Wr::Wrap<Tr>>,
        Wr: ?Sized + Pure + Map,
        Tr: ?Sized + Trait,
        In: ?Sized + Trait,
    > MapFn<In> for SelectMap0<F, T, Wr, Tr>
{
    type Out = Wr::Wrap<F::Out>;

    fn run(self, x: impl Impl<In>) -> impl Impl<Self::Out> {
        Trait::union(
            self.0
                .run0(x)
                .map_left(Wr::pure)
                .map_right(|x| self.1.w_map(SelectMap01::<_, F, In>::new(x))),
        )
    }
}

pub struct SelectMap1<F, T, Wr: ?Sized, Tr: ?Sized>(F, T, PhantomData<Wr>, PhantomData<Tr>);

impl<
        F: SelectFn<Tr, In>,
        T: Impl<Wr::Wrap<Tr>>,
        Wr: ?Sized + Pure + Map,
        Tr: ?Sized + Trait,
        In: ?Sized + Trait,
    > MapFn<In> for SelectMap1<F, T, Wr, Tr>
{
    type Out = Wr::Wrap<F::Out>;

    fn run(self, x: impl Impl<In>) -> impl Impl<Self::Out> {
        Trait::union(
            self.0
                .run1(x)
                .map_left(Wr::pure)
                .map_right(|x| self.1.w_map(SelectMap10::<_, F, In>::new(x))),
        )
    }
}

pub trait SelectMapExt: Pure + Map + Flatten {
    fn select_0<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        x0.w_map(SelectMap::<Self, In1>::run0(x1, f)).w_flatten()
    }

    fn select_1<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        x1.w_map(SelectMap::<Self, In0>::run1(x0, f)).w_flatten()
    }
}

impl<Wr: ?Sized + Pure + Map + Flatten> SelectMapExt for Wr {}
