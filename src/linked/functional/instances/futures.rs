use either::Either;
use futures::future::select;

use crate::linked::{
    functional::{Flatten, Map, Map2, MapFn, MapFn2, Pure, Select, SelectFn, Union, UnionFn, Wrap},
    traits::future::{ToFuture, ToFutureExt},
    Impl, Trait,
};

struct Futures;

impl Wrap for Futures {
    type Wrap<Tr: ?Sized + Trait> = ToFuture<Tr>;
}

impl Pure for Futures {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>> {
        async { x }
    }
}

impl Map for Futures {
    fn map<In: ?Sized + Trait, F: MapFn<In>>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        async { f.run(x.to_future().await) }
    }
}

impl Map2 for Futures {
    fn map2<In0: ?Sized + Trait, In1: ?Sized + Trait, F: MapFn2<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        async {
            let (x0, x1) = futures::join!(x0.to_future(), x1.to_future());
            f.run(x0, x1)
        }
    }
}

impl Union for Futures {
    fn union<F: UnionFn>(
        x: Either<impl Impl<Self::Wrap<F::Out>>, impl Impl<Self::Wrap<F::Out>>>,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        async {
            F::union(match x {
                Either::Left(x) => Either::Left(x.to_future().await),
                Either::Right(x) => Either::Right(x.to_future().await),
            })
        }
    }
}

impl Select for Futures {
    fn select<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<Self, In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        async {
            F::union(
                match select(Box::pin(x0.to_future()), Box::pin(x1.to_future())).await {
                    futures::future::Either::Left((x, y)) => Either::Left(f.run0(x, y)),
                    futures::future::Either::Right((x, y)) => Either::Right(f.run1(x, y)),
                },
            )
        }
    }
}

impl Flatten for Futures {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>> {
        async { x.to_future().await.to_future().await }
    }
}
