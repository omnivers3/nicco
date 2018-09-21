// Copyright 2018-2018 The Omnivers3 Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Main entry point for interacting with the parser

use tokenizer::{ Tokenizer, TokenizerOptions, TokenizerResult };
use tree_builder::{ TreeBuilder, TreeBuilderOptions, TreeSink };

/// Settings struct which routes provided values into paresing process dependencies
#[derive(Clone, Default)]
pub struct ParseOptions {
    /// Provide settings to inform the tokenizer to use during the parse
    pub tokenizer: TokenizerOptions,
    /// Provider settings to inform the tree builder implementation
    pub tree_builder: TreeBuilderOptions,
}

/// A Nicco parser struct
/// 

/// Parse a Nicco document
