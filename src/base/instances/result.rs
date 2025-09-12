use std::convert::identity;

use either::Either;
use ghost::phantom;

use crate::{
    base::{functor::*, morphism::*},
    existence::Sometimes,
};

#[phantom]
pub struct Results<E>;

impl<T, E> BaseUnwrap<Result<T, E>> for Results<E> {
    type T = T;
}

impl<E> BaseWrap for Results<E> {
    type Wrap<T> = Result<T, E>;
}

impl<E> BasePure for Results<E> {
    fn pure<T>(x: T) -> Self::Wrap<T> {
        Ok(x)
    }
}

impl<E> BaseMap for Results<E> {
    fn map<Out, In>(x: Self::Wrap<In>, f: impl FnOnce(In) -> Out) -> Self::Wrap<Out> {
        x.map(f)
    }
}

impl<E> BaseMap2 for Results<E> {
    fn map2<Out, In0, In1>(
        x0: Self::Wrap<In0>,
        x1: Self::Wrap<In1>,
        f: impl FnOnce(In0, In1) -> Out,
    ) -> Self::Wrap<Out> {
        Ok(f(x0?, x1?))
    }
}

impl<E> BaseSelect for Results<E> {
    fn select<In0, In1>(
        x0: Self::Wrap<In0>,
        x1: Self::Wrap<In1>,
    ) -> BaseSelectWrap<Self, In0, In1> {
        match (x0, x1) {
            (Err(e), Err(_)) => Err(e),
            (Err(e), Ok(x1)) => Ok(Either::Right((x1, Err(e)))),
            (Ok(x0), x1) => Ok(Either::Left((x0, x1))),
        }
    }
}

impl<E> BaseFlatten for Results<E> {
    fn flatten<T>(x: Self::Wrap<Self::Wrap<T>>) -> Self::Wrap<T> {
        x.and_then(identity)
    }
}

impl<E> BaseIterate for Results<E> {
    fn iterate<F: BaseIterateFn<Self>>(mut f: F) -> Self::Wrap<F::Out> {
        Ok(loop {
            match f.run()? {
                Either::Left(next) => f = next,
                Either::Right(x) => break x,
            }
        })
    }
}

impl<E> BaseToEither for Results<E> {
    type L = Sometimes;
    type R = Sometimes;

    fn either<In, Out>(x: Self::Wrap<In>) -> BaseToEitherWrap<Self, In, Out> {
        match x {
            Ok(x) => Either::Left((x, Sometimes)),
            Err(e) => Either::Right((Err(e), Sometimes)),
        }
    }
}

impl<E> BaseTranspose for Results<E> {
    fn transpose<Wr: ?Sized + BasePure + BaseMap, T>(
        x: Self::Wrap<Wr::Wrap<T>>,
    ) -> Wr::Wrap<Self::Wrap<T>> {
        match x {
            Ok(x) => x.b_map(Ok),
            Err(e) => Wr::pure(Err(e)),
        }
    }
}

impl<E> BaseInspect for Results<E> {
    fn inspect<Out, In>(x: Self::Wrap<In>, f: impl FnOnce(&mut In) -> Out) -> Self::Wrap<Out> {
        x.map(|mut x| f(&mut x))
    }
}
