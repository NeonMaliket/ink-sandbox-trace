use std::{
    error::Error,
    io::{Empty, Stdout},
    result::Result,
    sync::{Arc, Mutex},
};

use dap::server::Server;

pub(crate) type DynResult<T> = Result<T, Box<dyn Error>>;
pub(crate) type DapServerOut = Arc<Mutex<Server<Empty, Stdout>>>;
pub(crate) type SharedDapState = Arc<Mutex<crate::state::DapState>>;
