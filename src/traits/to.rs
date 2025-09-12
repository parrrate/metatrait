use either::Either;

use crate::{Impl, Trait};

pub struct To<Tr: ?Sized>(Tr);

impl<Tr: ?Sized + Trait> Trait for To<Tr> {
    type Assocaited = Tr;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> = Imp;
    type Out<'out, Imp: Impl<Self>> = Imp::Associated;
    type Sample = fn() -> Tr::Sample;

    fn union(x: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {
        || Tr::union(x.map_left(ToExt::to).map_right(ToExt::to))
    }
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
