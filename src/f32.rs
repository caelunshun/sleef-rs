#[cfg(test)]
mod tst;
#[cfg(test)]
pub(crate) use tst::*;

use crate::common::*;
use core::f32::consts::{FRAC_1_PI, FRAC_2_PI, FRAC_PI_2, FRAC_PI_4, PI};
use doubled::*;

pub(crate) const F1_32: f32 = (1u64 << 32) as f32;
pub(crate) const F1_30: f32 = (1u32 << 30) as f32;
pub(crate) const F1_25: f32 = (1u32 << 25) as f32;
pub(crate) const F1_24: f32 = (1u32 << 24) as f32;
pub(crate) const F1_23: f32 = (1u32 << 23) as f32;
pub(crate) const F1_12: f32 = (1u32 << 12) as f32;
pub(crate) const F1_10: f32 = (1u32 << 10) as f32;

pub(crate) const PI_A_F: f32 = 3.140_625;
pub(crate) const PI_B_F: f32 = 0.000_967_025_756_835_937_5;
pub(crate) const PI_C_F: f32 = 6.277_114_152_908_325_195_3_e-7;
pub(crate) const PI_D_F: f32 = 1.215_420_125_655_342_076_2_e-10;
pub(crate) const TRIGRANGEMAX_F: f32 = 39000.;

pub(crate) const PI_A2_F: f32 = 3.141_479_492_187_5;
pub(crate) const PI_B2_F: f32 = 0.000_113_159_418_106_079_101_56;
pub(crate) const PI_C2_F: f32 = 1.984_187_258_941_005_893_6_e-9;
pub(crate) const TRIGRANGEMAX2_F: f32 = 125.0;

pub(crate) const SLEEF_FP_ILOGB0: i32 = -2_147_483_648;
pub(crate) const SLEEF_FP_ILOGBNAN: i32 = 2_147_483_647;
pub(crate) const SQRT_FLT_MAX: f32 = 18_446_743_523_953_729_536.;
pub(crate) const L10U_F: f32 = 0.301_025_390_6;
pub(crate) const L10L_F: f32 = 4.605_038_981_e-6;
pub(crate) const TRIGRANGEMAX4_F: f32 = 8e+6;
pub(crate) const L2U_F: f32 = 0.693_145_751_953_125;
pub(crate) const L2L_F: f32 = 1.428_606_765_330_187_045_e-6;
pub(crate) const R_LN2_F: f32 =
    1.442_695_040_888_963_407_359_924_681_001_892_137_426_645_954_152_985_934_135_449_406_931;
pub(crate) const LOG10_2_F: f32 = 3.321_928_094_887_362_347_870_319_429_489_390_175_864_831_393;

pub(crate) const D_PI: Doubled<f32> = Doubled::new(
    3.141_592_741_012_573_242_2,
    -8.742_277_657_347_585_773_1_e-8,
);
pub(crate) const D_LN2: Doubled<f32> =
    Doubled::new(0.693_147_182_464_599_609_38, -1.904_654_323_148_236_017_e-9);

mod u05;
#[rustfmt::skip]
pub use u05::{
    sincospif as sincospi_u05,
    sqrtf as sqrt_u05,
    hypotf as hypot_u05,
    sinpif as sinpi_u05,
    cospif as cospi_u05,
};

mod u10;
#[rustfmt::skip]
pub use u10::{
    sinf as sin_u10,
    cosf as cos_u10,
    sincosf as sincos_u10,
    tanf as tan_u10,
    atan2f as atan2_u10,
    asinf as asin_u10,
    acosf as acos_u10,
    atanf as atan_u10,
    expf as exp_u10,
    cbrtf as cbrt_u10,
    logf as log_u10,
    powf as pow_u10,
    sinhf as sinh_u10,
    coshf as cosh_u10,
    tanhf as tanh_u10,
    asinhf as asinh_u10,
    acoshf as acosh_u10,
    atanhf as atanh_u10,
    exp10f as exp10_u10,
    expm1f as expm1_u10,
    log10f as log10_u10,
    log2f as log2_u10,
    tgammaf as tgamma_u10,
    lgammaf as lgamma_u10,
    erff as erf_u10,
    log1pf as log1p_u10,
    exp2f as exp2_u10,
};

