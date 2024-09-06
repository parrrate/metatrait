use either::Either;

use super::functor::BaseWrap;

pub trait BaseIterateFn<Wr: ?Sized + BaseWrap>: Sized {
    type Out;
    fn run(self) -> Wr::Wrap<Either<Self, Self::Out>>;
}
