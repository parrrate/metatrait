use either::Either;

use crate::{
    cat::{functor::*, morphism::*},
    existence::{Never, Sometimes},
    traits::either::IntoEitherExt,
    Impl, Sample, StructuralExt, Trait,
};

pub struct Verbatim;

impl<Tr: ?Sized + Trait> Unwrap<Tr> for Verbatim {
    type Tr = Tr;
}

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

impl Iterate for Verbatim {
    fn iterate<F: IterateFn<Self>>(mut f: F) -> impl Impl<Self::Wrap<F::Out>> {
        loop {
            match f.run().into_either() {
                Either::Left(next) => f = next,
                Either::Right(x) => break x,
            }
        }
    }
}

impl ToEither for Verbatim {
    type L = Sometimes;
    type R = Never;

    fn either<In: ?Sized + Trait, Out: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
    ) -> Either<(impl Impl<In>, Self::L), (impl Impl<Self::Wrap<Out>>, Self::R)> {
        Either::<_, (Sample<Self::Wrap<Out>>, _)>::Left((x, Sometimes))
    }
}

impl Transpose for Verbatim {
    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>> {
        x
    }
}

impl Inspect for Verbatim {
    fn inspect<F: InspectFn<In, Self>, In: ?Sized + Trait>(
        mut x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        f.run(&mut x).free()
    }
}

#[cfg(test)]
mod test {
    use crate::traits::is::IsExt;

    use super::*;

    #[test]
    fn test() {
        let x = Verbatim::pure(0);
        let x = Verbatim::map(x, |x| x + 1);
        let x = Verbatim::map(x, |x| x + 1);
        let x = Verbatim::map(x, |x| x + 1);
        let x = Verbatim::map(x, |x| x + 1);
        let x = Verbatim::map(x, |x| x + 1);
        let x = x.into_that();
        assert_eq!(x, 5);
    }
}
