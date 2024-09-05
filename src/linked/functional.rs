use std::{convert::Infallible, marker::PhantomData};

use either::Either;
use ghost::phantom;

use super::{
    traits::{
        empty::Empty,
        is::{Is, IsExt},
    },
    Impl, Trait,
};

pub mod instances;

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

pub trait BaseFn {
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

pub trait Select: Wrap {
    fn select<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<In0, In1>>(
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

#[phantom]
pub struct FlattenFn<Wr: ?Sized, Tr: ?Sized>;

impl<Wr: ?Sized + Flatten, Tr: ?Sized + Trait> MapFn<Wr::Wrap<Wr::Wrap<Tr>>> for FlattenFn<Wr, Tr> {
    type Out = Wr::Wrap<Tr>;

    fn run(self, x: impl Impl<Wr::Wrap<Wr::Wrap<Tr>>>) -> impl Impl<Self::Out> {
        Wr::flatten(x)
    }
}

pub trait Transpose: Wrap {
    fn either<In: ?Sized + Trait, Out: ?Sized + Trait>(
        _: impl Impl<Self::Wrap<In>>,
    ) -> Either<impl Impl<In>, impl Impl<Self::Wrap<Out>>>;

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
                .map_right(|x| Wr::map(self.1, SelectMap01::<_, F, In>::new(x))),
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
                .map_right(|x| Wr::map(self.1, SelectMap10::<_, F, In>::new(x))),
        )
    }
}

pub trait SelectMapExt: Pure + Map + Flatten {
    fn select_0<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        Self::flatten(Self::map(x0, SelectMap::<Self, In1>::run0(x1, f)))
    }

    fn select_1<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        Self::flatten(Self::map(x1, SelectMap::<Self, In0>::run1(x0, f)))
    }
}

impl<Wr: ?Sized + Pure + Map + Flatten> SelectMapExt for Wr {}
