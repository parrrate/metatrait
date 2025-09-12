use std::marker::PhantomData;

use either::Either;

use crate::linked::{
    functional::{
        BaseFn, Flatten, FlattenFn, Map, Map2, MapFn, MapFn2, Pure, Select, SelectFn, SelectMap01,
        SelectMap10, Transpose, TransposeFn, Wrap,
    },
    Impl, Trait,
};

pub struct Composition<Uo, Ui>(Uo, Ui);

impl<Uo: Wrap, Ui: Wrap> Wrap for Composition<Uo, Ui> {
    type Wrap<Tr: ?Sized + Trait> = Uo::Wrap<Ui::Wrap<Tr>>;
}

impl<Uo: Pure, Ui: Pure> Pure for Composition<Uo, Ui> {
    fn pure<Tr: ?Sized + Trait>(x: impl Impl<Tr>) -> impl Impl<Self::Wrap<Tr>> {
        Uo::pure(Ui::pure(x))
    }
}

impl<Uo: Map, Ui: Map> Map for Composition<Uo, Ui> {
    fn map<In: ?Sized + Trait, F: MapFn<In>>(
        x: impl Impl<Self::Wrap<In>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        struct Tmp<F, Ui, In: ?Sized>(F, PhantomData<Ui>, PhantomData<In>);
        impl<In: ?Sized + Trait, F: MapFn<In>, Ui: Map> MapFn<Ui::Wrap<In>> for Tmp<F, Ui, In> {
            type Out = Ui::Wrap<F::Out>;

            fn run(self, x: impl Impl<Ui::Wrap<In>>) -> impl Impl<Self::Out> {
                Ui::map(x, self.0)
            }
        }

        Uo::map(x, Tmp::<F, Ui, In>(f, PhantomData, PhantomData))
    }
}

impl<Uo: Map2, Ui: Map2> Map2 for Composition<Uo, Ui> {
    fn map2<In0: ?Sized + Trait, In1: ?Sized + Trait, F: MapFn2<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        struct Tmp<F, Ui, In0: ?Sized, In1: ?Sized>(
            F,
            PhantomData<Ui>,
            PhantomData<In0>,
            PhantomData<In1>,
        );
        impl<In0: ?Sized + Trait, In1: ?Sized + Trait, F: MapFn2<In0, In1>, Ui: Map2>
            MapFn2<Ui::Wrap<In0>, Ui::Wrap<In1>> for Tmp<F, Ui, In0, In1>
        {
            type Out = Ui::Wrap<F::Out>;

            fn run(
                self,
                x0: impl Impl<Ui::Wrap<In0>>,
                x1: impl Impl<Ui::Wrap<In1>>,
            ) -> impl Impl<Self::Out> {
                Ui::map2(x0, x1, self.0)
            }
        }

        Uo::map2(
            x0,
            x1,
            Tmp::<F, Ui, In0, In1>(f, PhantomData, PhantomData, PhantomData),
        )
    }
}