mod u15;
#[rustfmt::skip]
pub use u15::{
    erfcf as erfc_u15,
};

mod u35;
#[rustfmt::skip]
pub use u35::{
    sinf as sin_u35,
    cosf as cos_u35,
    tanf as tan_u35,
    sincosf as sincos_u35,
    sincospif as sincospi_u35,
    atanf as atan_u35,
    atan2f as atan2_u35,
    asinf as asin_u35,
    acosf as acos_u35,
    logf as log_u35,
    sqrtf as sqrt_u35,
    cbrtf as cbrt_u35,
    sinhf as sinh_u35,
    coshf as cosh_u35,
    tanhf as tanh_u35,
    hypotf as hypot_u35,
    exp2f as exp2_u35,
    exp10f as exp10_u35,
    log2f as log2_u35,
};
mod fast;
#[rustfmt::skip]
pub use fast::{
    sinf as sin_fast,
    cosf as cos_fast,
    powf as pow_fast,
};

impl crate::Sleef for f32 {
    type Int = i32;
    #[inline]
    fn sin(self) -> Self {
        u35::sinf(self)
    }
    #[inline]
    fn cos(self) -> Self {
        u35::cosf(self)
    }
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        u35::sincosf(self)
    }
    #[inline]
    fn tan(self) -> Self {
        u35::tanf(self)
    }
    #[inline]
    fn asin(self) -> Self {
        u35::asinf(self)
    }
    #[inline]
    fn acos(self) -> Self {
        u35::acosf(self)
    }
    #[inline]
    fn atan(self) -> Self {
        u35::atanf(self)
    }
    #[inline]
    fn atan2(self, other: Self) -> Self {
        u35::atan2f(self, other)
    }
    #[inline]
    fn ln(self) -> Self {
        u35::logf(self)
    }
    #[inline]
    fn cbrt(self) -> Self {
        u35::cbrtf(self)
    }
    #[inline]
    fn exp(self) -> Self {
        u10::expf(self)
    }
    #[inline]
    fn pow(self, other: Self) -> Self {
        u10::powf(self, other)
    }
    #[inline]
    fn sinh(self) -> Self {
        u10::sinhf(self)
    }
    #[inline]
    fn cosh(self) -> Self {
        u10::coshf(self)
    }
    #[inline]
    fn tanh(self) -> Self {
        u10::tanhf(self)
    }
    #[inline]
    fn asinh(self) -> Self {
        u10::asinhf(self)
    }
    #[inline]
    fn acosh(self) -> Self {
        u10::acoshf(self)
    }
    #[inline]
    fn atanh(self) -> Self {
        u10::atanhf(self)
    }
    #[inline]
    fn exp2(self) -> Self {
        u10::exp2f(self)
    }
    #[inline]
    fn exp10(self) -> Self {
        u10::exp10f(self)
    }
    #[inline]
    fn exp_m1(self) -> Self {
        u10::expm1f(self)
    }
    #[inline]
    fn log10(self) -> Self {
        u10::log10f(self)
    }
    #[inline]
    fn log2(self) -> Self {
        u10::log2f(self)
    }
    #[inline]
    fn log_1p(self) -> Self {
        u10::log1pf(self)
    }
    #[inline]
    fn ldexp(self, other: Self::Int) -> Self {
        ldexpf(self, other)
    }
    #[inline]
    fn ilogb(self) -> Self::Int {
        ilogbf(self)
    }
    #[inline]
    fn fma(self, y: Self, z: Self) -> Self {
        fmaf(self, y, z)
    }
    #[inline]
    fn sqrt(self) -> Self {
        u35::sqrtf(self)
    }
    #[inline]
    fn abs(self) -> Self {
        fabsf(self)
    }
    #[inline]
    fn copy_sign(self, other: Self) -> Self {
        copysignf(self, other)
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        fmaxf(self, other)
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        fminf(self, other)
    }
    #[inline]
    fn fdim(self, other: Self) -> Self {
        fdimf(self, other)
    }
    #[inline]
    fn truncate(self) -> Self {
        truncf(self)
    }
    #[inline]
    fn floor(self) -> Self {
        floorf(self)
    }
    #[inline]
    fn ceil(self) -> Self {
        ceilf(self)
    }
    #[inline]
    fn round(self) -> Self {
        rintf(self)
    }
    #[inline]
    fn next_after(self, other: Self) -> Self {
        nextafterf(self, other)
    }
    #[inline]
    fn frfrexp(self) -> Self {
        frfrexpf(self)
    }
    #[inline]
    fn expfrexp(self) -> Self::Int {
        expfrexpf(self)
    }
    #[inline]
    fn fmod(self, other: Self) -> Self {
        fmodf(self, other)
    }
    #[inline]
    fn remainder(self, other: Self) -> Self {
        remainderf(self, other)
    }
    #[inline]
    fn modf(self) -> (Self, Self) {
        modff(self)
    }
    #[inline]
    fn sin_cos_pi(self) -> (Self, Self) {
        u35::sincospif(self)
    }
    #[inline]
    fn sin_pi(self) -> Self {
        u05::sinpif(self)
    }
    #[inline]
    fn cos_pi(self) -> Self {
        u05::cospif(self)
    }
    #[inline]
    fn hypot(self, other: Self) -> Self {
        u35::hypotf(self, other)
    }
    #[inline]
    fn gamma(self) -> Self {
        u10::tgammaf(self)
    }
    #[inline]
    fn lgamma(self) -> Self {
        u10::lgammaf(self)
    }
    #[inline]
    fn erf(self) -> Self {
        u10::erff(self)
    }
    #[inline]
    fn erfc(self) -> Self {
        u15::erfcf(self)
    }
}

