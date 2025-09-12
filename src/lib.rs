#[doc(no_inline)]
pub use either::Either;

pub mod base;
pub mod cat;
pub mod existence;
pub mod traits;

pub trait Trait {
    type Assocaited: ?Sized + Trait;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>>;
    type Out<'out, Imp: Impl<Self>>;
    type Sample: Impl<Self>;
    type Common<'a>: 'a + Impl<Self>
    where
        Self: 'a;

    fn union(_: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self>;
    fn common<'a>(_: impl 'a + Impl<Self>) -> Self::Common<'a>
    where
        Self: 'a;
}

pub trait Impl<Tr: ?Sized + Trait>: Sized {
    type Associated: Impl<Tr::Assocaited>;
    fn method<'out: 'tmp, 'tmp>(_: Tr::In<'out, 'tmp, Self>) -> Tr::Out<'out, Self>
    where
        Self: 'tmp;
}

pub type Sample<Tr> = <Tr as Trait>::Sample;
pub type Common<'a, Tr> = <Tr as Trait>::Common<'a>;

pub trait Structural: Trait {
    type Free: Impl<Self>;
    fn free(_: impl Impl<Self>) -> Self::Free;
}

pub trait StructuralExt<Tr: ?Sized + Structural>: Impl<Tr> {
    fn free(self) -> Tr::Free {
        Tr::free(self)
    }
}

impl<Tr: ?Sized + Structural, T: Impl<Tr>> StructuralExt<Tr> for T {}
