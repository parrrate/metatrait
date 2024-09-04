use either::Either;

use crate::linked::{
    functional::{
        Flatten, Map, Map2, MapExt, MapFn2, Pure, Select, SelectFn, Union, UnionFn, Wrap,
    },
    traits::to::{To, ToExt},
    Impl, Trait,
};

pub struct Lazy;

impl Wrap for Lazy {
    type Wrap<Tr: ?Sized + Trait> = To<Tr>;
}

impl Pure for Lazy {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>> {
        || x
    }
}

impl Map for Lazy {
    fn map<In: ?Sized + Trait, F: crate::linked::functional::MapFn<In>>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        || f.run(x.to())
    }
}

impl Map2 for Lazy {
    fn map2<In0: ?Sized + Trait, In1: ?Sized + Trait, F: MapFn2<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        || f.run(x0.to(), x1.to())
    }
}

impl Union for Lazy {
    fn union<F: UnionFn>(
        x: Either<impl Impl<Self::Wrap<F::Out>>, impl Impl<Self::Wrap<F::Out>>>,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        || {
            F::union(match x {
                Either::Left(x) => Either::Left(x.to()),
                Either::Right(x) => Either::Right(x.to()),
            })
        }
    }
}

impl Select for Lazy {
    fn select<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<Self, In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        Self::select_0(x0, x1, f)
    }
}

impl Flatten for Lazy {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>> {
        || x.to().to()
    }
}
