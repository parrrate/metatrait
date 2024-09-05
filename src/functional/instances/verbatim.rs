use either::Either;

use crate::{
    functional::{Flatten, Map, Map2, MapFn, MapFn2, Pure, Select, SelectFn, Transpose, Wrap},
    Impl, Sample, Trait,
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
    fn map<F: MapFn<In>, In: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        f.run(x)
    }
}

impl Map2 for Verbatim {
    fn map2<F: MapFn2<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        f.run(x0, x1)
    }
}

impl Select for Verbatim {
    fn select<F: SelectFn<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        Trait::union(f.run0(x0).map_right(|x0| F::run01(x0, x1)))
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
    fn either<In: ?Sized + Trait, Out: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
    ) -> Either<impl Impl<In>, impl Impl<Self::Wrap<Out>>> {
        Either::<_, Sample<Self::Wrap<Out>>>::Left(x)
    }

    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>> {
        x
    }
}

#[cfg(test)]
mod test {
    use crate::traits::{
        empty::Empty,
        is::{Is, IsExt},
    };

    use super::*;

    #[test]
    fn test() {
        let x = Verbatim::pure::<Is<_, Empty>>(0);
        let x = Verbatim::map(x, |x| x + 1);
        let x = Verbatim::map(x, |x| x + 1);
        let x = Verbatim::map(x, |x| x + 1);
        let x = Verbatim::map(x, |x| x + 1);
        let x = Verbatim::map(x, |x| x + 1);
        let x = x.into_that();
        assert_eq!(x, 5);
    }
}
