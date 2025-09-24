pub type BoxError = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Generic(BoxError),
}

impl Error {
    pub fn as_error(self) -> BoxError {
        match self {
            Error::Generic(err) => err,
        }
    }
}

use std::ops::ControlFlow;

impl<T> std::ops::Try for std::result::Result<T, Error> {
    type Output = T;
    type Residual = std::result::Result<std::convert::Infallible, Error>;

    fn from_output(output: Self::Output) -> Self {
        Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Ok(v) => ControlFlow::Continue(v),
            Err(e) => ControlFlow::Break(Error::Generic(e)),
        }
    }
}
