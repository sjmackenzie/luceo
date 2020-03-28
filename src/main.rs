use std::rc::Rc;
use std::cell::RefCell;
use { libcopernica::{Router}, };
use std::thread;
struct CopernicaState {
    query_val: String,
}
mod gui;

fn main() {
    let copernica_state = Rc::new(RefCell::new(CopernicaState { query_val: "".into()}));
    gui::setup();
    let mut r = Router::new();
    thread::spawn(move || { r.run(); });
}
