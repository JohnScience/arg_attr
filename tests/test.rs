#![allow(dead_code)]

use arg_attr::args;

#[args(_url: String)]
fn silly_drop() {
    drop(_url);
}
