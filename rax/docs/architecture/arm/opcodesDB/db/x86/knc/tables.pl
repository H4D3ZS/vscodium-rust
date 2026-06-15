use strict;
use warnings;

DEF n_i32 => { dt => 'I32', encodedin => 'SSS', swizzle => 'N_I32', table => 'n_i32_tbl' };
DEF n_f32 => { dt => 'F32', encodedin => 'SSS', swizzle => 'N_F32', table => 'n_f32_tbl' };
DEF n_i64 => { dt => 'I64', encodedin => 'SSS', swizzle => 'N_I64', table => 'n_i64_tbl' };
DEF su_f64 => { dt => 'F64', encodedin => 'SSS', swizzle => 'SU_F64', table => 'su_f64_tbl' };
DEF u_f64 => { dt => 'F64', encodedin => 'SSS', swizzle => 'U_F64', table => 'u_f64_tbl' };
DEF su_f32 => { dt => 'F32', encodedin => 'SSS', swizzle => 'SU_F32', table => 'su_f32_tbl' };
DEF su_mf32 => { dt => 'F32', encodedin => 'SSS', swizzle => 'SU_F32', table => 'su_mf32_tbl' };
DEF su_hf32 => { dt => 'F32', encodedin => 'SSS', swizzle => 'SU_F32', table => 'su_hf32_tbl' };
DEF u_f32 => { dt => 'F32', encodedin => 'SSS', swizzle => 'U_F32', table => 'u_f32_tbl' };
DEF su_i64 => { dt => 'I64', encodedin => 'SSS', swizzle => 'SU_I64', table => 'su_i64_tbl' };
DEF u_i64 => { dt => 'I64', encodedin => 'SSS', swizzle => 'U_I64', table => 'u_i64_tbl' };
DEF su_i32 => { dt => 'I32', encodedin => 'SSS', swizzle => 'SU_I32', table => 'su_i32_tbl' };
DEF su_mi32 => { dt => 'I32', encodedin => 'SSS', swizzle => 'SU_I32', table => 'su_mi32_tbl' };
DEF su_hi32 => { dt => 'I32', encodedin => 'SSS', swizzle => 'SU_I32', table => 'su_hi32_tbl' };
DEF u_i32 => { dt => 'I32', encodedin => 'SSS', swizzle => 'U_I32', table => 'u_i32_tbl' };
DEF d_f32 => { dt => 'F32', encodedin => 'SSS', swizzle => 'D_F32', table => 'd_f32_tbl' };
DEF d_i32 => { dt => 'I32', encodedin => 'SSS', swizzle => 'D_I32', table => 'd_i32_tbl' };
DEF d_i64 => { dt => 'I64', encodedin => 'SSS', swizzle => 'D_I64', table => 'd_i64_tbl' };
DEF d_f64 => { dt => 'F64', encodedin => 'SSS', swizzle => 'D_F64', table => 'd_f64_tbl' };

DEF r_i32 => { dt => 'I32', encodedin => 'SSS', swizzle => 'R_I32' };
DEF r_i64 => { dt => 'I64', encodedin => 'SSS', swizzle => 'R_I64' };


