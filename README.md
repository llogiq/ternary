# Kleene logic within Rust's type system

Values are `True`, `False` and `Unknown`. Operations are `Not`, `BitAnd`
and `BitOr` from `std::ops`. There is also the `Ternary` enum which 
represents the values at runtime and the `ToTernary` trait that adds the
`to_ternary()` methods to our value types.

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
