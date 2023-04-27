//! This is a crate for usual util for `Future`s, like <strong>retry</strong> and <strong>delay</strong>.

/// A Future can be cloned via https://docs.rs/futures/latest/futures/future/trait.FutureExt.html#method.shared.
/// This is useful to pass the future to multiple consumers, but not suitable for retry.
///
/// To have retries with Futures, you need some kind of Future factory, to create a new Future for a retry
/// when an error occurs. Ideally this retry mechanism would be wrapped in its own Future, to hide the complexity for consumers.
///
/// If not, it's doesn't work:
/// ```
/// use std::error::Error;
/// use std::future::Future;
/// use std::pin::Pin;
/// use tokio::time::sleep;
/// use tokio::time::Duration;
///
/// fn retry_with_times(
///     mut retry_times: i32,
///     duration: Duration,
///     task: impl Future<Output = Result<String, Box<dyn Error>>> + Send + Clone,
/// ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error>>> + Send>> {
///     Box::pin(async move {
///         retry_times -= 1;
///         match task.await {
///             Ok(value) => Ok(value),
///             Err(err) => {
///                 if retry_times <= 0 {
///                     return Err(err);
///                 }
///                 let _ = sleep(duration).await;
///                 retry_with_times(retry_times, duration, task).await
///             }
///         }
///     })
/// }
/// ```
///
/// There's a crate which does that already: https://docs.rs/futures-retry/latest/futures_retry/struct.FutureRetry.html
pub fn retry_with_times() {}
