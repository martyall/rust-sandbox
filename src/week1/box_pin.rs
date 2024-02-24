use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Implementation must be meaningful, and
        // obviously call something requiring `&mut self`.
        // The point here is to practice dealing with
        // `Pin<&mut Self>` -> `&mut self` conversion
        // in different contexts, without introducing
        // any `Unpin` trait bounds.
    }
}

pub struct MeasurableFuture<Fut> {
    pub inner_future: Fut,
    pub started_at: Option<std::time::Instant>,
}

impl<Fut: Unpin> MutMeSomehow for MeasurableFuture<Fut> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let mf = self.get_mut();
        mf.started_at = Some(Instant::now());
    }
}

impl<Fut: Future + Unpin> Future for MeasurableFuture<Fut> {
    type Output = <Fut as Future>::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner: &mut Fut = &mut self.get_mut().inner_future;
        Pin::new(inner).poll(cx)
    }
}
