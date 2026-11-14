//! Python source facts for Fensu: [`ProgramHandle`] parses a module once and serves its fact rows.
#![forbid(unsafe_code)]

#[doc(hidden)]
pub mod constants;
#[doc(hidden)]
pub mod dead_code;
#[doc(hidden)]
pub mod extension;
pub mod facts;
#[doc(hidden)]
pub mod mapping;
#[doc(hidden)]
pub mod parsing;
#[doc(hidden)]
pub mod positions;
#[doc(hidden)]
pub mod snapshot;
pub(crate) mod syntax;

pub use extension::models::ProgramHandle;
pub use parsing::models::ParseFailure;
