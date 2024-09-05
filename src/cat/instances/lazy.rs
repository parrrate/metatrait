use either::Either;

use crate::{
    cat::{functor::*, morphism::*},
    traits::{
        is::IsExt,
        to::{To, ToExt},
    },
    Impl, Trait,
};

pub struct Lazy;

impl<Tr: ?Sized + Trait> Unwrap<Lazy> for To<Tr> {
    type Tr = Tr;
}

impl Wrap for Lazy {
    type Wrap<Tr: ?Sized + Trait> = To<Tr>;
}

impl Pure for Lazy {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>> {
        || x
    }
}

impl Map for Lazy {
    fn map<F: MapFn<In>, In: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        || f.run(x.to())
    }
}

impl Map2 for Lazy {
    fn map2<F: MapFn2<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        || f.run(x0.to(), x1.to())
    }
}

impl Select for Lazy {
    fn select<F: SelectFn<In0, In1>, In0: ?Sized + Trait, In1: ?Sized + Trait>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        || Trait::union(f.run0(x0.to()).map_right(|x0| F::run01(x0, x1.to())))
    }
}

impl Flatten for Lazy {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>> {
        || x.to().to()
    }
}

impl Iterate for Lazy {
    fn iterate<F: IterateFn<Self>>(mut f: F) -> impl Impl<Self::Wrap<F::Out>> {
        || loop {
            match f.done() {
                Either::Left(x) => break x,
                Either::Right(next) => f = next.to().into_that(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::traits::is::{Is, IsExt};

    use super::*;

    #[test]
    fn test() {
        let x = Lazy::pure::<Is<_>>(0);
        let x = Lazy::map(x, |x| x + 1);
        let x = Lazy::map(x, |x| x + 1);
        let x = Lazy::map(x, |x| x + 1);
        let x = Lazy::map(x, |x| x + 1);
        let x = Lazy::map(x, |x| x + 1);
        let x = x.to();
        let x = x.into_that();
        assert_eq!(x, 5);
    }
}
