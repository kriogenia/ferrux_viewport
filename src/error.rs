use thiserror::Error;

#[derive(Error, Debug)]
pub enum ViewportError {
    #[error("error ocurred while rendering")]
    Rendering
}