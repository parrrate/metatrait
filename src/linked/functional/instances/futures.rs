use either::Either;
use futures::future::select;

use crate::linked::{
    functional::{Flatten, Map, Map2, MapFn, MapFn2, Pure, Select, SelectFn, Wrap},
    traits::future::{ToFuture, ToFutureExt},
    Impl, Trait,
};

pub struct Futures;

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

impl Select for Futures {
    fn select<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        async {
            Trait::union(
                match select(
                    std::pin::pin!(x0.to_future()),
                    std::pin::pin!(x1.to_future()),
                )
                .await
                {
                    futures::future::Either::Left((x, y)) => {
                        Either::Left(Trait::union(match f.run0(x) {
                            Either::Left(x) => Either::Left(x),
                            Either::Right(x) => Either::Right(F::run01(x, y.await)),
                        }))
                    }
                    futures::future::Either::Right((x, y)) => {
                        Either::Right(Trait::union(match f.run1(x) {
                            Either::Left(x) => Either::Left(x),
                            Either::Right(x) => Either::Right(F::run10(x, y.await)),
                        }))
                    }
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

#[cfg(test)]
mod test {
    use crate::linked::traits::{
        empty::Empty,
        is::{Is, IsExt},
    };

    use super::*;

    #[test]
    fn test() {
        let x = Futures::pure::<Is<_, Empty>>(0);
        let x = Futures::map(x, |x| x + 1);
        let x = Futures::map(x, |x| x + 1);
        let x = Futures::map(x, |x| x + 1);
        let x = Futures::map(x, |x| x + 1);
        let x = Futures::map(x, |x| x + 1);
        let x = x.to_future();
        let x = futures::executor::block_on(x);
        let x = x.into_that();
        assert_eq!(x, 5);
    }
}
