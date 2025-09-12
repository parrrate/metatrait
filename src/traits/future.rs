use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use either::Either;
use futures::future::Pending;
use pin_project::pin_project;

use crate::{Impl, Trait};

pub struct ToFuture<Tr: ?Sized>(Tr);

impl<Tr: ?Sized + Trait> Trait for ToFuture<Tr> {
    type Assocaited = Tr;
    type In<'out: 'tmp, 'tmp, Imp: 'tmp + Impl<Self>> =
        (Pin<&'tmp mut Imp>, &'tmp mut Context<'out>);
    type Out<'out, Imp: Impl<Self>> = Poll<Imp::Associated>;
    type Sample = Pending<Tr::Sample>;

    fn union(x: Either<impl Impl<Self>, impl Impl<Self>>) -> impl Impl<Self> {
        async {
            Tr::union(match x {
                Either::Left(x) => Either::Left(x.to_future().await),
                Either::Right(x) => Either::Right(x.to_future().await),
            })
        }
    }
}

impl<F: Future<Output = Out>, Out: Impl<Tr>, Tr: ?Sized + Trait> Impl<ToFuture<Tr>> for F {
    type Associated = F::Output;

    fn method<'out: 'tmp, 'tmp>((this, cx): (Pin<&mut Self>, &mut Context<'_>)) -> Poll<F::Output> {
        this.poll(cx)
    }
}

#[pin_project]
struct TraitFuture<F, Tr: ?Sized>(#[pin] F, PhantomData<Tr>);

impl<F: Impl<ToFuture<Tr>>, Tr: ?Sized + Trait> Future for TraitFuture<F, Tr> {
    type Output = F::Associated;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        F::method((self.project().0, cx))
    }
}

pub trait ToFutureExt<Tr: ?Sized + Trait>: Impl<ToFuture<Tr>> {
    fn to_future(self) -> impl Future<Output = Self::Associated> {
        TraitFuture(self, PhantomData)
    }
}

impl<F: Impl<ToFuture<Tr>>, Tr: ?Sized + Trait> ToFutureExt<Tr> for F {}
