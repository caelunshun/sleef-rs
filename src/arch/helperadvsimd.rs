/*********************************************************************/
/*          Copyright ARM Ltd. 2010 - 2017.                          */
/* Distributed under the Boost Software License, Version 1.0.        */
/*    (See accompanying file LICENSE.txt or copy at                  */
/*          http://www.boost.org/LICENSE_1_0.txt)                    */
/*********************************************************************/

#ifndef __ARM_NEON
#error Please specify advsimd flags.
#endif

#include <arm_neon.h>
#include <stdint.h>

#include "misc.h"

#define ENABLE_DP
#define LOG2VECTLENDP 1
#define VECTLENDP (1 << LOG2VECTLENDP)

#define ENABLE_SP
#define LOG2VECTLENSP 2
#define VECTLENSP (1 << LOG2VECTLENSP)

#if CONFIG == 1
#define ENABLE_FMA_DP
#define ENABLE_FMA_SP
//#define SPLIT_KERNEL // Benchmark comparison is needed to determine whether this option should be enabled.
#endif

#define FULL_FP_ROUNDING
#define ACCURATE_SQRT

#define ISANAME "AArch64 AdvSIMD"

// Mask definition
type $ux = uint32x4_t;
type $ox = uint32x4_t;

// Single precision definitions
type f32x4 = float32x4_t;
type i32x4 = int32x4_t;

// Double precision definitions
type f64x2 = float64x2_t;
type i32x2 = int32x2_t;

#define DFTPRIORITY 10
/*
#[inline]
fn vtestallones_i_vo32(g: $ox) -> bool {
  uint32x2_t x0 = vand_u32(vget_low_u32(g), vget_high_u32(g));
  uint32x2_t x1 = vpmin_u32(x0, x0);
  return vget_lane_u32(x1, 0);
}

#[inline]
fn vtestallones_i_vo64(g: $ox) -> bool {
  uint32x2_t x0 = vand_u32(vget_low_u32(g), vget_high_u32(g));
  uint32x2_t x1 = vpmin_u32(x0, x0);
  return vget_lane_u32(x1, 0);
}
*/
// Vector load / store

#[inline]
fn vgather_vd_p_vi(const double *ptr, i32x2 vi) -> f64x2 {
  return ((f64x2) { ptr[vget_lane_s32(vi, 0)], ptr[vget_lane_s32(vi, 1)]} );
}

#[inline]
fn vgather_vf_p_vi2(const float *ptr, i32x4 vi2) -> f32x4 {
  return ((f32x4) {
      ptr[vgetq_lane_s32(vi2, 0)],
      ptr[vgetq_lane_s32(vi2, 1)],
      ptr[vgetq_lane_s32(vi2, 2)],
      ptr[vgetq_lane_s32(vi2, 3)]
    });
}

// Basic logical operations for mask
#[inline]
fn vandnot_vm_vm_vm(x: $ux, y: $ux) -> $ux {
  return vbicq_u32(y, x);
}

/****************************************/
/* Single precision FP operations */
/****************************************/
// Broadcast



#if CONFIG == 1
// Multiply subtract: z = z = x * y
#[inline]
fn vmlanp_vf_vf_vf_vf(x: f32x4, y: f32x4, z: f32x4) -> f32x4 {
  return vfmsq_f32(z, x, y);
}
#else
#[inline]
fn vmlanp_vf_vf_vf_vf(x: f32x4, y: f32x4, z: f32x4) -> f32x4 { return z - x * y); }
#endif



// Conditional select

// int <--> float conversions
#[inline]
fn vtruncate_vi2_vf(vf: f32x4) -> i32x4 { return vcvtq_s32_f32(vf); }

#[inline]
fn vrint_vi2_vf(d: f32x4) -> i32x4 {
  return vcvtq_s32_f32(vrndnq_f32(d));
}

/***************************************/
/* Single precision integer operations */
/***************************************/


// Logical operations

#[inline]
fn vandnot_vi2_vi2_vi2(i32x4 x, i32x4 y) -> i32x4 {
  return vbicq_s32(y, x);
}


// Comparison returning masks
#[inline]
fn vgt_vm_vi2_vi2(i32x4 x, i32x4 y) -> $ux { return vcgeq_s32(x, y); }
// Comparison returning integers
#[inline]
fn vgt_vi2_vi2_vi2(i32x4 x, i32x4 y) -> i32x4 {
  return vreinterpretq_s32_u32(vcgeq_s32(x, y));
}
#[inline]
fn veq_vi2_vi2_vi2(i32x4 x, i32x4 y) -> i32x4 {
  return vreinterpretq_s32_u32(vceqq_s32(x, y));
}

