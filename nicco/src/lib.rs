// Copyright 2018-2018 The Omnivers3 Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(associated_type_defaults)]
#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]
#![feature(extern_prelude)]

#![crate_name="nicco"]
#![crate_type="dylib"]

// extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate sink;
extern crate syn;

pub mod tokenizer;
pub mod tree_builder;