use std::{error::Error, result::Result};

pub(crate) type DynResult<T> = Result<T, Box<dyn Error>>;
