use futures::{prelude::*, ready};
use pin_utils::{unsafe_pinned, unsafe_unpinned};
use std::{
  pin::Pin,
  task::{Context, Poll},
};

#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct InjectBeforeErrorStream<St, F>
where
  St: TryStream,
{
  stream: St,
  f: F,
  error: Option<St::Error>,
}

// impl<St: Unpin, F> Unpin for InjectBeforeErrorStream<St, F> {}

#[allow(clippy::use_self)]
impl<St, F> InjectBeforeErrorStream<St, F>
where
  St: TryStream,
  F: FnMut(&St::Error) -> St::Ok,
{
  unsafe_pinned!(stream: St);
  unsafe_unpinned!(f: F);
  unsafe_unpinned!(error: Option<St::Error>);

  pub fn new(s: St, f: F) -> InjectBeforeErrorStream<St, F> {
    Self {
      stream: s,
      f,
      error: None,
    }
  }
}

impl<St, F> Stream for InjectBeforeErrorStream<St, F>
where
  St: TryStream,
  F: FnMut(&St::Error) -> St::Ok,
{
  type Item = Result<St::Ok, St::Error>;

  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<St::Ok, St::Error>>> {
    if let Some(e) = self.as_mut().error().take() {
      Poll::Ready(Some(Err(e)))
    } else {
      match ready!(self.as_mut().stream().try_poll_next(cx)) {
        None => Poll::Ready(None),
        Some(item) => match item {
          Err(e) => {
            let item = (self.as_mut().f())(&e);
            *self.as_mut().error() = Some(e);

            Poll::Ready(Some(Ok(item)))
          }
          other => Poll::Ready(Some(other)),
        },
      }
    }
  }
}

pub trait InjectBeforeErrorTryStreamExt: TryStream {
  fn inject_before_error<F>(self, f: F) -> InjectBeforeErrorStream<Self, F>
  where
    F: FnMut(&Self::Error) -> Self::Ok,
    Self: Sized,
  {
    InjectBeforeErrorStream::new(self, f)
  }
}

impl<T: ?Sized> InjectBeforeErrorTryStreamExt for T where T: TryStream {}

#[test]
fn test_inject_before_error_stream() {
  futures::executor::block_on(async {
    let items: Vec<Result<u8, String>> = vec![Ok(2), Ok(4), Ok(6), Err("bap".to_string())];

    let st = stream::iter(items);
    st.inject_before_error(|e| {
      println!("err {:?}", e);

      0
    })
    .map_ok(|n| {
      println!("process {}", n);
    })
    .map_err(|e| {
      println!("errrr {:?}", e);
    })
    .for_each(|_| future::ready(()))
    .await;
  });
}
