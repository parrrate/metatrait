use crate::linked::{
    functional::{Map, Map2, MapFn, MapFn2, Pure, Wrap},
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
