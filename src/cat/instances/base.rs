use either::Either;

use crate::{
    base::functor::*,
    cat::{functor::*, morphism::*},
    traits::base::{Base, BaseExt},
    Impl, Trait,
};

pub struct BaseInstance<WrB: ?Sized>(WrB);

impl<WrB: ?Sized + BaseMap> Wrap for BaseInstance<WrB> {
    type Wrap<Tr: ?Sized + Trait> = Base<WrB, Tr>;
}

impl<WrB: ?Sized + BaseMap + BasePure> Pure for BaseInstance<WrB> {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>> {
        WrB::pure(x)
    }
}

impl<WrB: ?Sized + BaseMap> Map for BaseInstance<WrB> {
    fn map<F: MapFn<In>, In: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        WrB::map(x.into_base(), |x| f.run(x))
    }
}

impl<WrB: ?Sized + BaseMap + BaseMap2> Map2 for BaseInstance<WrB> {
    fn map2<F: MapFn2<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        WrB::map2(x0.into_base(), x1.into_base(), |x0, x1| f.run(x0, x1))
    }
}

impl<WrB: ?Sized + BaseMap + BaseSelect + BaseFlatten + BasePure> Select for BaseInstance<WrB> {
    fn select<F: SelectFn<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        WrB::flatten(WrB::map(WrB::select(x0.into_base(), x1.into_base()), |x| {
            WrB::map(
                match x {
                    Either::Left((x, y)) => WrB::map(
                        match f.run0(x) {
                            Either::Left(x) => WrB::pure(Either::Left(x)),
                            Either::Right(x) => WrB::map(y, |y| Either::Right(F::run01(x, y))),
                        },
                        |x| Either::Left(Trait::union(x)),
                    ),
                    Either::Right((x, y)) => WrB::map(
                        match f.run1(x) {
                            Either::Left(x) => WrB::pure(Either::Left(x)),
                            Either::Right(x) => WrB::map(y, |y| Either::Right(F::run10(x, y))),
                        },
                        |x| Either::Right(Trait::union(x)),
                    ),
                },
                Trait::union,
            )
        }))
    }
}

impl<WrB: ?Sized + BaseMap + BaseFlatten> Flatten for BaseInstance<WrB> {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>> {
        WrB::flatten(WrB::map(x.into_base(), |x| x.into_base()))
    }
}

impl<WrB: ?Sized + BaseMap + BaseToEither> ToEither for BaseInstance<WrB> {
    fn either<In: ?Sized + Trait, Out: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
    ) -> Either<impl Impl<In>, impl Impl<Self::Wrap<Out>>> {
        match WrB::either::<_, Out::Sample>(x.into_base()) {
            Either::Left(x) => Either::Left(x),
            Either::Right(x) => Either::Right(x),
        }
    }
}
