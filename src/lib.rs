pub mod error;
mod fetch;
mod location;

pub use fetch::get_locations;
pub use location::{Location, Person};
