macro_rules! impl_math_f32 {
    ($f32x:ident, $u32x:ident, $m32x:ident, $i32x:ident) => {
        use crate::common::*;
        use doubled::*;

        const ZERO: $f32x = $f32x::splat(0.);
        const ONE: $f32x = $f32x::splat(1.);
        const F1_32X: $f32x = $f32x::splat((1u64 << 32) as f32);
        const F1_30X: $f32x = $f32x::splat((1u32 << 30) as f32);
        const F1_25X: $f32x = $f32x::splat((1u32 << 25) as f32);
        const F1_24X: $f32x = $f32x::splat((1u32 << 24) as f32);
        const F1_23X: $f32x = $f32x::splat((1u32 << 23) as f32);
        const F1_12X: $f32x = $f32x::splat((1u32 << 12) as f32);
        const F1_10X: $f32x = $f32x::splat((1u32 << 10) as f32);

        //---------???????
        //--------- Naive implementation ???????
        #[inline]
        fn vandnot_vm_vm_vm(x: $u32x, y: $u32x) -> $u32x { y & !x }

        #[inline]
        fn vandnot_vo_vo_vo(x: $m32x, y: $m32x) -> $m32x { y & !x }

        #[inline]
        fn vand_vm_vo32_vm(x: $m32x, y: $u32x) -> $u32x { $u32x::from_bits(x) & y }
        #[inline]
        fn vor_vm_vo32_vm(x: $m32x, y: $u32x) -> $u32x {  $u32x::from_bits(x) | y }
        #[inline]
        fn vandnot_vm_vo32_vm(x: $m32x, y: $u32x) -> $u32x { y & !$u32x::from_bits(x) }

        #[inline]
        fn vandnot_vi2_vi2_vi2(x: $i32x, y: $i32x) -> $i32x { y & !x }

        #[inline]
        fn vand_vi2_vo_vi2(x: $m32x, y: $i32x) -> $i32x { $i32x::from_bits(x) & y }

        #[inline]
        fn vgt_vi2_vi2_vi2(x: $i32x, y: $i32x) -> $i32x { $i32x::from_bits(x.gt(y)) }


        impl SqrtAsDoubled for $f32x {
            #[inline]
            fn sqrt_as_doubled(self) -> Doubled<Self> {
                let t = self.sqrt();
                ((self + t.mul_as_doubled(t)) * t.recpre()).scale(Self::splat(0.5))
            }
        }

        impl Round for $f32x {
            type Int = $i32x;
            #[inline]
            fn truncate(self) -> Self {
                Self::from_cast(self.truncatei())
            }
            #[inline]
            fn truncatei(self) -> Self::Int {
                Self::Int::from_cast(self)
            }
            #[inline]
            fn rint(self) -> Self {
                rintf(self)
            }
            #[inline]
            fn rinti(self) -> Self::Int {
                Self::Int::from_cast(self.rint())
            }
        }

        #[inline]
        fn vmlanp_vf_vf_vf_vf(x: $f32x, y: $f32x, z: $f32x) -> $f32x { z - x * y }


        #[inline]
        fn vgather_vf_p_vi2(ptr: &[f32], vi: $i32x) -> $f32x {
          let mut ar = [0_f32; $f32x::lanes()];
          for i in 0..$f32x::lanes() {
              ar[i] = ptr[vi.extract(i) as usize];
          }
          $f32x::from_slice_aligned(&ar)
        }


        //----------???????
        //----------???????

        //-------------------

        //----------------------
        impl VectorizedSelect<f32> for $m32x {
            type Output = $f32x;
            fn select_splat(self, l: f32, r: f32) -> Self::Output {
                self.select(Self::Output::splat(l), Self::Output::splat(r))
            }
        }
        impl DoubledSelect<$f32x> for $m32x {
            fn select_doubled(self, l: Doubled<$f32x>, r: Doubled<$f32x>) -> Doubled<$f32x> {
                Doubled::new(self.select(l.0, r.0), self.select(l.1, r.1))
            }
        }

        #[inline]
        fn vsel_vf_vo_vo_f_f_f(o0: $m32x, o1: $m32x, d0: f32, d1: f32, d2: f32) -> $f32x {
          o0.select($f32x::splat(d0), o1.select_splat(d1, d2))
        }

        #[inline]
        fn vsel_vf_vo_vo_vo_f_f_f_f(o0: $m32x, o1: $m32x, o2: $m32x, d0: f32, d1: f32, d2: f32, d3: f32) -> $f32x {
          o0.select($f32x::splat(d0), o1.select($f32x::splat(d1), o2.select_splat(d2, d3)))
        }

        #[inline]
        fn vsel_vf2_vo_f_f_f_f(o: $m32x, x1: f32, y1: f32, x0: f32, y0: f32) -> Doubled<$f32x> {
            Doubled::new(o.select_splat(x1, x0), o.select_splat(y1, y0))
        }

        #[inline]
        fn vsel_vf2_vo_vo_d_d_d(o0: $m32x, o1: $m32x, d0: f64, d1: f64, d2: f64) -> Doubled<$f32x> {
            o0.select_doubled(Doubled::from(d0),
                o1.select_doubled(Doubled::from(d1), Doubled::from(d2)),
            )
        }

        #[inline]
        fn vsel_vf2_vo_vo_vo_d_d_d_d(
            o0: $m32x,
            o1: $m32x,
            o2: $m32x,
            d0: f64,
            d1: f64,
            d2: f64,
            d3: f64,
        ) -> Doubled<$f32x>  {
            o0.select_doubled(Doubled::from(d0),
                o1.select_doubled(Doubled::from(d1),
                    o2.select_doubled(Doubled::from(d2), Doubled::from(d3)),
                ),
            )
        }

        //---------------------

        #[inline]
        fn visnegzero_vo_vf(d: $f32x) -> $m32x {
            $i32x::from_bits(d).eq($i32x::from_bits($f32x::splat(-0.)))
        }

        #[inline]
        fn vsignbit_vm_vf(f: $f32x) -> $u32x {
            $u32x::from_bits(f) & $u32x::from_bits($f32x::splat(-0.))
        }
        #[inline]
        fn vmulsign_vf_vf_vf(x: $f32x, y: $f32x) -> $f32x {
            $f32x::from_bits($u32x::from_bits(x) ^ vsignbit_vm_vf(y))
        }
        #[inline]
        fn vcopysign_vf_vf_vf(x: $f32x, y: $f32x) -> $f32x {
            $f32x::from_bits(
                vandnot_vm_vm_vm($u32x::from_bits($f32x::splat(-0.)), $u32x::from_bits(x))
                    ^ ($u32x::from_bits($f32x::splat(-0.)) & $u32x::from_bits(y)),
            )
        }
        #[inline]
        fn vsign_vf_vf(f: $f32x) -> $f32x {
            $f32x::from_bits(
                $u32x::from_bits(ONE)
                    | ($u32x::from_bits($f32x::splat(-0.)) & $u32x::from_bits(f)),
            )
        }
        #[inline]
        fn vsignbit_vo_vf(d: $f32x) -> $m32x {
            ($u32x::from_bits(d) & $u32x::splat(0x80000000)).eq($u32x::splat(0x80000000))
        }
        #[inline]
        fn vsel_vi2_vf_vf_vi2_vi2(f0: $f32x, f1: $f32x, x: $i32x, y: $i32x) -> $i32x {
            f0.lt(f1).select(x, y)
        }
        #[inline]
        fn vsel_vi2_vf_vi2(d: $f32x, x: $i32x) -> $i32x {
            vand_vi2_vo_vi2(vsignbit_vo_vf(d), x)
        }
        #[inline]
        fn visint_vo_vf(y: $f32x) -> $m32x {
            y.truncate().eq(y)
        }

        /*#[cfg(
            all(not(feature = "enable_avx512f"),
            not(feature = "enable_avx512fnofma")
        ))]*/
        #[inline]
        fn vilogbk_vi2_vf(mut d: $f32x) -> $i32x {
            let o = d.lt($f32x::splat(5.421010862427522e-20));
            d = o.select($f32x::splat(1.8446744073709552e19) * d, d);
            let q = $i32x::from_cast($u32x::from_bits(d) >> 23) & $i32x::splat(0xff);
            q - o.select($i32x::splat(64 + 0x7f), $i32x::splat(0x7f))
        }
        /*#[cfg(
            all(not(feature = "enable_avx512f"),
            not(feature = "enable_avx512fnofma")
        ))]*/
        #[inline]
        fn vilogb2k_vi2_vf(d: $f32x) -> $i32x {
            let q = $u32x::from_bits(d);
            let mut q = $i32x::from_bits(q >> 23);
            q = q & $i32x::splat(0xff);
            q - $i32x::splat(0x7f)
        }

        //

        pub fn ilogbf(d: $f32x) -> $i32x {
            let mut e = vilogbk_vi2_vf(d.abs());
            e = d
                .eq(ZERO)
                .select($i32x::splat(SLEEF_FP_ILOGB0), e);
            e = d.is_nan().select($i32x::splat(SLEEF_FP_ILOGBNAN), e);
            d.is_infinite().select($i32x::splat(i32::MAX), e)
        }
        #[inline]
        fn vpow2i_vf_vi2(q: $i32x) -> $f32x {
            $f32x::from_bits($u32x::from_bits((q + $i32x::splat(0x7f)) << 23))
        }
        #[inline]
        fn vldexp_vf_vf_vi2(mut x: $f32x, mut q: $i32x) -> $f32x {
            let mut m = q >> 31;
            m = (((m + q) >> 6) - m) << 4;
            q = q - (m << 2);
            m = m + $i32x::splat(0x7f);
            m = vgt_vi2_vi2_vi2(m, $i32x::splat(0)) & m;
            let n = vgt_vi2_vi2_vi2(m, $i32x::splat(0xff));
            m = vandnot_vi2_vi2_vi2(n, m) | (n & $i32x::splat(0xff));
            let u = $f32x::from_bits($u32x::from_bits(m << 23));
            x *= u * u * u * u;
            let u = $f32x::from_bits($u32x::from_bits((q + $i32x::splat(0x7f)) << 23));
            x * u
        }
        #[inline]
        fn vldexp2_vf_vf_vi2(d: $f32x, e: $i32x) -> $f32x {
            d * vpow2i_vf_vi2(e >> 1) * vpow2i_vf_vi2(e - (e >> 1))
        }
        #[inline]
        fn vldexp3_vf_vf_vi2(d: $f32x, q: $i32x) -> $f32x {
            $f32x::from_bits($i32x::from_bits(d) + (q << 23))
        }

        pub fn ldexpf(x: $f32x, q: $i32x) -> $f32x {
            vldexp_vf_vf_vi2(x, q)
        }

        #[inline]
        fn rempisubf(x: $f32x) -> ($f32x, $i32x) {
            if cfg!(feature = "full_fp_rounding") {
                let y = (x * $f32x::splat(4.)).rint();
                let vi = (y - x.rint() * $f32x::splat(4.)).truncatei();
                (x - y * $f32x::splat(0.25), vi)
            } else {
                let mut fr = x - F1_10X * (x * (ONE / F1_10X)).truncate();
                let mut vi = x
                    .gt(ZERO)
                    .select($i32x::splat(4), $i32x::splat(3))
                    + (fr * $f32x::splat(8.)).truncatei();
                vi = (($i32x::splat(7) & vi) - $i32x::splat(3)) >> 1;
                fr -= $f32x::splat(0.25)
                    * (fr.mul_add($f32x::splat(4.), vmulsign_vf_vf_vf($f32x::splat(0.5), x)))
                        .truncate();
                fr = fr
                    .abs()
                    .gt($f32x::splat(0.25))
                    .select(fr - vmulsign_vf_vf_vf($f32x::splat(0.5), x), fr);
                fr = fr.abs().gt($f32x::splat(1e+10)).select(ZERO, fr);
                let o = x.abs().eq($f32x::splat(0.12499999254941940308));
                fr = o.select(x, fr);
                vi = o.select($i32x::splat(0), vi);
                (fr, vi)
            }
        }
        #[inline]
        fn rempif(mut a: $f32x) -> (Doubled<$f32x>, $i32x) {
            let mut ex = vilogb2k_vi2_vf(a);
            /*if cfg!(feature = "enable_avx512f") || cfg!(feature = "enable_avx512fnofma") {
                ex = vandnot_vi2_vi2_vi2(ex >> 31, ex);
                ex = ex & $i32x::splat(127);
            }*/
            ex -= $i32x::splat(25);
            let q = vand_vi2_vo_vi2(ex.gt($i32x::splat(90 - 25)), $i32x::splat(-64));
            a = vldexp3_vf_vf_vi2(a, q);
            ex = vandnot_vi2_vi2_vi2(ex >> 31, ex);
            ex = ex << 2;
            let mut x = a.mul_as_doubled(vgather_vf_p_vi2(&REMPITABSP, ex));
            let (did, mut q) = rempisubf(x.0);
            x.0 = did;
            x = x.normalize();
            let y = a.mul_as_doubled(vgather_vf_p_vi2(&REMPITABSP[1..], ex));
            x += y;
            let (did, dii) = rempisubf(x.0);
            q = q + dii;
            x.0 = did;
            x = x.normalize();
            let mut y = Doubled::new(
                vgather_vf_p_vi2(&REMPITABSP[2..], ex),
                vgather_vf_p_vi2(&REMPITABSP[3..], ex),
            );
            y *= a;
            x += y;
            x = x.normalize();
            x *= Doubled::from((3.1415927410125732422 * 2., -8.7422776573475857731e-08 * 2.));
            x = a.abs().lt($f32x::splat(0.7)).select_doubled(Doubled::new(a, ZERO),
                x,
            );
            (x, q)
        }

        pub fn modff(x: $f32x) -> ($f32x, $f32x) {
            let fr = x - $f32x::from_cast(x.truncatei());
            let fr = x.abs().gt(F1_23X).select(ZERO, fr);
            (vcopysign_vf_vf_vf(fr, x), vcopysign_vf_vf_vf(x - fr, x))
        }

        #[inline]
        fn atan2kf(y: $f32x, x: $f32x) -> $f32x {
            let q = vsel_vi2_vf_vi2(x, $i32x::splat(-2));
            let x = x.abs();

            let q = vsel_vi2_vf_vf_vi2_vi2(x, y, q + $i32x::splat(1), q);
            let p = x.lt(y);
            let s = p.select(-x, y);
            let mut t = x.max(y);

            let s = s / t;
            t = s * s;

            let u = $f32x::splat(0.00282363896258175373077393)
                .mul_add(t, $f32x::splat(-0.0159569028764963150024414))
                .mul_add(t, $f32x::splat(0.0425049886107444763183594))
                .mul_add(t, $f32x::splat(-0.0748900920152664184570312))
                .mul_add(t, $f32x::splat(0.106347933411598205566406))
                .mul_add(t, $f32x::splat(-0.142027363181114196777344))
                .mul_add(t, $f32x::splat(0.199926957488059997558594))
                .mul_add(t, $f32x::splat(-0.333331018686294555664062));

            let t = s.mul_add(t * u, s);
            $f32x::from_cast(q).mul_add($f32x::FRAC_PI_2, t)
        }
        #[inline]
        fn visinf2_vf_vf_vf(d: $f32x, m: $f32x) -> $f32x {
            $f32x::from_bits(vand_vm_vo32_vm(
                d.is_infinite(),
                vsignbit_vm_vf(d) | $u32x::from_bits(m),
            ))
        }

        #[inline]
        fn logkf(mut d: $f32x) -> Doubled<$f32x> {
            let m: $f32x;

            let ef = /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma")*/
            {
                let o = d.lt($f32x::splat(f32::MIN));
                d = o.select(d * (F1_32X * F1_32X), d);
                let mut e = vilogb2k_vi2_vf(d * $f32x::splat(1. / 0.75));
                m = vldexp3_vf_vf_vi2(d, -e);
                e = o.select(e - $i32x::splat(64), e);
                $f32x::from_cast(e)
            }/* else {
                let mut e = vgetexp_vf_vf(d * $f32x::splat(1. / 0.75));
                e = e.eq($f32x::INFINITY).select($f32x::splat(128.), e);
                m = vgetmant_vf_vf(d);
                e
            }*/;

            let x = $f32x::splat(-1.).add_as_doubled(m) / ONE.add_as_doubled(m);
            let x2 = x.square();

            let t = $f32x::splat(0.240320354700088500976562)
                .mul_add(x2.0, $f32x::splat(0.285112679004669189453125))
                .mul_add(x2.0, $f32x::splat(0.400007992982864379882812));
            let c = Doubled::from((0.66666662693023681640625, 3.69183861259614332084311e-09));

            let mut s = Doubled::from((0.69314718246459960938, -1.904654323148236017e-09)) * ef;

            s = s.add_checked(x.scale($f32x::splat(2.)));
            s.add_checked(x2 * x * (x2 * t + c))
        }

        #[inline]
        fn expkf(d: Doubled<$f32x>) -> $f32x {
            let u = (d.0 + d.1) * $f32x::splat(R_LN2_F);
            let q = u.rinti();

            let mut s = d + $f32x::from_cast(q) * $f32x::splat(-L2U_F);
            s += $f32x::from_cast(q) * $f32x::splat(-L2L_F);

            s = s.normalize();

            let mut u = $f32x::splat(0.00136324646882712841033936)
                .mul_add(s.0, $f32x::splat(0.00836596917361021041870117))
                .mul_add(s.0, $f32x::splat(0.0416710823774337768554688))
                .mul_add(s.0, $f32x::splat(0.166665524244308471679688))
                .mul_add(s.0, $f32x::splat(0.499999850988388061523438));

            let mut t = s.add_checked(s.square() * u);

            t = ONE.add_checked(t);
            u = t.0 + t.1;
            u = vldexp_vf_vf_vi2(u, q);

            $f32x::from_bits(vandnot_vm_vo32_vm(
                d.0.lt($f32x::splat(-104.)),
                $u32x::from_bits(u),
            ))
        }

        #[inline]
        fn expk2f(d: Doubled<$f32x>) -> Doubled<$f32x> {
            let u = (d.0 + d.1) * $f32x::splat(R_LN2_F);
            let q = u.rinti();

            let mut s = d + $f32x::from_cast(q) * $f32x::splat(-L2U_F);
            s += $f32x::from_cast(q) * $f32x::splat(-L2L_F);

            let u = $f32x::splat(0.1980960224e-3)
                .mul_add(s.0, $f32x::splat(0.1394256484e-2))
                .mul_add(s.0, $f32x::splat(0.8333456703e-2))
                .mul_add(s.0, $f32x::splat(0.4166637361e-1));

            let mut t = s * u + $f32x::splat(0.166666659414234244790680580464e+0);
            t = s * t + $f32x::splat(0.5);
            t = s + s.square() * t;

            t = ONE.add_checked(t);

            t.0 = vldexp2_vf_vf_vi2(t.0, q);
            t.1 = vldexp2_vf_vf_vi2(t.1, q);

            t.0 = $f32x::from_bits(vandnot_vm_vo32_vm(
                d.0.lt($f32x::splat(-104.)),
                $u32x::from_bits(t.0),
            ));
            t.1 = $f32x::from_bits(vandnot_vm_vo32_vm(
                d.0.lt($f32x::splat(-104.)),
                $u32x::from_bits(t.1),
            ));

            t
        }

        #[inline]
        fn logk2f(d: Doubled<$f32x>) -> Doubled<$f32x> {
            let e = /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma") {*/
                vilogbk_vi2_vf(d.0 * $f32x::splat(1. / 0.75))
            /*} else {
                vgetexp_vf_vf(d.0 * $f32x::splat(1. / 0.75)).rinti()
            }*/;
            let m = d.scale(vpow2i_vf_vi2(-e));

            let x = (m + $f32x::splat(-1.)) / (m + ONE);
            let x2 = x.square();

            let t = $f32x::splat(0.2392828464508056640625)
                .mul_add(x2.0, $f32x::splat(0.28518211841583251953125))
                .mul_add(x2.0, $f32x::splat(0.400005877017974853515625))
                .mul_add(x2.0, $f32x::splat(0.666666686534881591796875));

            let mut s = Doubled::new(
                $f32x::splat(0.69314718246459960938),
                $f32x::splat(-1.904654323148236017e-09),
            ) * $f32x::from_cast(e);
            s = s.add_checked(x.scale($f32x::splat(2.)));
            s.add_checked(x2 * x * t)
        }

        pub fn exp2f(d: $f32x) -> $f32x {
            let mut u = d.rint();
            let q = u.rinti();

            let s = d - u;

            u = $f32x::splat(0.1535920892e-3)
                .mul_add(s, $f32x::splat(0.1339262701e-2))
                .mul_add(s, $f32x::splat(0.9618384764e-2))
                .mul_add(s, $f32x::splat(0.5550347269e-1))
                .mul_add(s, $f32x::splat(0.2402264476e+0))
                .mul_add(s, $f32x::splat(0.6931471825e+0));

            if !cfg!(target_feature = "fma") {
                u = u.mul_adde(s, ONE);
            } else {
                u = ONE.add_checked(u.mul_as_doubled(s)).normalize().0;
            }

            u = vldexp2_vf_vf_vi2(u, q);

            u = d
                .ge($f32x::splat(128.))
                .select($f32x::INFINITY, u);
            $f32x::from_bits(vandnot_vm_vo32_vm(
                d.lt($f32x::splat(-150.)),
                $u32x::from_bits(u),
            ))
        }

        pub fn log1pf(d: $f32x) -> $f32x {
            let m: $f32x;

            let dp1 = d + ONE;

            let mut s =
                /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma")*/ {
                    let o = dp1.lt($f32x::splat(f32::MIN));
                    let dp1 = o.select(dp1 * (F1_32X * F1_32X), dp1);
                    let e = vilogb2k_vi2_vf(dp1 * $f32x::splat(1. / 0.75));
                    let t = vldexp3_vf_vf_vi2(ONE, -e);
                    m = d.mul_add(t, t - ONE);
                    let e = o.select(e - $i32x::splat(64), e);
                    Doubled::from((0.69314718246459960938, -1.904654323148236017e-09)) * $f32x::from_cast(e)
                }/* else {
                    let e = vgetexp_vf_vf(dp1, $f32x::splat(1. / 0.75));
                    let e = e.eq($f32x::INFINITY).select($f32x::splat(128.), e);
                    let t = vldexp3_vf_vf_vi2(ONE, -e.rinti());
                    m = d.mul_add(t, t - ONE);
                    Doubled::from((0.69314718246459960938, -1.904654323148236017e-09)) * e
                }*/;

            let x = Doubled::new(m, ZERO) / $f32x::splat(2.).add_checked_as_doubled(m);
            let x2 = x.0 * x.0;

            let t = $f32x::splat(0.3027294874e+0)
                .mul_add(x2, $f32x::splat(0.3996108174e+0))
                .mul_add(x2, $f32x::splat(0.6666694880e+0));

            s = s.add_checked(x.scale($f32x::splat(2.)));
            s = s.add_checked(x2 * x.0 * t);

            let mut r = s.0 + s.1;

            r = d
                .gt($f32x::splat(1e+38))
                .select($f32x::INFINITY, r);
            r = $f32x::from_bits(vor_vm_vo32_vm($f32x::splat(-1.).gt(d), $u32x::from_bits(r)));
            r = d
                .eq($f32x::splat(-1.))
                .select($f32x::NEG_INFINITY, r);
            visnegzero_vo_vf(d).select($f32x::splat(-0.), r)
        }

        //

        pub fn fabsf(x: $f32x) -> $f32x {
            x.abs()
        }

        pub fn copysignf(x: $f32x, y: $f32x) -> $f32x {
            vcopysign_vf_vf_vf(x, y)
        }

        pub fn fmaxf(x: $f32x, y: $f32x) -> $f32x {
            if cfg!(target_arch = "x86_64") || cfg!(target_arch = "x86")
            /*    && !cfg!(feature = "enable_vecext")
                        && !cfg!(feature = "enable_purec")*/
            {
                y.is_nan().select(x, x.max(y))
            } else {
                y.is_nan().select(x, x.gt(y).select(x, y))
            }
        }

        pub fn fminf(x: $f32x, y: $f32x) -> $f32x {
            if cfg!(target_arch = "x86_64") || cfg!(target_arch = "x86")
            /*    && !cfg!(feature = "enable_vecext")
                        && !cfg!(feature = "enable_purec")*/
            {
                y.is_nan().select(x, x.min(y))
            } else {
                y.is_nan().select(x, y.gt(x).select(x, y))
            }
        }

        pub fn fdimf(x: $f32x, y: $f32x) -> $f32x {
            let ret = x - y;
            (ret.lt(ZERO) | x.eq(y)).select(ZERO, ret)
        }

        pub fn truncf(x: $f32x) -> $f32x {
            let fr = x - $f32x::from_cast(x.truncatei());
            (x.is_infinite() | x.abs().ge(F1_23X))
                .select(x, vcopysign_vf_vf_vf(x - fr, x))
        }

        pub fn floorf(x: $f32x) -> $f32x {
            let fr = x - $f32x::from_cast(x.truncatei());
            let fr = fr.lt(ZERO).select(fr + ONE, fr);
            (x.is_infinite() | x.abs().ge(F1_23X))
                .select(x, vcopysign_vf_vf_vf(x - fr, x))
        }

        pub fn ceilf(x: $f32x) -> $f32x {
            let fr = x - $f32x::from_cast(x.truncatei());
            let fr = fr.le(ZERO).select(fr, fr - ONE);
            (x.is_infinite() | x.abs().ge(F1_23X))
                .select(x, vcopysign_vf_vf_vf(x - fr, x))
        }

        pub fn roundf(d: $f32x) -> $f32x {
            let mut x = d + $f32x::splat(0.5);
            let fr = x - $f32x::from_cast(x.truncatei());
            x = (x.le(ZERO) & fr.eq(ZERO)).select(x - ONE, x);
            let fr = fr.lt(ZERO).select(fr + ONE, fr);
            x = d
                .eq($f32x::splat(0.4999999701976776123))
                .select(ZERO, x);
            (d.is_infinite() | d.abs().ge(F1_23X))
                .select(d, vcopysign_vf_vf_vf(x - fr, d))
        }

        pub fn rintf(d: $f32x) -> $f32x {
            let mut x = d + $f32x::splat(0.5);
            let isodd = ($i32x::splat(1) & x.truncatei()).eq($i32x::splat(1));
            let mut fr = x - $f32x::from_cast(x.truncatei());
            fr = (fr.lt(ZERO) | (fr.eq(ZERO) & isodd))
                .select(fr + ONE, fr);
            x = d
                .eq($f32x::splat(0.50000005960464477539))
                .select(ZERO, x);
            (d.is_infinite() | d.abs().ge(F1_23X))
                .select(d, vcopysign_vf_vf_vf(x - fr, d))
        }

        pub fn fmaf(mut x: $f32x, mut y: $f32x, mut z: $f32x) -> $f32x {
            let h2 = x * y + z;
            let mut q = ONE;
            let o = h2.abs().lt($f32x::splat(1e-38));
            const C0: $f32x = F1_25X;
            let c1: $f32x = C0 * C0;
            let c2: $f32x = c1 * c1;
            {
                x = o.select(x * c1, x);
                y = o.select(y * c1, y);
                z = o.select(z * c2, z);
                q = o.select(ONE / c2, q);
            }
            let o = h2.abs().gt($f32x::splat(1e+38));
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
            let h2 = o.select(z, h2);

            o = h2.is_infinite() | h2.is_nan();

            o.select(h2, ret * q)
        }

        pub fn sqrtf(d: $f32x) -> $f32x {
        //   if cfg!(feature = "accurate_sqrt") {
                d.sqrt()
        /*    } else {
                // fall back to approximation if ACCURATE_SQRT is undefined
                u05::sqrtf(d)
            }*/
        }

        pub fn nextafterf(x: $f32x, y: $f32x) -> $f32x {
            let x = x
                .eq(ZERO)
                .select(vmulsign_vf_vf_vf(ZERO, y), x);
            let mut xi2 = $i32x::from_bits(x);
            let c = vsignbit_vo_vf(x) ^ y.ge(x);

            xi2 = c.select($i32x::splat(0) - (xi2 ^ $i32x::splat(1 << 31)), xi2);

            xi2 = x.ne(y).select(xi2 - $i32x::splat(1), xi2);

            xi2 = c.select($i32x::splat(0) - (xi2 ^ $i32x::splat(1 << 31)), xi2);

            let mut ret = $f32x::from_bits(xi2);

            ret = (ret.eq(ZERO) & x.ne(ZERO))
                .select(vmulsign_vf_vf_vf(ZERO, x), ret);

            ret = (x.eq(ZERO) & y.eq(ZERO)).select(y, ret);

            (x.is_nan() | y.is_nan()).select($f32x::NAN, ret)
        }

        pub fn frfrexpf(x: $f32x) -> $f32x {
            let x = x
                .abs()
                .lt($f32x::splat(f32::MIN))
                .select(x * F1_32X, x);

            let mut xm = $u32x::from_bits(x);
            xm &= $u32x::splat(!0x7f800000u32);
            xm |= $u32x::splat(0x3f000000u32);

            let ret = $f32x::from_bits(xm);

            let ret =
                x.is_infinite().select(vmulsign_vf_vf_vf($f32x::INFINITY, x), ret);
            x.eq(ZERO).select(x, ret)
        }

        pub fn expfrexpf(_x: $f32x) -> $i32x {
            /*
                                  x = x.abs().lt($f32x::splat(f32::MIN)).select(x * F1_63X, x);

                                  let mut ret = $i32x::from_cast($ix::from_bits(x);
                                  ret = (vsrl_vi_vi_i(ret, 20) & $ix::splat(0x7ff)) - $ix::splat(0x3fe);

                                  (x.eq(ZERO) | x.is_nan() | x.is_infinite()).select($ix::splat(0), ret)
                                  */
            $i32x::splat(0)
        }
        #[inline]
        fn vtoward0f(x: $f32x) -> $f32x {
            let t = $f32x::from_bits($i32x::from_bits(x) - $i32x::splat(1));
            x.eq(ZERO).select(ZERO, t)
        }
        #[inline]
        fn vptruncf(x: $f32x) -> $f32x {
            if cfg!(feature = "full_fp_rounding") {
                x.truncate()
            } else {
                let fr = x - $f32x::from_cast(x.truncatei());
                x.abs().ge(F1_23X).select(x, x - fr)
            }
        }

        pub fn fmodf(x: $f32x, y: $f32x) -> $f32x {
            let nu = x.abs();
            let de = y.abs();
            let s = ONE;
            let o = de.lt($f32x::splat(f32::MIN));
            let nu = o.select(nu * F1_25X, nu);
            let de = o.select(de * F1_25X, de);
            let s = o.select(s * (ONE / F1_25X), s);
            let rde = vtoward0f(de.recpre());
            #[cfg(any(feature = "enable_neon32", feature = "enable_neon32vfpv4"))]
            {
                let rde = vtoward0f(rde);
            }
            let mut r = Doubled::new(nu, ZERO);

            for _ in 0..8 {
                // ceil(log2(FLT_MAX) / 22)+1
                let q =
                    ((de + de).gt(r.0) & r.0.ge(de)).select(ONE, vtoward0f(r.0) * rde);
                r = (r + vptruncf(q).mul_as_doubled(-de)).normalize();
                if r.0.lt(de).all() {
                    break;
                }
            }

            let mut ret = (r.0 + r.1) * s;
            ret = (r.0 + r.1).eq(de).select(ZERO, ret);

            ret = vmulsign_vf_vf_vf(ret, x);

            ret = nu.lt(de).select(x, ret);
            de.eq(ZERO)
                .select($f32x::NAN, ret)
        }

        //
        #[inline]
        fn sinpifk(d: $f32x) -> Doubled<$f32x> {
            let u = d * $f32x::splat(4.);
            let q = u.truncatei();
            let q = (q + ($i32x::from_bits($u32x::from_bits(q) >> 31) ^ $i32x::splat(1)))
                & $i32x::splat(!1);
            let o = (q & $i32x::splat(2)).eq($i32x::splat(2));

            let s = u - $f32x::from_cast(q);
            let t = s;
            let s = s * s;
            let s2 = t.mul_as_doubled(t);

            //

            let u = o.select_splat(-0.2430611801e-7, 0.3093842054e-6)
                .mul_add(s, o.select_splat(0.3590577080e-5, -0.3657307388e-4))
                .mul_add(s, o.select_splat(-0.3259917721e-3, 0.2490393585e-2));
            let mut x = u * s + vsel_vf2_vo_f_f_f_f(
                o,
                0.015854343771934509277,
                4.4940051354032242811e-10,
                -0.080745510756969451904,
                -1.3373665339076936258e-09,
            );
            x = s2 * x + vsel_vf2_vo_f_f_f_f(
                o,
                -0.30842512845993041992,
                -9.0728339030733922277e-09,
                0.78539818525314331055,
                -2.1857338617566484855e-08,
            );

            x *= o.select_doubled(s2, Doubled::new(t, ZERO));
            x = o.select_doubled(x + ONE, x);

            let o = (q & $i32x::splat(4)).eq($i32x::splat(4));
            x.0 = $f32x::from_bits(
                vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(x.0),
            );
            x.1 = $f32x::from_bits(
                vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(x.1),
            );

            x
        }

        #[inline]
        fn cospifk(d: $f32x) -> Doubled<$f32x> {
            let u = d * $f32x::splat(4.);
            let q = u.truncatei();
            let q = (q + ($i32x::from_bits($u32x::from_bits(q) >> 31) ^ $i32x::splat(1)))
                & $i32x::splat(!1);
            let o = (q & $i32x::splat(2)).eq($i32x::splat(0));

            let s = u - $f32x::from_cast(q);
            let t = s;
            let s = s * s;
            let s2 = t.mul_as_doubled(t);

            //

            let u = o.select_splat(-0.2430611801e-7, 0.3093842054e-6)
                .mul_add(s, o.select_splat(0.3590577080e-5, -0.3657307388e-4))
                .mul_add(s, o.select_splat(-0.3259917721e-3, 0.2490393585e-2));
            let mut x = u * s + vsel_vf2_vo_f_f_f_f(
                o,
                0.015854343771934509277,
                4.4940051354032242811e-10,
                -0.080745510756969451904,
                -1.3373665339076936258e-09,
            );
            x = s2 * x + vsel_vf2_vo_f_f_f_f(
                o,
                -0.30842512845993041992,
                -9.0728339030733922277e-09,
                0.78539818525314331055,
                -2.1857338617566484855e-08,
            );

            x *= o.select_doubled(s2, Doubled::new(t, ZERO));
            x = o.select_doubled(x + ONE, x);

            let o = ((q + $i32x::splat(2)) & $i32x::splat(4)).eq($i32x::splat(4));
            x.0 = $f32x::from_bits(
                vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(x.0),
            );
            x.1 = $f32x::from_bits(
                vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(x.1),
            );

            x
        }

        /* TODO AArch64: potential optimization by using `vfmad_lane_f64` */
        fn gammafk(a: $f32x) -> (Doubled<$f32x>, Doubled<$f32x>) {
            let mut clln = Doubled::from((1., 0.));
            let mut clld = Doubled::from((1., 0.));

            let otiny = a.abs().lt($f32x::splat(1e-30));
            let oref = a.lt($f32x::splat(0.5));

            let x = otiny.select_doubled(Doubled::from((0., 0.)),
                oref.select_doubled(ONE.add_as_doubled(-a),
                    Doubled::new(a, ZERO),
                ),
            );

            let o0 = $f32x::splat(0.5).le(x.0) & x.0.le($f32x::splat(1.2));
            let o2 = $f32x::splat(2.3).le(x.0);

            let mut y = ((x + ONE) * x).normalize();
            y = ((x + $f32x::splat(2.)) * y).normalize();

            let o = o2 & x.0.le($f32x::splat(7.));
            clln = o.select_doubled(y, clln);

            let mut x = o.select_doubled(x + $f32x::splat(3.), x);
            let t = o2.select(x.0.recpre(), (x + o0.select_splat(-1., -2.)).normalize().0);

            let u = vsel_vf_vo_vo_f_f_f(
                o2,
                o0,
                0.000839498720672087279971000786,
                0.9435157776e+0,
                0.1102489550e-3,
            ).mul_add(
                t,
                vsel_vf_vo_vo_f_f_f(
                    o2,
                    o0,
                    -5.17179090826059219329394422e-05,
                    0.8670063615e+0,
                    0.8160019934e-4,
                ),
            ).mul_add(
                t,
                vsel_vf_vo_vo_f_f_f(
                    o2,
                    o0,
                    -0.000592166437353693882857342347,
                    0.4826702476e+0,
                    0.1528468856e-3,
                ),
            ).mul_add(
                t,
                vsel_vf_vo_vo_f_f_f(
                    o2,
                    o0,
                    6.97281375836585777403743539e-05,
                    -0.8855129778e-1,
                    -0.2355068718e-3,
                ),
            ).mul_add(
                t,
                vsel_vf_vo_vo_f_f_f(
                    o2,
                    o0,
                    0.000784039221720066627493314301,
                    0.1013825238e+0,
                    0.4962242092e-3,
                ),
            ).mul_add(
                t,
                vsel_vf_vo_vo_f_f_f(
                    o2,
                    o0,
                    -0.000229472093621399176949318732,
                    -0.1493408978e+0,
                    -0.1193488017e-2,
                ),
            ).mul_add(
                t,
                vsel_vf_vo_vo_f_f_f(
                    o2,
                    o0,
                    -0.002681327160493827160473958490,
                    0.1697509140e+0,
                    0.2891599433e-2,
                ),
            ).mul_add(
                t,
                vsel_vf_vo_vo_f_f_f(
                    o2,
                    o0,
                    0.003472222222222222222175164840,
                    -0.2072454542e+0,
                    -0.7385451812e-2,
                ),
            ).mul_add(
                t,
                vsel_vf_vo_vo_f_f_f(
                    o2,
                    o0,
                    0.083333333333333333335592087900,
                    0.2705872357e+0,
                    0.2058077045e-1,
                ),
            );

            y = (x + $f32x::splat(-0.5)) * logk2f(x);
            y += -x;
            y += Doubled::from(0.91893853320467278056_f64); // 0.5*log(2*M_PI)

            let mut z = u.mul_as_doubled(t) + o0.select_splat(-0.400686534596170958447352690395e+0,
                -0.673523028297382446749257758235e-1,
            );
            z = z * t + o0.select_splat(0.822466960142643054450325495997e+0,
                0.322467033928981157743538726901e+0,
            );
            z = z * t + o0.select_splat(-0.577215665946766039837398973297e+0,
                0.422784335087484338986941629852e+0,
            );
            z = z * t;

            let mut clc = o2.select_doubled(y, z);

            clld = o2.select_doubled(u.mul_as_doubled(t) + ONE, clld);

            y = clln;

            clc = otiny.select_doubled(Doubled::from(41.58883083359671856503_f64), // log(2^60)
                oref.select_doubled(Doubled::<$f32x>::from(1.1447298858494001639_f64) + (-clc), clc),
            ); // log(M_PI)
            clln = otiny.select_doubled(Doubled::from((1., 0.)),
                oref.select_doubled(clln, clld),
            );

            if !(!oref).all() {
                let t = a
                    - F1_12X * $f32x::from_cast((a * (ONE / F1_12X)).truncatei());
                x = clld * sinpifk(t);
            }

            clld = otiny.select_doubled(Doubled::new(a * (F1_30X * F1_30X), ZERO),
                oref.select_doubled(x, y),
            );

            (clc, clln / clld)
        }

        #[inline]
        fn expm1fk(d: $f32x) -> $f32x {
            let q = (d * $f32x::splat(R_LN2_F)).rinti();
            let s = $f32x::from_cast(q).mul_add($f32x::splat(-L2U_F), d);
            let s = $f32x::from_cast(q).mul_add($f32x::splat(-L2L_F), s);

            let u = $f32x::splat(0.000198527617612853646278381)
                .mul_add(s, $f32x::splat(0.00139304355252534151077271))
                .mul_add(s, $f32x::splat(0.00833336077630519866943359))
                .mul_add(s, $f32x::splat(0.0416664853692054748535156))
                .mul_add(s, $f32x::splat(0.166666671633720397949219))
                .mul_add(s, $f32x::splat(0.5));

            let u = (s * s).mul_add(u, s);

            q.eq($i32x::splat(0)).select(
                u,
                vldexp2_vf_vf_vi2(u + ONE, q) - ONE,
            )
        }

        pub mod u05 {
            //! Functions with 0.5 ULP error bound
            use super::*;

            pub fn sincospif(d: $f32x) -> ($f32x, $f32x) {
                let u = d * $f32x::splat(4.);
                let q = u.truncatei();
                let q = (q + ($i32x::from_bits($u32x::from_bits(q) >> 31) ^ $i32x::splat(1)))
                    & $i32x::splat(!1);
                let s = u - $f32x::from_cast(q);

                let t = s;
                let s = s * s;
                let s2 = t.mul_as_doubled(t);

                //

                let u = $f32x::splat(0.3093842054e-6)
                    .mul_add(s, $f32x::splat(-0.3657307388e-4))
                    .mul_add(s, $f32x::splat(0.2490393585e-2));
                let mut x = u * s + Doubled::from((-0.080745510756969451904, -1.3373665339076936258e-09));
                x = s2 * x + Doubled::from((0.78539818525314331055, -2.1857338617566484855e-08));

                x *= t;
                let rx = x.0 + x.1;

                let rx = visnegzero_vo_vf(d).select($f32x::splat(-0.), rx);

                //

                let u = $f32x::splat(-0.2430611801e-7)
                    .mul_add(s, $f32x::splat(0.3590577080e-5))
                    .mul_add(s, $f32x::splat(-0.3259917721e-3));
                x = u * s + Doubled::from((0.015854343771934509277, 4.4940051354032242811e-10));
                x = s2 * x + Doubled::from((-0.30842512845993041992, -9.0728339030733922277e-09));

                x = x * s2 + ONE;
                let ry = x.0 + x.1;

                //

                let o = (q & $i32x::splat(2)).eq($i32x::splat(0));
                let mut rsin = o.select(rx, ry);
                let mut rcos = o.select(ry, rx);

                let o = (q & $i32x::splat(4)).eq($i32x::splat(4));
                rsin = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(rsin),
                );

                let o = ((q + $i32x::splat(2)) & $i32x::splat(4)).eq($i32x::splat(4));
                rcos = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(rcos),
                );

                let o = d.abs().gt($f32x::splat(1e+7));
                rsin = $f32x::from_bits(vandnot_vm_vo32_vm(o, $u32x::from_bits(rsin)));
                rcos = $f32x::from_bits(vandnot_vm_vo32_vm(o, $u32x::from_bits(rcos)));

                let o = d.is_infinite();
                rsin = $f32x::from_bits(vor_vm_vo32_vm(o, $u32x::from_bits(rsin)));
                rcos = $f32x::from_bits(vor_vm_vo32_vm(o, $u32x::from_bits(rcos)));

                (rsin, rcos)
            }

            pub fn sqrtf(d: $f32x) -> $f32x {
                let d = d.lt(ZERO).select($f32x::NAN, d);

                let o = d.lt($f32x::splat(5.2939559203393770e-23));
                let d = o.select(d * $f32x::splat(1.8889465931478580e+22), d);
                let q = o.select(
                    $f32x::splat(7.2759576141834260e-12 * 0.5),
                    $f32x::splat(0.5),
                );

                let o = d.gt($f32x::splat(1.8446744073709552e+19));
                let d = o.select(d * $f32x::splat(5.4210108624275220e-20), d);
                let q = o.select($f32x::splat(4294967296.0 * 0.5), q);

                let mut x = $f32x::from_bits(
                    $i32x::splat(0x5f375a86)
                        - $i32x::from_bits(
                            $u32x::from_bits(d + $f32x::splat(1e-45)) >> 1,
                        ),
                );

                x *= $f32x::splat(1.5) - $f32x::splat(0.5) * d * x * x;
                x *= $f32x::splat(1.5) - $f32x::splat(0.5) * d * x * x;
                x *= $f32x::splat(1.5) - $f32x::splat(0.5) * d * x * x;
                x *= d;

                let d2 = (d + x.mul_as_doubled(x)) * x.recpre_as_doubled();

                x = (d2.0 + d2.1) * q;

                x = d.eq($f32x::INFINITY).select($f32x::INFINITY, x);
                d.eq(ZERO).select(d, x)
            }

            pub fn hypotf(x: $f32x, y: $f32x) -> $f32x {
                let x = x.abs();
                let y = y.abs();
                let min = x.min(y);
                let n = min;
                let max = x.max(y);
                let d = max;

                let o = max.lt($f32x::splat(f32::MIN));
                let n = o.select(n * F1_24X, n);
                let d = o.select(d * F1_24X, d);

                let t = Doubled::new(n, ZERO) / Doubled::new(d, ZERO);
                let t = (t.square() + ONE).sqrt() * max;
                let mut ret = t.0 + t.1;
                ret = ret.is_nan().select($f32x::INFINITY, ret);
                ret = min.eq(ZERO).select(max, ret);
                ret = (x.is_nan() | y.is_nan()).select($f32x::NAN, ret);
                (x.eq($f32x::INFINITY) | y.eq($f32x::INFINITY))
                    .select($f32x::INFINITY, ret)
            }

            pub fn xsinpif(d: $f32x) -> $f32x {
                let x = sinpifk(d);
                let mut r = x.0 + x.1;

                r = visnegzero_vo_vf(d).select($f32x::splat(-0.), r);
                r = $f32x::from_bits(vandnot_vm_vo32_vm(
                    d.abs().gt($f32x::splat(TRIGRANGEMAX4_F)),
                    $u32x::from_bits(r),
                ));
                $f32x::from_bits(vor_vm_vo32_vm(d.is_infinite(), $u32x::from_bits(r)))
            }

            pub fn cospif(d: $f32x) -> $f32x {
                let x = cospifk(d);
                let r = x.0 + x.1;

                let r = d
                    .abs()
                    .gt($f32x::splat(TRIGRANGEMAX4_F))
                    .select(ONE, r);
                $f32x::from_bits(vor_vm_vo32_vm(d.is_infinite(), $u32x::from_bits(r)))
            }

        }

        pub mod u10 {
            //! Functions with 1.0 ULP error bound
            use super::*;

            pub fn sinf(d: $f32x) -> $f32x {
                let mut q: $i32x;
                let mut s: Doubled<$f32x>;

                if d.abs().lt($f32x::splat(TRIGRANGEMAX2_F)).all() {
                    let u = (d * $f32x::FRAC_1_PI).rint();
                    q = u.rinti();
                    let v = u.mul_add($f32x::splat(-PI_A2_F), d);
                    s = v.add_as_doubled(u * $f32x::splat(-PI_B2_F));
                    s = s.add_checked(u * $f32x::splat(-PI_C2_F));
                } else {
                    let (mut dfidf, dfii) = rempif(d);
                    q = dfii & $i32x::splat(3);
                    q = q + q + dfidf
                        .0
                        .gt(ZERO)
                        .select($i32x::splat(2), $i32x::splat(1));
                    q = q >> 2;
                    let o = (dfii & $i32x::splat(1)).eq($i32x::splat(1));
                    let mut x = Doubled::new(
                        vmulsign_vf_vf_vf($f32x::splat(3.1415927410125732422 * -0.5), dfidf.0),
                        vmulsign_vf_vf_vf($f32x::splat(-8.7422776573475857731e-08 * -0.5), dfidf.0),
                    );
                    x = dfidf + x;
                    dfidf = o.select_doubled(x, dfidf);
                    s = dfidf.normalize();

                    s.0 = $f32x::from_bits(vor_vm_vo32_vm(
                        d.is_infinite() | d.is_nan(),
                        $u32x::from_bits(s.0),
                    ));
                }

                let t = s;
                let s = s.square();

                let mut u = $f32x::splat(2.6083159809786593541503e-06)
                    .mul_add(s.0, $f32x::splat(-0.0001981069071916863322258))
                    .mul_add(s.0, $f32x::splat(0.00833307858556509017944336));

                let x = ONE.add_checked(
                    $f32x::splat(-0.166666597127914428710938).add_checked_as_doubled(u * s.0) * s,
                );

                u = t.mul_as_f(x);

                u = $f32x::from_bits(
                    vand_vm_vo32_vm(
                        (q & $i32x::splat(1)).eq($i32x::splat(1)),
                        $u32x::from_bits($f32x::splat(-0.)),
                    ) ^ $u32x::from_bits(u),
                );

                visnegzero_vo_vf(d).select(d, u)
            }

            pub fn cosf(d: $f32x) -> $f32x {
                let mut q: $i32x;
                let mut s: Doubled<$f32x>;

                if d.abs().lt($f32x::splat(TRIGRANGEMAX2_F)).all() {
                    let dq = (d.mul_add($f32x::FRAC_1_PI, $f32x::splat(-0.5)))
                        .rint()
                        .mul_add($f32x::splat(2.), ONE);
                    q = dq.rinti();
                    s = d.add_as_doubled(dq * $f32x::splat(-PI_A2_F * 0.5));
                    s += dq * $f32x::splat(-PI_B2_F * 0.5);
                    s += dq * $f32x::splat(-PI_C2_F * 0.5);
                } else {
                    let (mut dfidf, dfii) = rempif(d);
                    q = dfii & $i32x::splat(3);
                    q = q + q + dfidf
                        .0
                        .gt(ZERO)
                        .select($i32x::splat(8), $i32x::splat(7));
                    q = q >> 1;
                    let o = (dfii & $i32x::splat(1)).eq($i32x::splat(0));
                    let y = dfidf
                        .0
                        .gt(ZERO)
                        .select(ZERO, $f32x::splat(-1.));
                    let mut x = Doubled::new(
                        vmulsign_vf_vf_vf($f32x::splat(3.1415927410125732422 * -0.5), y),
                        vmulsign_vf_vf_vf($f32x::splat(-8.7422776573475857731e-08 * -0.5), y),
                    );
                    x = dfidf + x;
                    dfidf = o.select_doubled(x, dfidf);
                    s = dfidf.normalize();

                    s.0 = $f32x::from_bits(vor_vm_vo32_vm(
                        d.is_infinite() | d.is_nan(),
                        $u32x::from_bits(s.0),
                    ));
                }

                let t = s;
                s = s.square();

                let u = $f32x::splat(2.6083159809786593541503e-06)
                    .mul_add(s.0, $f32x::splat(-0.0001981069071916863322258))
                    .mul_add(s.0, $f32x::splat(0.00833307858556509017944336));

                let x = ONE.add_checked(
                    $f32x::splat(-0.166666597127914428710938).add_checked_as_doubled(u * s.0) * s,
                );

                let u = t.mul_as_f(x);

                $f32x::from_bits(
                    vand_vm_vo32_vm(
                        (q & $i32x::splat(2)).eq($i32x::splat(0)),
                        $u32x::from_bits($f32x::splat(-0.)),
                    ) ^ $u32x::from_bits(u),
                )
            }

            pub fn sincosf(d: $f32x) -> ($f32x, $f32x) {
                let q: $i32x;
                let mut s: Doubled<$f32x>;

                if d.abs().lt($f32x::splat(TRIGRANGEMAX2_F)).all() {
                    let u = (d * $f32x::FRAC_2_PI).rint();
                    q = u.rinti();
                    let v = u.mul_add($f32x::splat(-PI_A2_F * 0.5), d);
                    s = v.add_as_doubled(u * $f32x::splat(-PI_B2_F * 0.5));
                    s = s.add_checked(u * $f32x::splat(-PI_C2_F * 0.5));
                } else {
                    let (dfidf, dfii) = rempif(d);
                    q = dfii;
                    s = dfidf;
                    let o = d.is_infinite() | d.is_nan();
                    s.0 = $f32x::from_bits(vor_vm_vo32_vm(o, $u32x::from_bits(s.0)));
                }

                let t = s;

                s.0 = s.square_as_f();

                let u = $f32x::splat(-0.000195169282960705459117889)
                    .mul_add(s.0, $f32x::splat(0.00833215750753879547119141))
                    .mul_add(s.0, $f32x::splat(-0.166666537523269653320312))
                    * (s.0 * t.0);

                let x = t.add_checked(u);
                let rx = x.0 + x.1;

                let rx = visnegzero_vo_vf(d).select($f32x::splat(-0.), rx);

                let u = $f32x::splat(-2.71811842367242206819355e-07)
                    .mul_add(s.0, $f32x::splat(2.47990446951007470488548e-05))
                    .mul_add(s.0, $f32x::splat(-0.00138888787478208541870117))
                    .mul_add(s.0, $f32x::splat(0.0416666641831398010253906))
                    .mul_add(s.0, $f32x::splat(-0.5));

                let x = ONE.add_checked(s.0.mul_as_doubled(u));
                let ry = x.0 + x.1;

                let o = (q & $i32x::splat(1)).eq($i32x::splat(0));
                let mut rsin = o.select(rx, ry);
                let mut rcos = o.select(ry, rx);

                let o = (q & $i32x::splat(2)).eq($i32x::splat(2));
                rsin = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(rsin),
                );

                let o = ((q + $i32x::splat(1)) & $i32x::splat(2)).eq($i32x::splat(2));
                rcos = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(rcos),
                );

                (rsin, rcos)
            }


            pub fn tanf(d: $f32x) -> $f32x {
                let q: $i32x;

                let mut s = if d.abs().lt($f32x::splat(TRIGRANGEMAX2_F)).all() {
                    let u = (d * $f32x::FRAC_2_PI).rint();
                    q = u.rinti();
                    let v = u.mul_add($f32x::splat(-PI_A2_F * 0.5), d);
                    v.add_as_doubled(u * $f32x::splat(-PI_B2_F * 0.5))
                        .add_checked(u * $f32x::splat(-PI_C2_F * 0.5))
                } else {
                    let (dfidf, dfii) = rempif(d);
                    q = dfii;
                    let o = d.is_infinite() | d.is_nan();
                    Doubled::new(
                        $f32x::from_bits(vor_vm_vo32_vm(o, $u32x::from_bits(dfidf.0))),
                        $f32x::from_bits(vor_vm_vo32_vm(o, $u32x::from_bits(dfidf.1))),
                    )
                };

                let o = (q & $i32x::splat(1)).eq($i32x::splat(1));
                let n = vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.)));
                s.0 = $f32x::from_bits($u32x::from_bits(s.0) ^ n);
                s.1 = $f32x::from_bits($u32x::from_bits(s.1) ^ n);

                let t = s;
                s = s.square();
                s = s.normalize();

                let u = $f32x::splat(0.00446636462584137916564941)
                    .mul_add(s.0, $f32x::splat(-8.3920182078145444393158e-05))
                    .mul_add(s.0, $f32x::splat(0.0109639242291450500488281))
                    .mul_add(s.0, $f32x::splat(0.0212360303848981857299805))
                    .mul_add(s.0, $f32x::splat(0.0540687143802642822265625));

                let mut x = $f32x::splat(0.133325666189193725585938).add_checked_as_doubled(u * s.0);
                x = ONE
                    .add_checked($f32x::splat(0.33333361148834228515625).add_checked(s * x) * s);
                x = t * x;

                x = o.select_doubled(x.recpre(), x);

                let u = x.0 + x.1;

                visnegzero_vo_vf(d).select(d, u)
            }


            //
            #[inline]
            fn atan2kf_u1(y: Doubled<$f32x>, mut x: Doubled<$f32x>) -> Doubled<$f32x> {
                let q =
                    vsel_vi2_vf_vf_vi2_vi2(x.0, ZERO, $i32x::splat(-2), $i32x::splat(0));
                let p = x.0.lt(ZERO);
                let r = vand_vm_vo32_vm(p, $u32x::from_bits($f32x::splat(-0.)));
                x.0 = $f32x::from_bits($u32x::from_bits(x.0) ^ r);
                x.1 = $f32x::from_bits($u32x::from_bits(x.1) ^ r);

                let q = vsel_vi2_vf_vf_vi2_vi2(x.0, y.0, q + $i32x::splat(1), q);
                let p = x.0.lt(y.0);
                let s = p.select_doubled(-x, y);
                let mut t = p.select_doubled(y, x);

                let s = s / t;
                t = s.square();
                t = t.normalize();

                let u = $f32x::splat(-0.00176397908944636583328247)
                    .mul_add(t.0, $f32x::splat(0.0107900900766253471374512))
                    .mul_add(t.0, $f32x::splat(-0.0309564601629972457885742))
                    .mul_add(t.0, $f32x::splat(0.0577365085482597351074219))
                    .mul_add(t.0, $f32x::splat(-0.0838950723409652709960938))
                    .mul_add(t.0, $f32x::splat(0.109463557600975036621094))
                    .mul_add(t.0, $f32x::splat(-0.142626821994781494140625))
                    .mul_add(t.0, $f32x::splat(0.199983194470405578613281));

                t *= $f32x::splat(-0.333332866430282592773438).add_checked_as_doubled(u * t.0);
                t = s * ONE.add_checked(t);
                (Doubled::from((1.5707963705062866211, -4.3711388286737928865e-08)) * $f32x::from_cast(q))
                    .add_checked(t)
            }

            pub fn atan2f(mut y: $f32x, mut x: $f32x) -> $f32x {
                let o = x.abs().lt($f32x::splat(2.9387372783541830947e-39)); // nexttowardf((1.0 / FLT_MAX), 1)
                x = o.select(x * F1_24X, x);
                y = o.select(y * F1_24X, y);

                let d = atan2kf_u1(
                    Doubled::new(y.abs(), ZERO),
                    Doubled::new(x, ZERO),
                );
                let mut r = d.0 + d.1;

                r = vmulsign_vf_vf_vf(r, x);
                r = (x.is_infinite() | x.eq(ZERO)).select(
                    $f32x::FRAC_PI_2
                        - visinf2_vf_vf_vf(x, vmulsign_vf_vf_vf($f32x::FRAC_PI_2, x)),
                    r,
                );
                r = y.is_infinite().select(
                    $f32x::FRAC_PI_2
                        - visinf2_vf_vf_vf(x, vmulsign_vf_vf_vf($f32x::FRAC_PI_4, x)),
                    r,
                );
                r = y.eq(ZERO).select(
                    $f32x::from_bits(vand_vm_vo32_vm(
                        vsignbit_vo_vf(x),
                        $u32x::from_bits($f32x::PI),
                    )),
                    r,
                );

                $f32x::from_bits(vor_vm_vo32_vm(
                    x.is_nan() | y.is_nan(),
                    $u32x::from_bits(vmulsign_vf_vf_vf(r, y)),
                ))
            }

            pub fn asinf(d: $f32x) -> $f32x {
                let o = d.abs().lt($f32x::splat(0.5));
                let x2 = o.select(d * d, (ONE - d.abs()) * $f32x::splat(0.5));
                let mut x = o.select_doubled(Doubled::new(d.abs(), ZERO), x2.sqrt_as_doubled());
                x = d.abs().eq(ONE).select_doubled(Doubled::from((0., 0.)), x);

                let u = $f32x::splat(0.4197454825e-1)
                    .mul_add(x2, $f32x::splat(0.2424046025e-1))
                    .mul_add(x2, $f32x::splat(0.4547423869e-1))
                    .mul_add(x2, $f32x::splat(0.7495029271e-1))
                    .mul_add(x2, $f32x::splat(0.1666677296e+0))
                    * (x2 * x.0);

                let y = Doubled::from((3.1415927410125732422 / 4., -8.7422776573475857731e-08 / 4.))
                    .sub_checked(x)
                    .sub_checked(u);

                let r = o.select(u + x.0, (y.0 + y.1) * $f32x::splat(2.));
                vmulsign_vf_vf_vf(r, d)
            }

            pub fn acosf(d: $f32x) -> $f32x {
                let o = d.abs().lt($f32x::splat(0.5));
                let x2 = o.select(d * d, (ONE - d.abs()) * $f32x::splat(0.5));

                let mut x = o.select_doubled(Doubled::new(d.abs(), ZERO), x2.sqrt_as_doubled());
                x = d.abs().eq(ONE).select_doubled(Doubled::from((0., 0.)), x);

                let u = $f32x::splat(0.4197454825e-1)
                    .mul_add(x2, $f32x::splat(0.2424046025e-1))
                    .mul_add(x2, $f32x::splat(0.4547423869e-1))
                    .mul_add(x2, $f32x::splat(0.7495029271e-1))
                    .mul_add(x2, $f32x::splat(0.1666677296e+0))
                    * (x2 * x.0);

                let mut y = Doubled::from((3.1415927410125732422 / 2., -8.7422776573475857731e-08 / 2.))
                    .sub_checked(vmulsign_vf_vf_vf(x.0, d).add_checked_as_doubled(vmulsign_vf_vf_vf(u, d)));
                x = x.add_checked(u);

                y = o.select_doubled(y, x.scale($f32x::splat(2.)));

                y = vandnot_vo_vo_vo(o, d.lt(ZERO)).select_doubled(Doubled::from((3.1415927410125732422, -8.7422776573475857731e-08)).sub_checked(y),
                    y,
                );

                y.0 + y.1
            }

            pub fn atanf(d: $f32x) -> $f32x {
                let d2 = atan2kf_u1(Doubled::new(d.abs(), ZERO), Doubled::from((1., 0.)));
                let mut r = d2.0 + d2.1;
                r = d.is_infinite().select($f32x::splat(1.570796326794896557998982), r);
                vmulsign_vf_vf_vf(r, d)
            }


            pub fn expf(d: $f32x) -> $f32x {
                let q = (d * $f32x::splat(R_LN2_F)).rinti();

                let s = $f32x::from_cast(q).mul_add($f32x::splat(-L2U_F), d);
                let s = $f32x::from_cast(q).mul_add($f32x::splat(-L2L_F), s);

                let mut u = $f32x::splat(0.000198527617612853646278381)
                    .mul_add(s, $f32x::splat(0.00139304355252534151077271))
                    .mul_add(s, $f32x::splat(0.00833336077630519866943359))
                    .mul_add(s, $f32x::splat(0.0416664853692054748535156))
                    .mul_add(s, $f32x::splat(0.166666671633720397949219))
                    .mul_add(s, $f32x::splat(0.5));

                u = ONE + (s * s).mul_add(u, s);

                u = vldexp2_vf_vf_vi2(u, q);

                u = $f32x::from_bits(vandnot_vm_vo32_vm(
                    d.lt($f32x::splat(-104.)),
                    $u32x::from_bits(u),
                ));
                $f32x::splat(100.)
                    .lt(d)
                    .select($f32x::INFINITY, u)
            }

            pub fn cbrtf(mut d: $f32x) -> $f32x {
                let mut q2 = Doubled::from((1., 0.));

                /*if cfg!(feature = "enable_avx512f") || cfg!(feature = "enable_avx512fnofma") {
                    let s = d;
                }*/
                let e = vilogbk_vi2_vf(d.abs()) + $i32x::splat(1);
                d = vldexp2_vf_vf_vi2(d, -e);

                let t = $f32x::from_cast(e) + $f32x::splat(6144.);
                let qu = (t * $f32x::splat(1. / 3.)).truncatei();
                let re = (t - $f32x::from_cast(qu) * $f32x::splat(3.)).truncatei();

                q2 = re.eq($i32x::splat(1)).select_doubled(Doubled::from((1.2599210739135742188, -2.4018701694217270415e-08)),
                    q2,
                );
                q2 = re.eq($i32x::splat(2)).select_doubled(Doubled::from((1.5874010324478149414, 1.9520385308169352356e-08)),
                    q2,
                );

                q2.0 = vmulsign_vf_vf_vf(q2.0, d);
                q2.1 = vmulsign_vf_vf_vf(q2.1, d);
                d = d.abs();

                let mut x = $f32x::splat(-0.601564466953277587890625)
                    .mul_add(d, $f32x::splat(2.8208892345428466796875))
                    .mul_add(d, $f32x::splat(-5.532182216644287109375))
                    .mul_add(d, $f32x::splat(5.898262500762939453125))
                    .mul_add(d, $f32x::splat(-3.8095417022705078125))
                    .mul_add(d, $f32x::splat(2.2241256237030029296875));

                let mut y = x * x;
                y = y * y;
                x -= vmlanp_vf_vf_vf_vf(d, y, x) * $f32x::splat(-1. / 3.);

                let mut z = x;

                let mut u = x.mul_as_doubled(x);
                u = u * u;
                u *= d;
                u += -x;
                y = u.0 + u.1;

                y = $f32x::splat(-2. / 3.) * y * z;
                let mut v = z.mul_as_doubled(z) + y;
                v *= d;
                v *= q2;
                z = vldexp2_vf_vf_vi2(v.0 + v.1, qu - $i32x::splat(2048));

                z = d.is_infinite().select(vmulsign_vf_vf_vf($f32x::INFINITY, q2.0), z);
                z = d
                    .eq(ZERO)
                    .select($f32x::from_bits(vsignbit_vm_vf(q2.0)), z);

                /*if cfg!(feature = "enable_avx512f") || cfg!(feature = "enable_avx512fnofma") {
                    z = s.is_infinite().select(vmulsign_vf_vf_vf($f32x::INFINITY, s), z);
                    z = s
                        .eq(ZERO)
                        .select(vmulsign_vf_vf_vf(ZERO, s), z);
                }*/

                z
            }


            pub fn logf(mut d: $f32x) -> $f32x {
                let m: $f32x;

                let mut s = /*if !cfg!(feature = "enable_avx512f")
                    && !cfg!(feature = "enable_avx512fnofma")*/
                {
                    let o = d.lt($f32x::splat(f32::MIN));
                    d = o.select(d * (F1_32X * F1_32X), d);
                    let mut e = vilogb2k_vi2_vf(d * $f32x::splat(1. / 0.75));
                    m = vldexp3_vf_vf_vi2(d, -e);
                    e = o.select(e - $i32x::splat(64), e);
                    Doubled::from((0.69314718246459960938, -1.904654323148236017e-09)) * $f32x::from_cast(e)
                }/* else {
                    let mut e = vgetexp_vf_vf(d * $f32x::splat(1. / 0.75));
                    e = e.eq($f32x::INFINITY).select($f32x::splat(128.), e);
                    m = vgetmant_vf_vf(d);
                    Doubled::from((0.69314718246459960938, -1.904654323148236017e-09)) * e
                }*/;

                let x = $f32x::splat(-1.).add_as_doubled(m) / ONE.add_as_doubled(m);
                let x2 = x.0 * x.0;

                let t = $f32x::splat(0.3027294874e+0)
                    .mul_add(x2, $f32x::splat(0.3996108174e+0))
                    .mul_add(x2, $f32x::splat(0.6666694880e+0));

                s = s.add_checked(x.scale($f32x::splat(2.)));
                s = s.add_checked(x2 * x.0 * t);

                let r = s.0 + s.1;

                /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma") {*/
                    let r = d.eq($f32x::INFINITY).select($f32x::INFINITY, r);
                    let r =
                        (d.lt(ZERO) | d.is_nan()).select($f32x::NAN, r);
                    d.eq(ZERO)
                        .select($f32x::NEG_INFINITY, r)
                /*} else {
                    vfixup_vf_vf_vf_vi2_i(
                        r,
                        d,
                        $i32x::splat((4 << (2 * 4)) | (3 << (4 * 4)) | (5 << (5 * 4)) | (2 << (6 * 4))),
                        0,
                    )
                }*/
            }


            pub fn powf(x: $f32x, y: $f32x) -> $f32x {
                if true {
                    let yisint = y.truncate().eq(y) | y.abs().gt(F1_24X);
                    let yisodd = (y.truncatei() & $i32x::splat(1)).eq($i32x::splat(1))
                        & yisint
                        & y.abs().lt(F1_24X);

                    #[cfg(any(feature = "enable_neon32", feature = "enable_neon32vfpv4"))]
                    {
                        let yisodd = vandnot_vm_vo32_vm(y.is_infinite(), yisodd);
                    }

                    let mut result = expkf(logkf(x.abs()) * y);

                    result = result.is_nan().select($f32x::INFINITY, result);

                    result *= x.gt(ZERO).select(
                        ONE,
                        yisint.select(
                            yisodd.select($f32x::splat(-1.), ONE),
                            $f32x::NAN,
                        ),
                    );

                    let efx = vmulsign_vf_vf_vf(x.abs() - ONE, y);

                    result = y.is_infinite().select(
                        $f32x::from_bits(vandnot_vm_vo32_vm(
                            efx.lt(ZERO),
                            $u32x::from_bits(
                                efx.eq(ZERO)
                                    .select(ONE, $f32x::INFINITY),
                            ),
                        )),
                        result,
                    );

                    result = (x.is_infinite() | x.eq(ZERO)).select(
                        yisodd.select(vsign_vf_vf(x), ONE) * $f32x::from_bits(
                            vandnot_vm_vo32_vm(
                                x.eq(ZERO).select(-y, y).lt(ZERO),
                                $u32x::from_bits($f32x::INFINITY),
                            ),
                        ),
                        result,
                    );

                    result = $f32x::from_bits(vor_vm_vo32_vm(
                        x.is_nan() | y.is_nan(),
                        $u32x::from_bits(result),
                    ));

                    (y.eq(ZERO) | x.eq(ONE))
                        .select(ONE, result)
                } else {
                    expkf(logkf(x) * y)
                }
            }


            pub fn sinhf(x: $f32x) -> $f32x {
                let mut y = x.abs();
                let d = expk2f(Doubled::new(y, ZERO));
                let d = d.sub_checked(d.recpre());
                y = (d.0 + d.1) * $f32x::splat(0.5);

                y = (x.abs().gt($f32x::splat(89.)) | y.is_nan())
                    .select($f32x::INFINITY, y);
                y = vmulsign_vf_vf_vf(y, x);
                $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)))
            }

            pub fn coshf(x: $f32x) -> $f32x {
                let mut y = x.abs();
                let d = expk2f(Doubled::new(y, ZERO));
                let d = d.add_checked(d.recpre());
                y = (d.0 + d.1) * $f32x::splat(0.5);

                y = (x.abs().gt($f32x::splat(89.)) | y.is_nan())
                    .select($f32x::INFINITY, y);
                $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)))
            }

            pub fn tanhf(x: $f32x) -> $f32x {
                let mut y = x.abs();
                let d = expk2f(Doubled::new(y, ZERO));
                let e = d.recpre();
                let d = d.add_checked(-e) / d.add_checked(e);
                y = d.0 + d.1;

                y = (x.abs().gt($f32x::splat(8.664339742)) | y.is_nan())
                    .select(ONE, y);
                y = vmulsign_vf_vf_vf(y, x);
                $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)))
            }


            pub fn asinhf(x: $f32x) -> $f32x {
                let mut y = x.abs();
                let o = y.gt(ONE);

                let mut d = o.select_doubled(x.recpre_as_doubled(), Doubled::new(y, ZERO));
                d = (d.square() + ONE).sqrt();
                d = o.select_doubled(d * y, d);

                d = logk2f((d + x).normalize());
                y = d.0 + d.1;

                y = (x.abs().gt($f32x::splat(SQRT_FLT_MAX)) | y.is_nan())
                    .select(vmulsign_vf_vf_vf($f32x::INFINITY, x), y);
                y = $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)));
                visnegzero_vo_vf(x).select($f32x::splat(-0.), y)
            }

            pub fn acoshf(x: $f32x) -> $f32x {
                let d = logk2f(
                    x.add_as_doubled(ONE).sqrt() * x.add_as_doubled($f32x::splat(-1.)).sqrt() + x,
                );
                let mut y = d.0 + d.1;

                y = (x.abs().gt($f32x::splat(SQRT_FLT_MAX)) | y.is_nan())
                    .select($f32x::INFINITY, y);

                y = $f32x::from_bits(vandnot_vm_vo32_vm(
                    x.eq(ONE),
                    $u32x::from_bits(y),
                ));

                y = $f32x::from_bits(vor_vm_vo32_vm(x.lt(ONE), $u32x::from_bits(y)));
                $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)))
            }

            pub fn atanhf(x: $f32x) -> $f32x {
                let mut y = x.abs();
                let d = logk2f(ONE.add_as_doubled(y) / ONE.add_as_doubled(-y));
                y = $f32x::from_bits(vor_vm_vo32_vm(
                    y.gt(ONE),
                    $u32x::from_bits(y.eq(ONE).select(
                        $f32x::INFINITY,
                        (d.0 + d.1) * $f32x::splat(0.5),
                    )),
                ));

                y = $f32x::from_bits(vor_vm_vo32_vm(
                    x.is_infinite() | y.is_nan(),
                    $u32x::from_bits(y),
                ));
                y = vmulsign_vf_vf_vf(y, x);
                $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)))
            }


            pub fn exp10f(d: $f32x) -> $f32x {
                let mut u = (d * $f32x::splat(LOG10_2_F)).rint();
                let q = u.rinti();

                let s = u.mul_add($f32x::splat(-L10U_F), d);
                let s = u.mul_add($f32x::splat(-L10L_F), s);

                u = $f32x::splat(0.2064004987e+0)
                    .mul_add(s, $f32x::splat(0.5417877436e+0))
                    .mul_add(s, $f32x::splat(0.1171286821e+1))
                    .mul_add(s, $f32x::splat(0.2034656048e+1))
                    .mul_add(s, $f32x::splat(0.2650948763e+1))
                    .mul_add(s, $f32x::splat(0.2302585125e+1));

                if !cfg!(target_feature = "fma") {
                    u = u.mul_adde(s, ONE);
                } else {
                    u = ONE.add_checked(u.mul_as_doubled(s)).normalize().0;
                }

                u = vldexp2_vf_vf_vi2(u, q);

                u = d
                    .gt($f32x::splat(38.5318394191036238941387))
                    .select($f32x::INFINITY, u);
                $f32x::from_bits(vandnot_vm_vo32_vm(
                    d.lt($f32x::splat(-50.)),
                    $u32x::from_bits(u),
                ))
            }

            pub fn expm1f(a: $f32x) -> $f32x {
                let d = expk2f(Doubled::new(a, ZERO)) + $f32x::splat(-1.);
                let mut x = d.0 + d.1;
                x = a
                    .gt($f32x::splat(88.72283172607421875))
                    .select($f32x::INFINITY, x);
                x = a
                    .lt($f32x::splat(-16.635532333438687426013570))
                    .select($f32x::splat(-1.), x);
                visnegzero_vo_vf(a).select($f32x::splat(-0.), x)
            }

            pub fn log10f(mut d: $f32x) -> $f32x {
                let m: $f32x;

                let mut s =
                    /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma")*/ {
                        let o = d.lt($f32x::splat(f32::MIN));
                        d = o.select(d * (F1_32X * F1_32X), d);
                        let mut e = vilogb2k_vi2_vf(d * $f32x::splat(1. / 0.75));
                        m = vldexp3_vf_vf_vi2(d, -e);
                        e = o.select(e - $i32x::splat(64), e);
                        Doubled::from((0.30103001, -1.432098889e-08)) * $f32x::from_cast(e)
                    }/* else {
                        let mut e = vgetexp_vf_vf(d * $f32x::splat(1. / 0.75));
                        e = e.eq($f32x::INFINITY).select($f32x::splat(128.), e);
                        m = vgetmant_vf_vf(d);
                        Doubled::from((0.30103001, -1.432098889e-08)) * e
                    }*/;

                let x = $f32x::splat(-1.).add_as_doubled(m) / ONE.add_as_doubled(m);
                let x2 = x.0 * x.0;

                let t = $f32x::splat(0.1314289868e+0)
                    .mul_add(x2, $f32x::splat(0.1735493541e+0))
                    .mul_add(x2, $f32x::splat(0.2895309627e+0));

                s = s.add_checked(x * Doubled::from((0.868588984, -2.170757285e-08)));
                s = s.add_checked(x2 * x.0 * t);

                let mut r = s.0 + s.1;

                /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma") {*/
                    r = d.eq($f32x::INFINITY).select($f32x::INFINITY, r);
                    r = (d.lt(ZERO) | d.is_nan()).select($f32x::NAN, r);
                    d.eq(ZERO)
                        .select($f32x::NEG_INFINITY, r)
                /*} else {
                    vfixup_vf_vf_vf_vi2_i(
                        r,
                        d,
                        $i32x::splat((4 << (2 * 4)) | (3 << (4 * 4)) | (5 << (5 * 4)) | (2 << (6 * 4))),
                        0,
                    )
                }*/
            }

            pub fn log2f(mut d: $f32x) -> $f32x {
                let m: $f32x;

                let ef =
                    /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma")*/ {
                        let o = d.lt($f32x::splat(f32::MIN));
                        d = o.select(d * (F1_32X * F1_32X), d);
                        let mut e = vilogb2k_vi2_vf(d * $f32x::splat(1. / 0.75));
                        m = vldexp3_vf_vf_vi2(d, -e);
                        e = o.select(e - $i32x::splat(64), e);
                        $f32x::from_cast(e)
                    }/* else {
                        let mut e = vgetexp_vf_vf(d * $f32x::splat(1. / 0.75));
                        e = e.eq($f32x::INFINITY).select($f32x::splat(128.), e);
                        m = vgetmant_vf_vf(d);
                        e
                    }*/;

                let x = $f32x::splat(-1.).add_as_doubled(m) / ONE.add_as_doubled(m);
                let x2 = x.0 * x.0;

                let t = $f32x::splat(0.4374550283e+0)
                    .mul_add(x2, $f32x::splat(0.5764790177e+0))
                    .mul_add(x2, $f32x::splat(0.9618012905120));
                let mut s = ef + x * Doubled::from((2.8853900432586669922, 3.2734474483568488616e-08));
                s += x2 * x.0 * t;

                let mut r = s.0 + s.1;

                /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma") {*/
                    r = d.eq($f32x::INFINITY).select($f32x::INFINITY, r);
                    r = (d.lt(ZERO) | d.is_nan()).select($f32x::NAN, r);
                    d.eq(ZERO)
                        .select($f32x::NEG_INFINITY, r)
                /*} else {
                    vfixup_vf_vf_vf_vi2_i(
                        r,
                        d,
                        $i32x::splat((4 << (2 * 4)) | (3 << (4 * 4)) | (5 << (5 * 4)) | (2 << (6 * 4))),
                        0,
                    )
                }*/
            }



            pub fn tgammaf(a: $f32x) -> $f32x {
                let (da, db) = gammafk(a);
                let y = expk2f(da) * db;
                let r = y.0 + y.1;

                let o = a.eq($f32x::NEG_INFINITY)
                    | (a.lt(ZERO) & visint_vo_vf(a))
                    | (a.is_finite() & a.lt(ZERO) & r.is_nan());
                let r = o.select($f32x::NAN, r);

                let o = (a.eq($f32x::INFINITY) | a.is_finite())
                    & a.ge($f32x::splat(-f32::MIN))
                    & (a.eq(ZERO) | a.gt($f32x::splat(36.)) | r.is_nan());
                o.select(vmulsign_vf_vf_vf($f32x::INFINITY, a), r)
            }

            pub fn lgammaf(a: $f32x) -> $f32x {
                let (da, db) = gammafk(a);
                let y = da + logk2f(db.abs());
                let r = y.0 + y.1;

                let o = a.is_infinite()
                    | ((a.le(ZERO) & visint_vo_vf(a))
                        | (a.is_finite() & r.is_nan()));
                o.select($f32x::INFINITY, r)
            }

            /* TODO AArch64: potential optimization by using `vfmad_lane_f64` */
            pub fn erff(a: $f32x) -> $f32x {
                let s = a;

                let a = a.abs();
                let o0 = a.lt($f32x::splat(1.1));
                let o1 = a.lt($f32x::splat(2.4));
                let o2 = a.lt($f32x::splat(4.));
                let u = o0.select(a * a, a);

                let t =
                    vsel_vf_vo_vo_f_f_f(o0, o1, 0.7089292194e-4, -0.1792667899e-4, -0.9495757695e-5)
                        .mul_add(
                            u,
                            vsel_vf_vo_vo_f_f_f(
                                o0,
                                o1,
                                -0.7768311189e-3,
                                0.3937633010e-3,
                                0.2481465926e-3,
                            ),
                        ).mul_add(
                            u,
                            vsel_vf_vo_vo_f_f_f(
                                o0,
                                o1,
                                0.5159463733e-2,
                                -0.3949181177e-2,
                                -0.2918176819e-2,
                            ),
                        ).mul_add(
                            u,
                            vsel_vf_vo_vo_f_f_f(
                                o0,
                                o1,
                                -0.2683781274e-1,
                                0.2445474640e-1,
                                0.2059706673e-1,
                            ),
                        ).mul_add(
                            u,
                            vsel_vf_vo_vo_f_f_f(
                                o0,
                                o1,
                                0.1128318012e+0,
                                -0.1070996150e+0,
                                -0.9901899844e-1,
                            ),
                        );
                let mut d = t.mul_as_doubled(u);
                d += vsel_vf2_vo_vo_d_d_d(
                    o0,
                    o1,
                    -0.376125876000657465175213237214e+0,
                    -0.634588905908410389971210809210e+0,
                    -0.643598050547891613081201721633e+0,
                );
                d *= u;
                d += vsel_vf2_vo_vo_d_d_d(
                    o0,
                    o1,
                    0.112837916021059138255978217023e+1,
                    -0.112879855826694507209862753992e+1,
                    -0.112461487742845562801052956293e+1,
                );
                d *= a;
                d = o0.select_doubled(d, ONE.add_checked(-expk2f(d)));
                let u = vmulsign_vf_vf_vf(o2.select(d.0 + d.1, ONE), s);
                a.is_nan().select($f32x::NAN, u)
            }

        }

        pub mod u15 {
            //! Functions with 1.5 ULP error bound
            use super::*;

            /* TODO AArch64: potential optimization by using `vfmad_lane_f64` */
            pub fn erfcf(a: $f32x) -> $f32x {
                let s = a;
                let a = a.abs();
                let o0 = a.lt(ONE);
                let o1 = a.lt($f32x::splat(2.2));
                let o2 = a.lt($f32x::splat(4.3));
                let o3 = a.lt($f32x::splat(10.1));

                let u = o1.select_doubled(Doubled::new(a, ZERO),
                    Doubled::from((1., 0.)) / Doubled::new(a, ZERO),
                );

                let t = vsel_vf_vo_vo_vo_f_f_f_f(
                    o0,
                    o1,
                    o2,
                    -0.8638041618e-4,
                    -0.6236977242e-5,
                    -0.3869504035e+0,
                    0.1115344167e+1,
                ).mul_add(
                    u.0,
                    vsel_vf_vo_vo_vo_f_f_f_f(
                        o0,
                        o1,
                        o2,
                        0.6000166177e-3,
                        0.5749821503e-4,
                        0.1288077235e+1,
                        -0.9454904199e+0,
                    ),
                ).mul_add(
                    u.0,
                    vsel_vf_vo_vo_vo_f_f_f_f(
                        o0,
                        o1,
                        o2,
                        -0.1665703603e-2,
                        0.6002851478e-5,
                        -0.1816803217e+1,
                        -0.3667259514e+0,
                    ),
                ).mul_add(
                    u.0,
                    vsel_vf_vo_vo_vo_f_f_f_f(
                        o0,
                        o1,
                        o2,
                        0.1795156277e-3,
                        -0.2851036377e-2,
                        0.1249150872e+1,
                        0.7155663371e+0,
                    ),
                ).mul_add(
                    u.0,
                    vsel_vf_vo_vo_vo_f_f_f_f(
                        o0,
                        o1,
                        o2,
                        0.1914106123e-1,
                        0.2260518074e-1,
                        -0.1328857988e+0,
                        -0.1262947265e-1,
                    ),
                );

                let mut d = u * t;
                d += vsel_vf2_vo_vo_vo_d_d_d_d(
                    o0,
                    o1,
                    o2,
                    -0.102775359343930288081655368891e+0,
                    -0.105247583459338632253369014063e+0,
                    -0.482365310333045318680618892669e+0,
                    -0.498961546254537647970305302739e+0,
                );
                d *= u;
                d += vsel_vf2_vo_vo_vo_d_d_d_d(
                    o0,
                    o1,
                    o2,
                    -0.636619483208481931303752546439e+0,
                    -0.635609463574589034216723775292e+0,
                    -0.134450203224533979217859332703e-2,
                    -0.471199543422848492080722832666e-4,
                );
                d *= u;
                d += vsel_vf2_vo_vo_vo_d_d_d_d(
                    o0,
                    o1,
                    o2,
                    -0.112837917790537404939545770596e+1,
                    -0.112855987376668622084547028949e+1,
                    -0.572319781150472949561786101080e+0,
                    -0.572364030327966044425932623525e+0,
                );

                let mut x = o1.select_doubled(d, Doubled::new(-a, ZERO)) * a;
                x = o1.select_doubled(x, x + d);

                x = expk2f(x);
                x = o1.select_doubled(x, x * u);

                let mut r = o3.select(x.0 + x.1, ZERO);
                r = vsignbit_vo_vf(s).select($f32x::splat(2.) - r, r);
                s.is_nan().select($f32x::NAN, r)
            }

        }

        pub mod u35 {
            //! Functions with 3.5 ULP error bound
            use super::*;

            pub fn sinf(mut d: $f32x) -> $f32x {
                let mut q: $i32x;
                let u: $f32x;
                let r = d;

                if d.abs().lt($f32x::splat(TRIGRANGEMAX2_F)).all() {
                    q = (d * $f32x::FRAC_1_PI).rinti();
                    u = $f32x::from_cast(q);
                    d = u.mul_add($f32x::splat(-PI_A2_F), d);
                    d = u.mul_add($f32x::splat(-PI_B2_F), d);
                    d = u.mul_add($f32x::splat(-PI_C2_F), d);
                } else if d.abs().lt($f32x::splat(TRIGRANGEMAX_F)).all() {
                    q = (d * $f32x::FRAC_1_PI).rinti();
                    u = $f32x::from_cast(q);
                    d = u.mul_add($f32x::splat(-PI_A_F), d);
                    d = u.mul_add($f32x::splat(-PI_B_F), d);
                    d = u.mul_add($f32x::splat(-PI_C_F), d);
                    d = u.mul_add($f32x::splat(-PI_D_F), d);
                } else {
                    let (mut dfidf, dfii) = rempif(d);
                    q = dfii & $i32x::splat(3);
                    q = q + q + dfidf
                        .0
                        .gt(ZERO)
                        .select($i32x::splat(2), $i32x::splat(1));
                    q = q >> 2;
                    let o = (dfii & $i32x::splat(1)).eq($i32x::splat(1));
                    let mut x = Doubled::new(
                        vmulsign_vf_vf_vf($f32x::splat(3.1415927410125732422 * -0.5), dfidf.0),
                        vmulsign_vf_vf_vf($f32x::splat(-8.7422776573475857731e-08 * -0.5), dfidf.0),
                    );
                    x = dfidf + x;
                    dfidf = o.select_doubled(x, dfidf);
                    d = dfidf.0 + dfidf.1;

                    d = $f32x::from_bits(vor_vm_vo32_vm(
                        r.is_infinite() | r.is_nan(),
                        $u32x::from_bits(d),
                    ));
                }

                let s = d * d;

                d = $f32x::from_bits(
                    vand_vm_vo32_vm(
                        (q & $i32x::splat(1)).eq($i32x::splat(1)),
                        $u32x::from_bits($f32x::splat(-0.)),
                    ) ^ $u32x::from_bits(d),
                );

                let mut u = $f32x::splat(2.6083159809786593541503e-06)
                    .mul_add(s, $f32x::splat(-0.0001981069071916863322258))
                    .mul_add(s, $f32x::splat(0.00833307858556509017944336))
                    .mul_add(s, $f32x::splat(-0.166666597127914428710938));

                u = s * (u * d) + d;

                visnegzero_vo_vf(r).select(r, u)
            }

            pub fn cosf(mut d: $f32x) -> $f32x {
                let mut q: $i32x;
                let r = d;

                if d.abs().lt($f32x::splat(TRIGRANGEMAX2_F)).all() {
                    q = (d * $f32x::FRAC_1_PI - $f32x::splat(0.5)).rinti();
                    q = q + q + $i32x::splat(1);

                    let u = $f32x::from_cast(q);
                    d = u.mul_add($f32x::splat(-PI_A2_F * 0.5), d);
                    d = u.mul_add($f32x::splat(-PI_B2_F * 0.5), d);
                    d = u.mul_add($f32x::splat(-PI_C2_F * 0.5), d);
                } else if d.abs().lt($f32x::splat(TRIGRANGEMAX_F)).all() {
                    q = (d * $f32x::FRAC_1_PI - $f32x::splat(0.5)).rinti();
                    q = q + q + $i32x::splat(1);

                    let u = $f32x::from_cast(q);
                    d = u.mul_add($f32x::splat(-PI_A_F * 0.5), d);
                    d = u.mul_add($f32x::splat(-PI_B_F * 0.5), d);
                    d = u.mul_add($f32x::splat(-PI_C_F * 0.5), d);
                    d = u.mul_add($f32x::splat(-PI_D_F * 0.5), d);
                } else {
                    let (mut dfidf, dfii) = rempif(d);
                    q = dfii & $i32x::splat(3);
                    q = q + q + dfidf
                        .0
                        .gt(ZERO)
                        .select($i32x::splat(8), $i32x::splat(7));
                    q = q >> 1;
                    let o = (dfii & $i32x::splat(1)).eq($i32x::splat(0));
                    let y = dfidf
                        .0
                        .gt(ZERO)
                        .select(ZERO, $f32x::splat(-1.));
                    let mut x = Doubled::new(
                        vmulsign_vf_vf_vf($f32x::splat(3.1415927410125732422 * -0.5), y),
                        vmulsign_vf_vf_vf($f32x::splat(-8.7422776573475857731e-08 * -0.5), y),
                    );
                    x = dfidf + x;
                    dfidf = o.select_doubled(x, dfidf);
                    d = dfidf.0 + dfidf.1;

                    d = $f32x::from_bits(vor_vm_vo32_vm(
                        r.is_infinite() | r.is_nan(),
                        $u32x::from_bits(d),
                    ));
                }

                let s = d * d;

                d = $f32x::from_bits(
                    vand_vm_vo32_vm(
                        (q & $i32x::splat(2)).eq($i32x::splat(0)),
                        $u32x::from_bits($f32x::splat(-0.)),
                    ) ^ $u32x::from_bits(d),
                );

                let u = $f32x::splat(2.6083159809786593541503e-06)
                    .mul_add(s, $f32x::splat(-0.0001981069071916863322258))
                    .mul_add(s, $f32x::splat(0.00833307858556509017944336))
                    .mul_add(s, $f32x::splat(-0.166666597127914428710938));

                s * (u * d) + d
            }

            pub fn tanf(d: $f32x) -> $f32x {
                let q: $i32x;

                let mut x = d;

                if d.abs().lt($f32x::splat(TRIGRANGEMAX2_F * 0.5)).all() {
                    q = (d * $f32x::FRAC_2_PI).rinti();
                    let u = $f32x::from_cast(q);
                    x = u.mul_add($f32x::splat(-PI_A2_F * 0.5), x);
                    x = u.mul_add($f32x::splat(-PI_B2_F * 0.5), x);
                    x = u.mul_add($f32x::splat(-PI_C2_F * 0.5), x);
                } else if d.abs().lt($f32x::splat(TRIGRANGEMAX_F)).all() {
                    q = (d * (2. * $f32x::FRAC_1_PI)).rinti();
                    let u = $f32x::from_cast(q);
                    x = u.mul_add($f32x::splat(-PI_A_F * 0.5), x);
                    x = u.mul_add($f32x::splat(-PI_B_F * 0.5), x);
                    x = u.mul_add($f32x::splat(-PI_C_F * 0.5), x);
                    x = u.mul_add($f32x::splat(-PI_D_F * 0.5), x);
                } else {
                    let (dfidf, dfii) = rempif(d);
                    q = dfii;
                    x = dfidf.0 + dfidf.1;
                    x = $f32x::from_bits(vor_vm_vo32_vm(
                        d.is_infinite() | d.is_nan(),
                        $u32x::from_bits(x),
                    ));
                    x = visnegzero_vo_vf(d).select(d, x);
                }

                let s = x * x;

                let o = (q & $i32x::splat(1)).eq($i32x::splat(1));
                x = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(x),
                );

                let mut u = $f32x::splat(0.00927245803177356719970703)
                    .mul_add(s, $f32x::splat(0.00331984995864331722259521))
                    .mul_add(s, $f32x::splat(0.0242998078465461730957031))
                    .mul_add(s, $f32x::splat(0.0534495301544666290283203))
                    .mul_add(s, $f32x::splat(0.133383005857467651367188))
                    .mul_add(s, $f32x::splat(0.333331853151321411132812));

                u = s.mul_add(u * x, x);

                o.select(u.recpre(), u)
            }

            pub fn sincosf(d: $f32x) -> ($f32x, $f32x) {
                let q: $i32x;
                let mut s = d;

                if d.abs().lt($f32x::splat(TRIGRANGEMAX2_F)).all() {
                    q = (d * $f32x::FRAC_2_PI).rinti();
                    let u = $f32x::from_cast(q);
                    s = u.mul_add($f32x::splat(-PI_A2_F * 0.5), s);
                    s = u.mul_add($f32x::splat(-PI_B2_F * 0.5), s);
                    s = u.mul_add($f32x::splat(-PI_C2_F * 0.5), s);
                } else if d.abs().lt($f32x::splat(TRIGRANGEMAX_F)).all() {
                    q = (d * $f32x::FRAC_2_PI).rinti();
                    let u = $f32x::from_cast(q);
                    s = u.mul_add($f32x::splat(-PI_A_F * 0.5), s);
                    s = u.mul_add($f32x::splat(-PI_B_F * 0.5), s);
                    s = u.mul_add($f32x::splat(-PI_C_F * 0.5), s);
                    s = u.mul_add($f32x::splat(-PI_D_F * 0.5), s);
                } else {
                    let (dfidf, dfii) = rempif(d);
                    q = dfii;
                    s = dfidf.0 + dfidf.1;
                    s = $f32x::from_bits(vor_vm_vo32_vm(
                        d.is_infinite() | d.is_nan(),
                        $u32x::from_bits(s),
                    ));
                }

                let t = s;

                s = s * s;

                let u = $f32x::splat(-0.000195169282960705459117889)
                    .mul_add(s, $f32x::splat(0.00833215750753879547119141))
                    .mul_add(s, $f32x::splat(-0.166666537523269653320312));

                let rx = (u * s).mul_add(t, t);
                let rx = visnegzero_vo_vf(d).select($f32x::splat(-0.), rx);

                let u = $f32x::splat(-2.71811842367242206819355e-07)
                    .mul_add(s, $f32x::splat(2.47990446951007470488548e-05))
                    .mul_add(s, $f32x::splat(-0.00138888787478208541870117))
                    .mul_add(s, $f32x::splat(0.0416666641831398010253906))
                    .mul_add(s, $f32x::splat(-0.5));

                let ry = s.mul_add(u, ONE);

                let o = (q & $i32x::splat(1)).eq($i32x::splat(0));
                let mut rsin = o.select(rx, ry);
                let mut rcos = o.select(ry, rx);

                let o = (q & $i32x::splat(2)).eq($i32x::splat(2));
                rsin = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(rsin),
                );

                let o = ((q + $i32x::splat(1)) & $i32x::splat(2)).eq($i32x::splat(2));
                rcos = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(rcos),
                );

                (rsin, rcos)
            }

            pub fn sincospif(d: $f32x) -> ($f32x, $f32x) {
                let u = d * $f32x::splat(4.);
                let q = u.truncatei();
                let q = (q + ($i32x::from_bits($u32x::from_bits(q) >> 31) ^ $i32x::splat(1)))
                    & $i32x::splat(!1);
                let s = u - $f32x::from_cast(q);

                let t = s;
                let s = s * s;

                //

                let u = $f32x::splat(-0.3600925265e-4)
                    .mul_add(s, $f32x::splat(0.2490088111e-2))
                    .mul_add(s, $f32x::splat(-0.8074551076e-1))
                    .mul_add(s, $f32x::splat(0.7853981853e+0));

                let rx = u * t;

                //

                let u = $f32x::splat(0.3539815225e-5)
                    .mul_add(s, $f32x::splat(-0.3259574005e-3))
                    .mul_add(s, $f32x::splat(0.1585431583e-1))
                    .mul_add(s, $f32x::splat(-0.3084251285e+0))
                    .mul_add(s, ONE);

                let ry = u;

                //

                let o = (q & $i32x::splat(2)).eq($i32x::splat(0));
                let mut rsin = o.select(rx, ry);
                let mut rcos = o.select(ry, rx);

                let o = (q & $i32x::splat(4)).eq($i32x::splat(4));
                rsin = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(rsin),
                );

                let o = ((q + $i32x::splat(2)) & $i32x::splat(4)).eq($i32x::splat(4));
                rcos = $f32x::from_bits(
                    vand_vm_vo32_vm(o, $u32x::from_bits($f32x::splat(-0.))) ^ $u32x::from_bits(rcos),
                );

                let o = d.abs().gt($f32x::splat(1e+7));
                rsin = $f32x::from_bits(vandnot_vm_vo32_vm(o, $u32x::from_bits(rsin)));
                rcos = $f32x::from_bits(vandnot_vm_vo32_vm(o, $u32x::from_bits(rcos)));

                let o = d.is_infinite();
                rsin = $f32x::from_bits(vor_vm_vo32_vm(o, $u32x::from_bits(rsin)));
                rcos = $f32x::from_bits(vor_vm_vo32_vm(o, $u32x::from_bits(rcos)));

                (rsin, rcos)
            }

            pub fn atanf(d: $f32x) -> $f32x {
                let q = vsel_vi2_vf_vi2(d, $i32x::splat(2));
                let s = d.abs();

                let q = vsel_vi2_vf_vf_vi2_vi2(ONE, s, q + $i32x::splat(1), q);
                let s = ONE.lt(s).select(s.recpre(), s);

                let mut t = s * s;

                let u = $f32x::splat(0.00282363896258175373077393)
                    .mul_add(t, $f32x::splat(-0.0159569028764963150024414))
                    .mul_add(t, $f32x::splat(0.0425049886107444763183594))
                    .mul_add(t, $f32x::splat(-0.0748900920152664184570312))
                    .mul_add(t, $f32x::splat(0.106347933411598205566406))
                    .mul_add(t, $f32x::splat(-0.142027363181114196777344))
                    .mul_add(t, $f32x::splat(0.199926957488059997558594))
                    .mul_add(t, $f32x::splat(-0.333331018686294555664062));

                t = s.mul_add(t * u, s);

                t = (q & $i32x::splat(1))
                    .eq($i32x::splat(1))
                    .select($f32x::FRAC_PI_2 - t, t);

                t = $f32x::from_bits(
                    vand_vm_vo32_vm(
                        (q & $i32x::splat(2)).eq($i32x::splat(2)),
                        $u32x::from_bits($f32x::splat(-0.)),
                    ) ^ $u32x::from_bits(t),
                );

                if cfg!(feature = "enable_neon32") || cfg!(feature = "enable_neon32vfpv4") {
                    t = d.is_infinite().select(
                        vmulsign_vf_vf_vf($f32x::splat(1.5874010519681994747517056), d),
                        t,
                    );
                }

                t
            }

            pub fn atan2f(y: $f32x, x: $f32x) -> $f32x {
                let mut r = atan2kf(y.abs(), x);

                r = vmulsign_vf_vf_vf(r, x);
                r = (x.is_infinite() | x.eq(ZERO)).select(
                    $f32x::FRAC_PI_2
                        - visinf2_vf_vf_vf(x, vmulsign_vf_vf_vf($f32x::FRAC_PI_2, x)),
                    r,
                );
                r = y.is_infinite().select(
                    $f32x::FRAC_PI_2
                        - visinf2_vf_vf_vf(x, vmulsign_vf_vf_vf($f32x::FRAC_PI_4, x)),
                    r,
                );

                r = y.eq(ZERO).select(
                    $f32x::from_bits(vand_vm_vo32_vm(
                        vsignbit_vo_vf(x),
                        $u32x::from_bits($f32x::PI),
                    )),
                    r,
                );

                $f32x::from_bits(vor_vm_vo32_vm(
                    x.is_nan() | y.is_nan(),
                    $u32x::from_bits(vmulsign_vf_vf_vf(r, y)),
                ))
            }

            pub fn asinf(d: $f32x) -> $f32x {
                let o = d.abs().lt($f32x::splat(0.5));
                let x2 = o.select(d * d, (ONE - d.abs()) * $f32x::splat(0.5));
                let x = o.select(d.abs(), x2.sqrt());

                let u = $f32x::splat(0.4197454825e-1)
                    .mul_add(x2, $f32x::splat(0.2424046025e-1))
                    .mul_add(x2, $f32x::splat(0.4547423869e-1))
                    .mul_add(x2, $f32x::splat(0.7495029271e-1))
                    .mul_add(x2, $f32x::splat(0.1666677296e+0))
                    .mul_add(x * x2, x);

                let r = o.select(u, u.mul_add($f32x::splat(-2.), $f32x::FRAC_PI_2));
                vmulsign_vf_vf_vf(r, d)
            }

            pub fn acosf(d: $f32x) -> $f32x {
                let o = d.abs().lt($f32x::splat(0.5));
                let x2 = o.select(d * d, (ONE - d.abs()) * $f32x::splat(0.5));
                let mut x = o.select(d.abs(), x2.sqrt());
                x = d.abs().eq(ONE).select(ZERO, x);

                let u = $f32x::splat(0.4197454825e-1)
                    .mul_add(x2, $f32x::splat(0.2424046025e-1))
                    .mul_add(x2, $f32x::splat(0.4547423869e-1))
                    .mul_add(x2, $f32x::splat(0.7495029271e-1))
                    .mul_add(x2, $f32x::splat(0.1666677296e+0))
                    * (x2 * x);

                let y = $f32x::splat(3.1415926535897932 / 2.)
                    - (vmulsign_vf_vf_vf(x, d) + vmulsign_vf_vf_vf(u, d));
                x = x + u;
                let r = o.select(y, x * $f32x::splat(2.));
                vandnot_vo_vo_vo(o, d.lt(ZERO)).select(
                    Doubled::from((3.1415927410125732422, -8.7422776573475857731e-08))
                        .add_checked(-r)
                        .0,
                    r,
                )
            }

            pub fn logf(mut d: $f32x) -> $f32x {
                let m: $f32x;

                let ef = /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma")*/
                {
                    let o = d.lt($f32x::splat(f32::MIN));
                    d = o.select(d * (F1_32X * F1_32X), d);
                    let mut e = vilogb2k_vi2_vf(d * $f32x::splat(1. / 0.75));
                    m = vldexp3_vf_vf_vi2(d, -e);
                    e = o.select(e - $i32x::splat(64), e);
                    $f32x::from_cast(e)
                }/* else {
                    let mut e = vgetexp_vf_vf(d * $f32x::splat(1. / 0.75));
                    e = e.eq($f32x::INFINITY).select($f32x::splat(128.), e);
                    m = vgetmant_vf_vf(d);
                    e
                }*/;

                let mut x = ($f32x::splat(-1.) + m) / (ONE + m);
                let x2 = x * x;

                let t = $f32x::splat(0.2392828464508056640625)
                    .mul_add(x2, $f32x::splat(0.28518211841583251953125))
                    .mul_add(x2, $f32x::splat(0.400005877017974853515625))
                    .mul_add(x2, $f32x::splat(0.666666686534881591796875))
                    .mul_add(x2, $f32x::splat(2.));

                x = x.mul_add(t, $f32x::splat(0.693147180559945286226764) * ef);
                /*if !cfg!(feature = "enable_avx512f") && !cfg!(feature = "enable_avx512fnofma") {*/
                    x = d.eq($f32x::INFINITY).select($f32x::INFINITY, x);
                    x = (d.lt(ZERO) | d.is_nan()).select($f32x::NAN, x);
                    d.eq(ZERO)
                        .select($f32x::NEG_INFINITY, x)
                /*} else {
                    vfixup_vf_vf_vf_vi2_i(x, d, $i32x::splat(5 << (5 * 4)), 0)
                }*/
            }

            #[cfg(any(feature = "enable_neon32", feature = "enable_neon32vfpv4"))]
            pub fn sqrtf(d: $f32x) -> $f32x {
                let e = $f32x::from_bits(
                    $u32x::splat(0x20000000)
                        + ($u32x::splat(0x7f000000) & ($u32x::from_bits(d) >> 1)),
                );
                let m = $f32x::from_bits(
                    $i32x::splat(0x3f000000) + ($i32x::splat(0x01ffffff) & $i32x::from_bits(d)),
                );
                let mut x = vrsqrteq_f32(m);
                x = vmulq_f32(x, vrsqrtsq_f32(m, vmulq_f32(x, x)));
                let mut u = vmulq_f32(x, m);
                u = vmlaq_f32(u, vmlsq_f32(m, u, u), vmulq_f32(x, vdupq_n_f32(0.5)));
                e = $f32x::from_bits(vandnot_vm_vo32_vm(
                    d.eq(ZERO),
                    $u32x::from_bits(e),
                ));
                u = e * u;

                u = d.is_infinite().select($f32x::INFINITY, u);
                u = $f32x::from_bits(vor_vm_vo32_vm(
                    d.is_nan() | d.lt(ZERO),
                    $u32x::from_bits(u),
                ));
                vmulsign_vf_vf_vf(u, d)
            }
            /*#[cfg(feature = "enable_vecext")]
                    pub fn xsqrtf_u35(d: $f32x) -> $f32x {
                        let mut q = d.sqrt();
                        q = visnegzero_vo_vf(d).select($f32x::splat(-0.), q);
                        d.eq($f32x::INFINITY).select($f32x::INFINITY, q)
                    }*/
            #[cfg(all(
                        not(feature = "enable_neon32"),
                        not(feature = "enable_neon32vfpv4"),
                    //    not(feature = "enable_vecext")
                    ))]
            pub fn sqrtf(d: $f32x) -> $f32x {
                d.sqrt()
            }

            pub fn cbrtf(mut d: $f32x) -> $f32x {
                let mut q = ONE;

                /*if cfg!(feature = "enable_avx512f") || cfg!(feature = "enable_avx512fnofma") {
                    let s = d;
                }*/
                let e = vilogbk_vi2_vf(d.abs()) + $i32x::splat(1);
                d = vldexp2_vf_vf_vi2(d, -e);

                let t = $f32x::from_cast(e) + $f32x::splat(6144.);
                let qu = (t * $f32x::splat(1. / 3.)).truncatei();
                let re = (t - $f32x::from_cast(qu) * $f32x::splat(3.)).truncatei();

                q = re
                    .eq($i32x::splat(1))
                    .select($f32x::splat(1.2599210498948731647672106), q);
                q = re
                    .eq($i32x::splat(2))
                    .select($f32x::splat(1.5874010519681994747517056), q);
                q = vldexp2_vf_vf_vi2(q, qu - $i32x::splat(2048));

                q = vmulsign_vf_vf_vf(q, d);
                d = d.abs();

                let x = $f32x::splat(-0.601564466953277587890625)
                    .mul_add(d, $f32x::splat(2.8208892345428466796875))
                    .mul_add(d, $f32x::splat(-5.532182216644287109375))
                    .mul_add(d, $f32x::splat(5.898262500762939453125))
                    .mul_add(d, $f32x::splat(-3.8095417022705078125))
                    .mul_add(d, $f32x::splat(2.2241256237030029296875));

                let mut y = d * x * x;
                y = (y - $f32x::splat(2. / 3.) * y * y.mul_add(x, $f32x::splat(-1.))) * q;

                /*if cfg!(feature = "enable_avx512f") || cfg!(feature = "enable_avx512fnofma") {
                    y = s.is_infinite().select(vmulsign_vf_vf_vf($f32x::INFINITY, s), y);
                    y = s
                        .eq(ZERO)
                        .select(vmulsign_vf_vf_vf(ZERO, s), y);
                }*/

                y
            }


            pub fn sinhf(x: $f32x) -> $f32x {
                let e = expm1fk(x.abs());
                let mut y = (e + $f32x::splat(2.)) / (e + ONE);
                y *= $f32x::splat(0.5) * e;

                y = (x.abs().gt($f32x::splat(88.)) | y.is_nan())
                    .select($f32x::INFINITY, y);
                y = vmulsign_vf_vf_vf(y, x);
                $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)))
            }

            pub fn coshf(x: $f32x) -> $f32x {
                let e = u10::expf(x.abs());
                let mut y = $f32x::splat(0.5).mul_add(e, $f32x::splat(0.5) / e);

                y = (x.abs().gt($f32x::splat(88.)) | y.is_nan())
                    .select($f32x::INFINITY, y);
                $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)))
            }

            pub fn tanhf(x: $f32x) -> $f32x {
                let d = expm1fk($f32x::splat(2.) * x.abs());
                let mut y = d / ($f32x::splat(2.) + d);

                y = (x.abs().gt($f32x::splat(8.664339742)) | y.is_nan())
                    .select(ONE, y);
                y = vmulsign_vf_vf_vf(y, x);
                $f32x::from_bits(vor_vm_vo32_vm(x.is_nan(), $u32x::from_bits(y)))
            }


            pub fn hypotf(x: $f32x, y: $f32x) -> $f32x {
                let x = x.abs();
                let y = y.abs();
                let min = x.min(y);
                let max = x.max(y);

                let t = min / max;
                let mut ret = max * t.mul_add(t, ONE).sqrt();
                ret = min.eq(ZERO).select(max, ret);
                ret = (x.is_nan() | y.is_nan()).select($f32x::NAN, ret);
                (x.eq($f32x::INFINITY) | y.eq($f32x::INFINITY))
                    .select($f32x::INFINITY, ret)
            }


        }

    };
}
