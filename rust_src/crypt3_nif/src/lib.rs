extern crate bincode;
extern crate core;
extern crate libc;
extern crate rustler;
extern crate serde;
extern crate siphasher;

mod atoms;
mod bindings;
mod nif;

rustler::init!("crypt3_nif", [nif::encrypt], load = nif::on_load);