impl MaskType for f32 {
    type Mask = bool;
}

impl BitsType for f32 {
    type Bits = u32;
}

impl MulAdd for f32 {
    #[inline]
    fn mla(self, y: Self, z: Self) -> Self {
        if cfg!(target_feature = "fma") {
            self.mul_add(y, z)
        } else {
            self * y + z
        }
    }
}

impl Poly<Self> for f32 {
    fn c2v(c: Self) -> Self {
        c
    }
}

impl Sign for f32 {
    #[inline]
    fn sign_bit(self) -> Self::Bits {
        self.to_bits() & (1 << 31)
    }
    #[inline]
    fn sign(self) -> Self {
        mulsignf(1., self)
    }
    #[inline]
    fn mul_sign(self, other: Self) -> Self {
        mulsignf(self, other)
    }
    #[inline]
    fn or_sign(self, other: Self) -> Self {
        Self::from_bits(self.to_bits() | other.sign_bit())
    }
    #[inline]
    fn copy_sign(self, other: Self) -> Self {
        copysignfk(self, other)
    }
}

impl IsInt for f32 {
    #[inline]
    fn is_integer(self) -> Self::Mask {
        self == (self as i32 as f32)
    }
}

impl IsNegZero for f32 {
    #[inline]
    fn is_neg_zero(self) -> Self::Mask {
        self.to_bits() == (-0f32).to_bits()
    }
}

/// Multiply left value with sign of right value
#[inline]
pub fn mulsignf(x: f32, y: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ (y.to_bits() & (1 << 31)))
}

#[inline]
fn copysignfk(x: f32, y: f32) -> f32 {
    f32::from_bits((x.to_bits() & !(1 << 31)) ^ (y.to_bits() & (1 << 31)))
}

/// Sign of a number
#[inline]
pub fn signf(d: f32) -> f32 {
    mulsignf(1., d)
}

