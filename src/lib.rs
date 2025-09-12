use either::Either;

pub mod cat;
pub mod traits;

pub trait Trait {
    type Assocaited: ?Sized + Trait;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>>;
    type Out<'out, Imp: Impl<Self>>;
    type Sample: Impl<Self>;

    fn union(_: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self>;
}

pub trait Impl<Tr: ?Sized + Trait>: Sized {
    type Associated: Impl<Tr::Assocaited>;
    fn method<'out: 'tmp, 'tmp>(_: Tr::In<'out, 'tmp, Self>) -> Tr::Out<'out, Self>
    where
        Self: 'tmp;
}

pub type Sample<Tr> = <Tr as Trait>::Sample;
