use either::Either;

use crate::{
    base::functor::*,
    cat::{functor::*, morphism::*, util::PureFn},
    traits::base::{Base, BaseExt},
    Impl, Trait,
};

pub struct BaseInstance<WrB: ?Sized>(WrB);

impl<WrB: ?Sized + BaseMap, Tr: ?Sized + Trait> Unwrap<Base<WrB, Tr>> for BaseInstance<WrB> {
    type Tr = Tr;
}

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
        x.into_base().b_map(|x| f.run(x))
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
        WrB::select(x0.into_base(), x1.into_base())
            .b_map(|x| {
                match x {
                    Either::Left((x, y)) => match f.run0(x) {
                        Either::Left(x) => WrB::pure(Either::Left(x)),
                        Either::Right(x) => y.b_map(|y| Either::Right(F::run01(x, y))),
                    }
                    .b_map(|x| Either::Left(Trait::union(x))),
                    Either::Right((x, y)) => match f.run1(x) {
                        Either::Left(x) => WrB::pure(Either::Left(x)),
                        Either::Right(x) => y.b_map(|y| Either::Right(F::run10(x, y))),
                    }
                    .b_map(|x| Either::Right(Trait::union(x))),
                }
                .b_map(Trait::union)
            })
            .b_flatten()
    }
}

impl<WrB: ?Sized + BaseMap + BaseFlatten> Flatten for BaseInstance<WrB> {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>> {
        x.into_base().b_map(|x| x.into_base()).b_flatten()
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

impl<WrB: ?Sized + BaseMap + BaseToEither + BasePure> Transpose for BaseInstance<WrB> {
    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>> {
        Trait::union(match Self::either::<_, Tr>(x.into_base()) {
            Either::Left(x) => Either::Left(x.w_map(PureFn::<Self, _>)),
            Either::Right(x) => Either::Right(Wr::pure(x)),
        })
    }
}