#[inline]
fn fabsfk(x: f32) -> f32 {
    f32::from_bits(0x_7fff_ffff & x.to_bits())
}

#[inline]
fn rintfk(x: f32) -> f32 {
    (if x < 0. { x - 0.5 } else { x + 0.5 }) as i32 as f32
}

#[inline]
fn ceilfk(x: f32) -> i32 {
    (x as i32) + (if x < 0. { 0 } else { 1 })
}

#[inline]
fn ilogbkf(mut d: f32) -> i32 {
    let m = d < 5.421_010_862_427_522_e-20;
    d = if m {
        1.844_674_407_370_955_2_e19 * d
    } else {
        d
    };
    let q = ((d.to_bits() >> 23) & 0xff) as i32;
    if m {
        q - (64 + 0x7f)
    } else {
        q - 0x7f
    }
}

// vilogb2kf is similar to ilogbkf, but the argument has to be a
// normalized FP value.
#[inline]
fn ilogb2kf(d: f32) -> i32 {
    ((d.to_bits() >> 23) & 0xff) as i32 - 0x7f
}

#[inline]
fn pow2if(q: i32) -> f32 {
    f32::from_bits(((q + 0x7f) as u32) << 23)
}

#[inline]
fn ldexpkf(mut x: f32, mut q: i32) -> f32 {
    let mut m = q >> 31;
    m = (((m.wrapping_add(q)) >> 6) - m) << 4;
    q -= m << 2;
    m += 127;
    m = if m < 0 { 0 } else { m };
    m = if m > 255 { 255 } else { m };
    let mut u = f32::from_bits((m as u32) << 23);
    x = x * u * u * u * u;
    u = f32::from_bits(((q + 0x7f) as u32) << 23);
    x * u
}

#[inline]
fn ldexp2kf(d: f32, e: i32) -> f32 {
    // faster than ldexpkf, short reach
    d * pow2if(e >> 1) * pow2if(e - (e >> 1))
}

#[inline]
fn ldexp3kf(d: f32, e: i32) -> f32 {
    // very fast, no denormal
    f32::from_bits(((d.to_bits() as i32) + (e << 23)) as u32)
}

fn rempisubf(x: f32) -> (f32, i32) {
    let mut fr = x - F1_10 * ((x * (1. / F1_10)) as i32 as f32);
    let mut reti = ((7 & ((if x > 0. { 4 } else { 3 }) + ((fr * 8.) as i32))) - 3) >> 1;
    fr = fr - 0.25 * ((fr * 4. + 0.5.mul_sign(x)) as i32 as f32);
    fr = if fabsfk(fr) > 0.125 {
        fr - 0.5.mul_sign(x)
    } else {
        fr
    };
    fr = if fabsfk(fr) > 1e+10 { 0. } else { fr };
    if fabsfk(x) == 0.124_999_992_549_419_403_08 {
        fr = x;
        reti = 0;
    }
    (fr, reti)
}

fn rempif(a: f32) -> (Doubled<f32>, i32) {
    let mut ex = ilogb2kf(a) - 25;
    let mut q = if ex > (90 - 25) { -64 } else { 0 };
    let a = ldexp3kf(a, q);
    if ex < 0 {
        ex = 0;
    }
    let ex = (ex * 4) as usize;
    let mut x = a.mul_as_doubled(crate::tables::REMPITABSP[ex]);
    let (did, dii) = rempisubf(x.0);
    q = dii;
    x.0 = did;
    x = x.normalize();
    let mut y = a.mul_as_doubled(crate::tables::REMPITABSP[ex + 1]);
    x += y;
    let (did, dii) = rempisubf(x.0);
    q += dii;
    x.0 = did;
    x = x.normalize();
    y = Doubled::new(
        crate::tables::REMPITABSP[ex + 2],
        crate::tables::REMPITABSP[ex + 3],
    ) * a;
    x += y;
    x = x.normalize();
    x *= Doubled::new(D_PI.0 * 2., D_PI.1 * 2.);
    (if fabsfk(a) < 0.7 { Doubled::from(a) } else { x }, q)
}

