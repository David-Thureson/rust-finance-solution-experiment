#![allow(dead_code)]

use locale::{self, LoLocaleFactory};

pub fn main() {
    try_locale();
}

fn try_locale() {
    // https://docs.rs/locale/0.2.2/locale/index.html

    let mut locale_factory = locale::user_locale_factory();
    dbg!(&locale_factory);
    dbg!(&locale_factory.get_numeric());
}

