pub mod functional;
pub mod traits;

pub trait Trait {
    type Assocaited: ?Sized + Trait;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>>;
    type Out<'out, Imp: Impl<Self>>;
}

pub trait Impl<Tr: ?Sized + Trait>: Sized {
    type Associated: Impl<Tr::Assocaited>;
    fn method<'out: 'tmp, 'tmp>(_: Tr::In<'out, 'tmp, Self>) -> Tr::Out<'out, Self>
    where
        Self: 'tmp;
}
