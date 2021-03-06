// Copyright 2013-2014 The Num-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use algebra::{Field, CommutativeRing};
use ident;

/// A number that can be written without a fractional or decimal component.
///
/// # Handling of Division and Modulus
///
/// The most interesting part of this trait is the support for different
/// interpretations of integer division and modulus for discrete numbers. This
/// topic is discussed in greater depth in Daan Leijen's
/// _[Division and Modulus for Computer Scientists]
/// (http://legacy.cs.uu.nl/daan/download/papers/divmodnote-letter.pdf)_.
pub trait Integer
    : Eq + Ord
    + CommutativeRing {
    #[inline]
    fn succ(&self) -> Self { *self + ident::unit() }

    /// Truncated division satisfying:
    ///
    /// ~~~notrust
    /// t_div(a, b) = trunc(a / b)              ∀ a, b ∈ Self where b ≠ 0
    /// ~~~
    ///
    /// This is the form of division adopted by the ISO C99 standard for the `/`
    /// operator, and is usually more efficient than `f_div` due to better
    /// support on processors.
    fn t_div(a: &Self, b: &Self) -> Self;

    /// The remainder after truncated division satisfying:
    ///
    /// ~~~notrust
    /// t_mod(a, b) = a - (b * t_div(a, b))     ∀ a, b ∈ Self where b ≠ 0
    /// ~~~
    ///
    /// This is the form of modulus adopted by the ISO C99 standard for the `%`
    /// operator, and is usually more efficient than `f_mod` due to better
    /// support on processors.
    fn t_mod(a: &Self, b: &Self) -> Self;

    /// Calculates `t_div` and `t_mod` simultaneously.
    #[inline]
    fn t_div_mod(a: &Self, b: &Self) -> (Self, Self) {
        (t_div(a, b), t_mod(a, b))
    }

    /// Floored division satisfying:
    ///
    /// ~~~notrust
    /// f_div(a, b) = ⌊a / b⌋                   ∀ a, b ∈ Self where b ≠ 0
    /// ~~~
    fn f_div(a: &Self, b: &Self) -> Self;

    /// The remainder after floored division satisfying:
    ///
    /// ~~~notrust
    /// f_mod(a, b) = a - (b * f_div(a, b))     ∀ a, b where b ≠ 0
    /// ~~~
    fn f_mod(a: &Self, b: &Self) -> Self;

    /// Calculates `f_div` and `f_mod` simultaneously.
    #[inline]
    fn f_div_mod(a: &Self, b: &Self) -> (Self, Self) {
        (f_div(a, b), f_mod(a, b))
    }

    /// Greatest Common Divisor (GCD)
    fn gcd(a: &Self, b: &Self) -> Self;

    /// Lowest Common Multiple (LCM)
    fn lcm(a: &Self, b: &Self) -> Self;
}

pub trait ModularInteger
    : Integer {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn congruent(x: &Self, y: &Self) -> bool;
    #[inline]
    fn pred(&self) -> Self { *self - ident::unit() }
}

#[inline]
pub fn t_div<T: Integer>(a: &T, b: &T) -> T {
    Integer::t_div(a, b)
}

#[inline]
pub fn t_mod<T: Integer>(a: &T, b: &T) -> T {
    Integer::t_mod(a, b)
}

#[inline]
pub fn t_div_mod<T: Integer>(a: &T, b: &T) -> (T, T) {
    Integer::t_div_mod(a, b)
}

#[inline]
pub fn f_div<T: Integer>(a: &T, b: &T) -> T {
    Integer::f_div(a, b)
}

#[inline]
pub fn f_mod<T: Integer>(a: &T, b: &T) -> T {
    Integer::f_mod(a, b)
}

#[inline]
pub fn f_div_mod<T: Integer>(a: &T, b: &T) -> (T, T) {
    Integer::f_div_mod(a, b)
}

trait Real
    : PartialOrd
    + Field {
}

impl Real for f32  {}
impl Real for f64  {}
