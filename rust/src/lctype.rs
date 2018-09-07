extern crate libc;
extern "C" {
    #[no_mangle]
    static mut l_memcontrol: Memcontrol_0;
    /*
     ** generic variable for debug tricks
     */
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
    /*
     ** generic extra include file
     */
    /*
     ** RCS ident string
     */
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
}
/* 16-bit ints */
/* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
/*
** $Id: ltests.h,v 2.49 2015/09/22 14:18:24 roberto Exp roberto $
** Internal Header for Debugging of the Lua Implementation
** See Copyright Notice in lua.h
*/
/* test Lua with no compatibility code */
/* turn on assertions */
/* to avoid warnings, and to make sure value is really unused */
/* test for sizes in 'l_sprintf' (make sure whole buffer is available) */
/* memory-allocator control variables */
pub type Memcontrol_0 = Memcontrol;
#[no_mangle]
pub static mut luai_ctype_: [lu_byte; 257] = unsafe {
    [
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0x8i32 as lu_byte,
        0x8i32 as lu_byte,
        0x8i32 as lu_byte,
        0x8i32 as lu_byte,
        0x8i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0xci32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x16i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x5i32 as lu_byte,
        0x4i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x15i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x5i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0x4i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
        0i32 as lu_byte,
    ]
};
