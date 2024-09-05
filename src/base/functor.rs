use either::Either;

pub trait BaseUnwrap<Wr: ?Sized + BaseWrap>: Sized
where
    Wr: BaseWrap<Wrap<Self::T> = Self>,
{
    type T;
}

pub trait BaseWrap {
    type Wrap<T>: BaseUnwrap<Self, T = T>;
}

pub trait BasePure: BaseWrap {
    fn pure<T>(_: T) -> Self::Wrap<T>;
}

pub trait BaseMap: BaseWrap {
    fn map<Out, In>(_: Self::Wrap<In>, _: impl FnOnce(In) -> Out) -> Self::Wrap<Out>;
}

pub trait BaseMap2: BaseWrap {
    fn map2<Out, In0, In1>(
        _: Self::Wrap<In0>,
        _: Self::Wrap<In1>,
        _: impl FnOnce(In0, In1) -> Out,
    ) -> Self::Wrap<Out>;
}

pub type BaseEitherWrap<Wr, In0, In1> = <Wr as BaseWrap>::Wrap<
    Either<(In0, <Wr as BaseWrap>::Wrap<In1>), (In1, <Wr as BaseWrap>::Wrap<In0>)>,
>;

pub trait BaseSelect: BaseWrap {
    fn select<In0, In1>(_: Self::Wrap<In0>, _: Self::Wrap<In1>) -> BaseEitherWrap<Self, In0, In1>;
}

pub trait BaseFlatten: BaseWrap {
    fn flatten<T>(_: Self::Wrap<Self::Wrap<T>>) -> Self::Wrap<T>;
}

pub trait BaseToEither: BaseWrap {
    fn either<In, Out>(_: Self::Wrap<In>) -> Either<In, Self::Wrap<Out>>;
}

pub trait BaseTranspose: BaseWrap {
    fn transpose<Wr: ?Sized + BasePure + BaseMap, T>(
        _: Self::Wrap<Wr::Wrap<T>>,
    ) -> Wr::Wrap<Self::Wrap<T>>;
}

pub trait BaseFunctor: BaseMap {}

impl<Wr: ?Sized + BaseMap> BaseFunctor for Wr {}

pub trait BaseApplicative: BaseFunctor + BaseMap2 + BasePure {}

impl<Wr: ?Sized + BaseFunctor + BaseMap2 + BasePure> BaseApplicative for Wr {}

pub trait BaseMonad: BaseApplicative + BaseFlatten {}

impl<Wr: ?Sized + BaseApplicative + BaseFlatten> BaseMonad for Wr {}

pub trait BaseWrappedMapExt<Wr: ?Sized + BaseMap<Wrap<Self::T> = Self>>: BaseUnwrap<Wr> {
    fn b_map<Out>(self, f: impl FnOnce(Self::T) -> Out) -> Wr::Wrap<Out> {
        Wr::map(self, f)
    }
}

impl<Wr: ?Sized + BaseMap<Wrap<Self::T> = Self>, To: BaseUnwrap<Wr>> BaseWrappedMapExt<Wr> for To {}

pub trait BaseWrappedFlattenExt<
    Wr: ?Sized + BaseFlatten<Wrap<T> = Self> + BaseFlatten<Wrap<U> = T>,
    T: BaseUnwrap<Wr, T = U>,
    U,
>: BaseUnwrap<Wr, T = T>
{
    fn b_flatten(self) -> T {
        Wr::flatten::<U>(self)
    }
}

impl<
        Wr: ?Sized + BaseFlatten<Wrap<T> = Self> + BaseFlatten<Wrap<U> = T>,
        T: BaseUnwrap<Wr, T = U>,
        U,
        To: BaseUnwrap<Wr, T = T>,
    > BaseWrappedFlattenExt<Wr, T, U> for To
{
}
