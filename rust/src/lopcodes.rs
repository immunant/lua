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
    static mut l_memcontrol: Memcontrol;
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
}
pub type lu_byte = libc::c_uchar;
pub const OpArgK: OpArgMask = 3;
pub type Memcontrol = Memcontrol_0;
pub const iAsBx: OpMode = 2;
pub const iAx: OpMode = 3;
pub type OpArgMask = libc::c_uint;
pub const OpArgU: OpArgMask = 1;
pub type OpMode = libc::c_uint;
pub const OpArgN: OpArgMask = 0;
pub const OpArgR: OpArgMask = 2;
pub const iABx: OpMode = 1;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Memcontrol_0 {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
pub const iABC: OpMode = 0;
#[no_mangle]
pub static mut luaP_opmodes: [lu_byte; 47] =
    unsafe {
        [(0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABx as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgN as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABx as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 0i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 0i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 0i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgR as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 0i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iAsBx as libc::c_int) as
             lu_byte,
         (1i32 << 7i32 | 0i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (1i32 << 7i32 | 0i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (1i32 << 7i32 | 0i32 << 6i32 | (OpArgK as libc::c_int) << 4i32 |
              (OpArgK as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (1i32 << 7i32 | 0i32 << 6i32 | (OpArgN as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (1i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 0i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iAsBx as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iAsBx as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 0i32 << 6i32 | (OpArgN as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgR as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iAsBx as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 0i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABx as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 1i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgN as libc::c_int) << 2i32 | iABC as libc::c_int) as
             lu_byte,
         (0i32 << 7i32 | 0i32 << 6i32 | (OpArgU as libc::c_int) << 4i32 |
              (OpArgU as libc::c_int) << 2i32 | iAx as libc::c_int) as
             lu_byte]
    };
#[no_mangle]
pub static mut luaP_opnames: [*const libc::c_char; 48] =
    unsafe {
        [b"MOVE\x00" as *const u8 as *const libc::c_char,
         b"LOADK\x00" as *const u8 as *const libc::c_char,
         b"LOADKX\x00" as *const u8 as *const libc::c_char,
         b"LOADBOOL\x00" as *const u8 as *const libc::c_char,
         b"LOADNIL\x00" as *const u8 as *const libc::c_char,
         b"GETUPVAL\x00" as *const u8 as *const libc::c_char,
         b"GETTABUP\x00" as *const u8 as *const libc::c_char,
         b"GETTABLE\x00" as *const u8 as *const libc::c_char,
         b"SETTABUP\x00" as *const u8 as *const libc::c_char,
         b"SETUPVAL\x00" as *const u8 as *const libc::c_char,
         b"SETTABLE\x00" as *const u8 as *const libc::c_char,
         b"NEWTABLE\x00" as *const u8 as *const libc::c_char,
         b"SELF\x00" as *const u8 as *const libc::c_char,
         b"ADD\x00" as *const u8 as *const libc::c_char,
         b"SUB\x00" as *const u8 as *const libc::c_char,
         b"MUL\x00" as *const u8 as *const libc::c_char,
         b"MOD\x00" as *const u8 as *const libc::c_char,
         b"POW\x00" as *const u8 as *const libc::c_char,
         b"DIV\x00" as *const u8 as *const libc::c_char,
         b"IDIV\x00" as *const u8 as *const libc::c_char,
         b"BAND\x00" as *const u8 as *const libc::c_char,
         b"BOR\x00" as *const u8 as *const libc::c_char,
         b"BXOR\x00" as *const u8 as *const libc::c_char,
         b"SHL\x00" as *const u8 as *const libc::c_char,
         b"SHR\x00" as *const u8 as *const libc::c_char,
         b"UNM\x00" as *const u8 as *const libc::c_char,
         b"BNOT\x00" as *const u8 as *const libc::c_char,
         b"NOT\x00" as *const u8 as *const libc::c_char,
         b"LEN\x00" as *const u8 as *const libc::c_char,
         b"CONCAT\x00" as *const u8 as *const libc::c_char,
         b"JMP\x00" as *const u8 as *const libc::c_char,
         b"EQ\x00" as *const u8 as *const libc::c_char,
         b"LT\x00" as *const u8 as *const libc::c_char,
         b"LE\x00" as *const u8 as *const libc::c_char,
         b"TEST\x00" as *const u8 as *const libc::c_char,
         b"TESTSET\x00" as *const u8 as *const libc::c_char,
         b"CALL\x00" as *const u8 as *const libc::c_char,
         b"TAILCALL\x00" as *const u8 as *const libc::c_char,
         b"RETURN\x00" as *const u8 as *const libc::c_char,
         b"FORLOOP\x00" as *const u8 as *const libc::c_char,
         b"FORPREP\x00" as *const u8 as *const libc::c_char,
         b"TFORCALL\x00" as *const u8 as *const libc::c_char,
         b"TFORLOOP\x00" as *const u8 as *const libc::c_char,
         b"SETLIST\x00" as *const u8 as *const libc::c_char,
         b"CLOSURE\x00" as *const u8 as *const libc::c_char,
         b"VARARG\x00" as *const u8 as *const libc::c_char,
         b"EXTRAARG\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char]
    };
