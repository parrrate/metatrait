use std::marker::PhantomData;

use either::Either;

use crate::{base::functor::*, Impl, Trait};

pub struct Base<Wr: ?Sized, Tr: ?Sized>(PhantomData<Wr>, Tr);

impl<Wr: ?Sized + BaseMap, Tr: ?Sized + Trait> Trait for Base<Wr, Tr> {
    type Assocaited = Tr;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + crate::Impl<Self>> = Imp;
    type Out<'out, Imp: crate::Impl<Self>> = Wr::Wrap<Imp::Associated>;
    type Sample = Wr::Wrap<Tr::Sample>;
    type Common<'a> = Wr::Wrap<Tr::Common<'a>> where Self: 'a;

    fn union(x: Either<impl crate::Impl<Self>, impl crate::Impl<Self>>) -> impl crate::Impl<Self> {
        match x {
            Either::Left(x) => x.into_base().b_map(Either::Left),
            Either::Right(x) => x.into_base().b_map(Either::Right),
        }
        .b_map(Trait::union)
    }

    fn common<'a>(x: impl 'a + Impl<Self>) -> Self::Common<'a>
    where
        Self: 'a,
    {
        x.into_base().b_map(Trait::common)
    }
}

impl<Wr: ?Sized + BaseMap<Wrap<T::T> = T>, Tr: ?Sized + Trait, T: BaseWrapped<Wr, T: Impl<Tr>>>
    Impl<Base<Wr, Tr>> for T
{
    type Associated = T::T;

    fn method<'out: 'tmp, 'tmp>(
        x: <Base<Wr, Tr> as Trait>::In<'out, 'tmp, Self>,
    ) -> <Base<Wr, Tr> as Trait>::Out<'out, Self>
    where
        Self: 'tmp,
    {
        x
    }
}

pub trait BaseExt<Tr: ?Sized + Trait> {
    fn into_base<Wr: ?Sized + BaseMap>(self) -> Wr::Wrap<Self::Associated>
    where
        Self: Impl<Base<Wr, Tr>>,
    {
        Self::method(self)
    }
}

impl<That, Tr: ?Sized + Trait> BaseExt<Tr> for That {}