impl<Uo: Select, Ui: Map + Transpose + Pure> Select for Composition<Uo, Ui> {
    fn select<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<In0, In1>>(
        x0: impl Impl<Self::Wrap<In0>>,
        x1: impl Impl<Self::Wrap<In1>>,
        f: F,
    ) -> impl Impl<Self::Wrap<F::Out>> {
        struct Tmp<F, Uo, Ui, In0: ?Sized, In1: ?Sized>(
            F,
            PhantomData<Uo>,
            PhantomData<Ui>,
            PhantomData<In0>,
            PhantomData<In1>,
        );
        impl<
                In0: ?Sized + Trait,
                In1: ?Sized + Trait,
                F: SelectFn<In0, In1>,
                Uo: Wrap,
                Ui: Map,
            > BaseFn for Tmp<F, Uo, Ui, In0, In1>
        {
            type Out = Ui::Wrap<F::Out>;
        }
        impl<
                In0: ?Sized + Trait,
                In1: ?Sized + Trait,
                F: SelectFn<In0, In1>,
                Uo: Wrap,
                Ui: Map + Transpose + Pure,
            > SelectFn<Ui::Wrap<In0>, Ui::Wrap<In1>> for Tmp<F, Uo, Ui, In0, In1>
        {
            type Tr0 = F::Tr0;
            type Tr1 = F::Tr1;

            fn run0(
                self,
                x: impl Impl<Ui::Wrap<In0>>,
            ) -> Either<impl Impl<Self::Out>, impl Impl<Self::Tr0>> {
                match Ui::either(x) {
                    Either::Left(x) => match self.0.run0(x) {
                        Either::Left(x) => Either::Left(Either::Left(Ui::pure(x))),
                        Either::Right(x) => Either::Right(x),
                    },
                    Either::Right(x) => Either::Left(Either::Right(x)),
                }
                .map_left(Trait::union)
            }

            fn run1(
                self,
                x: impl Impl<Ui::Wrap<In1>>,
            ) -> Either<impl Impl<Self::Out>, impl Impl<Self::Tr1>> {
                match Ui::either(x) {
                    Either::Left(x) => match self.0.run1(x) {
                        Either::Left(x) => Either::Left(Either::Left(Ui::pure(x))),
                        Either::Right(x) => Either::Right(x),
                    },
                    Either::Right(x) => Either::Left(Either::Right(x)),
                }
                .map_left(Trait::union)
            }

            fn run01(x: impl Impl<Self::Tr0>, y: impl Impl<Ui::Wrap<In1>>) -> impl Impl<Self::Out> {
                Ui::map(y, SelectMap01::<_, F, In0>::new(x))
            }

            fn run10(x: impl Impl<Self::Tr1>, y: impl Impl<Ui::Wrap<In0>>) -> impl Impl<Self::Out> {
                Ui::map(y, SelectMap10::<_, F, In1>::new(x))
            }
        }

        Uo::select(
            x0,
            x1,
            Tmp::<F, Uo, Ui, In0, In1>(f, PhantomData, PhantomData, PhantomData, PhantomData),
        )
    }
}

impl<Uo: Flatten + Map + Pure, Ui: Flatten + Transpose> Flatten for Composition<Uo, Ui> {
    fn flatten<Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Self::Wrap<Tr>>>,
    ) -> impl Impl<Self::Wrap<Tr>> {
        Uo::map(
            Uo::flatten(Uo::map(x, TransposeFn::<Uo, Ui, Ui::Wrap<Tr>>)),
            FlattenFn::<Ui, Tr>,
        )
    }
}

impl<Uo: Transpose + Map + Pure, Ui: Transpose> Transpose for Composition<Uo, Ui> {
    fn either<In: ?Sized + Trait, Out: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<In>>,
    ) -> Either<impl Impl<In>, impl Impl<Self::Wrap<Out>>> {
        match Uo::either::<Ui::Wrap<In>, Ui::Wrap<Out>>(x) {
            Either::Left(x) => match Ui::either::<In, Out>(x) {
                Either::Left(x) => Either::Left(x),
                Either::Right(x) => Either::Right(Either::Left(Uo::pure::<Ui::Wrap<Out>>(x))),
            },
            Either::Right(x) => Either::Right(Either::Right(x)),
        }
        .map_right(Trait::union)
    }

    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>> {
        Uo::transpose::<Wr, _>(Uo::map(x, TransposeFn::<Wr, Ui, Tr>))
    }
}

#[cfg(test)]
mod test {
    use crate::linked::{
        functional::instances::{futures::Futures, lazy::Lazy},
        traits::{
            empty::Empty,
            future::ToFutureExt,
            is::{Is, IsExt},
            to::ToExt,
        },
    };

    use super::*;

    #[test]
    fn test() {
        type U = Composition<Futures, Lazy>;
        let x = U::pure::<Is<_, Empty>>(0);
        let x = U::map(x, |x| x + 1);
        let x = U::map(x, |x| x + 1);
        let x = U::map(x, |x| x + 1);
        let x = U::map(x, |x| x + 1);
        let x = U::map(x, |x| x + 1);
        let x = x.to_future();
        let x = futures::executor::block_on(x);
        let x = x.to();
        let x = x.into_that();
        assert_eq!(x, 5);
    }
}
