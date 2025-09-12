use either::Either;

use crate::{
    base::{functor::*, morphism::*},
    existence::Sometimes,
};

pub struct Options;

impl<T> BaseUnwrap<Option<T>> for Options {
    type T = T;
}

impl BaseWrap for Options {
    type Wrap<T> = Option<T>;
}

impl BasePure for Options {
    fn pure<T>(x: T) -> Self::Wrap<T> {
        Some(x)
    }
}

impl BaseMap for Options {
    fn map<Out, In>(x: Self::Wrap<In>, f: impl FnOnce(In) -> Out) -> Self::Wrap<Out> {
        x.map(f)
    }
}

impl BaseMap2 for Options {
    fn map2<Out, In0, In1>(
        x0: Self::Wrap<In0>,
        x1: Self::Wrap<In1>,
        f: impl FnOnce(In0, In1) -> Out,
    ) -> Self::Wrap<Out> {
        x0.zip(x1).map(|(x0, x1)| f(x0, x1))
    }
}

impl BaseSelect for Options {
    fn select<In0, In1>(
        x0: Self::Wrap<In0>,
        x1: Self::Wrap<In1>,
    ) -> BaseSelectWrap<Self, In0, In1> {
        match (x0, x1) {
            (None, None) => None,
            (None, Some(x1)) => Some(Either::Right((x1, None))),
            (Some(x0), x1) => Some(Either::Left((x0, x1))),
        }
    }
}

impl BaseFlatten for Options {
    fn flatten<T>(x: Self::Wrap<Self::Wrap<T>>) -> Self::Wrap<T> {
        x.flatten()
    }
}

impl BaseIterate for Options {
    fn iterate<F: BaseIterateFn<Self>>(mut f: F) -> Self::Wrap<F::Out> {
        Some(loop {
            match f.run()? {
                Either::Left(next) => f = next,
                Either::Right(x) => break x,
            }
        })
    }
}

impl BaseToEither for Options {
    type L = Sometimes;
    type R = Sometimes;

    fn either<In, Out>(x: Self::Wrap<In>) -> BaseToEitherWrap<Self, In, Out> {
        match x {
            Some(x) => Either::Left((x, Sometimes)),
            None => Either::Right((None, Sometimes)),
        }
    }
}

impl BaseTranspose for Options {
    fn transpose<Wr: ?Sized + BasePure + BaseMap, T>(
        x: Self::Wrap<Wr::Wrap<T>>,
    ) -> Wr::Wrap<Self::Wrap<T>> {
        match x {
            Some(x) => x.b_map(Some),
            None => Wr::pure(None),
        }
    }
}

impl BaseInspect for Options {
    fn inspect<Out, In>(
        x: Self::Wrap<In>,
        f: impl FnOnce(&mut In) -> Self::Wrap<Out>,
    ) -> Self::Wrap<Out> {
        x.and_then(|mut x| f(&mut x))
    }
}
