use crate::linked::{Impl, Trait};

pub struct To<Tr: ?Sized>(Tr);

impl<Tr: ?Sized + Trait> Trait for To<Tr> {
    type Assocaited = Tr;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = Imp::Associated;
}

impl<F: FnOnce() -> Out, Out: Impl<Tr>, Tr: ?Sized + Trait> Impl<To<Tr>> for F {
    type Associated = Out;

    fn method<'out: 'tmp, 'tmp>(
        this: <To<Tr> as Trait>::In<'out, 'tmp, Self>,
    ) -> <To<Tr> as Trait>::Out<'out, Self>
    where
        Self: 'tmp,
    {
        this()
    }
}

pub trait ToExt<Tr: ?Sized + Trait> {
    type Output: Impl<Tr>;
    fn to(self) -> Self::Output;
}

impl<F: Impl<To<Tr>>, Tr: ?Sized + Trait> ToExt<Tr> for F {
    type Output = F::Associated;

    fn to(self) -> Self::Output {
        Self::method(self)
    }
}
