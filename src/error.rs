//! Package with the errors that can occur using the library

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ViewportError {
	#[error("no adapter was found to manage the rendering")]
	AdapterNotFound,
    #[error("error ocurred while rendering")]
    Rendering
}