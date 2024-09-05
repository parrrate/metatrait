use either::Either;

pub trait BaseUnwrap<T>: BaseWrap<Wrap<Self::T> = T> {
    type T;
}

pub trait BaseWrapped<Wr: ?Sized + BaseWrap>: Sized
where
    Wr: BaseWrap<Wrap<Self::T> = Self>,
{
    type T;
}

impl<Wr: ?Sized + BaseUnwrap<T>, T> BaseWrapped<Wr> for T {
    type T = <Wr as BaseUnwrap<T>>::T;
}

pub trait BaseWrap {
    type Wrap<T>: BaseWrapped<Self, T = T>;
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

pub type BaseSelectWrap<Wr, In0, In1> = <Wr as BaseWrap>::Wrap<
    Either<(In0, <Wr as BaseWrap>::Wrap<In1>), (In1, <Wr as BaseWrap>::Wrap<In0>)>,
>;

pub trait BaseSelect: BaseWrap {
    fn select<In0, In1>(_: Self::Wrap<In0>, _: Self::Wrap<In1>) -> BaseSelectWrap<Self, In0, In1>;
}

pub trait BaseFlatten: BaseWrap {
    fn flatten<T>(_: Self::Wrap<Self::Wrap<T>>) -> Self::Wrap<T>;
}

pub type BaseToEitherWrap<Wr, In, Out> =
    Either<(In, <Wr as BaseToEither>::L), (<Wr as BaseWrap>::Wrap<Out>, <Wr as BaseToEither>::R)>;

pub trait BaseToEither: BaseWrap {
    type L;
    type R;
    fn either<In, Out>(_: Self::Wrap<In>) -> BaseToEitherWrap<Self, In, Out>;
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

pub trait BaseWrappedMapExt<Wr: ?Sized + BaseMap<Wrap<Self::T> = Self>>: BaseWrapped<Wr> {
    fn b_map<Out>(self, f: impl FnOnce(Self::T) -> Out) -> Wr::Wrap<Out> {
        Wr::map(self, f)
    }
}

impl<Wr: ?Sized + BaseMap<Wrap<Self::T> = Self>, To: BaseWrapped<Wr>> BaseWrappedMapExt<Wr> for To {}

pub trait BaseWrappedFlattenExt<
    Wr: ?Sized + BaseFlatten<Wrap<Ti> = Self> + BaseFlatten<Wrap<T> = Ti>,
    Ti: BaseWrapped<Wr, T = T>,
    T,
>: BaseWrapped<Wr, T = Ti>
{
    fn b_flatten(self) -> Ti {
        Wr::flatten::<T>(self)
    }
}

impl<
        Wr: ?Sized + BaseFlatten<Wrap<Ti> = Self> + BaseFlatten<Wrap<T> = Ti>,
        Ti: BaseWrapped<Wr, T = T>,
        T,
        To: BaseWrapped<Wr, T = Ti>,
    > BaseWrappedFlattenExt<Wr, Ti, T> for To
{
}
