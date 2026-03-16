use std::future::Future;

use either::Either;
use futures::{
    future::{ready, select},
    FutureExt,
};

use crate::{
    cat::{functor::*, morphism::*},
    traits::{
        either::IntoEitherExt,
        future::{IntoFuture2, ToFuture},
    },
    FreeExt, Impl, Trait,
};

pub struct Futures;

impl<Tr: ?Sized + Trait> Unwrap<ToFuture<Tr>> for Futures {
    type Tr = Tr;
}

impl Wrap for Futures {
    type Wrap<Tr: ?Sized + Trait> = ToFuture<Tr>;
}

impl Pure for Futures {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Future<Output: Impl<Tr>> {
        ready(x)
    }
}

impl Map for Futures {
    fn map<F: MapFn<In>, In: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Future<Output: Impl<F::Out>> {
        x.t_into_future().map(|x| f.run(x))
    }
}

impl Map2 for Futures {
    fn map2<F: MapFn2<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Future<Output: Impl<F::Out>> {
        futures::future::join(x0.t_into_future(), x1.t_into_future()).map(|(x0, x1)| f.run(x0, x1))
    }
}

impl Select for Futures {
    async fn select<F: SelectFn<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<F::Out> {
        Trait::union(
            match select(
                std::pin::pin!(x0.t_into_future()),
                std::pin::pin!(x1.t_into_future()),
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

impl Flatten for Futures {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Future<Output: Impl<Tr>> {
        x.t_into_future().map(|x| x.t_into_future()).flatten()
    }
}

impl Iterate for Futures {
    async fn iterate<F: IterateFn<Self>>(mut f: F) -> impl Impl<F::Out> {
        loop {
            match f.run().t_into_future().await.into_either() {
                Either::Left(next) => f = next,
                Either::Right(x) => break x,
            }
        }
    }
}

impl Inspect for Futures {
    async fn inspect<F: InspectFn<In, Self>, In: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<F::Out> {
        f.run(&mut x.t_into_future().await)
            .t_into_future()
            .await
            .free()
    }
}

#[cfg(test)]
mod test {
    use crate::traits::is::Into2;

    use super::*;

    #[test]
    fn test() {
        let x = Futures::pure(0i32);
        let x = Futures::map(x, |x| x + 1);
        let x = Futures::map(x, |x| x + 1);
        let x = Futures::map(x, |x| x + 1);
        let x = Futures::map(x, |x| x + 1);
        let x = Futures::map(x, |x| x + 1);
        let x = x.t_into_future();
        let x = futures::executor::block_on(x);
        let x = x.t_into();
        assert_eq!(x, 5);
    }
}
