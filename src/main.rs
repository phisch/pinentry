use std::io::{stdin, stdout};

use pinentry::Pinentry;

fn main() {
    let mut pinentry = Pinentry::new(stdin().lock(), stdout());

    pinentry.run();
}