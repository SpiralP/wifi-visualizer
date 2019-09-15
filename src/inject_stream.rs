// use futures::prelude::*;

// pub struct InjectBeforeErrorStream<S, F>
// where
//   S: Stream,
//   F: FnMut(&S::Error) -> S::Item,
// {
//   stream: S,
//   f: F,
//   error: Option<S::Error>,
// }

// #[allow(clippy::use_self)]
// impl<S, F> InjectBeforeErrorStream<S, F>
// where
//   S: Stream,
//   F: FnMut(&S::Error) -> S::Item,
// {
//   pub fn new(s: S, f: F) -> InjectBeforeErrorStream<S, F> {
//     Self {
//       stream: s,
//       f,
//       error: None,
//     }
//   }
// }

// impl<S, F> Stream for InjectBeforeErrorStream<S, F>
// where
//   S: Stream,
//   F: FnMut(&S::Error) -> S::Item,
// {
//   type Item = S::Item;
//   type Error = S::Error;

//   fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
//     if let Some(e) = self.error.take() {
//       Err(e)
//     } else {
//       match self.stream.poll() {
//         Err(e) => {
//           let item = (self.f)(&e);
//           self.error = Some(e);

//           Ok(Async::Ready(Some(item)))
//         }
//         other => other,
//       }
//     }
//   }
// }

// pub trait InjectBeforeErrorStreamExt<F>: Stream
// where
//   F: FnMut(&Self::Error) -> Self::Item,
//   Self: Sized,
// {
//   fn inject_before_error(self, f: F) -> InjectBeforeErrorStream<Self, F> {
//     InjectBeforeErrorStream::new(self, f)
//   }
// }

// impl<S, F> InjectBeforeErrorStreamExt<F> for S
// where
//   S: Stream,
//   F: FnMut(&S::Error) -> S::Item,
//   Self: Sized,
// {
// }
