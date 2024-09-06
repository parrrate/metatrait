pub trait From2<T>: Sized {
    fn from2(value: T) -> Self;
}

impl<U, T: Into<U>> From2<T> for U {
    fn from2(value: T) -> Self {
        value.into()
    }
}

pub trait Into2<T>: Sized {
    fn into2(self) -> T;
}

impl<U, T: From2<U>> Into2<T> for U {
    fn into2(self) -> T {
        T::from2(self)
    }
}

pub trait When: Into<Sometimes> + From<Never> + From<(Sometimes, Self)> {
    type And<E: When>: When + Into<Self> + Into<E> + From<(Self, E)>;
    type Or<E: When>: When + From2<Self> + From2<E>;
}

pub struct Sometimes;
pub enum Never {}

impl From<Never> for Sometimes {
    fn from(x: Never) -> Self {
        match x {}
    }
}

impl From<(Sometimes, Sometimes)> for Sometimes {
    fn from(_: (Sometimes, Sometimes)) -> Self {
        Self
    }
}

impl From<(Sometimes, Never)> for Never {
    fn from((_, x): (Sometimes, Never)) -> Self {
        x
    }
}

impl<E> From<(Never, E)> for Never {
    fn from((x, _): (Never, E)) -> Self {
        x
    }
}

impl When for Sometimes {
    type And<E: When> = E;
    type Or<E: When> = Self;
}

impl When for Never {
    type And<E: When> = Self;
    type Or<E: When> = E;
}
