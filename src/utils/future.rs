use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Wrap a future that is not `Send` and make it `Send`
pub struct SendFuture<F>(pub F);

unsafe impl<F> Send for SendFuture<F> {}

impl<F> Future for SendFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: Since we're in a single-threaded WASM environment, this is safe.
        unsafe {
            let this = self.get_unchecked_mut();
            Pin::new_unchecked(&mut this.0).poll(cx)
        }
    }
}
