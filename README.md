# Kleene logic within Rust's type system

[![Build Status](https://travis-ci.org/llogiq/ternary.svg)](https://travis-ci.org/llogiq/ternary)
[![Current Version](http://meritbadge.herokuapp.com/ternary)](https://crates.io/crates/ternary)
[![Coverage Status](https://coveralls.io/repos/github/llogiq/ternary/badge.svg?branch=master)](https://coveralls.io/github/llogiq/ternary?branch=master)
[![License: MIT/Apache](https://img.shields.io/crates/l/ternary.svg)](LICENSE)

Values are `True`, `False` and `Unknown`. Operations are `Not`, `BitAnd`
and `BitOr` from `std::ops`. There is also the `Ternary` enum which 
represents the values at runtime and the `ToTernary` trait that adds the
`to_ternary()` methods to our value types.

For more information read the [API documentation](http://llogiq.github.io/ternary)

Examples:

`Same` and `Not`

```
 use ternary::{True, False, Unknown, Same};
 use std::ops::Not;
type NotTrue = <<True as Not>::Output as Same<False>>::Output;
type NotFalse = <<False as Not>::Output as Same<True>>::Output;
type NotUnknown = <<Unknown as Not>::Output as Same<Unknown>>::Output;
```

Transforming Values to Runtime, `BitAnd` and `BitOr`

```
assert_eq!(Ternary::T, <True as BitOr<<Unknown as 
    BitAnd<False>>::Output>>::Output::to_ternary());
```

To use it, add `ternary = "0.1.0"` to your dependencies.