#[inline]
fn expk2f(d: Doubled<f32>) -> Doubled<f32> {
    let qf = rintfk(f32::from(d) * R_LN2_F);

    let q = qf as i32;
    let mut s = d + qf * -L2U_F;
    s += qf * -L2L_F;

    let u = 0.198_096_022_4_e-3_f32
        .mla(s.0, 0.139_425_648_4_e-2)
        .mla(s.0, 0.833_345_670_3_e-2)
        .mla(s.0, 0.416_663_736_1_e-1);

    let mut t = s * u + 0.166_666_659_414_234_244_790_680_580_464;
    t = s * t + 0.5;
    t = s + s.square() * t;

    t = 1. + t;

    t = Doubled::new(ldexp2kf(t.0, q), ldexp2kf(t.1, q));

    if d.0 < -104. {
        Doubled::from(0.)
    } else {
        t
    }
}

#[inline]
fn sinpifk(d: f32) -> Doubled<f32> {
    let u = d * 4.;
    let q = ceilfk(u) & !1;
    let o = (q & 2) != 0;

    let mut s = u - (q as f32);
    let t = s;
    s = s * s;
    let s2 = t.mul_as_doubled(t);

    //

    let u = (if o {
        -0.243_061_180_1_e-7_f32
    } else {
        0.309_384_205_4_e-6
    })
    .mla(
        s,
        if o {
            0.359_057_708_e-5
        } else {
            -0.365_730_738_8_e-4
        },
    )
    .mla(
        s,
        if o {
            -0.325_991_772_1_e-3
        } else {
            0.249_039_358_5_e-2
        },
    );
    let mut x = u * s
        + (if o {
            Doubled::new(
                0.015_854_343_771_934_509_277,
                4.494_005_135_403_224_281_1_e-10,
            )
        } else {
            Doubled::new(
                -0.080_745_510_756_969_451_904,
                -1.337_366_533_907_693_625_8_e-9,
            )
        });
    x = s2 * x
        + (if o {
            Doubled::new(
                -0.308_425_128_459_930_419_92,
                -9.072_833_903_073_392_227_7_e-9,
            )
        } else {
            Doubled::new(
                0.785_398_185_253_143_310_55,
                -2.185_733_861_756_648_485_5_e-8,
            )
        });

    x *= if o { s2 } else { Doubled::from(t) };
    x = if o { x + 1. } else { x };

    //

    if (q & 4) != 0 {
        x = -x;
    }
    x
}

#[inline]
fn cospifk(d: f32) -> Doubled<f32> {
    let u = d * 4.;
    let q = ceilfk(u) & !1;
    let o = (q & 2) == 0;

    let mut s = u - (q as f32);
    let t = s;
    s = s * s;
    let s2 = t.mul_as_doubled(t);

    //

    let u = (if o {
        -0.243_061_180_1_e-7_f32
    } else {
        0.309_384_205_4_e-6
    })
    .mla(
        s,
        if o {
            0.359_057_708_e-5
        } else {
            -0.365_730_738_8_e-4
        },
    )
    .mla(
        s,
        if o {
            -0.325_991_772_1_e-3
        } else {
            0.249_039_358_5_e-2
        },
    );
    let mut x = u * s
        + (if o {
            Doubled::new(
                0.015_854_343_771_934_509_277,
                4.494_005_135_403_224_281_1_e-10,
            )
        } else {
            Doubled::new(
                -0.080_745_510_756_969_451_904,
                -1.337_366_533_907_693_625_8_e-9,
            )
        });
    x = s2 * x
        + (if o {
            Doubled::new(
                -0.308_425_128_459_930_419_92,
                -9.072_833_903_073_392_227_7_e-9,
            )
        } else {
            Doubled::new(
                0.785_398_185_253_143_310_55,
                -2.185_733_861_756_648_485_5_e-8,
            )
        });

    x *= if o { s2 } else { Doubled::from(t) };
    x = if o { x + 1. } else { x };

    //

    if ((q + 2) & 4) != 0 {
        x = -x;
    }
    x
}