SWIZZLE TABLE n_i32_tbl => [
  { mdt => "U32", nelem => 16, vdt => "U32" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE n_f32_tbl => [
  { mdt => "F32", nelem => 16, vdt => "F32" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE n_i64_tbl => [
  { mdt => "U64", nelem => 8, vdt => "U64" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE su_f64_tbl => [
  { mdt => "F64", nelem => 8, vdt => "F64" },
  { broadcasting => 1, mdt => "F64", nelem => 1, vdt => "F64" },
  { broadcasting => 1, mdt => "F64", nelem => 4, vdt => "F64" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE u_f64_tbl => [
  { mdt => "F64", nelem => 8, vdt => "F64" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE su_f32_tbl => [
  { mdt => "F32", nelem => 16, vdt => "F32" },
  { broadcasting => 1, mdt => "F32", nelem => 1, vdt => "F32" },
  { broadcasting => 1, mdt => "F32", nelem => 4, vdt => "F32" },
  { converting => 1, mdt => "F16", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "U8", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "S8", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "U16", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "S16", nelem => 16, vdt => "F32" },
];

SWIZZLE TABLE su_mf32_tbl => [
  { mdt => "F32", nelem => 16, vdt => "F32" },
  { reserved => 1 },
  { broadcasting => 1, mdt => "F32", nelem => 4, vdt => "F32" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE su_hf32_tbl => [
  { mdt => "F32", nelem => 8, vdt => "F32" },
  { broadcasting => 1, mdt => "F32", nelem => 1, vdt => "F32" },
  { broadcasting => 1, mdt => "F32", nelem => 4, vdt => "F32" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE u_f32_tbl => [
  { mdt => "F32", nelem => 16, vdt => "F32" },
  { reserved => 1 },
  { reserved => 1 },
  { converting => 1, mdt => "F16", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "U8", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "S8", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "U16", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "S16", nelem => 16, vdt => "F32" },
];

SWIZZLE TABLE su_i64_tbl => [
  { mdt => "S64", nelem => 8, vdt => "S64" },
  { broadcasting => 1, mdt => "S64", nelem => 1, vdt => "S64" },
  { broadcasting => 1, mdt => "S64", nelem => 4, vdt => "S64" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE u_i64_tbl => [
  { mdt => "S64", nelem => 8, vdt => "S64" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE su_i32_tbl => [
  { mdt => "S32", nelem => 16, vdt => "S32" },
  { broadcasting => 1, mdt => "S32", nelem => 1, vdt => "S32" },
  { broadcasting => 1, mdt => "S32", nelem => 4, vdt => "S32" },
  { reserved => 1 },
  { converting => 1, mdt => "U8", nelem => 16, vdt => "U32" },
  { converting => 1, mdt => "S8", nelem => 16, vdt => "S32" },
  { converting => 1, mdt => "U16", nelem => 16, vdt => "U32" },
  { converting => 1, mdt => "S16", nelem => 16, vdt => "S32" },
];

SWIZZLE TABLE su_mi32_tbl => [
  { mdt => "S32", nelem => 16, vdt => "S32" },
  { reserved => 1 },
  { broadcasting => 1, mdt => "S32", nelem => 4, vdt => "S32" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE su_hi32_tbl => [
  { mdt => "S32", nelem => 8, vdt => "S32" },
  { broadcasting => 1, mdt => "S32", nelem => 1, vdt => "S32" },
  { broadcasting => 1, mdt => "S32", nelem => 4, vdt => "S32" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE u_i32_tbl => [
  { mdt => "S32", nelem => 16, vdt => "S32" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { converting => 1, mdt => "U8", nelem => 16, vdt => "U32" },
  { converting => 1, mdt => "S8", nelem => 16, vdt => "S32" },
  { converting => 1, mdt => "U16", nelem => 16, vdt => "U32" },
  { converting => 1, mdt => "S16", nelem => 16, vdt => "S32" },
];

SWIZZLE TABLE d_f32_tbl => [
  { mdt => "F32", nelem => 16, vdt => "F32" },
  { reserved => 1 },
  { reserved => 1 },
  { converting => 1, mdt => "F16", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "U8", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "S8", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "U16", nelem => 16, vdt => "F32" },
  { converting => 1, mdt => "S16", nelem => 16, vdt => "F32" },
];

SWIZZLE TABLE d_i32_tbl => [
  { mdt => "S32", nelem => 16, vdt => "S32" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { converting => 1, mdt => "U8", nelem => 16, vdt => "U32" },
  { converting => 1, mdt => "S8", nelem => 16, vdt => "S32" },
  { converting => 1, mdt => "U16", nelem => 16, vdt => "U32" },
  { converting => 1, mdt => "S16", nelem => 16, vdt => "S32" },
];

SWIZZLE TABLE d_i64_tbl => [
  { mdt => "S64", nelem => 8, vdt => "S64" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

SWIZZLE TABLE d_f64_tbl => [
  { mdt => "F64", nelem => 8, vdt => "F64" },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
  { reserved => 1 },
];

