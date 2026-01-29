use std::{
    error::Error,
    io::{Stdin, Stdout},
    result::Result,
    sync::{Arc, Mutex},
};

use dap::server::Server;

pub(crate) type DynResult<T> = Result<T, Box<dyn Error>>;
pub(crate) type DapServer = Arc<Mutex<Server<Stdin, Stdout>>>;
