use std::{
    error::Error,
    io::{Empty, Stdout},
    result::Result,
};

use dap::server::Server;

pub(crate) type DynResult<T> = Result<T, Box<dyn Error>>;
pub(crate) type DapServerOut = Server<Empty, Stdout>;
