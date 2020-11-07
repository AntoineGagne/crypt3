extern crate bincode;
extern crate pwhash;
extern crate core;
extern crate rustler;
extern crate serde;
extern crate siphasher;

mod atoms;
mod nif;

rustler::init!(
    "crypt3_nif",
    [
        nif::encrypt
    ],
    load = nif::on_load
);
