use std::rc::Rc;
use std::cell::RefCell;
use { libcopernica::{Router}, };
use std::thread;
mod gui;
mod state;

fn main() {
    gui::setup();
    let mut r = Router::new();
    thread::spawn(move || { r.run(); });
}