/// Integer exponent of an FP number
pub fn ilogbf(d: f32) -> i32 {
    let mut e = ilogbkf(fabsfk(d));
    e = if d == 0. { SLEEF_FP_ILOGB0 } else { e };
    e = if d.is_nan() { SLEEF_FP_ILOGBNAN } else { e };
    if d.is_infinite() {
        i32::MAX
    } else {
        e
    }
}

/// Absolute value
pub fn fabsf(x: f32) -> f32 {
    fabsfk(x)
}

/// Copy sign of a number
pub fn copysignf(x: f32, y: f32) -> f32 {
    copysignfk(x, y)
}

/// Maximum of two numbers
pub fn fmaxf(x: f32, y: f32) -> f32 {
    if y.is_nan() || (x > y) {
        x
    } else {
        y
    }
}

/// Minimum of two numbers
pub fn fminf(x: f32, y: f32) -> f32 {
    if y.is_nan() || (x < y) {
        x
    } else {
        y
    }
}

/// Positive difference
pub fn fdimf(x: f32, y: f32) -> f32 {
    let ret = x - y;
    if (ret < 0.) || (x == y) {
        0.
    } else {
        ret
    }
}

/// Round to integer towards zero
pub fn truncf(x: f32) -> f32 {
    let fr = x - (x as i32 as f32);
    if x.is_infinite() || (fabsfk(x) >= F1_23) {
        x
    } else {
        (x - fr).copy_sign(x)
    }
}

/// Round to integer towards minus infinity
pub fn floorf(x: f32) -> f32 {
    let mut fr = x - (x as i32 as f32);
    fr = if fr < 0. { fr + 1. } else { fr };
    if x.is_infinite() || (fabsfk(x) >= F1_23) {
        x
    } else {
        (x - fr).copy_sign(x)
    }
}

/// Round to integer towards plus infinity
pub fn ceilf(x: f32) -> f32 {
    let mut fr = x - (x as i32 as f32);
    fr = if fr <= 0. { fr } else { fr - 1. };
    if x.is_infinite() || (fabsfk(x) >= F1_23) {
        x
    } else {
        (x - fr).copy_sign(x)
    }
}

/// Round to integer away from zero
pub fn roundf(d: f32) -> f32 {
    let mut x = d + 0.5;
    let mut fr = x - (x as i32 as f32);
    if (fr == 0.) && (x <= 0.) {
        x -= 1.
    };
    fr = if fr < 0. { fr + 1. } else { fr };
    x = if d == 0.499_999_970_197_677_612_3 {
        0.
    } else {
        x
    }; // nextafterf(0.5, 0)
    if d.is_infinite() || (fabsfk(d) >= F1_23) {
        d
    } else {
        (x - fr).copy_sign(d)
    }
}

/// Round to integer, ties round to even
pub fn rintf(d: f32) -> f32 {
    let mut x = d + 0.5;
    let isodd = (1 & (x as i32)) != 0;
    let mut fr = x - (x as i32 as f32);
    fr = if (fr < 0.) || ((fr == 0.) && isodd) {
        fr + 1.
    } else {
        fr
    };
    x = if d == 0.500_000_059_604_644_775_39 {
        0.
    } else {
        x
    }; // nextafterf(0.5, 1)
    if d.is_infinite() || (fabsfk(d) >= F1_23) {
        d
    } else {
        (x - fr).copy_sign(d)
    }
}

/// Integral and fractional value of FP number
pub fn modff(x: f32) -> (f32, f32) {
    let mut fr = x - (x as i32 as f32);
    fr = if fabsfk(x) > F1_23 { 0. } else { fr };
    (fr.copy_sign(x), (x - fr).copy_sign(x))
}

