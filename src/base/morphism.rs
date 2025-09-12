use either::Either;

use super::functor::*;

pub trait BaseIterateFn<Wr: ?Sized + BaseWrap>: Sized {
    type Out;
    fn run(self) -> Wr::Wrap<Either<Self, Self::Out>>;
}

pub trait BaseIterateFnExt<Wr: ?Sized + BaseIterate>: BaseIterateFn<Wr> {
    fn run_iterate(self) -> Wr::Wrap<Self::Out> {
        Wr::iterate(self)
    }
}

impl<Wr: ?Sized + BaseIterate, F: BaseIterateFn<Wr>> BaseIterateFnExt<Wr> for F {}
