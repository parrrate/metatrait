use either::Either;
use metatrait::{
    cat::{
        functor::*,
        instances::verbatim::Verbatim,
        morphism::*,
        util::{IsToEither, Wraps},
    },
    traits::{
        either::IntoEither,
        is::{Is, IsExt},
    },
    Impl, Trait,
};

trait Fetch {
    type T;
    type Wr: ?Sized + Map + Pure + Iterate;
    fn fetch(self) -> impl Wraps<Self::Wr, Self::T>;
}

struct BoxFetch<T>(Box<Stack<T, Self>>);

impl<T> Fetch for BoxFetch<T> {
    type T = Stack<T, Self>;
    type Wr = Verbatim;

    fn fetch(self) -> impl Wraps<Self::Wr, Self::T> {
        *self.0
    }
}

struct Stack<T, F: Fetch<T = Self>> {
    _value: T,
    next: Option<F>,
}

struct Count<F>(usize, F);

impl<F: Fetch> TraitFn for Count<F> {
    type Out = Is<usize>;
}

impl<T, F: Fetch<T = Stack<T, F>>> IterateFn<F::Wr> for Count<F> {
    fn run(self) -> impl Impl<<F::Wr as Wrap>::Wrap<IntoEither<Self, Self::Out>>> {
        self.1
            .fetch()
            .w_map(move |x: Stack<T, F>| match x.next {
                Some(f) => Either::Left(Self(self.0 + 1, f)),
                None => Either::Right(self.0 + 1),
            })
            .w_map(IsToEither)
    }
}

impl<T, F: Fetch<T = Self>> Stack<T, F> {
    fn count(self) -> impl Wraps<F::Wr, usize> {
        Trait::union(if let Some(next) = self.next {
            Either::Left(F::Wr::iterate(Count(1, next)))
        } else {
            Either::Right(F::Wr::pure(1))
        })
    }
}

fn main() {
    let stack = Stack {
        _value: "123",
        next: Some(BoxFetch(Box::new(Stack {
            _value: "456",
            next: None,
        }))),
    };
    let count = stack.count().into_that();
    println!("{count}");
}
