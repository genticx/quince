#[cfg(not(feature = "wasm-bindgen"))]
mod native;
#[cfg(feature = "wasm-bindgen")]
mod wasm;

use super::{PinResponse, PinataClient, Result};
use serde::Serialize;

#[cfg(not(feature = "wasm-bindgen"))]
pub use native::*;
#[cfg(feature = "wasm-bindgen")]
pub use wasm::*;
