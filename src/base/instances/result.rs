use std::convert::identity;

use either::Either;
use ghost::phantom;

use crate::base::functor::*;

#[phantom]
pub struct Results<E>;

impl<T, E> BaseUnwrap<Results<E>> for Result<T, E> {
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
    ) -> BaseEitherWrap<Self, In0, In1> {
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

impl<E> BaseToEither for Results<E> {
    fn either<In, Out>(x: Self::Wrap<In>) -> Either<In, Self::Wrap<Out>> {
        match x {
            Ok(x) => Either::Left(x),
            Err(e) => Either::Right(Err(e)),
        }
    }
}

impl<E> BaseTranspose for Results<E> {
    fn transpose<Wr: ?Sized + BasePure + BaseMap, T>(
        x: Self::Wrap<Wr::Wrap<T>>,
    ) -> Wr::Wrap<Self::Wrap<T>> {
        match x {
            Ok(x) => Wr::map(x, Ok),
            Err(e) => Wr::pure(Err(e)),
        }
    }
}