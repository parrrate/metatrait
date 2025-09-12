use std::marker::PhantomData;

use either::Either;
use metatrait::{
    base::functor::*,
    existence::{Never, Sometimes},
};

struct Phantoms;

impl<T> BaseUnwrap<PhantomData<T>> for Phantoms {
    type T = T;
}

impl BaseWrap for Phantoms {
    type Wrap<T> = PhantomData<T>;
}

impl BasePure for Phantoms {
    fn pure<T>(_: T) -> Self::Wrap<T> {
        PhantomData
    }
}

impl BaseMap for Phantoms {
    fn map<Out, In>(_: Self::Wrap<In>, _: impl FnOnce(In) -> Out) -> Self::Wrap<Out> {
        PhantomData
    }
}

impl BaseMap2 for Phantoms {
    fn map2<Out, In0, In1>(
        _: Self::Wrap<In0>,
        _: Self::Wrap<In1>,
        _: impl FnOnce(In0, In1) -> Out,
    ) -> Self::Wrap<Out> {
        PhantomData
    }
}

impl BaseSelect for Phantoms {
    fn select<In0, In1>(_: Self::Wrap<In0>, _: Self::Wrap<In1>) -> BaseSelectWrap<Self, In0, In1> {
        PhantomData
    }
}

impl BaseFlatten for Phantoms {
    fn flatten<T>(_: Self::Wrap<Self::Wrap<T>>) -> Self::Wrap<T> {
        PhantomData
    }
}

impl BaseToEither for Phantoms {
    type L = Never;
    type R = Sometimes;

    fn either<In, Out>(_: Self::Wrap<In>) -> Either<(In, Self::L), (Self::Wrap<Out>, Self::R)> {
        Either::Right((PhantomData, Sometimes))
    }
}

fn main() {}
