use crate::linked::{Impl, Trait};

pub struct Is<That, Tr: ?Sized>(That, Tr);

impl<That, Tr: ?Sized + Trait> Trait for Is<That, Tr> {
    type Assocaited = Tr;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = That;
}

impl<That: Impl<Tr>, Tr: ?Sized + Trait> Impl<Is<Self, Tr>> for That {
    type Associated = Self;

    fn method<'out: 'tmp, 'tmp>(this: Self) -> Self
    where
        Self: 'tmp,
    {
        this
    }
}

pub trait IsExt<Tr: ?Sized + Trait> {
    fn into_that<That>(self) -> That
    where
        Self: Impl<Is<That, Tr>>,
    {
        Self::method(self)
    }
}

impl<That: Impl<Tr>, Tr: ?Sized + Trait> IsExt<Tr> for That {}
