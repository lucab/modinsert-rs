extern crate modinsert;

use std::env;
use std::ffi::CString;

fn main() {
    let modname = env::args().nth(1).expect("missing module name");
    modinsert::try_load(&CString::new(modname).unwrap());
}
