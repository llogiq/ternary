//! Kleene logic within Rust's type system
//!
//! Values are `True`, `False` and `Unknown`. Operations are `Not`, `BitAnd`
//! and `BitOr` from `std::ops`. There is also the `Ternary` enum which 
//! represents the values at runtime and the `ToTernary` trait that adds the
//! `to_ternary()` methods to our value types.
//!
//! Examples:
//!
//! `Same` and `Not`
//!
//! ```
//!# use ternary::{True, False, Unknown, Same};
//!# use std::ops::Not;
//! type NotTrue = <<True as Not>::Output as Same<False>>::Output;
//! type NotFalse = <<False as Not>::Output as Same<True>>::Output;
//! type NotUnknown = <<Unknown as Not>::Output as Same<Unknown>>::Output;
//! ```
//! 
//! Transforming Values to Runtime, `BitAnd` and `BitOr`
//!
//! ```
//!# use ternary::{True, False, Unknown, Same, Ternary, ToTernary};
//!# use std::ops::{BitAnd, BitOr};
//! assert_eq!(Ternary::T, <True as BitOr<<Unknown as BitAnd<False>>::Output>>::Output::to_ternary());
//! ```
use std::ops::{Not, BitAnd, BitOr};

/// Our True type value
pub enum True {}

/// Our False type value
pub enum False {}

/// Our Unknown type value
pub enum Unknown {}

/// runtime representation
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Ternary { T, F, U }

/// conversion to runtime enum
///
/// examples:
/// ```
///# use ternary::{True, False, Unknown, Ternary, ToTernary};
/// assert_eq!(True::to_ternary(), Ternary::T);
/// assert_eq!(False::to_ternary(), Ternary::F);
/// assert_eq!(Unknown::to_ternary(), Ternary::U);
/// ```
pub trait ToTernary {
    fn to_ternary() -> Ternary;
}

impl ToTernary for True {
    #[inline] fn to_ternary() -> Ternary { Ternary::T }
}

impl ToTernary for False {
    #[inline] fn to_ternary() -> Ternary { Ternary::F }
}

impl ToTernary for Unknown {
    #[inline] fn to_ternary() -> Ternary { Ternary::U }
}

/// Not

/// !True == False
impl Not for True {
    type Output = False;
    fn not(self) -> Self::Output { unreachable!() }
}

///!False == True
impl Not for False {
    type Output = True;
    fn not(self) -> Self::Output { unreachable!() }
}

///!Unknown == Unknown
impl Not for Unknown {
    type Output = Unknown;
    fn not(self) -> Self::Output { unreachable!() }
}

/// BitAnd

/// True & X == X
impl<X: ToTernary> BitAnd<X> for True {
    type Output = X;
    fn bitand(self, _: X) -> Self::Output { unreachable!() }
}

/// False & X == False
impl<X: ToTernary> BitAnd<X> for False {
    type Output = False;
    fn bitand(self, _: X) -> Self::Output { unreachable!() }
}

/// Unknown & True == Unknown
impl BitAnd<True> for Unknown {
    type Output = Unknown;
    fn bitand(self, _: True) -> Self::Output { unreachable!() }
}

/// Unknown & Unknown == Unknown
impl BitAnd<Unknown> for Unknown {
    type Output = Unknown;
    fn bitand(self, _: Unknown) -> Self::Output { unreachable!() }
}

/// Unknown & False == False
impl BitAnd<False> for Unknown {
    type Output = False;
    fn bitand(self, _: False) -> Self::Output { unreachable!() }
}


/// BitOr

/// True | X == True
impl<X: ToTernary> BitOr<X> for True {
    type Output = True;
    fn bitor(self, _: X) -> Self::Output { unreachable!() }
}

/// False | X == X
impl<X: ToTernary> BitOr<X> for False {
    type Output = X;
    fn bitor(self, _: X) -> Self::Output { unreachable!() }
}

/// Unknown | True == True
impl BitOr<True> for Unknown {
    type Output = True;
    fn bitor(self, _: True) -> Self::Output { unreachable!() }
}

/// Unknown | Unknown == Unknown
impl BitOr<Unknown> for Unknown {
    type Output = Unknown;
    fn bitor(self, _: Unknown) -> Self::Output { unreachable!() }
}

/// Unknown | False == Unknown
impl BitOr<False> for Unknown {
    type Output = Unknown;
    fn bitor(self, _: False) -> Self::Output { unreachable!() }
}

/// shamelessly copied from typenum
pub trait Same<Rhs = Self> {
    type Output;
}

impl<T> Same<T> for T {
    type Output = T;
}