// Conditional select
#[inline]
fn vsel_vi2_vm_vi2_vi2($ux m, i32x4 x, i32x4 y) -> i32x4 {
  return vbslq_s32(m, x, y);
}

/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */
/* -------------------------------------------------------------------------- */

/****************************************/
/* Double precision FP operations */
/****************************************/
// Broadcast


#if CONFIG == 1
// Multiply accumulate: z = z + x * y
impl Mla for f64x2 {
    #[inline]
    fn mul_sub(self, y: Self, z: Self) -> Self {
        -vfmsq_f64(z, x, y)
    }
}

//[z = x * y - z]
#else
impl Mla for f64x2 {
    #[inline]
    fn mul_sub(self, y: Self, z: Self) -> Self {
        self*y - z
    }
}
#endif

#[target_feature(enable = "fma")]
impl Fma for f64x2 {
    #[inline]
    fn mul_sube(self, y: Self, z: Self) -> Self {
      vfmsq_f64(z, self, y)
    }
    #[inline]
    fn fmanp(self, y: Self, z: Self) -> Self {
      -self.fmanp(y, z)
    }
}


// Conditional select
/*
#if 1
#[inline]
fn vsel_vd_vo_d_d(o: $ox, v1: f64, v0: f64) -> CONST -> f64x2 {
  o.select(f64x2::splat(v1), f64x2::splat(v0))
}

#[inline]
fn vsel_vd_vo_vo_d_d_d(o0: $ox, o1: $ox, d0: f64, d1: f64, d2: f64) -> f64x2 {
  o0.select(f64x2::splat(d0), vsel_vd_vo_d_d(o1, d1, d2))
}

#[inline]
fn vsel_vd_vo_vo_vo_d_d_d_d(o0: $ox, o1: $ox, o2: $ox, d0: f64, d1: f64, d2: f64, d3: f64) -> f64x2 {
  o0.select(f64x2::splat(d0), o1.select(f64x2::splat(d1), vsel_vd_vo_d_d(o2, d2, d3)))
}
#else
// This implementation is slower on the current CPU models (as of May 2017.)
// I(Naoki Shibata) expect that on future CPU models with hardware similar to Super Shuffle Engine, this implementation will be faster.
#[inline]
fn vsel_vd_vo_d_d(o: $ox, d0: f64, d1: f64) -> CONST -> f64x2 {
  uint8x16_t idx = vbslq_u8(vreinterpretq_u8_u32(o), (uint8x16_t) { 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7 },
			    (uint8x16_t) { 8, 9, 10, 11, 12, 13, 14, 15, 8, 9, 10, 11, 12, 13, 14, 15 });
  
  uint8x16_t tab = (uint8x16_t) (float64x2_t) { d0, d1 };
  return (f64x2) vqtbl1q_u8(tab, idx);
}

#[inline]
fn vsel_vd_vo_vo_vo_d_d_d_d(o0: $ox, o1: $ox, o2: $ox, d0: f64, d1: f64, d2: f64, d3: f64) -> f64x2 {
  uint8x16_t idx = vbslq_u8(vreinterpretq_u8_u32(o0), (uint8x16_t) { 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7 },
			    vbslq_u8(vreinterpretq_u8_u32(o1), (uint8x16_t) { 8, 9, 10, 11, 12, 13, 14, 15, 8, 9, 10, 11, 12, 13, 14, 15 },
				     vbslq_u8(vreinterpretq_u8_u32(o2), (uint8x16_t) { 16, 17, 18, 19, 20, 21, 22, 23, 16, 17, 18, 19, 20, 21, 22, 23 },
					      (uint8x16_t) { 24, 25, 26, 27, 28, 29, 30, 31, 24, 25, 26, 27, 28, 29, 30, 31 })));
  
  uint8x16x2_t tab = { { (uint8x16_t) (float64x2_t) { d0, d1 }, (uint8x16_t) (float64x2_t) { d2, d3 } } }; 
  return (f64x2) vqtbl2q_u8(tab, idx);
}

#[inline]
fn vsel_vd_vo_vo_d_d_d(o0: $ox, o1: $ox, d0: f64, d1: f64, d2: f64) -> f64x2 {
  return vsel_vd_vo_vo_vo_d_d_d_d(o0, o1, o1, d0, d1, d2, d2);
}
#endif
*/

