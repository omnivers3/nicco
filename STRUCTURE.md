# nicco directory structure

The module structure is also documented in the output produced by `cargo doc`, alongside individual functions etc.

`src/`: The main html5ever library crate.

`src/driver.rs`: Provides the highest-level interfaces to the parser, i.e. "here's a string, give me a DOM"

`src/tokenizer/`: The first stage of HTML parsing, corresponding to WHATWG's [section 12.2.5 "Tokenization"](https://html.spec.whatwg.org/multipage/#tokenization)

`src/tree_builder/`: The second (and final) stage, corresponding to [section 12.2.6 "Tree Construction"](https://html.spec.whatwg.org/multipage/#tree-construction)

`src/serialize/`: Turning trees back into strings. Corresponds to [section 12.3 "Serialising HTML fragments"](https://html.spec.whatwg.org/multipage/#serialising-html-fragments)

`dom_sink/`: Types that nicco can use to represent the DOM, if you do not provide your own DOM implementation.

`macros/`: Code used at build-time to expand the `match_token!` "macro" in `src/tree_builder/rules.rs`.

`tests/`: Integration tests. This is a single executable crate that runs html5ever on the various [html5lib-tests](https://github.com/html5lib/html5lib-tests). There are also unit tests throughout the library code. See `README.md` for information on running tests.

`bench/`: Benchmarks. Another executable crate.

`examples/` and `dom_sink/examples`: Examples of using the library.  Each `.rs` file is an executable crate.

`data/`: Various data used in building and benchmarking the parser.