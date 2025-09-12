use std::marker::PhantomData;

use crate::linked::{
    functional::{
        Flatten, FlattenFn, Map, Map2, MapFn, MapFn2, Pure, Select, SelectFn, SelectMap, Transpose,
        TransposeFn, Wrap,
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

impl<Uo: Select, Ui: Map> Select for Composition<Uo, Ui> {
    fn select<In0: ?Sized + Trait, In1: ?Sized + Trait, F: SelectFn<Self, In0, In1>>(
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
        impl<
                In0: ?Sized + Trait,
                In1: ?Sized + Trait,
                F: SelectFn<Composition<Uo, Ui>, In0, In1>,
                Ui: Map,
                Uo: Wrap,
            > SelectFn<Uo, Ui::Wrap<In0>, Ui::Wrap<In1>> for Tmp<F, Ui, In0, In1>
        {
            type Out = Ui::Wrap<F::Out>;

            fn run0(
                self,
                x: impl Impl<Ui::Wrap<In0>>,
                y: impl Impl<Uo::Wrap<Ui::Wrap<In1>>>,
            ) -> impl Impl<Self::Out> {
                Ui::map(x, SelectMap::<Composition<Uo, Ui>, In1>::run0(y, self.0))
            }

            fn run1(
                self,
                x: impl Impl<Ui::Wrap<In1>>,
                y: impl Impl<Uo::Wrap<Ui::Wrap<In0>>>,
            ) -> impl Impl<Self::Out> {
                Ui::map(x, SelectMap::<Composition<Uo, Ui>, In0>::run1(y, self.0))
            }
        }

        Uo::select(
            x0,
            x1,
            Tmp::<F, Ui, In0, In1>(f, PhantomData, PhantomData, PhantomData),
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

impl<Uo: Transpose + Map, Ui: Transpose> Transpose for Composition<Uo, Ui> {
    fn transpose<Wr: ?Sized + Pure + Map, Tr: ?Sized + Trait>(
        x: impl Impl<Self::Wrap<Wr::Wrap<Tr>>>,
    ) -> impl Impl<Wr::Wrap<Self::Wrap<Tr>>> {
        Uo::transpose::<Wr, _>(Uo::map(x, TransposeFn::<Wr, Ui, Tr>))
    }
}
