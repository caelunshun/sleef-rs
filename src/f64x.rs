#[macro_use]
mod u05_impl;
#[macro_use]
mod u10_impl;
#[macro_use]
mod u15_impl;
#[macro_use]
mod u35_impl;

macro_rules! impl_math_f64 {
    ($f64x:ident, $u64x:ident, $m64x:ident, $i64x:ident, $ux:ident, $mx:ident, $ix:ident) => {
        use crate::common::*;
        use doubled::*;

        const ZERO: $f64x = $f64x::splat(0.);
        const ONE: $f64x = $f64x::splat(1.);
        const HALF: $f64x = $f64x::splat(0.5);
        const D1_63X: $f64x = $f64x::splat((1u64 << 63) as f64);
        const D1_60X: $f64x = $f64x::splat((1u64 << 60) as f64);
        const D1_54X: $f64x = $f64x::splat((1u64 << 54) as f64);
        const D1_53X: $f64x = $f64x::splat((1u64 << 53) as f64);
        const D1_52X: $f64x = $f64x::splat((1u64 << 52) as f64);
        const D1_32X: $f64x = $f64x::splat((1u64 << 32) as f64);
        const D1_31X: $f64x = $f64x::splat((1u64 << 31) as f64);
        const D1_28X: $f64x = $f64x::splat((1u64 << 28) as f64);
        const D1_24X: $f64x = $f64x::splat((1u64 << 24) as f64);
        const D1_23X: $f64x = $f64x::splat((1u64 << 23) as f64);

        const PI_A: $f64x = $f64x::splat(3.141_592_621_803_283_691_4);
        const PI_B: $f64x = $f64x::splat(3.178_650_942_459_171_346_9_e-8);
        const PI_C: $f64x = $f64x::splat(1.224_646_786_410_718_850_2_e-16);
        const PI_D: $f64x = $f64x::splat(1.273_663_432_702_189_981_6_e-24);
        const TRIGRANGEMAX: $f64x = $f64x::splat(1e+14);

        const PI_A2: $f64x = $f64x::splat(3.141_592_653_589_793_116);
        const PI_B2: $f64x = $f64x::splat(1.224_646_799_147_353_207_2_e-16);
        const TRIGRANGEMAX2: $f64x = $f64x::splat(15.);

        const SLEEF_FP_ILOGB0: $f64x = $f64x::splat(-2_147_483_648 as f64);
        const SLEEF_FP_ILOGBNAN: $f64x = $f64x::splat(2_147_483_647 as f64);
        const SQRT_DBL_MAX: $f64x = $f64x::splat(1.340_780_792_994_259_635_5_e+154);
        const M_2_PI_H: $f64x = $f64x::splat(0.636_619_772_367_581_382_43);
        const M_2_PI_L: $f64x = $f64x::splat(-3.935_735_335_036_497_176_4_e-17);
        const TRIGRANGEMAX3: $f64x = $f64x::splat(1e+9);
        const L2U: $f64x = $f64x::splat(0.693_147_180_559_662_956_511_601_805_686_950_683_593_75);
        const L2L: $f64x = $f64x::splat(0.282_352_905_630_315_771_225_884_481_750_134_360_255_254_120_68_e-12);
        const R_LN2: $f64x = $f64x::splat(1.442_695_040_888_963_407_359_924_681_001_892_137_426_645_954_152_985_934_135_449_406_931);
        const L10U: $f64x = $f64x::splat(0.301_029_995_663_839_144_98); // log 2 / log 10
        const L10L: $f64x = $f64x::splat(1.420_502_322_726_609_941_8_e-13);
        const LOG10_2: $f64x = $f64x::splat(3.321_928_094_887_362_347_870_319_429_489_390_175_864_831_393);

        //---------???????
        //--------- Naive implementation ???????
        #[inline]
        fn vandnot_vm_vm_vm(x: $u64x, y: $u64x) -> $u64x { y & !x }

        #[inline]
        fn vandnot_vo_vo_vo(x: $m64x, y: $m64x) -> $m64x { y & !x }

        #[inline]
        fn vand_vm_vo64_vm(x: $m64x, y: $u64x) -> $u64x { $u64x::from_bits(x) & y }
        #[inline]
        fn vor_vm_vo64_vm(x: $m64x, y: $u64x) -> $u64x { $u64x::from_bits(x) | y }
        #[inline]
        fn vandnot_vm_vo64_vm(x: $m64x, y: $u64x) -> $u64x { y & !$u64x::from_bits(x) }

        #[inline]
        fn vandnot_vi_vi_vi(x: $ix, y: $ix) -> $ix { y & !x }

        #[inline]
        fn vand_vi_vo_vi(x: $mx, y: $ix) -> $ix { $ix::from_bits(x) & y }
        #[inline]
        fn vandnot_vi_vo_vi(x: $mx, y: $ix) -> $ix { $ix::from_bits(x) & !y }

        #[inline]
        fn veq_vi2_vi2_vi2(x: $i64x, y: $i64x) -> $i64x { $i64x::from_bits(x.eq(y)) }

        impl Round for $f64x {
            type Int = $ix;
            #[inline]
            fn trunc(self) -> Self {
                Self::from_cast(self.trunci())
            }
            #[inline]
            fn trunci(self) -> Self::Int {
                Self::Int::from_cast(self)
            }
            #[inline]
            fn round(self) -> Self {
                rint(self)
            }
            #[inline]
            fn roundi(self) -> Self::Int {
                Self::Int::from_cast(self.round())
            }
        }

        impl MulSub for $f64x {
            #[inline]
            fn mul_sub(self, y: Self, z: Self) -> Self {
                self*y - z
            }
        }

        #[inline]
        fn vgather_vd_p_vi(ptr: &[f64], vi: $ix) -> $f64x {
            let mut ar = [0_f64; $f64x::lanes()];
            for i in 0..$f64x::lanes() {
                ar[i] = ptr[vi.extract(i) as usize];
            }
            $f64x::from_slice_aligned(&ar)
        }

        #[inline]
        fn vrev21_vi2_vi2(i: $i64x) -> $i64x {
            const L : usize = $i64x::lanes()*2;
            let i2 = Simd::<[i32;L]>::from_bits(i);
            let mut r = [0_i32;L];
            i2.write_to_slice_aligned(&mut r);
            for i in 0..L/2 {
                r.swap(i*2, i*2+1);
            }
            $i64x::from_bits(Simd::<[i32;L]>::from_slice_aligned(&r))
        }

        //----------???????
        //----------???????

        // ------------

        impl SqrtAsDoubled for $f64x {
            #[inline]
            fn sqrt_as_doubled(self) -> Doubled<Self> {
                let t = self.sqrt();
                ((self + t.mul_as_doubled(t)) * t.recpre_as_doubled()).scale(Self::splat(0.5))
            }
        }
        // --------------------
        impl VectorizedSelect<f64> for $m64x {
            type Output = $f64x;
            fn select_splat(self, l: f64, r: f64) -> Self::Output {
                self.select(Self::Output::splat(l), Self::Output::splat(r))
            }
        }
        impl DoubledSelect<$f64x> for $m64x {
            fn select_doubled(self, l: Doubled<$f64x>, r: Doubled<$f64x>) -> Doubled<$f64x> {
                Doubled::new(self.select(l.0, r.0), self.select(l.1, r.1))
            }
        }

        #[inline]
        fn vsel_vd_vo_vo_d_d_d(o0: $m64x, o1: $m64x, d0: f64, d1: f64, d2: f64) -> $f64x {
            o0.select($f64x::splat(d0), o1.select_splat(d1, d2))
        }

        #[inline]
        fn vsel_vd_vo_vo_vo_d_d_d_d(
            o0: $m64x,
            o1: $m64x,
            o2: $m64x,
            d0: f64,
            d1: f64,
            d2: f64,
            d3: f64,
        ) -> $f64x {
            o0.select(
                $f64x::splat(d0),
                o1.select($f64x::splat(d1), o2.select_splat(d2, d3)),
            )
        }
        // -------------------

        #[inline]
        fn vsel_vd2_vo_d_d_d_d(o: $m64x, x1: f64, y1: f64, x0: f64, y0: f64) -> Doubled<$f64x> {
            Doubled::new(o.select_splat(x1, x0), o.select_splat(y1, y0))
        }
        // -----------
        impl Sign for $f64x {
            type Mask = $m64x;
            type Bits = $u64x;
            #[inline]
            fn is_sign_negative(self) -> Self::Mask {
                self.sign_bit().ne(Self::Bits::splat(0))
            }
            #[inline]
            fn is_sign_positive(self) -> Self::Mask {
                !self.is_sign_negative()
            }
            #[inline]
            fn sign_bit(self) -> Self::Bits {
                Self::Bits::from_bits(self) & Self::Bits::from_bits(Self::splat(-0.))
            }
            #[inline]
            fn sign(self) -> Self {
                ONE.mul_sign(self)
            }
            #[inline]
            fn mul_sign(self, other: Self) -> Self {
                Self::from_bits(Self::Bits::from_bits(self) ^ other.sign_bit())
            }
            #[inline]
            fn copy_sign(self, other: Self) -> Self {
                Self::from_bits(
                    vandnot_vm_vm_vm(Self::Bits::from_bits(Self::splat(-0.)), Self::Bits::from_bits(self))
                        ^ (other.sign_bit()),
                )
            }
        }

        // return d0 < d1 ? x : y
        #[inline]
        fn vsel_vi_vd_vd_vi_vi(d0: $f64x, d1: $f64x, x: $ix, y: $ix) -> $ix {
            d0.lt(d1).select(x, y)
        }

        // return d0 < 0 ? x : 0
        #[inline]
        fn vsel_vi_vd_vi(d: $f64x, x: $ix) -> $ix {
            vand_vi_vo_vi($mx::from_cast(d.is_sign_negative()), x)
        }
        #[inline]
        fn visnegzero_vo_vd(d: $f64x) -> $m64x {
            $u64x::from_bits(d).eq($u64x::from_bits($f64x::splat(-0.)))
        }
        #[inline]
        fn vpow2i_vd_vi(q: $ix) -> $f64x {
            let q = $ix::splat(0x3ff) + q;
            let r = $i64x::from_cast(q);
            $f64x::from_bits(r << 20)
        }
        #[inline]
        fn vldexp_vd_vd_vi(x: $f64x, q: $ix) -> $f64x {
            let mut m = q >> 31;
            m = (((m + q) >> 9) - m) << 7;
            let q = q - (m << 2);
            m = $ix::splat(0x3ff) + m;
            m = vandnot_vi_vo_vi($ix::splat(0).gt(m), m);
            m = m.gt($ix::splat(0x7ff)).select($ix::splat(0x7ff), m);
            let r = $i64x::from_cast(m);
            let y = $f64x::from_bits(r << 20);
            x * y * y * y * y * vpow2i_vd_vi(q)
        }
        #[inline]
        fn vldexp2_vd_vd_vi(d: $f64x, e: $ix) -> $f64x {
            d * vpow2i_vd_vi(e >> 1) * vpow2i_vd_vi(e - (e >> 1))
        }
        #[inline]
        fn vldexp3_vd_vd_vi(d: $f64x, q: $ix) -> $f64x {
            $f64x::from_bits($i64x::from_bits(d) + ($i64x::from_cast(q) << 20))
        }

        /*#[cfg(all(
            not(feature = "enable_avx512f"),
            not(feature = "enable_avx512fnofma")
        ))]*/
        #[inline]
        fn vilogbk_vi_vd(mut d: $f64x) -> $ix {
            let o = d.lt($f64x::splat(4.909_093_465_297_726_6_e-91));
            d = o.select($f64x::splat(2.037_035_976_334_486_e90) * d, d);
            let mut q = $ix::from_cast($u64x::from_bits(d));
            q &= $ix::splat(((1 << 12) - 1) << 20);
            q = $ix::from_bits($ux::from_bits(q) >> 20);
            q - $mx::from_cast(o).select($ix::splat(300 + 0x3ff), $ix::splat(0x3ff))
        }
        /*#[cfg(all(
            not(feature = "enable_avx512f"),
            not(feature = "enable_avx512fnofma")
        ))]*/
        #[inline]
        fn vilogb2k_vi_vd(d: $f64x) -> $ix {
            let mut q = $ix::from_cast($u64x::from_bits(d));
            q = $ix::from_bits($ux::from_bits(q) >> 20);
            q &= $ix::splat(0x7ff);
            q - $ix::splat(0x3ff)
        }

        impl IsInt for $f64x {
            type Mask = $m64x;
            #[inline]
            fn is_integer(self) -> Self::Mask {
                let mut x = (self * (ONE / D1_31X)).trunc();
                x = (-D1_31X).mul_add(x, self);
                x.trunc().eq(x) | self.abs().gt(D1_53X)
            }
        }

        #[inline]
        fn visodd_vo_vd(d: $f64x) -> $m64x {
            let mut x = (d * (ONE / D1_31X)).trunc();
            x = (-D1_31X).mul_add(x, d);

            $m64x::from_cast((x.trunci() & $ix::splat(1)).eq($ix::splat(1)))
                & d.abs().lt(D1_53X)
        }

        //

        pub fn ldexp(x: $f64x, q: $ix) -> $f64x {
            vldexp_vd_vd_vi(x, q)
        }

        pub fn ilogb(d: $f64x) -> $ix {
            let mut e = $f64x::from_cast(vilogbk_vi_vd(d.abs()));
            e = d
                .eq(ZERO)
                .select(SLEEF_FP_ILOGB0, e);
            e = d.is_nan().select(SLEEF_FP_ILOGBNAN, e);
            e = d.is_infinite().select($f64x::splat(f64::MAX), e);
            e.roundi()
        }

        #[inline]
        fn rempisub(x: $f64x) -> ($f64x, $ix) {
            if cfg!(feature = "full_fp_rounding") {
                let y = (x * $f64x::splat(4.)).round();
                let vi = (y - x.round() * $f64x::splat(4.)).trunci();
                (x - y * $f64x::splat(0.25), vi)
            } else {
                let mut fr = x - D1_28X * (x * (ONE / D1_28X)).trunc();
                let mut vi = $mx::from_cast(x.gt(ZERO))
                    .select($ix::splat(4), $ix::splat(3))
                    + (fr * $f64x::splat(8.)).trunci();
                vi = (($ix::splat(7) & vi) - $ix::splat(3)) >> 1;
                fr = fr - $f64x::splat(0.25) * fr
                    .mul_add($f64x::splat(4.), HALF.mul_sign(x))
                    .trunc();
                fr = fr
                    .abs()
                    .gt($f64x::splat(0.25))
                    .select(fr - HALF.mul_sign(x), fr);
                fr = fr.abs().gt($f64x::splat(1e+10)).select(ZERO, fr);
                let o = x.abs().eq($f64x::splat(0.124_999_999_999_999_986_12));
                fr = o.select(x, fr);
                vi = $mx::from_cast(o).select($ix::splat(0), vi);
                (fr, vi)
            }
        }
        #[inline]
        fn rempi(mut a: $f64x) -> (Doubled<$f64x>, $ix) {
            let mut ex = vilogb2k_vi_vd(a);
            /*if cfg!(feature = "enable_avx512f") || cfg!(feature = "enable_avx512fnofma") {
                ex = vandnot_vi_vi_vi(ex >> 31, ex);
                ex = ex & $ix::splat(1023);
            }*/
            ex -= $ix::splat(55);
            let mut q = vand_vi_vo_vi(ex.gt($ix::splat(700 - 55)), $ix::splat(-64));
            a = vldexp3_vd_vd_vi(a, q);
            ex = vandnot_vi_vi_vi(ex >> 31, ex);
            ex <<= 2;
            let mut x = a.mul_as_doubled(vgather_vd_p_vi(&crate::tables::REMPITABDP, ex));
            let (did, dii) = rempisub(x.0);
            q = dii;
            x.0 = did;
            x = x.normalize();
            let mut y = a.mul_as_doubled(vgather_vd_p_vi(&crate::tables::REMPITABDP[1..], ex));
            x += y;
            let (did, dii) = rempisub(x.0);
            q += dii;
            x.0 = did;
            x = x.normalize();
            y = Doubled::new(
                vgather_vd_p_vi(&crate::tables::REMPITABDP[2..], ex),
                vgather_vd_p_vi(&crate::tables::REMPITABDP[3..], ex),
            );
            y *= a;
            x += y;
            x = x.normalize();
            x *= Doubled::from((3.141_592_653_589_793_116 * 2., 1.224_646_799_147_353_207_2_e-16 * 2.));
            let o = a.abs().lt($f64x::splat(0.7));
            x.0 = o.select(a, x.0);
            x.1 = $f64x::from_bits(vandnot_vm_vo64_vm(o, $u64x::from_bits(x.1)));
            (x, q)
        }

        #[inline]
        fn cospik(d: $f64x) -> Doubled<$f64x> {
            let u = d * $f64x::splat(4.);
            let mut q = u.trunci();
            q = (q + ($ix::from_bits($ux::from_bits(q) >> 31) ^ $ix::splat(1))) & $ix::splat(!1);
            let o = $m64x::from_cast((q & $ix::splat(2)).eq($ix::splat(0)));

            let s = u - $f64x::from_cast(q);
            let t = s;
            let s = s * s;
            let s2 = t.mul_as_doubled(t);

            //

            let u = o.select_splat(9.944_803_876_268_437_740_902_08_e-16,
                -2.024_611_207_851_823_992_958_68_e-14,
            ).mul_add(
                s,
                o.select_splat(-3.897_962_260_629_327_991_640_47_e-13,
                    6.948_218_305_801_794_613_277_84_e-12,
                ),
            ).mul_add(
                s,
                o.select_splat(1.150_115_825_399_960_352_669_01_e-10,
                    -1.757_247_499_528_531_799_526_64_e-9,
                ),
            ).mul_add(
                s,
                o.select_splat(-2.461_136_950_104_469_749_535_9_e-8,
                    3.133_616_889_668_683_928_784_22_e-7,
                ),
            ).mul_add(
                s,
                o.select_splat(3.590_860_448_590_527_540_050_62_e-6,
                    -3.657_620_418_216_155_192_036_1_e-5,
                ),
            ).mul_add(
                s,
                o.select_splat(-0.000_325_991_886_927_389_905_997_954,
                    0.002_490_394_570_192_718_502_743_560,
                ),
            );
            let mut x = u * s + vsel_vd2_vo_d_d_d_d(
                o,
                0.015_854_344_243_815_501_891_425_9,
                -1.046_932_722_806_315_219_088_45_e-18,
                -0.080_745_512_188_280_785_248_473_1,
                3.618_524_750_670_371_048_499_87_e-18,
            );
            x = s2 * x + vsel_vd2_vo_d_d_d_d(
                o,
                -0.308_425_137_534_042_437_259_529,
                -1.956_984_921_336_335_503_383_45_e-17,
                0.785_398_163_397_448_278_999_491,
                3.062_871_137_271_550_026_071_05_e-17,
            );

            x *= o.select_doubled(s2, Doubled::new(t, ZERO));
            x = o.select_doubled(x + ONE, x);

            let o = $m64x::from_cast((q + $ix::splat(2) & $ix::splat(4)).eq($ix::splat(4)));
            x.0 = $f64x::from_bits(
                vand_vm_vo64_vm(o, $u64x::from_bits($f64x::splat(-0.))) ^ $u64x::from_bits(x.0),
            );
            x.1 = $f64x::from_bits(
                vand_vm_vo64_vm(o, $u64x::from_bits($f64x::splat(-0.))) ^ $u64x::from_bits(x.1),
            );

            x
        }

        #[inline]
        fn atan2k(y: $f64x, x: $f64x) -> $f64x {
            let q = vsel_vi_vd_vi(x, $ix::splat(-2));
            let x = x.abs();

            let q = vsel_vi_vd_vd_vi_vi(x, y, q + $ix::splat(1), q);
            let p = x.lt(y);
            let s = p.select(-x, y);
            let mut t = x.max(y);

            let s = s / t;
            t = s * s;

            let mut u: $f64x;
            if cfg!(feature = "split_kernel") {
                let t2 = t * t;

                u = $f64x::splat(-1.887_960_084_630_734_965_637_46_e-5)
                    .mul_add(t2, $f64x::splat(-0.001_106_118_314_866_724_825_634_71))
                    .mul_add(t2, $f64x::splat(-0.008_898_961_958_876_554_917_408_09))
                    .mul_add(t2, $f64x::splat(-0.025_451_762_493_231_264_161_686_1))
                    .mul_add(t2, $f64x::splat(-0.040_762_919_127_683_650_000_193_4))
                    .mul_add(t2, $f64x::splat(-0.052_367_485_230_348_245_761_611_3))
                    .mul_add(t2, $f64x::splat(-0.066_657_357_936_108_052_598_456_2))
                    .mul_add(t2, $f64x::splat(-0.090_908_995_008_245_008_229_153))
                    .mul_add(t2, $f64x::splat(-0.142_857_142_667_713_293_837_65))
                    .mul_add(t2, $f64x::splat(-0.333_333_333_333_311_110_369_124));

                let v = $f64x::splat(0.000_209_850_076_645_816_976_906_797)
                    .mul_add(t2, $f64x::splat(0.003_700_267_441_887_131_192_324_03))
                    .mul_add(t2, $f64x::splat(0.016_599_329_773_529_201_970_117))
                    .mul_add(t2, $f64x::splat(0.033_785_258_000_135_306_999_389_7))
                    .mul_add(t2, $f64x::splat(0.046_666_715_007_784_062_563_267_5))
                    .mul_add(t2, $f64x::splat(0.058_766_639_292_667_358_085_431_3))
                    .mul_add(t2, $f64x::splat(0.076_921_953_831_176_961_835_502_9))
                    .mul_add(t2, $f64x::splat(0.111_111_105_648_261_418_443_745))
                    .mul_add(t2, $f64x::splat(0.199_999_999_996_591_265_594_148));

                u = v.mul_add(t, u);
            } else {
                u = $f64x::splat(-1.887_960_084_630_734_965_637_46_e-5)
                    .mul_add(t, $f64x::splat(0.000_209_850_076_645_816_976_906_797))
                    .mul_add(t, $f64x::splat(-0.001_106_118_314_866_724_825_634_71))
                    .mul_add(t, $f64x::splat(0.003_700_267_441_887_131_192_324_03))
                    .mul_add(t, $f64x::splat(-0.008_898_961_958_876_554_917_408_09))
                    .mul_add(t, $f64x::splat(0.016_599_329_773_529_201_970_117))
                    .mul_add(t, $f64x::splat(-0.025_451_762_493_231_264_161_686_1))
                    .mul_add(t, $f64x::splat(0.033_785_258_000_135_306_999_389_7))
                    .mul_add(t, $f64x::splat(-0.040_762_919_127_683_650_000_193_4))
                    .mul_add(t, $f64x::splat(0.046_666_715_007_784_062_563_267_5))
                    .mul_add(t, $f64x::splat(-0.052_367_485_230_348_245_761_611_3))
                    .mul_add(t, $f64x::splat(0.058_766_639_292_667_358_085_431_3))
                    .mul_add(t, $f64x::splat(-0.066_657_357_936_108_052_598_456_2))
                    .mul_add(t, $f64x::splat(0.076_921_953_831_176_961_835_502_9))
                    .mul_add(t, $f64x::splat(-0.090_908_995_008_245_008_229_153))
                    .mul_add(t, $f64x::splat(0.111_111_105_648_261_418_443_745))
                    .mul_add(t, $f64x::splat(-0.142_857_142_667_713_293_837_65))
                    .mul_add(t, $f64x::splat(0.199_999_999_996_591_265_594_148))
                    .mul_add(t, $f64x::splat(-0.333_333_333_333_311_110_369_124));
            }

            t = s.mul_add(t * u, s);
            $f64x::from_cast(q).mul_add($f64x::FRAC_PI_2, t)
        }
        #[inline]
        fn visinf2_vd_vd_vd(d: $f64x, m: $f64x) -> $f64x {
            $f64x::from_bits(vand_vm_vo64_vm(
                d.is_infinite(),
                ($u64x::from_bits(d) & $u64x::from_bits($f64x::splat(-0.))) | $u64x::from_bits(m),
            ))
        }

        #[inline]
        fn expm1k(d: $f64x) -> $f64x {
            let mut u = (d * R_LN2).round();
            let q = u.roundi();

            let s = u.mul_add(-L2U, d);
            let s = u.mul_add(-L2L, s);

            u = $f64x::splat(2.088_606_211_072_836_875_363_41_e-9)
                .mul_add(s, $f64x::splat(2.511_129_308_928_765_186_106_61_e-8))
                .mul_add(s, $f64x::splat(2.755_739_112_349_004_718_933_38_e-7))
                .mul_add(s, $f64x::splat(2.755_723_629_119_288_276_294_23_e-6))
                .mul_add(s, $f64x::splat(2.480_158_715_923_547_299_879_1_e-5))
                .mul_add(s, $f64x::splat(0.000_198_412_698_960_509_205_564_975))
                .mul_add(s, $f64x::splat(0.001_388_888_888_897_744_922_079_62))
                .mul_add(s, $f64x::splat(0.008_333_333_333_316_527_216_649_84))
                .mul_add(s, $f64x::splat(0.041_666_666_666_666_504_759_142_2))
                .mul_add(s, $f64x::splat(0.166_666_666_666_666_851_703_837))
                .mul_add(s, HALF);
            u = (s * s).mul_add(u, s);

            $m64x::from_cast(q.eq($ix::splat(0))).select(
                u,
                vldexp2_vd_vd_vi(u + ONE, q) - ONE,
            )
        }
        #[inline]
        fn logk(mut d: $f64x) -> Doubled<$f64x> {
            let m: $f64x;

            let mut s =
                /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma")*/ {
                    let o = d.lt($f64x::splat(f64::MIN));
                    d = o.select(d * (D1_32X * D1_32X), d);
                    let mut e = vilogb2k_vi_vd(d * $f64x::splat(1. / 0.75));
                    m = vldexp3_vd_vd_vi(d, -e);
                    e = $mx::from_cast(o).select(e - $ix::splat(64), e);
                    Doubled::from((0.693_147_180_559_945_286_226_764, 2.319_046_813_846_299_558_417_771_e-17))
                        * $f64x::from_cast(e)
                }/* else {
                    let mut e = vgetexp_vd_vd(d * $f64x::splat(1. / 0.75));
                    e = e.eq($f64x::INFINITY).select($f64x::splat(1024.), e);
                    m = vgetmant_vd_vd(d);
                    Doubled::new(
                        $f64x::splat(0.693_147_180_559_945_286_226_764),
                        $f64x::splat(2.319_046_813_846_299_558_417_771_e-17),
                    ) * e
                }*/;

            let x = $f64x::splat(-1.).add_as_doubled(m) / ONE.add_as_doubled(m);
            let x2 = x.square();

            let t = $f64x::splat(0.116_255_524_079_935_043_668_677)
                .mul_add(x2.0, $f64x::splat(0.103_239_680_901_072_952_701_192))
                .mul_add(x2.0, $f64x::splat(0.117_754_809_412_463_995_466_069))
                .mul_add(x2.0, $f64x::splat(0.133_329_810_868_462_739_215_09))
                .mul_add(x2.0, $f64x::splat(0.153_846_227_114_512_262_845_736))
                .mul_add(x2.0, $f64x::splat(0.181_818_180_850_050_775_676_507))
                .mul_add(x2.0, $f64x::splat(0.222_222_222_230_083_560_345_903))
                .mul_add(x2.0, $f64x::splat(0.285_714_285_714_249_172_087_875))
                .mul_add(x2.0, $f64x::splat(0.400_000_000_000_000_077_715_612));
            let c = Doubled::from((0.666_666_666_666_666_629_659_233, 3.805_549_625_424_120_563_366_16_e-17));

            s = s.add_checked(x.scale($f64x::splat(2.)));
            s.add_checked(x2 * x * (x2 * t + c))
        }

        #[inline]
        fn expk(d: Doubled<$f64x>) -> $f64x {
            let mut u = (d.0 + d.1) * R_LN2;
            let dq = u.round();
            let q = dq.roundi();

            let mut s = d + dq * (-L2U);
            s += dq * (-L2L);

            s = s.normalize();

            u = $f64x::splat(2.510_696_834_209_504_195_271_39_e-8)
                .mul_add(s.0, $f64x::splat(2.762_861_667_702_706_491_168_55_e-7))
                .mul_add(s.0, $f64x::splat(2.755_724_967_250_235_741_438_64_e-6))
                .mul_add(s.0, $f64x::splat(2.480_149_739_898_197_941_141_53_e-5))
                .mul_add(s.0, $f64x::splat(0.000_198_412_698_809_069_797_676_111))
                .mul_add(s.0, $f64x::splat(0.001_388_888_893_997_712_896_052_9))
                .mul_add(s.0, $f64x::splat(0.008_333_333_333_323_714_176_010_81))
                .mul_add(s.0, $f64x::splat(0.041_666_666_666_540_952_412_844_9))
                .mul_add(s.0, $f64x::splat(0.166_666_666_666_666_740_681_535))
                .mul_add(s.0, $f64x::splat(0.500_000_000_000_000_999_200_722));

            let mut t = s.add_checked(s.square() * u);

            t = ONE.add_checked(t);
            u = t.0 + t.1;
            u = vldexp2_vd_vd_vi(u, q);

            $f64x::from_bits(vandnot_vm_vo64_vm(
                d.0.lt($f64x::splat(-1000.)),
                $u64x::from_bits(u),
            ))
        }

        #[inline]
        fn expk2(d: Doubled<$f64x>) -> Doubled<$f64x> {
            let u = (d.0 + d.1) * R_LN2;
            let dq = u.round();
            let q = dq.roundi();

            let s = d + dq * (-L2U) + dq * (-L2L);

            let u = $f64x::splat(0.160_247_221_970_993_207_2_e-9)
                .mul_add(s.0, $f64x::splat(0.209_225_518_356_315_700_7_e-8))
                .mul_add(s.0, $f64x::splat(0.250_523_002_378_264_446_5_e-7))
                .mul_add(s.0, $f64x::splat(0.275_572_480_090_213_530_3_e-6))
                .mul_add(s.0, $f64x::splat(0.275_573_189_238_604_437_3_e-5))
                .mul_add(s.0, $f64x::splat(0.248_015_873_560_581_506_5_e-4))
                .mul_add(s.0, $f64x::splat(0.198_412_698_414_807_185_8_e-3))
                .mul_add(s.0, $f64x::splat(0.138_888_888_888_676_325_5_e-2))
                .mul_add(s.0, $f64x::splat(0.833_333_333_333_334_709_5_e-2))
                .mul_add(s.0, $f64x::splat(0.416_666_666_666_666_990_5_e-1));

            let mut t = s * u + $f64x::splat(0.166_666_666_666_666_657_4);
            t = s * t + HALF;
            t = s + s.square() * t;

            t = ONE.add_checked(t);

            t.0 = vldexp2_vd_vd_vi(t.0, q);
            t.1 = vldexp2_vd_vd_vi(t.1, q);

            t.0 = $f64x::from_bits(vandnot_vm_vo64_vm(
                d.0.lt($f64x::splat(-1000.)),
                $u64x::from_bits(t.0),
            ));
            t.1 = $f64x::from_bits(vandnot_vm_vo64_vm(
                d.0.lt($f64x::splat(-1000.)),
                $u64x::from_bits(t.1),
            ));

            t
        }

        #[inline]
        fn logk2(d: Doubled<$f64x>) -> Doubled<$f64x> {
            let e = vilogbk_vi_vd(d.0 * $f64x::splat(1. / 0.75));

            let m = Doubled::new(vldexp2_vd_vd_vi(d.0, -e), vldexp2_vd_vd_vi(d.1, -e));

            let x = (m + $f64x::splat(-1.)) / (m + ONE);
            let x2 = x.square();

            let t = $f64x::splat(0.138_604_363_904_671_679_108_56)
                .mul_add(x2.0, $f64x::splat(0.131_699_838_841_615_374_240_845))
                .mul_add(x2.0, $f64x::splat(0.153_914_168_346_271_945_653_214))
                .mul_add(x2.0, $f64x::splat(0.181_816_523_941_564_611_721_589))
                .mul_add(x2.0, $f64x::splat(0.222_222_246_326_620_354_039_96))
                .mul_add(x2.0, $f64x::splat(0.285_714_285_511_134_091_777_308))
                .mul_add(x2.0, $f64x::splat(0.400_000_000_000_914_013_309_483))
                .mul_add(x2.0, $f64x::splat(0.666_666_666_666_664_853_302_393));

            let mut s = Doubled::from((0.693_147_180_559_945_286_226_764, 2.319_046_813_846_299_558_417_771_e-17))
                * $f64x::from_cast(e);
            s = s.add_checked(x.scale($f64x::splat(2.)));
            s.add_checked(x2 * x * t)
        }

        //
        #[inline]
        fn vcast_vi2_i_i(i0: i32, i1: i32) -> $i64x {
              const L : usize = $i64x::lanes();
              let mut a = [0_i32;L*2];
              for j in 0..L {
                a[2*j] = i0;
                a[2*j+1] = i1;
          }
          $i64x::from_bits(Simd::<[i32; L*2]>::from_slice_aligned(&a))
        }

        #[inline]
        fn vcast_vi2_u_u(i0: u32, i1: u32) -> $i64x {
              const L : usize = $i64x::lanes();
              let mut a = [0_u32;L*2];
              for j in 0..L {
                a[2*j] = i0;
                a[2*j+1] = i1;
          }
          $i64x::from_bits(Simd::<[u32; L*2]>::from_slice_aligned(&a))
        }

        #[inline]
        pub fn fabs(x: $f64x) -> $f64x {
            x.abs()
        }

        #[inline]
        pub fn copysign(x: $f64x, y: $f64x) -> $f64x {
            x.copy_sign(y)
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] //  && !defined(ENABLE_VECEXT) && !defined(ENABLE_PUREC)
        pub fn fmax(x: $f64x, y: $f64x) -> $f64x {
            y.is_nan().select(x, x.max(y))
        }
        #[cfg(all(not(target_arch = "x86"), not(target_arch = "x86_64")))]
        pub fn fmax(x: $f64x, y: $f64x) -> $f64x {
            y.is_nan().select(x, x.gt(y).select(x, y))
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] //  && !defined(ENABLE_VECEXT) && !defined(ENABLE_PUREC)
        pub fn fmin(x: $f64x, y: $f64x) -> $f64x {
            y.is_nan().select(x, x.min(y))
        }
        #[cfg(all(not(target_arch = "x86"), not(target_arch = "x86_64")))]
        pub fn fmin(x: $f64x, y: $f64x) -> $f64x {
            y.is_nan().select(x, y.gt(x).select(x, y))
        }

        pub fn fdim(x: $f64x, y: $f64x) -> $f64x {
            let ret = x - y;
            (ret.lt(ZERO) | x.eq(y)).select(ZERO, ret)
        }

        pub fn trunc(x: $f64x) -> $f64x {
            let mut fr = x
                - D1_31X
                    * $f64x::from_cast((x * (ONE / D1_31X)).trunci());
            fr -= $f64x::from_cast(fr.trunci());
            (x.is_infinite() | x.abs().ge(D1_52X))
                .select(x, (x - fr).copy_sign(x))
        }

        pub fn floor(x: $f64x) -> $f64x {
            let mut fr = x
                - D1_31X
                    * $f64x::from_cast((x * (ONE / D1_31X)).trunci());
            fr -= $f64x::from_cast(fr.trunci());
            fr = fr.lt(ZERO).select(fr + ONE, fr);
            (x.is_infinite() | x.abs().ge(D1_52X))
                .select(x, (x - fr).copy_sign(x))
        }

        pub fn ceil(x: $f64x) -> $f64x {
            let mut fr = x
                - D1_31X
                    * $f64x::from_cast((x * (ONE / D1_31X)).trunci());
            fr -= $f64x::from_cast(fr.trunci());
            fr = fr.le(ZERO).select(fr, fr - ONE);
            (x.is_infinite() | x.abs().ge(D1_52X))
                .select(x, (x - fr).copy_sign(x))
        }

        pub fn round(d: $f64x) -> $f64x {
            let mut x = d + HALF;
            let mut fr = x
                - D1_31X
                    * $f64x::from_cast((x * (ONE / D1_31X)).trunci());
            fr -= $f64x::from_cast(fr.trunci());
            x = (x.le(ZERO) & fr.eq(ZERO)).select(x - ONE, x);
            fr = fr.lt(ZERO).select(fr + ONE, fr);
            x = d
                .eq($f64x::splat(0.499_999_999_999_999_944_49))
                .select(ZERO, x);
            (d.is_infinite() | d.abs().ge(D1_52X))
                .select(d, (x - fr).copy_sign(d))
        }

        pub fn rint(d: $f64x) -> $f64x {
            let mut x = d + HALF;
            let mut fr = x
                - D1_31X
                    * $f64x::from_cast((x * (ONE / D1_31X)).trunci());
            let isodd = $m64x::from_cast(($ix::splat(1) & fr.trunci()).eq($ix::splat(1)));
            fr -= $f64x::from_cast(fr.trunci());
            fr = (fr.lt(ZERO) | (fr.eq(ZERO) & isodd))
                .select(fr + ONE, fr);
            x = d
                .eq($f64x::splat(0.500_000_000_000_000_111_02))
                .select(ZERO, x);
            (d.is_infinite() | d.abs().ge(D1_52X))
                .select(d, (x - fr).copy_sign(d))
        }

        pub fn nextafter(x: $f64x, y: $f64x) -> $f64x {
            let x = x
                .eq(ZERO)
                .select(ZERO.mul_sign(y), x);
            let mut xi2 = $i64x::from_bits(x);
            let c = x.is_sign_negative() ^ y.ge(x);

            let mut t = (xi2 ^ vcast_vi2_u_u(0x_7fff_ffff, 0x_ffff_ffff)) + vcast_vi2_i_i(0, 1);
            t += vrev21_vi2_vi2(vcast_vi2_i_i(0, 1) & veq_vi2_vi2_vi2(t, vcast_vi2_i_i(-1, 0)));
            xi2 = $i64x::from_bits(c.select($f64x::from_bits(t), $f64x::from_bits(xi2)));

            xi2 -= $i64x::from_cast(vand_vm_vo64_vm(x.ne(y), $u64x::from_u32((0, 1))));

            xi2 = $i64x::from_bits(x.ne(y).select(
                $f64x::from_bits(
                    xi2 + vrev21_vi2_vi2(
                        vcast_vi2_i_i(0, -1) & veq_vi2_vi2_vi2(xi2, vcast_vi2_i_i(0, -1)),
                    ),
                ),
                $f64x::from_bits(xi2),
            ));

            let mut t = (xi2 ^ vcast_vi2_u_u(0x_7fff_ffff, 0x_ffff_ffff)) + vcast_vi2_i_i(0, 1);
            t += vrev21_vi2_vi2(vcast_vi2_i_i(0, 1) & veq_vi2_vi2_vi2(t, vcast_vi2_i_i(-1, 0)));
            xi2 = $i64x::from_bits(c.select($f64x::from_bits(t), $f64x::from_bits(xi2)));

            let mut ret = $f64x::from_bits(xi2);

            ret = (ret.eq(ZERO) & x.ne(ZERO))
                .select(ZERO.mul_sign(x), ret);

            ret = (x.eq(ZERO) & y.eq(ZERO)).select(y, ret);

            (x.is_nan() | y.is_nan()).select($f64x::NAN, ret)
        }

        pub fn frfrexp(x: $f64x) -> $f64x {
            let x = x
                .abs()
                .lt($f64x::splat(f64::MIN))
                .select(x * D1_63X, x);

            let mut xm = $u64x::from_bits(x);
            xm &= $u64x::from_u32((!0x_7ff0_0000, !0));
            xm |= $u64x::from_u32((0x_3fe0_0000, 0));

            let ret = $f64x::from_bits(xm);

            let ret =
                x.is_infinite().select($f64x::INFINITY.mul_sign(x), ret);
            x.eq(ZERO).select(x, ret)
        }

        pub fn expfrexp(x: $f64x) -> $ix {
            let x = x
                .abs()
                .lt($f64x::splat(f64::MIN))
                .select(x * D1_63X, x);

            let mut ret = $ix::from_cast($u64x::from_bits(x));
            ret = ($ix::from_bits($ux::from_bits(ret) >> 20) & $ix::splat(0x7ff))
                - $ix::splat(0x3fe);

            (x.eq(ZERO) | x.is_nan() | x.is_infinite()).select($ix::splat(0), ret)
        }

        pub fn fma(mut x: $f64x, mut y: $f64x, mut z: $f64x) -> $f64x {
            let mut h2 = x * y + z;
            let mut q = ONE;
            const C0: $f64x = D1_54X;
            let c1: $f64x = C0 * C0;
            let c2: $f64x = c1 * c1;
            let o = h2.abs().lt($f64x::splat(1e-300));
            {
                x = o.select(x * c1, x);
                y = o.select(y * c1, y);
                z = o.select(z * c2, z);
                q = o.select(ONE / c2, q);
            }
            let o = h2.abs().gt($f64x::splat(1e+300));
            {
                x = o.select(x * (ONE / c1), x);
                y = o.select(y * (ONE / c1), y);
                z = o.select(z * (ONE / c2), z);
                q = o.select(c2, q);
            }
            let d = x.mul_as_doubled(y) + z;
            let ret = (x.eq(ZERO) | y.eq(ZERO)).select(z, d.0 + d.1);
            let mut o = z.is_infinite();
            o = vandnot_vo_vo_vo(x.is_infinite(), o);
            o = vandnot_vo_vo_vo(x.is_nan(), o);
            o = vandnot_vo_vo_vo(y.is_infinite(), o);
            o = vandnot_vo_vo_vo(y.is_nan(), o);
            h2 = o.select(z, h2);

            let o = h2.is_infinite() | h2.is_nan();

            o.select(h2, ret * q)
        }

        //#[cfg(feature = "accurate_sqrt")]
        pub fn sqrt(d: $f64x) -> $f64x {
            d.sqrt()
        }
        // fall back to approximation if ACCURATE_SQRT is undefined
        /*#[cfg(not(feature = "accurate_sqrt"))]
        pub fn xsqrt(d: $f64x) -> $f64x {
            u05::sqrt(d)
        }*/

        #[inline]
        fn vtoward0(x: $f64x) -> $f64x {
            // returns nextafter(x, 0)
            let t = $f64x::from_bits($u64x::from_bits(x) + $u64x::from_bits(vcast_vi2_i_i(-1, -1)));
            x.eq(ZERO).select(ZERO, t)
        }
        #[cfg(feature = "full_fp_rounding")]
        #[inline]
        fn vptrunc(x: $f64x) -> $f64x {
            // round to integer toward 0, positive argument only
            x.trunc()
        }
        #[cfg(not(feature = "full_fp_rounding"))]
        #[inline]
        fn vptrunc(x: $f64x) -> $f64x {
            let mut fr = (-D1_31X).mul_add(
                $f64x::from_cast((x * (ONE / D1_31X)).trunci()),
                x,
            );
            fr -= $f64x::from_cast(fr.trunci());
            x.abs().ge(D1_52X).select(x, x - fr)
        }

        /* TODO AArch64: potential optimization by using `vfmad_lane_f64` */
        pub fn fmod(x: $f64x, y: $f64x) -> $f64x {
            let nu = x.abs();
            let de = y.abs();
            let s = ONE;
            let o = de.lt($f64x::splat(f64::MIN));
            let nu = o.select(nu * D1_54X, nu);
            let de = o.select(de * D1_54X, de);
            let s = o.select(s * (ONE / D1_54X), s);
            let rde = vtoward0(de.recpre());
            let mut r = Doubled::new(nu, ZERO);

            for _ in 0..21 {
                // ceil(log2(DBL_MAX) / 51) + 1
                let q =
                    ((de + de).gt(r.0) & r.0.ge(de)).select(ONE, vtoward0(r.0) * rde);
                let q = $f64x::from_bits(
                    $u64x::from_bits(vptrunc(q)) & $u64x::from_u32((0x_ffff_ffff, 0x_ffff_fffe)),
                );
                r = (r + q.mul_as_doubled(-de)).normalize();
                if r.0.lt(de).all() {
                    break;
                }
            }

            let mut ret = r.0 * s;
            ret = (r.0 + r.1).eq(de).select(ZERO, ret);

            ret = ret.mul_sign(x);

            ret = nu.lt(de).select(x, ret);
            de.eq(ZERO).select($f64x::NAN, ret)
        }

        /* TODO AArch64: potential optimization by using `vfmad_lane_f64` */
        fn gammak(a: $f64x) -> (Doubled<$f64x>, Doubled<$f64x>) {
            let mut clln = Doubled::from((1., 0.));
            let mut clld = Doubled::from((1., 0.));

            let otiny = a.abs().lt($f64x::splat(1e-306));
            let oref = a.lt(HALF);

            let mut x = otiny.select_doubled(Doubled::from((0., 0.)),
                oref.select_doubled(ONE.add_as_doubled(-a),
                    Doubled::new(a, ZERO),
                ),
            );

            let o0 = HALF.le(x.0) & x.0.le($f64x::splat(1.1));
            let o2 = $f64x::splat(2.3).le(x.0);

            let mut y = ((x + ONE) * x).normalize();
            y = ((x + $f64x::splat(2.)) * y).normalize();
            y = ((x + $f64x::splat(3.)) * y).normalize();
            y = ((x + $f64x::splat(4.)) * y).normalize();

            let o = o2 & x.0.le($f64x::splat(7.));
            clln = o.select_doubled(y, clln);

            x = o.select_doubled(x + $f64x::splat(5.), x);

            let t = o2.select(x.0.recpre(), (x + o0.select_splat(-1., -2.)).normalize().0);

            let u = vsel_vd_vo_vo_d_d_d(
                o2,
                o0,
                -156.801_412_704_022_726_379_848_862,
                0.294_791_677_282_761_419_6_e+2,
                0.707_481_600_086_460_927_9_e-7,
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    1.120_804_464_289_911_606_838_558_16,
                    0.128_145_969_182_782_010_9_e+3,
                    0.400_924_433_300_873_044_3_e-6,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    13.397_985_455_142_589_218_333_060_2,
                    0.261_754_402_578_451_504_3_e+3,
                    0.104_011_464_162_824_694_6_e-5,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -0.116_546_276_599_463_200_848_033_357,
                    0.328_702_285_568_579_043_2_e+3,
                    0.150_834_915_073_332_916_7_e-5,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -1.391_801_093_265_337_481_495_562_41,
                    0.281_814_586_773_034_818_6_e+3,
                    0.128_814_307_493_390_102_e-5,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    0.015_056_113_040_026_424_412_918_973_4,
                    0.172_867_041_467_355_960_5_e+3,
                    0.474_416_774_988_499_393_7_e-6,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    0.179_540_117_061_234_856_098_844_714,
                    0.774_873_576_403_041_681_7_e+2,
                    -0.655_481_630_654_248_990_2_e-7,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -0.002_481_743_600_264_997_730_942_489_28,
                    0.251_285_664_308_093_075_2_e+2,
                    -0.318_925_247_145_259_984_4_e-6,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -0.029_527_880_945_699_120_504_851_034_1,
                    0.576_679_210_614_007_686_8_e+1,
                    0.135_888_382_147_035_537_7_e-6,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    0.000_540_164_767_892_604_515_196_325_186,
                    0.727_027_547_399_618_057_1,
                    -0.434_393_127_715_733_604_e-6,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    0.006_403_362_833_808_069_794_787_256_2,
                    0.839_670_912_457_914_780_9_e-1,
                    0.972_478_589_740_677_955_5_e-6,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -0.000_162_516_262_783_915_816_896_611_252,
                    -0.821_155_866_974_680_459_5_e-1,
                    -0.203_688_605_722_596_601_1_e-5,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -0.001_914_438_498_565_477_526_465_972_39,
                    0.682_883_182_834_188_445_8_e-1,
                    0.437_336_314_181_972_581_5_e-5,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    7.204_895_416_020_010_558_983_115_17_e-5,
                    -0.771_248_133_996_167_151_1_e-1,
                    -0.943_995_126_830_400_867_7_e-5,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    0.000_839_498_720_672_087_279_971_000_786,
                    0.833_749_202_301_731_495_7_e-1,
                    0.205_072_703_037_638_980_4_e-4,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -5.171_790_908_260_592_193_293_944_22_e-5,
                    -0.909_496_493_145_624_251_8_e-1,
                    -0.449_262_018_343_118_401_8_e-4,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -0.000_592_166_437_353_693_882_857_342_347,
                    0.100_099_631_357_592_935_8,
                    0.994_575_123_607_187_593_1_e-4,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    6.972_813_758_365_857_774_037_435_39_e-5,
                    -0.111_334_286_154_420_772_4,
                    -0.223_154_759_903_498_319_6_e-3,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    0.000_784_039_221_720_066_627_493_314_301,
                    0.125_509_667_321_302_087_5,
                    0.509_669_524_710_196_762_2_e-3,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -0.000_229_472_093_621_399_176_949_318_732,
                    -0.144_049_896_784_305_436_8,
                    -0.119_275_391_166_788_697_1_e-2,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    -0.002_681_327_160_493_827_160_473_958_490,
                    0.169_557_177_004_194_981_1,
                    0.289_051_033_074_221_031_e-2,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    0.003_472_222_222_222_222_222_175_164_840,
                    -0.207_385_551_028_409_276_2,
                    -0.738_555_102_867_446_185_8_e-2,
                ),
            ).mul_add(
                t,
                vsel_vd_vo_vo_d_d_d(
                    o2,
                    o0,
                    0.083_333_333_333_333_333_335_592_087_900,
                    0.270_580_808_427_781_593_9,
                    0.205_808_084_277_845_533_5_e-1,
                ),
            );

            let mut y = (x + $f64x::splat(-0.5)) * logk2(x);
            y += -x;
            y += Doubled::from((0.918_938_533_204_672_780_56, -3.878_294_158_067_241_449_8_e-17)); // 0.5*log(2*M_PI)

            let mut z = u.mul_as_doubled(t)
                + o0.select_splat(-0.400_685_634_386_531_486_2, -0.673_523_010_531_981_020_1_e-1);
            z = z * t + o0.select_splat(0.822_467_033_424_113_203, 0.322_467_033_424_113_203);
            z = z * t + o0.select_splat(-0.577_215_664_901_532_865_5, 0.422_784_335_098_467_134_5);
            z *= t;

            let mut clc = o2.select_doubled(y, z);

            clld = o2.select_doubled(u.mul_as_doubled(t) + ONE, clld);

            y = clln;

            clc = otiny.select_doubled(Doubled::from((83.177_661_667_193_433_459_033_3, 3.671_034_596_315_685_072_218_78_e-15)), // log(2^120)
                oref.select_doubled(Doubled::<$f64x>::from((1.144_729_885_849_400_163_9, 1.026_595_116_270_782_638_e-17)) + (-clc),
                    clc,
                ),
            ); // log(M_PI)
            clln = otiny.select_doubled(Doubled::from((1., 0.)),
                oref.select_doubled(clln, clld),
            );

            if !(!oref).all() {
                let t = a - D1_28X * $f64x::from_cast((a * (ONE / D1_28X)).trunci());
                x = clld * sinpik(t);
            }

            clld = otiny.select_doubled(Doubled::new(a * (D1_60X * D1_60X), ZERO),
                oref.select_doubled(x, y),
            );

            (clc, clln / clld)
        }

        #[inline]
        fn sinpik(d: $f64x) -> Doubled<$f64x> {
            let u = d * $f64x::splat(4.);
            let mut q = u.trunci();
            q = (q + ($ix::from_bits($ux::from_bits(q) >> 31) ^ $ix::splat(1))) & $ix::splat(!1);
            let o = $m64x::from_cast((q & $ix::splat(2)).eq($ix::splat(2)));

            let s = u - $f64x::from_cast(q);
            let t = s;
            let s = s * s;
            let s2 = t.mul_as_doubled(t);

            //

            let u = o.select_splat(9.944_803_876_268_437_740_902_08_e-16,
                -2.024_611_207_851_823_992_958_68_e-14,
            ).mul_add(
                s,
                o.select_splat(-3.897_962_260_629_327_991_640_47_e-13,
                    6.948_218_305_801_794_613_277_84_e-12,
                ),
            ).mul_add(
                s,
                o.select_splat(1.150_115_825_399_960_352_669_01_e-10,
                    -1.757_247_499_528_531_799_526_64_e-9,
                ),
            ).mul_add(
                s,
                o.select_splat(-2.461_136_950_104_469_749_535_9_e-8,
                    3.133_616_889_668_683_928_784_22_e-7,
                ),
            ).mul_add(
                s,
                o.select_splat(3.590_860_448_590_527_540_050_62_e-6,
                    -3.657_620_418_216_155_192_036_1_e-5,
                ),
            ).mul_add(
                s,
                o.select_splat(-0.000_325_991_886_927_389_905_997_954,
                    0.002_490_394_570_192_718_502_743_56,
                ),
            );
            let mut x = u * s + vsel_vd2_vo_d_d_d_d(
                o,
                0.015_854_344_243_815_501_891_425_9,
                -1.046_932_722_806_315_219_088_45_e-18,
                -0.080_745_512_188_280_785_248_473_1,
                3.618_524_750_670_371_048_499_87_e-18,
            );
            x = s2 * x + vsel_vd2_vo_d_d_d_d(
                o,
                -0.308_425_137_534_042_437_259_529,
                -1.956_984_921_336_335_503_383_45_e-17,
                0.785_398_163_397_448_278_999_491,
                3.062_871_137_271_550_026_071_05_e-17,
            );

            x *= o.select_doubled(s2, Doubled::new(t, ZERO));
            x = o.select_doubled(x + ONE, x);

            let o = $m64x::from_cast((q & $ix::splat(4)).eq($ix::splat(4)));
            x.0 = $f64x::from_bits(
                vand_vm_vo64_vm(o, $u64x::from_bits($f64x::splat(-0.))) ^ $u64x::from_bits(x.0),
            );
            x.1 = $f64x::from_bits(
                vand_vm_vo64_vm(o, $u64x::from_bits($f64x::splat(-0.))) ^ $u64x::from_bits(x.1),
            );

            x
        }

        pub fn modf(x: $f64x) -> ($f64x, $f64x) {
            let mut fr = x
                - D1_31X
                    * $f64x::from_cast((x * (ONE / D1_31X)).trunci());
            fr -= $f64x::from_cast(fr.trunci());
            fr = x.abs().gt(D1_52X).select(ZERO, fr);

            (fr.copy_sign(x), (x - fr).copy_sign(x))
        }

        pub mod u05 {
            //! Functions with 0.5 ULP error bound
            impl_math_f64_u05!($f64x, $u64x, $m64x, $i64x, $ux, $mx, $ix);
        }

        pub mod u10 {
            //! Functions with 1.0 ULP error bound
            impl_math_f64_u10!($f64x, $u64x, $m64x, $i64x, $ux, $mx, $ix);
        }

        pub mod u15 {
            //! Functions with 1.5 ULP error bound
            impl_math_f64_u15!($f64x, $u64x, $m64x, $i64x, $ux, $mx, $ix);
        }

        pub mod u35 {
            //! Functions with 3.5 ULP error bound
            impl_math_f64_u35!($f64x, $u64x, $m64x, $i64x, $ux, $mx, $ix);
        }

    };
}
