//! Basic types and their methods for linear algebra

use ndarray::LinalgScalar;
use num_complex::Complex;
use num_traits::*;
use rand::Rng;
use rand::distributions::*;
use std::fmt::Debug;
use std::iter::Sum;

use super::lapack_traits::LapackScalar;

pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;

/// General Scalar trait. This generalizes complex and real number.
///
/// You can use the following operations with `A: Scalar`:
///
/// - [abs](trait.Absolute.html#method.abs)
/// - [squared](trait.Absolute.html#tymethod.squared)
/// - [sqrt](trait.SquareRoot.html#tymethod.sqrt)
/// - [exp](trait.Exponential.html#tymethod.exp)
/// - [conj](trait.Conjugate.html#tymethod.conj)
/// - [randn](trait.RandNormal.html#tymethod.randn)
///
pub trait Scalar
    : LapackScalar
    + LinalgScalar
    + AssociatedReal
    + AssociatedComplex
    + Absolute
    + SquareRoot
    + Exponential
    + Conjugate
    + RandNormal
    + Debug {
}

impl Scalar for f32 {}
impl Scalar for f64 {}
impl Scalar for c32 {}
impl Scalar for c64 {}

pub trait RealScalar: Scalar + Float + Sum {
    fn from_f64(f64) -> Self;
}

impl RealScalar for f32 {
    fn from_f64(f: f64) -> Self {
        f as f32
    }
}
impl RealScalar for f64 {
    fn from_f64(f: f64) -> Self {
        f
    }
}

/// Convert `f64` into `RealScalar`
///
/// ```rust
/// fn mult2<A: RealScalar>(a: A) -> A {
///     a * into_real(2.0);
/// }
/// ```
pub fn into_real<T: RealScalar>(f: f64) -> T {
    T::from_f64(f)
}

/// Define associating real float type
pub trait AssociatedReal: Sized {
    type Real: RealScalar;
    fn inject(Self::Real) -> Self;
    fn add_real(self, Self::Real) -> Self;
    fn sub_real(self, Self::Real) -> Self;
    fn mul_real(self, Self::Real) -> Self;
    fn div_real(self, Self::Real) -> Self;
}

/// Define associating complex type
pub trait AssociatedComplex: Sized {
    type Complex;
    fn inject(Self) -> Self::Complex;
    fn add_complex(self, Self::Complex) -> Self::Complex;
    fn sub_complex(self, Self::Complex) -> Self::Complex;
    fn mul_complex(self, Self::Complex) -> Self::Complex;
}

/// Define `abs()` more generally
pub trait Absolute: AssociatedReal {
    fn squared(&self) -> Self::Real;
    fn abs(&self) -> Self::Real {
        self.squared().sqrt()
    }
}

/// Define `sqrt()` more generally
pub trait SquareRoot {
    fn sqrt(&self) -> Self;
}

/// Define `exp()` more generally
pub trait Exponential {
    fn exp(&self) -> Self;
}

/// Complex conjugate value
pub trait Conjugate: Copy {
    fn conj(self) -> Self;
}

/// Scalars which can be initialized from Gaussian random number
pub trait RandNormal {
    fn randn<R: Rng>(&mut R) -> Self;
}

macro_rules! impl_traits {
    ($real:ty, $complex:ty) => {

impl AssociatedReal for $real {
    type Real = $real;
    fn inject(r: Self::Real) -> Self { r }
    fn add_real(self, r: Self::Real) -> Self { self + r }
    fn sub_real(self, r: Self::Real) -> Self { self - r }
    fn mul_real(self, r: Self::Real) -> Self { self * r }
    fn div_real(self, r: Self::Real) -> Self { self / r }
}

impl AssociatedReal for $complex {
    type Real = $real;
    fn inject(r: Self::Real) -> Self { Self::new(r, 0.0) }
    fn add_real(self, r: Self::Real) -> Self { self + r }
    fn sub_real(self, r: Self::Real) -> Self { self - r }
    fn mul_real(self, r: Self::Real) -> Self { self * r }
    fn div_real(self, r: Self::Real) -> Self { self / r }
}

impl AssociatedComplex for $real {
    type Complex = $complex;
    fn inject(r: Self) -> Self::Complex { Self::Complex::new(r, 0.0) }
    fn add_complex(self, c: Self::Complex) -> Self::Complex { self + c }
    fn sub_complex(self, c: Self::Complex) -> Self::Complex { self - c }
    fn mul_complex(self, c: Self::Complex) -> Self::Complex { self * c }
}

impl AssociatedComplex for $complex {
    type Complex = $complex;
    fn inject(c: Self) -> Self::Complex { c }
    fn add_complex(self, c: Self::Complex) -> Self::Complex { self + c }
    fn sub_complex(self, c: Self::Complex) -> Self::Complex { self - c }
    fn mul_complex(self, c: Self::Complex) -> Self::Complex { self * c }
}

impl Absolute for $real {
    fn squared(&self) -> Self::Real {
        *self * *self
    }
    fn abs(&self) -> Self::Real{
        Float::abs(*self)
    }
}

impl Absolute for $complex {
    fn squared(&self) -> Self::Real {
        self.norm_sqr()
    }
    fn abs(&self) -> Self::Real {
        self.norm()
    }
}

impl SquareRoot for $real {
    fn sqrt(&self) -> Self {
        Float::sqrt(*self)
    }
}

impl SquareRoot for $complex {
    fn sqrt(&self) -> Self {
        Complex::sqrt(self)
    }
}

impl Exponential for $real {
    fn exp(&self) -> Self {
        Float::exp(*self)
    }
}

impl Exponential for $complex {
    fn exp(&self) -> Self {
        Complex::exp(self)
    }
}

impl Conjugate for $real {
    fn conj(self) -> Self {
        self
    }
}

impl Conjugate for $complex {
    fn conj(self) -> Self {
        Complex::conj(&self)
    }
}

impl RandNormal for $real {
    fn randn<R: Rng>(rng: &mut R) -> Self {
        let dist = Normal::new(0., 1.);
        dist.ind_sample(rng) as $real
    }
}

impl RandNormal for $complex {
    fn randn<R: Rng>(rng: &mut R) -> Self {
        let dist = Normal::new(0., 1.);
        let re = dist.ind_sample(rng) as $real;
        let im = dist.ind_sample(rng) as $real;
        Self::new(re, im)
    }
}

}} // impl_traits!

impl_traits!(f64, c64);
impl_traits!(f32, c32);
