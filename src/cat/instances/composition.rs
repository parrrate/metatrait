use std::marker::PhantomData;

use either::Either;
use ghost::phantom;

use crate::{
    cat::{
        functor::*,
        morphism::*,
        util::{FlattenFn, SelectMap01, SelectMap10, TransposeFn},
    },
    existence::{Into2, When},
    traits::either::{IntoEither, IntoEitherExt},
    Impl, Trait,
};

use super::either::Eithers;

pub struct Composition<WrO, WrI>(WrO, WrI);

impl<
        WrO: Wrap<Wrap<Ti> = To>,
        WrI: Wrap<Wrap<T> = Ti>,
        To: ?Sized + Wrapped<WrO, Tr = Ti>,
        Ti: ?Sized + Wrapped<WrI, Tr = T>,
        T: ?Sized + Trait,
    > Unwrap<To> for Composition<WrO, WrI>
{
    type Tr = T;
}

impl<WrO: Wrap, WrI: Wrap> Wrap for Composition<WrO, WrI> {
    type Wrap<Tr: ?Sized + Trait> = WrO::Wrap<WrI::Wrap<Tr>>;
}

impl<WrO: Pure, WrI: Pure> Pure for Composition<WrO, WrI> {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>> {
        WrO::pure(WrI::pure(x))
    }
}

struct CompositionMap<F, Wr: ?Sized, In: ?Sized>(F, PhantomData<Wr>, PhantomData<In>);

impl<F: MapFn<In>, Wr: ?Sized + Map, In: ?Sized + Trait> CompositionMap<F, Wr, In> {
    fn new(f: F) -> Self {
        Self(f, PhantomData, PhantomData)
    }
}

impl<F: MapFn<In>, Wr: ?Sized + Map, In: ?Sized + Trait> MapFn<Wr::Wrap<In>>
    for CompositionMap<F, Wr, In>
{
    type Out = Wr::Wrap<F::Out>;

    fn run(self, x: impl Impl<Wr::Wrap<In>>) -> impl Impl<Self::Out> {
        x.w_map(self.0)
    }
}

impl<WrO: Map, WrI: Map> Map for Composition<WrO, WrI> {
    fn map<F: MapFn<In>, In: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        x.w_map(CompositionMap::<_, WrI, _>::new(f))
    }
}

struct CompositionMap2<F, Wr: ?Sized, In0: ?Sized, In1: ?Sized>(
    F,
    PhantomData<Wr>,
    PhantomData<In0>,
    PhantomData<In1>,
);

impl<F: MapFn2<In0, In1>, Wr: ?Sized + Map2, In0: ?Sized + Trait, In1: ?Sized + Trait>
    CompositionMap2<F, Wr, In0, In1>
{
    fn new(f: F) -> Self {
        Self(f, PhantomData, PhantomData, PhantomData)
    }
}

impl<F: MapFn2<In0, In1>, Wr: ?Sized + Map2, In0: ?Sized + Trait, In1: ?Sized + Trait>
    MapFn2<Wr::Wrap<In0>, Wr::Wrap<In1>> for CompositionMap2<F, Wr, In0, In1>
{
    type Out = Wr::Wrap<F::Out>;

    fn run(
        self,
        x0: impl Impl<Wr::Wrap<In0>>,
        x1: impl Impl<Wr::Wrap<In1>>,
    ) -> impl Impl<Self::Out> {
        Wr::map2(x0, x1, self.0)
    }
}

impl<WrO: Map2, WrI: Map2> Map2 for Composition<WrO, WrI> {
    fn map2<F: MapFn2<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        WrO::map2(x0, x1, CompositionMap2::<_, WrI, _, _>::new(f))
    }
}

struct CompositionSelect<F, WrI, In0: ?Sized, In1: ?Sized>(
    F,
    PhantomData<WrI>,
    PhantomData<In0>,
    PhantomData<In1>,
);

impl<F: SelectFn<In0, In1>, WrI: Map, In0: ?Sized + Trait, In1: ?Sized + Trait> TraitFn
    for CompositionSelect<F, WrI, In0, In1>
{
    type Out = WrI::Wrap<F::Out>;
}

impl<
        F: SelectFn<In0, In1>,
        WrI: Map + ToEither + Pure,
        In0: ?Sized + Trait,
        In1: ?Sized + Trait,
    > SelectFn<WrI::Wrap<In0>, WrI::Wrap<In1>> for CompositionSelect<F, WrI, In0, In1>
{
    type Tr0 = F::Tr0;
    type Tr1 = F::Tr1;

    fn run0(
        self,
        x: impl Impl<WrI::Wrap<In0>>,
    ) -> Either<impl Impl<Self::Out>, impl Impl<Self::Tr0>> {
        match WrI::either(x) {
            Either::Left((x, _)) => match self.0.run0(x) {
                Either::Left(x) => Either::Left(Either::Left(WrI::pure(x))),
                Either::Right(x) => Either::Right(x),
            },
            Either::Right((x, _)) => Either::Left(Either::Right(x)),
        }
        .map_left(Trait::union)
    }

    fn run1(
        self,
        x: impl Impl<WrI::Wrap<In1>>,
    ) -> Either<impl Impl<Self::Out>, impl Impl<Self::Tr1>> {
        match WrI::either(x) {
            Either::Left((x, _)) => match self.0.run1(x) {
                Either::Left(x) => Either::Left(Either::Left(WrI::pure(x))),
                Either::Right(x) => Either::Right(x),
            },
            Either::Right((x, _)) => Either::Left(Either::Right(x)),
        }
        .map_left(Trait::union)
    }

    fn run01(x: impl Impl<Self::Tr0>, y: impl Impl<WrI::Wrap<In1>>) -> impl Impl<Self::Out> {
        y.w_map(SelectMap01::<_, F, _>::new(x))
    }

    fn run10(x: impl Impl<Self::Tr1>, y: impl Impl<WrI::Wrap<In0>>) -> impl Impl<Self::Out> {
        y.w_map(SelectMap10::<_, F, _>::new(x))
    }
}

