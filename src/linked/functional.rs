use std::{convert::Infallible, marker::PhantomData};

use either::Either;
use ghost::phantom;

use super::{Impl, Trait};

pub mod instances;

pub trait MapFn<In: ?Sized + Trait> {
    type Out: ?Sized + Trait;
    fn run(self, _: impl Impl<In>) -> impl Impl<Self::Out>;
}

pub trait MapFn2<In0: ?Sized + Trait, In1: ?Sized + Trait> {
    type Out: ?Sized + Trait;
    fn run(self, _: impl Impl<In0>, _: impl Impl<In1>) -> impl Impl<Self::Out>;
}

pub trait UnionFn {
    type Out: ?Sized + Trait;
    fn union(_: Either<impl Impl<Self::Out>, impl Impl<Self::Out>>) -> impl Impl<Self::Out>;
}

pub trait SelectFn<Wr: ?Sized + Wrap, In0: ?Sized + Trait, In1: ?Sized + Trait>: UnionFn {
    fn run0(self, _: impl Impl<In0>, _: impl Impl<Wr::Wrap<In1>>) -> impl Impl<Self::Out>;
    fn run1(self, _: impl Impl<In1>, _: impl Impl<Wr::Wrap<In0>>) -> impl Impl<Self::Out>;
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

pub struct SelectMap0<F, T, Wr: ?Sized, Tr: ?Sized>(F, T, PhantomData<Wr>, PhantomData<Tr>);

impl<
        F: SelectFn<Wr, In, Tr>,
        T: Impl<Wr::Wrap<Tr>>,
        Wr: ?Sized + Wrap,
        Tr: ?Sized + Trait,
        In: ?Sized + Trait,
    > MapFn<In> for SelectMap0<F, T, Wr, Tr>
{
    type Out = F::Out;

    fn run(self, x: impl Impl<In>) -> impl Impl<Self::Out> {
        self.0.run0(x, self.1)
    }
}

pub struct SelectMap1<F, T, Wr: ?Sized, Tr: ?Sized>(F, T, PhantomData<Wr>, PhantomData<Tr>);

impl<
        F: SelectFn<Wr, Tr, In>,
        T: Impl<Wr::Wrap<Tr>>,
        Wr: ?Sized + Wrap,
        Tr: ?Sized + Trait,
        In: ?Sized + Trait,
    > MapFn<In> for SelectMap1<F, T, Wr, Tr>
{
    type Out = F::Out;

    fn run(self, x: impl Impl<In>) -> impl Impl<Self::Out> {
        self.0.run1(x, self.1)
    }
}

pub trait Wrap {
    type Wrap<Tr: ?Sized + Trait>: ?Sized + Trait;
}

pub trait Pure: Wrap {
    fn pure<Tr: ?Sized + Trait>(_: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>>;
}

pub trait Map: Wrap {
    fn map<In: ?Sized + Trait, F: MapFn<In>>(
        _: impl Impl<Self::Wrap<In>>,
        _: F,
    ) -> impl Impl<Self::Wrap<F::Out>>;
}

pub trait Map2: Wrap {
    fn map2<In0: ?Sized + Trait, In1: ?Sized + Trait, F: MapFn2<In0, In1>>(
        _: impl Impl<Self::Wrap<In0>>,
        _: impl Impl<Self::Wrap<In1>>,
        _: F,
    ) -> impl Impl<Self::Wrap<F::Out>>;
}

pub trait Union: Wrap {
    fn union<F: UnionFn>(
        _: Either<impl Impl<Self::Wrap<F::Out>>, impl Impl<Self::Wrap<F::Out>>>,
    ) -> impl Impl<Self::Wrap<F::Out>>;
}

pub trait Select: Union {
    fn select<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<Self, In0, In1>>(
        _: impl Impl<Self::Wrap<In0>>,
        _: impl Impl<Self::Wrap<In1>>,
        _: F,
    ) -> impl Impl<Self::Wrap<F::Out>>;
}

pub trait MapExt: Map {
    fn select_0<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<Self, In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        Self::map(x0, SelectMap::<Self, In1>::run0(x1, f))
    }

    fn select_1<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<Self, In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        Self::map(x0, SelectMap::<Self, In1>::run0(x1, f))
    }
}

impl<Wr: ?Sized + Map> MapExt for Wr {}

pub trait Flatten: Wrap {
    fn flatten<Tr: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>>;
}

#[phantom]
pub struct FlattenFn<Wr: ?Sized, Tr: ?Sized>;

impl<Wr: ?Sized + Flatten, Tr: ?Sized + Trait> MapFn<Wr::Wrap<Wr::Wrap<Tr>>> for FlattenFn<Wr, Tr> {
    type Out = Wr::Wrap<Tr>;

    fn run(self, x: impl Impl<Wr::Wrap<Wr::Wrap<Tr>>>) -> impl Impl<Self::Out> {
        Wr::flatten(x)
    }
}

pub trait Transpose: Wrap {
    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>>;
}

#[phantom]
pub struct TransposeFn<Uo: ?Sized, Ui: ?Sized, Tr: ?Sized>;

impl<Uo: ?Sized + Pure + Map, Ui: ?Sized + Transpose, Tr: ?Sized + Trait>
    MapFn<Ui::Wrap<Uo::Wrap<Tr>>> for TransposeFn<Uo, Ui, Tr>
{
    type Out = Uo::Wrap<Ui::Wrap<Tr>>;

    fn run(self, x: impl Impl<Ui::Wrap<Uo::Wrap<Tr>>>) -> impl Impl<Self::Out> {
        Ui::transpose::<Uo, Tr>(x)
    }
}

pub trait Functor: Map {}

impl<Wr: ?Sized + Map> Functor for Wr {}

pub trait Applicative: Functor + Map2 + Pure {}

impl<Wr: ?Sized + Functor + Map2 + Pure> Applicative for Wr {}

pub trait Monad: Applicative + Flatten {}

impl<Wr: ?Sized + Applicative + Flatten> Monad for Wr {}
