//! [`Service`]s that return responses without wrapping other [`Service`]s.
//!
//! These kinds of services are also referred to as "leaf services" since they sit at the leaves of
//! a [tree] of services.
//!
//! [`Service`]: https://docs.rs/tower/latest/tower/trait.Service.html
//! [tree]: https://en.wikipedia.org/wiki/Tree_(data_structure)

#[cfg(feature = "redirect")]
pub mod redirect;

#[cfg(feature = "redirect")]
#[doc(inline)]
pub use self::redirect::Redirect;

#[cfg(feature = "fs")]
#[cfg(not(target_arch = "wasm32"))]
pub mod fs;

#[cfg(feature = "fs")]
#[cfg(not(target_arch = "wasm32"))]
#[doc(inline)]
pub use self::fs::{ServeDir, ServeFile};
