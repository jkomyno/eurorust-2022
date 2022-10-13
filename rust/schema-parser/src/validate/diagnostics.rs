use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", tsify::declare)]
type DatamodelError = String;

/// Represents a list of validation or parser errors.
/// This is used to accumulate multiple errors and warnings during validation.
/// It is used to not error out early and instead show multiple errors at once.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct Diagnostics {
  errors: Vec<DatamodelError>,
}

impl Diagnostics {
  pub fn new() -> Diagnostics {
    Diagnostics { errors: Vec::new() }
  }

  pub fn errors(&self) -> &[DatamodelError] {
    &self.errors
  }

  pub fn push_error(&mut self, err: DatamodelError) {
    self.errors.push(err)
  }

  /// Returns true, if there is at least one error in this collection.
  pub fn has_errors(&self) -> bool {
    !self.errors.is_empty()
  }

  pub fn to_result(&mut self) -> Result<(), Diagnostics> {
    if self.has_errors() {
      Err(std::mem::take(self))
    } else {
      Ok(())
    }
  }
}

impl From<DatamodelError> for Diagnostics {
  fn from(error: DatamodelError) -> Self {
    let mut col = Diagnostics::new();
    col.push_error(error);
    col
  }
}

impl Default for Diagnostics {
  fn default() -> Self {
    Self::new()
  }
}