/// Multiply by integral power of `2`
///
/// These functions return the result of multiplying ***m*** by `2` raised to the power ***x***.
pub fn ldexpf(x: f32, mut exp: i32) -> f32 {
    if exp > 300 {
        exp = 300;
    }
    if exp < -300 {
        exp = -300;
    }

    let mut e0 = exp >> 2;
    if exp < 0 {
        e0 += 1;
    }
    if (-50 < exp) && (exp < 50) {
        e0 = 0;
    }
    let e1 = exp - (e0 << 2);

    let p = pow2if(e0);
    x * pow2if(e1) * p * p * p * p
}

/// Find the next representable FP value
pub fn nextafterf(x: f32, y: f32) -> f32 {
    let mut cxi = (if x == 0. { 0.0.mul_sign(y) } else { x }).to_bits() as i32;
    let c = (cxi < 0) == (y < x);
    if c {
        cxi = -(cxi ^ i32::MIN);
    }

    if x != y {
        cxi -= 1;
    }

    if c {
        cxi = -(cxi ^ i32::MIN);
    }

    let cxf = f32::from_bits(cxi as u32);

    if x.is_nan() || y.is_nan() {
        f32::NAN
    } else if (x == 0.) && (y == 0.) {
        y
    } else if (cxf == 0.) && (x != 0.) {
        0.0.mul_sign(x)
    } else {
        cxf
    }
}

#[test]
fn test_nextafterf() {
    test_ff_f(
        nextafterf,
        |mut f, t| {
            let prec = f.prec();
            f.set_prec(24);
            f.next_toward(&t);
            f.set_prec(prec);
            f
        },
        f32::MIN..=f32::MAX,
        f32::MIN..=f32::MAX,
        0.1,
    );
}

/// Fractional component of an FP number
pub fn frfrexpf(mut x: f32) -> f32 {
    if fabsfk(x) < f32::MIN_POSITIVE {
        x *= F1_30;
    }

    let mut cxu = x.to_bits();
    cxu &= !0x_7f80_0000_u32;
    cxu |= 0x_3f00_0000_u32;

    if x == 0. {
        x
    } else if x.is_infinite() {
        f32::INFINITY.mul_sign(x)
    } else {
        f32::from_bits(cxu)
    }
}

/// Exponent of an FP number
pub fn expfrexpf(mut x: f32) -> i32 {
    let mut ret = if fabsfk(x) < f32::MIN_POSITIVE {
        x *= F1_30;
        -30
    } else {
        0
    };

    ret += (((x.to_bits() >> 23) & 0xff) as i32) - 0x7e;

    if (x == 0.) || x.is_nan() || x.is_infinite() {
        0
    } else {
        ret
    }
}

/// FP remainder
pub fn fmodf(x: f32, y: f32) -> f32 {
    #[inline]
    fn toward0(d: f32) -> f32 {
        if d == 0. {
            0.
        } else {
            f32::from_bits(d.to_bits() - 1)
        }
    }

    #[inline]
    fn trunc_positive(x: f32) -> f32 {
        if fabsfk(x) >= F1_23 {
            x
        } else {
            x - (x - (x as i32 as f32))
        }
    }

    let mut nu = fabsfk(x);
    let mut de = fabsfk(y);
    let s = if de < f32::MIN_POSITIVE {
        nu *= F1_25;
        de *= F1_25;
        1. / F1_25
    } else {
        1.
    };

    let mut r = Doubled::from(nu);
    let rde = toward0(1. / de);

    for _ in 0..8 {
        // ceil(log2(FLT_MAX) / 22)+1
        let mut q = trunc_positive(toward0(r.0) * rde);
        q = if (3. * de > r.0) && (r.0 >= de) {
            2.
        } else {
            q
        };
        q = if (2. * de > r.0) && (r.0 >= de) {
            1.
        } else {
            q
        };
        r = (r + (q.mul_as_doubled(-de))).normalize();
        if r.0 < de {
            break;
        }
    }

    let r = f32::from(r);
    let mut ret = if r == de { 0. } else { r * s };

    ret = ret.mul_sign(x);
    if de == 0. {
        f32::NAN
    } else if nu < de {
        x
    } else {
        ret
    }
}