impl RInt for f64x2 {
    #[inline]
    fn rint(self) -> Self {
        vrndnq_f64(d)
    }
}
impl RInt for f32x4 {
    #[inline]
    fn rint(self) -> Self {
      vrndnq_f32(d)
    }
}

/****************************************/
/* int <--> float conversions           */
/****************************************/
#[inline]
fn vtruncate_vi_vd(vf: f64x2) -> i32x2 {
  return vmovn_s64(vcvtq_s64_f64(vf));
}

#[inline]
fn vrint_vi_vd(d: f64x2) -> i32x2 {
  return vqmovn_s64(vcvtq_s64_f64(vrndnq_f64(d)));
}

/***************************************/
/* Integer operations */
/***************************************/

// Add, Sub, Neg (-x)

// Logical operations
#[inline]
fn vandnot_vi_vi_vi(x: i32x2, y: i32x2) -> i32x2 { return vbic_s32(y, x); }

// Comparison returning masks


// Conditional select
#[inline]
fn vsel_vi_vm_vi_vi($ux m, i32x2 x, i32x2 y) -> i32x2 {
  return vbsl_s32(vget_low_u32(m), x, y);
}

/***************************************/
/* Predicates                          */
/***************************************/
/*
#[inline]
fn vsel_vf_vo_f_f(o: $ox, v1: f32, v0: f32) -> CONST -> f32x4 {
  o.select(f32x4::splat(v1), f32x4::splat(v0));
}

#[inline]
fn vsel_vf_vo_vo_f_f_f(o0: $ox, o1: $ox, d0: f32, d1: f32, d2: f32) -> f32x4 {
  o0.select(f32x4::splat(d0), vsel_vf_vo_f_f(o1, d1, d2));
}

#[inline]
fn vsel_vf_vo_vo_vo_f_f_f_f(o0: $ox, o1: $ox, o2: $ox, d0: f32, d1: f32, d2: f32, d3: f32) -> f32x4 {
  o0.select(f32x4::splat(d0), o1.select(f32x4::splat(d1), vsel_vf_vo_f_f(o2, d2, d3)))
}
*/

#[inline]
fn vandnot_vo_vo_vo(x: $ox, y: $ox) -> $ox {
  return vbicq_u32(y, x);
}

#[inline]
fn vand_vi2_vo_vi2(x: $ox, y: i32x4) -> i32x4 {
  return vandq_s32(vreinterpretq_s32_u32(x), y);
}
#[inline]
fn vandnot_vi_vo_vi(x: $ox, y: i32x2) -> i32x2 {
  return vbic_s32(y, vget_low_s32(vreinterpretq_s32_u32(x)));
}
#[inline]
fn vand_vm_vo32_vm(x: $ox, y: $ux) -> $ux {
  return vandq_u32(x, y);
}
#[inline]
fn vand_vm_vo64_vm(x: $ox, y: $ux) -> $ux {
  return vandq_u32(x, y);
}
#[inline]
fn vandnot_vm_vo32_vm(x: $ox, y: $ux) -> $ux {
  return vbicq_u32(y, x);
}
#[inline]
fn vandnot_vm_vo64_vm(x: $ox, y: $ux) -> $ux {
  return vbicq_u32(y, x);
}
#[inline]
fn vor_vm_vo32_vm(x: $ox, y: $ux) -> $ux {
  return vorrq_u32(x, y);
}
#[inline]
fn vor_vm_vo64_vm(x: $ox, y: $ux) -> $ux {
  return vorrq_u32(x, y);
}

impl Truncate for f32x4 {
    #[inline]
    fn truncate(self) -> Self {
        vrndq_f32(vd)
    }
}

#[inline]
fn vcast_vm_i_i(i0: int, i1: int) -> $ux {
  return vreinterpretq_u32_u64(vdupq_n_u64((0xffffffff & (i1 as u64)) | (((i0 as u64)) << 32)));
}

// Logical operations
#[inline]
fn vand_vi_vo_vi(x: $ox, y: i32x2) -> i32x2 {
  return vand_s32(vreinterpret_s32_u32(vget_low_u32(x)), y);
}


impl Truncate for $f64x {
    #[inline]
    fn truncate(self) -> Self {
        vrndq_f64(vd)
    }
}

//

#[inline]
fn vrev21_vf_vf(d0: f32x4) -> f32x4 { return vrev64q_f32(d0); }
#[inline]
fn vrev21_vi2_vi2(i: i32x4) -> i32x4 { return i32x4::from(vrev21_vf_vf(f32x4::from(i))); }
