// Crate-wide configuration.
#[allow(unused_imports)]
#[macro_use]
extern crate all_asserts;
#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;
#[allow(unused_imports)]
#[macro_use]
extern crate derivative;
#[allow(unused_imports)]
#[macro_use]
extern crate derive_builder;
#[allow(unused_imports)]
#[macro_use]
extern crate derive_more;
#[allow(unused_imports)]
#[macro_use]
extern crate log;
#[allow(unused_imports)]
#[macro_use]
extern crate serde;
#[allow(unused_imports)]
#[macro_use]
extern crate strum;
#[allow(unused_imports)]
#[macro_use]
extern crate thiserror;

pub mod conclusions;
pub mod conditions;
pub mod contexts;
pub mod operations;
pub mod rule;
pub mod traits;
pub mod values;

pub mod prelude {
  pub use super::conclusions::prelude::*;
  pub use super::conditions::prelude::*;
  pub use super::contexts::prelude::*;
  pub use super::operations::prelude::*;
  pub use super::rule::Rule;
  pub use super::traits::prelude::*;
  pub use super::values::prelude::*;
}

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  #[allow(unused_imports)]
  use super::*;

  // Call this function at the beginning of each test module.
  pub fn init() {
    // Enable logging for tests.
    let _ = builder().is_test(true).try_init();
    // Enable backtraces.
    set_var("RUST_BACKTRACE", "1");
  }
}
