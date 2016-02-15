#![allow(non_snake_case)]

extern crate libc;
extern crate vorbis_sys;

#[repr(C)]
pub struct ovectl_ratemanage_arg {
    pub management_active: libc::c_int,
    pub bitrate_hard_min: libc::c_long,
    pub bitrate_hard_max: libc::c_long,
    pub bitrate_hard_window: libc::c_double,
    pub bitrate_av_lo: libc::c_long,
    pub bitrate_av_hi: libc::c_long,
    pub bitrate_av_window: libc::c_double,
    pub bitrate_av_window_center: libc::c_double
}
#[repr(C)]
pub struct ovectl_ratemanage2_arg {
    pub management_active: libc::c_int,
    pub bitrate_limit_min_kbps: libc::c_long,
    pub bitrate_limit_max_kbps: libc::c_long,
    pub bitrate_limit_reservoir_bits: libc::c_long,
    pub bitrate_limit_reservoir_bias: libc::c_double,
    pub bitrate_average_kbps: libc::c_long,
    pub bitrate_average_damping: libc::c_double
}

pub const OV_ECTL_RATEMANAGE2_GET: libc::c_int = 0x14;
pub const OV_ECTL_RATEMANAGE2_SET: libc::c_int = 0x15;
pub const OV_ECTL_LOWPASS_GET: libc::c_int = 0x20;
pub const OV_ECTL_LOWPASS_SET: libc::c_int = 0x21;
pub const OV_ECTL_IBLOCK_GET: libc::c_int = 0x30;
pub const OV_ECTL_IBLOCK_SET: libc::c_int = 0x31;
pub const OV_ECTL_COUPLING_GET: libc::c_int = 0x40;
pub const OV_ECTL_COUPLING_SET: libc::c_int = 0x41;
pub const OV_ECTL_RATEMANAGE_GET: libc::c_int = 0x10;
pub const OV_ECTL_RATEMANAGE_SET: libc::c_int = 0x11;
pub const OV_ECTL_RATEMANAGE_AVG: libc::c_int = 0x12;
pub const OV_ECTL_RATEMANAGE_HARD: libc::c_int = 0x13;

extern {
    pub fn vorbis_encode_init(vi: *mut vorbis_sys::vorbis_info, channels: libc::c_long,
        rate: libc::c_long, max_bitrate: libc::c_long,
        nominal_bitrate: libc::c_long, min_bitrate: libc::c_long) -> libc::c_int;
    pub fn vorbis_encode_setup_managed(vi: *mut vorbis_sys::vorbis_info, channels: libc::c_long,
        rate: libc::c_long, max_bitrate: libc::c_long,
        nominal_bitrate: libc::c_long, min_bitrate: libc::c_long) -> libc::c_int;
    pub fn vorbis_encode_setup_vbr(vi: *mut vorbis_sys::vorbis_info, channels: libc::c_long,
        rate: libc::c_long, quality: libc::c_float) -> libc::c_int;
    pub fn vorbis_encode_init_vbr(vi: *mut vorbis_sys::vorbis_info, channels: libc::c_long,
        rate: libc::c_long, base_quality: libc::c_float) -> libc::c_int;
    pub fn vorbis_encode_setup_init(vi: *mut vorbis_sys::vorbis_info) -> libc::c_int;
    pub fn vorbis_encode_ctl(vi: *mut vorbis_sys::vorbis_info, number: libc::c_int, arg: *mut libc::c_void) -> libc::c_int;
}
