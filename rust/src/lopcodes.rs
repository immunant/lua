#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(libc)]
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
/* argument is a register or a jump offset */
pub const OpArgR: OpArgMask = 2;
/* 16-bit ints */
 /* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = libc::c_uchar;
/* argument is used */
pub const OpArgU: OpArgMask = 1;
/*===========================================================================
  Notes:
  (*) In OP_CALL, if (B == 0) then B = top. If (C == 0), then 'top' is
  set to last_result+1, so next open instruction (OP_CALL, OP_RETURN,
  OP_SETLIST) may use 'top'.

  (*) In OP_VARARG, if (B == 0) then use actual number of varargs and
  set top (like in OP_CALL with C == 0).

  (*) In OP_RETURN, if (B == 0) then return up to 'top'.

  (*) In OP_SETLIST, if (B == 0) then B = 'top'; if (C == 0) then next
  'instruction' is EXTRAARG(real C).

  (*) In OP_LOADKX, the next 'instruction' is always EXTRAARG.

  (*) For comparisons, A specifies what condition the test should accept
  (true or false).

  (*) All 'skips' (pc++) assume that next instruction is a jump.

===========================================================================*/
/*
** masks for instruction properties. The format is:
** bits 0-1: op mode
** bits 2-3: C arg mode
** bits 4-5: B arg mode
** bit 6: instruction set register A
** bit 7: operator is a test (next instruction must be a jump)
*/
pub type OpArgMask = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
/* argument is not used */
pub const OpArgN: OpArgMask = 0;
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
pub const iABx: OpMode = 1;
/*
** $Id: lopcodes.h,v 1.148 2014/10/25 11:50:46 roberto Exp roberto $
** Opcodes for Lua virtual machine
** See Copyright Notice in lua.h
*/
/*===========================================================================
  We assume that instructions are unsigned numbers.
  All instructions have an opcode in the first 6 bits.
  Instructions can have the following fields:
	'A' : 8 bits
	'B' : 9 bits
	'C' : 9 bits
	'Ax' : 26 bits ('A', 'B', and 'C' together)
	'Bx' : 18 bits ('B' and 'C' together)
	'sBx' : signed Bx

  A signed argument is represented in excess K; that is, the number
  value is the unsigned value minus K. K is exactly the maximum value
  for that argument (so that -max is represented by 0, and +max is
  represented by 2*max), which is half the maximum for the corresponding
  unsigned argument.
===========================================================================*/
pub type OpMode = libc::c_uint;
pub const iAsBx: OpMode = 2;
pub const iABC: OpMode = 0;
/* argument is a constant or register/constant */
pub const OpArgK: OpArgMask = 3;
/* basic instruction format */
pub const iAx: OpMode = 3;
#[no_mangle]
pub static mut luaP_opmodes: [lu_byte; 47] = unsafe {
    [
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABx as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgN as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABx as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 0i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 0i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 0i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgR as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 0i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iAsBx as libc::c_int) as lu_byte,
        (1i32 << 7i32
            | 0i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (1i32 << 7i32
            | 0i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (1i32 << 7i32
            | 0i32 << 6i32
            | (OpArgK as libc::c_int) << 4i32
            | (OpArgK as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (1i32 << 7i32
            | 0i32 << 6i32
            | (OpArgN as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (1i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 0i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iAsBx as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iAsBx as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 0i32 << 6i32
            | (OpArgN as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgR as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iAsBx as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 0i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABx as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 1i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgN as libc::c_int) << 2i32
            | iABC as libc::c_int) as lu_byte,
        (0i32 << 7i32
            | 0i32 << 6i32
            | (OpArgU as libc::c_int) << 4i32
            | (OpArgU as libc::c_int) << 2i32
            | iAx as libc::c_int) as lu_byte,
    ]
};
#[no_mangle]
pub static mut luaP_opnames: [*const libc::c_char; 48] = unsafe {
    [
        b"MOVE\x00" as *const u8 as *const libc::c_char,
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
        0 as *const libc::c_char,
    ]
};
