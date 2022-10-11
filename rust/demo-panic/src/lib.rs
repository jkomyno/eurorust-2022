#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = triggerPanic))]
pub fn trigger_panic_wasm(message: String) {
  trigger_panic(message);
}

pub fn trigger_panic(message: String) {
  // println! disappears in Wasm
  println!("Preparing to trigger a panic: {}", message);
  panic!("{}", message);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[should_panic(expected = "This is a test panic")]
  #[test]
  fn it_works() {
    let message = String::from("This is a test panic");
    trigger_panic(message)
  }
}
