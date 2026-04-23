//! Item image proxy.
//!
//! POE2 item art lives at `cdn.poe2db.tw/image/Art/2DItems/...webp`, which
//! requires a `Referer: https://poe2db.tw/` header — browser `<img>` tags
//! won't send that for us. We proxy through the backend: fetch once with
//! the right Referer, cache on disk, and return a `data:` URL the frontend
//! can drop into an `<img src>` directly.
//!
//! The `web.poecdn.com/image/Art/2DItems/.../*.png` URLs already embedded
//! in item records are the input — we rewrite to poe2db's CDN path +
//! `.webp` extension internally.

pub mod commands;
pub mod service;
pub mod traits;

pub use commands::*;
pub use service::ItemImageServiceImpl;
pub use traits::ItemImageService;
