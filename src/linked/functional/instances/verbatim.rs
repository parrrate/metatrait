use either::Either;

use crate::linked::{
    functional::{
        Flatten, Map, Map2, MapExt, MapFn, MapFn2, Pure, Select, SelectFn, Transpose, Union,
        UnionFn, Wrap,
    },
    Impl, Trait,
};

pub struct Verbatim;

impl Wrap for Verbatim {
    type Wrap<Tr: ?Sized + Trait> = Tr;
}

impl Pure for Verbatim {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>> {
        x
    }
}

impl Map for Verbatim {
    fn map<In: ?Sized + Trait, F: MapFn<In>>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        f.run(x)
    }
}

impl Map2 for Verbatim {
    fn map2<In0: ?Sized + Trait, In1: ?Sized + Trait, F: MapFn2<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        f.run(x0, x1)
    }
}

impl Union for Verbatim {
    fn union<F: UnionFn>(
        x: Either<impl Impl<Self::Wrap<F::Out>>, impl Impl<Self::Wrap<F::Out>>>,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        F::union(x)
    }
}

impl Select for Verbatim {
    fn select<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<Self, In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        Self::select_0(x0, x1, f)
    }
}

impl Flatten for Verbatim {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>> {
        x
    }
}

impl Transpose for Verbatim {
    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>> {
        x
    }
}