// TODO: add test for fmodf

#[inline]
fn rintfk2(d: f32) -> f32 {
    let x = d + 0.5;
    let isodd = 1 & (x as i32) != 0;
    let mut fr = x - (x as i32 as f32);
    fr = if fr < 0. || (fr == 0. && isodd) {
        fr + 1.
    } else {
        fr
    };
    if fabsfk(d) >= F1_23 {
        d
    } else {
        (x - fr).copy_sign(d)
    }
}

/// FP remainder
pub fn remainderf(x: f32, y: f32) -> f32 {
    let mut n = fabsfk(x);
    let mut d = fabsfk(y);
    let mut s = 1.;
    if d < f32::MIN_POSITIVE * 2. {
        n *= F1_25;
        d *= F1_25;
        s = 1. / F1_25;
    }
    let rd = 1. / d;
    let mut r = Doubled::from(n);
    let mut qisodd = false;

    for _ in 0..8 {
        // ceil(log2(FLT_MAX) / 22)+1
        let mut q = rintfk2(r.0 * rd);
        if fabsfk(r.0) < 1.5 * d {
            q = if r.0 < 0. { -1. } else { 1. };
        }
        if fabsfk(r.0) < 0.5 * d || (fabsfk(r.0) == 0.5 * d && !qisodd) {
            q = 0.;
        }
        if q == 0. {
            break;
        }
        if (q * -d).is_infinite() {
            q += (-1.0).mul_sign(r.0);
        }
        qisodd ^= (1 & (q as isize)) != 0 && fabsfk(q) < F1_24; // TODO: check
        r = (r + q.mul_as_doubled(-d)).normalize();
    }

    let mut ret = r.0 * s;
    ret = ret.mul_sign(x);
    if y.is_infinite() {
        ret = if x.is_infinite() { f32::NAN } else { x };
    }
    if d == 0. {
        f32::NAN
    } else {
        ret
    }
}

#[test]
fn test_remainderf() {
    test_ff_f(
        remainderf,
        rug::Float::remainder,
        f32::MIN..=f32::MAX,
        f32::MIN..=f32::MAX,
        0.5,
    );
}

/// Fused multiply and accumulate
///
/// This function compute (***x*** × ***y*** + ***z***) without rounding, and then return the rounded value of the result.
/// This function may return infinity with a correct sign if the absolute value of the correct return value is greater than `1e+33`.
/// The error bounds of the returned value is `max(0.500_01 ULP, f32::MIN_POSITIVE)`.
pub fn fmaf(mut x: f32, mut y: f32, mut z: f32) -> f32 {
    const C0: f32 = F1_25;
    const C1: f32 = C0 * C0;
    const C2: f32 = C1 * C1;

    let mut h2 = x * y + z;
    let q = if fabsfk(h2) < 1e-38 {
        x *= C1;
        y *= C1;
        z *= C2;
        1. / C2
    } else if fabsfk(h2) > 1e+38 {
        x *= 1. / C1;
        y *= 1. / C1;
        z *= 1. / C2;
        C2
    } else {
        1.
    };

    let mut d = x.mul_as_doubled(y);
    d += z;
    let ret = if (x == 0.) || (y == 0.) {
        z
    } else {
        f32::from(d)
    };
    if z.is_infinite() && !x.is_infinite() && !x.is_nan() && !y.is_infinite() && !y.is_nan() {
        h2 = z;
    }
    if h2.is_infinite() || h2.is_nan() {
        h2
    } else {
        ret * q
    }
}

/*
/// Square root function
///
/// The error bound of the returned value is `0.5001 ULP`
pub fn sqrtf(d: f32) -> f32 {
    SQRTF(d)
}
*/
