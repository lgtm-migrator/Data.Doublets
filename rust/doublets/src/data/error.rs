use crate::{Doublet, Link};
use num::LinkType;
use std::backtrace::Backtrace;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinksError<T: LinkType> {
    #[error("link [{0}] does not exist.")]
    NotExists(T),

    #[error("link [{0}] has dependencies")]
    HasDeps(Link<T>),

    #[error("link [{0}] already exists")]
    AlreadyExists(Doublet<T>),

    #[error("limit for the number of links in the storage has been reached ({0})")]
    LimitReached(T),

    #[error("unable to allocate memory for links storage: `{0}`")]
    AllocFailed(
        #[from]
        #[backtrace]
        io::Error,
    ),
}
