use ::libc;
use super::openjpeg::*;
use super::thread::*;
use super::mqc::*;
use super::math::*;
use super::t1_luts::*;

extern "C" {

  fn abs(_: libc::c_int) -> libc::c_int;

  fn lrintf(_: libc::c_float) -> libc::c_long;

  fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

  fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

  fn opj_calloc(numOfElements: size_t, sizeOfElements: size_t) -> *mut libc::c_void;

  fn opj_aligned_malloc(size: size_t) -> *mut libc::c_void;

  fn opj_aligned_free(ptr: *mut libc::c_void);

  fn opj_realloc(m: *mut libc::c_void, s: size_t) -> *mut libc::c_void;

  fn opj_free(m: *mut libc::c_void);

  fn opj_event_msg(
    event_mgr: *mut opj_event_mgr_t,
    event_type: OPJ_INT32,
    fmt: *const libc::c_char,
    _: ...
  ) -> OPJ_BOOL;

  fn opj_mutex_create() -> *mut opj_mutex_t;

  fn opj_mutex_lock(mutex: *mut opj_mutex_t);

  fn opj_mutex_unlock(mutex: *mut opj_mutex_t);

  fn opj_mutex_destroy(mutex: *mut opj_mutex_t);

  fn opj_tls_get(tls: *mut opj_tls_t, key: libc::c_int) -> *mut libc::c_void;

  fn opj_tls_set(
    tls: *mut opj_tls_t,
    key: libc::c_int,
    value: *mut libc::c_void,
    free_func: opj_tls_free_func,
  ) -> OPJ_BOOL;

  fn opj_thread_pool_submit_job(
    tp: *mut opj_thread_pool_t,
    job_fn: opj_job_fn,
    user_data: *mut libc::c_void,
  ) -> OPJ_BOOL;

  fn opj_thread_pool_wait_completion(tp: *mut opj_thread_pool_t, max_remaining_jobs: libc::c_int);

  fn opj_thread_pool_get_thread_count(tp: *mut opj_thread_pool_t) -> libc::c_int;

  fn opj_mqc_byteout(mqc: *mut opj_mqc_t);

  fn opj_mqc_numbytes(mqc: *mut opj_mqc_t) -> OPJ_UINT32;

  fn opj_mqc_resetstates(mqc: *mut opj_mqc_t);

  fn opj_mqc_setstate(mqc: *mut opj_mqc_t, ctxno: OPJ_UINT32, msb: OPJ_UINT32, prob: OPJ_INT32);

  fn opj_mqc_init_enc(mqc: *mut opj_mqc_t, bp: *mut OPJ_BYTE);

  fn opj_mqc_flush(mqc: *mut opj_mqc_t);

  fn opj_mqc_bypass_init_enc(mqc: *mut opj_mqc_t);

  fn opj_mqc_bypass_get_extra_bytes(mqc: *mut opj_mqc_t, erterm: OPJ_BOOL) -> OPJ_UINT32;

  fn opj_mqc_bypass_flush_enc(mqc: *mut opj_mqc_t, erterm: OPJ_BOOL);

  fn opj_mqc_reset_enc(mqc: *mut opj_mqc_t);

  fn opj_mqc_restart_init_enc(mqc: *mut opj_mqc_t);

  fn opj_mqc_erterm_enc(mqc: *mut opj_mqc_t);

  fn opj_mqc_segmark_enc(mqc: *mut opj_mqc_t);

  fn opj_mqc_init_dec(
    mqc: *mut opj_mqc_t,
    bp: *mut OPJ_BYTE,
    len: OPJ_UINT32,
    extra_writable_bytes: OPJ_UINT32,
  );

  fn opj_mqc_raw_init_dec(
    mqc: *mut opj_mqc_t,
    bp: *mut OPJ_BYTE,
    len: OPJ_UINT32,
    extra_writable_bytes: OPJ_UINT32,
  );

  fn opq_mqc_finish_dec(mqc: *mut opj_mqc_t);

  fn opj_tcd_is_band_empty(band: *mut opj_tcd_band_t) -> OPJ_BOOL;

  fn opj_tcd_is_subband_area_of_interest(
    tcd: *mut opj_tcd_t,
    compno: OPJ_UINT32,
    resno: OPJ_UINT32,
    bandno: OPJ_UINT32,
    x0: OPJ_UINT32,
    y0: OPJ_UINT32,
    x1: OPJ_UINT32,
    y1: OPJ_UINT32,
  ) -> OPJ_BOOL;

  fn opj_dwt_getnorm_real(level: OPJ_UINT32, orient: OPJ_UINT32) -> OPJ_FLOAT64;

  fn opj_dwt_getnorm(level: OPJ_UINT32, orient: OPJ_UINT32) -> OPJ_FLOAT64;
  /* *
  Decode 1 HT code-block
  @param t1 T1 handle
  @param cblk Code-block coding parameters
  @param orient
  @param roishift Region of interest shifting value
  @param cblksty Code-block style
  @param p_manager the event manager
  @param p_manager_mutex mutex for the event manager
  @param check_pterm whether PTERM correct termination should be checked
  */

  fn opj_t1_ht_decode_cblk(
    t1: *mut opj_t1_t,
    cblk: *mut opj_tcd_cblk_dec_t,
    orient: OPJ_UINT32,
    roishift: OPJ_UINT32,
    cblksty: OPJ_UINT32,
    p_manager: *mut opj_event_mgr_t,
    p_manager_mutex: *mut opj_mutex_t,
    check_pterm: OPJ_BOOL,
  ) -> OPJ_BOOL;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opj_t1 {
  pub mqc: opj_mqc_t,
  pub data: *mut OPJ_INT32,
  pub flags: *mut opj_flag_t,
  pub w: OPJ_UINT32,
  pub h: OPJ_UINT32,
  pub datasize: OPJ_UINT32,
  pub flagssize: OPJ_UINT32,
  pub encoder: OPJ_BOOL,
  pub mustuse_cblkdatabuffer: OPJ_BOOL,
  pub cblkdatabuffer: *mut OPJ_BYTE,
  pub cblkdatabuffersize: OPJ_UINT32,
}
pub type opj_t1_t = opj_t1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opj_t1_cblk_encode_processing_job_t {
  pub compno: OPJ_UINT32,
  pub resno: OPJ_UINT32,
  pub cblk: *mut opj_tcd_cblk_enc_t,
  pub tile: *mut opj_tcd_tile_t,
  pub band: *mut opj_tcd_band_t,
  pub tilec: *mut opj_tcd_tilecomp_t,
  pub tccp: *mut opj_tccp_t,
  pub mct_norms: *const OPJ_FLOAT64,
  pub mct_numcomps: OPJ_UINT32,
  pub pret: *mut OPJ_BOOL,
  pub mutex: *mut opj_mutex_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opj_t1_cblk_decode_processing_job_t {
  pub whole_tile_decoding: OPJ_BOOL,
  pub resno: OPJ_UINT32,
  pub cblk: *mut opj_tcd_cblk_dec_t,
  pub band: *mut opj_tcd_band_t,
  pub tilec: *mut opj_tcd_tilecomp_t,
  pub tccp: *mut opj_tccp_t,
  pub mustuse_cblkdatabuffer: OPJ_BOOL,
  pub pret: *mut OPJ_BOOL,
  pub p_manager: *mut opj_event_mgr_t,
  pub p_manager_mutex: *mut opj_mutex_t,
  pub check_pterm: OPJ_BOOL,
}

#[inline]
unsafe extern "C" fn opj_mqc_raw_decode(mut mqc: *mut opj_mqc_t) -> OPJ_UINT32 {
  let mut d: OPJ_UINT32 = 0;
  if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
    if (*mqc).c == 0xff as libc::c_int as libc::c_uint {
      if *(*mqc).bp as libc::c_int > 0x8f as libc::c_int {
        (*mqc).c = 0xff as libc::c_int as OPJ_UINT32;
        (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
      } else {
        (*mqc).c = *(*mqc).bp as OPJ_UINT32;
        (*mqc).bp = (*mqc).bp.offset(1);
        (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
      }
    } else {
      (*mqc).c = *(*mqc).bp as OPJ_UINT32;
      (*mqc).bp = (*mqc).bp.offset(1);
      (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
    }
  }
  (*mqc).ct = (*mqc).ct.wrapping_sub(1);
  d = (*mqc).c >> (*mqc).ct & 0x1 as libc::c_uint;
  return d;
}

/* * @name Local static functions */
/*@{*/
/*@}*/
/*@}*/
/* ----------------------------------------------------------------------- */
#[inline]
unsafe extern "C" fn opj_t1_getctxno_zc(mut mqc: *mut opj_mqc_t, mut f: OPJ_UINT32) -> OPJ_BYTE {
  return *(*mqc).lut_ctxno_zc_orient.offset(
    (f & ((1 as libc::c_uint) << 0 as libc::c_int
      | (1 as libc::c_uint) << 1 as libc::c_int
      | (1 as libc::c_uint) << 2 as libc::c_int
      | (1 as libc::c_uint) << 3 as libc::c_int
      | (1 as libc::c_uint) << 5 as libc::c_int
      | (1 as libc::c_uint) << 6 as libc::c_int
      | (1 as libc::c_uint) << 7 as libc::c_int
      | (1 as libc::c_uint) << 8 as libc::c_int)) as isize,
  );
}
#[inline]
unsafe extern "C" fn opj_t1_getctxtno_sc_or_spb_index(
  mut fX: OPJ_UINT32,
  mut pfX: OPJ_UINT32,
  mut nfX: OPJ_UINT32,
  mut ci: OPJ_UINT32,
) -> OPJ_UINT32 {
  /*
    0 pfX T1_CHI_THIS           T1_LUT_SGN_W
    1 tfX T1_SIGMA_1            T1_LUT_SIG_N
    2 nfX T1_CHI_THIS           T1_LUT_SGN_E
    3 tfX T1_SIGMA_3            T1_LUT_SIG_W
    4  fX T1_CHI_(THIS - 1)     T1_LUT_SGN_N
    5 tfX T1_SIGMA_5            T1_LUT_SIG_E
    6  fX T1_CHI_(THIS + 1)     T1_LUT_SGN_S
    7 tfX T1_SIGMA_7            T1_LUT_SIG_S
  */
  let mut lu = fX >> ci.wrapping_mul(3 as libc::c_uint)
    & ((1 as libc::c_uint) << 1 as libc::c_int
      | (1 as libc::c_uint) << 3 as libc::c_int
      | (1 as libc::c_uint) << 5 as libc::c_int
      | (1 as libc::c_uint) << 7 as libc::c_int);
  lu |= pfX >> (19 as libc::c_int as libc::c_uint).wrapping_add(ci.wrapping_mul(3 as libc::c_uint))
    & (1 as libc::c_uint) << 0 as libc::c_int;
  lu |= nfX
    >> (19 as libc::c_int as libc::c_uint)
      .wrapping_sub(2 as libc::c_uint)
      .wrapping_add(ci.wrapping_mul(3 as libc::c_uint))
    & (1 as libc::c_uint) << 2 as libc::c_int;
  if ci == 0 as libc::c_uint {
    lu |= fX >> (18 as libc::c_int as libc::c_uint).wrapping_sub(4 as libc::c_uint)
      & (1 as libc::c_uint) << 4 as libc::c_int
  } else {
    lu |= fX
      >> (19 as libc::c_int as libc::c_uint)
        .wrapping_sub(4 as libc::c_uint)
        .wrapping_add(
          ci.wrapping_sub(1 as libc::c_uint)
            .wrapping_mul(3 as libc::c_uint),
        )
      & (1 as libc::c_uint) << 4 as libc::c_int
  }
  lu |= fX
    >> (22 as libc::c_int as libc::c_uint)
      .wrapping_sub(6 as libc::c_uint)
      .wrapping_add(ci.wrapping_mul(3 as libc::c_uint))
    & (1 as libc::c_uint) << 6 as libc::c_int;
  return lu;
}
#[inline]
unsafe extern "C" fn opj_t1_getctxno_sc(mut lu: OPJ_UINT32) -> OPJ_BYTE {
  return lut_ctxno_sc[lu as usize];
}
#[inline]
unsafe extern "C" fn opj_t1_getctxno_mag(mut f: OPJ_UINT32) -> OPJ_UINT32 {
  let mut tmp = if f
    & ((1 as libc::c_uint) << 0 as libc::c_int
      | (1 as libc::c_uint) << 1 as libc::c_int
      | (1 as libc::c_uint) << 2 as libc::c_int
      | (1 as libc::c_uint) << 3 as libc::c_int
      | (1 as libc::c_uint) << 5 as libc::c_int
      | (1 as libc::c_uint) << 6 as libc::c_int
      | (1 as libc::c_uint) << 7 as libc::c_int
      | (1 as libc::c_uint) << 8 as libc::c_int)
    != 0
  {
    (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int) + 1 as libc::c_int
  } else {
    (0 as libc::c_int + 9 as libc::c_int) + 5 as libc::c_int
  } as OPJ_UINT32;
  let mut tmp2 = if f & (1 as libc::c_uint) << 20 as libc::c_int != 0 {
    (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 2 as libc::c_int) as libc::c_uint
  } else {
    tmp
  };
  return tmp2;
}
#[inline]
unsafe extern "C" fn opj_t1_getspb(mut lu: OPJ_UINT32) -> OPJ_BYTE {
  return lut_spb[lu as usize];
}
unsafe extern "C" fn opj_t1_getnmsedec_sig(mut x: OPJ_UINT32, mut bitpos: OPJ_UINT32) -> OPJ_INT16 {
  if bitpos > 0 as libc::c_int as libc::c_uint {
    return lut_nmsedec_sig[(x >> bitpos
      & (((1 as libc::c_int) << 7 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
      as usize];
  }
  return lut_nmsedec_sig0
    [(x & (((1 as libc::c_int) << 7 as libc::c_int) - 1 as libc::c_int) as libc::c_uint) as usize];
}
unsafe extern "C" fn opj_t1_getnmsedec_ref(mut x: OPJ_UINT32, mut bitpos: OPJ_UINT32) -> OPJ_INT16 {
  if bitpos > 0 as libc::c_int as libc::c_uint {
    return lut_nmsedec_ref[(x >> bitpos
      & (((1 as libc::c_int) << 7 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
      as usize];
  }
  return lut_nmsedec_ref0
    [(x & (((1 as libc::c_int) << 7 as libc::c_int) - 1 as libc::c_int) as libc::c_uint) as usize];
}
/* east */
/* mark target as significant */
/* west */
/* north-west, north, north-east */
/* south-west, south, south-east */
#[inline]
unsafe extern "C" fn opj_t1_update_flags(
  mut flagsp: *mut opj_flag_t,
  mut ci: OPJ_UINT32,
  mut s: OPJ_UINT32,
  mut stride: OPJ_UINT32,
  mut vsc: OPJ_UINT32,
) {
  let ref mut fresh0 = *flagsp.offset(-(1 as libc::c_int) as isize);
  *fresh0 |= ((1 as libc::c_uint) << 5 as libc::c_int) << (3 as libc::c_uint).wrapping_mul(ci);
  *flagsp |= (s << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
    << (3 as libc::c_uint).wrapping_mul(ci);
  let ref mut fresh1 = *flagsp.offset(1 as libc::c_int as isize);
  *fresh1 |= ((1 as libc::c_uint) << 3 as libc::c_int) << (3 as libc::c_uint).wrapping_mul(ci);
  if ci == 0 as libc::c_uint && vsc == 0 {
    let mut north = flagsp.offset(-(stride as isize));
    *north |= s << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
    let ref mut fresh2 = *north.offset(-(1 as libc::c_int) as isize);
    *fresh2 |= (1 as libc::c_uint) << 17 as libc::c_int;
    let ref mut fresh3 = *north.offset(1 as libc::c_int as isize);
    *fresh3 |= (1 as libc::c_uint) << 15 as libc::c_int
  }
  if ci == 3 as libc::c_uint {
    let mut south = flagsp.offset(stride as isize);
    *south |= s << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
    let ref mut fresh4 = *south.offset(-(1 as libc::c_int) as isize);
    *fresh4 |= (1 as libc::c_uint) << 2 as libc::c_int;
    let ref mut fresh5 = *south.offset(1 as libc::c_int as isize);
    *fresh5 |= (1 as libc::c_uint) << 0 as libc::c_int
  };
}
/* *
Decode significant pass
*/
/* *
Encode significant pass
*/
/* BYPASS/LAZY MODE */
/* BYPASS/LAZY MODE */
#[inline]
unsafe extern "C" fn opj_t1_dec_sigpass_step_raw(
  mut t1: *mut opj_t1_t,
  mut flagsp: *mut opj_flag_t,
  mut datap: *mut OPJ_INT32,
  mut oneplushalf: OPJ_INT32,
  mut vsc: OPJ_UINT32,
  mut ci: OPJ_UINT32,
) {
  let mut v: OPJ_UINT32 = 0; /* RAW component */
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc; /* MQC component */
  let flags = *flagsp;
  if flags
    & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
      << ci.wrapping_mul(3 as libc::c_uint)
    == 0 as libc::c_uint
    && flags
      & ((1 as libc::c_uint) << 0 as libc::c_int
        | (1 as libc::c_uint) << 1 as libc::c_int
        | (1 as libc::c_uint) << 2 as libc::c_int
        | (1 as libc::c_uint) << 3 as libc::c_int
        | (1 as libc::c_uint) << 5 as libc::c_int
        | (1 as libc::c_uint) << 6 as libc::c_int
        | (1 as libc::c_uint) << 7 as libc::c_int
        | (1 as libc::c_uint) << 8 as libc::c_int)
        << ci.wrapping_mul(3 as libc::c_uint)
      != 0 as libc::c_uint
  {
    if opj_mqc_raw_decode(mqc) != 0 {
      v = opj_mqc_raw_decode(mqc);
      *datap = if v != 0 { -oneplushalf } else { oneplushalf };
      opj_t1_update_flags(
        flagsp,
        ci,
        v,
        (*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint),
        vsc,
      );
    }
    *flagsp |= ((1 as libc::c_uint) << 21 as libc::c_int) << ci.wrapping_mul(3 as libc::c_uint)
  };
}
#[inline]
unsafe extern "C" fn opj_t1_dec_sigpass_step_mqc(
  mut t1: *mut opj_t1_t,
  mut flagsp: *mut opj_flag_t,
  mut datap: *mut OPJ_INT32,
  mut oneplushalf: OPJ_INT32,
  mut ci: OPJ_UINT32,
  mut flags_stride: OPJ_UINT32,
  mut vsc: OPJ_UINT32,
) {
  let mut v: OPJ_UINT32 = 0;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  if *flagsp
    & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
      << ci.wrapping_mul(3 as libc::c_uint)
    == 0 as libc::c_uint
    && *flagsp
      & ((1 as libc::c_uint) << 0 as libc::c_int
        | (1 as libc::c_uint) << 1 as libc::c_int
        | (1 as libc::c_uint) << 2 as libc::c_int
        | (1 as libc::c_uint) << 3 as libc::c_int
        | (1 as libc::c_uint) << 5 as libc::c_int
        | (1 as libc::c_uint) << 6 as libc::c_int
        | (1 as libc::c_uint) << 7 as libc::c_int
        | (1 as libc::c_uint) << 8 as libc::c_int)
        << ci.wrapping_mul(3 as libc::c_uint)
      != 0 as libc::c_uint
  {
    let mut ctxt1 =
      opj_t1_getctxno_zc(mqc, *flagsp >> ci.wrapping_mul(3 as libc::c_uint)) as OPJ_UINT32;
    (*mqc).curctx =
      &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize) as *mut *const opj_mqc_state_t;
    (*mqc).a =
      ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
    if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
      if (*mqc).a < (**(*mqc).curctx).qeval {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v = (**(*mqc).curctx).mps;
        *(*mqc).curctx = (**(*mqc).curctx).nmps
      } else {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
        *(*mqc).curctx = (**(*mqc).curctx).nlps
      }
      loop {
        if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
          let mut l_c: OPJ_UINT32 = 0;
          l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
            if l_c > 0x8f as libc::c_int as libc::c_uint {
              (*mqc).c = ((*mqc).c as libc::c_uint)
                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
              (*mqc).end_of_byte_stream_counter = (*mqc).end_of_byte_stream_counter.wrapping_add(1)
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
            }
          } else {
            (*mqc).bp = (*mqc).bp.offset(1);
            (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
          }
        }
        (*mqc).a <<= 1 as libc::c_int;
        (*mqc).c <<= 1 as libc::c_int;
        (*mqc).ct = (*mqc).ct.wrapping_sub(1);
        if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
          break;
        }
      }
    } else {
      (*mqc).c = ((*mqc).c as libc::c_uint)
        .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
        as OPJ_UINT32;
      if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        if (*mqc).a < (**(*mqc).curctx).qeval {
          v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
          *(*mqc).curctx = (**(*mqc).curctx).nlps
        } else {
          v = (**(*mqc).curctx).mps;
          *(*mqc).curctx = (**(*mqc).curctx).nmps
        }
        loop {
          if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
            let mut l_c_0: OPJ_UINT32 = 0;
            l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
              if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                (*mqc).c = ((*mqc).c as libc::c_uint)
                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                (*mqc).end_of_byte_stream_counter =
                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
              }
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
            }
          }
          (*mqc).a <<= 1 as libc::c_int;
          (*mqc).c <<= 1 as libc::c_int;
          (*mqc).ct = (*mqc).ct.wrapping_sub(1);
          if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
            break;
          }
        }
      } else {
        v = (**(*mqc).curctx).mps
      }
    }
    if v != 0 {
      let mut lu = opj_t1_getctxtno_sc_or_spb_index(
        *flagsp,
        *flagsp.offset(-(1 as libc::c_int) as isize),
        *flagsp.offset(1 as libc::c_int as isize),
        ci,
      );
      let mut ctxt2 = opj_t1_getctxno_sc(lu) as OPJ_UINT32;
      let mut spb = opj_t1_getspb(lu) as OPJ_UINT32;
      (*mqc).curctx =
        &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2 as isize) as *mut *const opj_mqc_state_t;
      (*mqc).a = ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32
        as OPJ_UINT32;
      if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
        if (*mqc).a < (**(*mqc).curctx).qeval {
          (*mqc).a = (**(*mqc).curctx).qeval;
          v = (**(*mqc).curctx).mps;
          *(*mqc).curctx = (**(*mqc).curctx).nmps
        } else {
          (*mqc).a = (**(*mqc).curctx).qeval;
          v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
          *(*mqc).curctx = (**(*mqc).curctx).nlps
        }
        loop {
          if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
            let mut l_c_1: OPJ_UINT32 = 0;
            l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
              if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                (*mqc).c = ((*mqc).c as libc::c_uint)
                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                (*mqc).end_of_byte_stream_counter =
                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
              }
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
            }
          }
          (*mqc).a <<= 1 as libc::c_int;
          (*mqc).c <<= 1 as libc::c_int;
          (*mqc).ct = (*mqc).ct.wrapping_sub(1);
          if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
            break;
          }
        }
      } else {
        (*mqc).c = ((*mqc).c as libc::c_uint)
          .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int)
          as OPJ_UINT32 as OPJ_UINT32;
        if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
          if (*mqc).a < (**(*mqc).curctx).qeval {
            v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
            *(*mqc).curctx = (**(*mqc).curctx).nlps
          } else {
            v = (**(*mqc).curctx).mps;
            *(*mqc).curctx = (**(*mqc).curctx).nmps
          }
          loop {
            if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
              let mut l_c_2: OPJ_UINT32 = 0;
              l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                  (*mqc).c = ((*mqc).c as libc::c_uint)
                    .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                    as OPJ_UINT32 as OPJ_UINT32;
                  (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                  (*mqc).end_of_byte_stream_counter =
                    (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int)
                    as OPJ_UINT32 as OPJ_UINT32;
                  (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
                }
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
              }
            }
            (*mqc).a <<= 1 as libc::c_int;
            (*mqc).c <<= 1 as libc::c_int;
            (*mqc).ct = (*mqc).ct.wrapping_sub(1);
            if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
              break;
            }
          }
        } else {
          v = (**(*mqc).curctx).mps
        }
      }
      v = v ^ spb;
      *datap.offset(ci.wrapping_mul(0 as libc::c_int as libc::c_uint) as isize) =
        if v != 0 { -oneplushalf } else { oneplushalf };
      let ref mut fresh6 = *flagsp.offset(-(1 as libc::c_int) as isize);
      *fresh6 |= ((1 as libc::c_uint) << 5 as libc::c_int) << (3 as libc::c_uint).wrapping_mul(ci);
      *flagsp |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
        << (3 as libc::c_uint).wrapping_mul(ci);
      let ref mut fresh7 = *flagsp.offset(1 as libc::c_int as isize);
      *fresh7 |= ((1 as libc::c_uint) << 3 as libc::c_int) << (3 as libc::c_uint).wrapping_mul(ci);
      if ci == 0 as libc::c_uint && vsc == 0 {
        let mut north = flagsp.offset(-(flags_stride as isize));
        *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
        let ref mut fresh8 = *north.offset(-(1 as libc::c_int) as isize);
        *fresh8 |= (1 as libc::c_uint) << 17 as libc::c_int;
        let ref mut fresh9 = *north.offset(1 as libc::c_int as isize);
        *fresh9 |= (1 as libc::c_uint) << 15 as libc::c_int
      }
      if ci == 3 as libc::c_uint {
        let mut south = flagsp.offset(flags_stride as isize);
        *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
        let ref mut fresh10 = *south.offset(-(1 as libc::c_int) as isize);
        *fresh10 |= (1 as libc::c_uint) << 2 as libc::c_int;
        let ref mut fresh11 = *south.offset(1 as libc::c_int as isize);
        *fresh11 |= (1 as libc::c_uint) << 0 as libc::c_int
      }
    }
    *flagsp |= ((1 as libc::c_uint) << 21 as libc::c_int) << ci.wrapping_mul(3 as libc::c_uint)
  };
}
/* *
Encode significant pass
*/
unsafe extern "C" fn opj_t1_enc_sigpass(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
  mut nmsedec: *mut OPJ_INT32,
  mut type_0: OPJ_BYTE,
  mut cblksty: OPJ_UINT32,
) {
  let mut i: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let one = (1 as libc::c_int) << bpno + (7 as libc::c_int - 1 as libc::c_int);
  let mut f: *mut opj_flag_t = &mut *(*t1).flags.offset(
    ((0 as libc::c_int + 1 as libc::c_int) as libc::c_uint).wrapping_add(
      ((0 as libc::c_int / 4 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
        .wrapping_mul((*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint)),
    ) as isize,
  ) as *mut opj_flag_t;
  let extra = 2 as libc::c_int as OPJ_UINT32;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut datap: *const OPJ_INT32 = (*t1).data;
  *nmsedec = 0 as libc::c_int;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    let w = (*t1).w;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < w {
      if !(*f == 0 as libc::c_uint) {
        let mut v: OPJ_UINT32 = 0;
        let ci = 0 as libc::c_int as OPJ_UINT32;
        let vsc = cblksty & 0x8 as libc::c_int as libc::c_uint;
        let mut l_datap: *const OPJ_INT32 =
          &*datap.offset(0 as libc::c_int as isize) as *const OPJ_INT32;
        let mut flagsp = f;
        let flags = *flagsp;
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << ci.wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << ci.wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1 =
            opj_t1_getctxno_zc(mqc, flags >> ci.wrapping_mul(3 as libc::c_uint)) as OPJ_UINT32;
          v = if *l_datap as OPJ_UINT32 & 0x7fffffff as libc::c_uint & one as OPJ_UINT32 != 0 {
            1 as libc::c_int
          } else {
            0 as libc::c_int
          } as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize) as *mut *const opj_mqc_state_t;
          if type_0 as libc::c_int == 1 as libc::c_int {
            if ct == 0xdeadbeef as libc::c_uint {
              ct = 8 as libc::c_int as OPJ_UINT32
            }
            ct = ct.wrapping_sub(1);
            c = c.wrapping_add(v << ct);
            if ct == 0 as libc::c_int as libc::c_uint {
              *(*mqc).bp = c as OPJ_BYTE;
              ct = 8 as libc::c_int as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                ct = 7 as libc::c_int as OPJ_UINT32
              }
              (*mqc).bp = (*mqc).bp.offset(1);
              c = 0 as libc::c_int as OPJ_UINT32
            }
          } else if (**curctx).mps == v {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                a = (**curctx).qeval
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
              *curctx = (**curctx).nmps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            }
          } else {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a < (**curctx).qeval {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            } else {
              a = (**curctx).qeval
            }
            *curctx = (**curctx).nlps;
            loop {
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if ct == 0 as libc::c_int as libc::c_uint {
                (*mqc).c = c;
                opj_mqc_byteout(mqc);
                c = (*mqc).c;
                ct = (*mqc).ct
              }
              if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          }
          if v != 0 {
            let mut lu = opj_t1_getctxtno_sc_or_spb_index(
              *flagsp,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              ci,
            );
            let mut ctxt2 = opj_t1_getctxno_sc(lu) as OPJ_UINT32;
            v = *l_datap as OPJ_UINT32 >> 31 as libc::c_int;
            *nmsedec += opj_t1_getnmsedec_sig(
              *l_datap as OPJ_UINT32 & 0x7fffffff as libc::c_uint,
              bpno as OPJ_UINT32,
            ) as libc::c_int;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2 as isize) as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else {
              let mut spb = opj_t1_getspb(lu) as OPJ_UINT32;
              if (**curctx).mps == v ^ spb {
                a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    a = (**curctx).qeval
                  } else {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  }
                  *curctx = (**curctx).nmps;
                  loop {
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if ct == 0 as libc::c_int as libc::c_uint {
                      (*mqc).c = c;
                      opj_mqc_byteout(mqc);
                      c = (*mqc).c;
                      ct = (*mqc).ct
                    }
                    if !(a & 0x8000 as libc::c_int as libc::c_uint
                      == 0 as libc::c_int as libc::c_uint)
                    {
                      break;
                    }
                  }
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
              } else {
                a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                if a < (**curctx).qeval {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                } else {
                  a = (**curctx).qeval
                }
                *curctx = (**curctx).nlps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              }
            }
            opj_t1_update_flags(
              flagsp,
              ci,
              v,
              (*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint),
              vsc,
            );
          }
          *flagsp |=
            ((1 as libc::c_uint) << 21 as libc::c_int) << ci.wrapping_mul(3 as libc::c_uint)
        }
        let mut v_0: OPJ_UINT32 = 0;
        let ci_0 = 1 as libc::c_int as OPJ_UINT32;
        let vsc_0 = 0 as libc::c_int as OPJ_UINT32;
        let mut l_datap_0: *const OPJ_INT32 =
          &*datap.offset(1 as libc::c_int as isize) as *const OPJ_INT32;
        let mut flagsp_0 = f;
        let flags_0 = *flagsp_0;
        if flags_0
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << ci_0.wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags_0
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << ci_0.wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_0 =
            opj_t1_getctxno_zc(mqc, flags_0 >> ci_0.wrapping_mul(3 as libc::c_uint)) as OPJ_UINT32;
          v_0 = if *l_datap_0 as OPJ_UINT32 & 0x7fffffff as libc::c_uint & one as OPJ_UINT32 != 0 {
            1 as libc::c_int
          } else {
            0 as libc::c_int
          } as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize) as *mut *const opj_mqc_state_t;
          if type_0 as libc::c_int == 1 as libc::c_int {
            if ct == 0xdeadbeef as libc::c_uint {
              ct = 8 as libc::c_int as OPJ_UINT32
            }
            ct = ct.wrapping_sub(1);
            c = c.wrapping_add(v_0 << ct);
            if ct == 0 as libc::c_int as libc::c_uint {
              *(*mqc).bp = c as OPJ_BYTE;
              ct = 8 as libc::c_int as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                ct = 7 as libc::c_int as OPJ_UINT32
              }
              (*mqc).bp = (*mqc).bp.offset(1);
              c = 0 as libc::c_int as OPJ_UINT32
            }
          } else if (**curctx).mps == v_0 {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                a = (**curctx).qeval
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
              *curctx = (**curctx).nmps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            }
          } else {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a < (**curctx).qeval {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            } else {
              a = (**curctx).qeval
            }
            *curctx = (**curctx).nlps;
            loop {
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if ct == 0 as libc::c_int as libc::c_uint {
                (*mqc).c = c;
                opj_mqc_byteout(mqc);
                c = (*mqc).c;
                ct = (*mqc).ct
              }
              if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          }
          if v_0 != 0 {
            let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
              *flagsp_0,
              *flagsp_0.offset(-(1 as libc::c_int) as isize),
              *flagsp_0.offset(1 as libc::c_int as isize),
              ci_0,
            );
            let mut ctxt2_0 = opj_t1_getctxno_sc(lu_0) as OPJ_UINT32;
            v_0 = *l_datap_0 as OPJ_UINT32 >> 31 as libc::c_int;
            *nmsedec += opj_t1_getnmsedec_sig(
              *l_datap_0 as OPJ_UINT32 & 0x7fffffff as libc::c_uint,
              bpno as OPJ_UINT32,
            ) as libc::c_int;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_0 as isize)
              as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v_0 << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else {
              let mut spb_0 = opj_t1_getspb(lu_0) as OPJ_UINT32;
              if (**curctx).mps == v_0 ^ spb_0 {
                a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    a = (**curctx).qeval
                  } else {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  }
                  *curctx = (**curctx).nmps;
                  loop {
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if ct == 0 as libc::c_int as libc::c_uint {
                      (*mqc).c = c;
                      opj_mqc_byteout(mqc);
                      c = (*mqc).c;
                      ct = (*mqc).ct
                    }
                    if !(a & 0x8000 as libc::c_int as libc::c_uint
                      == 0 as libc::c_int as libc::c_uint)
                    {
                      break;
                    }
                  }
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
              } else {
                a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                if a < (**curctx).qeval {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                } else {
                  a = (**curctx).qeval
                }
                *curctx = (**curctx).nlps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              }
            }
            opj_t1_update_flags(
              flagsp_0,
              ci_0,
              v_0,
              (*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint),
              vsc_0,
            );
          }
          *flagsp_0 |=
            ((1 as libc::c_uint) << 21 as libc::c_int) << ci_0.wrapping_mul(3 as libc::c_uint)
        }
        let mut v_1: OPJ_UINT32 = 0;
        let ci_1 = 2 as libc::c_int as OPJ_UINT32;
        let vsc_1 = 0 as libc::c_int as OPJ_UINT32;
        let mut l_datap_1: *const OPJ_INT32 =
          &*datap.offset(2 as libc::c_int as isize) as *const OPJ_INT32;
        let mut flagsp_1 = f;
        let flags_1 = *flagsp_1;
        if flags_1
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << ci_1.wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags_1
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << ci_1.wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_1 =
            opj_t1_getctxno_zc(mqc, flags_1 >> ci_1.wrapping_mul(3 as libc::c_uint)) as OPJ_UINT32;
          v_1 = if *l_datap_1 as OPJ_UINT32 & 0x7fffffff as libc::c_uint & one as OPJ_UINT32 != 0 {
            1 as libc::c_int
          } else {
            0 as libc::c_int
          } as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize) as *mut *const opj_mqc_state_t;
          if type_0 as libc::c_int == 1 as libc::c_int {
            if ct == 0xdeadbeef as libc::c_uint {
              ct = 8 as libc::c_int as OPJ_UINT32
            }
            ct = ct.wrapping_sub(1);
            c = c.wrapping_add(v_1 << ct);
            if ct == 0 as libc::c_int as libc::c_uint {
              *(*mqc).bp = c as OPJ_BYTE;
              ct = 8 as libc::c_int as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                ct = 7 as libc::c_int as OPJ_UINT32
              }
              (*mqc).bp = (*mqc).bp.offset(1);
              c = 0 as libc::c_int as OPJ_UINT32
            }
          } else if (**curctx).mps == v_1 {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                a = (**curctx).qeval
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
              *curctx = (**curctx).nmps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            }
          } else {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a < (**curctx).qeval {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            } else {
              a = (**curctx).qeval
            }
            *curctx = (**curctx).nlps;
            loop {
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if ct == 0 as libc::c_int as libc::c_uint {
                (*mqc).c = c;
                opj_mqc_byteout(mqc);
                c = (*mqc).c;
                ct = (*mqc).ct
              }
              if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          }
          if v_1 != 0 {
            let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
              *flagsp_1,
              *flagsp_1.offset(-(1 as libc::c_int) as isize),
              *flagsp_1.offset(1 as libc::c_int as isize),
              ci_1,
            );
            let mut ctxt2_1 = opj_t1_getctxno_sc(lu_1) as OPJ_UINT32;
            v_1 = *l_datap_1 as OPJ_UINT32 >> 31 as libc::c_int;
            *nmsedec += opj_t1_getnmsedec_sig(
              *l_datap_1 as OPJ_UINT32 & 0x7fffffff as libc::c_uint,
              bpno as OPJ_UINT32,
            ) as libc::c_int;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_1 as isize)
              as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v_1 << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else {
              let mut spb_1 = opj_t1_getspb(lu_1) as OPJ_UINT32;
              if (**curctx).mps == v_1 ^ spb_1 {
                a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    a = (**curctx).qeval
                  } else {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  }
                  *curctx = (**curctx).nmps;
                  loop {
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if ct == 0 as libc::c_int as libc::c_uint {
                      (*mqc).c = c;
                      opj_mqc_byteout(mqc);
                      c = (*mqc).c;
                      ct = (*mqc).ct
                    }
                    if !(a & 0x8000 as libc::c_int as libc::c_uint
                      == 0 as libc::c_int as libc::c_uint)
                    {
                      break;
                    }
                  }
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
              } else {
                a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                if a < (**curctx).qeval {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                } else {
                  a = (**curctx).qeval
                }
                *curctx = (**curctx).nlps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              }
            }
            opj_t1_update_flags(
              flagsp_1,
              ci_1,
              v_1,
              (*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint),
              vsc_1,
            );
          }
          *flagsp_1 |=
            ((1 as libc::c_uint) << 21 as libc::c_int) << ci_1.wrapping_mul(3 as libc::c_uint)
        }
        let mut v_2: OPJ_UINT32 = 0;
        let ci_2 = 3 as libc::c_int as OPJ_UINT32;
        let vsc_2 = 0 as libc::c_int as OPJ_UINT32;
        let mut l_datap_2: *const OPJ_INT32 =
          &*datap.offset(3 as libc::c_int as isize) as *const OPJ_INT32;
        let mut flagsp_2 = f;
        let flags_2 = *flagsp_2;
        if flags_2
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << ci_2.wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags_2
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << ci_2.wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_2 =
            opj_t1_getctxno_zc(mqc, flags_2 >> ci_2.wrapping_mul(3 as libc::c_uint)) as OPJ_UINT32;
          v_2 = if *l_datap_2 as OPJ_UINT32 & 0x7fffffff as libc::c_uint & one as OPJ_UINT32 != 0 {
            1 as libc::c_int
          } else {
            0 as libc::c_int
          } as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize) as *mut *const opj_mqc_state_t;
          if type_0 as libc::c_int == 1 as libc::c_int {
            if ct == 0xdeadbeef as libc::c_uint {
              ct = 8 as libc::c_int as OPJ_UINT32
            }
            ct = ct.wrapping_sub(1);
            c = c.wrapping_add(v_2 << ct);
            if ct == 0 as libc::c_int as libc::c_uint {
              *(*mqc).bp = c as OPJ_BYTE;
              ct = 8 as libc::c_int as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                ct = 7 as libc::c_int as OPJ_UINT32
              }
              (*mqc).bp = (*mqc).bp.offset(1);
              c = 0 as libc::c_int as OPJ_UINT32
            }
          } else if (**curctx).mps == v_2 {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                a = (**curctx).qeval
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
              *curctx = (**curctx).nmps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            }
          } else {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a < (**curctx).qeval {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            } else {
              a = (**curctx).qeval
            }
            *curctx = (**curctx).nlps;
            loop {
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if ct == 0 as libc::c_int as libc::c_uint {
                (*mqc).c = c;
                opj_mqc_byteout(mqc);
                c = (*mqc).c;
                ct = (*mqc).ct
              }
              if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          }
          if v_2 != 0 {
            let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
              *flagsp_2,
              *flagsp_2.offset(-(1 as libc::c_int) as isize),
              *flagsp_2.offset(1 as libc::c_int as isize),
              ci_2,
            );
            let mut ctxt2_2 = opj_t1_getctxno_sc(lu_2) as OPJ_UINT32;
            v_2 = *l_datap_2 as OPJ_UINT32 >> 31 as libc::c_int;
            *nmsedec += opj_t1_getnmsedec_sig(
              *l_datap_2 as OPJ_UINT32 & 0x7fffffff as libc::c_uint,
              bpno as OPJ_UINT32,
            ) as libc::c_int;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_2 as isize)
              as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v_2 << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else {
              let mut spb_2 = opj_t1_getspb(lu_2) as OPJ_UINT32;
              if (**curctx).mps == v_2 ^ spb_2 {
                a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    a = (**curctx).qeval
                  } else {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  }
                  *curctx = (**curctx).nmps;
                  loop {
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if ct == 0 as libc::c_int as libc::c_uint {
                      (*mqc).c = c;
                      opj_mqc_byteout(mqc);
                      c = (*mqc).c;
                      ct = (*mqc).ct
                    }
                    if !(a & 0x8000 as libc::c_int as libc::c_uint
                      == 0 as libc::c_int as libc::c_uint)
                    {
                      break;
                    }
                  }
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
              } else {
                a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                if a < (**curctx).qeval {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                } else {
                  a = (**curctx).qeval
                }
                *curctx = (**curctx).nlps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              }
            }
            opj_t1_update_flags(
              flagsp_2,
              ci_2,
              v_2,
              (*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint),
              vsc_2,
            );
          }
          *flagsp_2 |=
            ((1 as libc::c_uint) << 21 as libc::c_int) << ci_2.wrapping_mul(3 as libc::c_uint)
        }
      }
      /* Nothing to do for any of the 4 data points */
      i = i.wrapping_add(1);
      f = f.offset(1);
      datap = datap.offset(4 as libc::c_int as isize)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    f = f.offset(extra as isize)
  }
  if k < (*t1).h {
    let mut j: OPJ_UINT32 = 0;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < (*t1).w {
      if *f == 0 as libc::c_uint {
        /* Nothing to do for any of the 4 data points */
        datap = datap.offset((*t1).h.wrapping_sub(k) as isize)
      } else {
        j = k;
        while j < (*t1).h {
          let mut v_3: OPJ_UINT32 = 0;
          let ci_3 = j.wrapping_sub(k);
          let vsc_3 = (j == k
            && cblksty & 0x8 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint)
            as libc::c_int as OPJ_UINT32;
          let mut l_datap_3: *const OPJ_INT32 =
            &*datap.offset(0 as libc::c_int as isize) as *const OPJ_INT32;
          let mut flagsp_3 = f;
          let flags_3 = *flagsp_3;
          if flags_3
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << ci_3.wrapping_mul(3 as libc::c_uint)
            == 0 as libc::c_uint
            && flags_3
              & ((1 as libc::c_uint) << 0 as libc::c_int
                | (1 as libc::c_uint) << 1 as libc::c_int
                | (1 as libc::c_uint) << 2 as libc::c_int
                | (1 as libc::c_uint) << 3 as libc::c_int
                | (1 as libc::c_uint) << 5 as libc::c_int
                | (1 as libc::c_uint) << 6 as libc::c_int
                | (1 as libc::c_uint) << 7 as libc::c_int
                | (1 as libc::c_uint) << 8 as libc::c_int)
                << ci_3.wrapping_mul(3 as libc::c_uint)
              != 0 as libc::c_uint
          {
            let mut ctxt1_3 =
              opj_t1_getctxno_zc(mqc, flags_3 >> ci_3.wrapping_mul(3 as libc::c_uint))
                as OPJ_UINT32;
            v_3 = if *l_datap_3 as OPJ_UINT32 & 0x7fffffff as libc::c_uint & one as OPJ_UINT32 != 0
            {
              1 as libc::c_int
            } else {
              0 as libc::c_int
            } as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_3 as isize)
              as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v_3 << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else if (**curctx).mps == v_3 {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
                *curctx = (**curctx).nmps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
            } else {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a < (**curctx).qeval {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              } else {
                a = (**curctx).qeval
              }
              *curctx = (**curctx).nlps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            }
            if v_3 != 0 {
              let mut lu_3 = opj_t1_getctxtno_sc_or_spb_index(
                *flagsp_3,
                *flagsp_3.offset(-(1 as libc::c_int) as isize),
                *flagsp_3.offset(1 as libc::c_int as isize),
                ci_3,
              );
              let mut ctxt2_3 = opj_t1_getctxno_sc(lu_3) as OPJ_UINT32;
              v_3 = *l_datap_3 as OPJ_UINT32 >> 31 as libc::c_int;
              *nmsedec += opj_t1_getnmsedec_sig(
                *l_datap_3 as OPJ_UINT32 & 0x7fffffff as libc::c_uint,
                bpno as OPJ_UINT32,
              ) as libc::c_int;
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_3 as isize)
                as *mut *const opj_mqc_state_t;
              if type_0 as libc::c_int == 1 as libc::c_int {
                if ct == 0xdeadbeef as libc::c_uint {
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
                ct = ct.wrapping_sub(1);
                c = c.wrapping_add(v_3 << ct);
                if ct == 0 as libc::c_int as libc::c_uint {
                  *(*mqc).bp = c as OPJ_BYTE;
                  ct = 8 as libc::c_int as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = 0 as libc::c_int as OPJ_UINT32
                }
              } else {
                let mut spb_3 = opj_t1_getspb(lu_3) as OPJ_UINT32;
                if (**curctx).mps == v_3 ^ spb_3 {
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval
                    } else {
                      c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32
                        as OPJ_UINT32
                    }
                    *curctx = (**curctx).nmps;
                    loop {
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if ct == 0 as libc::c_int as libc::c_uint {
                        (*mqc).c = c;
                        opj_mqc_byteout(mqc);
                        c = (*mqc).c;
                        ct = (*mqc).ct
                      }
                      if !(a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint)
                      {
                        break;
                      }
                    }
                  } else {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  }
                } else {
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if a < (**curctx).qeval {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  } else {
                    a = (**curctx).qeval
                  }
                  *curctx = (**curctx).nlps;
                  loop {
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if ct == 0 as libc::c_int as libc::c_uint {
                      (*mqc).c = c;
                      opj_mqc_byteout(mqc);
                      c = (*mqc).c;
                      ct = (*mqc).ct
                    }
                    if !(a & 0x8000 as libc::c_int as libc::c_uint
                      == 0 as libc::c_int as libc::c_uint)
                    {
                      break;
                    }
                  }
                }
              }
              opj_t1_update_flags(
                flagsp_3,
                ci_3,
                v_3,
                (*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint),
                vsc_3,
              );
            }
            *flagsp_3 |=
              ((1 as libc::c_uint) << 21 as libc::c_int) << ci_3.wrapping_mul(3 as libc::c_uint)
          }
          j = j.wrapping_add(1);
          datap = datap.offset(1)
        }
      }
      i = i.wrapping_add(1);
      f = f.offset(1)
    }
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
}
/* *
Decode significant pass
*/
unsafe extern "C" fn opj_t1_dec_sigpass_raw(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
  mut cblksty: OPJ_INT32,
) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1).flags.offset(
    ((0 as libc::c_int + 1 as libc::c_int) as libc::c_uint).wrapping_add(
      ((0 as libc::c_int / 4 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
        .wrapping_mul((*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint)),
    ) as isize,
  ) as *mut opj_flag_t;
  let l_w = (*t1).w;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags != 0 as libc::c_int as libc::c_uint {
        opj_t1_dec_sigpass_step_raw(
          t1,
          flagsp,
          data,
          oneplushalf,
          (cblksty & 0x8 as libc::c_int) as OPJ_UINT32,
          0 as libc::c_uint,
        );
        opj_t1_dec_sigpass_step_raw(
          t1,
          flagsp,
          data.offset(l_w as isize),
          oneplushalf,
          0 as libc::c_int as OPJ_UINT32,
          1 as libc::c_uint,
        );
        opj_t1_dec_sigpass_step_raw(
          t1,
          flagsp,
          data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize),
          oneplushalf,
          0 as libc::c_int as OPJ_UINT32,
          2 as libc::c_uint,
        );
        opj_t1_dec_sigpass_step_raw(
          t1,
          flagsp,
          data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize),
          oneplushalf,
          0 as libc::c_int as OPJ_UINT32,
          3 as libc::c_uint,
        );
      }
      i = i.wrapping_add(1);
      flagsp = flagsp.offset(1);
      data = data.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    flagsp = flagsp.offset(2 as libc::c_int as isize);
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
  }
  if k < (*t1).h {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (*t1).h.wrapping_sub(k) {
        opj_t1_dec_sigpass_step_raw(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          (cblksty & 0x8 as libc::c_int) as OPJ_UINT32,
          j,
        );
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1);
      flagsp = flagsp.offset(1);
      data = data.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_sigpass_mqc_64x64_novsc(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1)
    .flags
    .offset((66 as libc::c_int + 1 as libc::c_int) as isize)
    as *mut opj_flag_t;
  let l_w = 64 as libc::c_int as OPJ_UINT32;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < 64 as libc::c_int as libc::c_uint & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags != 0 as libc::c_int as libc::c_uint {
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1 = opj_t1_getctxno_zc(
            mqc,
            flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c: OPJ_UINT32 = 0;
                l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_0: OPJ_UINT32 = 0;
                  l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              0 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2 = opj_t1_getctxno_sc(lu) as OPJ_UINT32;
            let mut spb = opj_t1_getspb(lu) as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2 as isize) as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_1: OPJ_UINT32 = 0;
                  l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_2: OPJ_UINT32 = 0;
                    l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb;
            *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh12 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh12 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            let ref mut fresh13 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh13 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north = flagsp.offset(-(66 as libc::c_int as isize));
              *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh14 = *north.offset(-(1 as libc::c_int) as isize);
              *fresh14 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh15 = *north.offset(1 as libc::c_int as isize);
              *fresh15 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south = flagsp.offset(66 as libc::c_int as isize);
              *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh16 = *south.offset(-(1 as libc::c_int) as isize);
              *fresh16 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh17 = *south.offset(1 as libc::c_int as isize);
              *fresh17 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_0 = opj_t1_getctxno_zc(
            mqc,
            flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              1 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_0 = opj_t1_getctxno_sc(lu_0) as OPJ_UINT32;
            let mut spb_0 = opj_t1_getspb(lu_0) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_0 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_5: OPJ_UINT32 = 0;
                  l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_6: OPJ_UINT32 = 0;
                    l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_0;
            *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh18 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh18 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            let ref mut fresh19 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh19 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_0 = flagsp.offset(-(66 as libc::c_int as isize));
              *north_0 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh20 = *north_0.offset(-(1 as libc::c_int) as isize);
              *fresh20 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh21 = *north_0.offset(1 as libc::c_int as isize);
              *fresh21 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_0 = flagsp.offset(66 as libc::c_int as isize);
              *south_0 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh22 = *south_0.offset(-(1 as libc::c_int) as isize);
              *fresh22 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh23 = *south_0.offset(1 as libc::c_int as isize);
              *fresh23 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_1 = opj_t1_getctxno_zc(
            mqc,
            flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_7: OPJ_UINT32 = 0;
                l_c_7 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_7 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_7 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_7 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_8: OPJ_UINT32 = 0;
                  l_c_8 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_8 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_8 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_8 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              2 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_1 = opj_t1_getctxno_sc(lu_1) as OPJ_UINT32;
            let mut spb_1 = opj_t1_getspb(lu_1) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_1 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_9: OPJ_UINT32 = 0;
                  l_c_9 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_9 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_9 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_9 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_10: OPJ_UINT32 = 0;
                    l_c_10 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_10 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_10 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_10 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_1;
            *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh24 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh24 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            let ref mut fresh25 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh25 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_1 = flagsp.offset(-(66 as libc::c_int as isize));
              *north_1 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh26 = *north_1.offset(-(1 as libc::c_int) as isize);
              *fresh26 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh27 = *north_1.offset(1 as libc::c_int as isize);
              *fresh27 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_1 = flagsp.offset(66 as libc::c_int as isize);
              *south_1 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh28 = *south_1.offset(-(1 as libc::c_int) as isize);
              *fresh28 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh29 = *south_1.offset(1 as libc::c_int as isize);
              *fresh29 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_2 = opj_t1_getctxno_zc(
            mqc,
            flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_11: OPJ_UINT32 = 0;
                l_c_11 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_11 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_11 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_11 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_12: OPJ_UINT32 = 0;
                  l_c_12 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_12 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_12 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_12 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              3 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_2 = opj_t1_getctxno_sc(lu_2) as OPJ_UINT32;
            let mut spb_2 = opj_t1_getspb(lu_2) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_2 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_13: OPJ_UINT32 = 0;
                  l_c_13 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_13 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_13 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_13 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_14: OPJ_UINT32 = 0;
                    l_c_14 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_14 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_14 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_14 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_2;
            *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh30 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh30 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            let ref mut fresh31 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh31 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_2 = flagsp.offset(-(66 as libc::c_int as isize));
              *north_2 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh32 = *north_2.offset(-(1 as libc::c_int) as isize);
              *fresh32 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh33 = *north_2.offset(1 as libc::c_int as isize);
              *fresh33 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_2 = flagsp.offset(66 as libc::c_int as isize);
              *south_2 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh34 = *south_2.offset(-(1 as libc::c_int) as isize);
              *fresh34 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh35 = *south_2.offset(1 as libc::c_int as isize);
              *fresh35 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        *flagsp = flags
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < 64 as libc::c_int as libc::c_uint {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (64 as libc::c_int as libc::c_uint).wrapping_sub(k) {
        opj_t1_dec_sigpass_step_mqc(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          j,
          66 as libc::c_int as OPJ_UINT32,
          0 as libc::c_int as OPJ_UINT32,
        );
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_sigpass_mqc_64x64_vsc(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1)
    .flags
    .offset((66 as libc::c_int + 1 as libc::c_int) as isize)
    as *mut opj_flag_t;
  let l_w = 64 as libc::c_int as OPJ_UINT32;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < 64 as libc::c_int as libc::c_uint & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags != 0 as libc::c_int as libc::c_uint {
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1 = opj_t1_getctxno_zc(
            mqc,
            flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c: OPJ_UINT32 = 0;
                l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_0: OPJ_UINT32 = 0;
                  l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              0 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2 = opj_t1_getctxno_sc(lu) as OPJ_UINT32;
            let mut spb = opj_t1_getspb(lu) as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2 as isize) as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_1: OPJ_UINT32 = 0;
                  l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_2: OPJ_UINT32 = 0;
                    l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb;
            *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh36 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh36 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            let ref mut fresh37 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh37 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 1 as libc::c_int == 0 {
              let mut north = flagsp.offset(-(66 as libc::c_int as isize));
              *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh38 = *north.offset(-(1 as libc::c_int) as isize);
              *fresh38 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh39 = *north.offset(1 as libc::c_int as isize);
              *fresh39 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south = flagsp.offset(66 as libc::c_int as isize);
              *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh40 = *south.offset(-(1 as libc::c_int) as isize);
              *fresh40 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh41 = *south.offset(1 as libc::c_int as isize);
              *fresh41 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_0 = opj_t1_getctxno_zc(
            mqc,
            flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              1 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_0 = opj_t1_getctxno_sc(lu_0) as OPJ_UINT32;
            let mut spb_0 = opj_t1_getspb(lu_0) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_0 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_5: OPJ_UINT32 = 0;
                  l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_6: OPJ_UINT32 = 0;
                    l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_0;
            *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh42 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh42 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            let ref mut fresh43 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh43 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_0 = flagsp.offset(-(66 as libc::c_int as isize));
              *north_0 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh44 = *north_0.offset(-(1 as libc::c_int) as isize);
              *fresh44 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh45 = *north_0.offset(1 as libc::c_int as isize);
              *fresh45 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_0 = flagsp.offset(66 as libc::c_int as isize);
              *south_0 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh46 = *south_0.offset(-(1 as libc::c_int) as isize);
              *fresh46 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh47 = *south_0.offset(1 as libc::c_int as isize);
              *fresh47 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_1 = opj_t1_getctxno_zc(
            mqc,
            flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_7: OPJ_UINT32 = 0;
                l_c_7 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_7 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_7 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_7 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_8: OPJ_UINT32 = 0;
                  l_c_8 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_8 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_8 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_8 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              2 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_1 = opj_t1_getctxno_sc(lu_1) as OPJ_UINT32;
            let mut spb_1 = opj_t1_getspb(lu_1) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_1 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_9: OPJ_UINT32 = 0;
                  l_c_9 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_9 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_9 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_9 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_10: OPJ_UINT32 = 0;
                    l_c_10 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_10 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_10 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_10 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_1;
            *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh48 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh48 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            let ref mut fresh49 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh49 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_1 = flagsp.offset(-(66 as libc::c_int as isize));
              *north_1 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh50 = *north_1.offset(-(1 as libc::c_int) as isize);
              *fresh50 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh51 = *north_1.offset(1 as libc::c_int as isize);
              *fresh51 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_1 = flagsp.offset(66 as libc::c_int as isize);
              *south_1 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh52 = *south_1.offset(-(1 as libc::c_int) as isize);
              *fresh52 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh53 = *south_1.offset(1 as libc::c_int as isize);
              *fresh53 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_2 = opj_t1_getctxno_zc(
            mqc,
            flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_11: OPJ_UINT32 = 0;
                l_c_11 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_11 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_11 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_11 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_12: OPJ_UINT32 = 0;
                  l_c_12 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_12 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_12 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_12 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              3 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_2 = opj_t1_getctxno_sc(lu_2) as OPJ_UINT32;
            let mut spb_2 = opj_t1_getspb(lu_2) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_2 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_13: OPJ_UINT32 = 0;
                  l_c_13 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_13 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_13 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_13 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_14: OPJ_UINT32 = 0;
                    l_c_14 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_14 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_14 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_14 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_2;
            *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh54 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh54 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            let ref mut fresh55 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh55 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_2 = flagsp.offset(-(66 as libc::c_int as isize));
              *north_2 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh56 = *north_2.offset(-(1 as libc::c_int) as isize);
              *fresh56 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh57 = *north_2.offset(1 as libc::c_int as isize);
              *fresh57 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_2 = flagsp.offset(66 as libc::c_int as isize);
              *south_2 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh58 = *south_2.offset(-(1 as libc::c_int) as isize);
              *fresh58 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh59 = *south_2.offset(1 as libc::c_int as isize);
              *fresh59 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        *flagsp = flags
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < 64 as libc::c_int as libc::c_uint {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (64 as libc::c_int as libc::c_uint).wrapping_sub(k) {
        opj_t1_dec_sigpass_step_mqc(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          j,
          66 as libc::c_int as OPJ_UINT32,
          1 as libc::c_int as OPJ_UINT32,
        );
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_sigpass_mqc_generic_novsc(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1).flags.offset(
    (*t1)
      .w
      .wrapping_add(2 as libc::c_uint)
      .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
  ) as *mut opj_flag_t;
  let l_w = (*t1).w;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags != 0 as libc::c_int as libc::c_uint {
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1 = opj_t1_getctxno_zc(
            mqc,
            flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c: OPJ_UINT32 = 0;
                l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_0: OPJ_UINT32 = 0;
                  l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              0 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2 = opj_t1_getctxno_sc(lu) as OPJ_UINT32;
            let mut spb = opj_t1_getspb(lu) as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2 as isize) as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_1: OPJ_UINT32 = 0;
                  l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_2: OPJ_UINT32 = 0;
                    l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb;
            *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh60 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh60 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            let ref mut fresh61 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh61 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
              *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh62 = *north.offset(-(1 as libc::c_int) as isize);
              *fresh62 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh63 = *north.offset(1 as libc::c_int as isize);
              *fresh63 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
              *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh64 = *south.offset(-(1 as libc::c_int) as isize);
              *fresh64 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh65 = *south.offset(1 as libc::c_int as isize);
              *fresh65 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_0 = opj_t1_getctxno_zc(
            mqc,
            flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              1 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_0 = opj_t1_getctxno_sc(lu_0) as OPJ_UINT32;
            let mut spb_0 = opj_t1_getspb(lu_0) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_0 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_5: OPJ_UINT32 = 0;
                  l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_6: OPJ_UINT32 = 0;
                    l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_0;
            *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh66 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh66 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            let ref mut fresh67 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh67 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_0 = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
              *north_0 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh68 = *north_0.offset(-(1 as libc::c_int) as isize);
              *fresh68 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh69 = *north_0.offset(1 as libc::c_int as isize);
              *fresh69 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_0 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
              *south_0 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh70 = *south_0.offset(-(1 as libc::c_int) as isize);
              *fresh70 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh71 = *south_0.offset(1 as libc::c_int as isize);
              *fresh71 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_1 = opj_t1_getctxno_zc(
            mqc,
            flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_7: OPJ_UINT32 = 0;
                l_c_7 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_7 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_7 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_7 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_8: OPJ_UINT32 = 0;
                  l_c_8 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_8 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_8 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_8 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              2 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_1 = opj_t1_getctxno_sc(lu_1) as OPJ_UINT32;
            let mut spb_1 = opj_t1_getspb(lu_1) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_1 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_9: OPJ_UINT32 = 0;
                  l_c_9 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_9 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_9 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_9 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_10: OPJ_UINT32 = 0;
                    l_c_10 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_10 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_10 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_10 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_1;
            *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh72 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh72 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            let ref mut fresh73 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh73 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_1 = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
              *north_1 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh74 = *north_1.offset(-(1 as libc::c_int) as isize);
              *fresh74 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh75 = *north_1.offset(1 as libc::c_int as isize);
              *fresh75 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_1 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
              *south_1 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh76 = *south_1.offset(-(1 as libc::c_int) as isize);
              *fresh76 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh77 = *south_1.offset(1 as libc::c_int as isize);
              *fresh77 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_2 = opj_t1_getctxno_zc(
            mqc,
            flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_11: OPJ_UINT32 = 0;
                l_c_11 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_11 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_11 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_11 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_12: OPJ_UINT32 = 0;
                  l_c_12 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_12 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_12 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_12 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              3 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_2 = opj_t1_getctxno_sc(lu_2) as OPJ_UINT32;
            let mut spb_2 = opj_t1_getspb(lu_2) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_2 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_13: OPJ_UINT32 = 0;
                  l_c_13 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_13 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_13 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_13 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_14: OPJ_UINT32 = 0;
                    l_c_14 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_14 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_14 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_14 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_2;
            *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh78 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh78 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            let ref mut fresh79 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh79 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_2 = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
              *north_2 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh80 = *north_2.offset(-(1 as libc::c_int) as isize);
              *fresh80 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh81 = *north_2.offset(1 as libc::c_int as isize);
              *fresh81 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_2 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
              *south_2 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh82 = *south_2.offset(-(1 as libc::c_int) as isize);
              *fresh82 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh83 = *south_2.offset(1 as libc::c_int as isize);
              *fresh83 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        *flagsp = flags
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < (*t1).h {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (*t1).h.wrapping_sub(k) {
        opj_t1_dec_sigpass_step_mqc(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          j,
          (*t1).w.wrapping_add(2 as libc::c_uint),
          0 as libc::c_int as OPJ_UINT32,
        );
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_sigpass_mqc_generic_vsc(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1).flags.offset(
    (*t1)
      .w
      .wrapping_add(2 as libc::c_uint)
      .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
  ) as *mut opj_flag_t;
  let l_w = (*t1).w;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags != 0 as libc::c_int as libc::c_uint {
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1 = opj_t1_getctxno_zc(
            mqc,
            flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c: OPJ_UINT32 = 0;
                l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_0: OPJ_UINT32 = 0;
                  l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              0 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2 = opj_t1_getctxno_sc(lu) as OPJ_UINT32;
            let mut spb = opj_t1_getspb(lu) as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2 as isize) as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_1: OPJ_UINT32 = 0;
                  l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_2: OPJ_UINT32 = 0;
                    l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb;
            *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh84 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh84 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            let ref mut fresh85 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh85 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
            if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 1 as libc::c_int == 0 {
              let mut north = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
              *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh86 = *north.offset(-(1 as libc::c_int) as isize);
              *fresh86 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh87 = *north.offset(1 as libc::c_int as isize);
              *fresh87 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
              *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh88 = *south.offset(-(1 as libc::c_int) as isize);
              *fresh88 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh89 = *south.offset(1 as libc::c_int as isize);
              *fresh89 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_0 = opj_t1_getctxno_zc(
            mqc,
            flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              1 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_0 = opj_t1_getctxno_sc(lu_0) as OPJ_UINT32;
            let mut spb_0 = opj_t1_getspb(lu_0) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_0 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_5: OPJ_UINT32 = 0;
                  l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_6: OPJ_UINT32 = 0;
                    l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_0;
            *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh90 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh90 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            let ref mut fresh91 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh91 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
            if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_0 = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
              *north_0 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh92 = *north_0.offset(-(1 as libc::c_int) as isize);
              *fresh92 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh93 = *north_0.offset(1 as libc::c_int as isize);
              *fresh93 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_0 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
              *south_0 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh94 = *south_0.offset(-(1 as libc::c_int) as isize);
              *fresh94 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh95 = *south_0.offset(1 as libc::c_int as isize);
              *fresh95 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_1 = opj_t1_getctxno_zc(
            mqc,
            flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_7: OPJ_UINT32 = 0;
                l_c_7 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_7 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_7 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_7 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_8: OPJ_UINT32 = 0;
                  l_c_8 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_8 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_8 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_8 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              2 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_1 = opj_t1_getctxno_sc(lu_1) as OPJ_UINT32;
            let mut spb_1 = opj_t1_getspb(lu_1) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_1 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_9: OPJ_UINT32 = 0;
                  l_c_9 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_9 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_9 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_9 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_10: OPJ_UINT32 = 0;
                    l_c_10 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_10 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_10 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_10 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_1;
            *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh96 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh96 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            let ref mut fresh97 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh97 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
            if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_1 = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
              *north_1 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh98 = *north_1.offset(-(1 as libc::c_int) as isize);
              *fresh98 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh99 = *north_1.offset(1 as libc::c_int as isize);
              *fresh99 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_1 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
              *south_1 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh100 = *south_1.offset(-(1 as libc::c_int) as isize);
              *fresh100 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh101 = *south_1.offset(1 as libc::c_int as isize);
              *fresh101 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == 0 as libc::c_uint
          && flags
            & ((1 as libc::c_uint) << 0 as libc::c_int
              | (1 as libc::c_uint) << 1 as libc::c_int
              | (1 as libc::c_uint) << 2 as libc::c_int
              | (1 as libc::c_uint) << 3 as libc::c_int
              | (1 as libc::c_uint) << 5 as libc::c_int
              | (1 as libc::c_uint) << 6 as libc::c_int
              | (1 as libc::c_uint) << 7 as libc::c_int
              | (1 as libc::c_uint) << 8 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            != 0 as libc::c_uint
        {
          let mut ctxt1_2 = opj_t1_getctxno_zc(
            mqc,
            flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          ) as OPJ_UINT32;
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_11: OPJ_UINT32 = 0;
                l_c_11 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_11 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_11 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_11 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_12: OPJ_UINT32 = 0;
                  l_c_12 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_12 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_12 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_12 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          if v != 0 {
            let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
              flags,
              *flagsp.offset(-(1 as libc::c_int) as isize),
              *flagsp.offset(1 as libc::c_int as isize),
              3 as libc::c_int as OPJ_UINT32,
            );
            let mut ctxt2_2 = opj_t1_getctxno_sc(lu_2) as OPJ_UINT32;
            let mut spb_2 = opj_t1_getspb(lu_2) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_2 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_13: OPJ_UINT32 = 0;
                  l_c_13 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_13 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_13 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_13 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_14: OPJ_UINT32 = 0;
                    l_c_14 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_14 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_14 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_14 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            v = v ^ spb_2;
            *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
              if v != 0 { -oneplushalf } else { oneplushalf };
            let ref mut fresh102 = *flagsp.offset(-(1 as libc::c_int) as isize);
            *fresh102 |= ((1 as libc::c_uint) << 5 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            let ref mut fresh103 = *flagsp.offset(1 as libc::c_int as isize);
            *fresh103 |= ((1 as libc::c_uint) << 3 as libc::c_int)
              << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
            if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
              let mut north_2 = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
              *north_2 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
              let ref mut fresh104 = *north_2.offset(-(1 as libc::c_int) as isize);
              *fresh104 |= (1 as libc::c_uint) << 17 as libc::c_int;
              let ref mut fresh105 = *north_2.offset(1 as libc::c_int as isize);
              *fresh105 |= (1 as libc::c_uint) << 15 as libc::c_int
            }
            if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
              let mut south_2 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
              *south_2 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
              let ref mut fresh106 = *south_2.offset(-(1 as libc::c_int) as isize);
              *fresh106 |= (1 as libc::c_uint) << 2 as libc::c_int;
              let ref mut fresh107 = *south_2.offset(1 as libc::c_int as isize);
              *fresh107 |= (1 as libc::c_uint) << 0 as libc::c_int
            }
          }
          flags |= ((1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        *flagsp = flags
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < (*t1).h {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (*t1).h.wrapping_sub(k) {
        opj_t1_dec_sigpass_step_mqc(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          j,
          (*t1).w.wrapping_add(2 as libc::c_uint),
          1 as libc::c_int as OPJ_UINT32,
        );
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_sigpass_mqc(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
  mut cblksty: OPJ_INT32,
) {
  if (*t1).w == 64 as libc::c_int as libc::c_uint && (*t1).h == 64 as libc::c_int as libc::c_uint {
    if cblksty & 0x8 as libc::c_int != 0 {
      opj_t1_dec_sigpass_mqc_64x64_vsc(t1, bpno);
    } else {
      opj_t1_dec_sigpass_mqc_64x64_novsc(t1, bpno);
    }
  } else if cblksty & 0x8 as libc::c_int != 0 {
    opj_t1_dec_sigpass_mqc_generic_vsc(t1, bpno);
  } else {
    opj_t1_dec_sigpass_mqc_generic_novsc(t1, bpno);
  };
}
/* *
Decode refinement pass
*/
/* *
Encode refinement pass step
*/
/* BYPASS/LAZY MODE */
#[inline]
unsafe extern "C" fn opj_t1_dec_refpass_step_raw(
  mut t1: *mut opj_t1_t,
  mut flagsp: *mut opj_flag_t,
  mut datap: *mut OPJ_INT32,
  mut poshalf: OPJ_INT32,
  mut ci: OPJ_UINT32,
) {
  let mut v: OPJ_UINT32 = 0; /* RAW component */
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc; /* MQC component */
  if *flagsp
    & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
      << ci.wrapping_mul(3 as libc::c_uint)
    == ((1 as libc::c_uint) << 4 as libc::c_int) << ci.wrapping_mul(3 as libc::c_uint)
  {
    v = opj_mqc_raw_decode(mqc);
    *datap += if v ^ (*datap < 0 as libc::c_int) as libc::c_int as libc::c_uint != 0 {
      poshalf
    } else {
      -poshalf
    };
    *flagsp |= ((1 as libc::c_uint) << 20 as libc::c_int) << ci.wrapping_mul(3 as libc::c_uint)
  };
}
#[inline]
unsafe extern "C" fn opj_t1_dec_refpass_step_mqc(
  mut t1: *mut opj_t1_t,
  mut flagsp: *mut opj_flag_t,
  mut datap: *mut OPJ_INT32,
  mut poshalf: OPJ_INT32,
  mut ci: OPJ_UINT32,
) {
  let mut v: OPJ_UINT32 = 0;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  if *flagsp
    & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
      << ci.wrapping_mul(3 as libc::c_uint)
    == ((1 as libc::c_uint) << 4 as libc::c_int) << ci.wrapping_mul(3 as libc::c_uint)
  {
    let mut ctxt = opj_t1_getctxno_mag(*flagsp >> ci.wrapping_mul(3 as libc::c_uint));
    (*mqc).curctx =
      &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt as isize) as *mut *const opj_mqc_state_t;
    (*mqc).a =
      ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
    if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
      if (*mqc).a < (**(*mqc).curctx).qeval {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v = (**(*mqc).curctx).mps;
        *(*mqc).curctx = (**(*mqc).curctx).nmps
      } else {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
        *(*mqc).curctx = (**(*mqc).curctx).nlps
      }
      loop {
        if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
          let mut l_c: OPJ_UINT32 = 0;
          l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
            if l_c > 0x8f as libc::c_int as libc::c_uint {
              (*mqc).c = ((*mqc).c as libc::c_uint)
                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
              (*mqc).end_of_byte_stream_counter = (*mqc).end_of_byte_stream_counter.wrapping_add(1)
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
            }
          } else {
            (*mqc).bp = (*mqc).bp.offset(1);
            (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
          }
        }
        (*mqc).a <<= 1 as libc::c_int;
        (*mqc).c <<= 1 as libc::c_int;
        (*mqc).ct = (*mqc).ct.wrapping_sub(1);
        if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
          break;
        }
      }
    } else {
      (*mqc).c = ((*mqc).c as libc::c_uint)
        .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
        as OPJ_UINT32;
      if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        if (*mqc).a < (**(*mqc).curctx).qeval {
          v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
          *(*mqc).curctx = (**(*mqc).curctx).nlps
        } else {
          v = (**(*mqc).curctx).mps;
          *(*mqc).curctx = (**(*mqc).curctx).nmps
        }
        loop {
          if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
            let mut l_c_0: OPJ_UINT32 = 0;
            l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
              if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                (*mqc).c = ((*mqc).c as libc::c_uint)
                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                (*mqc).end_of_byte_stream_counter =
                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
              }
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
            }
          }
          (*mqc).a <<= 1 as libc::c_int;
          (*mqc).c <<= 1 as libc::c_int;
          (*mqc).ct = (*mqc).ct.wrapping_sub(1);
          if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
            break;
          }
        }
      } else {
        v = (**(*mqc).curctx).mps
      }
    }
    let ref mut fresh108 =
      *datap.offset(ci.wrapping_mul(0 as libc::c_int as libc::c_uint) as isize);
    *fresh108 += if v
      ^ (*datap.offset(ci.wrapping_mul(0 as libc::c_int as libc::c_uint) as isize)
        < 0 as libc::c_int) as libc::c_int as libc::c_uint
      != 0
    {
      poshalf
    } else {
      -poshalf
    };
    *flagsp |= ((1 as libc::c_uint) << 20 as libc::c_int) << ci.wrapping_mul(3 as libc::c_uint)
  };
}
/* *
Encode refinement pass
*/
unsafe extern "C" fn opj_t1_enc_refpass(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
  mut nmsedec: *mut OPJ_INT32,
  mut type_0: OPJ_BYTE,
) {
  let mut i: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let one = (1 as libc::c_int) << bpno + (7 as libc::c_int - 1 as libc::c_int);
  let mut f: *mut opj_flag_t = &mut *(*t1).flags.offset(
    ((0 as libc::c_int + 1 as libc::c_int) as libc::c_uint).wrapping_add(
      ((0 as libc::c_int / 4 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
        .wrapping_mul((*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint)),
    ) as isize,
  ) as *mut opj_flag_t;
  let extra = 2 as libc::c_uint;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut datap: *const OPJ_INT32 = (*t1).data;
  *nmsedec = 0 as libc::c_int;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < (*t1).w {
      let flags = *f;
      let mut flagsUpdated = flags;
      if !(flags
        & ((1 as libc::c_uint) << 4 as libc::c_int
          | (1 as libc::c_uint) << 7 as libc::c_int
          | (1 as libc::c_uint) << 10 as libc::c_int
          | (1 as libc::c_uint) << 13 as libc::c_int)
        == 0 as libc::c_int as libc::c_uint)
      {
        if !(flags
          & ((1 as libc::c_uint) << 21 as libc::c_int
            | (1 as libc::c_uint) << 24 as libc::c_int
            | (1 as libc::c_uint) << 27 as libc::c_int
            | (1 as libc::c_uint) << 30 as libc::c_int)
          == (1 as libc::c_uint) << 21 as libc::c_int
            | (1 as libc::c_uint) << 24 as libc::c_int
            | (1 as libc::c_uint) << 27 as libc::c_int
            | (1 as libc::c_uint) << 30 as libc::c_int)
        {
          let mut v: OPJ_UINT32 = 0;
          if flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == ((1 as libc::c_uint) << 4 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          {
            let shift_flags =
              flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint);
            let mut ctxt = opj_t1_getctxno_mag(shift_flags);
            let mut abs_data =
              *datap.offset(0 as libc::c_int as isize) as OPJ_UINT32 & 0x7fffffff as libc::c_uint;
            *nmsedec += opj_t1_getnmsedec_ref(abs_data, bpno as OPJ_UINT32) as libc::c_int;
            v = if abs_data as OPJ_INT32 & one != 0 {
              1 as libc::c_int
            } else {
              0 as libc::c_int
            } as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt as isize) as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else if (**curctx).mps == v {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
                *curctx = (**curctx).nmps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
            } else {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a < (**curctx).qeval {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              } else {
                a = (**curctx).qeval
              }
              *curctx = (**curctx).nlps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            }
            flagsUpdated |= ((1 as libc::c_uint) << 20 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          }
          let mut v_0: OPJ_UINT32 = 0;
          if flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == ((1 as libc::c_uint) << 4 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          {
            let shift_flags_0 =
              flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint);
            let mut ctxt_0 = opj_t1_getctxno_mag(shift_flags_0);
            let mut abs_data_0 =
              *datap.offset(1 as libc::c_int as isize) as OPJ_UINT32 & 0x7fffffff as libc::c_uint;
            *nmsedec += opj_t1_getnmsedec_ref(abs_data_0, bpno as OPJ_UINT32) as libc::c_int;
            v_0 = if abs_data_0 as OPJ_INT32 & one != 0 {
              1 as libc::c_int
            } else {
              0 as libc::c_int
            } as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_0 as isize) as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v_0 << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else if (**curctx).mps == v_0 {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
                *curctx = (**curctx).nmps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
            } else {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a < (**curctx).qeval {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              } else {
                a = (**curctx).qeval
              }
              *curctx = (**curctx).nlps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            }
            flagsUpdated |= ((1 as libc::c_uint) << 20 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          }
          let mut v_1: OPJ_UINT32 = 0;
          if flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == ((1 as libc::c_uint) << 4 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          {
            let shift_flags_1 =
              flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint);
            let mut ctxt_1 = opj_t1_getctxno_mag(shift_flags_1);
            let mut abs_data_1 =
              *datap.offset(2 as libc::c_int as isize) as OPJ_UINT32 & 0x7fffffff as libc::c_uint;
            *nmsedec += opj_t1_getnmsedec_ref(abs_data_1, bpno as OPJ_UINT32) as libc::c_int;
            v_1 = if abs_data_1 as OPJ_INT32 & one != 0 {
              1 as libc::c_int
            } else {
              0 as libc::c_int
            } as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_1 as isize) as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v_1 << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else if (**curctx).mps == v_1 {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
                *curctx = (**curctx).nmps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
            } else {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a < (**curctx).qeval {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              } else {
                a = (**curctx).qeval
              }
              *curctx = (**curctx).nlps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            }
            flagsUpdated |= ((1 as libc::c_uint) << 20 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          }
          let mut v_2: OPJ_UINT32 = 0;
          if flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == ((1 as libc::c_uint) << 4 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          {
            let shift_flags_2 =
              flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint);
            let mut ctxt_2 = opj_t1_getctxno_mag(shift_flags_2);
            let mut abs_data_2 =
              *datap.offset(3 as libc::c_int as isize) as OPJ_UINT32 & 0x7fffffff as libc::c_uint;
            *nmsedec += opj_t1_getnmsedec_ref(abs_data_2, bpno as OPJ_UINT32) as libc::c_int;
            v_2 = if abs_data_2 as OPJ_INT32 & one != 0 {
              1 as libc::c_int
            } else {
              0 as libc::c_int
            } as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_2 as isize) as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v_2 << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else if (**curctx).mps == v_2 {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
                *curctx = (**curctx).nmps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
            } else {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a < (**curctx).qeval {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              } else {
                a = (**curctx).qeval
              }
              *curctx = (**curctx).nlps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            }
            flagsUpdated |= ((1 as libc::c_uint) << 20 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          }
          *f = flagsUpdated
        }
      }
      /* all processed by sigpass */
      i = i.wrapping_add(1);
      f = f.offset(1);
      datap = datap.offset(4 as libc::c_int as isize)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    f = f.offset(extra as isize)
  }
  if k < (*t1).h {
    let mut j: OPJ_UINT32 = 0;
    let remaining_lines = (*t1).h.wrapping_sub(k);
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < (*t1).w {
      if *f
        & ((1 as libc::c_uint) << 4 as libc::c_int
          | (1 as libc::c_uint) << 7 as libc::c_int
          | (1 as libc::c_uint) << 10 as libc::c_int
          | (1 as libc::c_uint) << 13 as libc::c_int)
        == 0 as libc::c_int as libc::c_uint
      {
        /* none significant */
        datap = datap.offset(remaining_lines as isize)
      } else {
        j = 0 as libc::c_int as OPJ_UINT32;
        while j < remaining_lines {
          let mut v_3: OPJ_UINT32 = 0;
          if *f
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << j.wrapping_mul(3 as libc::c_uint)
            == ((1 as libc::c_uint) << 4 as libc::c_int) << j.wrapping_mul(3 as libc::c_uint)
          {
            let shift_flags_3 = *f >> j.wrapping_mul(3 as libc::c_uint);
            let mut ctxt_3 = opj_t1_getctxno_mag(shift_flags_3);
            let mut abs_data_3 =
              *datap.offset(0 as libc::c_int as isize) as OPJ_UINT32 & 0x7fffffff as libc::c_uint;
            *nmsedec += opj_t1_getnmsedec_ref(abs_data_3, bpno as OPJ_UINT32) as libc::c_int;
            v_3 = if abs_data_3 as OPJ_INT32 & one != 0 {
              1 as libc::c_int
            } else {
              0 as libc::c_int
            } as OPJ_UINT32;
            curctx =
              &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_3 as isize) as *mut *const opj_mqc_state_t;
            if type_0 as libc::c_int == 1 as libc::c_int {
              if ct == 0xdeadbeef as libc::c_uint {
                ct = 8 as libc::c_int as OPJ_UINT32
              }
              ct = ct.wrapping_sub(1);
              c = c.wrapping_add(v_3 << ct);
              if ct == 0 as libc::c_int as libc::c_uint {
                *(*mqc).bp = c as OPJ_BYTE;
                ct = 8 as libc::c_int as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
                (*mqc).bp = (*mqc).bp.offset(1);
                c = 0 as libc::c_int as OPJ_UINT32
              }
            } else if (**curctx).mps == v_3 {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
                *curctx = (**curctx).nmps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
            } else {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a < (**curctx).qeval {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              } else {
                a = (**curctx).qeval
              }
              *curctx = (**curctx).nlps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            }
            *f |= ((1 as libc::c_uint) << 20 as libc::c_int) << j.wrapping_mul(3 as libc::c_uint)
          }
          j = j.wrapping_add(1);
          datap = datap.offset(1)
        }
      }
      i = i.wrapping_add(1);
      f = f.offset(1)
    }
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
}
/* *
Decode refinement pass
*/
unsafe extern "C" fn opj_t1_dec_refpass_raw(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  let mut one: OPJ_INT32 = 0;
  let mut poshalf: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1).flags.offset(
    ((0 as libc::c_int + 1 as libc::c_int) as libc::c_uint).wrapping_add(
      ((0 as libc::c_int / 4 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
        .wrapping_mul((*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint)),
    ) as isize,
  ) as *mut opj_flag_t;
  let l_w = (*t1).w;
  one = (1 as libc::c_int) << bpno;
  poshalf = one >> 1 as libc::c_int;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags != 0 as libc::c_int as libc::c_uint {
        opj_t1_dec_refpass_step_raw(t1, flagsp, data, poshalf, 0 as libc::c_uint);
        opj_t1_dec_refpass_step_raw(
          t1,
          flagsp,
          data.offset(l_w as isize),
          poshalf,
          1 as libc::c_uint,
        );
        opj_t1_dec_refpass_step_raw(
          t1,
          flagsp,
          data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize),
          poshalf,
          2 as libc::c_uint,
        );
        opj_t1_dec_refpass_step_raw(
          t1,
          flagsp,
          data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize),
          poshalf,
          3 as libc::c_uint,
        );
      }
      i = i.wrapping_add(1);
      flagsp = flagsp.offset(1);
      data = data.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    flagsp = flagsp.offset(2 as libc::c_int as isize);
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
  }
  if k < (*t1).h {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (*t1).h.wrapping_sub(k) {
        opj_t1_dec_refpass_step_raw(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          poshalf,
          j,
        );
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1);
      flagsp = flagsp.offset(1);
      data = data.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_refpass_mqc_64x64(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  let mut one: OPJ_INT32 = 0;
  let mut poshalf: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1)
    .flags
    .offset((66 as libc::c_int + 1 as libc::c_int) as isize)
    as *mut opj_flag_t;
  let l_w = 64 as libc::c_int as OPJ_UINT32;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  poshalf = one >> 1 as libc::c_int;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < 64 as libc::c_int as libc::c_uint & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags != 0 as libc::c_int as libc::c_uint {
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == ((1 as libc::c_uint) << 4 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        {
          let mut ctxt = opj_t1_getctxno_mag(
            flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          );
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c: OPJ_UINT32 = 0;
                l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_0: OPJ_UINT32 = 0;
                  l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          let ref mut fresh109 =
            *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
          *fresh109 += if v
            ^ (*data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
              < 0 as libc::c_int) as libc::c_int as libc::c_uint
            != 0
          {
            poshalf
          } else {
            -poshalf
          };
          flags |= ((1 as libc::c_uint) << 20 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == ((1 as libc::c_uint) << 4 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        {
          let mut ctxt_0 = opj_t1_getctxno_mag(
            flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          );
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_0 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_1: OPJ_UINT32 = 0;
                l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_2: OPJ_UINT32 = 0;
                  l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          let ref mut fresh110 =
            *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
          *fresh110 += if v
            ^ (*data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
              < 0 as libc::c_int) as libc::c_int as libc::c_uint
            != 0
          {
            poshalf
          } else {
            -poshalf
          };
          flags |= ((1 as libc::c_uint) << 20 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == ((1 as libc::c_uint) << 4 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        {
          let mut ctxt_1 = opj_t1_getctxno_mag(
            flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          );
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          let ref mut fresh111 =
            *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
          *fresh111 += if v
            ^ (*data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
              < 0 as libc::c_int) as libc::c_int as libc::c_uint
            != 0
          {
            poshalf
          } else {
            -poshalf
          };
          flags |= ((1 as libc::c_uint) << 20 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == ((1 as libc::c_uint) << 4 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        {
          let mut ctxt_2 = opj_t1_getctxno_mag(
            flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          );
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_2 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_5: OPJ_UINT32 = 0;
                l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_6: OPJ_UINT32 = 0;
                  l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          let ref mut fresh112 =
            *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
          *fresh112 += if v
            ^ (*data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
              < 0 as libc::c_int) as libc::c_int as libc::c_uint
            != 0
          {
            poshalf
          } else {
            -poshalf
          };
          flags |= ((1 as libc::c_uint) << 20 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        *flagsp = flags
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < 64 as libc::c_int as libc::c_uint {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (64 as libc::c_int as libc::c_uint).wrapping_sub(k) {
        opj_t1_dec_refpass_step_mqc(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          poshalf,
          j,
        );
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_refpass_mqc_generic(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  let mut one: OPJ_INT32 = 0;
  let mut poshalf: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1).flags.offset(
    (*t1)
      .w
      .wrapping_add(2 as libc::c_uint)
      .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
  ) as *mut opj_flag_t;
  let l_w = (*t1).w;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  poshalf = one >> 1 as libc::c_int;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags != 0 as libc::c_int as libc::c_uint {
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == ((1 as libc::c_uint) << 4 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        {
          let mut ctxt = opj_t1_getctxno_mag(
            flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          );
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c: OPJ_UINT32 = 0;
                l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_0: OPJ_UINT32 = 0;
                  l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          let ref mut fresh113 =
            *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
          *fresh113 += if v
            ^ (*data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
              < 0 as libc::c_int) as libc::c_int as libc::c_uint
            != 0
          {
            poshalf
          } else {
            -poshalf
          };
          flags |= ((1 as libc::c_uint) << 20 as libc::c_int)
            << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == ((1 as libc::c_uint) << 4 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        {
          let mut ctxt_0 = opj_t1_getctxno_mag(
            flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          );
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_0 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_1: OPJ_UINT32 = 0;
                l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_2: OPJ_UINT32 = 0;
                  l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          let ref mut fresh114 =
            *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
          *fresh114 += if v
            ^ (*data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
              < 0 as libc::c_int) as libc::c_int as libc::c_uint
            != 0
          {
            poshalf
          } else {
            -poshalf
          };
          flags |= ((1 as libc::c_uint) << 20 as libc::c_int)
            << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == ((1 as libc::c_uint) << 4 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        {
          let mut ctxt_1 = opj_t1_getctxno_mag(
            flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          );
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_1 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          let ref mut fresh115 =
            *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
          *fresh115 += if v
            ^ (*data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
              < 0 as libc::c_int) as libc::c_int as libc::c_uint
            != 0
          {
            poshalf
          } else {
            -poshalf
          };
          flags |= ((1 as libc::c_uint) << 20 as libc::c_int)
            << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        if flags
          & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
          == ((1 as libc::c_uint) << 4 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        {
          let mut ctxt_2 = opj_t1_getctxno_mag(
            flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
          );
          curctx =
            &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt_2 as isize) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_5: OPJ_UINT32 = 0;
                l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_6: OPJ_UINT32 = 0;
                  l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          let ref mut fresh116 =
            *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
          *fresh116 += if v
            ^ (*data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize)
              < 0 as libc::c_int) as libc::c_int as libc::c_uint
            != 0
          {
            poshalf
          } else {
            -poshalf
          };
          flags |= ((1 as libc::c_uint) << 20 as libc::c_int)
            << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
        }
        *flagsp = flags
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < (*t1).h {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (*t1).h.wrapping_sub(k) {
        opj_t1_dec_refpass_step_mqc(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          poshalf,
          j,
        );
        j = j.wrapping_add(1)
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_refpass_mqc(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  if (*t1).w == 64 as libc::c_int as libc::c_uint && (*t1).h == 64 as libc::c_int as libc::c_uint {
    opj_t1_dec_refpass_mqc_64x64(t1, bpno);
  } else {
    opj_t1_dec_refpass_mqc_generic(t1, bpno);
  };
}
/* *
Decode clean-up pass
*/
/* *
Encode clean-up pass step
*/
unsafe extern "C" fn opj_t1_dec_clnpass_step(
  mut t1: *mut opj_t1_t,
  mut flagsp: *mut opj_flag_t,
  mut datap: *mut OPJ_INT32,
  mut oneplushalf: OPJ_INT32,
  mut ci: OPJ_UINT32,
  mut vsc: OPJ_UINT32,
) {
  let mut v: OPJ_UINT32 = 0; /* MQC component */
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  if 1 as libc::c_int == 0
    || *flagsp
      & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
        << ci.wrapping_mul(3 as libc::c_uint)
      == 0
  {
    let mut current_block_189: u64;
    if 0 as libc::c_int == 0 {
      let mut ctxt1 =
        opj_t1_getctxno_zc(mqc, *flagsp >> ci.wrapping_mul(3 as libc::c_uint)) as OPJ_UINT32;
      (*mqc).curctx =
        &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize) as *mut *const opj_mqc_state_t;
      (*mqc).a = ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32
        as OPJ_UINT32;
      if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
        if (*mqc).a < (**(*mqc).curctx).qeval {
          (*mqc).a = (**(*mqc).curctx).qeval;
          v = (**(*mqc).curctx).mps;
          *(*mqc).curctx = (**(*mqc).curctx).nmps
        } else {
          (*mqc).a = (**(*mqc).curctx).qeval;
          v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
          *(*mqc).curctx = (**(*mqc).curctx).nlps
        }
        loop {
          if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
            let mut l_c: OPJ_UINT32 = 0;
            l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
              if l_c > 0x8f as libc::c_int as libc::c_uint {
                (*mqc).c = ((*mqc).c as libc::c_uint)
                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                (*mqc).end_of_byte_stream_counter =
                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
              }
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
            }
          }
          (*mqc).a <<= 1 as libc::c_int;
          (*mqc).c <<= 1 as libc::c_int;
          (*mqc).ct = (*mqc).ct.wrapping_sub(1);
          if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
            break;
          }
        }
      } else {
        (*mqc).c = ((*mqc).c as libc::c_uint)
          .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int)
          as OPJ_UINT32 as OPJ_UINT32;
        if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
          if (*mqc).a < (**(*mqc).curctx).qeval {
            v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
            *(*mqc).curctx = (**(*mqc).curctx).nlps
          } else {
            v = (**(*mqc).curctx).mps;
            *(*mqc).curctx = (**(*mqc).curctx).nmps
          }
          loop {
            if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
              let mut l_c_0: OPJ_UINT32 = 0;
              l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                  (*mqc).c = ((*mqc).c as libc::c_uint)
                    .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                    as OPJ_UINT32 as OPJ_UINT32;
                  (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                  (*mqc).end_of_byte_stream_counter =
                    (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int)
                    as OPJ_UINT32 as OPJ_UINT32;
                  (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
                }
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
              }
            }
            (*mqc).a <<= 1 as libc::c_int;
            (*mqc).c <<= 1 as libc::c_int;
            (*mqc).ct = (*mqc).ct.wrapping_sub(1);
            if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
              break;
            }
          }
        } else {
          v = (**(*mqc).curctx).mps
        }
      }
      if v == 0 {
        current_block_189 = 16708048892964637133;
      } else {
        current_block_189 = 15855550149339537395;
      }
    } else {
      current_block_189 = 15855550149339537395;
    }
    match current_block_189 {
      15855550149339537395 => {
        let mut lu = opj_t1_getctxtno_sc_or_spb_index(
          *flagsp,
          *flagsp.offset(-(1 as libc::c_int) as isize),
          *flagsp.offset(1 as libc::c_int as isize),
          ci,
        );
        (*mqc).curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
          as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
          lu
        ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
        (*mqc).a = ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32
          as OPJ_UINT32;
        if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
          if (*mqc).a < (**(*mqc).curctx).qeval {
            (*mqc).a = (**(*mqc).curctx).qeval;
            v = (**(*mqc).curctx).mps;
            *(*mqc).curctx = (**(*mqc).curctx).nmps
          } else {
            (*mqc).a = (**(*mqc).curctx).qeval;
            v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
            *(*mqc).curctx = (**(*mqc).curctx).nlps
          }
          loop {
            if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
              let mut l_c_1: OPJ_UINT32 = 0;
              l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                  (*mqc).c = ((*mqc).c as libc::c_uint)
                    .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                    as OPJ_UINT32 as OPJ_UINT32;
                  (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                  (*mqc).end_of_byte_stream_counter =
                    (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int)
                    as OPJ_UINT32 as OPJ_UINT32;
                  (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
                }
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
              }
            }
            (*mqc).a <<= 1 as libc::c_int;
            (*mqc).c <<= 1 as libc::c_int;
            (*mqc).ct = (*mqc).ct.wrapping_sub(1);
            if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
              break;
            }
          }
        } else {
          (*mqc).c = ((*mqc).c as libc::c_uint)
            .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int)
            as OPJ_UINT32 as OPJ_UINT32;
          if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            if (*mqc).a < (**(*mqc).curctx).qeval {
              v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *(*mqc).curctx = (**(*mqc).curctx).nlps
            } else {
              v = (**(*mqc).curctx).mps;
              *(*mqc).curctx = (**(*mqc).curctx).nmps
            }
            loop {
              if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_2: OPJ_UINT32 = 0;
                l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                    (*mqc).c = ((*mqc).c as libc::c_uint)
                      .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int)
                    as OPJ_UINT32 as OPJ_UINT32;
                  (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              (*mqc).a <<= 1 as libc::c_int;
              (*mqc).c <<= 1 as libc::c_int;
              (*mqc).ct = (*mqc).ct.wrapping_sub(1);
              if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            v = (**(*mqc).curctx).mps
          }
        }
        v = v ^ opj_t1_getspb(lu) as libc::c_uint;
        *datap.offset(ci.wrapping_mul(0 as libc::c_int as libc::c_uint) as isize) =
          if v != 0 { -oneplushalf } else { oneplushalf };
        let ref mut fresh117 = *flagsp.offset(-(1 as libc::c_int) as isize);
        *fresh117 |=
          ((1 as libc::c_uint) << 5 as libc::c_int) << (3 as libc::c_uint).wrapping_mul(ci);
        *flagsp |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
          << (3 as libc::c_uint).wrapping_mul(ci);
        let ref mut fresh118 = *flagsp.offset(1 as libc::c_int as isize);
        *fresh118 |=
          ((1 as libc::c_uint) << 3 as libc::c_int) << (3 as libc::c_uint).wrapping_mul(ci);
        if ci == 0 as libc::c_uint && vsc == 0 {
          let mut north = flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
          *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
          let ref mut fresh119 = *north.offset(-(1 as libc::c_int) as isize);
          *fresh119 |= (1 as libc::c_uint) << 17 as libc::c_int;
          let ref mut fresh120 = *north.offset(1 as libc::c_int as isize);
          *fresh120 |= (1 as libc::c_uint) << 15 as libc::c_int
        }
        if ci == 3 as libc::c_uint {
          let mut south = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
          *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
          let ref mut fresh121 = *south.offset(-(1 as libc::c_int) as isize);
          *fresh121 |= (1 as libc::c_uint) << 2 as libc::c_int;
          let ref mut fresh122 = *south.offset(1 as libc::c_int as isize);
          *fresh122 |= (1 as libc::c_uint) << 0 as libc::c_int
        }
      }
      _ => {}
    }
  };
}
/* *
Encode clean-up pass
*/
unsafe extern "C" fn opj_t1_enc_clnpass(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
  mut nmsedec: *mut OPJ_INT32,
  mut cblksty: OPJ_UINT32,
) {
  let mut i: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let one = (1 as libc::c_int) << bpno + (7 as libc::c_int - 1 as libc::c_int);
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut datap: *const OPJ_INT32 = (*t1).data;
  let mut f: *mut opj_flag_t = &mut *(*t1).flags.offset(
    ((0 as libc::c_int + 1 as libc::c_int) as libc::c_uint).wrapping_add(
      ((0 as libc::c_int / 4 as libc::c_int + 1 as libc::c_int) as libc::c_uint)
        .wrapping_mul((*t1).w.wrapping_add(2 as libc::c_int as libc::c_uint)),
    ) as isize,
  ) as *mut opj_flag_t;
  let extra = 2 as libc::c_uint;
  *nmsedec = 0 as libc::c_int;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    let mut current_block_315: u64;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < (*t1).w {
      let mut agg: OPJ_UINT32 = 0;
      let mut runlen: OPJ_UINT32 = 0;
      agg = (*f == 0) as libc::c_int as OPJ_UINT32;
      if agg != 0 {
        runlen = 0 as libc::c_int as OPJ_UINT32;
        while runlen < 4 as libc::c_int as libc::c_uint {
          if *datap as OPJ_UINT32 & 0x7fffffff as libc::c_uint & one as OPJ_UINT32 != 0 {
            break;
          }
          runlen = runlen.wrapping_add(1);
          datap = datap.offset(1)
        }
        curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
          (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int) as OPJ_UINT32
            as isize,
        ) as *mut *const opj_mqc_state_t;
        if (**curctx).mps
          == (runlen != 4 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint
        {
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            if a < (**curctx).qeval {
              a = (**curctx).qeval
            } else {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            }
            *curctx = (**curctx).nmps;
            loop {
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if ct == 0 as libc::c_int as libc::c_uint {
                (*mqc).c = c;
                opj_mqc_byteout(mqc);
                c = (*mqc).c;
                ct = (*mqc).ct
              }
              if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
          }
        } else {
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if a < (**curctx).qeval {
            c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
          } else {
            a = (**curctx).qeval
          }
          *curctx = (**curctx).nlps;
          loop {
            a <<= 1 as libc::c_int;
            c <<= 1 as libc::c_int;
            ct = ct.wrapping_sub(1);
            if ct == 0 as libc::c_int as libc::c_uint {
              (*mqc).c = c;
              opj_mqc_byteout(mqc);
              c = (*mqc).c;
              ct = (*mqc).ct
            }
            if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
              break;
            }
          }
        }
        if runlen == 4 as libc::c_int as libc::c_uint {
          current_block_315 = 1394248824506584008;
        } else {
          curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
            (0 as libc::c_int
              + 9 as libc::c_int
              + 5 as libc::c_int
              + 3 as libc::c_int
              + 1 as libc::c_int) as OPJ_UINT32 as isize,
          ) as *mut *const opj_mqc_state_t;
          if (**curctx).mps == runlen >> 1 as libc::c_int {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                a = (**curctx).qeval
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
              *curctx = (**curctx).nmps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            }
          } else {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a < (**curctx).qeval {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            } else {
              a = (**curctx).qeval
            }
            *curctx = (**curctx).nlps;
            loop {
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if ct == 0 as libc::c_int as libc::c_uint {
                (*mqc).c = c;
                opj_mqc_byteout(mqc);
                c = (*mqc).c;
                ct = (*mqc).ct
              }
              if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          }
          if (**curctx).mps == runlen & 1 as libc::c_int as libc::c_uint {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                a = (**curctx).qeval
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
              *curctx = (**curctx).nmps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            }
          } else {
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if a < (**curctx).qeval {
              c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
            } else {
              a = (**curctx).qeval
            }
            *curctx = (**curctx).nlps;
            loop {
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if ct == 0 as libc::c_int as libc::c_uint {
                (*mqc).c = c;
                opj_mqc_byteout(mqc);
                c = (*mqc).c;
                ct = (*mqc).ct
              }
              if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          }
          current_block_315 = 3297745280902459415;
        }
      } else {
        runlen = 0 as libc::c_int as OPJ_UINT32;
        current_block_315 = 3297745280902459415;
      }
      match current_block_315 {
        3297745280902459415 => {
          let mut v: OPJ_UINT32 = 0;
          let mut ci: OPJ_UINT32 = 0;
          let flagsp = f;
          let mut l_datap = datap;
          let check = (1 as libc::c_uint) << 4 as libc::c_int
            | (1 as libc::c_uint) << 7 as libc::c_int
            | (1 as libc::c_uint) << 10 as libc::c_int
            | (1 as libc::c_uint) << 13 as libc::c_int
            | (1 as libc::c_uint) << 21 as libc::c_int
            | (1 as libc::c_uint) << 24 as libc::c_int
            | (1 as libc::c_uint) << 27 as libc::c_int
            | (1 as libc::c_uint) << 30 as libc::c_int;
          if *flagsp & check == check {
            if runlen == 0 as libc::c_int as libc::c_uint {
              *flagsp &= !((1 as libc::c_uint) << 21 as libc::c_int
                | (1 as libc::c_uint) << 24 as libc::c_int
                | (1 as libc::c_uint) << 27 as libc::c_int
                | (1 as libc::c_uint) << 30 as libc::c_int)
            } else if runlen == 1 as libc::c_int as libc::c_uint {
              *flagsp &= !((1 as libc::c_uint) << 24 as libc::c_int
                | (1 as libc::c_uint) << 27 as libc::c_int
                | (1 as libc::c_uint) << 30 as libc::c_int)
            } else if runlen == 2 as libc::c_int as libc::c_uint {
              *flagsp &= !((1 as libc::c_uint) << 27 as libc::c_int
                | (1 as libc::c_uint) << 30 as libc::c_int)
            } else if runlen == 3 as libc::c_int as libc::c_uint {
              *flagsp &= !((1 as libc::c_uint) << 30 as libc::c_int)
            }
          } else {
            ci = runlen;
            while ci < 4 as libc::c_uint {
              let mut goto_PARTIAL = 0 as libc::c_int;
              if agg != 0 as libc::c_int as libc::c_uint && ci == runlen {
                goto_PARTIAL = 1 as libc::c_int
              } else if *flagsp
                & ((1 as libc::c_uint) << 4 as libc::c_int
                  | (1 as libc::c_uint) << 21 as libc::c_int)
                  << ci.wrapping_mul(3 as libc::c_uint)
                == 0
              {
                let mut ctxt1 =
                  opj_t1_getctxno_zc(mqc, *flagsp >> ci.wrapping_mul(3 as libc::c_uint))
                    as OPJ_UINT32;
                curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize)
                  as *mut *const opj_mqc_state_t;
                v = if *l_datap as OPJ_UINT32 & 0x7fffffff as libc::c_uint & one as OPJ_UINT32 != 0
                {
                  1 as libc::c_int
                } else {
                  0 as libc::c_int
                } as OPJ_UINT32;
                if (**curctx).mps == v {
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval
                    } else {
                      c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32
                        as OPJ_UINT32
                    }
                    *curctx = (**curctx).nmps;
                    loop {
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if ct == 0 as libc::c_int as libc::c_uint {
                        (*mqc).c = c;
                        opj_mqc_byteout(mqc);
                        c = (*mqc).c;
                        ct = (*mqc).ct
                      }
                      if !(a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint)
                      {
                        break;
                      }
                    }
                  } else {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  }
                } else {
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if a < (**curctx).qeval {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  } else {
                    a = (**curctx).qeval
                  }
                  *curctx = (**curctx).nlps;
                  loop {
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if ct == 0 as libc::c_int as libc::c_uint {
                      (*mqc).c = c;
                      opj_mqc_byteout(mqc);
                      c = (*mqc).c;
                      ct = (*mqc).ct
                    }
                    if !(a & 0x8000 as libc::c_int as libc::c_uint
                      == 0 as libc::c_int as libc::c_uint)
                    {
                      break;
                    }
                  }
                }
                if v != 0 {
                  goto_PARTIAL = 1 as libc::c_int
                }
              }
              if goto_PARTIAL != 0 {
                let mut vsc: OPJ_UINT32 = 0;
                let mut ctxt2: OPJ_UINT32 = 0;
                let mut spb: OPJ_UINT32 = 0;
                let mut lu = opj_t1_getctxtno_sc_or_spb_index(
                  *flagsp,
                  *flagsp.offset(-(1 as libc::c_int) as isize),
                  *flagsp.offset(1 as libc::c_int as isize),
                  ci,
                );
                *nmsedec += opj_t1_getnmsedec_sig(
                  *l_datap as OPJ_UINT32 & 0x7fffffff as libc::c_uint,
                  bpno as OPJ_UINT32,
                ) as libc::c_int;
                ctxt2 = opj_t1_getctxno_sc(lu) as OPJ_UINT32;
                curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2 as isize)
                  as *mut *const opj_mqc_state_t;
                v = *l_datap as OPJ_UINT32 >> 31 as libc::c_int;
                spb = opj_t1_getspb(lu) as OPJ_UINT32;
                if (**curctx).mps == v ^ spb {
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval
                    } else {
                      c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32
                        as OPJ_UINT32
                    }
                    *curctx = (**curctx).nmps;
                    loop {
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if ct == 0 as libc::c_int as libc::c_uint {
                        (*mqc).c = c;
                        opj_mqc_byteout(mqc);
                        c = (*mqc).c;
                        ct = (*mqc).ct
                      }
                      if !(a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint)
                      {
                        break;
                      }
                    }
                  } else {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  }
                } else {
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if a < (**curctx).qeval {
                    c =
                      (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                  } else {
                    a = (**curctx).qeval
                  }
                  *curctx = (**curctx).nlps;
                  loop {
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if ct == 0 as libc::c_int as libc::c_uint {
                      (*mqc).c = c;
                      opj_mqc_byteout(mqc);
                      c = (*mqc).c;
                      ct = (*mqc).ct
                    }
                    if !(a & 0x8000 as libc::c_int as libc::c_uint
                      == 0 as libc::c_int as libc::c_uint)
                    {
                      break;
                    }
                  }
                }
                vsc = if cblksty & 0x8 as libc::c_int as libc::c_uint != 0
                  && ci == 0 as libc::c_int as libc::c_uint
                {
                  1 as libc::c_int
                } else {
                  0 as libc::c_int
                } as OPJ_UINT32;
                opj_t1_update_flags(flagsp, ci, v, (*t1).w.wrapping_add(2 as libc::c_uint), vsc);
              }
              *flagsp &= !(((1 as libc::c_uint) << 21 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(ci));
              l_datap = l_datap.offset(1);
              ci = ci.wrapping_add(1)
            }
          }
          datap = datap.offset((4 as libc::c_int as libc::c_uint).wrapping_sub(runlen) as isize)
        }
        _ => {}
      }
      i = i.wrapping_add(1);
      f = f.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    f = f.offset(extra as isize)
  }
  if k < (*t1).h {
    let agg_0 = 0 as libc::c_int as OPJ_UINT32;
    let runlen_0 = 0 as libc::c_int as OPJ_UINT32;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < (*t1).w {
      let mut v_0: OPJ_UINT32 = 0;
      let mut ci_0: OPJ_UINT32 = 0;
      let flagsp_0 = f;
      let mut l_datap_0 = datap;
      let check_0 = (1 as libc::c_uint) << 4 as libc::c_int
        | (1 as libc::c_uint) << 7 as libc::c_int
        | (1 as libc::c_uint) << 10 as libc::c_int
        | (1 as libc::c_uint) << 13 as libc::c_int
        | (1 as libc::c_uint) << 21 as libc::c_int
        | (1 as libc::c_uint) << 24 as libc::c_int
        | (1 as libc::c_uint) << 27 as libc::c_int
        | (1 as libc::c_uint) << 30 as libc::c_int;
      if *flagsp_0 & check_0 == check_0 {
        if runlen_0 == 0 as libc::c_int as libc::c_uint {
          *flagsp_0 &= !((1 as libc::c_uint) << 21 as libc::c_int
            | (1 as libc::c_uint) << 24 as libc::c_int
            | (1 as libc::c_uint) << 27 as libc::c_int
            | (1 as libc::c_uint) << 30 as libc::c_int)
        } else if runlen_0 == 1 as libc::c_int as libc::c_uint {
          *flagsp_0 &= !((1 as libc::c_uint) << 24 as libc::c_int
            | (1 as libc::c_uint) << 27 as libc::c_int
            | (1 as libc::c_uint) << 30 as libc::c_int)
        } else if runlen_0 == 2 as libc::c_int as libc::c_uint {
          *flagsp_0 &=
            !((1 as libc::c_uint) << 27 as libc::c_int | (1 as libc::c_uint) << 30 as libc::c_int)
        } else if runlen_0 == 3 as libc::c_int as libc::c_uint {
          *flagsp_0 &= !((1 as libc::c_uint) << 30 as libc::c_int)
        }
      } else {
        ci_0 = runlen_0;
        while ci_0 < (*t1).h.wrapping_sub(k) {
          let mut goto_PARTIAL_0 = 0 as libc::c_int;
          if agg_0 != 0 as libc::c_int as libc::c_uint && ci_0 == runlen_0 {
            goto_PARTIAL_0 = 1 as libc::c_int
          } else if *flagsp_0
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << ci_0.wrapping_mul(3 as libc::c_uint)
            == 0
          {
            let mut ctxt1_0 =
              opj_t1_getctxno_zc(mqc, *flagsp_0 >> ci_0.wrapping_mul(3 as libc::c_uint))
                as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize)
              as *mut *const opj_mqc_state_t;
            v_0 = if *l_datap_0 as OPJ_UINT32 & 0x7fffffff as libc::c_uint & one as OPJ_UINT32 != 0
            {
              1 as libc::c_int
            } else {
              0 as libc::c_int
            } as OPJ_UINT32;
            if (**curctx).mps == v_0 {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
                *curctx = (**curctx).nmps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
            } else {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a < (**curctx).qeval {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              } else {
                a = (**curctx).qeval
              }
              *curctx = (**curctx).nlps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            }
            if v_0 != 0 {
              goto_PARTIAL_0 = 1 as libc::c_int
            }
          }
          if goto_PARTIAL_0 != 0 {
            let mut vsc_0: OPJ_UINT32 = 0;
            let mut ctxt2_0: OPJ_UINT32 = 0;
            let mut spb_0: OPJ_UINT32 = 0;
            let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
              *flagsp_0,
              *flagsp_0.offset(-(1 as libc::c_int) as isize),
              *flagsp_0.offset(1 as libc::c_int as isize),
              ci_0,
            );
            *nmsedec += opj_t1_getnmsedec_sig(
              *l_datap_0 as OPJ_UINT32 & 0x7fffffff as libc::c_uint,
              bpno as OPJ_UINT32,
            ) as libc::c_int;
            ctxt2_0 = opj_t1_getctxno_sc(lu_0) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt2_0 as isize)
              as *mut *const opj_mqc_state_t;
            v_0 = *l_datap_0 as OPJ_UINT32 >> 31 as libc::c_int;
            spb_0 = opj_t1_getspb(lu_0) as OPJ_UINT32;
            if (**curctx).mps == v_0 ^ spb_0 {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval
                } else {
                  c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
                }
                *curctx = (**curctx).nmps;
                loop {
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if ct == 0 as libc::c_int as libc::c_uint {
                    (*mqc).c = c;
                    opj_mqc_byteout(mqc);
                    c = (*mqc).c;
                    ct = (*mqc).ct
                  }
                  if !(a & 0x8000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
                  {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              }
            } else {
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if a < (**curctx).qeval {
                c = (c as libc::c_uint).wrapping_add((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32
              } else {
                a = (**curctx).qeval
              }
              *curctx = (**curctx).nlps;
              loop {
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if ct == 0 as libc::c_int as libc::c_uint {
                  (*mqc).c = c;
                  opj_mqc_byteout(mqc);
                  c = (*mqc).c;
                  ct = (*mqc).ct
                }
                if !(a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint)
                {
                  break;
                }
              }
            }
            vsc_0 = if cblksty & 0x8 as libc::c_int as libc::c_uint != 0
              && ci_0 == 0 as libc::c_int as libc::c_uint
            {
              1 as libc::c_int
            } else {
              0 as libc::c_int
            } as OPJ_UINT32;
            opj_t1_update_flags(
              flagsp_0,
              ci_0,
              v_0,
              (*t1).w.wrapping_add(2 as libc::c_uint),
              vsc_0,
            );
          }
          *flagsp_0 &=
            !(((1 as libc::c_uint) << 21 as libc::c_int) << (3 as libc::c_uint).wrapping_mul(ci_0));
          l_datap_0 = l_datap_0.offset(1);
          ci_0 = ci_0.wrapping_add(1)
        }
      }
      datap = datap.offset((*t1).h.wrapping_sub(k) as isize);
      i = i.wrapping_add(1);
      f = f.offset(1)
    }
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
}
/* FALLTHRU */
/* FALLTHRU */
/* FALLTHRU */
unsafe extern "C" fn opj_t1_dec_clnpass_check_segsym(
  mut t1: *mut opj_t1_t,
  mut cblksty: OPJ_INT32,
) {
  if cblksty & 0x20 as libc::c_int != 0 {
    let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
    let mut v: OPJ_UINT32 = 0;
    let mut v2: OPJ_UINT32 = 0;
    (*mqc).curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
      (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int)
        as OPJ_UINT32 as isize,
    ) as *mut *const opj_mqc_state_t;
    (*mqc).a =
      ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
    if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
      if (*mqc).a < (**(*mqc).curctx).qeval {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v = (**(*mqc).curctx).mps;
        *(*mqc).curctx = (**(*mqc).curctx).nmps
      } else {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
        *(*mqc).curctx = (**(*mqc).curctx).nlps
      }
      loop {
        if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
          let mut l_c: OPJ_UINT32 = 0;
          l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
            if l_c > 0x8f as libc::c_int as libc::c_uint {
              (*mqc).c = ((*mqc).c as libc::c_uint)
                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
              (*mqc).end_of_byte_stream_counter = (*mqc).end_of_byte_stream_counter.wrapping_add(1)
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
            }
          } else {
            (*mqc).bp = (*mqc).bp.offset(1);
            (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
          }
        }
        (*mqc).a <<= 1 as libc::c_int;
        (*mqc).c <<= 1 as libc::c_int;
        (*mqc).ct = (*mqc).ct.wrapping_sub(1);
        if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
          break;
        }
      }
    } else {
      (*mqc).c = ((*mqc).c as libc::c_uint)
        .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
        as OPJ_UINT32;
      if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        if (*mqc).a < (**(*mqc).curctx).qeval {
          v = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
          *(*mqc).curctx = (**(*mqc).curctx).nlps
        } else {
          v = (**(*mqc).curctx).mps;
          *(*mqc).curctx = (**(*mqc).curctx).nmps
        }
        loop {
          if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
            let mut l_c_0: OPJ_UINT32 = 0;
            l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
              if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                (*mqc).c = ((*mqc).c as libc::c_uint)
                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                (*mqc).end_of_byte_stream_counter =
                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
              }
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
            }
          }
          (*mqc).a <<= 1 as libc::c_int;
          (*mqc).c <<= 1 as libc::c_int;
          (*mqc).ct = (*mqc).ct.wrapping_sub(1);
          if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
            break;
          }
        }
      } else {
        v = (**(*mqc).curctx).mps
      }
    }
    (*mqc).a =
      ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
    if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
      if (*mqc).a < (**(*mqc).curctx).qeval {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v2 = (**(*mqc).curctx).mps;
        *(*mqc).curctx = (**(*mqc).curctx).nmps
      } else {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v2 = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
        *(*mqc).curctx = (**(*mqc).curctx).nlps
      }
      loop {
        if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
          let mut l_c_1: OPJ_UINT32 = 0;
          l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
            if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
              (*mqc).c = ((*mqc).c as libc::c_uint)
                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
              (*mqc).end_of_byte_stream_counter = (*mqc).end_of_byte_stream_counter.wrapping_add(1)
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
            }
          } else {
            (*mqc).bp = (*mqc).bp.offset(1);
            (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
          }
        }
        (*mqc).a <<= 1 as libc::c_int;
        (*mqc).c <<= 1 as libc::c_int;
        (*mqc).ct = (*mqc).ct.wrapping_sub(1);
        if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
          break;
        }
      }
    } else {
      (*mqc).c = ((*mqc).c as libc::c_uint)
        .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
        as OPJ_UINT32;
      if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        if (*mqc).a < (**(*mqc).curctx).qeval {
          v2 = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
          *(*mqc).curctx = (**(*mqc).curctx).nlps
        } else {
          v2 = (**(*mqc).curctx).mps;
          *(*mqc).curctx = (**(*mqc).curctx).nmps
        }
        loop {
          if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
            let mut l_c_2: OPJ_UINT32 = 0;
            l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
              if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                (*mqc).c = ((*mqc).c as libc::c_uint)
                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                (*mqc).end_of_byte_stream_counter =
                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
              }
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
            }
          }
          (*mqc).a <<= 1 as libc::c_int;
          (*mqc).c <<= 1 as libc::c_int;
          (*mqc).ct = (*mqc).ct.wrapping_sub(1);
          if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
            break;
          }
        }
      } else {
        v2 = (**(*mqc).curctx).mps
      }
    }
    v = v << 1 as libc::c_int | v2;
    (*mqc).a =
      ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
    if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
      if (*mqc).a < (**(*mqc).curctx).qeval {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v2 = (**(*mqc).curctx).mps;
        *(*mqc).curctx = (**(*mqc).curctx).nmps
      } else {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v2 = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
        *(*mqc).curctx = (**(*mqc).curctx).nlps
      }
      loop {
        if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
          let mut l_c_3: OPJ_UINT32 = 0;
          l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
            if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
              (*mqc).c = ((*mqc).c as libc::c_uint)
                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
              (*mqc).end_of_byte_stream_counter = (*mqc).end_of_byte_stream_counter.wrapping_add(1)
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
            }
          } else {
            (*mqc).bp = (*mqc).bp.offset(1);
            (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
          }
        }
        (*mqc).a <<= 1 as libc::c_int;
        (*mqc).c <<= 1 as libc::c_int;
        (*mqc).ct = (*mqc).ct.wrapping_sub(1);
        if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
          break;
        }
      }
    } else {
      (*mqc).c = ((*mqc).c as libc::c_uint)
        .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
        as OPJ_UINT32;
      if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        if (*mqc).a < (**(*mqc).curctx).qeval {
          v2 = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
          *(*mqc).curctx = (**(*mqc).curctx).nlps
        } else {
          v2 = (**(*mqc).curctx).mps;
          *(*mqc).curctx = (**(*mqc).curctx).nmps
        }
        loop {
          if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
            let mut l_c_4: OPJ_UINT32 = 0;
            l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
              if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                (*mqc).c = ((*mqc).c as libc::c_uint)
                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                (*mqc).end_of_byte_stream_counter =
                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
              }
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
            }
          }
          (*mqc).a <<= 1 as libc::c_int;
          (*mqc).c <<= 1 as libc::c_int;
          (*mqc).ct = (*mqc).ct.wrapping_sub(1);
          if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
            break;
          }
        }
      } else {
        v2 = (**(*mqc).curctx).mps
      }
    }
    v = v << 1 as libc::c_int | v2;
    (*mqc).a =
      ((*mqc).a as libc::c_uint).wrapping_sub((**(*mqc).curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
    if ((*mqc).c >> 16 as libc::c_int) < (**(*mqc).curctx).qeval {
      if (*mqc).a < (**(*mqc).curctx).qeval {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v2 = (**(*mqc).curctx).mps;
        *(*mqc).curctx = (**(*mqc).curctx).nmps
      } else {
        (*mqc).a = (**(*mqc).curctx).qeval;
        v2 = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
        *(*mqc).curctx = (**(*mqc).curctx).nlps
      }
      loop {
        if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
          let mut l_c_5: OPJ_UINT32 = 0;
          l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
            if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
              (*mqc).c = ((*mqc).c as libc::c_uint)
                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
              (*mqc).end_of_byte_stream_counter = (*mqc).end_of_byte_stream_counter.wrapping_add(1)
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
            }
          } else {
            (*mqc).bp = (*mqc).bp.offset(1);
            (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
          }
        }
        (*mqc).a <<= 1 as libc::c_int;
        (*mqc).c <<= 1 as libc::c_int;
        (*mqc).ct = (*mqc).ct.wrapping_sub(1);
        if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
          break;
        }
      }
    } else {
      (*mqc).c = ((*mqc).c as libc::c_uint)
        .wrapping_sub((**(*mqc).curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
        as OPJ_UINT32;
      if (*mqc).a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        if (*mqc).a < (**(*mqc).curctx).qeval {
          v2 = ((**(*mqc).curctx).mps == 0) as libc::c_int as OPJ_UINT32;
          *(*mqc).curctx = (**(*mqc).curctx).nlps
        } else {
          v2 = (**(*mqc).curctx).mps;
          *(*mqc).curctx = (**(*mqc).curctx).nmps
        }
        loop {
          if (*mqc).ct == 0 as libc::c_int as libc::c_uint {
            let mut l_c_6: OPJ_UINT32 = 0;
            l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
              if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                (*mqc).c = ((*mqc).c as libc::c_uint)
                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 8 as libc::c_int as OPJ_UINT32;
                (*mqc).end_of_byte_stream_counter =
                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                (*mqc).ct = 7 as libc::c_int as OPJ_UINT32
              }
            } else {
              (*mqc).bp = (*mqc).bp.offset(1);
              (*mqc).c = ((*mqc).c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              (*mqc).ct = 8 as libc::c_int as OPJ_UINT32
            }
          }
          (*mqc).a <<= 1 as libc::c_int;
          (*mqc).c <<= 1 as libc::c_int;
          (*mqc).ct = (*mqc).ct.wrapping_sub(1);
          if !((*mqc).a < 0x8000 as libc::c_int as libc::c_uint) {
            break;
          }
        }
      } else {
        v2 = (**(*mqc).curctx).mps
      }
    }
    v = v << 1 as libc::c_int | v2
    /*
    if (v!=0xa) {
        opj_event_msg(t1->cinfo, EVT_WARNING, "Bad segmentation symbol %x\n", v);
    }
    */
  };
}
unsafe extern "C" fn opj_t1_dec_clnpass_64x64_novsc(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut runlen: OPJ_UINT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let l_w = 64 as libc::c_int as OPJ_UINT32;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1)
    .flags
    .offset((66 as libc::c_int + 1 as libc::c_int) as isize)
    as *mut opj_flag_t;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < 64 as libc::c_int as libc::c_uint & !(3 as libc::c_uint) {
    let mut current_block_1828: u64;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags == 0 as libc::c_int as libc::c_uint {
        let mut partial = 1 as libc::c_int as OPJ_UINT32;
        curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
          (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int) as OPJ_UINT32
            as isize,
        ) as *mut *const opj_mqc_state_t;
        a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
        if (c >> 16 as libc::c_int) < (**curctx).qeval {
          if a < (**curctx).qeval {
            a = (**curctx).qeval;
            v = (**curctx).mps;
            *curctx = (**curctx).nmps
          } else {
            a = (**curctx).qeval;
            v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
            *curctx = (**curctx).nlps
          }
          loop {
            if ct == 0 as libc::c_int as libc::c_uint {
              let mut l_c: OPJ_UINT32 = 0;
              l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                if l_c > 0x8f as libc::c_int as libc::c_uint {
                  c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                    as OPJ_UINT32 as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32;
                  (*mqc).end_of_byte_stream_counter =
                    (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                  as OPJ_UINT32;
                ct = 8 as libc::c_int as OPJ_UINT32
              }
            }
            a <<= 1 as libc::c_int;
            c <<= 1 as libc::c_int;
            ct = ct.wrapping_sub(1);
            if !(a < 0x8000 as libc::c_int as libc::c_uint) {
              break;
            }
          }
        } else {
          c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
            as OPJ_UINT32;
          if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            if a < (**curctx).qeval {
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            } else {
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_0: OPJ_UINT32 = 0;
                l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            v = (**curctx).mps
          }
        }
        if v == 0 {
          current_block_1828 = 2979737022853876585;
        } else {
          curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
            (0 as libc::c_int
              + 9 as libc::c_int
              + 5 as libc::c_int
              + 3 as libc::c_int
              + 1 as libc::c_int) as OPJ_UINT32 as isize,
          ) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              runlen = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              runlen = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_1: OPJ_UINT32 = 0;
                l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                runlen = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                runlen = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_2: OPJ_UINT32 = 0;
                  l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              runlen = (**curctx).mps
            }
          }
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          runlen = runlen << 1 as libc::c_int | v;
          let mut current_block_1045: u64;
          match runlen {
            0 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_452: u64;
                if 1 as libc::c_int == 0 {
                  let mut ctxt1 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_5: OPJ_UINT32 = 0;
                        l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_6: OPJ_UINT32 = 0;
                          l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_452 = 16116475693927719422;
                  } else {
                    current_block_452 = 14785121481331406365;
                  }
                } else {
                  current_block_452 = 14785121481331406365;
                }
                match current_block_452 {
                  14785121481331406365 => {
                    let mut lu = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      0 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_7: OPJ_UINT32 = 0;
                          l_c_7 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_7 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_7 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_7 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_8: OPJ_UINT32 = 0;
                            l_c_8 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_8 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_8 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_8 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu) as libc::c_uint;
                    *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh123 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh123 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    let ref mut fresh124 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh124 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north = flagsp.offset(-(66 as libc::c_int as isize));
                      *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh125 = *north.offset(-(1 as libc::c_int) as isize);
                      *fresh125 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh126 = *north.offset(1 as libc::c_int as isize);
                      *fresh126 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south = flagsp.offset(66 as libc::c_int as isize);
                      *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh127 = *south.offset(-(1 as libc::c_int) as isize);
                      *fresh127 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh128 = *south.offset(1 as libc::c_int as isize);
                      *fresh128 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 16576344162140347375;
            }
            1 => {
              current_block_1045 = 16576344162140347375;
            }
            2 => {
              current_block_1045 = 15278417807852362347;
            }
            3 => {
              current_block_1045 = 4643784342201421966;
            }
            _ => {
              current_block_1045 = 14550598362925142901;
            }
          }
          match current_block_1045 {
            16576344162140347375 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_648: u64;
                if partial == 0 {
                  let mut ctxt1_0 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_9: OPJ_UINT32 = 0;
                        l_c_9 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_9 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_9 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_9 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_10: OPJ_UINT32 = 0;
                          l_c_10 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_10 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_10 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_10 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_648 = 2685306683101115345;
                  } else {
                    current_block_648 = 6535359582234382107;
                  }
                } else {
                  current_block_648 = 6535359582234382107;
                }
                match current_block_648 {
                  6535359582234382107 => {
                    let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      1 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_0
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_11: OPJ_UINT32 = 0;
                          l_c_11 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_11 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_11 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_11 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_12: OPJ_UINT32 = 0;
                            l_c_12 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_12 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_12 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_12 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_0) as libc::c_uint;
                    *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh129 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh129 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    let ref mut fresh130 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh130 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_0 = flagsp.offset(-(66 as libc::c_int as isize));
                      *north_0 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh131 = *north_0.offset(-(1 as libc::c_int) as isize);
                      *fresh131 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh132 = *north_0.offset(1 as libc::c_int as isize);
                      *fresh132 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_0 = flagsp.offset(66 as libc::c_int as isize);
                      *south_0 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh133 = *south_0.offset(-(1 as libc::c_int) as isize);
                      *fresh133 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh134 = *south_0.offset(1 as libc::c_int as isize);
                      *fresh134 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 15278417807852362347;
            }
            _ => {}
          }
          match current_block_1045 {
            15278417807852362347 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_844: u64;
                if partial == 0 {
                  let mut ctxt1_1 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_13: OPJ_UINT32 = 0;
                        l_c_13 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_13 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_13 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_13 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_14: OPJ_UINT32 = 0;
                          l_c_14 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_14 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_14 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_14 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_844 = 10171483622901412108;
                  } else {
                    current_block_844 = 16329970517403260811;
                  }
                } else {
                  current_block_844 = 16329970517403260811;
                }
                match current_block_844 {
                  16329970517403260811 => {
                    let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      2 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_1
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_15: OPJ_UINT32 = 0;
                          l_c_15 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_15 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_15 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_15 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_16: OPJ_UINT32 = 0;
                            l_c_16 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_16 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_16 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_16 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_1) as libc::c_uint;
                    *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh135 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh135 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    let ref mut fresh136 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh136 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_1 = flagsp.offset(-(66 as libc::c_int as isize));
                      *north_1 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh137 = *north_1.offset(-(1 as libc::c_int) as isize);
                      *fresh137 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh138 = *north_1.offset(1 as libc::c_int as isize);
                      *fresh138 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_1 = flagsp.offset(66 as libc::c_int as isize);
                      *south_1 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh139 = *south_1.offset(-(1 as libc::c_int) as isize);
                      *fresh139 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh140 = *south_1.offset(1 as libc::c_int as isize);
                      *fresh140 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 4643784342201421966;
            }
            _ => {}
          }
          match current_block_1045 {
            4643784342201421966 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_1040: u64;
                if partial == 0 {
                  let mut ctxt1_2 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_17: OPJ_UINT32 = 0;
                        l_c_17 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_17 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_17 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_17 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_18: OPJ_UINT32 = 0;
                          l_c_18 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_18 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_18 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_18 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_1040 = 10095684397801062190;
                  } else {
                    current_block_1040 = 3833074357519289034;
                  }
                } else {
                  current_block_1040 = 3833074357519289034;
                }
                match current_block_1040 {
                  3833074357519289034 => {
                    let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      3 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_2
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_19: OPJ_UINT32 = 0;
                          l_c_19 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_19 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_19 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_19 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_20: OPJ_UINT32 = 0;
                            l_c_20 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_20 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_20 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_20 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_2) as libc::c_uint;
                    *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh141 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh141 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    let ref mut fresh142 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh142 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_2 = flagsp.offset(-(66 as libc::c_int as isize));
                      *north_2 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh143 = *north_2.offset(-(1 as libc::c_int) as isize);
                      *fresh143 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh144 = *north_2.offset(1 as libc::c_int as isize);
                      *fresh144 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_2 = flagsp.offset(66 as libc::c_int as isize);
                      *south_2 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh145 = *south_2.offset(-(1 as libc::c_int) as isize);
                      *fresh145 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh146 = *south_2.offset(1 as libc::c_int as isize);
                      *fresh146 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
            }
            _ => {}
          }
          current_block_1828 = 10917493918967617673;
        }
      } else {
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1236: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_3 = opj_t1_getctxno_zc(
              mqc,
              flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_3 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_21: OPJ_UINT32 = 0;
                  l_c_21 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_21 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_21 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_21 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_22: OPJ_UINT32 = 0;
                    l_c_22 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_22 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_22 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_22 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1236 = 1928523173154854502;
            } else {
              current_block_1236 = 12323252596376536154;
            }
          } else {
            current_block_1236 = 12323252596376536154;
          }
          match current_block_1236 {
            12323252596376536154 => {
              let mut lu_3 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                0 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_3
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_23: OPJ_UINT32 = 0;
                    l_c_23 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_23 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_23 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_23 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_24: OPJ_UINT32 = 0;
                      l_c_24 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_24 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_24 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_24 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_3) as libc::c_uint;
              *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh147 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh147 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              let ref mut fresh148 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh148 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_3 = flagsp.offset(-(66 as libc::c_int as isize));
                *north_3 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh149 = *north_3.offset(-(1 as libc::c_int) as isize);
                *fresh149 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh150 = *north_3.offset(1 as libc::c_int as isize);
                *fresh150 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_3 = flagsp.offset(66 as libc::c_int as isize);
                *south_3 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh151 = *south_3.offset(-(1 as libc::c_int) as isize);
                *fresh151 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh152 = *south_3.offset(1 as libc::c_int as isize);
                *fresh152 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1431: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_4 = opj_t1_getctxno_zc(
              mqc,
              flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_4 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_25: OPJ_UINT32 = 0;
                  l_c_25 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_25 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_25 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_25 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_26: OPJ_UINT32 = 0;
                    l_c_26 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_26 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_26 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_26 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1431 = 7360758368027237918;
            } else {
              current_block_1431 = 1372330848231551451;
            }
          } else {
            current_block_1431 = 1372330848231551451;
          }
          match current_block_1431 {
            1372330848231551451 => {
              let mut lu_4 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                1 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_4
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_27: OPJ_UINT32 = 0;
                    l_c_27 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_27 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_27 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_27 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_28: OPJ_UINT32 = 0;
                      l_c_28 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_28 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_28 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_28 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_4) as libc::c_uint;
              *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh153 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh153 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              let ref mut fresh154 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh154 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_4 = flagsp.offset(-(66 as libc::c_int as isize));
                *north_4 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh155 = *north_4.offset(-(1 as libc::c_int) as isize);
                *fresh155 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh156 = *north_4.offset(1 as libc::c_int as isize);
                *fresh156 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_4 = flagsp.offset(66 as libc::c_int as isize);
                *south_4 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh157 = *south_4.offset(-(1 as libc::c_int) as isize);
                *fresh157 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh158 = *south_4.offset(1 as libc::c_int as isize);
                *fresh158 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1626: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_5 = opj_t1_getctxno_zc(
              mqc,
              flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_5 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_29: OPJ_UINT32 = 0;
                  l_c_29 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_29 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_29 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_29 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_30: OPJ_UINT32 = 0;
                    l_c_30 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_30 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_30 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_30 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1626 = 15312420746610300268;
            } else {
              current_block_1626 = 911994807774668979;
            }
          } else {
            current_block_1626 = 911994807774668979;
          }
          match current_block_1626 {
            911994807774668979 => {
              let mut lu_5 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                2 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_5
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_31: OPJ_UINT32 = 0;
                    l_c_31 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_31 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_31 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_31 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_32: OPJ_UINT32 = 0;
                      l_c_32 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_32 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_32 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_32 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_5) as libc::c_uint;
              *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh159 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh159 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              let ref mut fresh160 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh160 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_5 = flagsp.offset(-(66 as libc::c_int as isize));
                *north_5 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh161 = *north_5.offset(-(1 as libc::c_int) as isize);
                *fresh161 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh162 = *north_5.offset(1 as libc::c_int as isize);
                *fresh162 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_5 = flagsp.offset(66 as libc::c_int as isize);
                *south_5 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh163 = *south_5.offset(-(1 as libc::c_int) as isize);
                *fresh163 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh164 = *south_5.offset(1 as libc::c_int as isize);
                *fresh164 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1821: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_6 = opj_t1_getctxno_zc(
              mqc,
              flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_6 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_33: OPJ_UINT32 = 0;
                  l_c_33 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_33 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_33 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_33 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_34: OPJ_UINT32 = 0;
                    l_c_34 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_34 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_34 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_34 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1821 = 17698901117243856307;
            } else {
              current_block_1821 = 12990703611517236731;
            }
          } else {
            current_block_1821 = 12990703611517236731;
          }
          match current_block_1821 {
            12990703611517236731 => {
              let mut lu_6 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                3 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_6
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_35: OPJ_UINT32 = 0;
                    l_c_35 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_35 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_35 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_35 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_36: OPJ_UINT32 = 0;
                      l_c_36 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_36 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_36 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_36 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_6) as libc::c_uint;
              *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh165 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh165 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              let ref mut fresh166 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh166 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_6 = flagsp.offset(-(66 as libc::c_int as isize));
                *north_6 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh167 = *north_6.offset(-(1 as libc::c_int) as isize);
                *fresh167 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh168 = *north_6.offset(1 as libc::c_int as isize);
                *fresh168 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_6 = flagsp.offset(66 as libc::c_int as isize);
                *south_6 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh169 = *south_6.offset(-(1 as libc::c_int) as isize);
                *fresh169 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh170 = *south_6.offset(1 as libc::c_int as isize);
                *fresh170 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        current_block_1828 = 10917493918967617673;
      }
      match current_block_1828 {
        10917493918967617673 => {
          *flagsp = flags
            & !((1 as libc::c_uint) << 21 as libc::c_int
              | (1 as libc::c_uint) << 24 as libc::c_int
              | (1 as libc::c_uint) << 27 as libc::c_int
              | (1 as libc::c_uint) << 30 as libc::c_int)
        }
        _ => {}
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < 64 as libc::c_int as libc::c_uint {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (64 as libc::c_int as libc::c_uint).wrapping_sub(k) {
        opj_t1_dec_clnpass_step(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          j,
          0 as libc::c_int as OPJ_UINT32,
        );
        j = j.wrapping_add(1)
      }
      *flagsp &= !((1 as libc::c_uint) << 21 as libc::c_int
        | (1 as libc::c_uint) << 24 as libc::c_int
        | (1 as libc::c_uint) << 27 as libc::c_int
        | (1 as libc::c_uint) << 30 as libc::c_int);
      i = i.wrapping_add(1);
      flagsp = flagsp.offset(1);
      data = data.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_clnpass_64x64_vsc(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut runlen: OPJ_UINT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let l_w = 64 as libc::c_int as OPJ_UINT32;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1)
    .flags
    .offset((66 as libc::c_int + 1 as libc::c_int) as isize)
    as *mut opj_flag_t;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < 64 as libc::c_int as libc::c_uint & !(3 as libc::c_uint) {
    let mut current_block_1828: u64;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags == 0 as libc::c_int as libc::c_uint {
        let mut partial = 1 as libc::c_int as OPJ_UINT32;
        curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
          (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int) as OPJ_UINT32
            as isize,
        ) as *mut *const opj_mqc_state_t;
        a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
        if (c >> 16 as libc::c_int) < (**curctx).qeval {
          if a < (**curctx).qeval {
            a = (**curctx).qeval;
            v = (**curctx).mps;
            *curctx = (**curctx).nmps
          } else {
            a = (**curctx).qeval;
            v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
            *curctx = (**curctx).nlps
          }
          loop {
            if ct == 0 as libc::c_int as libc::c_uint {
              let mut l_c: OPJ_UINT32 = 0;
              l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                if l_c > 0x8f as libc::c_int as libc::c_uint {
                  c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                    as OPJ_UINT32 as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32;
                  (*mqc).end_of_byte_stream_counter =
                    (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                  as OPJ_UINT32;
                ct = 8 as libc::c_int as OPJ_UINT32
              }
            }
            a <<= 1 as libc::c_int;
            c <<= 1 as libc::c_int;
            ct = ct.wrapping_sub(1);
            if !(a < 0x8000 as libc::c_int as libc::c_uint) {
              break;
            }
          }
        } else {
          c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
            as OPJ_UINT32;
          if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            if a < (**curctx).qeval {
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            } else {
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_0: OPJ_UINT32 = 0;
                l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            v = (**curctx).mps
          }
        }
        if v == 0 {
          current_block_1828 = 2979737022853876585;
        } else {
          curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
            (0 as libc::c_int
              + 9 as libc::c_int
              + 5 as libc::c_int
              + 3 as libc::c_int
              + 1 as libc::c_int) as OPJ_UINT32 as isize,
          ) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              runlen = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              runlen = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_1: OPJ_UINT32 = 0;
                l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                runlen = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                runlen = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_2: OPJ_UINT32 = 0;
                  l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              runlen = (**curctx).mps
            }
          }
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          runlen = runlen << 1 as libc::c_int | v;
          let mut current_block_1045: u64;
          match runlen {
            0 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_452: u64;
                if 1 as libc::c_int == 0 {
                  let mut ctxt1 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_5: OPJ_UINT32 = 0;
                        l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_6: OPJ_UINT32 = 0;
                          l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_452 = 16116475693927719422;
                  } else {
                    current_block_452 = 14785121481331406365;
                  }
                } else {
                  current_block_452 = 14785121481331406365;
                }
                match current_block_452 {
                  14785121481331406365 => {
                    let mut lu = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      0 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_7: OPJ_UINT32 = 0;
                          l_c_7 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_7 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_7 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_7 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_8: OPJ_UINT32 = 0;
                            l_c_8 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_8 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_8 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_8 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu) as libc::c_uint;
                    *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh171 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh171 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    let ref mut fresh172 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh172 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 1 as libc::c_int == 0
                    {
                      let mut north = flagsp.offset(-(66 as libc::c_int as isize));
                      *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh173 = *north.offset(-(1 as libc::c_int) as isize);
                      *fresh173 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh174 = *north.offset(1 as libc::c_int as isize);
                      *fresh174 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south = flagsp.offset(66 as libc::c_int as isize);
                      *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh175 = *south.offset(-(1 as libc::c_int) as isize);
                      *fresh175 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh176 = *south.offset(1 as libc::c_int as isize);
                      *fresh176 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 13073922968472590235;
            }
            1 => {
              current_block_1045 = 13073922968472590235;
            }
            2 => {
              current_block_1045 = 10189523088492235255;
            }
            3 => {
              current_block_1045 = 1361787845153501308;
            }
            _ => {
              current_block_1045 = 14550598362925142901;
            }
          }
          match current_block_1045 {
            13073922968472590235 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_648: u64;
                if partial == 0 {
                  let mut ctxt1_0 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_9: OPJ_UINT32 = 0;
                        l_c_9 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_9 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_9 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_9 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_10: OPJ_UINT32 = 0;
                          l_c_10 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_10 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_10 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_10 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_648 = 2685306683101115345;
                  } else {
                    current_block_648 = 6535359582234382107;
                  }
                } else {
                  current_block_648 = 6535359582234382107;
                }
                match current_block_648 {
                  6535359582234382107 => {
                    let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      1 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_0
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_11: OPJ_UINT32 = 0;
                          l_c_11 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_11 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_11 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_11 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_12: OPJ_UINT32 = 0;
                            l_c_12 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_12 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_12 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_12 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_0) as libc::c_uint;
                    *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh177 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh177 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    let ref mut fresh178 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh178 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_0 = flagsp.offset(-(66 as libc::c_int as isize));
                      *north_0 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh179 = *north_0.offset(-(1 as libc::c_int) as isize);
                      *fresh179 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh180 = *north_0.offset(1 as libc::c_int as isize);
                      *fresh180 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_0 = flagsp.offset(66 as libc::c_int as isize);
                      *south_0 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh181 = *south_0.offset(-(1 as libc::c_int) as isize);
                      *fresh181 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh182 = *south_0.offset(1 as libc::c_int as isize);
                      *fresh182 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 10189523088492235255;
            }
            _ => {}
          }
          match current_block_1045 {
            10189523088492235255 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_844: u64;
                if partial == 0 {
                  let mut ctxt1_1 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_13: OPJ_UINT32 = 0;
                        l_c_13 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_13 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_13 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_13 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_14: OPJ_UINT32 = 0;
                          l_c_14 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_14 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_14 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_14 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_844 = 10171483622901412108;
                  } else {
                    current_block_844 = 16329970517403260811;
                  }
                } else {
                  current_block_844 = 16329970517403260811;
                }
                match current_block_844 {
                  16329970517403260811 => {
                    let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      2 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_1
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_15: OPJ_UINT32 = 0;
                          l_c_15 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_15 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_15 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_15 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_16: OPJ_UINT32 = 0;
                            l_c_16 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_16 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_16 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_16 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_1) as libc::c_uint;
                    *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh183 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh183 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    let ref mut fresh184 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh184 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_1 = flagsp.offset(-(66 as libc::c_int as isize));
                      *north_1 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh185 = *north_1.offset(-(1 as libc::c_int) as isize);
                      *fresh185 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh186 = *north_1.offset(1 as libc::c_int as isize);
                      *fresh186 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_1 = flagsp.offset(66 as libc::c_int as isize);
                      *south_1 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh187 = *south_1.offset(-(1 as libc::c_int) as isize);
                      *fresh187 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh188 = *south_1.offset(1 as libc::c_int as isize);
                      *fresh188 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 1361787845153501308;
            }
            _ => {}
          }
          match current_block_1045 {
            1361787845153501308 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_1040: u64;
                if partial == 0 {
                  let mut ctxt1_2 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_17: OPJ_UINT32 = 0;
                        l_c_17 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_17 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_17 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_17 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_18: OPJ_UINT32 = 0;
                          l_c_18 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_18 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_18 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_18 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_1040 = 10095684397801062190;
                  } else {
                    current_block_1040 = 3833074357519289034;
                  }
                } else {
                  current_block_1040 = 3833074357519289034;
                }
                match current_block_1040 {
                  3833074357519289034 => {
                    let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      3 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_2
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_19: OPJ_UINT32 = 0;
                          l_c_19 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_19 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_19 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_19 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_20: OPJ_UINT32 = 0;
                            l_c_20 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_20 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_20 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_20 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_2) as libc::c_uint;
                    *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh189 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh189 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    let ref mut fresh190 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh190 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_2 = flagsp.offset(-(66 as libc::c_int as isize));
                      *north_2 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh191 = *north_2.offset(-(1 as libc::c_int) as isize);
                      *fresh191 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh192 = *north_2.offset(1 as libc::c_int as isize);
                      *fresh192 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_2 = flagsp.offset(66 as libc::c_int as isize);
                      *south_2 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh193 = *south_2.offset(-(1 as libc::c_int) as isize);
                      *fresh193 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh194 = *south_2.offset(1 as libc::c_int as isize);
                      *fresh194 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
            }
            _ => {}
          }
          current_block_1828 = 10917493918967617673;
        }
      } else {
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1236: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_3 = opj_t1_getctxno_zc(
              mqc,
              flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_3 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_21: OPJ_UINT32 = 0;
                  l_c_21 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_21 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_21 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_21 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_22: OPJ_UINT32 = 0;
                    l_c_22 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_22 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_22 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_22 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1236 = 1928523173154854502;
            } else {
              current_block_1236 = 12323252596376536154;
            }
          } else {
            current_block_1236 = 12323252596376536154;
          }
          match current_block_1236 {
            12323252596376536154 => {
              let mut lu_3 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                0 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_3
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_23: OPJ_UINT32 = 0;
                    l_c_23 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_23 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_23 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_23 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_24: OPJ_UINT32 = 0;
                      l_c_24 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_24 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_24 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_24 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_3) as libc::c_uint;
              *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh195 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh195 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              let ref mut fresh196 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh196 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 1 as libc::c_int == 0 {
                let mut north_3 = flagsp.offset(-(66 as libc::c_int as isize));
                *north_3 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh197 = *north_3.offset(-(1 as libc::c_int) as isize);
                *fresh197 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh198 = *north_3.offset(1 as libc::c_int as isize);
                *fresh198 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_3 = flagsp.offset(66 as libc::c_int as isize);
                *south_3 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh199 = *south_3.offset(-(1 as libc::c_int) as isize);
                *fresh199 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh200 = *south_3.offset(1 as libc::c_int as isize);
                *fresh200 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1431: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_4 = opj_t1_getctxno_zc(
              mqc,
              flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_4 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_25: OPJ_UINT32 = 0;
                  l_c_25 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_25 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_25 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_25 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_26: OPJ_UINT32 = 0;
                    l_c_26 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_26 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_26 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_26 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1431 = 7360758368027237918;
            } else {
              current_block_1431 = 1372330848231551451;
            }
          } else {
            current_block_1431 = 1372330848231551451;
          }
          match current_block_1431 {
            1372330848231551451 => {
              let mut lu_4 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                1 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_4
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_27: OPJ_UINT32 = 0;
                    l_c_27 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_27 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_27 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_27 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_28: OPJ_UINT32 = 0;
                      l_c_28 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_28 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_28 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_28 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_4) as libc::c_uint;
              *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh201 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh201 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              let ref mut fresh202 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh202 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_4 = flagsp.offset(-(66 as libc::c_int as isize));
                *north_4 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh203 = *north_4.offset(-(1 as libc::c_int) as isize);
                *fresh203 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh204 = *north_4.offset(1 as libc::c_int as isize);
                *fresh204 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_4 = flagsp.offset(66 as libc::c_int as isize);
                *south_4 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh205 = *south_4.offset(-(1 as libc::c_int) as isize);
                *fresh205 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh206 = *south_4.offset(1 as libc::c_int as isize);
                *fresh206 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1626: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_5 = opj_t1_getctxno_zc(
              mqc,
              flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_5 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_29: OPJ_UINT32 = 0;
                  l_c_29 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_29 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_29 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_29 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_30: OPJ_UINT32 = 0;
                    l_c_30 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_30 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_30 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_30 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1626 = 15312420746610300268;
            } else {
              current_block_1626 = 911994807774668979;
            }
          } else {
            current_block_1626 = 911994807774668979;
          }
          match current_block_1626 {
            911994807774668979 => {
              let mut lu_5 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                2 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_5
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_31: OPJ_UINT32 = 0;
                    l_c_31 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_31 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_31 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_31 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_32: OPJ_UINT32 = 0;
                      l_c_32 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_32 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_32 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_32 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_5) as libc::c_uint;
              *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh207 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh207 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              let ref mut fresh208 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh208 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_5 = flagsp.offset(-(66 as libc::c_int as isize));
                *north_5 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh209 = *north_5.offset(-(1 as libc::c_int) as isize);
                *fresh209 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh210 = *north_5.offset(1 as libc::c_int as isize);
                *fresh210 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_5 = flagsp.offset(66 as libc::c_int as isize);
                *south_5 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh211 = *south_5.offset(-(1 as libc::c_int) as isize);
                *fresh211 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh212 = *south_5.offset(1 as libc::c_int as isize);
                *fresh212 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1821: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_6 = opj_t1_getctxno_zc(
              mqc,
              flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_6 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_33: OPJ_UINT32 = 0;
                  l_c_33 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_33 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_33 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_33 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_34: OPJ_UINT32 = 0;
                    l_c_34 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_34 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_34 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_34 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1821 = 17698901117243856307;
            } else {
              current_block_1821 = 12990703611517236731;
            }
          } else {
            current_block_1821 = 12990703611517236731;
          }
          match current_block_1821 {
            12990703611517236731 => {
              let mut lu_6 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                3 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_6
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_35: OPJ_UINT32 = 0;
                    l_c_35 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_35 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_35 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_35 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_36: OPJ_UINT32 = 0;
                      l_c_36 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_36 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_36 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_36 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_6) as libc::c_uint;
              *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh213 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh213 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              let ref mut fresh214 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh214 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_6 = flagsp.offset(-(66 as libc::c_int as isize));
                *north_6 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh215 = *north_6.offset(-(1 as libc::c_int) as isize);
                *fresh215 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh216 = *north_6.offset(1 as libc::c_int as isize);
                *fresh216 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_6 = flagsp.offset(66 as libc::c_int as isize);
                *south_6 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh217 = *south_6.offset(-(1 as libc::c_int) as isize);
                *fresh217 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh218 = *south_6.offset(1 as libc::c_int as isize);
                *fresh218 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        current_block_1828 = 10917493918967617673;
      }
      match current_block_1828 {
        10917493918967617673 => {
          *flagsp = flags
            & !((1 as libc::c_uint) << 21 as libc::c_int
              | (1 as libc::c_uint) << 24 as libc::c_int
              | (1 as libc::c_uint) << 27 as libc::c_int
              | (1 as libc::c_uint) << 30 as libc::c_int)
        }
        _ => {}
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < 64 as libc::c_int as libc::c_uint {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (64 as libc::c_int as libc::c_uint).wrapping_sub(k) {
        opj_t1_dec_clnpass_step(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          j,
          1 as libc::c_int as OPJ_UINT32,
        );
        j = j.wrapping_add(1)
      }
      *flagsp &= !((1 as libc::c_uint) << 21 as libc::c_int
        | (1 as libc::c_uint) << 24 as libc::c_int
        | (1 as libc::c_uint) << 27 as libc::c_int
        | (1 as libc::c_uint) << 30 as libc::c_int);
      i = i.wrapping_add(1);
      flagsp = flagsp.offset(1);
      data = data.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_clnpass_generic_novsc(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut runlen: OPJ_UINT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let l_w = (*t1).w;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1).flags.offset(
    (*t1)
      .w
      .wrapping_add(2 as libc::c_uint)
      .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
  ) as *mut opj_flag_t;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    let mut current_block_1828: u64;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags == 0 as libc::c_int as libc::c_uint {
        let mut partial = 1 as libc::c_int as OPJ_UINT32;
        curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
          (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int) as OPJ_UINT32
            as isize,
        ) as *mut *const opj_mqc_state_t;
        a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
        if (c >> 16 as libc::c_int) < (**curctx).qeval {
          if a < (**curctx).qeval {
            a = (**curctx).qeval;
            v = (**curctx).mps;
            *curctx = (**curctx).nmps
          } else {
            a = (**curctx).qeval;
            v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
            *curctx = (**curctx).nlps
          }
          loop {
            if ct == 0 as libc::c_int as libc::c_uint {
              let mut l_c: OPJ_UINT32 = 0;
              l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                if l_c > 0x8f as libc::c_int as libc::c_uint {
                  c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                    as OPJ_UINT32 as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32;
                  (*mqc).end_of_byte_stream_counter =
                    (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                  as OPJ_UINT32;
                ct = 8 as libc::c_int as OPJ_UINT32
              }
            }
            a <<= 1 as libc::c_int;
            c <<= 1 as libc::c_int;
            ct = ct.wrapping_sub(1);
            if !(a < 0x8000 as libc::c_int as libc::c_uint) {
              break;
            }
          }
        } else {
          c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
            as OPJ_UINT32;
          if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            if a < (**curctx).qeval {
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            } else {
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_0: OPJ_UINT32 = 0;
                l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            v = (**curctx).mps
          }
        }
        if v == 0 {
          current_block_1828 = 2979737022853876585;
        } else {
          curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
            (0 as libc::c_int
              + 9 as libc::c_int
              + 5 as libc::c_int
              + 3 as libc::c_int
              + 1 as libc::c_int) as OPJ_UINT32 as isize,
          ) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              runlen = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              runlen = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_1: OPJ_UINT32 = 0;
                l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                runlen = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                runlen = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_2: OPJ_UINT32 = 0;
                  l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              runlen = (**curctx).mps
            }
          }
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          runlen = runlen << 1 as libc::c_int | v;
          let mut current_block_1045: u64;
          match runlen {
            0 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_452: u64;
                if 1 as libc::c_int == 0 {
                  let mut ctxt1 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_5: OPJ_UINT32 = 0;
                        l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_6: OPJ_UINT32 = 0;
                          l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_452 = 16116475693927719422;
                  } else {
                    current_block_452 = 14785121481331406365;
                  }
                } else {
                  current_block_452 = 14785121481331406365;
                }
                match current_block_452 {
                  14785121481331406365 => {
                    let mut lu = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      0 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_7: OPJ_UINT32 = 0;
                          l_c_7 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_7 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_7 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_7 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_8: OPJ_UINT32 = 0;
                            l_c_8 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_8 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_8 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_8 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu) as libc::c_uint;
                    *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh219 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh219 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    let ref mut fresh220 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh220 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north =
                        flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                      *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh221 = *north.offset(-(1 as libc::c_int) as isize);
                      *fresh221 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh222 = *north.offset(1 as libc::c_int as isize);
                      *fresh222 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south =
                        flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                      *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh223 = *south.offset(-(1 as libc::c_int) as isize);
                      *fresh223 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh224 = *south.offset(1 as libc::c_int as isize);
                      *fresh224 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 16467187608951699113;
            }
            1 => {
              current_block_1045 = 16467187608951699113;
            }
            2 => {
              current_block_1045 = 17838868982845454862;
            }
            3 => {
              current_block_1045 = 15069527104374245571;
            }
            _ => {
              current_block_1045 = 14550598362925142901;
            }
          }
          match current_block_1045 {
            16467187608951699113 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_648: u64;
                if partial == 0 {
                  let mut ctxt1_0 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_9: OPJ_UINT32 = 0;
                        l_c_9 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_9 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_9 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_9 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_10: OPJ_UINT32 = 0;
                          l_c_10 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_10 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_10 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_10 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_648 = 2685306683101115345;
                  } else {
                    current_block_648 = 6535359582234382107;
                  }
                } else {
                  current_block_648 = 6535359582234382107;
                }
                match current_block_648 {
                  6535359582234382107 => {
                    let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      1 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_0
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_11: OPJ_UINT32 = 0;
                          l_c_11 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_11 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_11 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_11 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_12: OPJ_UINT32 = 0;
                            l_c_12 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_12 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_12 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_12 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_0) as libc::c_uint;
                    *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh225 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh225 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    let ref mut fresh226 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh226 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_0 =
                        flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                      *north_0 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh227 = *north_0.offset(-(1 as libc::c_int) as isize);
                      *fresh227 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh228 = *north_0.offset(1 as libc::c_int as isize);
                      *fresh228 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_0 =
                        flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                      *south_0 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh229 = *south_0.offset(-(1 as libc::c_int) as isize);
                      *fresh229 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh230 = *south_0.offset(1 as libc::c_int as isize);
                      *fresh230 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 17838868982845454862;
            }
            _ => {}
          }
          match current_block_1045 {
            17838868982845454862 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_844: u64;
                if partial == 0 {
                  let mut ctxt1_1 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_13: OPJ_UINT32 = 0;
                        l_c_13 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_13 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_13 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_13 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_14: OPJ_UINT32 = 0;
                          l_c_14 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_14 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_14 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_14 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_844 = 10171483622901412108;
                  } else {
                    current_block_844 = 16329970517403260811;
                  }
                } else {
                  current_block_844 = 16329970517403260811;
                }
                match current_block_844 {
                  16329970517403260811 => {
                    let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      2 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_1
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_15: OPJ_UINT32 = 0;
                          l_c_15 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_15 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_15 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_15 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_16: OPJ_UINT32 = 0;
                            l_c_16 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_16 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_16 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_16 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_1) as libc::c_uint;
                    *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh231 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh231 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    let ref mut fresh232 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh232 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_1 =
                        flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                      *north_1 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh233 = *north_1.offset(-(1 as libc::c_int) as isize);
                      *fresh233 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh234 = *north_1.offset(1 as libc::c_int as isize);
                      *fresh234 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_1 =
                        flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                      *south_1 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh235 = *south_1.offset(-(1 as libc::c_int) as isize);
                      *fresh235 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh236 = *south_1.offset(1 as libc::c_int as isize);
                      *fresh236 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 15069527104374245571;
            }
            _ => {}
          }
          match current_block_1045 {
            15069527104374245571 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_1040: u64;
                if partial == 0 {
                  let mut ctxt1_2 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_17: OPJ_UINT32 = 0;
                        l_c_17 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_17 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_17 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_17 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_18: OPJ_UINT32 = 0;
                          l_c_18 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_18 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_18 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_18 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_1040 = 10095684397801062190;
                  } else {
                    current_block_1040 = 3833074357519289034;
                  }
                } else {
                  current_block_1040 = 3833074357519289034;
                }
                match current_block_1040 {
                  3833074357519289034 => {
                    let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      3 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_2
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_19: OPJ_UINT32 = 0;
                          l_c_19 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_19 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_19 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_19 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_20: OPJ_UINT32 = 0;
                            l_c_20 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_20 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_20 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_20 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_2) as libc::c_uint;
                    *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh237 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh237 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    let ref mut fresh238 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh238 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_2 =
                        flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                      *north_2 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh239 = *north_2.offset(-(1 as libc::c_int) as isize);
                      *fresh239 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh240 = *north_2.offset(1 as libc::c_int as isize);
                      *fresh240 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_2 =
                        flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                      *south_2 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh241 = *south_2.offset(-(1 as libc::c_int) as isize);
                      *fresh241 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh242 = *south_2.offset(1 as libc::c_int as isize);
                      *fresh242 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
            }
            _ => {}
          }
          current_block_1828 = 10917493918967617673;
        }
      } else {
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1236: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_3 = opj_t1_getctxno_zc(
              mqc,
              flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_3 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_21: OPJ_UINT32 = 0;
                  l_c_21 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_21 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_21 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_21 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_22: OPJ_UINT32 = 0;
                    l_c_22 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_22 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_22 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_22 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1236 = 1928523173154854502;
            } else {
              current_block_1236 = 12323252596376536154;
            }
          } else {
            current_block_1236 = 12323252596376536154;
          }
          match current_block_1236 {
            12323252596376536154 => {
              let mut lu_3 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                0 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_3
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_23: OPJ_UINT32 = 0;
                    l_c_23 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_23 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_23 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_23 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_24: OPJ_UINT32 = 0;
                      l_c_24 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_24 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_24 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_24 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_3) as libc::c_uint;
              *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh243 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh243 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              let ref mut fresh244 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh244 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_3 =
                  flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                *north_3 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh245 = *north_3.offset(-(1 as libc::c_int) as isize);
                *fresh245 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh246 = *north_3.offset(1 as libc::c_int as isize);
                *fresh246 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_3 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                *south_3 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh247 = *south_3.offset(-(1 as libc::c_int) as isize);
                *fresh247 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh248 = *south_3.offset(1 as libc::c_int as isize);
                *fresh248 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1431: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_4 = opj_t1_getctxno_zc(
              mqc,
              flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_4 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_25: OPJ_UINT32 = 0;
                  l_c_25 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_25 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_25 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_25 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_26: OPJ_UINT32 = 0;
                    l_c_26 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_26 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_26 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_26 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1431 = 7360758368027237918;
            } else {
              current_block_1431 = 1372330848231551451;
            }
          } else {
            current_block_1431 = 1372330848231551451;
          }
          match current_block_1431 {
            1372330848231551451 => {
              let mut lu_4 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                1 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_4
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_27: OPJ_UINT32 = 0;
                    l_c_27 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_27 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_27 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_27 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_28: OPJ_UINT32 = 0;
                      l_c_28 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_28 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_28 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_28 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_4) as libc::c_uint;
              *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh249 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh249 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              let ref mut fresh250 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh250 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_4 =
                  flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                *north_4 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh251 = *north_4.offset(-(1 as libc::c_int) as isize);
                *fresh251 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh252 = *north_4.offset(1 as libc::c_int as isize);
                *fresh252 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_4 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                *south_4 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh253 = *south_4.offset(-(1 as libc::c_int) as isize);
                *fresh253 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh254 = *south_4.offset(1 as libc::c_int as isize);
                *fresh254 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1626: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_5 = opj_t1_getctxno_zc(
              mqc,
              flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_5 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_29: OPJ_UINT32 = 0;
                  l_c_29 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_29 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_29 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_29 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_30: OPJ_UINT32 = 0;
                    l_c_30 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_30 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_30 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_30 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1626 = 15312420746610300268;
            } else {
              current_block_1626 = 911994807774668979;
            }
          } else {
            current_block_1626 = 911994807774668979;
          }
          match current_block_1626 {
            911994807774668979 => {
              let mut lu_5 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                2 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_5
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_31: OPJ_UINT32 = 0;
                    l_c_31 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_31 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_31 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_31 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_32: OPJ_UINT32 = 0;
                      l_c_32 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_32 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_32 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_32 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_5) as libc::c_uint;
              *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh255 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh255 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              let ref mut fresh256 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh256 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_5 =
                  flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                *north_5 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh257 = *north_5.offset(-(1 as libc::c_int) as isize);
                *fresh257 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh258 = *north_5.offset(1 as libc::c_int as isize);
                *fresh258 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_5 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                *south_5 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh259 = *south_5.offset(-(1 as libc::c_int) as isize);
                *fresh259 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh260 = *south_5.offset(1 as libc::c_int as isize);
                *fresh260 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1821: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_6 = opj_t1_getctxno_zc(
              mqc,
              flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_6 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_33: OPJ_UINT32 = 0;
                  l_c_33 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_33 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_33 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_33 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_34: OPJ_UINT32 = 0;
                    l_c_34 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_34 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_34 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_34 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1821 = 17698901117243856307;
            } else {
              current_block_1821 = 12990703611517236731;
            }
          } else {
            current_block_1821 = 12990703611517236731;
          }
          match current_block_1821 {
            12990703611517236731 => {
              let mut lu_6 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                3 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_6
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_35: OPJ_UINT32 = 0;
                    l_c_35 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_35 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_35 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_35 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_36: OPJ_UINT32 = 0;
                      l_c_36 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_36 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_36 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_36 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_6) as libc::c_uint;
              *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh261 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh261 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              let ref mut fresh262 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh262 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_6 =
                  flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                *north_6 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh263 = *north_6.offset(-(1 as libc::c_int) as isize);
                *fresh263 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh264 = *north_6.offset(1 as libc::c_int as isize);
                *fresh264 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_6 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                *south_6 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh265 = *south_6.offset(-(1 as libc::c_int) as isize);
                *fresh265 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh266 = *south_6.offset(1 as libc::c_int as isize);
                *fresh266 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        current_block_1828 = 10917493918967617673;
      }
      match current_block_1828 {
        10917493918967617673 => {
          *flagsp = flags
            & !((1 as libc::c_uint) << 21 as libc::c_int
              | (1 as libc::c_uint) << 24 as libc::c_int
              | (1 as libc::c_uint) << 27 as libc::c_int
              | (1 as libc::c_uint) << 30 as libc::c_int)
        }
        _ => {}
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < (*t1).h {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (*t1).h.wrapping_sub(k) {
        opj_t1_dec_clnpass_step(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          j,
          0 as libc::c_int as OPJ_UINT32,
        );
        j = j.wrapping_add(1)
      }
      *flagsp &= !((1 as libc::c_uint) << 21 as libc::c_int
        | (1 as libc::c_uint) << 24 as libc::c_int
        | (1 as libc::c_uint) << 27 as libc::c_int
        | (1 as libc::c_uint) << 30 as libc::c_int);
      i = i.wrapping_add(1);
      flagsp = flagsp.offset(1);
      data = data.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_clnpass_generic_vsc(mut t1: *mut opj_t1_t, mut bpno: OPJ_INT32) {
  let mut one: OPJ_INT32 = 0;
  let mut half: OPJ_INT32 = 0;
  let mut oneplushalf: OPJ_INT32 = 0;
  let mut runlen: OPJ_UINT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut k: OPJ_UINT32 = 0;
  let l_w = (*t1).w;
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut data = (*t1).data;
  let mut flagsp: *mut opj_flag_t = &mut *(*t1).flags.offset(
    (*t1)
      .w
      .wrapping_add(2 as libc::c_uint)
      .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
  ) as *mut opj_flag_t;
  let mut curctx = (*mqc).curctx;
  let mut c = (*mqc).c;
  let mut a = (*mqc).a;
  let mut ct = (*mqc).ct;
  let mut v: OPJ_UINT32 = 0;
  one = (1 as libc::c_int) << bpno;
  half = one >> 1 as libc::c_int;
  oneplushalf = one | half;
  k = 0 as libc::c_int as OPJ_UINT32;
  while k < (*t1).h & !(3 as libc::c_uint) {
    let mut current_block_1828: u64;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      let mut flags = *flagsp;
      if flags == 0 as libc::c_int as libc::c_uint {
        let mut partial = 1 as libc::c_int as OPJ_UINT32;
        curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
          (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int) as OPJ_UINT32
            as isize,
        ) as *mut *const opj_mqc_state_t;
        a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
        if (c >> 16 as libc::c_int) < (**curctx).qeval {
          if a < (**curctx).qeval {
            a = (**curctx).qeval;
            v = (**curctx).mps;
            *curctx = (**curctx).nmps
          } else {
            a = (**curctx).qeval;
            v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
            *curctx = (**curctx).nlps
          }
          loop {
            if ct == 0 as libc::c_int as libc::c_uint {
              let mut l_c: OPJ_UINT32 = 0;
              l_c = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
              if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                if l_c > 0x8f as libc::c_int as libc::c_uint {
                  c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                    as OPJ_UINT32 as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32;
                  (*mqc).end_of_byte_stream_counter =
                    (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c << 9 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 7 as libc::c_int as OPJ_UINT32
                }
              } else {
                (*mqc).bp = (*mqc).bp.offset(1);
                c = (c as libc::c_uint).wrapping_add(l_c << 8 as libc::c_int) as OPJ_UINT32
                  as OPJ_UINT32;
                ct = 8 as libc::c_int as OPJ_UINT32
              }
            }
            a <<= 1 as libc::c_int;
            c <<= 1 as libc::c_int;
            ct = ct.wrapping_sub(1);
            if !(a < 0x8000 as libc::c_int as libc::c_uint) {
              break;
            }
          }
        } else {
          c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int) as OPJ_UINT32
            as OPJ_UINT32;
          if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            if a < (**curctx).qeval {
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            } else {
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_0: OPJ_UINT32 = 0;
                l_c_0 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_0 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_0 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_0 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            v = (**curctx).mps
          }
        }
        if v == 0 {
          current_block_1828 = 2979737022853876585;
        } else {
          curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(
            (0 as libc::c_int
              + 9 as libc::c_int
              + 5 as libc::c_int
              + 3 as libc::c_int
              + 1 as libc::c_int) as OPJ_UINT32 as isize,
          ) as *mut *const opj_mqc_state_t;
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              runlen = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              runlen = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_1: OPJ_UINT32 = 0;
                l_c_1 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_1 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_1 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_1 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                runlen = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                runlen = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_2: OPJ_UINT32 = 0;
                  l_c_2 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_2 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_2 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_2 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              runlen = (**curctx).mps
            }
          }
          a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
          if (c >> 16 as libc::c_int) < (**curctx).qeval {
            if a < (**curctx).qeval {
              a = (**curctx).qeval;
              v = (**curctx).mps;
              *curctx = (**curctx).nmps
            } else {
              a = (**curctx).qeval;
              v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
              *curctx = (**curctx).nlps
            }
            loop {
              if ct == 0 as libc::c_int as libc::c_uint {
                let mut l_c_3: OPJ_UINT32 = 0;
                l_c_3 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                  if l_c_3 > 0x8f as libc::c_int as libc::c_uint {
                    c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                      as OPJ_UINT32 as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32;
                    (*mqc).end_of_byte_stream_counter =
                      (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_3 << 9 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 7 as libc::c_int as OPJ_UINT32
                  }
                } else {
                  (*mqc).bp = (*mqc).bp.offset(1);
                  c = (c as libc::c_uint).wrapping_add(l_c_3 << 8 as libc::c_int) as OPJ_UINT32
                    as OPJ_UINT32;
                  ct = 8 as libc::c_int as OPJ_UINT32
                }
              }
              a <<= 1 as libc::c_int;
              c <<= 1 as libc::c_int;
              ct = ct.wrapping_sub(1);
              if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                break;
              }
            }
          } else {
            c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
              as OPJ_UINT32 as OPJ_UINT32;
            if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
              if a < (**curctx).qeval {
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              } else {
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_4: OPJ_UINT32 = 0;
                  l_c_4 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_4 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_4 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_4 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              v = (**curctx).mps
            }
          }
          runlen = runlen << 1 as libc::c_int | v;
          let mut current_block_1045: u64;
          match runlen {
            0 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_452: u64;
                if 1 as libc::c_int == 0 {
                  let mut ctxt1 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_5: OPJ_UINT32 = 0;
                        l_c_5 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_5 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_5 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_5 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_6: OPJ_UINT32 = 0;
                          l_c_6 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_6 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_6 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_6 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_452 = 16116475693927719422;
                  } else {
                    current_block_452 = 14785121481331406365;
                  }
                } else {
                  current_block_452 = 14785121481331406365;
                }
                match current_block_452 {
                  14785121481331406365 => {
                    let mut lu = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      0 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_7: OPJ_UINT32 = 0;
                          l_c_7 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_7 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_7 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_7 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_8: OPJ_UINT32 = 0;
                            l_c_8 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_8 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_8 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_8 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu) as libc::c_uint;
                    *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh267 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh267 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    let ref mut fresh268 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh268 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
                    if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 1 as libc::c_int == 0
                    {
                      let mut north =
                        flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                      *north |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh269 = *north.offset(-(1 as libc::c_int) as isize);
                      *fresh269 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh270 = *north.offset(1 as libc::c_int as isize);
                      *fresh270 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south =
                        flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                      *south |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh271 = *south.offset(-(1 as libc::c_int) as isize);
                      *fresh271 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh272 = *south.offset(1 as libc::c_int as isize);
                      *fresh272 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 16883973241247358161;
            }
            1 => {
              current_block_1045 = 16883973241247358161;
            }
            2 => {
              current_block_1045 = 14822033375505401679;
            }
            3 => {
              current_block_1045 = 10086818405175024066;
            }
            _ => {
              current_block_1045 = 14550598362925142901;
            }
          }
          match current_block_1045 {
            16883973241247358161 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_648: u64;
                if partial == 0 {
                  let mut ctxt1_0 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_0 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_9: OPJ_UINT32 = 0;
                        l_c_9 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_9 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_9 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_9 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_10: OPJ_UINT32 = 0;
                          l_c_10 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_10 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_10 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_10 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_648 = 2685306683101115345;
                  } else {
                    current_block_648 = 6535359582234382107;
                  }
                } else {
                  current_block_648 = 6535359582234382107;
                }
                match current_block_648 {
                  6535359582234382107 => {
                    let mut lu_0 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      1 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_0
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_11: OPJ_UINT32 = 0;
                          l_c_11 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_11 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_11 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_11 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_12: OPJ_UINT32 = 0;
                            l_c_12 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_12 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_12 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_12 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_0) as libc::c_uint;
                    *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh273 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh273 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    let ref mut fresh274 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh274 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
                    if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_0 =
                        flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                      *north_0 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh275 = *north_0.offset(-(1 as libc::c_int) as isize);
                      *fresh275 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh276 = *north_0.offset(1 as libc::c_int as isize);
                      *fresh276 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_0 =
                        flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                      *south_0 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh277 = *south_0.offset(-(1 as libc::c_int) as isize);
                      *fresh277 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh278 = *south_0.offset(1 as libc::c_int as isize);
                      *fresh278 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 14822033375505401679;
            }
            _ => {}
          }
          match current_block_1045 {
            14822033375505401679 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_844: u64;
                if partial == 0 {
                  let mut ctxt1_1 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_1 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_13: OPJ_UINT32 = 0;
                        l_c_13 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_13 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_13 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_13 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_14: OPJ_UINT32 = 0;
                          l_c_14 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_14 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_14 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_14 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_844 = 10171483622901412108;
                  } else {
                    current_block_844 = 16329970517403260811;
                  }
                } else {
                  current_block_844 = 16329970517403260811;
                }
                match current_block_844 {
                  16329970517403260811 => {
                    let mut lu_1 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      2 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_1
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_15: OPJ_UINT32 = 0;
                          l_c_15 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_15 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_15 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_15 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_16: OPJ_UINT32 = 0;
                            l_c_16 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_16 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_16 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_16 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_1) as libc::c_uint;
                    *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh279 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh279 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    let ref mut fresh280 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh280 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
                    if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_1 =
                        flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                      *north_1 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh281 = *north_1.offset(-(1 as libc::c_int) as isize);
                      *fresh281 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh282 = *north_1.offset(1 as libc::c_int as isize);
                      *fresh282 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_1 =
                        flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                      *south_1 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh283 = *south_1.offset(-(1 as libc::c_int) as isize);
                      *fresh283 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh284 = *south_1.offset(1 as libc::c_int as isize);
                      *fresh284 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
              partial = 0 as libc::c_int as OPJ_UINT32;
              current_block_1045 = 10086818405175024066;
            }
            _ => {}
          }
          match current_block_1045 {
            10086818405175024066 => {
              if 0 as libc::c_int == 0
                || flags
                  & ((1 as libc::c_uint) << 4 as libc::c_int
                    | (1 as libc::c_uint) << 21 as libc::c_int)
                    << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
                  == 0
              {
                let mut current_block_1040: u64;
                if partial == 0 {
                  let mut ctxt1_2 = opj_t1_getctxno_zc(
                    mqc,
                    flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
                  ) as OPJ_UINT32;
                  curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_2 as isize)
                    as *mut *const opj_mqc_state_t;
                  a =
                    (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
                  if (c >> 16 as libc::c_int) < (**curctx).qeval {
                    if a < (**curctx).qeval {
                      a = (**curctx).qeval;
                      v = (**curctx).mps;
                      *curctx = (**curctx).nmps
                    } else {
                      a = (**curctx).qeval;
                      v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                      *curctx = (**curctx).nlps
                    }
                    loop {
                      if ct == 0 as libc::c_int as libc::c_uint {
                        let mut l_c_17: OPJ_UINT32 = 0;
                        l_c_17 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                        if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                          if l_c_17 > 0x8f as libc::c_int as libc::c_uint {
                            c = (c as libc::c_uint)
                              .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32;
                            (*mqc).end_of_byte_stream_counter =
                              (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_17 << 9 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 7 as libc::c_int as OPJ_UINT32
                          }
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_17 << 8 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32
                        }
                      }
                      a <<= 1 as libc::c_int;
                      c <<= 1 as libc::c_int;
                      ct = ct.wrapping_sub(1);
                      if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                        break;
                      }
                    }
                  } else {
                    c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                      as OPJ_UINT32 as OPJ_UINT32;
                    if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                    {
                      if a < (**curctx).qeval {
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      } else {
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_18: OPJ_UINT32 = 0;
                          l_c_18 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_18 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_18 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_18 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      v = (**curctx).mps
                    }
                  }
                  if v == 0 {
                    current_block_1040 = 10095684397801062190;
                  } else {
                    current_block_1040 = 3833074357519289034;
                  }
                } else {
                  current_block_1040 = 3833074357519289034;
                }
                match current_block_1040 {
                  3833074357519289034 => {
                    let mut lu_2 = opj_t1_getctxtno_sc_or_spb_index(
                      flags,
                      *flagsp.offset(-(1 as libc::c_int) as isize),
                      *flagsp.offset(1 as libc::c_int as isize),
                      3 as libc::c_int as OPJ_UINT32,
                    );
                    curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                      as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                      lu_2
                    )
                      as OPJ_UINT32
                      as isize) as *mut *const opj_mqc_state_t;
                    a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32
                      as OPJ_UINT32;
                    if (c >> 16 as libc::c_int) < (**curctx).qeval {
                      if a < (**curctx).qeval {
                        a = (**curctx).qeval;
                        v = (**curctx).mps;
                        *curctx = (**curctx).nmps
                      } else {
                        a = (**curctx).qeval;
                        v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                        *curctx = (**curctx).nlps
                      }
                      loop {
                        if ct == 0 as libc::c_int as libc::c_uint {
                          let mut l_c_19: OPJ_UINT32 = 0;
                          l_c_19 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                          if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                            if l_c_19 > 0x8f as libc::c_int as libc::c_uint {
                              c = (c as libc::c_uint)
                                .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32;
                              (*mqc).end_of_byte_stream_counter =
                                (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_19 << 9 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 7 as libc::c_int as OPJ_UINT32
                            }
                          } else {
                            (*mqc).bp = (*mqc).bp.offset(1);
                            c = (c as libc::c_uint).wrapping_add(l_c_19 << 8 as libc::c_int)
                              as OPJ_UINT32 as OPJ_UINT32;
                            ct = 8 as libc::c_int as OPJ_UINT32
                          }
                        }
                        a <<= 1 as libc::c_int;
                        c <<= 1 as libc::c_int;
                        ct = ct.wrapping_sub(1);
                        if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                          break;
                        }
                      }
                    } else {
                      c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                        as OPJ_UINT32 as OPJ_UINT32;
                      if a & 0x8000 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                      {
                        if a < (**curctx).qeval {
                          v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                          *curctx = (**curctx).nlps
                        } else {
                          v = (**curctx).mps;
                          *curctx = (**curctx).nmps
                        }
                        loop {
                          if ct == 0 as libc::c_int as libc::c_uint {
                            let mut l_c_20: OPJ_UINT32 = 0;
                            l_c_20 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                            if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                              if l_c_20 > 0x8f as libc::c_int as libc::c_uint {
                                c = (c as libc::c_uint)
                                  .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 8 as libc::c_int as OPJ_UINT32;
                                (*mqc).end_of_byte_stream_counter =
                                  (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                              } else {
                                (*mqc).bp = (*mqc).bp.offset(1);
                                c = (c as libc::c_uint).wrapping_add(l_c_20 << 9 as libc::c_int)
                                  as OPJ_UINT32 as OPJ_UINT32;
                                ct = 7 as libc::c_int as OPJ_UINT32
                              }
                            } else {
                              (*mqc).bp = (*mqc).bp.offset(1);
                              c = (c as libc::c_uint).wrapping_add(l_c_20 << 8 as libc::c_int)
                                as OPJ_UINT32 as OPJ_UINT32;
                              ct = 8 as libc::c_int as OPJ_UINT32
                            }
                          }
                          a <<= 1 as libc::c_int;
                          c <<= 1 as libc::c_int;
                          ct = ct.wrapping_sub(1);
                          if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                            break;
                          }
                        }
                      } else {
                        v = (**curctx).mps
                      }
                    }
                    v = v ^ opj_t1_getspb(lu_2) as libc::c_uint;
                    *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                      if v != 0 { -oneplushalf } else { oneplushalf };
                    let ref mut fresh285 = *flagsp.offset(-(1 as libc::c_int) as isize);
                    *fresh285 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    let ref mut fresh286 = *flagsp.offset(1 as libc::c_int as isize);
                    *fresh286 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                      << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
                    if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint
                      && 0 as libc::c_int == 0
                    {
                      let mut north_2 =
                        flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                      *north_2 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                      let ref mut fresh287 = *north_2.offset(-(1 as libc::c_int) as isize);
                      *fresh287 |= (1 as libc::c_uint) << 17 as libc::c_int;
                      let ref mut fresh288 = *north_2.offset(1 as libc::c_int as isize);
                      *fresh288 |= (1 as libc::c_uint) << 15 as libc::c_int
                    }
                    if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                      let mut south_2 =
                        flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                      *south_2 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                      let ref mut fresh289 = *south_2.offset(-(1 as libc::c_int) as isize);
                      *fresh289 |= (1 as libc::c_uint) << 2 as libc::c_int;
                      let ref mut fresh290 = *south_2.offset(1 as libc::c_int as isize);
                      *fresh290 |= (1 as libc::c_uint) << 0 as libc::c_int
                    }
                  }
                  _ => {}
                }
              }
            }
            _ => {}
          }
          current_block_1828 = 10917493918967617673;
        }
      } else {
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1236: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_3 = opj_t1_getctxno_zc(
              mqc,
              flags >> (0 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_3 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_21: OPJ_UINT32 = 0;
                  l_c_21 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_21 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_21 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_21 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_22: OPJ_UINT32 = 0;
                    l_c_22 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_22 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_22 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_22 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1236 = 1928523173154854502;
            } else {
              current_block_1236 = 12323252596376536154;
            }
          } else {
            current_block_1236 = 12323252596376536154;
          }
          match current_block_1236 {
            12323252596376536154 => {
              let mut lu_3 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                0 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_3
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_23: OPJ_UINT32 = 0;
                    l_c_23 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_23 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_23 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_23 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_24: OPJ_UINT32 = 0;
                      l_c_24 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_24 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_24 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_24 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_3) as libc::c_uint;
              *data.offset((0 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh291 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh291 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              let ref mut fresh292 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh292 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(0 as libc::c_int as libc::c_uint);
              if 0 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 1 as libc::c_int == 0 {
                let mut north_3 =
                  flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                *north_3 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh293 = *north_3.offset(-(1 as libc::c_int) as isize);
                *fresh293 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh294 = *north_3.offset(1 as libc::c_int as isize);
                *fresh294 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 0 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_3 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                *south_3 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh295 = *south_3.offset(-(1 as libc::c_int) as isize);
                *fresh295 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh296 = *south_3.offset(1 as libc::c_int as isize);
                *fresh296 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1431: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_4 = opj_t1_getctxno_zc(
              mqc,
              flags >> (1 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_4 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_25: OPJ_UINT32 = 0;
                  l_c_25 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_25 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_25 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_25 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_26: OPJ_UINT32 = 0;
                    l_c_26 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_26 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_26 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_26 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1431 = 7360758368027237918;
            } else {
              current_block_1431 = 1372330848231551451;
            }
          } else {
            current_block_1431 = 1372330848231551451;
          }
          match current_block_1431 {
            1372330848231551451 => {
              let mut lu_4 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                1 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_4
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_27: OPJ_UINT32 = 0;
                    l_c_27 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_27 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_27 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_27 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_28: OPJ_UINT32 = 0;
                      l_c_28 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_28 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_28 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_28 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_4) as libc::c_uint;
              *data.offset((1 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh297 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh297 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              let ref mut fresh298 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh298 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(1 as libc::c_int as libc::c_uint);
              if 1 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_4 =
                  flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                *north_4 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh299 = *north_4.offset(-(1 as libc::c_int) as isize);
                *fresh299 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh300 = *north_4.offset(1 as libc::c_int as isize);
                *fresh300 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 1 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_4 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                *south_4 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh301 = *south_4.offset(-(1 as libc::c_int) as isize);
                *fresh301 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh302 = *south_4.offset(1 as libc::c_int as isize);
                *fresh302 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1626: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_5 = opj_t1_getctxno_zc(
              mqc,
              flags >> (2 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_5 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_29: OPJ_UINT32 = 0;
                  l_c_29 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_29 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_29 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_29 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_30: OPJ_UINT32 = 0;
                    l_c_30 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_30 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_30 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_30 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1626 = 15312420746610300268;
            } else {
              current_block_1626 = 911994807774668979;
            }
          } else {
            current_block_1626 = 911994807774668979;
          }
          match current_block_1626 {
            911994807774668979 => {
              let mut lu_5 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                2 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_5
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_31: OPJ_UINT32 = 0;
                    l_c_31 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_31 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_31 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_31 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_32: OPJ_UINT32 = 0;
                      l_c_32 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_32 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_32 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_32 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_5) as libc::c_uint;
              *data.offset((2 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh303 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh303 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              let ref mut fresh304 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh304 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint);
              if 2 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_5 =
                  flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                *north_5 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh305 = *north_5.offset(-(1 as libc::c_int) as isize);
                *fresh305 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh306 = *north_5.offset(1 as libc::c_int as isize);
                *fresh306 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 2 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_5 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                *south_5 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh307 = *south_5.offset(-(1 as libc::c_int) as isize);
                *fresh307 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh308 = *south_5.offset(1 as libc::c_int as isize);
                *fresh308 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        if 1 as libc::c_int == 0
          || flags
            & ((1 as libc::c_uint) << 4 as libc::c_int | (1 as libc::c_uint) << 21 as libc::c_int)
              << (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint)
            == 0
        {
          let mut current_block_1821: u64;
          if 0 as libc::c_int == 0 {
            let mut ctxt1_6 = opj_t1_getctxno_zc(
              mqc,
              flags >> (3 as libc::c_int as libc::c_uint).wrapping_mul(3 as libc::c_uint),
            ) as OPJ_UINT32;
            curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset(ctxt1_6 as isize)
              as *mut *const opj_mqc_state_t;
            a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
            if (c >> 16 as libc::c_int) < (**curctx).qeval {
              if a < (**curctx).qeval {
                a = (**curctx).qeval;
                v = (**curctx).mps;
                *curctx = (**curctx).nmps
              } else {
                a = (**curctx).qeval;
                v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                *curctx = (**curctx).nlps
              }
              loop {
                if ct == 0 as libc::c_int as libc::c_uint {
                  let mut l_c_33: OPJ_UINT32 = 0;
                  l_c_33 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                  if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                    if l_c_33 > 0x8f as libc::c_int as libc::c_uint {
                      c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                        as OPJ_UINT32 as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32;
                      (*mqc).end_of_byte_stream_counter =
                        (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_33 << 9 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 7 as libc::c_int as OPJ_UINT32
                    }
                  } else {
                    (*mqc).bp = (*mqc).bp.offset(1);
                    c = (c as libc::c_uint).wrapping_add(l_c_33 << 8 as libc::c_int) as OPJ_UINT32
                      as OPJ_UINT32;
                    ct = 8 as libc::c_int as OPJ_UINT32
                  }
                }
                a <<= 1 as libc::c_int;
                c <<= 1 as libc::c_int;
                ct = ct.wrapping_sub(1);
                if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                  break;
                }
              }
            } else {
              c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                as OPJ_UINT32 as OPJ_UINT32;
              if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                if a < (**curctx).qeval {
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                } else {
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_34: OPJ_UINT32 = 0;
                    l_c_34 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_34 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_34 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_34 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                v = (**curctx).mps
              }
            }
            if v == 0 {
              current_block_1821 = 17698901117243856307;
            } else {
              current_block_1821 = 12990703611517236731;
            }
          } else {
            current_block_1821 = 12990703611517236731;
          }
          match current_block_1821 {
            12990703611517236731 => {
              let mut lu_6 = opj_t1_getctxtno_sc_or_spb_index(
                flags,
                *flagsp.offset(-(1 as libc::c_int) as isize),
                *flagsp.offset(1 as libc::c_int as isize),
                3 as libc::c_int as OPJ_UINT32,
              );
              curctx = &mut *(*mqc).ctxs.as_mut_ptr().offset((opj_t1_getctxno_sc
                as unsafe extern "C" fn(_: OPJ_UINT32) -> OPJ_BYTE)(
                lu_6
              ) as OPJ_UINT32 as isize) as *mut *const opj_mqc_state_t;
              a = (a as libc::c_uint).wrapping_sub((**curctx).qeval) as OPJ_UINT32 as OPJ_UINT32;
              if (c >> 16 as libc::c_int) < (**curctx).qeval {
                if a < (**curctx).qeval {
                  a = (**curctx).qeval;
                  v = (**curctx).mps;
                  *curctx = (**curctx).nmps
                } else {
                  a = (**curctx).qeval;
                  v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                  *curctx = (**curctx).nlps
                }
                loop {
                  if ct == 0 as libc::c_int as libc::c_uint {
                    let mut l_c_35: OPJ_UINT32 = 0;
                    l_c_35 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                    if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                      if l_c_35 > 0x8f as libc::c_int as libc::c_uint {
                        c = (c as libc::c_uint).wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32;
                        (*mqc).end_of_byte_stream_counter =
                          (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_35 << 9 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 7 as libc::c_int as OPJ_UINT32
                      }
                    } else {
                      (*mqc).bp = (*mqc).bp.offset(1);
                      c = (c as libc::c_uint).wrapping_add(l_c_35 << 8 as libc::c_int) as OPJ_UINT32
                        as OPJ_UINT32;
                      ct = 8 as libc::c_int as OPJ_UINT32
                    }
                  }
                  a <<= 1 as libc::c_int;
                  c <<= 1 as libc::c_int;
                  ct = ct.wrapping_sub(1);
                  if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                    break;
                  }
                }
              } else {
                c = (c as libc::c_uint).wrapping_sub((**curctx).qeval << 16 as libc::c_int)
                  as OPJ_UINT32 as OPJ_UINT32;
                if a & 0x8000 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                  if a < (**curctx).qeval {
                    v = ((**curctx).mps == 0) as libc::c_int as OPJ_UINT32;
                    *curctx = (**curctx).nlps
                  } else {
                    v = (**curctx).mps;
                    *curctx = (**curctx).nmps
                  }
                  loop {
                    if ct == 0 as libc::c_int as libc::c_uint {
                      let mut l_c_36: OPJ_UINT32 = 0;
                      l_c_36 = *(*mqc).bp.offset(1 as libc::c_int as isize) as OPJ_UINT32;
                      if *(*mqc).bp as libc::c_int == 0xff as libc::c_int {
                        if l_c_36 > 0x8f as libc::c_int as libc::c_uint {
                          c = (c as libc::c_uint)
                            .wrapping_add(0xff00 as libc::c_int as libc::c_uint)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 8 as libc::c_int as OPJ_UINT32;
                          (*mqc).end_of_byte_stream_counter =
                            (*mqc).end_of_byte_stream_counter.wrapping_add(1)
                        } else {
                          (*mqc).bp = (*mqc).bp.offset(1);
                          c = (c as libc::c_uint).wrapping_add(l_c_36 << 9 as libc::c_int)
                            as OPJ_UINT32 as OPJ_UINT32;
                          ct = 7 as libc::c_int as OPJ_UINT32
                        }
                      } else {
                        (*mqc).bp = (*mqc).bp.offset(1);
                        c = (c as libc::c_uint).wrapping_add(l_c_36 << 8 as libc::c_int)
                          as OPJ_UINT32 as OPJ_UINT32;
                        ct = 8 as libc::c_int as OPJ_UINT32
                      }
                    }
                    a <<= 1 as libc::c_int;
                    c <<= 1 as libc::c_int;
                    ct = ct.wrapping_sub(1);
                    if !(a < 0x8000 as libc::c_int as libc::c_uint) {
                      break;
                    }
                  }
                } else {
                  v = (**curctx).mps
                }
              }
              v = v ^ opj_t1_getspb(lu_6) as libc::c_uint;
              *data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize) =
                if v != 0 { -oneplushalf } else { oneplushalf };
              let ref mut fresh309 = *flagsp.offset(-(1 as libc::c_int) as isize);
              *fresh309 |= ((1 as libc::c_uint) << 5 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              flags |= (v << 19 as libc::c_int | (1 as libc::c_uint) << 4 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              let ref mut fresh310 = *flagsp.offset(1 as libc::c_int as isize);
              *fresh310 |= ((1 as libc::c_uint) << 3 as libc::c_int)
                << (3 as libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint);
              if 3 as libc::c_int as libc::c_uint == 0 as libc::c_uint && 0 as libc::c_int == 0 {
                let mut north_6 =
                  flagsp.offset(-((*t1).w.wrapping_add(2 as libc::c_uint) as isize));
                *north_6 |= v << 31 as libc::c_int | (1 as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh311 = *north_6.offset(-(1 as libc::c_int) as isize);
                *fresh311 |= (1 as libc::c_uint) << 17 as libc::c_int;
                let ref mut fresh312 = *north_6.offset(1 as libc::c_int as isize);
                *fresh312 |= (1 as libc::c_uint) << 15 as libc::c_int
              }
              if 3 as libc::c_int as libc::c_uint == 3 as libc::c_uint {
                let mut south_6 = flagsp.offset((*t1).w.wrapping_add(2 as libc::c_uint) as isize);
                *south_6 |= v << 18 as libc::c_int | (1 as libc::c_uint) << 1 as libc::c_int;
                let ref mut fresh313 = *south_6.offset(-(1 as libc::c_int) as isize);
                *fresh313 |= (1 as libc::c_uint) << 2 as libc::c_int;
                let ref mut fresh314 = *south_6.offset(1 as libc::c_int as isize);
                *fresh314 |= (1 as libc::c_uint) << 0 as libc::c_int
              }
            }
            _ => {}
          }
        }
        current_block_1828 = 10917493918967617673;
      }
      match current_block_1828 {
        10917493918967617673 => {
          *flagsp = flags
            & !((1 as libc::c_uint) << 21 as libc::c_int
              | (1 as libc::c_uint) << 24 as libc::c_int
              | (1 as libc::c_uint) << 27 as libc::c_int
              | (1 as libc::c_uint) << 30 as libc::c_int)
        }
        _ => {}
      }
      i = i.wrapping_add(1);
      data = data.offset(1);
      flagsp = flagsp.offset(1)
    }
    k = (k as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
      as OPJ_UINT32;
    data = data.offset((3 as libc::c_int as libc::c_uint).wrapping_mul(l_w) as isize);
    flagsp = flagsp.offset(2 as libc::c_int as isize)
  }
  (*mqc).curctx = curctx;
  (*mqc).c = c;
  (*mqc).a = a;
  (*mqc).ct = ct;
  if k < (*t1).h {
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < l_w {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < (*t1).h.wrapping_sub(k) {
        opj_t1_dec_clnpass_step(
          t1,
          flagsp,
          data.offset(j.wrapping_mul(l_w) as isize),
          oneplushalf,
          j,
          1 as libc::c_int as OPJ_UINT32,
        );
        j = j.wrapping_add(1)
      }
      *flagsp &= !((1 as libc::c_uint) << 21 as libc::c_int
        | (1 as libc::c_uint) << 24 as libc::c_int
        | (1 as libc::c_uint) << 27 as libc::c_int
        | (1 as libc::c_uint) << 30 as libc::c_int);
      i = i.wrapping_add(1);
      flagsp = flagsp.offset(1);
      data = data.offset(1)
    }
  };
}
unsafe extern "C" fn opj_t1_dec_clnpass(
  mut t1: *mut opj_t1_t,
  mut bpno: OPJ_INT32,
  mut cblksty: OPJ_INT32,
) {
  if (*t1).w == 64 as libc::c_int as libc::c_uint && (*t1).h == 64 as libc::c_int as libc::c_uint {
    if cblksty & 0x8 as libc::c_int != 0 {
      opj_t1_dec_clnpass_64x64_vsc(t1, bpno);
    } else {
      opj_t1_dec_clnpass_64x64_novsc(t1, bpno);
    }
  } else if cblksty & 0x8 as libc::c_int != 0 {
    opj_t1_dec_clnpass_generic_vsc(t1, bpno);
  } else {
    opj_t1_dec_clnpass_generic_novsc(t1, bpno);
  }
  opj_t1_dec_clnpass_check_segsym(t1, cblksty);
}
/* * mod fixed_quality */
unsafe extern "C" fn opj_t1_getwmsedec(
  mut nmsedec: OPJ_INT32,
  mut compno: OPJ_UINT32,
  mut level: OPJ_UINT32,
  mut orient: OPJ_UINT32,
  mut bpno: OPJ_INT32,
  mut qmfbid: OPJ_UINT32,
  mut stepsize: OPJ_FLOAT64,
  mut _numcomps: OPJ_UINT32,
  mut mct_norms: *const OPJ_FLOAT64,
  mut mct_numcomps: OPJ_UINT32,
) -> OPJ_FLOAT64 {
  let mut w1 = 1 as libc::c_int as OPJ_FLOAT64; /* if (qmfbid == 0) */
  let mut w2: OPJ_FLOAT64 = 0.;
  let mut wmsedec: OPJ_FLOAT64 = 0.;
  if !mct_norms.is_null() && compno < mct_numcomps {
    w1 = *mct_norms.offset(compno as isize)
  }
  if qmfbid == 1 as libc::c_int as libc::c_uint {
    w2 = opj_dwt_getnorm(level, orient)
  } else {
    let log2_gain = if orient == 0 as libc::c_int as libc::c_uint {
      0 as libc::c_int
    } else if orient == 3 as libc::c_int as libc::c_uint {
      2 as libc::c_int
    } else {
      1 as libc::c_int
    };
    w2 = opj_dwt_getnorm_real(level, orient);
    /* Not sure this is right. But preserves past behaviour */
    stepsize /= ((1 as libc::c_int) << log2_gain) as libc::c_double
  }
  wmsedec = w1 * w2 * stepsize * ((1 as libc::c_int) << bpno) as libc::c_double;
  wmsedec *= wmsedec * nmsedec as libc::c_double / 8192.0f64;
  return wmsedec;
}
unsafe extern "C" fn opj_t1_allocate_buffers(
  mut t1: *mut opj_t1_t,
  mut w: OPJ_UINT32,
  mut h: OPJ_UINT32,
) -> OPJ_BOOL {
  let mut flagssize: OPJ_UINT32 = 0;
  let mut flags_stride: OPJ_UINT32 = 0;
  /* No risk of overflow. Prior checks ensure those assert are met */
  /* They are per the specification */

  assert!(w <= 1024 as libc::c_int as libc::c_uint);
  assert!(h <= 1024 as libc::c_int as libc::c_uint);
  assert!(w.wrapping_mul(h) <= 4096 as libc::c_int as libc::c_uint);
  /* encoder uses tile buffer, so no need to allocate */
  let mut datasize = w.wrapping_mul(h);
  if datasize > (*t1).datasize {
    opj_aligned_free((*t1).data as *mut libc::c_void);
    (*t1).data = opj_aligned_malloc(
      (datasize as libc::c_ulong).wrapping_mul(::std::mem::size_of::<OPJ_INT32>() as libc::c_ulong),
    ) as *mut OPJ_INT32;
    if (*t1).data.is_null() {
      /* FIXME event manager error callback */
      return 0 as libc::c_int;
    }
    (*t1).datasize = datasize
  }
  /* memset first arg is declared to never be null by gcc */
  if !(*t1).data.is_null() {
    memset(
      (*t1).data as *mut libc::c_void,
      0 as libc::c_int,
      (datasize as libc::c_ulong).wrapping_mul(::std::mem::size_of::<OPJ_INT32>() as libc::c_ulong),
    ); /* can't be 0U */
  }
  flags_stride = w.wrapping_add(2 as libc::c_uint);
  flagssize = h
    .wrapping_add(3 as libc::c_uint)
    .wrapping_div(4 as libc::c_uint)
    .wrapping_add(2 as libc::c_uint);
  flagssize = (flagssize as libc::c_uint).wrapping_mul(flags_stride) as OPJ_UINT32 as OPJ_UINT32;
  let mut p = 0 as *mut opj_flag_t;
  let mut x: OPJ_UINT32 = 0;
  let mut flags_height = h
    .wrapping_add(3 as libc::c_uint)
    .wrapping_div(4 as libc::c_uint);
  if flagssize > (*t1).flagssize {
    opj_aligned_free((*t1).flags as *mut libc::c_void);
    (*t1).flags = opj_aligned_malloc(
      (flagssize as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<opj_flag_t>() as libc::c_ulong),
    ) as *mut opj_flag_t;
    if (*t1).flags.is_null() {
      /* FIXME event manager error callback */
      return 0 as libc::c_int;
    }
  }
  (*t1).flagssize = flagssize;
  memset(
    (*t1).flags as *mut libc::c_void,
    0 as libc::c_int,
    (flagssize as libc::c_ulong).wrapping_mul(::std::mem::size_of::<opj_flag_t>() as libc::c_ulong),
  );
  p = &mut *(*t1).flags.offset(0 as libc::c_int as isize) as *mut opj_flag_t;
  x = 0 as libc::c_int as OPJ_UINT32;
  while x < flags_stride {
    /* magic value to hopefully stop any passes being interested in this entry */
    let fresh315 = p;
    p = p.offset(1);
    *fresh315 = (1 as libc::c_uint) << 21 as libc::c_int
      | (1 as libc::c_uint) << 24 as libc::c_int
      | (1 as libc::c_uint) << 27 as libc::c_int
      | (1 as libc::c_uint) << 30 as libc::c_int;
    x = x.wrapping_add(1)
  }
  p = &mut *(*t1).flags.offset(
    flags_height
      .wrapping_add(1 as libc::c_int as libc::c_uint)
      .wrapping_mul(flags_stride) as isize,
  ) as *mut opj_flag_t;
  x = 0 as libc::c_int as OPJ_UINT32;
  while x < flags_stride {
    /* magic value to hopefully stop any passes being interested in this entry */
    let fresh316 = p;
    p = p.offset(1);
    *fresh316 = (1 as libc::c_uint) << 21 as libc::c_int
      | (1 as libc::c_uint) << 24 as libc::c_int
      | (1 as libc::c_uint) << 27 as libc::c_int
      | (1 as libc::c_uint) << 30 as libc::c_int;
    x = x.wrapping_add(1)
  }
  if h.wrapping_rem(4 as libc::c_int as libc::c_uint) != 0 {
    let mut v = 0 as libc::c_int as OPJ_UINT32;
    p = &mut *(*t1)
      .flags
      .offset(flags_height.wrapping_mul(flags_stride) as isize) as *mut opj_flag_t;
    if h.wrapping_rem(4 as libc::c_int as libc::c_uint) == 1 as libc::c_int as libc::c_uint {
      v |= (1 as libc::c_uint) << 24 as libc::c_int
        | (1 as libc::c_uint) << 27 as libc::c_int
        | (1 as libc::c_uint) << 30 as libc::c_int
    } else if h.wrapping_rem(4 as libc::c_int as libc::c_uint) == 2 as libc::c_int as libc::c_uint {
      v |= (1 as libc::c_uint) << 27 as libc::c_int | (1 as libc::c_uint) << 30 as libc::c_int
    } else if h.wrapping_rem(4 as libc::c_int as libc::c_uint) == 3 as libc::c_int as libc::c_uint {
      v |= (1 as libc::c_uint) << 30 as libc::c_int
    }
    x = 0 as libc::c_int as OPJ_UINT32;
    while x < flags_stride {
      let fresh317 = p;
      p = p.offset(1);
      *fresh317 = v;
      x = x.wrapping_add(1)
    }
  }
  (*t1).w = w;
  (*t1).h = h;
  return 1 as libc::c_int;
}
/* ----------------------------------------------------------------------- */
/* ----------------------------------------------------------------------- */
/* *
 * Creates a new Tier 1 handle
 * and initializes the look-up tables of the Tier-1 coder/decoder
 * @return a new T1 handle if successful, returns NULL otherwise
*/
#[no_mangle]
pub unsafe extern "C" fn opj_t1_create(mut isEncoder: OPJ_BOOL) -> *mut opj_t1_t {
  let mut l_t1 = 0 as *mut opj_t1_t;
  l_t1 = opj_calloc(
    1 as libc::c_int as size_t,
    ::std::mem::size_of::<opj_t1_t>() as libc::c_ulong,
  ) as *mut opj_t1_t;
  if l_t1.is_null() {
    return 0 as *mut opj_t1_t;
  }
  (*l_t1).encoder = isEncoder;
  return l_t1;
}
/* *
 * Destroys a previously created T1 handle
 *
 * @param p_t1 Tier 1 handle to destroy
*/
#[no_mangle]
pub unsafe extern "C" fn opj_t1_destroy(mut p_t1: *mut opj_t1_t) {
  if p_t1.is_null() {
    return;
  }
  if !(*p_t1).data.is_null() {
    opj_aligned_free((*p_t1).data as *mut libc::c_void);
    (*p_t1).data = 0 as *mut OPJ_INT32
  }
  if !(*p_t1).flags.is_null() {
    opj_aligned_free((*p_t1).flags as *mut libc::c_void);
    (*p_t1).flags = 0 as *mut opj_flag_t
  }
  opj_free((*p_t1).cblkdatabuffer as *mut libc::c_void);
  opj_free(p_t1 as *mut libc::c_void);
}
unsafe extern "C" fn opj_t1_destroy_wrapper(mut t1: *mut libc::c_void) {
  opj_t1_destroy(t1 as *mut opj_t1_t);
}
unsafe extern "C" fn opj_t1_clbl_decode_processor(
  mut user_data: *mut libc::c_void,
  mut tls: *mut opj_tls_t,
) {
  let mut cblk = 0 as *mut opj_tcd_cblk_dec_t;
  let mut band = 0 as *mut opj_tcd_band_t;
  let mut tilec = 0 as *mut opj_tcd_tilecomp_t;
  let mut tccp = 0 as *mut opj_tccp_t;
  let mut datap = 0 as *mut OPJ_INT32;
  let mut cblk_w: OPJ_UINT32 = 0;
  let mut cblk_h: OPJ_UINT32 = 0;
  let mut x: OPJ_INT32 = 0;
  let mut y: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut job = 0 as *mut opj_t1_cblk_decode_processing_job_t;
  let mut t1 = 0 as *mut opj_t1_t;
  let mut resno: OPJ_UINT32 = 0;
  let mut tile_w: OPJ_UINT32 = 0;
  job = user_data as *mut opj_t1_cblk_decode_processing_job_t;
  cblk = (*job).cblk;
  if (*job).whole_tile_decoding == 0 {
    cblk_w = ((*cblk).x1 - (*cblk).x0) as OPJ_UINT32;
    cblk_h = ((*cblk).y1 - (*cblk).y0) as OPJ_UINT32;
    (*cblk).decoded_data = opj_aligned_malloc(
      (::std::mem::size_of::<OPJ_INT32>() as libc::c_ulong)
        .wrapping_mul(cblk_w as libc::c_ulong)
        .wrapping_mul(cblk_h as libc::c_ulong),
    ) as *mut OPJ_INT32;
    if (*cblk).decoded_data.is_null() {
      if !(*job).p_manager_mutex.is_null() {
        opj_mutex_lock((*job).p_manager_mutex);
      }
      opj_event_msg(
        (*job).p_manager,
        1 as libc::c_int,
        b"Cannot allocate cblk->decoded_data\n\x00" as *const u8 as *const libc::c_char,
      );
      if !(*job).p_manager_mutex.is_null() {
        opj_mutex_unlock((*job).p_manager_mutex);
      }
      ::std::ptr::write_volatile((*job).pret, 0 as libc::c_int);
      opj_free(job as *mut libc::c_void);
      return;
    }
    /* Zero-init required */
    memset(
      (*cblk).decoded_data as *mut libc::c_void,
      0 as libc::c_int,
      (::std::mem::size_of::<OPJ_INT32>() as libc::c_ulong)
        .wrapping_mul(cblk_w as libc::c_ulong)
        .wrapping_mul(cblk_h as libc::c_ulong),
    );
  } else if !(*cblk).decoded_data.is_null() {
    /* Not sure if that code path can happen, but better be */
    /* safe than sorry */
    opj_aligned_free((*cblk).decoded_data as *mut libc::c_void);
    (*cblk).decoded_data = 0 as *mut OPJ_INT32
  }
  resno = (*job).resno;
  band = (*job).band;
  tilec = (*job).tilec;
  tccp = (*job).tccp;
  tile_w = ((*(*tilec).resolutions.offset(
    (*tilec)
      .minimum_num_resolutions
      .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
  ))
  .x1
    - (*(*tilec).resolutions.offset(
      (*tilec)
        .minimum_num_resolutions
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
    ))
    .x0) as OPJ_UINT32;
  if *(*job).pret == 0 {
    opj_free(job as *mut libc::c_void);
    return;
  }
  t1 = opj_tls_get(tls, 0 as libc::c_int) as *mut opj_t1_t;
  if t1.is_null() {
    t1 = opj_t1_create(0 as libc::c_int);
    if t1.is_null() {
      opj_event_msg(
        (*job).p_manager,
        1 as libc::c_int,
        b"Cannot allocate Tier 1 handle\n\x00" as *const u8 as *const libc::c_char,
      );
      ::std::ptr::write_volatile((*job).pret, 0 as libc::c_int);
      opj_free(job as *mut libc::c_void);
      return;
    }
    if opj_tls_set(
      tls,
      0 as libc::c_int,
      t1 as *mut libc::c_void,
      Some(opj_t1_destroy_wrapper as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    ) == 0
    {
      opj_event_msg(
        (*job).p_manager,
        1 as libc::c_int,
        b"Unable to set t1 handle as TLS\n\x00" as *const u8 as *const libc::c_char,
      );
      opj_t1_destroy(t1);
      ::std::ptr::write_volatile((*job).pret, 0 as libc::c_int);
      opj_free(job as *mut libc::c_void);
      return;
    }
  }
  (*t1).mustuse_cblkdatabuffer = (*job).mustuse_cblkdatabuffer;
  if (*tccp).cblksty & 0x40 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint {
    if 0 as libc::c_int
      == opj_t1_ht_decode_cblk(
        t1,
        cblk,
        (*band).bandno,
        (*tccp).roishift as OPJ_UINT32,
        (*tccp).cblksty,
        (*job).p_manager,
        (*job).p_manager_mutex,
        (*job).check_pterm,
      )
    {
      ::std::ptr::write_volatile((*job).pret, 0 as libc::c_int);
      opj_free(job as *mut libc::c_void);
      return;
    }
  } else if 0 as libc::c_int
    == opj_t1_decode_cblk(
      t1,
      cblk,
      (*band).bandno,
      (*tccp).roishift as OPJ_UINT32,
      (*tccp).cblksty,
      (*job).p_manager,
      (*job).p_manager_mutex,
      (*job).check_pterm,
    )
  {
    ::std::ptr::write_volatile((*job).pret, 0 as libc::c_int);
    opj_free(job as *mut libc::c_void);
    return;
  }
  x = (*cblk).x0 - (*band).x0;
  y = (*cblk).y0 - (*band).y0;
  if (*band).bandno & 1 as libc::c_int as libc::c_uint != 0 {
    let mut pres: *mut opj_tcd_resolution_t = &mut *(*tilec)
      .resolutions
      .offset(resno.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
      as *mut opj_tcd_resolution_t;
    x += (*pres).x1 - (*pres).x0
  }
  if (*band).bandno & 2 as libc::c_int as libc::c_uint != 0 {
    let mut pres_0: *mut opj_tcd_resolution_t = &mut *(*tilec)
      .resolutions
      .offset(resno.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
      as *mut opj_tcd_resolution_t;
    y += (*pres_0).y1 - (*pres_0).y0
  }
  datap = if !(*cblk).decoded_data.is_null() {
    (*cblk).decoded_data
  } else {
    (*t1).data
  };
  cblk_w = (*t1).w;
  cblk_h = (*t1).h;
  if (*tccp).roishift != 0 {
    if (*tccp).roishift >= 31 as libc::c_int {
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < cblk_h {
        i = 0 as libc::c_int as OPJ_UINT32;
        while i < cblk_w {
          *datap.offset(j.wrapping_mul(cblk_w).wrapping_add(i) as isize) = 0 as libc::c_int;
          i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
      }
    } else {
      let mut thresh = (1 as libc::c_int) << (*tccp).roishift;
      j = 0 as libc::c_int as OPJ_UINT32;
      while j < cblk_h {
        i = 0 as libc::c_int as OPJ_UINT32;
        while i < cblk_w {
          let mut val = *datap.offset(j.wrapping_mul(cblk_w).wrapping_add(i) as isize);
          let mut mag = abs(val);
          if mag >= thresh {
            mag >>= (*tccp).roishift;
            *datap.offset(j.wrapping_mul(cblk_w).wrapping_add(i) as isize) =
              if val < 0 as libc::c_int { -mag } else { mag }
          }
          i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
      }
    }
  }
  /* Both can be non NULL if for example decoding a full tile and then */
  /* partially a tile. In which case partial decoding should be the */
  /* priority */
  assert!(!(*cblk).decoded_data.is_null() || !(*tilec).data.is_null()); /* if (tccp->qmfbid == 0) */
  if !(*cblk).decoded_data.is_null() {
    let mut cblk_size = cblk_w.wrapping_mul(cblk_h); /* resno */
    if (*tccp).qmfbid == 1 as libc::c_int as libc::c_uint {
      i = 0 as libc::c_int as OPJ_UINT32;
      while i < cblk_size {
        let ref mut fresh318 = *datap.offset(i as isize);
        *fresh318 /= 2 as libc::c_int;
        i = i.wrapping_add(1)
      }
    } else {
      let stepsize = 0.5f32 * (*band).stepsize;
      i = 0 as libc::c_int as OPJ_UINT32;
      while i < cblk_size {
        let mut tmp = *datap as OPJ_FLOAT32 * stepsize;
        memcpy(
          datap as *mut libc::c_void,
          &mut tmp as *mut OPJ_FLOAT32 as *const libc::c_void,
          ::std::mem::size_of::<OPJ_FLOAT32>() as libc::c_ulong,
        );
        datap = datap.offset(1);
        i = i.wrapping_add(1)
      }
    }
  } else if (*tccp).qmfbid == 1 as libc::c_int as libc::c_uint {
    let mut tiledp: *mut OPJ_INT32 = &mut *(*tilec).data.offset(
      (y as OPJ_SIZE_T)
        .wrapping_mul(tile_w as libc::c_ulong)
        .wrapping_add(x as OPJ_SIZE_T) as isize,
    ) as *mut OPJ_INT32;
    j = 0 as libc::c_int as OPJ_UINT32;
    while j < cblk_h {
      i = 0 as libc::c_int as OPJ_UINT32;
      while i < cblk_w & !(3 as libc::c_uint) {
        let mut tmp0 = *datap.offset(
          j.wrapping_mul(cblk_w)
            .wrapping_add(i)
            .wrapping_add(0 as libc::c_uint) as isize,
        );
        let mut tmp1 = *datap.offset(
          j.wrapping_mul(cblk_w)
            .wrapping_add(i)
            .wrapping_add(1 as libc::c_uint) as isize,
        );
        let mut tmp2 = *datap.offset(
          j.wrapping_mul(cblk_w)
            .wrapping_add(i)
            .wrapping_add(2 as libc::c_uint) as isize,
        );
        let mut tmp3 = *datap.offset(
          j.wrapping_mul(cblk_w)
            .wrapping_add(i)
            .wrapping_add(3 as libc::c_uint) as isize,
        );
        *tiledp.offset(
          (j as libc::c_ulong)
            .wrapping_mul(tile_w as OPJ_SIZE_T)
            .wrapping_add(i as libc::c_ulong)
            .wrapping_add(0 as libc::c_uint as libc::c_ulong) as isize,
        ) = tmp0 / 2 as libc::c_int;
        *tiledp.offset(
          (j as libc::c_ulong)
            .wrapping_mul(tile_w as OPJ_SIZE_T)
            .wrapping_add(i as libc::c_ulong)
            .wrapping_add(1 as libc::c_uint as libc::c_ulong) as isize,
        ) = tmp1 / 2 as libc::c_int;
        *tiledp.offset(
          (j as libc::c_ulong)
            .wrapping_mul(tile_w as OPJ_SIZE_T)
            .wrapping_add(i as libc::c_ulong)
            .wrapping_add(2 as libc::c_uint as libc::c_ulong) as isize,
        ) = tmp2 / 2 as libc::c_int;
        *tiledp.offset(
          (j as libc::c_ulong)
            .wrapping_mul(tile_w as OPJ_SIZE_T)
            .wrapping_add(i as libc::c_ulong)
            .wrapping_add(3 as libc::c_uint as libc::c_ulong) as isize,
        ) = tmp3 / 2 as libc::c_int;
        i = (i as libc::c_uint).wrapping_add(4 as libc::c_uint) as OPJ_UINT32 as OPJ_UINT32
      }
      while i < cblk_w {
        let mut tmp_0 = *datap.offset(j.wrapping_mul(cblk_w).wrapping_add(i) as isize);
        *tiledp.offset(
          (j as libc::c_ulong)
            .wrapping_mul(tile_w as OPJ_SIZE_T)
            .wrapping_add(i as libc::c_ulong) as isize,
        ) = tmp_0 / 2 as libc::c_int;
        i = i.wrapping_add(1)
      }
      j = j.wrapping_add(1)
    }
  } else {
    let stepsize_0 = 0.5f32 * (*band).stepsize;
    let mut tiledp_0 = &mut *(*tilec).data.offset(
      (y as OPJ_SIZE_T)
        .wrapping_mul(tile_w as libc::c_ulong)
        .wrapping_add(x as OPJ_SIZE_T) as isize,
    ) as *mut OPJ_INT32 as *mut OPJ_FLOAT32;
    j = 0 as libc::c_int as OPJ_UINT32;
    while j < cblk_h {
      let mut tiledp2 = tiledp_0;
      i = 0 as libc::c_int as OPJ_UINT32;
      while i < cblk_w {
        let mut tmp_1 = *datap as OPJ_FLOAT32 * stepsize_0;
        *tiledp2 = tmp_1;
        datap = datap.offset(1);
        tiledp2 = tiledp2.offset(1);
        i = i.wrapping_add(1)
      }
      tiledp_0 = tiledp_0.offset(tile_w as isize);
      j = j.wrapping_add(1)
    }
  }
  opj_free(job as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn opj_t1_decode_cblks(
  mut tcd: *mut opj_tcd_t,
  mut pret: *mut OPJ_BOOL,
  mut tilec: *mut opj_tcd_tilecomp_t,
  mut tccp: *mut opj_tccp_t,
  mut p_manager: *mut opj_event_mgr_t,
  mut p_manager_mutex: *mut opj_mutex_t,
  mut check_pterm: OPJ_BOOL,
) {
  let mut tp = (*tcd).thread_pool;
  let mut resno: OPJ_UINT32 = 0;
  let mut bandno: OPJ_UINT32 = 0;
  let mut precno: OPJ_UINT32 = 0;
  let mut cblkno: OPJ_UINT32 = 0;
  resno = 0 as libc::c_int as OPJ_UINT32;
  while resno < (*tilec).minimum_num_resolutions {
    let mut res: *mut opj_tcd_resolution_t =
      &mut *(*tilec).resolutions.offset(resno as isize) as *mut opj_tcd_resolution_t;
    bandno = 0 as libc::c_int as OPJ_UINT32;
    while bandno < (*res).numbands {
      let mut band: *mut opj_tcd_band_t =
        &mut *(*res).bands.as_mut_ptr().offset(bandno as isize) as *mut opj_tcd_band_t;
      precno = 0 as libc::c_int as OPJ_UINT32;
      while precno < (*res).pw.wrapping_mul((*res).ph) {
        let mut precinct: *mut opj_tcd_precinct_t =
          &mut *(*band).precincts.offset(precno as isize) as *mut opj_tcd_precinct_t;
        if opj_tcd_is_subband_area_of_interest(
          tcd,
          (*tilec).compno,
          resno,
          (*band).bandno,
          (*precinct).x0 as OPJ_UINT32,
          (*precinct).y0 as OPJ_UINT32,
          (*precinct).x1 as OPJ_UINT32,
          (*precinct).y1 as OPJ_UINT32,
        ) == 0
        {
          cblkno = 0 as libc::c_int as OPJ_UINT32;
          while cblkno < (*precinct).cw.wrapping_mul((*precinct).ch) {
            let mut cblk: *mut opj_tcd_cblk_dec_t =
              &mut *(*precinct).cblks.dec.offset(cblkno as isize) as *mut opj_tcd_cblk_dec_t;
            if !(*cblk).decoded_data.is_null() {
              opj_aligned_free((*cblk).decoded_data as *mut libc::c_void);
              (*cblk).decoded_data = 0 as *mut OPJ_INT32
            }
            cblkno = cblkno.wrapping_add(1)
          }
        } else {
          let mut current_block_34: u64;
          cblkno = 0 as libc::c_int as OPJ_UINT32;
          while cblkno < (*precinct).cw.wrapping_mul((*precinct).ch) {
            let mut cblk_0: *mut opj_tcd_cblk_dec_t =
              &mut *(*precinct).cblks.dec.offset(cblkno as isize) as *mut opj_tcd_cblk_dec_t;
            let mut job = 0 as *mut opj_t1_cblk_decode_processing_job_t;
            if opj_tcd_is_subband_area_of_interest(
              tcd,
              (*tilec).compno,
              resno,
              (*band).bandno,
              (*cblk_0).x0 as OPJ_UINT32,
              (*cblk_0).y0 as OPJ_UINT32,
              (*cblk_0).x1 as OPJ_UINT32,
              (*cblk_0).y1 as OPJ_UINT32,
            ) == 0
            {
              if !(*cblk_0).decoded_data.is_null() {
                opj_aligned_free((*cblk_0).decoded_data as *mut libc::c_void);
                (*cblk_0).decoded_data = 0 as *mut OPJ_INT32
              }
            } else {
              if (*tcd).whole_tile_decoding == 0 {
                let mut cblk_w = ((*cblk_0).x1 - (*cblk_0).x0) as OPJ_UINT32;
                let mut cblk_h = ((*cblk_0).y1 - (*cblk_0).y0) as OPJ_UINT32;
                if !(*cblk_0).decoded_data.is_null() {
                  current_block_34 = 2370887241019905314;
                } else if cblk_w == 0 as libc::c_int as libc::c_uint
                  || cblk_h == 0 as libc::c_int as libc::c_uint
                {
                  current_block_34 = 2370887241019905314;
                } else {
                  current_block_34 = 11913429853522160501;
                }
              } else {
                current_block_34 = 11913429853522160501;
              }
              match current_block_34 {
                2370887241019905314 => {}
                _ => {
                  job = opj_calloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<opj_t1_cblk_decode_processing_job_t>() as libc::c_ulong,
                  ) as *mut opj_t1_cblk_decode_processing_job_t;
                  if job.is_null() {
                    ::std::ptr::write_volatile(pret, 0 as libc::c_int);
                    return;
                  }
                  (*job).whole_tile_decoding = (*tcd).whole_tile_decoding;
                  (*job).resno = resno;
                  (*job).cblk = cblk_0;
                  (*job).band = band;
                  (*job).tilec = tilec;
                  (*job).tccp = tccp;
                  (*job).pret = pret;
                  (*job).p_manager_mutex = p_manager_mutex;
                  (*job).p_manager = p_manager;
                  (*job).check_pterm = check_pterm;
                  (*job).mustuse_cblkdatabuffer =
                    (opj_thread_pool_get_thread_count(tp) > 1 as libc::c_int) as libc::c_int;
                  opj_thread_pool_submit_job(
                    tp,
                    Some(
                      opj_t1_clbl_decode_processor
                        as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut opj_tls_t) -> (),
                    ),
                    job as *mut libc::c_void,
                  );
                  if *pret == 0 {
                    return;
                  }
                }
              }
            }
            cblkno = cblkno.wrapping_add(1)
          }
        }
        precno = precno.wrapping_add(1)
        /* bandno */
        /* precno */
        /* cblkno */
      }
      bandno = bandno.wrapping_add(1)
    }
    resno = resno.wrapping_add(1)
  }
}
/* *
Decode 1 code-block
@param t1 T1 handle
@param cblk Code-block coding parameters
@param orient
@param roishift Region of interest shifting value
@param cblksty Code-block style
@param p_manager the event manager
@param p_manager_mutex mutex for the event manager
@param check_pterm whether PTERM correct termination should be checked
*/
unsafe extern "C" fn opj_t1_decode_cblk(
  mut t1: *mut opj_t1_t,
  mut cblk: *mut opj_tcd_cblk_dec_t,
  mut orient: OPJ_UINT32,
  mut roishift: OPJ_UINT32,
  mut cblksty: OPJ_UINT32,
  mut p_manager: *mut opj_event_mgr_t,
  mut p_manager_mutex: *mut opj_mutex_t,
  mut check_pterm: OPJ_BOOL,
) -> OPJ_BOOL {
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc; /* MQC component */
  let mut bpno_plus_one: OPJ_INT32 = 0; /* BYPASS mode */
  let mut passtype: OPJ_UINT32 = 0;
  let mut segno: OPJ_UINT32 = 0;
  let mut passno: OPJ_UINT32 = 0;
  let mut cblkdata = 0 as *mut OPJ_BYTE;
  let mut cblkdataindex = 0 as libc::c_int as OPJ_UINT32;
  let mut type_0 = 0 as libc::c_int as OPJ_BYTE;
  let mut original_t1_data = 0 as *mut OPJ_INT32;
  (*mqc).lut_ctxno_zc_orient = lut_ctxno_zc
    .as_ptr()
    .offset((orient << 9 as libc::c_int) as isize);
  if opj_t1_allocate_buffers(
    t1,
    ((*cblk).x1 - (*cblk).x0) as OPJ_UINT32,
    ((*cblk).y1 - (*cblk).y0) as OPJ_UINT32,
  ) == 0
  {
    return 0 as libc::c_int;
  }
  bpno_plus_one = roishift.wrapping_add((*cblk).numbps) as OPJ_INT32;
  if bpno_plus_one >= 31 as libc::c_int {
    if !p_manager_mutex.is_null() {
      opj_mutex_lock(p_manager_mutex);
    }
    opj_event_msg(
      p_manager,
      2 as libc::c_int,
      b"opj_t1_decode_cblk(): unsupported bpno_plus_one = %d >= 31\n\x00" as *const u8
        as *const libc::c_char,
      bpno_plus_one,
    );
    if !p_manager_mutex.is_null() {
      opj_mutex_unlock(p_manager_mutex);
    }
    return 0 as libc::c_int;
  }
  passtype = 2 as libc::c_int as OPJ_UINT32;
  opj_mqc_resetstates(mqc);
  opj_mqc_setstate(
    mqc,
    (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int)
      as OPJ_UINT32,
    0 as libc::c_int as OPJ_UINT32,
    46 as libc::c_int,
  );
  opj_mqc_setstate(
    mqc,
    (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int) as OPJ_UINT32,
    0 as libc::c_int as OPJ_UINT32,
    3 as libc::c_int,
  );
  opj_mqc_setstate(
    mqc,
    0 as libc::c_int as OPJ_UINT32,
    0 as libc::c_int as OPJ_UINT32,
    4 as libc::c_int,
  );
  /* Even if we have a single chunk, in multi-threaded decoding */
  /* the insertion of our synthetic marker might potentially override */
  /* valid codestream of other codeblocks decoded in parallel. */
  if (*cblk).numchunks > 1 as libc::c_int as libc::c_uint || (*t1).mustuse_cblkdatabuffer != 0 {
    let mut i: OPJ_UINT32 = 0;
    let mut cblk_len: OPJ_UINT32 = 0;
    /* Compute whole codeblock length from chunk lengths */
    cblk_len = 0 as libc::c_int as OPJ_UINT32;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < (*cblk).numchunks {
      cblk_len = (cblk_len as libc::c_uint).wrapping_add((*(*cblk).chunks.offset(i as isize)).len)
        as OPJ_UINT32 as OPJ_UINT32;
      i = i.wrapping_add(1)
    }
    /* Allocate temporary memory if needed */
    if cblk_len.wrapping_add(2 as libc::c_int as libc::c_uint) > (*t1).cblkdatabuffersize {
      cblkdata = opj_realloc(
        (*t1).cblkdatabuffer as *mut libc::c_void,
        cblk_len.wrapping_add(2 as libc::c_int as libc::c_uint) as size_t,
      ) as *mut OPJ_BYTE;
      if cblkdata.is_null() {
        return 0 as libc::c_int;
      }
      (*t1).cblkdatabuffer = cblkdata;
      memset(
        (*t1).cblkdatabuffer.offset(cblk_len as isize) as *mut libc::c_void,
        0 as libc::c_int,
        2 as libc::c_int as libc::c_ulong,
      );
      (*t1).cblkdatabuffersize = cblk_len.wrapping_add(2 as libc::c_int as libc::c_uint)
    }
    /* Concatenate all chunks */
    cblkdata = (*t1).cblkdatabuffer;
    cblk_len = 0 as libc::c_int as OPJ_UINT32;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < (*cblk).numchunks {
      memcpy(
        cblkdata.offset(cblk_len as isize) as *mut libc::c_void,
        (*(*cblk).chunks.offset(i as isize)).data as *const libc::c_void,
        (*(*cblk).chunks.offset(i as isize)).len as libc::c_ulong,
      );
      cblk_len = (cblk_len as libc::c_uint).wrapping_add((*(*cblk).chunks.offset(i as isize)).len)
        as OPJ_UINT32 as OPJ_UINT32;
      i = i.wrapping_add(1)
    }
  } else if (*cblk).numchunks == 1 as libc::c_int as libc::c_uint {
    cblkdata = (*(*cblk).chunks.offset(0 as libc::c_int as isize)).data
  } else {
    /* Not sure if that can happen in practice, but avoid Coverity to */
    /* think we will dereference a null cblkdta pointer */
    return 1 as libc::c_int;
  }
  /* For subtile decoding, directly decode in the decoded_data buffer of */
  /* the code-block. Hack t1->data to point to it, and restore it later */
  if !(*cblk).decoded_data.is_null() {
    original_t1_data = (*t1).data;
    (*t1).data = (*cblk).decoded_data
  }
  segno = 0 as libc::c_int as OPJ_UINT32;
  while segno < (*cblk).real_num_segs {
    let mut seg: *mut opj_tcd_seg_t =
      &mut *(*cblk).segs.offset(segno as isize) as *mut opj_tcd_seg_t;
    /* BYPASS mode */
    type_0 = if bpno_plus_one <= (*cblk).numbps as OPJ_INT32 - 4 as libc::c_int
      && passtype < 2 as libc::c_int as libc::c_uint
      && cblksty & 0x1 as libc::c_int as libc::c_uint != 0
    {
      1 as libc::c_int
    } else {
      0 as libc::c_int
    } as OPJ_BYTE;
    if type_0 as libc::c_int == 1 as libc::c_int {
      opj_mqc_raw_init_dec(
        mqc,
        cblkdata.offset(cblkdataindex as isize),
        (*seg).len,
        2 as libc::c_int as OPJ_UINT32,
      );
    } else {
      opj_mqc_init_dec(
        mqc,
        cblkdata.offset(cblkdataindex as isize),
        (*seg).len,
        2 as libc::c_int as OPJ_UINT32,
      );
    }
    cblkdataindex =
      (cblkdataindex as libc::c_uint).wrapping_add((*seg).len) as OPJ_UINT32 as OPJ_UINT32;
    passno = 0 as libc::c_int as OPJ_UINT32;
    while passno < (*seg).real_num_passes && bpno_plus_one >= 1 as libc::c_int {
      match passtype {
        0 => {
          if type_0 as libc::c_int == 1 as libc::c_int {
            opj_t1_dec_sigpass_raw(t1, bpno_plus_one, cblksty as OPJ_INT32);
          } else {
            opj_t1_dec_sigpass_mqc(t1, bpno_plus_one, cblksty as OPJ_INT32);
          }
        }
        1 => {
          if type_0 as libc::c_int == 1 as libc::c_int {
            opj_t1_dec_refpass_raw(t1, bpno_plus_one);
          } else {
            opj_t1_dec_refpass_mqc(t1, bpno_plus_one);
          }
        }
        2 => {
          opj_t1_dec_clnpass(t1, bpno_plus_one, cblksty as OPJ_INT32);
        }
        _ => {}
      }
      if cblksty & 0x2 as libc::c_int as libc::c_uint != 0
        && type_0 as libc::c_int == 0 as libc::c_int
      {
        opj_mqc_resetstates(mqc);
        opj_mqc_setstate(
          mqc,
          (0 as libc::c_int
            + 9 as libc::c_int
            + 5 as libc::c_int
            + 3 as libc::c_int
            + 1 as libc::c_int) as OPJ_UINT32,
          0 as libc::c_int as OPJ_UINT32,
          46 as libc::c_int,
        );
        opj_mqc_setstate(
          mqc,
          (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int) as OPJ_UINT32,
          0 as libc::c_int as OPJ_UINT32,
          3 as libc::c_int,
        );
        opj_mqc_setstate(
          mqc,
          0 as libc::c_int as OPJ_UINT32,
          0 as libc::c_int as OPJ_UINT32,
          4 as libc::c_int,
        );
      }
      passtype = passtype.wrapping_add(1);
      if passtype == 3 as libc::c_int as libc::c_uint {
        passtype = 0 as libc::c_int as OPJ_UINT32;
        bpno_plus_one -= 1
      }
      passno = passno.wrapping_add(1)
    }
    opq_mqc_finish_dec(mqc);
    segno = segno.wrapping_add(1)
  }
  if check_pterm != 0 {
    if (*mqc).bp.offset(2 as libc::c_int as isize) < (*mqc).end {
      if !p_manager_mutex.is_null() {
        opj_mutex_lock(p_manager_mutex);
      }
      opj_event_msg(
        p_manager,
        2 as libc::c_int,
        b"PTERM check failure: %d remaining bytes in code block (%d used / %d)\n\x00" as *const u8
          as *const libc::c_char,
        (*mqc).end.wrapping_offset_from((*mqc).bp) as libc::c_long as libc::c_int
          - 2 as libc::c_int,
        (*mqc).bp.wrapping_offset_from((*mqc).start) as libc::c_long as libc::c_int,
        (*mqc).end.wrapping_offset_from((*mqc).start) as libc::c_long as libc::c_int,
      );
      if !p_manager_mutex.is_null() {
        opj_mutex_unlock(p_manager_mutex);
      }
    } else if (*mqc).end_of_byte_stream_counter > 2 as libc::c_int as libc::c_uint {
      if !p_manager_mutex.is_null() {
        opj_mutex_lock(p_manager_mutex);
      }
      opj_event_msg(
        p_manager,
        2 as libc::c_int,
        b"PTERM check failure: %d synthetized 0xFF markers read\n\x00" as *const u8
          as *const libc::c_char,
        (*mqc).end_of_byte_stream_counter,
      );
      if !p_manager_mutex.is_null() {
        opj_mutex_unlock(p_manager_mutex);
      }
    }
  }
  /* Restore original t1->data is needed */
  if !(*cblk).decoded_data.is_null() {
    (*t1).data = original_t1_data
  }
  return 1 as libc::c_int;
}
/* * Procedure to deal with a asynchronous code-block encoding job.
 *
 * @param user_data Pointer to a opj_t1_cblk_encode_processing_job_t* structure
 * @param tls       TLS handle.
 */
unsafe extern "C" fn opj_t1_cblk_encode_processor(
  mut user_data: *mut libc::c_void,
  mut tls: *mut opj_tls_t,
) {
  let mut job = user_data as *mut opj_t1_cblk_encode_processing_job_t; /* OPJ_TRUE == T1 for encoding */
  let mut cblk = (*job).cblk; /* if (tccp->qmfbid == 0) */
  let mut band: *const opj_tcd_band_t = (*job).band;
  let mut tilec: *const opj_tcd_tilecomp_t = (*job).tilec;
  let mut tccp: *const opj_tccp_t = (*job).tccp;
  let resno = (*job).resno;
  let mut t1 = 0 as *mut opj_t1_t;
  let tile_w = ((*tilec).x1 - (*tilec).x0) as OPJ_UINT32;
  let mut tiledp = 0 as *mut OPJ_INT32;
  let mut cblk_w: OPJ_UINT32 = 0;
  let mut cblk_h: OPJ_UINT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut x = (*cblk).x0 - (*band).x0;
  let mut y = (*cblk).y0 - (*band).y0;
  if *(*job).pret == 0 {
    opj_free(job as *mut libc::c_void);
    return;
  }
  t1 = opj_tls_get(tls, 0 as libc::c_int) as *mut opj_t1_t;
  if t1.is_null() {
    t1 = opj_t1_create(1 as libc::c_int);
    opj_tls_set(
      tls,
      0 as libc::c_int,
      t1 as *mut libc::c_void,
      Some(opj_t1_destroy_wrapper as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
  }
  if (*band).bandno & 1 as libc::c_int as libc::c_uint != 0 {
    let mut pres: *mut opj_tcd_resolution_t = &mut *(*tilec)
      .resolutions
      .offset(resno.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
      as *mut opj_tcd_resolution_t;
    x += (*pres).x1 - (*pres).x0
  }
  if (*band).bandno & 2 as libc::c_int as libc::c_uint != 0 {
    let mut pres_0: *mut opj_tcd_resolution_t = &mut *(*tilec)
      .resolutions
      .offset(resno.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
      as *mut opj_tcd_resolution_t;
    y += (*pres_0).y1 - (*pres_0).y0
  }
  if opj_t1_allocate_buffers(
    t1,
    ((*cblk).x1 - (*cblk).x0) as OPJ_UINT32,
    ((*cblk).y1 - (*cblk).y0) as OPJ_UINT32,
  ) == 0
  {
    ::std::ptr::write_volatile((*job).pret, 0 as libc::c_int);
    opj_free(job as *mut libc::c_void);
    return;
  }
  cblk_w = (*t1).w;
  cblk_h = (*t1).h;
  tiledp = &mut *(*tilec).data.offset(
    (y as OPJ_SIZE_T)
      .wrapping_mul(tile_w as libc::c_ulong)
      .wrapping_add(x as OPJ_SIZE_T) as isize,
  ) as *mut OPJ_INT32;
  if (*tccp).qmfbid == 1 as libc::c_int as libc::c_uint {
    let mut tiledp_u = tiledp as *mut OPJ_UINT32;
    let mut t1data = (*t1).data as *mut OPJ_UINT32;
    /* Do multiplication on unsigned type, even if the
     * underlying type is signed, to avoid potential
     * int overflow on large value (the output will be
     * incorrect in such situation, but whatever...)
     * This assumes complement-to-2 signed integer
     * representation
     * Fixes https://github.com/uclouvain/openjpeg/issues/1053
     */
    j = 0 as libc::c_int as OPJ_UINT32;
    while j < cblk_h & !(3 as libc::c_uint) {
      i = 0 as libc::c_int as OPJ_UINT32;
      while i < cblk_w {
        *t1data.offset(0 as libc::c_int as isize) = *tiledp_u.offset(
          j.wrapping_add(0 as libc::c_int as libc::c_uint)
            .wrapping_mul(tile_w)
            .wrapping_add(i) as isize,
        ) << 7 as libc::c_int - 1 as libc::c_int;
        *t1data.offset(1 as libc::c_int as isize) = *tiledp_u.offset(
          j.wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_mul(tile_w)
            .wrapping_add(i) as isize,
        ) << 7 as libc::c_int - 1 as libc::c_int;
        *t1data.offset(2 as libc::c_int as isize) = *tiledp_u.offset(
          j.wrapping_add(2 as libc::c_int as libc::c_uint)
            .wrapping_mul(tile_w)
            .wrapping_add(i) as isize,
        ) << 7 as libc::c_int - 1 as libc::c_int;
        *t1data.offset(3 as libc::c_int as isize) = *tiledp_u.offset(
          j.wrapping_add(3 as libc::c_int as libc::c_uint)
            .wrapping_mul(tile_w)
            .wrapping_add(i) as isize,
        ) << 7 as libc::c_int - 1 as libc::c_int;
        t1data = t1data.offset(4 as libc::c_int as isize);
        i = i.wrapping_add(1)
      }
      j = (j as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
        as OPJ_UINT32
    }
    if j < cblk_h {
      i = 0 as libc::c_int as OPJ_UINT32;
      while i < cblk_w {
        let mut k: OPJ_UINT32 = 0;
        k = j;
        while k < cblk_h {
          *t1data.offset(0 as libc::c_int as isize) = *tiledp_u
            .offset(k.wrapping_mul(tile_w).wrapping_add(i) as isize)
            << 7 as libc::c_int - 1 as libc::c_int;
          t1data = t1data.offset(1);
          k = k.wrapping_add(1)
        }
        i = i.wrapping_add(1)
      }
    }
  } else {
    let mut tiledp_f = tiledp as *mut OPJ_FLOAT32;
    let mut t1data_0 = (*t1).data;
    /* Change from "natural" order to "zigzag" order of T1 passes */
    /* Change from "natural" order to "zigzag" order of T1 passes */
    j = 0 as libc::c_int as OPJ_UINT32; /* fixed_quality */
    while j < cblk_h & !(3 as libc::c_uint) {
      i = 0 as libc::c_int as OPJ_UINT32; /* compno  */
      while i < cblk_w {
        *t1data_0.offset(0 as libc::c_int as isize) = opj_lrintf(
          *tiledp_f.offset(
            j.wrapping_add(0 as libc::c_int as libc::c_uint)
              .wrapping_mul(tile_w)
              .wrapping_add(i) as isize,
          ) / (*band).stepsize
            * ((1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        ) as OPJ_INT32;
        *t1data_0.offset(1 as libc::c_int as isize) = opj_lrintf(
          *tiledp_f.offset(
            j.wrapping_add(1 as libc::c_int as libc::c_uint)
              .wrapping_mul(tile_w)
              .wrapping_add(i) as isize,
          ) / (*band).stepsize
            * ((1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        ) as OPJ_INT32;
        *t1data_0.offset(2 as libc::c_int as isize) = opj_lrintf(
          *tiledp_f.offset(
            j.wrapping_add(2 as libc::c_int as libc::c_uint)
              .wrapping_mul(tile_w)
              .wrapping_add(i) as isize,
          ) / (*band).stepsize
            * ((1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        ) as OPJ_INT32;
        *t1data_0.offset(3 as libc::c_int as isize) = opj_lrintf(
          *tiledp_f.offset(
            j.wrapping_add(3 as libc::c_int as libc::c_uint)
              .wrapping_mul(tile_w)
              .wrapping_add(i) as isize,
          ) / (*band).stepsize
            * ((1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int) as libc::c_float,
        ) as OPJ_INT32;
        t1data_0 = t1data_0.offset(4 as libc::c_int as isize);
        i = i.wrapping_add(1)
      }
      j = (j as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as OPJ_UINT32
        as OPJ_UINT32
    }
    if j < cblk_h {
      i = 0 as libc::c_int as OPJ_UINT32;
      while i < cblk_w {
        let mut k_0: OPJ_UINT32 = 0;
        k_0 = j;
        while k_0 < cblk_h {
          *t1data_0.offset(0 as libc::c_int as isize) = opj_lrintf(
            *tiledp_f.offset(k_0.wrapping_mul(tile_w).wrapping_add(i) as isize) / (*band).stepsize
              * ((1 as libc::c_int) << 7 as libc::c_int - 1 as libc::c_int) as libc::c_float,
          ) as OPJ_INT32;
          t1data_0 = t1data_0.offset(1);
          k_0 = k_0.wrapping_add(1)
        }
        i = i.wrapping_add(1)
      }
    }
  }
  let mut cumwmsedec = opj_t1_encode_cblk(
    t1,
    cblk,
    (*band).bandno,
    (*job).compno,
    (*tilec)
      .numresolutions
      .wrapping_sub(1 as libc::c_int as libc::c_uint)
      .wrapping_sub(resno),
    (*tccp).qmfbid,
    (*band).stepsize as OPJ_FLOAT64,
    (*tccp).cblksty,
    (*(*job).tile).numcomps,
    (*job).mct_norms,
    (*job).mct_numcomps,
  );
  if !(*job).mutex.is_null() {
    opj_mutex_lock((*job).mutex);
  }
  (*(*job).tile).distotile += cumwmsedec;
  if !(*job).mutex.is_null() {
    opj_mutex_unlock((*job).mutex);
  }
  opj_free(job as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn opj_t1_encode_cblks(
  mut tcd: *mut opj_tcd_t,
  mut tile: *mut opj_tcd_tile_t,
  mut tcp: *mut opj_tcp_t,
  mut mct_norms: *const OPJ_FLOAT64,
  mut mct_numcomps: OPJ_UINT32,
) -> OPJ_BOOL {
  let mut ret = 1 as libc::c_int;
  let mut tp = (*tcd).thread_pool;
  let mut compno: OPJ_UINT32 = 0;
  let mut resno: OPJ_UINT32 = 0;
  let mut bandno: OPJ_UINT32 = 0;
  let mut precno: OPJ_UINT32 = 0;
  let mut cblkno: OPJ_UINT32 = 0;
  let mut mutex = opj_mutex_create();
  (*tile).distotile = 0 as libc::c_int as OPJ_FLOAT64;
  compno = 0 as libc::c_int as OPJ_UINT32;
  's_19: while compno < (*tile).numcomps {
    let mut tilec: *mut opj_tcd_tilecomp_t =
      &mut *(*tile).comps.offset(compno as isize) as *mut opj_tcd_tilecomp_t;
    let mut tccp: *mut opj_tccp_t = &mut *(*tcp).tccps.offset(compno as isize) as *mut opj_tccp_t;
    resno = 0 as libc::c_int as OPJ_UINT32;
    while resno < (*tilec).numresolutions {
      let mut res: *mut opj_tcd_resolution_t =
        &mut *(*tilec).resolutions.offset(resno as isize) as *mut opj_tcd_resolution_t;
      bandno = 0 as libc::c_int as OPJ_UINT32;
      while bandno < (*res).numbands {
        let mut band: *mut opj_tcd_band_t =
          &mut *(*res).bands.as_mut_ptr().offset(bandno as isize) as *mut opj_tcd_band_t;
        /* resno  */
        /* bandno */
        /* precno */
        /* Skip empty bands */
        if !(opj_tcd_is_band_empty(band) != 0) {
          precno = 0 as libc::c_int as OPJ_UINT32;
          while precno < (*res).pw.wrapping_mul((*res).ph) {
            let mut prc: *mut opj_tcd_precinct_t =
              &mut *(*band).precincts.offset(precno as isize) as *mut opj_tcd_precinct_t;
            cblkno = 0 as libc::c_int as OPJ_UINT32;
            while cblkno < (*prc).cw.wrapping_mul((*prc).ch) {
              let mut cblk: *mut opj_tcd_cblk_enc_t =
                &mut *(*prc).cblks.enc.offset(cblkno as isize) as *mut opj_tcd_cblk_enc_t;
              let mut job = opj_calloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<opj_t1_cblk_encode_processing_job_t>() as libc::c_ulong,
              ) as *mut opj_t1_cblk_encode_processing_job_t;
              if job.is_null() {
                ::std::ptr::write_volatile(&mut ret as *mut OPJ_BOOL, 0 as libc::c_int);
                break 's_19;
              } else {
                (*job).compno = compno;
                (*job).tile = tile;
                (*job).resno = resno;
                (*job).cblk = cblk;
                (*job).band = band;
                (*job).tilec = tilec;
                (*job).tccp = tccp;
                (*job).mct_norms = mct_norms;
                (*job).mct_numcomps = mct_numcomps;
                (*job).pret = &mut ret;
                (*job).mutex = mutex;
                opj_thread_pool_submit_job(
                  tp,
                  Some(
                    opj_t1_cblk_encode_processor
                      as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut opj_tls_t) -> (),
                  ),
                  job as *mut libc::c_void,
                );
                cblkno = cblkno.wrapping_add(1)
              }
            }
            precno = precno.wrapping_add(1)
            /* cblkno */
          }
        }
        bandno = bandno.wrapping_add(1)
      }
      resno = resno.wrapping_add(1)
    }
    compno = compno.wrapping_add(1)
  }
  opj_thread_pool_wait_completion((*tcd).thread_pool, 0 as libc::c_int);
  if !mutex.is_null() {
    opj_mutex_destroy(mutex);
  }
  return ret;
}
/* Returns whether the pass (bpno, passtype) is terminated */
unsafe extern "C" fn opj_t1_enc_is_term_pass(
  mut cblk: *mut opj_tcd_cblk_enc_t,
  mut cblksty: OPJ_UINT32,
  mut bpno: OPJ_INT32,
  mut passtype: OPJ_UINT32,
) -> libc::c_int {
  /* Is it the last cleanup pass ? */
  if passtype == 2 as libc::c_int as libc::c_uint && bpno == 0 as libc::c_int {
    return 1 as libc::c_int;
  }
  if cblksty & 0x4 as libc::c_int as libc::c_uint != 0 {
    return 1 as libc::c_int;
  }
  if cblksty & 0x1 as libc::c_int as libc::c_uint != 0 {
    /* For bypass arithmetic bypass, terminate the 4th cleanup pass */
    if bpno == (*cblk).numbps as OPJ_INT32 - 4 as libc::c_int
      && passtype == 2 as libc::c_int as libc::c_uint
    {
      return 1 as libc::c_int;
    }
    /* and beyond terminate all the magnitude refinement passes (in raw) */
    /* and cleanup passes (in MQC) */
    if bpno < (*cblk).numbps as OPJ_INT32 - 4 as libc::c_int
      && passtype > 0 as libc::c_int as libc::c_uint
    {
      return 1 as libc::c_int;
    }
  }
  return 0 as libc::c_int;
}
/* * Return "cumwmsedec" that should be used to increase tile->distotile */
/* * mod fixed_quality */
unsafe extern "C" fn opj_t1_encode_cblk(
  mut t1: *mut opj_t1_t,
  mut cblk: *mut opj_tcd_cblk_enc_t,
  mut orient: OPJ_UINT32,
  mut compno: OPJ_UINT32,
  mut level: OPJ_UINT32,
  mut qmfbid: OPJ_UINT32,
  mut stepsize: OPJ_FLOAT64,
  mut cblksty: OPJ_UINT32,
  mut numcomps: OPJ_UINT32,
  mut mct_norms: *const OPJ_FLOAT64,
  mut mct_numcomps: OPJ_UINT32,
) -> libc::c_double {
  let mut cumwmsedec = 0.0f64; /* MQC component */
  let mut mqc: *mut opj_mqc_t = &mut (*t1).mqc;
  let mut passno: OPJ_UINT32 = 0;
  let mut bpno: OPJ_INT32 = 0;
  let mut passtype: OPJ_UINT32 = 0;
  let mut nmsedec = 0 as libc::c_int;
  let mut max: OPJ_INT32 = 0;
  let mut i: OPJ_UINT32 = 0;
  let mut j: OPJ_UINT32 = 0;
  let mut type_0 = 0 as libc::c_int as OPJ_BYTE;
  let mut tempwmsedec: OPJ_FLOAT64 = 0.;
  let mut datap = 0 as *mut OPJ_INT32;
  (*mqc).lut_ctxno_zc_orient = lut_ctxno_zc
    .as_ptr()
    .offset((orient << 9 as libc::c_int) as isize);
  max = 0 as libc::c_int;
  datap = (*t1).data;
  j = 0 as libc::c_int as OPJ_UINT32;
  while j < (*t1).h {
    let w = (*t1).w;
    i = 0 as libc::c_int as OPJ_UINT32;
    while i < w {
      let mut tmp = *datap;
      if tmp < 0 as libc::c_int {
        let mut tmp_unsigned: OPJ_UINT32 = 0;
        max = opj_int_max(max, -tmp);
        tmp_unsigned = if tmp >= 0 as libc::c_int {
          tmp as OPJ_UINT32
        } else {
          (-tmp as OPJ_UINT32) | 0x80000000 as libc::c_uint
        };
        memcpy(
          datap as *mut libc::c_void,
          &mut tmp_unsigned as *mut OPJ_UINT32 as *const libc::c_void,
          ::std::mem::size_of::<OPJ_INT32>() as libc::c_ulong,
        );
      } else {
        max = opj_int_max(max, tmp)
      }
      i = i.wrapping_add(1);
      datap = datap.offset(1)
    }
    j = j.wrapping_add(1)
  }
  (*cblk).numbps = if max != 0 {
    (opj_int_floorlog2(max) + 1 as libc::c_int - (7 as libc::c_int - 1 as libc::c_int))
      as OPJ_UINT32
  } else {
    0 as libc::c_int as libc::c_uint
  };
  if (*cblk).numbps == 0 as libc::c_int as libc::c_uint {
    (*cblk).totalpasses = 0 as libc::c_int as OPJ_UINT32;
    return cumwmsedec;
  }
  bpno = (*cblk)
    .numbps
    .wrapping_sub(1 as libc::c_int as libc::c_uint) as OPJ_INT32;
  passtype = 2 as libc::c_int as OPJ_UINT32;
  opj_mqc_resetstates(mqc);
  opj_mqc_setstate(
    mqc,
    (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int)
      as OPJ_UINT32,
    0 as libc::c_int as OPJ_UINT32,
    46 as libc::c_int,
  );
  opj_mqc_setstate(
    mqc,
    (0 as libc::c_int + 9 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int) as OPJ_UINT32,
    0 as libc::c_int as OPJ_UINT32,
    3 as libc::c_int,
  );
  opj_mqc_setstate(
    mqc,
    0 as libc::c_int as OPJ_UINT32,
    0 as libc::c_int as OPJ_UINT32,
    4 as libc::c_int,
  );
  opj_mqc_init_enc(mqc, (*cblk).data);
  passno = 0 as libc::c_int as OPJ_UINT32;
  while bpno >= 0 as libc::c_int {
    let mut pass: *mut opj_tcd_pass_t =
      &mut *(*cblk).passes.offset(passno as isize) as *mut opj_tcd_pass_t;
    type_0 = if bpno < (*cblk).numbps as OPJ_INT32 - 4 as libc::c_int
      && passtype < 2 as libc::c_int as libc::c_uint
      && cblksty & 0x1 as libc::c_int as libc::c_uint != 0
    {
      1 as libc::c_int
    } else {
      0 as libc::c_int
    } as OPJ_BYTE;
    /* If the previous pass was terminating, we need to reset the encoder */
    if passno > 0 as libc::c_int as libc::c_uint
      && (*(*cblk)
        .passes
        .offset(passno.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
      .term() as libc::c_int
        != 0
    {
      if type_0 as libc::c_int == 1 as libc::c_int {
        opj_mqc_bypass_init_enc(mqc);
      } else {
        opj_mqc_restart_init_enc(mqc);
      }
    }
    match passtype {
      0 => {
        opj_t1_enc_sigpass(t1, bpno, &mut nmsedec, type_0, cblksty);
      }
      1 => {
        opj_t1_enc_refpass(t1, bpno, &mut nmsedec, type_0);
      }
      2 => {
        opj_t1_enc_clnpass(t1, bpno, &mut nmsedec, cblksty);
        /* code switch SEGMARK (i.e. SEGSYM) */
        if cblksty & 0x20 as libc::c_int as libc::c_uint != 0 {
          opj_mqc_segmark_enc(mqc);
        }
      }
      _ => {}
    }
    /* fixed_quality */
    tempwmsedec = opj_t1_getwmsedec(
      nmsedec,
      compno,
      level,
      orient,
      bpno,
      qmfbid,
      stepsize,
      numcomps,
      mct_norms,
      mct_numcomps,
    );
    cumwmsedec += tempwmsedec;
    (*pass).distortiondec = cumwmsedec;
    if opj_t1_enc_is_term_pass(cblk, cblksty, bpno, passtype) != 0 {
      /* If it is a terminated pass, terminate it */
      if type_0 as libc::c_int == 1 as libc::c_int {
        opj_mqc_bypass_flush_enc(
          mqc,
          (cblksty & 0x10 as libc::c_int as libc::c_uint) as OPJ_BOOL,
        );
      } else if cblksty & 0x10 as libc::c_int as libc::c_uint != 0 {
        opj_mqc_erterm_enc(mqc);
      } else {
        opj_mqc_flush(mqc);
      }
      (*pass).set_term(1 as libc::c_int as OPJ_BITFIELD);
      (*pass).rate = opj_mqc_numbytes(mqc)
    } else {
      /* Non terminated pass */
      let mut rate_extra_bytes: OPJ_UINT32 = 0;
      if type_0 as libc::c_int == 1 as libc::c_int {
        rate_extra_bytes = opj_mqc_bypass_get_extra_bytes(
          mqc,
          (cblksty & 0x10 as libc::c_int as libc::c_uint) as OPJ_BOOL,
        )
      } else {
        rate_extra_bytes = 3 as libc::c_int as OPJ_UINT32
      }
      (*pass).set_term(0 as libc::c_int as OPJ_BITFIELD);
      (*pass).rate = opj_mqc_numbytes(mqc).wrapping_add(rate_extra_bytes)
    }
    passtype = passtype.wrapping_add(1);
    if passtype == 3 as libc::c_int as libc::c_uint {
      passtype = 0 as libc::c_int as OPJ_UINT32;
      bpno -= 1
    }
    /* Code-switch "RESET" */
    if cblksty & 0x2 as libc::c_int as libc::c_uint != 0 {
      opj_mqc_reset_enc(mqc);
    }
    passno = passno.wrapping_add(1)
  }
  (*cblk).totalpasses = passno;
  if (*cblk).totalpasses != 0 {
    /* Make sure that pass rates are increasing */
    let mut last_pass_rate = opj_mqc_numbytes(mqc);
    passno = (*cblk).totalpasses;
    while passno > 0 as libc::c_int as libc::c_uint {
      passno = passno.wrapping_sub(1);
      let mut pass_0: *mut opj_tcd_pass_t =
        &mut *(*cblk).passes.offset(passno as isize) as *mut opj_tcd_pass_t;
      if (*pass_0).rate > last_pass_rate {
        (*pass_0).rate = last_pass_rate
      } else {
        last_pass_rate = (*pass_0).rate
      }
    }
  }
  passno = 0 as libc::c_int as OPJ_UINT32;
  while passno < (*cblk).totalpasses {
    let mut pass_1: *mut opj_tcd_pass_t =
      &mut *(*cblk).passes.offset(passno as isize) as *mut opj_tcd_pass_t;
    /* Prevent generation of FF as last data byte of a pass*/
    /* For terminating passes, the flushing procedure ensured this already */
    assert!((*pass_1).rate > 0 as libc::c_int as libc::c_uint);
    if *(*cblk).data.offset(
      (*pass_1)
        .rate
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
    ) as libc::c_int
      == 0xff as libc::c_int
    {
      (*pass_1).rate = (*pass_1).rate.wrapping_sub(1)
    }
    (*pass_1).len = (*pass_1)
      .rate
      .wrapping_sub(if passno == 0 as libc::c_int as libc::c_uint {
        0 as libc::c_int as libc::c_uint
      } else {
        (*(*cblk)
          .passes
          .offset(passno.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
        .rate
      });
    passno = passno.wrapping_add(1)
  }
  return cumwmsedec;
}
