extern crate serde;
extern crate serde_json;
extern crate serde_derive;

pub mod app;
pub mod board;
pub mod mcts;

fn main() {
    app::start()
}
