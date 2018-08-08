#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc)]
extern crate libc;
extern "C" {
    #[no_mangle]
    static mut l_memcontrol: Memcontrol_0;
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
pub type Memcontrol_0 = Memcontrol;
pub type lu_byte = libc::c_uchar;
#[no_mangle]
pub static mut luai_ctype_: [lu_byte; 257] =
    unsafe {
        [0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 8i32 as lu_byte, 8i32 as lu_byte,
         8i32 as lu_byte, 8i32 as lu_byte, 8i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 12i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte,
         4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte,
         4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte,
         4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte,
         4i32 as lu_byte, 22i32 as lu_byte, 22i32 as lu_byte,
         22i32 as lu_byte, 22i32 as lu_byte, 22i32 as lu_byte,
         22i32 as lu_byte, 22i32 as lu_byte, 22i32 as lu_byte,
         22i32 as lu_byte, 22i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte,
         4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte,
         4i32 as lu_byte, 21i32 as lu_byte, 21i32 as lu_byte,
         21i32 as lu_byte, 21i32 as lu_byte, 21i32 as lu_byte,
         21i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte,
         4i32 as lu_byte, 5i32 as lu_byte, 4i32 as lu_byte, 21i32 as lu_byte,
         21i32 as lu_byte, 21i32 as lu_byte, 21i32 as lu_byte,
         21i32 as lu_byte, 21i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte, 5i32 as lu_byte,
         5i32 as lu_byte, 5i32 as lu_byte, 4i32 as lu_byte, 4i32 as lu_byte,
         4i32 as lu_byte, 4i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte,
         0i32 as lu_byte, 0i32 as lu_byte, 0i32 as lu_byte]
    };