impl<WrO: Select, WrI: Map + ToEither + Pure> Select for Composition<WrO, WrI> {
    fn select<F: SelectFn<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        WrO::select(
            x0,
            x1,
            CompositionSelect::<_, WrI, _, _>(f, PhantomData, PhantomData, PhantomData),
        )
    }
}

impl<WrO: Flatten + Map + Pure, WrI: Flatten + Transpose> Flatten for Composition<WrO, WrI> {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>> {
        x.w_map(TransposeFn::<WrO, WrI, _>)
            .w_flatten()
            .w_map(FlattenFn::<WrI, _>)
    }
}

struct CompositionIterate<F, WrI: ?Sized>(F, PhantomData<WrI>);

impl<F: TraitFn, WrI: ?Sized + Wrap> TraitFn for CompositionIterate<F, WrI> {
    type Out = WrI::Wrap<F::Out>;
}

#[phantom]
struct CompositionIterateMap<F, Wr: ?Sized, Tr: ?Sized>;

impl<F, Wr: ?Sized + Transpose, Tr: ?Sized + Trait> MapFn<Wr::Wrap<IntoEither<F, Tr>>>
    for CompositionIterateMap<F, Wr, Tr>
{
    type Out = IntoEither<CompositionIterate<F, Wr>, Wr::Wrap<Tr>>;
    fn run(self, x: impl Impl<Wr::Wrap<IntoEither<F, Tr>>>) -> impl Impl<Self::Out> {
        Wr::transpose::<Eithers<_>, _>(x)
            .into_either()
            .map_left(|f| CompositionIterate(f, PhantomData))
    }
}

impl<F: IterateFn<Composition<WrO, WrI>>, WrO: Map, WrI: Transpose> IterateFn<WrO>
    for CompositionIterate<F, WrI>
{
    fn run(self) -> impl Impl<WrO::Wrap<IntoEither<Self, Self::Out>>> {
        WrO::map(self.0.run(), CompositionIterateMap)
    }
}

impl<WrO: Map + Iterate, WrI: Transpose> Iterate for Composition<WrO, WrI> {
    fn iterate<F: IterateFn<Self>>(f: F) -> impl Impl<Self::Wrap<F::Out>> {
        CompositionIterate(f, PhantomData).run_iterate()
    }
}

impl<WrO: ToEither + Pure, WrI: ToEither> ToEither for Composition<WrO, WrI> {
    type L = <WrO::L as When>::And<WrI::L>;
    type R = <WrO::R as When>::Or<WrI::R>;

    fn either<In: ?Sized + Trait, Out: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
    ) -> Either<(impl Impl<In>, Self::L), (impl Impl<Self::Wrap<Out>>, Self::R)> {
        match WrO::either(x) {
            Either::Left((x, eo)) => match WrI::either(x) {
                Either::Left((x, ei)) => Either::Left((x, (eo, ei).into())),
                Either::Right((x, ei)) => Either::Right((Either::Left(WrO::pure(x)), ei.into2())),
            },
            Either::Right((x, eo)) => Either::Right((Either::Right(x), eo.into2())),
        }
        .map_right(|(x, e)| (Trait::union(x), e))
    }
}

impl<WrO: Transpose + Map, WrI: Transpose> Transpose for Composition<WrO, WrI> {
    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>> {
        WrO::transpose::<Wr, _>(x.w_map(TransposeFn::<Wr, WrI, _>))
    }
}

#[cfg(all(test, feature = "futures"))]
mod test {
    use crate::{
        cat::{
            instances::{futures::Futures, lazy::Lazy},
            util::Wraps,
        },
        traits::{future::ToFutureExt, is::IsExt, to::ToExt},
    };

    use super::*;

    fn wrap_five<Wr: Map + Pure>() -> impl Wraps<Wr, i32> {
        let x = Wr::pure(0);
        let x = x.w_map(|x| x + 1);
        let x = x.w_map(|x| x + 1);
        let x = x.w_map(|x| x + 1);
        let x = x.w_map(|x| x + 1);
        let x = x.w_map(|x| x + 1);
        #[allow(clippy::let_and_return)]
        x
    }

    #[test]
    fn test() {
        type Wr = Composition<Futures, Lazy>;
        let x = wrap_five::<Wr>();
        let x = x.to_future();
        let x = futures::executor::block_on(x);
        let x = x.to();
        let x = x.into_that();
        assert_eq!(x, 5);
    }
}
