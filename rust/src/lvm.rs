#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    /*
    ** $Id: lstate.h,v 2.132 2016/10/19 12:31:42 roberto Exp roberto $
    ** Global State
    ** See Copyright Notice in lua.h
    */
    /*

** Some notes about garbage-collected objects: All objects in Lua must
** be kept somehow accessible until being freed, so all objects always
** belong to one (and only one) of these lists, using field 'next' of
** the 'CommonHeader' for the link:
**
** 'allgc': all objects not marked for finalization;
** 'finobj': all objects marked for finalization;
** 'tobefnz': all objects ready to be finalized;
** 'fixedgc': all objects that are not to be collected (currently
** only small strings, such as reserved words).

*/
    /* defined in ldo.c */
    pub type lua_longjmp;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut signgam: libc::c_int;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaO_fb2int(x: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> size_t;
    #[no_mangle]
    fn luaO_tostring(L: *mut lua_State_0, obj: StkId) -> ();
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaT_gettm(events: *mut Table, event: TMS, ename: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaT_gettmbyobj(L: *mut lua_State_0, o: *const TValue, event: TMS) -> *const TValue;
    #[no_mangle]
    fn luaT_callTM(
        L: *mut lua_State_0,
        f: *const TValue,
        p1: *const TValue,
        p2: *const TValue,
        p3: *mut TValue,
        hasres: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn luaT_trybinTM(
        L: *mut lua_State_0,
        p1: *const TValue,
        p2: *const TValue,
        res: StkId,
        event: TMS,
    ) -> ();
    #[no_mangle]
    fn luaT_callorderTM(
        L: *mut lua_State_0,
        p1: *const TValue,
        p2: *const TValue,
        event: TMS,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaG_typeerror(L: *mut lua_State_0, o: *const TValue, opname: *const libc::c_char) -> !;
    #[no_mangle]
    fn luaG_ordererror(L: *mut lua_State_0, p1: *const TValue, p2: *const TValue) -> !;
    #[no_mangle]
    fn luaG_traceexec(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaD_precall(L: *mut lua_State_0, func: StkId, nresults: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaD_call(L: *mut lua_State_0, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_poscall(
        L: *mut lua_State_0,
        ci: *mut CallInfo_0,
        firstResult: StkId,
        nres: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaD_growstack(L: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn luaF_newLclosure(L: *mut lua_State_0, nelems: libc::c_int) -> *mut LClosure_0;
    #[no_mangle]
    fn luaF_findupval(L: *mut lua_State_0, level: StkId) -> *mut UpVal;
    #[no_mangle]
    fn luaF_close(L: *mut lua_State_0, level: StkId) -> ();
    #[no_mangle]
    fn luaC_step(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaC_barrierback_(L: *mut lua_State_0, o: *mut Table) -> ();
    #[no_mangle]
    fn luaC_upvalbarrier_(L: *mut lua_State_0, uv: *mut UpVal) -> ();
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
    #[no_mangle]
    fn luaS_eqlngstr(a: *mut TString, b: *mut TString) -> libc::c_int;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State_0, str: *const libc::c_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaS_createlngstrobj(L: *mut lua_State_0, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaH_setint(L: *mut lua_State_0, t: *mut Table, key: lua_Integer, value: *mut TValue) -> ();
    #[no_mangle]
    fn luaH_getstr(t: *mut Table, key: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaH_get(t: *mut Table, key: *const TValue) -> *const TValue;
    #[no_mangle]
    fn luaH_newkey(L: *mut lua_State_0, t: *mut Table, key: *const TValue) -> *mut TValue;
    #[no_mangle]
    fn luaH_new(L: *mut lua_State_0) -> *mut Table;
    #[no_mangle]
    fn luaH_resize(
        L: *mut lua_State_0,
        t: *mut Table,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    ) -> ();
    #[no_mangle]
    fn luaH_resizearray(L: *mut lua_State_0, t: *mut Table, nasize: libc::c_uint) -> ();
    #[no_mangle]
    fn luaH_getn(t: *mut Table) -> libc::c_int;
    #[no_mangle]
    fn luaG_runerror(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> !;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
/*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/
pub const OP_TFORLOOP: OpCode = 42;
pub const TM_MOD: TMS = 9;
/*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/
pub const OP_RETURN: OpCode = 38;
/*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_CALL: OpCode = 36;
pub const TM_BNOT: TMS = 19;
pub const TM_CONCAT: TMS = 22;
pub const TM_LEN: TMS = 4;
pub const TM_UNM: TMS = 18;
/*	A sBx	R(A)-=R(A+2); pc+=sBx				*/
pub const OP_FORPREP: OpCode = 40;
pub const TM_IDIV: TMS = 12;
pub const TM_SHR: TMS = 17;
/*	A B C	R(A) := RK(B) - RK(C)				*/
pub const OP_SUB: OpCode = 14;
pub type Table = Table_0;
/*	A B	R(A) := ~R(B)					*/
pub const OP_BNOT: OpCode = 26;
/*	A C	if not (R(A) <=> C) then pc++			*/
pub const OP_TEST: OpCode = 34;
/*	A sBx	R(A)+=R(A+2);
			if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
pub const OP_FORLOOP: OpCode = 39;
/*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/
pub const OP_SETLIST: OpCode = 43;
pub const TM_MODE: TMS = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
pub const TM_ADD: TMS = 6;
/*
** Union of all collectable objects (only for conversions)
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata,
    cl: Closure,
    h: Table_0,
    p: Proto,
    th: lua_State,
}
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	R(A) := RK(B) / RK(C)				*/
pub const OP_DIV: OpCode = 18;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    c: CClosure_0,
    l: LClosure_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub numparams: lu_byte,
    pub is_vararg: lu_byte,
    pub maxstacksize: lu_byte,
    pub sizeupvalues: libc::c_int,
    pub sizek: libc::c_int,
    pub sizecode: libc::c_int,
    pub sizelineinfo: libc::c_int,
    pub sizep: libc::c_int,
    pub sizelocvars: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub k: *mut TValue,
    pub code: *mut Instruction,
    pub p: *mut *mut Proto,
    pub lineinfo: *mut libc::c_int,
    pub locvars: *mut LocVar_0,
    pub upvalues: *mut Upvaldesc_0,
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
/*	A B C	UpValue[A][RK(B)] := RK(C)			*/
pub const OP_SETTABUP: OpCode = 8;
/*	Ax	extra (larger) argument for previous opcode	*/
pub const OP_EXTRAARG: OpCode = 46;
pub type OpCode = libc::c_uint;
/*	A B C	R(A) := RK(B) << RK(C)				*/
pub const OP_SHL: OpCode = 23;
/*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
pub const OP_TAILCALL: OpCode = 37;
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
pub type Upvaldesc_0 = Upvaldesc;
pub const TM_NEWINDEX: TMS = 1;
/* last-created closure with this prototype */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 0],
}
/*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
pub const OP_LOADNIL: OpCode = 4;
/*	A B	UpValue[B] := R(A)				*/
pub const OP_SETUPVAL: OpCode = 9;
pub const TM_LT: TMS = 20;
pub const TM_LE: TMS = 21;
/*	A B C	R(A) := RK(B) ^ RK(C)				*/
pub const OP_POW: OpCode = 17;
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
pub const TM_GC: TMS = 2;
pub type TMS = libc::c_uint;
/*	A B	R(A) := length of R(B)				*/
pub const OP_LEN: OpCode = 28;
/*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/
pub const OP_VARARG: OpCode = 45;
pub const TM_POW: TMS = 10;
/*	A B C	R(A) := {} (size = B,C)				*/
pub const OP_NEWTABLE: OpCode = 11;
/*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/
pub const OP_TESTSET: OpCode = 35;
pub const TM_BXOR: TMS = 15;
/*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/
pub const OP_SELF: OpCode = 12;
/*	A B C	R(A) := RK(B) // RK(C)				*/
pub const OP_IDIV: OpCode = 19;
/*	A B C	R(A)[RK(B)] := RK(C)				*/
pub const OP_SETTABLE: OpCode = 10;
/*	A B	R(A) := not R(B)				*/
pub const OP_NOT: OpCode = 27;
/*	A B C	R(A) := RK(B) + RK(C)				*/
pub const OP_ADD: OpCode = 13;
/* number of elements in the enum */
pub const TM_N: TMS = 24;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
/*	A B	R(A) := UpValue[B]				*/
pub const OP_GETUPVAL: OpCode = 5;
/*----------------------------------------------------------------------
name		args	description
------------------------------------------------------------------------*/
/*	A B	R(A) := R(B)					*/
pub const OP_MOVE: OpCode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 0],
}
/*
** Ensures that address after this type is always fully aligned.
*/
pub type UTString = UTString_0;
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
pub type LocVar_0 = LocVar;
/* argument is a constant or register/constant */
pub const OpArgK: OpArgMask = 3;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
/*	A Bx	R(A) := closure(KPROTO[Bx])			*/
pub const OP_CLOSURE: OpCode = 44;
/* argument is a register or a jump offset */
pub const OpArgR: OpArgMask = 2;
pub const TM_CALL: TMS = 23;
pub const TM_SHL: TMS = 16;
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/* argument is used */
pub const OpArgU: OpArgMask = 1;
pub const TM_DIV: TMS = 11;
/*	A B C	R(A) := RK(B) ~ RK(C)				*/
pub const OP_BXOR: OpCode = 22;
/*	A 	R(A) := Kst(extra arg)				*/
pub const OP_LOADKX: OpCode = 2;
pub type LClosure_0 = LClosure;
/*
** Function Prototypes
*/
pub type Proto_0 = Proto;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString_0 {
    dummy: L_Umaxalign,
    tsv: TString,
}
/*
** Closures
*/
pub type CClosure_0 = CClosure;
/*	A B C	R(A) := RK(B) >> RK(C)				*/
pub const OP_SHR: OpCode = 24;
/* last tag method with fast access */
pub const TM_EQ: TMS = 5;
pub const TM_MUL: TMS = 8;
/*	A B C	R(A) := RK(B) & RK(C)				*/
pub const OP_BAND: OpCode = 20;
/*	A Bx	R(A) := Kst(Bx)					*/
pub const OP_LOADK: OpCode = 1;
/*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
pub const OP_TFORCALL: OpCode = 41;
/*	A B C	R(A) := RK(B) % RK(C)				*/
pub const OP_MOD: OpCode = 16;
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
/*	A B C	R(A) := UpValue[B][RK(C)]			*/
pub const OP_GETTABUP: OpCode = 6;
pub const TM_INDEX: TMS = 0;
pub const TM_SUB: TMS = 7;
pub const TM_BOR: TMS = 14;
/*	A B C	R(A) := R(B).. ... ..R(C)			*/
pub const OP_CONCAT: OpCode = 29;
/* argument is not used */
pub const OpArgN: OpArgMask = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table_0,
    pub len: size_t,
    pub user_: Value_0,
}
/*	A B C	R(A) := RK(B) | RK(C)				*/
pub const OP_BOR: OpCode = 21;
/*	A B C	R(A) := RK(B) * RK(C)				*/
pub const OP_MUL: OpCode = 15;
pub type ptrdiff_t = libc::c_long;
pub type intptr_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nci: libc::c_ushort,
    pub status: lu_byte,
    pub top: StkId,
    pub l_G: *mut global_State,
    pub ci: *mut CallInfo_0,
    pub oldpc: *const Instruction,
    pub stack_last: StkId,
    pub stack: StkId,
    pub openupval: *mut UpVal,
    pub gclist: *mut GCObject,
    pub twups: *mut lua_State,
    pub errorJmp: *mut lua_longjmp,
    pub base_ci: CallInfo_0,
    pub hook: lua_Hook,
    pub errfunc: ptrdiff_t,
    pub stacksize: libc::c_int,
    pub basehookcount: libc::c_int,
    pub hookcount: libc::c_int,
    pub nny: libc::c_ushort,
    pub nCcalls: libc::c_ushort,
    pub hookmask: sig_atomic_t,
    pub allowhook: lu_byte,
}
/* 16-bit ints */
 /* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = libc::c_uchar;
pub type sig_atomic_t = __sig_atomic_t;
/* Functions to be called by the debugger in specific events */
pub type lua_Hook = Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut lua_Debug) -> ()>;
/*
** {==============================================================
** some useful macros
** ===============================================================
*/
/* }============================================================== */
/*
** {==============================================================
** compatibility macros for unsigned conversions
** ===============================================================
*/
/* }============================================================== */
/*
** {======================================================================
** Debug API
** =======================================================================
*/
/*
** Event codes
*/
/*
** Event masks
*/
pub type lua_Debug = lua_Debug_0;
/* activation record */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug_0 {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub nups: libc::c_uchar,
    pub nparams: libc::c_uchar,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub short_src: [libc::c_char; 60],
    pub i_ci: *mut CallInfo,
}
/* active function */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: StkId,
    pub top: StkId,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: unnamed,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed {
    l: unnamed_1,
    c: unnamed_0,
}
/* only for C functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_0 {
    pub k: lua_KFunction,
    pub old_errfunc: ptrdiff_t,
    pub ctx: lua_KContext,
}
/* type for continuation-function contexts */
pub type lua_KContext = intptr_t;
/*
** Type for continuation functions
*/
pub type lua_KFunction = Option<
    unsafe extern "C" fn(_: *mut lua_State_0, _: libc::c_int, _: lua_KContext) -> libc::c_int,
>;
/*
** $Id: lua.h,v 1.331 2016/05/30 15:53:28 roberto Exp roberto $
** Lua - A Scripting Language
** Lua.org, PUC-Rio, Brazil (http://www.lua.org)
** See Copyright Notice at the end of this file
*/
/* mark for precompiled code ('<esc>Lua') */
/* option for multiple returns in 'lua_pcall' and 'lua_call' */
/*
** Pseudo-indices
** (-LUAI_MAXSTACK is the minimum valid index; we keep some free empty
** space after that to help overflow detection)
*/
/* thread status */
pub type lua_State_0 = lua_State;
/* only for Lua functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_1 {
    pub base: StkId,
    pub savedpc: *const Instruction,
}
/* internal assertions for in-house debugging */
/* to avoid problems with conditions too long */
/*
** assertion for checking API calls
*/
/* macro to avoid warnings about unused variables */
/* type casts (a macro highlights casts in the code) */
/* cast a signed lua_Integer to lua_Unsigned */
/*
** cast a lua_Unsigned to a signed lua_Integer; this cast is
** not strict ISO C, but two-complement architectures should
** work fine.
*/
/*
** non-return type
*/
/*
** maximum depth for nested C calls and syntactical nested non-terminals
** in a program. (Value must fit in an unsigned short int.)
*/
/*
** type for virtual-machine instructions;
** must be an unsigned with (at least) 4 bytes (see details in lopcodes.h)
*/
pub type Instruction = libc::c_uint;
/* macro defining a nil value */
/* raw type tag of a TValue */
/* tag with no variants (bits 0-3) */
/* type tag of a TValue (bits 0-3 for tags + variant bits 4-5) */
/* type tag of a TValue with no variants (bits 0-3) */
/* Macros to test type */
/* Macros to access values */
/* a dead value may get the 'gc' field, but cannot access its contents */
/* Macros for internal tests */
/* Macros to set values */
/*
** different types of assignments, according to destination
*/
/* from stack to (same) stack */
/* to stack (not from same stack) */
/* from table to same table */
/* to new object */
/* to table (define it as an expression to be used in macros) */
/*
** {======================================================
** types and prototypes
** =======================================================
*/
/* index to stack elements */
pub type StkId = *mut TValue;
pub type TValue = lua_TValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_TValue {
    pub value_: Value,
    pub tt_: libc::c_int,
}
/*
** Common Header for all collectable objects (in macro form, to be
** included in other objects)
*/
/*
** Common type has only the common header
*/
/*
** Tagged Values. This is the basic representation of values in Lua,
** an actual value plus a tag with its type.
*/
/*
** Union of all Lua values
*/
pub type Value = Value_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value_0 {
    gc: *mut GCObject,
    p: *mut libc::c_void,
    b: libc::c_int,
    f: lua_CFunction,
    i: lua_Integer,
    n: lua_Number,
}
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State_0) -> libc::c_int>;
/*
** $Id: lobject.h,v 2.116 2015/11/03 18:33:10 roberto Exp roberto $
** Type definitions for Lua objects
** See Copyright Notice in lua.h
*/
/*
** Extra tags for non-values
*/
/* function prototypes */
/* removed keys in tables */
/*
** number of all possible tags (including LUA_TNONE but excluding DEADKEY)
*/
/*
** tags for Tagged Values have the following use of bits:
** bits 0-3: actual tag (a LUA_T* value)
** bits 4-5: variant bits
** bit 6: whether value is collectable
*/
/*
** LUA_TFUNCTION variants:
** 0 - Lua function
** 1 - light C function
** 2 - regular C function (closure)
*/
/* Variant tags for functions */
/* Lua closure */
/* light C function */
/* C closure */
/* Variant tags for strings */
/* short strings */
/* long strings */
/* Variant tags for numbers */
/* float numbers */
/* integer numbers */
/* Bit mark for collectable types */
/* mark a tag as collectable */
/*
** Common type for all collectable objects
*/
pub type GCObject = GCObject_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCObject_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
}
/*
** Information about a call.
** When a thread yields, 'func' is adjusted to pretend that the
** top function has only the yielded values in its stack; in that
** case, the actual 'func' value is saved in field 'extra'.
** When a function calls another with a continuation, 'extra' keeps
** the function index so that, in case of errors, the continuation
** function can be called with the correct top.
*/
pub type CallInfo_0 = CallInfo;
/*
** Lua Upvalues
*/
pub type UpVal = UpVal_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal_0 {
    pub v: *mut TValue,
    pub refcount: lu_mem,
    pub u: unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    open: unnamed_3,
    value: TValue,
}
/* (when open) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
    pub next: *mut UpVal,
    pub touched: libc::c_int,
}
/*
** $Id: llimits.h,v 1.140 2015/10/21 18:40:47 roberto Exp roberto $
** Limits, basic types, and some other 'installation-dependent' definitions
** See Copyright Notice in lua.h
*/
/*
** 'lu_mem' and 'l_mem' are unsigned/signed integers big enough to count
** the total memory used by Lua (in bytes). Usually, 'size_t' and
** 'ptrdiff_t' should work, but we use 'long' for 16-bit machines.
*/
/* { external definitions? */
/* }{ */
pub type lu_mem = size_t;
/*
** Bits in CallInfo status
*/
/* original value of 'allowhook' */
/* call is running a Lua function */
/* call is running a debug hook */
/* call is running on a fresh invocation
                                   of luaV_execute */
/* call is a yieldable protected call */
/* call was tail called */
/* last hook called yielded */
/* using __lt for __le */
/* call is running a finalizer */
/* assume that CIST_OAH has offset 0 and that 'v' is strictly 0/1 */
/*
** 'global state', shared by all threads of this state
*/
pub type global_State = global_State_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_State_0 {
    pub frealloc: lua_Alloc,
    pub ud: *mut libc::c_void,
    pub totalbytes: l_mem,
    pub GCdebt: l_mem,
    pub GCmemtrav: lu_mem,
    pub GCestimate: lu_mem,
    pub strt: stringtable,
    pub l_registry: TValue,
    pub seed: libc::c_uint,
    pub currentwhite: lu_byte,
    pub gcstate: lu_byte,
    pub gckind: lu_byte,
    pub gcrunning: lu_byte,
    pub allgc: *mut GCObject,
    pub sweepgc: *mut *mut GCObject,
    pub finobj: *mut GCObject,
    pub gray: *mut GCObject,
    pub grayagain: *mut GCObject,
    pub weak: *mut GCObject,
    pub ephemeron: *mut GCObject,
    pub allweak: *mut GCObject,
    pub tobefnz: *mut GCObject,
    pub fixedgc: *mut GCObject,
    pub twups: *mut lua_State,
    pub gcfinnum: libc::c_uint,
    pub gcpause: libc::c_int,
    pub gcstepmul: libc::c_int,
    pub panic: lua_CFunction,
    pub mainthread: *mut lua_State,
    pub version: *const lua_Number,
    pub memerrmsg: *mut TString,
    pub tmname: [*mut TString; 24],
    pub mt: [*mut Table_0; 9],
    pub strcache: [[*mut TString; 5]; 23],
}
/*
** Header for string value; string bytes follow the end of this structure
** (aligned according to 'UTString'; see next).
*/
pub type TString = TString_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub extra: lu_byte,
    pub shrlen: lu_byte,
    pub hash: libc::c_uint,
    pub u: unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_4 {
    lnglen: size_t,
    hnext: *mut TString_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,
    pub lsizenode: lu_byte,
    pub sizearray: libc::c_uint,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table_0,
    pub gclist: *mut GCObject,
}
/* copy a value into a key without messing up field 'next' */
pub type Node = Node_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node_0 {
    pub i_val: TValue,
    pub i_key: TKey,
}
/*
** Tables
*/
pub type TKey = TKey_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union TKey_0 {
    nk: unnamed_5,
    tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_5 {
    pub value_: Value,
    pub tt_: libc::c_int,
    pub next: libc::c_int,
}
/*
** Atomic type (relative to signals) to better ensure that 'lua_sethook'
** is thread safe
*/
/* extra stack space to handle TM calls and some other extras */
/* kinds of Garbage Collection */
/* gc was forced by an allocation failure */
pub type stringtable = stringtable_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable_0 {
    pub hash: *mut *mut TString,
    pub nuse: libc::c_int,
    pub size: libc::c_int,
}
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void, _: size_t, _: size_t)
        -> *mut libc::c_void,
>;
/* unsigned integer type */
pub type lua_Unsigned = libc::c_ulonglong;
/*	A B C	R(A) := (Bool)B; if (C) pc++			*/
pub const OP_LOADBOOL: OpCode = 3;
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
/* test for lock/unlock */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct L_EXTRA {
    pub lock: libc::c_int,
    pub plock: *mut libc::c_int,
}
pub const TM_BAND: TMS = 13;
/*	A B	R(A) := -R(B)					*/
pub const OP_UNM: OpCode = 25;
/*	A B C	R(A) := R(B)[RK(C)]				*/
pub const OP_GETTABLE: OpCode = 7;
#[no_mangle]
pub unsafe extern "C" fn luaV_equalobj(
    mut L: *mut lua_State_0,
    mut t1: *const TValue,
    mut t2: *const TValue,
) -> libc::c_int {
    let mut tm: *const TValue = 0 as *const TValue;
    if (*t1).tt_ & 0x3fi32 != (*t2).tt_ & 0x3fi32 {
        /* not the same variant? */
        if (*t1).tt_ & 0xfi32 != (*t2).tt_ & 0xfi32 || (*t1).tt_ & 0xfi32 != 3i32 {
            /* only numbers can be equal with different variants */
            return 0i32;
        } else {
            /* two numbers with different variants */
            /* compare them as integers */
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            return (0 != if (*t1).tt_ == 3i32 | 1i32 << 4i32 {
                if (*t1).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((t1))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        415i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                i1 = (*t1).value_.i;
                1i32
            } else {
                luaV_tointeger(t1, &mut i1, 0i32)
            } && 0 != if (*t2).tt_ == 3i32 | 1i32 << 4i32 {
                if (*t2).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((t2))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        415i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                i2 = (*t2).value_.i;
                1i32
            } else {
                luaV_tointeger(t2, &mut i2, 0i32)
            } && i1 == i2) as libc::c_int;
        }
    } else {
        /* values have same type and same variant */
        match (*t1).tt_ & 0x3fi32 {
            0 => return 1i32,
            19 => {
                if (*t1).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((t1))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        421i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((t2))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        421i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                return ((*t1).value_.i == (*t2).value_.i) as libc::c_int;
            }
            3 => {
                if (*t1).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((t1))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        422i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((t2))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        422i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                return ((*t1).value_.n == (*t2).value_.n) as libc::c_int;
            }
            1 => {
                /* true must be 1 !! */
                if (*t1).tt_ == 1i32 {
                } else {
                    __assert_fail(
                        b"((((t1))->tt_) == (1))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        423i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ == 1i32 {
                } else {
                    __assert_fail(
                        b"((((t2))->tt_) == (1))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        423i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                return ((*t1).value_.b == (*t2).value_.b) as libc::c_int;
            }
            2 => {
                if (*t1).tt_ == 2i32 {
                } else {
                    __assert_fail(
                        b"((((t1))->tt_) == (2))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        424i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ == 2i32 {
                } else {
                    __assert_fail(
                        b"((((t2))->tt_) == (2))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        424i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                return ((*t1).value_.p == (*t2).value_.p) as libc::c_int;
            }
            22 => {
                if (*t1).tt_ == 6i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((t1))->tt_) == ((6 | (1 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        425i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ == 6i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((t2))->tt_) == ((6 | (1 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        425i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                return ((*t1).value_.f == (*t2).value_.f) as libc::c_int;
            }
            4 => {
                if (*t1).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((t1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        426i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t1).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((t1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        426i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(&mut (*((*t1).value_.gc as *mut GCUnion)).ts as *mut TString_0)).tt
                    as libc::c_int == 4i32 | 0i32 << 4i32
                {
                } else {
                    __assert_fail(b"(((((((((((t1))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((t1))->tt_)) & 0x0F)) == (4))\", \"lvm.c\", 426, __extension__ __PRETTY_FUNCTION__)), (((((((((t1)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((t1)->value_).gc)->tt) & 0x0F) == 4\", \"lvm.c\", 426, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((t1)->value_).gc))))->ts))))))->tt == (4 | (0 << 4))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lvm.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  426i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 63],
                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                };
                if (*t1).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((t1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        426i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t1).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((t1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        426i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((t2))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        426i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t2).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((t2)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        426i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                return (&mut (*((*t1).value_.gc as *mut GCUnion)).ts as *mut TString_0
                    == &mut (*((*t2).value_.gc as *mut GCUnion)).ts as *mut TString_0)
                    as libc::c_int;
            }
            20 => {
                if (*t1).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((t1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        427i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t1).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((t1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        427i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((t2))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        427i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t2).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((t2)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        427i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                return luaS_eqlngstr(
                    &mut (*((*t1).value_.gc as *mut GCUnion)).ts,
                    &mut (*((*t2).value_.gc as *mut GCUnion)).ts,
                );
            }
            7 => {
                if (*t1).tt_ == 7i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t1))->tt_) == (((7) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        429i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t1).value_.gc).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(
                        b"(((t1)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        429i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ == 7i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t2))->tt_) == (((7) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        429i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t2).value_.gc).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(
                        b"(((t2)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        429i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if &mut (*((*t1).value_.gc as *mut GCUnion)).u as *mut Udata
                    == &mut (*((*t2).value_.gc as *mut GCUnion)).u as *mut Udata
                {
                    return 1i32;
                } else if L.is_null() {
                    return 0i32;
                } else {
                    if (*t1).tt_ == 7i32 | 1i32 << 6i32 {
                    } else {
                        __assert_fail(b"((((t1))->tt_) == (((7) | (1 << 6))))\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"lvm.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      431i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 63],
                                                                &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                    };
                    if (*(*t1).value_.gc).tt as libc::c_int == 7i32 {
                    } else {
                        __assert_fail(b"(((t1)->value_).gc)->tt == 7\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"lvm.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      431i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 63],
                                                                &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                    };
                    tm = if (*(&mut (*((*t1).value_.gc as *mut GCUnion)).u as *mut Udata))
                        .metatable
                        .is_null()
                    {
                        0 as *const TValue
                    } else {
                        if (*t1).tt_ == 7i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"((((t1))->tt_) == (((7) | (1 << 6))))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lvm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              431i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 63],
                                                                        &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                        };
                        if (*(*t1).value_.gc).tt as libc::c_int == 7i32 {
                        } else {
                            __assert_fail(b"(((t1)->value_).gc)->tt == 7\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lvm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              431i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 63],
                                                                        &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                        };
                        if 0 != (*(*(&mut (*((*t1).value_.gc as *mut GCUnion)).u as *mut Udata))
                            .metatable)
                            .flags as libc::c_uint
                            & 1u32 << TM_EQ as libc::c_int
                        {
                            0 as *const TValue
                        } else {
                            if (*t1).tt_ == 7i32 | 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"((((t1))->tt_) == (((7) | (1 << 6))))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lvm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  431i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 63],
                                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                            };
                            if (*(*t1).value_.gc).tt as libc::c_int == 7i32 {
                            } else {
                                __assert_fail(b"(((t1)->value_).gc)->tt == 7\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lvm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  431i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 63],
                                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                            };
                            luaT_gettm(
                                (*&mut (*((*t1).value_.gc as *mut GCUnion)).u).metatable,
                                TM_EQ,
                                (*(*L).l_G).tmname[TM_EQ as libc::c_int as usize],
                            )
                        }
                    };
                    if tm.is_null() {
                        if (*t2).tt_ == 7i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"((((t2))->tt_) == (((7) | (1 << 6))))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          433i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 63],
                                                                    &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                        };
                        if (*(*t2).value_.gc).tt as libc::c_int == 7i32 {
                        } else {
                            __assert_fail(b"(((t2)->value_).gc)->tt == 7\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          433i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 63],
                                                                    &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                        };
                        tm = if (*(&mut (*((*t2).value_.gc as *mut GCUnion)).u as *mut Udata))
                            .metatable
                            .is_null()
                        {
                            0 as *const TValue
                        } else {
                            if (*t2).tt_ == 7i32 | 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"((((t2))->tt_) == (((7) | (1 << 6))))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lvm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  433i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 63],
                                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                            };
                            if (*(*t2).value_.gc).tt as libc::c_int == 7i32 {
                            } else {
                                __assert_fail(b"(((t2)->value_).gc)->tt == 7\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lvm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  433i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 63],
                                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                            };
                            if 0 != (*(*(&mut (*((*t2).value_.gc as *mut GCUnion)).u
                                as *mut Udata))
                                .metatable)
                                .flags as libc::c_uint
                                & 1u32 << TM_EQ as libc::c_int
                            {
                                0 as *const TValue
                            } else {
                                if (*t2).tt_ == 7i32 | 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"((((t2))->tt_) == (((7) | (1 << 6))))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"lvm.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      433i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 63],
                                                                                &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                                };
                                if (*(*t2).value_.gc).tt as libc::c_int == 7i32 {
                                } else {
                                    __assert_fail(b"(((t2)->value_).gc)->tt == 7\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"lvm.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      433i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 63],
                                                                                &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                                };
                                luaT_gettm(
                                    (*&mut (*((*t2).value_.gc as *mut GCUnion)).u).metatable,
                                    TM_EQ,
                                    (*(*L).l_G).tmname[TM_EQ as libc::c_int as usize],
                                )
                            }
                        }
                    }
                }
            }
            5 => {
                /* will try TM */
                if (*t1).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t1))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        437i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t1).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(((t1)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        437i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*t2).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t2))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        437i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t2).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(((t2)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        437i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if &mut (*((*t1).value_.gc as *mut GCUnion)).h as *mut Table_0
                    == &mut (*((*t2).value_.gc as *mut GCUnion)).h as *mut Table_0
                {
                    return 1i32;
                } else if L.is_null() {
                    return 0i32;
                } else {
                    if (*t1).tt_ == 5i32 | 1i32 << 6i32 {
                    } else {
                        __assert_fail(b"((((t1))->tt_) == (((5) | (1 << 6))))\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"lvm.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      439i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 63],
                                                                &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                    };
                    if (*(*t1).value_.gc).tt as libc::c_int == 5i32 {
                    } else {
                        __assert_fail(b"(((t1)->value_).gc)->tt == 5\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"lvm.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      439i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 63],
                                                                &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                    };
                    tm = if (*(&mut (*((*t1).value_.gc as *mut GCUnion)).h as *mut Table_0))
                        .metatable
                        .is_null()
                    {
                        0 as *const TValue
                    } else {
                        if (*t1).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"((((t1))->tt_) == (((5) | (1 << 6))))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lvm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              439i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 63],
                                                                        &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                        };
                        if (*(*t1).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(b"(((t1)->value_).gc)->tt == 5\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lvm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              439i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 63],
                                                                        &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                        };
                        if 0 != (*(*(&mut (*((*t1).value_.gc as *mut GCUnion)).h as *mut Table_0))
                            .metatable)
                            .flags as libc::c_uint
                            & 1u32 << TM_EQ as libc::c_int
                        {
                            0 as *const TValue
                        } else {
                            if (*t1).tt_ == 5i32 | 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"((((t1))->tt_) == (((5) | (1 << 6))))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lvm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  439i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 63],
                                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                            };
                            if (*(*t1).value_.gc).tt as libc::c_int == 5i32 {
                            } else {
                                __assert_fail(b"(((t1)->value_).gc)->tt == 5\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lvm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  439i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 63],
                                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                            };
                            luaT_gettm(
                                (*&mut (*((*t1).value_.gc as *mut GCUnion)).h).metatable,
                                TM_EQ,
                                (*(*L).l_G).tmname[TM_EQ as libc::c_int as usize],
                            )
                        }
                    };
                    if tm.is_null() {
                        if (*t2).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"((((t2))->tt_) == (((5) | (1 << 6))))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          441i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 63],
                                                                    &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                        };
                        if (*(*t2).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(b"(((t2)->value_).gc)->tt == 5\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          441i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 63],
                                                                    &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                        };
                        tm = if (*(&mut (*((*t2).value_.gc as *mut GCUnion)).h as *mut Table_0))
                            .metatable
                            .is_null()
                        {
                            0 as *const TValue
                        } else {
                            if (*t2).tt_ == 5i32 | 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"((((t2))->tt_) == (((5) | (1 << 6))))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lvm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  441i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 63],
                                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                            };
                            if (*(*t2).value_.gc).tt as libc::c_int == 5i32 {
                            } else {
                                __assert_fail(b"(((t2)->value_).gc)->tt == 5\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lvm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  441i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 63],
                                                                            &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                            };
                            if 0 != (*(*(&mut (*((*t2).value_.gc as *mut GCUnion)).h
                                as *mut Table_0))
                                .metatable)
                                .flags as libc::c_uint
                                & 1u32 << TM_EQ as libc::c_int
                            {
                                0 as *const TValue
                            } else {
                                if (*t2).tt_ == 5i32 | 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"((((t2))->tt_) == (((5) | (1 << 6))))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"lvm.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      441i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 63],
                                                                                &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                                };
                                if (*(*t2).value_.gc).tt as libc::c_int == 5i32 {
                                } else {
                                    __assert_fail(b"(((t2)->value_).gc)->tt == 5\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"lvm.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      441i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 63],
                                                                                &[libc::c_char; 63]>(b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
                                };
                                luaT_gettm(
                                    (*&mut (*((*t2).value_.gc as *mut GCUnion)).h).metatable,
                                    TM_EQ,
                                    (*(*L).l_G).tmname[TM_EQ as libc::c_int as usize],
                                )
                            }
                        }
                    }
                }
            }
            _ => {
                /* will try TM */
                if 0 != (*t1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((t1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        445i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                if 0 != (*t2).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((t2)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        445i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                return ((*t1).value_.gc == (*t2).value_.gc) as libc::c_int;
            }
        }
        /* no TM? */
        if tm.is_null() {
            /* objects are different */
            return 0i32;
        } else {
            /* call TM */
            luaT_callTM(L, tm, t1, t2, (*L).top, 1i32);
            return !((*(*L).top).tt_ == 0i32 || (*(*L).top).tt_ == 1i32 && {
                if (*(*L).top).tt_ == 1i32 {
                } else {
                    __assert_fail(
                        b"((((L->top))->tt_) == (1))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        450i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"int luaV_equalobj(lua_State *, const TValue *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                (*(*L).top).value_.b == 0i32
            }) as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tointeger(
    mut obj: *const TValue,
    mut p: *mut lua_Integer,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut f: lua_Number = 0.;
    let mut n: lua_Number = 0.;
    let mut current_block: u64;
    let mut v: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    loop {
        if (*obj).tt_ == 3i32 | 0i32 << 4i32 {
            if (*obj).tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((obj))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    99i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                    )).as_ptr(),
                );
            };
            n = (*obj).value_.n;
            f = floor(n);
            if n != f {
                current_block = 8258075665625361029;
                break;
            } else {
                current_block = 16559507199688588974;
                break;
            }
        } else if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
            if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((obj))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    109i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                    )).as_ptr(),
                );
            };
            *p = (*obj).value_.i;
            return 1i32;
        } else if (*obj).tt_ & 0xfi32 == 4i32 && {
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(b"sizeof((((((((((((obj))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((obj))->tt_)) & 0x0F)) == (4))\", \"lvm.c\", 113, __extension__ __PRETTY_FUNCTION__)), (((((((((obj)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((obj)->value_).gc)->tt) & 0x0F) == 4\", \"lvm.c\", 113, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((obj)->value_).gc))))->ts))))))->extra)\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            b"lvm.c\x00" as *const u8 as
                                                *const libc::c_char,
                                            113i32 as libc::c_uint,
                                            (*::std::mem::transmute::<&[u8; 55],
                                                                      &[libc::c_char; 55]>(b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00")).as_ptr());
            };
            if (*obj).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((obj))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    113i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((obj)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    113i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*obj).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((obj))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    113i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((obj)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    113i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                    )).as_ptr(),
                );
            };
            luaO_str2num(
                (&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString_0
                    as *mut libc::c_char)
                    .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize),
                &mut v,
            )
                == if (*(&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString_0)).tt
                    as libc::c_int == 4i32 | 0i32 << 4i32
                {
                    if (*obj).tt_ & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((((obj))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            113i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                                b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((obj)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            113i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                                b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*(&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString_0)).shrlen
                        as libc::c_ulong
                } else {
                    if (*obj).tt_ & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((((obj))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            113i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                                b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((obj)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            113i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                                b"int luaV_tointeger(const TValue *, lua_Integer *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*(&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString_0))
                        .u
                        .lnglen
                }.wrapping_add(1i32 as libc::c_ulong)
        } {
            obj = &mut v
        } else {
            return 0i32;
        }
    }
    match current_block {
        8258075665625361029 => {
            /* not an integral value? */
            if mode == 0i32 {
                /* fails if mode demands integral value */
                return 0i32;
            } else if mode > 1i32 {
                /* convert floor to ceil (remember: n != f) */
                f += 1i32 as libc::c_double
            }
        }
        _ => {}
    }
    return (f >= (-9223372036854775807i64 - 1i64) as libc::c_double
        && f < -((-9223372036854775807i64 - 1i64) as libc::c_double) && {
        *p = f as libc::c_longlong;
        0 != 1i32
    }) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessthan(
    mut L: *mut lua_State_0,
    mut l: *const TValue,
    mut r: *const TValue,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    /* both operands are numbers? */
    if (*l).tt_ & 0xfi32 == 3i32 && (*r).tt_ & 0xfi32 == 3i32 {
        return LTnum(l, r);
    } else if (*l).tt_ & 0xfi32 == 4i32 && (*r).tt_ & 0xfi32 == 4i32 {
        if (*l).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((l))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                370i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"int luaV_lessthan(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*l).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((l)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                370i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"int luaV_lessthan(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*r).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((r))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                370i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"int luaV_lessthan(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*r).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((r)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                370i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"int luaV_lessthan(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        return (l_strcmp(
            &mut (*((*l).value_.gc as *mut GCUnion)).ts,
            &mut (*((*r).value_.gc as *mut GCUnion)).ts,
        ) < 0i32) as libc::c_int;
    } else {
        /* no metamethod? */
        res = luaT_callorderTM(L, l, r, TM_LT);
        if res < 0i32 {
            /* error */
            luaG_ordererror(L, l, r);
        } else {
            return res;
        }
    };
}
/*
** Compare two strings 'ls' x 'rs', returning an integer smaller-equal-
** -larger than zero if 'ls' is smaller-equal-larger than 'rs'.
** The code is a little tricky because it allows '\0' in the strings
** and it uses 'strcoll' (to respect locales) for each segments
** of the strings.
*/
unsafe extern "C" fn l_strcmp(mut ls: *const TString, mut rs: *const TString) -> libc::c_int {
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof((ls)->extra)\x00" as *const u8 as *const libc::c_char,
            b"lvm.c\x00" as *const u8 as *const libc::c_char,
            250i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"int l_strcmp(const TString *, const TString *)\x00",
            )).as_ptr(),
        );
    };
    let mut l: *const libc::c_char = (ls as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
    let mut ll: size_t = if (*ls).tt as libc::c_int == 4i32 | 0i32 << 4i32 {
        (*ls).shrlen as libc::c_ulong
    } else {
        (*ls).u.lnglen
    };
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof((rs)->extra)\x00" as *const u8 as *const libc::c_char,
            b"lvm.c\x00" as *const u8 as *const libc::c_char,
            252i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"int l_strcmp(const TString *, const TString *)\x00",
            )).as_ptr(),
        );
    };
    let mut r: *const libc::c_char = (rs as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
    let mut lr: size_t = if (*rs).tt as libc::c_int == 4i32 | 0i32 << 4i32 {
        (*rs).shrlen as libc::c_ulong
    } else {
        (*rs).u.lnglen
    };
    loop {
        /* for each segment */
        let mut temp: libc::c_int = strcoll(l, r);
        /* not equal? */
        if temp != 0i32 {
            /* done */
            return temp;
        } else {
            /* strings are equal up to a '\0' */
            /* index of first '\0' in both strings */
            let mut len: size_t = strlen(l);
            /* 'rs' is finished? */
            if len == lr {
                /* check 'ls' */
                return if len == ll { 0i32 } else { 1i32 };
            } else if len == ll {
                /* 'ls' is smaller than 'rs' ('rs' is not finished) */
                return -1i32;
            } else {
                /* both strings longer than 'len'; go on comparing after the '\0' */
                len = len.wrapping_add(1);
                l = l.offset(len as isize);
                ll = (ll as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
                r = r.offset(len as isize);
                lr = (lr as libc::c_ulong).wrapping_sub(len) as size_t as size_t
            }
        }
    }
}
/*
** Return 'l < r', for numbers.
*/
unsafe extern "C" fn LTnum(mut l: *const TValue, mut r: *const TValue) -> libc::c_int {
    if (*l).tt_ == 3i32 | 1i32 << 4i32 {
        if (*l).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((l))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                321i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int LTnum(const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        let mut li: lua_Integer = (*l).value_.i;
        if (*r).tt_ == 3i32 | 1i32 << 4i32 {
            /* both are integers */
            if (*r).tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((r))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    323i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int LTnum(const TValue *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return (li < (*r).value_.i) as libc::c_int;
        } else {
            /* 'l' is int and 'r' is float */
            /* l < r ? */
            if (*r).tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((r))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    325i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int LTnum(const TValue *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return LTintfloat(li, (*r).value_.n);
        }
    } else {
        /* 'l' must be float */
        if (*l).tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((l))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                328i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int LTnum(const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        let mut lf: lua_Number = (*l).value_.n;
        if (*r).tt_ == 3i32 | 0i32 << 4i32 {
            /* both are float */
            if (*r).tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((r))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    330i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int LTnum(const TValue *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return (lf < (*r).value_.n) as libc::c_int;
        } else if !(lf == lf) {
            /* NaN < i is always false */
            return 0i32;
        } else {
            /* without NaN, (l < r)  <-->  not(r <= l) */
            /* not (r <= l) ? */
            if (*r).tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((r))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    334i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int LTnum(const TValue *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return (0 == LEintfloat((*r).value_.i, lf)) as libc::c_int;
        }
    };
}
/*
** Check whether integer 'i' is less than or equal to float 'f'.
** See comments on previous function.
*/
unsafe extern "C" fn LEintfloat(mut i: lua_Integer, mut f: lua_Number) -> libc::c_int {
    if !(-((1i32 as lua_Integer) << 53i32) <= i && i <= (1i32 as lua_Integer) << 53i32) {
        /* -minint == maxint + 1 */
        if f >= -((-9223372036854775807i64 - 1i64) as lua_Number) {
            /* f >= maxint + 1 > i */
            return 1i32;
        } else if f >= (-9223372036854775807i64 - 1i64) as lua_Number {
            /* compare them as integers */
            return (i <= f as lua_Integer) as libc::c_int;
        } else {
            return 0i32;
        }
    } else {
        return (i as lua_Number <= f) as libc::c_int;
    };
}
/*
** Check whether integer 'i' is less than float 'f'. If 'i' has an
** exact representation as a float ('l_intfitsf'), compare numbers as
** floats. Otherwise, if 'f' is outside the range for integers, result
** is trivial. Otherwise, compare them as integers. (When 'i' has no
** float representation, either 'f' is "far away" from 'i' or 'f' has
** no precision left for a fractional part; either way, how 'f' is
** truncated is irrelevant.) When 'f' is NaN, comparisons must result
** in false.
*/
unsafe extern "C" fn LTintfloat(mut i: lua_Integer, mut f: lua_Number) -> libc::c_int {
    if !(-((1i32 as lua_Integer) << 53i32) <= i && i <= (1i32 as lua_Integer) << 53i32) {
        /* -minint == maxint + 1 */
        if f >= -((-9223372036854775807i64 - 1i64) as lua_Number) {
            /* f >= maxint + 1 > i */
            return 1i32;
        } else if f > (-9223372036854775807i64 - 1i64) as lua_Number {
            /* compare them as integers */
            return (i < f as lua_Integer) as libc::c_int;
        } else {
            return 0i32;
        }
    } else {
        return ((i as lua_Number) < f) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessequal(
    mut L: *mut lua_State_0,
    mut l: *const TValue,
    mut r: *const TValue,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    /* both operands are numbers? */
    if (*l).tt_ & 0xfi32 == 3i32 && (*r).tt_ & 0xfi32 == 3i32 {
        return LEnum(l, r);
    } else if (*l).tt_ & 0xfi32 == 4i32 && (*r).tt_ & 0xfi32 == 4i32 {
        if (*l).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((l))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                390i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                    b"int luaV_lessequal(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*l).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((l)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                390i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                    b"int luaV_lessequal(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*r).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((r))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                390i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                    b"int luaV_lessequal(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*r).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((r)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                390i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                    b"int luaV_lessequal(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        return (l_strcmp(
            &mut (*((*l).value_.gc as *mut GCUnion)).ts,
            &mut (*((*r).value_.gc as *mut GCUnion)).ts,
        ) <= 0i32) as libc::c_int;
    } else {
        /* try 'le' */
        res = luaT_callorderTM(L, l, r, TM_LE);
        if res >= 0i32 {
            return res;
        } else {
            /* try 'lt': */
            /* mark it is doing 'lt' for 'le' */
            (*(*L).ci).callstatus =
                ((*(*L).ci).callstatus as libc::c_int | 1i32 << 7i32) as libc::c_ushort;
            res = luaT_callorderTM(L, r, l, TM_LT);
            /* clear mark */
            (*(*L).ci).callstatus =
                ((*(*L).ci).callstatus as libc::c_int ^ 1i32 << 7i32) as libc::c_ushort;
            if res < 0i32 {
                luaG_ordererror(L, l, r);
            } else {
                return (0 == res) as libc::c_int;
            }
        }
    };
}
/*
** Return 'l <= r', for numbers.
*/
unsafe extern "C" fn LEnum(mut l: *const TValue, mut r: *const TValue) -> libc::c_int {
    if (*l).tt_ == 3i32 | 1i32 << 4i32 {
        if (*l).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((l))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                344i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int LEnum(const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        let mut li: lua_Integer = (*l).value_.i;
        if (*r).tt_ == 3i32 | 1i32 << 4i32 {
            /* both are integers */
            if (*r).tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((r))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    346i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int LEnum(const TValue *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return (li <= (*r).value_.i) as libc::c_int;
        } else {
            /* 'l' is int and 'r' is float */
            /* l <= r ? */
            if (*r).tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((r))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    348i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int LEnum(const TValue *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return LEintfloat(li, (*r).value_.n);
        }
    } else {
        /* 'l' must be float */
        if (*l).tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((l))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                351i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int LEnum(const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        let mut lf: lua_Number = (*l).value_.n;
        if (*r).tt_ == 3i32 | 0i32 << 4i32 {
            /* both are float */
            if (*r).tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((r))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    353i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int LEnum(const TValue *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return (lf <= (*r).value_.n) as libc::c_int;
        } else if !(lf == lf) {
            /*  NaN <= i is always false */
            return 0i32;
        } else {
            /* without NaN, (l <= r)  <-->  not(r < l) */
            /* not (r < l) ? */
            if (*r).tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((r))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    357i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int LEnum(const TValue *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return (0 == LTintfloat((*r).value_.i, lf)) as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tonumber_(
    mut obj: *const TValue,
    mut n: *mut lua_Number,
) -> libc::c_int {
    let mut v: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
        if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((obj))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                76i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                )).as_ptr(),
            );
        };
        *n = (*obj).value_.i as lua_Number;
        return 1i32;
    } else if (*obj).tt_ & 0xfi32 == 4i32 && {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((((((((((((obj))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((obj))->tt_)) & 0x0F)) == (4))\", \"lvm.c\", 80, __extension__ __PRETTY_FUNCTION__)), (((((((((obj)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((obj)->value_).gc)->tt) & 0x0F) == 4\", \"lvm.c\", 80, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((obj)->value_).gc))))->ts))))))->extra)\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as
                                            *const libc::c_char,
                                        80i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 49],
                                                                  &[libc::c_char; 49]>(b"int luaV_tonumber_(const TValue *, lua_Number *)\x00")).as_ptr());
        };
        if (*obj).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((obj))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                80i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((obj)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                80i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                )).as_ptr(),
            );
        };
        if (*obj).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((obj))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                80i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((obj)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                80i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                )).as_ptr(),
            );
        };
        luaO_str2num(
            (&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString_0 as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize),
            &mut v,
        )
            == if (*(&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString_0)).tt
                as libc::c_int == 4i32 | 0i32 << 4i32
            {
                if (*obj).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((obj))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        80i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                            b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((obj)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        80i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                            b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                        )).as_ptr(),
                    );
                };
                (*(&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString_0)).shrlen
                    as libc::c_ulong
            } else {
                if (*obj).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((obj))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        80i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                            b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((obj)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        80i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                            b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                        )).as_ptr(),
                    );
                };
                (*(&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString_0))
                    .u
                    .lnglen
            }.wrapping_add(1i32 as libc::c_ulong)
    } {
        /* convert result of 'luaO_str2num' to a float */
        if v.tt_ & 0xfi32 == 3i32 {
        } else {
            __assert_fail(
                b"(((((((&v))->tt_)) & 0x0F)) == (3))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                81i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                )).as_ptr(),
            );
        };
        *n = if v.tt_ == 3i32 | 1i32 << 4i32 {
            if v.tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((&v))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    81i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                        b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                    )).as_ptr(),
                );
            };
            v.value_.i as lua_Number
        } else {
            if v.tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((&v))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    81i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                        b"int luaV_tonumber_(const TValue *, lua_Number *)\x00",
                    )).as_ptr(),
                );
            };
            v.value_.n
        };
        return 1i32;
    } else {
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishget(
    mut L: *mut lua_State_0,
    mut t: *const TValue,
    mut key: *mut TValue,
    mut val: StkId,
    mut slot: *const TValue,
) -> () {
    /* counter to avoid infinite loops */
    let mut loop_0: libc::c_int = 0;
    /* metamethod */
    let mut tm: *const TValue = 0 as *const TValue;
    loop_0 = 0i32;
    while loop_0 < 2000i32 {
        if slot.is_null() {
            /* 't' is not a table? */
            if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
            } else {
                __assert_fail(b"!((((t))->tt_) == (((5) | (1 << 6))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 167i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 82],
                                                        &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
            };
            tm = luaT_gettmbyobj(L, t, TM_INDEX);
            if (*tm).tt_ == 0i32 {
                /* no metamethod */
                luaG_typeerror(L, t, b"index\x00" as *const u8 as *const libc::c_char);
            }
        } else {
            /* else will try the metamethod */
            /* 't' is a table */
            if (*slot).tt_ == 0i32 {
            } else {
                __assert_fail(b"((((slot))->tt_) == (0))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 174i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 82],
                                                        &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
            };
            /* table's metamethod */
            if (*t).tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 175i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 82],
                                                        &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
            };
            if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(b"(((t)->value_).gc)->tt == 5\x00" as *const u8
                                  as *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 175i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 82],
                                                        &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
            };
            tm = if (*(&mut (*((*t).value_.gc as *mut GCUnion)).h as *mut Table_0))
                .metatable
                .is_null()
            {
                0 as *const TValue
            } else {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(b"((((t))->tt_) == (((5) | (1 << 6))))\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"lvm.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      175i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 82],
                                                                &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(b"(((t)->value_).gc)->tt == 5\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"lvm.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      175i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 82],
                                                                &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                };
                if 0 != (*(*(&mut (*((*t).value_.gc as *mut GCUnion)).h as *mut Table_0)).metatable)
                    .flags as libc::c_uint & 1u32 << TM_INDEX as libc::c_int
                {
                    0 as *const TValue
                } else {
                    if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                    } else {
                        __assert_fail(b"((((t))->tt_) == (((5) | (1 << 6))))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          175i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 82],
                                                                    &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                    };
                    if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                    } else {
                        __assert_fail(b"(((t)->value_).gc)->tt == 5\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          175i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 82],
                                                                    &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                    };
                    luaT_gettm(
                        (*&mut (*((*t).value_.gc as *mut GCUnion)).h).metatable,
                        TM_INDEX,
                        (*(*L).l_G).tmname[TM_INDEX as libc::c_int as usize],
                    )
                }
            };
            if tm.is_null() {
                /* no metamethod? */
                (*val).tt_ = 0i32;
                /* result is nil */
                return;
            }
        }
        /* else will try the metamethod */
        if (*tm).tt_ & 0xfi32 == 6i32 {
            /* is metamethod a function? */
            /* call it */
            luaT_callTM(L, tm, t, key, val, 1i32);
            return;
        } else {
            /* else try to access 'tm[key]' */
            t = tm;
            if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
                slot = 0 as *const TValue;
                0i32
            } else {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(b"((((t))->tt_) == (((5) | (1 << 6))))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lvm.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         187i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 82],
                                                                   &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(b"(((t)->value_).gc)->tt == 5\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"lvm.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         187i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 82],
                                                                   &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                };
                slot = luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, key);
                !((*slot).tt_ == 0i32) as libc::c_int
            } {
                /* fast track? */
                let mut io1: *mut TValue = val;
                *io1 = *slot;
                if 0 == (*io1).tt_ & 1i32 << 6i32 || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lvm.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             188i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 82],
                                                                       &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                    };
                    (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int
                        && (L.is_null() || {
                            if 0 != (*io1).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          b"lvm.c\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          188i32 as
                                                              libc::c_uint,
                                                          (*::std::mem::transmute::<&[u8; 82],
                                                                                    &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                            };
                            0 != ((*(*io1).value_.gc).marked as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                                & ((*(*L).l_G).currentwhite as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                        })
                } {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(b"0\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"lvm.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      188i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 82],
                                                                &[libc::c_char; 82]>(b"void luaV_finishget(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                    };
                };
                /* done */
                return;
            } else {
                loop_0 += 1
            }
        }
    }
    /* else repeat (tail call 'luaV_finishget') */
    luaG_runerror(
        L,
        b"\'__index\' chain too long; possible loop\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishset(
    mut L: *mut lua_State_0,
    mut t: *const TValue,
    mut key: *mut TValue,
    mut val: StkId,
    mut slot: *const TValue,
) -> () {
    /* counter to avoid infinite loops */
    let mut loop_0: libc::c_int = 0;
    loop_0 = 0i32;
    while loop_0 < 2000i32 {
        /* '__newindex' metamethod */
        let mut tm: *const TValue = 0 as *const TValue;
        if !slot.is_null() {
            /* is 't' a table? */
            /* save 't' table */
            if (*t).tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 210i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 82],
                                                        &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
            };
            if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(b"(((t)->value_).gc)->tt == 5\x00" as *const u8
                                  as *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 210i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 82],
                                                        &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
            };
            let mut h: *mut Table = &mut (*((*t).value_.gc as *mut GCUnion)).h;
            if (*slot).tt_ == 0i32 {
            } else {
                __assert_fail(b"((((slot))->tt_) == (0))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 211i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 82],
                                                        &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
            };
            /* old value must be nil */
            /* get metamethod */
            tm = if (*h).metatable.is_null() {
                0 as *const TValue
            } else if 0
                != (*(*h).metatable).flags as libc::c_uint & 1u32 << TM_NEWINDEX as libc::c_int
            {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*h).metatable,
                    TM_NEWINDEX,
                    (*(*L).l_G).tmname[TM_NEWINDEX as libc::c_int as usize],
                )
            };
            if tm.is_null() {
                /* no metamethod? */
                /* no previous entry? */
                if slot == &luaO_nilobject_ as *const TValue {
                    /* create one */
                    slot = luaH_newkey(L, h, key)
                }
                *(slot as *mut TValue) = *val;
                if 0 == (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 || {
                    if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lvm.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             217i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 82],
                                                                       &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                    };
                    (*(slot as *mut TValue)).tt_ & 0x3fi32
                        == (*(*(slot as *mut TValue)).value_.gc).tt as libc::c_int
                        && (L.is_null() || {
                            if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          b"lvm.c\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          217i32 as
                                                              libc::c_uint,
                                                          (*::std::mem::transmute::<&[u8; 82],
                                                                                    &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                            };
                            0 != ((*(*(slot as *mut TValue)).value_.gc).marked as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                                & ((*(*L).l_G).currentwhite as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                        })
                } {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(b"0\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"lvm.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      217i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 82],
                                                                &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                    };
                };
                (*h).flags = 0i32 as lu_byte;
                if 0 != (*val).tt_ & 1i32 << 6i32 && 0 != (*h).marked as libc::c_int & 1i32 << 2i32
                    && {
                        if 0 != (*val).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"(((val)->tt_) & (1 << 6))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lvm.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             219i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 82],
                                                                       &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                        };
                        0 != (*(*val).value_.gc).marked as libc::c_int
                            & (1i32 << 0i32 | 1i32 << 1i32)
                    } {
                    luaC_barrierback_(L, h);
                } else {
                };
                /* no metamethod and (now) there is an entry with given key */
                /* set its new value */
                return;
            }
        } else {
            /* else will try the metamethod */
            /* not a table; check metamethod */
            tm = luaT_gettmbyobj(L, t, TM_NEWINDEX);
            if (*tm).tt_ == 0i32 {
                luaG_typeerror(L, t, b"index\x00" as *const u8 as *const libc::c_char);
            }
        }
        /* try the metamethod */
        if (*tm).tt_ & 0xfi32 == 6i32 {
            luaT_callTM(L, tm, t, key, val, 0i32);
            return;
        } else {
            /* else repeat assignment over 'tm' */
            t = tm;
            if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
                slot = 0 as *const TValue;
                0i32
            } else {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(b"((((t))->tt_) == (((5) | (1 << 6))))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lvm.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         234i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 82],
                                                                   &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(b"(((t)->value_).gc)->tt == 5\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"lvm.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         234i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 82],
                                                                   &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                };
                slot = luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, key);
                if (*slot).tt_ == 0i32 {
                    0i32
                } else {
                    if 0 != (*val).tt_ & 1i32 << 6i32 && {
                        if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"((((t))->tt_) == (((5) | (1 << 6))))\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        b"lvm.c\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        234i32 as
                                                            libc::c_uint,
                                                        (*::std::mem::transmute::<&[u8; 82],
                                                                                  &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                        };
                        if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(b"(((t)->value_).gc)->tt == 5\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        b"lvm.c\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        234i32 as
                                                            libc::c_uint,
                                                        (*::std::mem::transmute::<&[u8; 82],
                                                                                  &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                        };
                        0 != (*(&mut (*((*t).value_.gc as *mut GCUnion)).h as *mut Table_0)).marked
                            as libc::c_int & 1i32 << 2i32
                    } && {
                        if 0 != (*val).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"(((val)->tt_) & (1 << 6))\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        b"lvm.c\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        234i32 as
                                                            libc::c_uint,
                                                        (*::std::mem::transmute::<&[u8; 82],
                                                                                  &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                        };
                        0 != (*(*val).value_.gc).marked as libc::c_int
                            & (1i32 << 0i32 | 1i32 << 1i32)
                    } {
                        if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"((((t))->tt_) == (((5) | (1 << 6))))\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"lvm.c\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 234i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 82],
                                                                           &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                        };
                        if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(b"(((t)->value_).gc)->tt == 5\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"lvm.c\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 234i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 82],
                                                                           &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                        };
                        luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
                    } else {
                    };
                    *(slot as *mut TValue) = *val;
                    if 0 == (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 || {
                        if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        b"lvm.c\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        234i32 as
                                                            libc::c_uint,
                                                        (*::std::mem::transmute::<&[u8; 82],
                                                                                  &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                        };
                        (*(slot as *mut TValue)).tt_ & 0x3fi32
                            == (*(*(slot as *mut TValue)).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"lvm.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     234i32 as
                                                                         libc::c_uint,
                                                                     (*::std::mem::transmute::<&[u8; 82],
                                                                                               &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                                };
                                0 != ((*(*(slot as *mut TValue)).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(b"0\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 b"lvm.c\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 234i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 82],
                                                                           &[libc::c_char; 82]>(b"void luaV_finishset(lua_State *, const TValue *, TValue *, StkId, const TValue *)\x00")).as_ptr());
                        };
                    };
                    1i32
                }
            } {
                /* done */
                return;
            } else {
                loop_0 += 1
            }
        }
    }
    /* else loop */
    luaG_runerror(
        L,
        b"\'__newindex\' chain too long; possible loop\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishOp(mut L: *mut lua_State_0) -> () {
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let mut base: StkId = (*ci).u.l.base;
    /* interrupted instruction */
    let mut inst: Instruction = *(*ci).u.l.savedpc.offset(-1isize);
    let mut op: OpCode = (inst >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
    match op as libc::c_uint {
        13 | 14 | 15 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 16 | 17 | 25 | 26 | 28 | 6 | 7 | 12 => {
            let mut io1: *mut TValue = base.offset(
                (inst >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int
                    as isize,
            );
            (*L).top = (*L).top.offset(-1isize);
            *io1 = *(*L).top;
            if 0 == (*io1).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        668i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void luaV_finishOp(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            668i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                                b"void luaV_finishOp(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    0 != ((*(*io1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                        & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        668i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void luaV_finishOp(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
            };
        }
        33 | 32 | 31 => {
            let mut res: libc::c_int = !((*(*L).top.offset(-1isize)).tt_ == 0i32
                || (*(*L).top.offset(-1isize)).tt_ == 1i32 && {
                    if (*(*L).top.offset(-1isize)).tt_ == 1i32 {
                    } else {
                        __assert_fail(
                            b"((((L->top - 1))->tt_) == (1))\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            672i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                                b"void luaV_finishOp(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*(*L).top.offset(-1isize)).value_.b == 0i32
                }) as libc::c_int;
            (*L).top = (*L).top.offset(-1isize);
            if 0 != (*ci).callstatus as libc::c_int & 1i32 << 7i32 {
                /* "<=" using "<" instead? */
                if op as libc::c_uint == OP_LE as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(
                        b"op == OP_LE\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        675i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void luaV_finishOp(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                /* clear mark */
                (*ci).callstatus =
                    ((*ci).callstatus as libc::c_int ^ 1i32 << 7i32) as libc::c_ushort;
                /* negate result */
                res = (0 == res) as libc::c_int
            }
            if (*(*ci).u.l.savedpc >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
                as libc::c_uint == OP_JMP as libc::c_int as libc::c_uint
            {
            } else {
                __assert_fail(b"(((OpCode)(((*ci->u.l.savedpc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) == OP_JMP\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 679i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 32],
                                                        &[libc::c_char; 32]>(b"void luaV_finishOp(lua_State *)\x00")).as_ptr());
            };
            /* condition failed? */
            if res != (inst >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                as libc::c_int
            {
                /* skip jump instruction */
                (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize)
            }
        }
        29 => {
            /* top when 'luaT_trybinTM' was called */
            let mut top: StkId = (*L).top.offset(-1isize);
            /* first element to concatenate */
            let mut b: libc::c_int = (inst >> 0i32 + 6i32 + 8i32 + 9i32
                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                as libc::c_int;
            /* yet to concatenate */
            let mut total: libc::c_int = top
                .offset(-1isize)
                .wrapping_offset_from(base.offset(b as isize))
                as libc::c_long as libc::c_int;
            let mut io1_0: *mut TValue = top.offset(-2isize);
            *io1_0 = *top;
            if 0 == (*io1_0).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        688i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void luaV_finishOp(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                (*io1_0).tt_ & 0x3fi32 == (*(*io1_0).value_.gc).tt as libc::c_int
                    && (L.is_null() || {
                        if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                688i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                                    b"void luaV_finishOp(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        0 != ((*(*io1_0).value_.gc).marked as libc::c_int
                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                            & ((*(*L).l_G).currentwhite as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                    })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        688i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void luaV_finishOp(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            /* put TM result in proper position */
            if total > 1i32 {
                /* are there elements to concat? */
                /* top is one after last element (at top-2) */
                (*L).top = top.offset(-1isize);
                /* concat them (may yield again) */
                luaV_concat(L, total);
            }
            let mut io1_1: *mut TValue = (*ci).u.l.base.offset(
                (inst >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int
                    as isize,
            );
            *io1_1 = *(*L).top.offset(-1isize);
            if 0 == (*io1_1).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        694i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void luaV_finishOp(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                (*io1_1).tt_ & 0x3fi32 == (*(*io1_1).value_.gc).tt as libc::c_int
                    && (L.is_null() || {
                        if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                694i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                                    b"void luaV_finishOp(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        0 != ((*(*io1_1).value_.gc).marked as libc::c_int
                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                            & ((*(*L).l_G).currentwhite as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                    })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        694i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void luaV_finishOp(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            /* move final result to final position */
            /* restore top */
            (*L).top = (*ci).top
        }
        41 => {
            if (*(*ci).u.l.savedpc >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
                as libc::c_uint == OP_TFORLOOP as libc::c_int as libc::c_uint
            {
            } else {
                __assert_fail(b"(((OpCode)(((*ci->u.l.savedpc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) == OP_TFORLOOP\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lvm.c\x00" as *const u8 as
                                  *const libc::c_char, 699i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 32],
                                                        &[libc::c_char; 32]>(b"void luaV_finishOp(lua_State *)\x00")).as_ptr());
            };
            /* correct top */
            (*L).top = (*ci).top
        }
        36 => {
            /* nresults >= 0? */
            if (inst >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                as libc::c_int - 1i32 >= 0i32
            {
                /* adjust results */
                (*L).top = (*ci).top
            }
        }
        37 | 8 | 10 => {}
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    710i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"void luaV_finishOp(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_concat(mut L: *mut lua_State_0, mut total: libc::c_int) -> () {
    let mut buff: [libc::c_char; 40] = [0; 40];
    if total >= 2i32 {
    } else {
        __assert_fail(
            b"total >= 2\x00" as *const u8 as *const libc::c_char,
            b"lvm.c\x00" as *const u8 as *const libc::c_char,
            476i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"void luaV_concat(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    loop {
        let mut top: StkId = (*L).top;
        /* number of elements handled in this pass (at least 2) */
        let mut n: libc::c_int = 2i32;
        if !((*top.offset(-2isize)).tt_ & 0xfi32 == 4i32
            || (*top.offset(-2isize)).tt_ & 0xfi32 == 3i32)
            || !((*top.offset(-1isize)).tt_ & 0xfi32 == 4i32
                || (*top.offset(-1isize)).tt_ & 0xfi32 == 3i32 && {
                    luaO_tostring(L, top.offset(-1isize));
                    0 != 1i32
                }) {
            luaT_trybinTM(
                L,
                top.offset(-2isize) as *const TValue,
                top.offset(-1isize) as *const TValue,
                top.offset(-2isize),
                TM_CONCAT,
            );
        } else if (*top.offset(-1isize)).tt_ == 4i32 | 0i32 << 4i32 | 1i32 << 6i32 && {
            if (*top.offset(-1isize)).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((top - 1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    482i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void luaV_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*top.offset(-1isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((top - 1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    482i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void luaV_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*(&mut (*((*top.offset(-1isize)).value_.gc as *mut GCUnion)).ts as *mut TString_0))
                .shrlen as libc::c_int == 0i32
        } {
            ((*top.offset(-2isize)).tt_ & 0xfi32 == 4i32
                || (*top.offset(-2isize)).tt_ & 0xfi32 == 3i32 && {
                    luaO_tostring(L, top.offset(-2isize));
                    0 != 1i32
                }) as libc::c_int;
        } else if (*top.offset(-2isize)).tt_ == 4i32 | 0i32 << 4i32 | 1i32 << 6i32 && {
            if (*top.offset(-2isize)).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((top - 2))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    484i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void luaV_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*top.offset(-2isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((top - 2)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    484i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void luaV_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*(&mut (*((*top.offset(-2isize)).value_.gc as *mut GCUnion)).ts as *mut TString_0))
                .shrlen as libc::c_int == 0i32
        } {
            /* first operand is an empty string? */
            let mut io1: *mut TValue = top.offset(-2isize);
            *io1 = *top.offset(-1isize);
            if 0 == (*io1).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        485i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            485i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                                b"void luaV_concat(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    0 != ((*(*io1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                        & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        485i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
            };
        } else {
            /* result is second op. */
            /* at least two non-empty string values; get as many as possible */
            if (*top.offset(-1isize)).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((top - 1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    489i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void luaV_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*top.offset(-1isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((top - 1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    489i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void luaV_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            let mut tl: size_t = if (*(&mut (*((*top.offset(-1isize)).value_.gc as *mut GCUnion)).ts
                as *mut TString_0))
                .tt as libc::c_int == 4i32 | 0i32 << 4i32
            {
                if (*top.offset(-1isize)).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((top - 1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        489i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*top.offset(-1isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((top - 1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        489i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                (*&mut (*((*top.offset(-1isize)).value_.gc as *mut GCUnion)).ts).shrlen
                    as libc::c_ulong
            } else {
                if (*top.offset(-1isize)).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((top - 1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        489i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*top.offset(-1isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((top - 1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        489i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                (*&mut (*((*top.offset(-1isize)).value_.gc as *mut GCUnion)).ts)
                    .u
                    .lnglen
            };
            let mut ts: *mut TString = 0 as *mut TString;
            /* collect total length and number of strings */
            n = 1i32;
            while n < total
                && ((*top.offset(-(n as isize)).offset(-1isize)).tt_ & 0xfi32 == 4i32
                    || (*top.offset(-(n as isize)).offset(-1isize)).tt_ & 0xfi32 == 3i32 && {
                        luaO_tostring(L, top.offset(-(n as isize)).offset(-1isize));
                        0 != 1i32
                    }) {
                if (*top.offset(-(n as isize)).offset(-1isize)).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((top - n - 1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        493i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*top.offset(-(n as isize)).offset(-1isize)).value_.gc).tt as libc::c_int
                    & 0xfi32 == 4i32
                {
                } else {
                    __assert_fail(
                        b"(((((top - n - 1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        493i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                let mut l: size_t = if (*(&mut (*((*top.offset(-(n as isize)).offset(-1isize))
                    .value_
                    .gc as *mut GCUnion))
                    .ts as *mut TString_0))
                    .tt as libc::c_int == 4i32 | 0i32 << 4i32
                {
                    if (*top.offset(-(n as isize)).offset(-1isize)).tt_ & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((((top - n - 1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            493i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                                b"void luaV_concat(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*top.offset(-(n as isize)).offset(-1isize)).value_.gc).tt as libc::c_int
                        & 0xfi32 == 4i32
                    {
                    } else {
                        __assert_fail(
                            b"(((((top - n - 1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            493i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                                b"void luaV_concat(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*&mut (*((*top.offset(-(n as isize)).offset(-1isize)).value_.gc
                        as *mut GCUnion))
                        .ts)
                        .shrlen as libc::c_ulong
                } else {
                    if (*top.offset(-(n as isize)).offset(-1isize)).tt_ & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((((top - n - 1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            493i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                                b"void luaV_concat(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*top.offset(-(n as isize)).offset(-1isize)).value_.gc).tt as libc::c_int
                        & 0xfi32 == 4i32
                    {
                    } else {
                        __assert_fail(
                            b"(((((top - n - 1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            493i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                                b"void luaV_concat(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*&mut (*((*top.offset(-(n as isize)).offset(-1isize)).value_.gc
                        as *mut GCUnion))
                        .ts)
                        .u
                        .lnglen
                };
                if l >= if (::std::mem::size_of::<size_t>() as libc::c_ulong)
                    < ::std::mem::size_of::<lua_Integer>() as libc::c_ulong
                {
                    !(0i32 as size_t)
                } else {
                    9223372036854775807i64 as size_t
                }.wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(tl)
                {
                    luaG_runerror(
                        L,
                        b"string length overflow\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                    tl = (tl as libc::c_ulong).wrapping_add(l) as size_t as size_t;
                    n += 1
                }
            }
            if tl <= 40i32 as libc::c_ulong {
                /* is result a short string? */
                buff = [0; 40];
                /* copy strings to buffer */
                copy2buff(top, n, buff.as_mut_ptr());
                ts = luaS_newlstr(L, buff.as_mut_ptr(), tl)
            } else {
                /* long string; copy strings directly to final result */
                ts = luaS_createlngstrobj(L, tl);
                if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
                } else {
                    __assert_fail(
                        b"sizeof((ts)->extra)\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        505i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                copy2buff(
                    top,
                    n,
                    (ts as *mut libc::c_char)
                        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize),
                );
            }
            let mut io: *mut TValue = top.offset(-(n as isize));
            let mut x_: *mut TString = ts;
            if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    507i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void luaV_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
            if 0 == (*io).tt_ & 1i32 << 6i32 || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        507i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
                    if 0 != (*io).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            507i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                                b"void luaV_concat(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    0 != ((*(*io).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                        & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        507i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"void luaV_concat(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
            };
        }
        /* create result */
        /* got 'n' strings to create 1 new */
        total -= n - 1i32;
        /* popped 'n' strings and pushed one */
        (*L).top = (*L).top.offset(-((n - 1i32) as isize));
        if !(total > 1i32) {
            break;
        }
    }
}
/* macro used by 'luaV_concat' to ensure that element at 'o' is a string */
/* copy strings in stack from top - n up to top - 1 to buffer */
unsafe extern "C" fn copy2buff(
    mut top: StkId,
    mut n: libc::c_int,
    mut buff: *mut libc::c_char,
) -> () {
    /* size already copied */
    let mut tl: size_t = 0i32 as size_t;
    loop {
        /* length of string being copied */
        if (*top.offset(-(n as isize))).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((top - n))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                464i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"void copy2buff(StkId, int, char *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*top.offset(-(n as isize))).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((top - n)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                    as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                464i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"void copy2buff(StkId, int, char *)\x00",
                )).as_ptr(),
            );
        };
        let mut l: size_t = if (*(&mut (*((*top.offset(-(n as isize))).value_.gc as *mut GCUnion))
            .ts as *mut TString_0))
            .tt as libc::c_int == 4i32 | 0i32 << 4i32
        {
            if (*top.offset(-(n as isize))).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((top - n))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    464i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void copy2buff(StkId, int, char *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*top.offset(-(n as isize))).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((top - n)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    464i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void copy2buff(StkId, int, char *)\x00",
                    )).as_ptr(),
                );
            };
            (*&mut (*((*top.offset(-(n as isize))).value_.gc as *mut GCUnion)).ts).shrlen
                as libc::c_ulong
        } else {
            if (*top.offset(-(n as isize))).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((top - n))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    464i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void copy2buff(StkId, int, char *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*top.offset(-(n as isize))).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((top - n)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    464i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void copy2buff(StkId, int, char *)\x00",
                    )).as_ptr(),
                );
            };
            (*&mut (*((*top.offset(-(n as isize))).value_.gc as *mut GCUnion)).ts)
                .u
                .lnglen
        };
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((((((((((((top - n))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((top - n))->tt_)) & 0x0F)) == (4))\", \"lvm.c\", 465, __extension__ __PRETTY_FUNCTION__)), (((((((((top - n)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((top - n)->value_).gc)->tt) & 0x0F) == 4\", \"lvm.c\", 465, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((top - n)->value_).gc))))->ts))))))->extra)\x00"
                              as *const u8 as *const libc::c_char,
                          b"lvm.c\x00" as *const u8 as *const libc::c_char,
                          465i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void copy2buff(StkId, int, char *)\x00")).as_ptr());
        };
        if (*top.offset(-(n as isize))).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((top - n))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                465i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"void copy2buff(StkId, int, char *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*top.offset(-(n as isize))).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((top - n)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                    as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                465i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"void copy2buff(StkId, int, char *)\x00",
                )).as_ptr(),
            );
        };
        memcpy(
            buff.offset(tl as isize) as *mut libc::c_void,
            (&mut (*((*top.offset(-(n as isize))).value_.gc as *mut GCUnion)).ts as *mut TString_0
                as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                as *const libc::c_void,
            l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        tl = (tl as libc::c_ulong).wrapping_add(l) as size_t as size_t;
        n -= 1;
        if !(n > 0i32) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaV_execute(mut L: *mut lua_State_0) -> () {
    let mut x__0: *mut LClosure_0 = 0 as *mut LClosure_0;
    let mut io_31: *mut TValue = 0 as *mut TValue;
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let mut cl: *mut LClosure_0 = 0 as *mut LClosure_0;
    let mut k: *mut TValue = 0 as *mut TValue;
    let mut base: StkId = 0 as *mut TValue;
    /* fresh invocation of 'luaV_execute" */
    (*ci).callstatus = ((*ci).callstatus as libc::c_int | 1i32 << 3i32) as libc::c_ushort;
    /* reentry point when frame changes (call/return) */
    loop {
        if ci == (*L).ci {
        } else {
            __assert_fail(
                b"ci == L->ci\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                794i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"void luaV_execute(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        /* local reference to function's closure */
        if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((ci->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                    as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                795i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"void luaV_execute(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*(*ci).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"(((ci->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                    as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                795i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"void luaV_execute(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        cl = &mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l;
        /* local reference to function's constant table */
        k = (*(*cl).p).k;
        /* local copy of function's base */
        base = (*ci).u.l.base;
        /* main loop of interpreter */
        's_17: loop {
            let mut i: Instruction = 0;
            let mut ra: StkId = 0 as *mut TValue;
            let fresh0 = (*ci).u.l.savedpc;
            (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1);
            i = *fresh0;
            if 0 != (*L).hookmask & (1i32 << 2i32 | 1i32 << 3i32) {
                luaG_traceexec(L);
                base = (*ci).u.l.base
            }
            ra = base.offset(
                (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int
                    as isize,
            );
            if base == (*ci).u.l.base {
            } else {
                __assert_fail(
                    b"base == ci->u.l.base\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    802i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                        b"void luaV_execute(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            if base <= (*L).top && (*L).top < (*L).stack.offset((*L).stacksize as isize) {
            } else {
                __assert_fail(
                    b"base <= L->top && L->top < L->stack + L->stacksize\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    802i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                        b"void luaV_execute(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            match (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
                as libc::c_uint
            {
                0 => {
                    let mut io1: *mut TValue = ra;
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgR as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgR\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          805i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    *io1 = *base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                805i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        805i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                805i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    continue;
                }
                1 => {
                    let mut rb: *mut TValue = k.offset(
                        (i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    let mut io1_0: *mut TValue = ra;
                    *io1_0 = *rb;
                    if 0 == (*io1_0).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                810i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_0).tt_ & 0x3fi32 == (*(*io1_0).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        810i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_0).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                810i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    continue;
                }
                2 => {
                    let mut rb_0: *mut TValue = 0 as *mut TValue;
                    if (*(*ci).u.l.savedpc >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                        as OpCode as libc::c_uint
                        == OP_EXTRAARG as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((OpCode)(((*ci->u.l.savedpc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) == OP_EXTRAARG\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          815i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let fresh1 = (*ci).u.l.savedpc;
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1);
                    rb_0 = k.offset(
                        (*fresh1 >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    let mut io1_1: *mut TValue = ra;
                    *io1_1 = *rb_0;
                    if 0 == (*io1_1).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                817i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_1).tt_ & 0x3fi32 == (*(*io1_1).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        817i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_1).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                817i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    continue;
                }
                3 => {
                    let mut io: *mut TValue = ra;
                    (*io).value_.b = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    (*io).tt_ = 1i32;
                    if !(0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int)
                    {
                        continue;
                    }
                    /* skip next instruction (if C) */
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize);
                    continue;
                }
                4 => {
                    let mut b: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    loop {
                        let fresh2 = ra;
                        ra = ra.offset(1);
                        (*fresh2).tt_ = 0i32;
                        let fresh3 = b;
                        b = b - 1;
                        if !(0 != fresh3) {
                            continue 's_17;
                        }
                    }
                }
                5 => {
                    let mut b_0: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    let mut io1_2: *mut TValue = ra;
                    *io1_2 = *(**(*cl).upvals.as_mut_ptr().offset(b_0 as isize)).v;
                    if 0 == (*io1_2).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_2).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                834i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_2).tt_ & 0x3fi32 == (*(*io1_2).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_2).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        834i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_2).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                834i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    continue;
                }
                6 => {
                    let mut upval: *mut TValue = (**(*cl).upvals.as_mut_ptr().offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    )).v;
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          839i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut slot: *const TValue = 0 as *const TValue;
                    if 0 != if !((*upval).tt_ == 5i32 | 1i32 << 6i32) {
                        slot = 0 as *const TValue;
                        0i32
                    } else {
                        if (*upval).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((upval))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                840i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*upval).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(
                                b"(((upval)->value_).gc)->tt == 5\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                840i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        slot = luaH_get(&mut (*((*upval).value_.gc as *mut GCUnion)).h, rc);
                        !((*slot).tt_ == 0i32) as libc::c_int
                    } {
                        let mut io1_3: *mut TValue = ra;
                        *io1_3 = *slot;
                        if 0 == (*io1_3).tt_ & 1i32 << 6i32 || {
                            if 0 != (*io1_3).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(
                                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    840i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*io1_3).tt_ & 0x3fi32 == (*(*io1_3).value_.gc).tt as libc::c_int
                                && (L.is_null() || {
                                    if 0 != (*io1_3).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(
                                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                                as *const libc::c_char,
                                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                            840i32 as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 31],
                                                &[libc::c_char; 31],
                                            >(
                                                b"void luaV_execute(lua_State *)\x00"
                                            )).as_ptr(),
                                        );
                                    };
                                    0 != ((*(*io1_3).value_.gc).marked as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                                        & ((*(*L).l_G).currentwhite as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                })
                        } {
                        } else {
                            if 0 != 0i32 {
                            } else {
                                __assert_fail(
                                    b"0\x00" as *const u8 as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    840i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                        };
                        continue;
                    } else {
                        luaV_finishget(L, upval, rc, ra, slot);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                7 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgR as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgR\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          844i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_1: StkId = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          845i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_0: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut slot_0: *const TValue = 0 as *const TValue;
                    if 0 != if !((*rb_1).tt_ == 5i32 | 1i32 << 6i32) {
                        slot_0 = 0 as *const TValue;
                        0i32
                    } else {
                        if (*rb_1).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                846i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*rb_1).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(
                                b"(((rb)->value_).gc)->tt == 5\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                846i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        slot_0 = luaH_get(&mut (*((*rb_1).value_.gc as *mut GCUnion)).h, rc_0);
                        !((*slot_0).tt_ == 0i32) as libc::c_int
                    } {
                        let mut io1_4: *mut TValue = ra;
                        *io1_4 = *slot_0;
                        if 0 == (*io1_4).tt_ & 1i32 << 6i32 || {
                            if 0 != (*io1_4).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(
                                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    846i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*io1_4).tt_ & 0x3fi32 == (*(*io1_4).value_.gc).tt as libc::c_int
                                && (L.is_null() || {
                                    if 0 != (*io1_4).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(
                                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                                as *const libc::c_char,
                                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                            846i32 as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 31],
                                                &[libc::c_char; 31],
                                            >(
                                                b"void luaV_execute(lua_State *)\x00"
                                            )).as_ptr(),
                                        );
                                    };
                                    0 != ((*(*io1_4).value_.gc).marked as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                                        & ((*(*L).l_G).currentwhite as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                })
                        } {
                        } else {
                            if 0 != 0i32 {
                            } else {
                                __assert_fail(
                                    b"0\x00" as *const u8 as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    846i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                        };
                        continue;
                    } else {
                        luaV_finishget(L, rb_1 as *const TValue, rc_0, ra, slot_0);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                8 => {
                    let mut upval_0: *mut TValue = (**(*cl).upvals.as_mut_ptr().offset(
                        (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int as isize,
                    )).v;
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          851i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_2: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          852i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_1: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut slot_1: *const TValue = 0 as *const TValue;
                    if !(0 == if !((*upval_0).tt_ == 5i32 | 1i32 << 6i32) {
                        slot_1 = 0 as *const TValue;
                        0i32
                    } else {
                        if (*upval_0).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((upval))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                853i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*upval_0).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(
                                b"(((upval)->value_).gc)->tt == 5\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                853i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        slot_1 = luaH_get(&mut (*((*upval_0).value_.gc as *mut GCUnion)).h, rb_2);
                        if (*slot_1).tt_ == 0i32 {
                            0i32
                        } else {
                            if 0 != (*rc_1).tt_ & 1i32 << 6i32 && {
                                if (*upval_0).tt_ == 5i32 | 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"((((upval))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        853i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                if (*(*upval_0).value_.gc).tt as libc::c_int == 5i32 {
                                } else {
                                    __assert_fail(
                                        b"(((upval)->value_).gc)->tt == 5\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        853i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != (*(&mut (*((*upval_0).value_.gc as *mut GCUnion)).h
                                    as *mut Table_0))
                                    .marked as libc::c_int
                                    & 1i32 << 2i32
                            } && {
                                if 0 != (*rc_1).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((rc)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        853i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != (*(*rc_1).value_.gc).marked as libc::c_int
                                    & (1i32 << 0i32 | 1i32 << 1i32)
                            } {
                                if (*upval_0).tt_ == 5i32 | 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"((((upval))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        853i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                if (*(*upval_0).value_.gc).tt as libc::c_int == 5i32 {
                                } else {
                                    __assert_fail(
                                        b"(((upval)->value_).gc)->tt == 5\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        853i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                luaC_barrierback_(
                                    L,
                                    &mut (*((*upval_0).value_.gc as *mut GCUnion)).h,
                                );
                            } else {
                            };
                            *(slot_1 as *mut TValue) = *rc_1;
                            if 0 == (*(slot_1 as *mut TValue)).tt_ & 1i32 << 6i32 || {
                                if 0 != (*(slot_1 as *mut TValue)).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        853i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                (*(slot_1 as *mut TValue)).tt_ & 0x3fi32
                                    == (*(*(slot_1 as *mut TValue)).value_.gc).tt as libc::c_int
                                    && (L.is_null() || {
                                        if 0 != (*(slot_1 as *mut TValue)).tt_ & 1i32 << 6i32 {
                                        } else {
                                            __assert_fail(
                                                b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                                853i32 as libc::c_uint,
                                                (*::std::mem::transmute::<
                                                    &[u8; 31],
                                                    &[libc::c_char; 31],
                                                >(
                                                    b"void luaV_execute(lua_State *)\x00"
                                                )).as_ptr(),
                                            );
                                        };
                                        0 != ((*(*(slot_1 as *mut TValue)).value_.gc).marked
                                            as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                            & ((*(*L).l_G).currentwhite as libc::c_int
                                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    })
                            } {
                            } else {
                                if 0 != 0i32 {
                                } else {
                                    __assert_fail(
                                        b"0\x00" as *const u8 as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        853i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                            };
                            1i32
                        }
                    }) {
                        continue;
                    }
                    luaV_finishset(L, upval_0, rb_2, rc_1, slot_1);
                    base = (*ci).u.l.base;
                    continue;
                }
                9 => {
                    let mut uv: *mut UpVal = *(*cl).upvals.as_mut_ptr().offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    let mut io1_5: *mut TValue = (*uv).v;
                    *io1_5 = *ra;
                    if 0 == (*io1_5).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_5).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                858i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_5).tt_ & 0x3fi32 == (*(*io1_5).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_5).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        858i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_5).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                858i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32
                        && !((*uv).v != &mut (*uv).u.value as *mut TValue)
                    {
                        luaC_upvalbarrier_(L, uv);
                    } else {
                    };
                    continue;
                }
                10 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          863i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_3: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          864i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_2: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut slot_2: *const TValue = 0 as *const TValue;
                    if !(0 == if !((*ra).tt_ == 5i32 | 1i32 << 6i32) {
                        slot_2 = 0 as *const TValue;
                        0i32
                    } else {
                        if (*ra).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((ra))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                865i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*ra).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(
                                b"(((ra)->value_).gc)->tt == 5\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                865i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        slot_2 = luaH_get(&mut (*((*ra).value_.gc as *mut GCUnion)).h, rb_3);
                        if (*slot_2).tt_ == 0i32 {
                            0i32
                        } else {
                            if 0 != (*rc_2).tt_ & 1i32 << 6i32 && {
                                if (*ra).tt_ == 5i32 | 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"((((ra))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        865i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                if (*(*ra).value_.gc).tt as libc::c_int == 5i32 {
                                } else {
                                    __assert_fail(
                                        b"(((ra)->value_).gc)->tt == 5\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        865i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != (*(&mut (*((*ra).value_.gc as *mut GCUnion)).h
                                    as *mut Table_0))
                                    .marked as libc::c_int
                                    & 1i32 << 2i32
                            } && {
                                if 0 != (*rc_2).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((rc)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        865i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != (*(*rc_2).value_.gc).marked as libc::c_int
                                    & (1i32 << 0i32 | 1i32 << 1i32)
                            } {
                                if (*ra).tt_ == 5i32 | 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"((((ra))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        865i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                if (*(*ra).value_.gc).tt as libc::c_int == 5i32 {
                                } else {
                                    __assert_fail(
                                        b"(((ra)->value_).gc)->tt == 5\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        865i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                luaC_barrierback_(L, &mut (*((*ra).value_.gc as *mut GCUnion)).h);
                            } else {
                            };
                            *(slot_2 as *mut TValue) = *rc_2;
                            if 0 == (*(slot_2 as *mut TValue)).tt_ & 1i32 << 6i32 || {
                                if 0 != (*(slot_2 as *mut TValue)).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        865i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                (*(slot_2 as *mut TValue)).tt_ & 0x3fi32
                                    == (*(*(slot_2 as *mut TValue)).value_.gc).tt as libc::c_int
                                    && (L.is_null() || {
                                        if 0 != (*(slot_2 as *mut TValue)).tt_ & 1i32 << 6i32 {
                                        } else {
                                            __assert_fail(
                                                b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                                865i32 as libc::c_uint,
                                                (*::std::mem::transmute::<
                                                    &[u8; 31],
                                                    &[libc::c_char; 31],
                                                >(
                                                    b"void luaV_execute(lua_State *)\x00"
                                                )).as_ptr(),
                                            );
                                        };
                                        0 != ((*(*(slot_2 as *mut TValue)).value_.gc).marked
                                            as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                            & ((*(*L).l_G).currentwhite as libc::c_int
                                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    })
                            } {
                            } else {
                                if 0 != 0i32 {
                                } else {
                                    __assert_fail(
                                        b"0\x00" as *const u8 as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        865i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                            };
                            1i32
                        }
                    }) {
                        continue;
                    }
                    luaV_finishset(L, ra as *const TValue, rb_3, rc_2, slot_2);
                    base = (*ci).u.l.base;
                    continue;
                }
                11 => {
                    let mut b_1: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    let mut c: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    let mut t: *mut Table = luaH_new(L);
                    let mut io_0: *mut TValue = ra;
                    let mut x_: *mut Table = t;
                    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                    } else {
                        __assert_fail(
                            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            872i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                b"void luaV_execute(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*io_0).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                    (*io_0).tt_ = 5i32 | 1i32 << 6i32;
                    if 0 == (*io_0).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io_0).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                872i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io_0).tt_ & 0x3fi32 == (*(*io_0).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io_0).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        872i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io_0).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                872i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    if b_1 != 0i32 || c != 0i32 {
                        luaH_resize(
                            L,
                            t,
                            luaO_fb2int(b_1) as libc::c_uint,
                            luaO_fb2int(c) as libc::c_uint,
                        );
                    }
                    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                        (*L).top = ra.offset(1isize);
                        luaC_step(L);
                        (*L).top = (*ci).top;
                        base = (*ci).u.l.base
                    }
                    let ref mut fresh4 = *(*((L as *mut libc::c_char)
                        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                        as *mut libc::c_void
                        as *mut L_EXTRA))
                        .plock;
                    *fresh4 -= 1;
                    if *fresh4 == 0i32 {
                    } else {
                        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          875i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let ref mut fresh5 = *(*((L as *mut libc::c_char)
                        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                        as *mut libc::c_void
                        as *mut L_EXTRA))
                        .plock;
                    let fresh6 = *fresh5;
                    *fresh5 = *fresh5 + 1;
                    if fresh6 == 0i32 {
                    } else {
                        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          875i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    continue;
                }
                12 => {
                    let mut aux: *const TValue = 0 as *const TValue;
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgR as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgR\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          880i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_4: StkId = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          881i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_3: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    /* key must be a string */
                    if (*rc_3).tt_ & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((((rc))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            882i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                b"void luaV_execute(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*rc_3).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((rc)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            882i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                b"void luaV_execute(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    let mut key: *mut TString = &mut (*((*rc_3).value_.gc as *mut GCUnion)).ts;
                    let mut io1_6: *mut TValue = ra.offset(1isize);
                    *io1_6 = *rb_4;
                    if 0 == (*io1_6).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_6).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                883i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_6).tt_ & 0x3fi32 == (*(*io1_6).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_6).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        883i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_6).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                883i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    if 0 != if !((*rb_4).tt_ == 5i32 | 1i32 << 6i32) {
                        aux = 0 as *const TValue;
                        0i32
                    } else {
                        if (*rb_4).tt_ == 5i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                884i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*rb_4).value_.gc).tt as libc::c_int == 5i32 {
                        } else {
                            __assert_fail(
                                b"(((rb)->value_).gc)->tt == 5\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                884i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        aux = luaH_getstr(&mut (*((*rb_4).value_.gc as *mut GCUnion)).h, key);
                        !((*aux).tt_ == 0i32) as libc::c_int
                    } {
                        let mut io1_7: *mut TValue = ra;
                        *io1_7 = *aux;
                        if 0 == (*io1_7).tt_ & 1i32 << 6i32 || {
                            if 0 != (*io1_7).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(
                                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    885i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*io1_7).tt_ & 0x3fi32 == (*(*io1_7).value_.gc).tt as libc::c_int
                                && (L.is_null() || {
                                    if 0 != (*io1_7).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(
                                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                                as *const libc::c_char,
                                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                            885i32 as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 31],
                                                &[libc::c_char; 31],
                                            >(
                                                b"void luaV_execute(lua_State *)\x00"
                                            )).as_ptr(),
                                        );
                                    };
                                    0 != ((*(*io1_7).value_.gc).marked as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                                        & ((*(*L).l_G).currentwhite as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                })
                        } {
                        } else {
                            if 0 != 0i32 {
                            } else {
                                __assert_fail(
                                    b"0\x00" as *const u8 as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    885i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                        };
                        continue;
                    } else {
                        luaV_finishget(L, rb_4 as *const TValue, rc_3, ra, aux);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                13 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          891i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_5: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          892i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_4: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut nb: lua_Number = 0.;
                    let mut nc: lua_Number = 0.;
                    if (*rb_5).tt_ == 3i32 | 1i32 << 4i32 && (*rc_4).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_5).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                895i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ib: lua_Integer = (*rb_5).value_.i;
                        if (*rc_4).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                895i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ic: lua_Integer = (*rc_4).value_.i;
                        let mut io_1: *mut TValue = ra;
                        (*io_1).value_.i =
                            (ib as lua_Unsigned).wrapping_add(ic as lua_Unsigned) as lua_Integer;
                        (*io_1).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_5).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rb_5).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                898i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nb = (*rb_5).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_5, &mut nb)
                    } && 0 != if (*rc_4).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rc_4).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                898i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nc = (*rc_4).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_4, &mut nc)
                    } {
                        let mut io_2: *mut TValue = ra;
                        (*io_2).value_.n = nb + nc;
                        (*io_2).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_5, rc_4, ra, TM_ADD);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                14 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          905i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_6: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          906i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_5: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut nb_0: lua_Number = 0.;
                    let mut nc_0: lua_Number = 0.;
                    if (*rb_6).tt_ == 3i32 | 1i32 << 4i32 && (*rc_5).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_6).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                909i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ib_0: lua_Integer = (*rb_6).value_.i;
                        if (*rc_5).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                909i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ic_0: lua_Integer = (*rc_5).value_.i;
                        let mut io_3: *mut TValue = ra;
                        (*io_3).value_.i = (ib_0 as lua_Unsigned).wrapping_sub(ic_0 as lua_Unsigned)
                            as lua_Integer;
                        (*io_3).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_6).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rb_6).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                912i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nb_0 = (*rb_6).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_6, &mut nb_0)
                    } && 0 != if (*rc_5).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rc_5).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                912i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nc_0 = (*rc_5).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_5, &mut nc_0)
                    } {
                        let mut io_4: *mut TValue = ra;
                        (*io_4).value_.n = nb_0 - nc_0;
                        (*io_4).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_6, rc_5, ra, TM_SUB);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                15 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          919i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_7: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          920i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_6: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut nb_1: lua_Number = 0.;
                    let mut nc_1: lua_Number = 0.;
                    if (*rb_7).tt_ == 3i32 | 1i32 << 4i32 && (*rc_6).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_7).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                923i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ib_1: lua_Integer = (*rb_7).value_.i;
                        if (*rc_6).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                923i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ic_1: lua_Integer = (*rc_6).value_.i;
                        let mut io_5: *mut TValue = ra;
                        (*io_5).value_.i = (ib_1 as lua_Unsigned).wrapping_mul(ic_1 as lua_Unsigned)
                            as lua_Integer;
                        (*io_5).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_7).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rb_7).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                926i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nb_1 = (*rb_7).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_7, &mut nb_1)
                    } && 0 != if (*rc_6).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rc_6).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                926i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nc_1 = (*rc_6).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_6, &mut nc_1)
                    } {
                        let mut io_6: *mut TValue = ra;
                        (*io_6).value_.n = nb_1 * nc_1;
                        (*io_6).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_7, rc_6, ra, TM_MUL);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                18 => {
                    /* float division (always with floats) */
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          933i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_8: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          934i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_7: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut nb_2: lua_Number = 0.;
                    let mut nc_2: lua_Number = 0.;
                    if 0 != if (*rb_8).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rb_8).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                936i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nb_2 = (*rb_8).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_8, &mut nb_2)
                    } && 0 != if (*rc_7).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rc_7).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                936i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nc_2 = (*rc_7).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_7, &mut nc_2)
                    } {
                        let mut io_7: *mut TValue = ra;
                        (*io_7).value_.n = nb_2 / nc_2;
                        (*io_7).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_8, rc_7, ra, TM_DIV);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                20 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          943i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_9: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          944i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_8: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut ib_2: lua_Integer = 0;
                    let mut ic_2: lua_Integer = 0;
                    if 0 != if (*rb_9).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_9).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                946i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ib_2 = (*rb_9).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_9, &mut ib_2, 0i32)
                    } && 0 != if (*rc_8).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rc_8).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                946i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ic_2 = (*rc_8).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_8, &mut ic_2, 0i32)
                    } {
                        let mut io_8: *mut TValue = ra;
                        (*io_8).value_.i =
                            (ib_2 as lua_Unsigned & ic_2 as lua_Unsigned) as lua_Integer;
                        (*io_8).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_9, rc_8, ra, TM_BAND);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                21 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          953i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_10: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          954i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_9: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut ib_3: lua_Integer = 0;
                    let mut ic_3: lua_Integer = 0;
                    if 0 != if (*rb_10).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_10).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                956i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ib_3 = (*rb_10).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_10, &mut ib_3, 0i32)
                    } && 0 != if (*rc_9).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rc_9).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                956i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ic_3 = (*rc_9).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_9, &mut ic_3, 0i32)
                    } {
                        let mut io_9: *mut TValue = ra;
                        (*io_9).value_.i =
                            (ib_3 as lua_Unsigned | ic_3 as lua_Unsigned) as lua_Integer;
                        (*io_9).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_10, rc_9, ra, TM_BOR);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                22 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          963i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_11: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          964i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_10: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut ib_4: lua_Integer = 0;
                    let mut ic_4: lua_Integer = 0;
                    if 0 != if (*rb_11).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_11).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                966i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ib_4 = (*rb_11).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_11, &mut ib_4, 0i32)
                    } && 0 != if (*rc_10).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rc_10).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                966i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ic_4 = (*rc_10).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_10, &mut ic_4, 0i32)
                    } {
                        let mut io_10: *mut TValue = ra;
                        (*io_10).value_.i =
                            (ib_4 as lua_Unsigned ^ ic_4 as lua_Unsigned) as lua_Integer;
                        (*io_10).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_11, rc_10, ra, TM_BXOR);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                23 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          973i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_12: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          974i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_11: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut ib_5: lua_Integer = 0;
                    let mut ic_5: lua_Integer = 0;
                    if 0 != if (*rb_12).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_12).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                976i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ib_5 = (*rb_12).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_12, &mut ib_5, 0i32)
                    } && 0 != if (*rc_11).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rc_11).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                976i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ic_5 = (*rc_11).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_11, &mut ic_5, 0i32)
                    } {
                        let mut io_11: *mut TValue = ra;
                        (*io_11).value_.i = luaV_shiftl(ib_5, ic_5);
                        (*io_11).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_12, rc_11, ra, TM_SHL);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                24 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          983i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_13: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          984i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_12: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut ib_6: lua_Integer = 0;
                    let mut ic_6: lua_Integer = 0;
                    if 0 != if (*rb_13).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_13).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                986i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ib_6 = (*rb_13).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_13, &mut ib_6, 0i32)
                    } && 0 != if (*rc_12).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rc_12).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                986i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ic_6 = (*rc_12).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_12, &mut ic_6, 0i32)
                    } {
                        let mut io_12: *mut TValue = ra;
                        (*io_12).value_.i = luaV_shiftl(ib_6, -ic_6);
                        (*io_12).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_13, rc_12, ra, TM_SHR);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                16 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          993i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_14: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          994i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_13: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut nb_3: lua_Number = 0.;
                    let mut nc_3: lua_Number = 0.;
                    if (*rb_14).tt_ == 3i32 | 1i32 << 4i32 && (*rc_13).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_14).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                997i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ib_7: lua_Integer = (*rb_14).value_.i;
                        if (*rc_13).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                997i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ic_7: lua_Integer = (*rc_13).value_.i;
                        let mut io_13: *mut TValue = ra;
                        (*io_13).value_.i = luaV_mod(L, ib_7, ic_7);
                        (*io_13).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_14).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rb_14).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1000i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nb_3 = (*rb_14).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_14, &mut nb_3)
                    } && 0 != if (*rc_13).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rc_13).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1000i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nc_3 = (*rc_13).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_13, &mut nc_3)
                    } {
                        let mut m: lua_Number = 0.;
                        m = fmod(nb_3, nc_3);
                        if m * nc_3 < 0i32 as libc::c_double {
                            m += nc_3
                        }
                        let mut io_14: *mut TValue = ra;
                        (*io_14).value_.n = m;
                        (*io_14).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_14, rc_13, ra, TM_MOD);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                19 => {
                    /* floor division */
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1009i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_15: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1010i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_14: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut nb_4: lua_Number = 0.;
                    let mut nc_4: lua_Number = 0.;
                    if (*rb_15).tt_ == 3i32 | 1i32 << 4i32 && (*rc_14).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_15).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1013i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ib_8: lua_Integer = (*rb_15).value_.i;
                        if (*rc_14).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1013i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ic_8: lua_Integer = (*rc_14).value_.i;
                        let mut io_15: *mut TValue = ra;
                        (*io_15).value_.i = luaV_div(L, ib_8, ic_8);
                        (*io_15).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_15).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rb_15).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1016i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nb_4 = (*rb_15).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_15, &mut nb_4)
                    } && 0 != if (*rc_14).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rc_14).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1016i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nc_4 = (*rc_14).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_14, &mut nc_4)
                    } {
                        let mut io_16: *mut TValue = ra;
                        (*io_16).value_.n = floor(nb_4 / nc_4);
                        (*io_16).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_15, rc_14, ra, TM_IDIV);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                17 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1023i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_16: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1024i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_15: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    let mut nb_5: lua_Number = 0.;
                    let mut nc_5: lua_Number = 0.;
                    if 0 != if (*rb_16).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rb_16).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1026i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nb_5 = (*rb_16).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_16, &mut nb_5)
                    } && 0 != if (*rc_15).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rc_15).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rc))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1026i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nc_5 = (*rc_15).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_15, &mut nc_5)
                    } {
                        let mut io_17: *mut TValue = ra;
                        (*io_17).value_.n = pow(nb_5, nc_5);
                        (*io_17).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_16, rc_15, ra, TM_POW);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                25 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgR as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgR\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1033i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_17: *mut TValue = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    let mut nb_6: lua_Number = 0.;
                    if (*rb_17).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_17).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1036i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut ib_9: lua_Integer = (*rb_17).value_.i;
                        let mut io_18: *mut TValue = ra;
                        (*io_18).value_.i = (0i32 as lua_Unsigned)
                            .wrapping_sub(ib_9 as lua_Unsigned)
                            as lua_Integer;
                        (*io_18).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_17).tt_ == 3i32 | 0i32 << 4i32 {
                        if (*rb_17).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1039i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        nb_6 = (*rb_17).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_17, &mut nb_6)
                    } {
                        let mut io_19: *mut TValue = ra;
                        (*io_19).value_.n = -nb_6;
                        (*io_19).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_17, rb_17, ra, TM_UNM);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                26 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgR as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgR\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1048i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_18: *mut TValue = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    let mut ib_10: lua_Integer = 0;
                    if 0 != if (*rb_18).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*rb_18).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1050i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        ib_10 = (*rb_18).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_18, &mut ib_10, 0i32)
                    } {
                        let mut io_20: *mut TValue = ra;
                        (*io_20).value_.i =
                            (!(0i32 as lua_Unsigned) ^ ib_10 as lua_Unsigned) as lua_Integer;
                        (*io_20).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_18, rb_18, ra, TM_BNOT);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                27 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgR as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgR\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1059i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_19: *mut TValue = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    /* next assignment may change this value */
                    let mut res: libc::c_int = ((*rb_19).tt_ == 0i32 || (*rb_19).tt_ == 1i32 && {
                        if (*rb_19).tt_ == 1i32 {
                        } else {
                            __assert_fail(
                                b"((((rb))->tt_) == (1))\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1060i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*rb_19).value_.b == 0i32
                    }) as libc::c_int;
                    let mut io_21: *mut TValue = ra;
                    (*io_21).value_.b = res;
                    (*io_21).tt_ = 1i32;
                    continue;
                }
                28 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgR as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgR\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1065i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    luaV_objlen(
                        L,
                        ra,
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        ) as *const TValue,
                    );
                    base = (*ci).u.l.base;
                    continue;
                }
                29 => {
                    let mut b_2: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    let mut c_0: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    let mut rb_20: StkId = 0 as *mut TValue;
                    /* mark the end of concat operands */
                    (*L).top = base.offset(c_0 as isize).offset(1isize);
                    luaV_concat(L, c_0 - b_2 + 1i32);
                    base = (*ci).u.l.base;
                    /* 'luaV_concat' may invoke TMs and move the stack */
                    ra = base.offset(
                        (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    rb_20 = base.offset(b_2 as isize);
                    let mut io1_8: *mut TValue = ra;
                    *io1_8 = *rb_20;
                    if 0 == (*io1_8).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_8).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1076i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_8).tt_ & 0x3fi32 == (*(*io1_8).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_8).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        1076i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_8).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1076i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                        (*L).top = if ra >= rb_20 {
                            ra.offset(1isize)
                        } else {
                            rb_20
                        };
                        luaC_step(L);
                        (*L).top = (*ci).top;
                        base = (*ci).u.l.base
                    }
                    let ref mut fresh7 = *(*((L as *mut libc::c_char)
                        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                        as *mut libc::c_void
                        as *mut L_EXTRA))
                        .plock;
                    *fresh7 -= 1;
                    if *fresh7 == 0i32 {
                    } else {
                        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1077i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let ref mut fresh8 = *(*((L as *mut libc::c_char)
                        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                        as *mut libc::c_void
                        as *mut L_EXTRA))
                        .plock;
                    let fresh9 = *fresh8;
                    *fresh8 = *fresh8 + 1;
                    if fresh9 == 0i32 {
                    } else {
                        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1077i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    /* restore top */
                    (*L).top = (*ci).top;
                    continue;
                }
                30 => {
                    let mut a: libc::c_int = (i >> 0i32 + 6i32
                        & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                        as libc::c_int;
                    if a != 0i32 {
                        luaF_close(L, (*ci).u.l.base.offset(a as isize).offset(-1isize));
                    }
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                        ((i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                            as libc::c_int
                            - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                            + 0i32) as isize,
                    );
                    continue;
                }
                31 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1086i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_21: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1087i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rc_16: *mut TValue = if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                        & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int as isize,
                        )
                    };
                    if luaV_equalobj(L, rb_21, rc_16)
                        != (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int
                    {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize)
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_0: libc::c_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int;
                        if a_0 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_0 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        )
                    }
                    base = (*ci).u.l.base;
                    continue;
                }
                32 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1098i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1098i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    if luaV_lessthan(
                        L,
                        if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int & 1i32 << 9i32 - 1i32
                        {
                            k.offset(
                                ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as libc::c_int
                                    & !(1i32 << 9i32 - 1i32))
                                    as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0i32 + 6i32 + 8i32 + 9i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as libc::c_int as isize,
                            )
                        },
                        if 0 != (i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int & 1i32 << 9i32 - 1i32
                        {
                            k.offset(
                                ((i >> 0i32 + 6i32 + 8i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as libc::c_int
                                    & !(1i32 << 9i32 - 1i32))
                                    as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0i32 + 6i32 + 8i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as libc::c_int as isize,
                            )
                        },
                    )
                        != (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int
                    {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize)
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_1: libc::c_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int;
                        if a_1 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_1 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        )
                    }
                    base = (*ci).u.l.base;
                    continue;
                }
                33 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1107i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 2i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgK as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 2) & 3))) == OpArgK\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1107i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    if luaV_lessequal(
                        L,
                        if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int & 1i32 << 9i32 - 1i32
                        {
                            k.offset(
                                ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as libc::c_int
                                    & !(1i32 << 9i32 - 1i32))
                                    as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0i32 + 6i32 + 8i32 + 9i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as libc::c_int as isize,
                            )
                        },
                        if 0 != (i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int & 1i32 << 9i32 - 1i32
                        {
                            k.offset(
                                ((i >> 0i32 + 6i32 + 8i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as libc::c_int
                                    & !(1i32 << 9i32 - 1i32))
                                    as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0i32 + 6i32 + 8i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as libc::c_int as isize,
                            )
                        },
                    )
                        != (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int
                    {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize)
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_2: libc::c_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int;
                        if a_2 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_2 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        )
                    }
                    base = (*ci).u.l.base;
                    continue;
                }
                34 => {
                    if 0 != if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                    {
                        ((*ra).tt_ == 0i32 || (*ra).tt_ == 1i32 && {
                            if (*ra).tt_ == 1i32 {
                            } else {
                                __assert_fail(
                                    b"((((ra))->tt_) == (1))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1115i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*ra).value_.b == 0i32
                        }) as libc::c_int
                    } else {
                        !((*ra).tt_ == 0i32 || (*ra).tt_ == 1i32 && {
                            if (*ra).tt_ == 1i32 {
                            } else {
                                __assert_fail(
                                    b"((((ra))->tt_) == (1))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1115i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*ra).value_.b == 0i32
                        }) as libc::c_int
                    } {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize);
                        continue;
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_3: libc::c_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int;
                        if a_3 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_3 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        );
                        continue;
                    }
                }
                35 => {
                    if (luaP_opmodes[(i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                         as OpCode as usize] as libc::c_int
                        >> 4i32 & 3i32) as OpArgMask as libc::c_uint
                        == OpArgR as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] >> 4) & 3))) == OpArgR\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1122i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let mut rb_22: *mut TValue = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    if 0 != if 0 != (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int
                    {
                        ((*rb_22).tt_ == 0i32 || (*rb_22).tt_ == 1i32 && {
                            if (*rb_22).tt_ == 1i32 {
                            } else {
                                __assert_fail(
                                    b"((((rb))->tt_) == (1))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1123i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*rb_22).value_.b == 0i32
                        }) as libc::c_int
                    } else {
                        !((*rb_22).tt_ == 0i32 || (*rb_22).tt_ == 1i32 && {
                            if (*rb_22).tt_ == 1i32 {
                            } else {
                                __assert_fail(
                                    b"((((rb))->tt_) == (1))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1123i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*rb_22).value_.b == 0i32
                        }) as libc::c_int
                    } {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize);
                        continue;
                    } else {
                        let mut io1_9: *mut TValue = ra;
                        *io1_9 = *rb_22;
                        if 0 == (*io1_9).tt_ & 1i32 << 6i32 || {
                            if 0 != (*io1_9).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(
                                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1126i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*io1_9).tt_ & 0x3fi32 == (*(*io1_9).value_.gc).tt as libc::c_int
                                && (L.is_null() || {
                                    if 0 != (*io1_9).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(
                                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                                as *const libc::c_char,
                                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                            1126i32 as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 31],
                                                &[libc::c_char; 31],
                                            >(
                                                b"void luaV_execute(lua_State *)\x00"
                                            )).as_ptr(),
                                        );
                                    };
                                    0 != ((*(*io1_9).value_.gc).marked as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                                        & ((*(*L).l_G).currentwhite as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                })
                        } {
                        } else {
                            if 0 != 0i32 {
                            } else {
                                __assert_fail(
                                    b"0\x00" as *const u8 as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1126i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                        };
                        i = *(*ci).u.l.savedpc;
                        let mut a_4: libc::c_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int;
                        if a_4 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_4 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        );
                        continue;
                    }
                }
                36 => {
                    let mut b_3: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    let mut nresults: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int - 1i32;
                    if b_3 != 0i32 {
                        /* else previous instruction set top */
                        (*L).top = ra.offset(b_3 as isize)
                    }
                    if 0 != luaD_precall(L, ra, nresults) {
                        /* C function? */
                        if nresults >= 0i32 {
                            /* adjust results */
                            (*L).top = (*ci).top
                        }
                        base = (*ci).u.l.base;
                        /* update 'base' */
                        continue;
                    } else {
                        /* Lua function */
                        ci = (*L).ci;
                        /* restart luaV_execute over new Lua function */
                        break;
                    }
                }
                37 => {
                    let mut b_4: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    if b_4 != 0i32 {
                        /* else previous instruction set top */
                        (*L).top = ra.offset(b_4 as isize)
                    }
                    if (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int - 1i32 == -1i32
                    {
                    } else {
                        __assert_fail(b"(((int)(((i)>>((0 + 6) + 8)) & ((~((~(Instruction)0)<<(9)))<<(0))))) - 1 == (-1)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1149i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    if 0 != luaD_precall(L, ra, -1i32) {
                        /* C function? */
                        base = (*ci).u.l.base;
                        /* update 'base' */
                        continue;
                    } else {
                        /* tail call: put called frame (n) in place of caller one (o) */
                        /* called frame */
                        let mut nci: *mut CallInfo_0 = (*L).ci;
                        /* caller frame */
                        let mut oci: *mut CallInfo_0 = (*nci).previous;
                        /* called function */
                        let mut nfunc: StkId = (*nci).func;
                        /* caller function */
                        let mut ofunc: StkId = (*oci).func;
                        /* last stack slot filled by 'precall' */
                        if (*nfunc).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((nfunc))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1160i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*nfunc).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"(((nfunc)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1160i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut lim: StkId = (*nci).u.l.base.offset(
                            (*(*&mut (*((*nfunc).value_.gc as *mut GCUnion)).cl.l).p).numparams
                                as libc::c_int as isize,
                        );
                        let mut aux_0: libc::c_int = 0;
                        /* close all upvalues from previous call */
                        if (*(*cl).p).sizep > 0i32 {
                            luaF_close(L, (*oci).u.l.base);
                        }
                        /* move new frame into old one */
                        aux_0 = 0i32;
                        while nfunc.offset(aux_0 as isize) < lim {
                            let mut io1_10: *mut TValue = ofunc.offset(aux_0 as isize);
                            *io1_10 = *nfunc.offset(aux_0 as isize);
                            if 0 == (*io1_10).tt_ & 1i32 << 6i32 || {
                                if 0 != (*io1_10).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        1166i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                (*io1_10).tt_ & 0x3fi32 == (*(*io1_10).value_.gc).tt as libc::c_int
                                    && (L.is_null() || {
                                        if 0 != (*io1_10).tt_ & 1i32 << 6i32 {
                                        } else {
                                            __assert_fail(
                                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                                1166i32 as libc::c_uint,
                                                (*::std::mem::transmute::<
                                                    &[u8; 31],
                                                    &[libc::c_char; 31],
                                                >(
                                                    b"void luaV_execute(lua_State *)\x00"
                                                )).as_ptr(),
                                            );
                                        };
                                        0 != ((*(*io1_10).value_.gc).marked as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                            & ((*(*L).l_G).currentwhite as libc::c_int
                                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    })
                            } {
                            } else {
                                if 0 != 0i32 {
                                } else {
                                    __assert_fail(
                                        b"0\x00" as *const u8 as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        1166i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                            };
                            aux_0 += 1
                        }
                        /* correct base */
                        (*oci).u.l.base = ofunc
                            .offset((*nci).u.l.base.wrapping_offset_from(nfunc) as libc::c_long
                                as isize);
                        /* correct top */
                        (*L).top = ofunc
                            .offset((*L).top.wrapping_offset_from(nfunc) as libc::c_long as isize);
                        (*oci).top = (*L).top;
                        (*oci).u.l.savedpc = (*nci).u.l.savedpc;
                        /* function was tail called */
                        (*oci).callstatus =
                            ((*oci).callstatus as libc::c_int | 1i32 << 5i32) as libc::c_ushort;
                        /* remove new frame */
                        (*L).ci = oci;
                        ci = (*L).ci;
                        if (*ofunc).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((ofunc))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1172i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*ofunc).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"(((ofunc)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1172i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*L).top
                            == (*oci).u.l.base.offset(
                                (*(*(&mut (*((*ofunc).value_.gc as *mut GCUnion)).cl.l
                                    as *mut LClosure_0))
                                    .p)
                                    .maxstacksize as libc::c_int
                                    as isize,
                            ) {
                        } else {
                            __assert_fail(b"L->top == oci->u.l.base + ((((((((ofunc))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))) ? (void) (0) : __assert_fail (\"((((ofunc))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\", \"lvm.c\", 1172, __extension__ __PRETTY_FUNCTION__)), (((((((ofunc)->value_).gc)->tt == (6 | (0 << 4))) ? (void) (0) : __assert_fail (\"(((ofunc)->value_).gc)->tt == (6 | (0 << 4))\", \"lvm.c\", 1172, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((ofunc)->value_).gc))))->cl.l)))))->p)->maxstacksize\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lvm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              1172i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 31],
                                                                        &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                        };
                        /* restart luaV_execute over new Lua function */
                        break;
                    }
                }
                38 => {
                    let mut b_5: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    if (*(*cl).p).sizep > 0i32 {
                        luaF_close(L, base);
                    }
                    b_5 = luaD_poscall(
                        L,
                        ci,
                        ra,
                        if b_5 != 0i32 {
                            b_5 - 1i32
                        } else {
                            (*L).top.wrapping_offset_from(ra) as libc::c_long as libc::c_int
                        },
                    );
                    /* local 'ci' still from callee */
                    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 3i32 {
                        /* external invocation: return */
                        return;
                    } else {
                        /* invocation via reentry: continue execution */
                        ci = (*L).ci;
                        if 0 != b_5 {
                            (*L).top = (*ci).top
                        }
                        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
                        } else {
                            __assert_fail(
                                b"((ci)->callstatus & (1<<1))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1186i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*ci).u.l.savedpc.offset(-1isize) >> 0i32
                            & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                            as OpCode as libc::c_uint
                            == OP_CALL as libc::c_int as libc::c_uint
                        {
                        } else {
                            __assert_fail(b"(((OpCode)(((*((ci)->u.l.savedpc - 1))>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) == OP_CALL\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lvm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              1187i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 31],
                                                                        &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                        };
                        /* restart luaV_execute over new Lua function */
                        break;
                    }
                }
                39 => {
                    if (*ra).tt_ == 3i32 | 1i32 << 4i32 {
                        /* integer loop? */
                        if (*ra.offset(2isize)).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((ra + 2))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1193i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut step: lua_Integer = (*ra.offset(2isize)).value_.i;
                        /* increment index */
                        if (*ra).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((ra))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1194i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut idx: lua_Integer = ((*ra).value_.i as lua_Unsigned)
                            .wrapping_add(step as lua_Unsigned)
                            as lua_Integer;
                        if (*ra.offset(1isize)).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((ra + 1))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1195i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut limit: lua_Integer = (*ra.offset(1isize)).value_.i;
                        if !(0 != if (0i32 as libc::c_longlong) < step {
                            (idx <= limit) as libc::c_int
                        } else {
                            (limit <= idx) as libc::c_int
                        }) {
                            continue;
                        }
                        /* jump back */
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32))
                                as isize,
                        );
                        let mut io_22: *mut TValue = ra;
                        if (*io_22).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((io))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1198i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io_22).value_.i = idx;
                        /* update internal index... */
                        let mut io_23: *mut TValue = ra.offset(3isize);
                        (*io_23).value_.i = idx;
                        (*io_23).tt_ = 3i32 | 1i32 << 4i32;
                        /* ...and external index */
                        continue;
                    } else {
                        /* floating loop */
                        if (*ra.offset(2isize)).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((ra + 2))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1203i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut step_0: lua_Number = (*ra.offset(2isize)).value_.n;
                        /* inc. index */
                        if (*ra).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((ra))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1204i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut idx_0: lua_Number = (*ra).value_.n + step_0;
                        if (*ra.offset(1isize)).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((ra + 1))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1205i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        let mut limit_0: lua_Number = (*ra.offset(1isize)).value_.n;
                        if !(0 != if (0i32 as libc::c_double) < step_0 {
                            (idx_0 <= limit_0) as libc::c_int
                        } else {
                            (limit_0 <= idx_0) as libc::c_int
                        }) {
                            continue;
                        }
                        /* jump back */
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32))
                                as isize,
                        );
                        let mut io_24: *mut TValue = ra;
                        if (*io_24).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((io))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1209i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io_24).value_.n = idx_0;
                        /* update internal index... */
                        let mut io_25: *mut TValue = ra.offset(3isize);
                        (*io_25).value_.n = idx_0;
                        (*io_25).tt_ = 3i32 | 0i32 << 4i32;
                        /* ...and external index */
                        continue;
                    }
                }
                40 => {
                    let mut init: *mut TValue = ra;
                    let mut plimit: *mut TValue = ra.offset(1isize);
                    let mut pstep: *mut TValue = ra.offset(2isize);
                    let mut ilimit: lua_Integer = 0;
                    let mut stopnow: libc::c_int = 0;
                    if (*init).tt_ == 3i32 | 1i32 << 4i32 && (*pstep).tt_ == 3i32 | 1i32 << 4i32
                        && {
                            if (*pstep).tt_ == 3i32 | 1i32 << 4i32 {
                            } else {
                                __assert_fail(
                                    b"((((pstep))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1222i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            0 != forlimit(plimit, &mut ilimit, (*pstep).value_.i, &mut stopnow)
                        } {
                        /* all values are integer */
                        let mut initv: lua_Integer = if 0 != stopnow {
                            0i32 as libc::c_longlong
                        } else {
                            if (*init).tt_ == 3i32 | 1i32 << 4i32 {
                            } else {
                                __assert_fail(
                                    b"((((init))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1224i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*init).value_.i
                        };
                        let mut io_26: *mut TValue = plimit;
                        (*io_26).value_.i = ilimit;
                        (*io_26).tt_ = 3i32 | 1i32 << 4i32;
                        let mut io_27: *mut TValue = init;
                        if (*pstep).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(
                                b"((((pstep))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1226i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io_27).value_.i = (initv as lua_Unsigned)
                            .wrapping_sub((*pstep).value_.i as lua_Unsigned)
                            as lua_Integer;
                        (*io_27).tt_ = 3i32 | 1i32 << 4i32
                    } else {
                        /* try making all values floats */
                        let mut ninit: lua_Number = 0.;
                        let mut nlimit: lua_Number = 0.;
                        let mut nstep: lua_Number = 0.;
                        if 0 == if (*plimit).tt_ == 3i32 | 0i32 << 4i32 {
                            if (*plimit).tt_ == 3i32 | 0i32 << 4i32 {
                            } else {
                                __assert_fail(
                                    b"((((plimit))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1230i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            nlimit = (*plimit).value_.n;
                            1i32
                        } else {
                            luaV_tonumber_(plimit, &mut nlimit)
                        } {
                            luaG_runerror(
                                L,
                                b"\'for\' limit must be a number\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            let mut io_28: *mut TValue = plimit;
                            (*io_28).value_.n = nlimit;
                            (*io_28).tt_ = 3i32 | 0i32 << 4i32;
                            if 0 == if (*pstep).tt_ == 3i32 | 0i32 << 4i32 {
                                if (*pstep).tt_ == 3i32 | 0i32 << 4i32 {
                                } else {
                                    __assert_fail(
                                        b"((((pstep))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        1233i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                nstep = (*pstep).value_.n;
                                1i32
                            } else {
                                luaV_tonumber_(pstep, &mut nstep)
                            } {
                                luaG_runerror(
                                    L,
                                    b"\'for\' step must be a number\x00" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                let mut io_29: *mut TValue = pstep;
                                (*io_29).value_.n = nstep;
                                (*io_29).tt_ = 3i32 | 0i32 << 4i32;
                                if 0 == if (*init).tt_ == 3i32 | 0i32 << 4i32 {
                                    if (*init).tt_ == 3i32 | 0i32 << 4i32 {
                                    } else {
                                        __assert_fail(
                                            b"((((init))->tt_) == ((3 | (0 << 4))))\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                            1236i32 as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 31],
                                                &[libc::c_char; 31],
                                            >(
                                                b"void luaV_execute(lua_State *)\x00"
                                            )).as_ptr(),
                                        );
                                    };
                                    ninit = (*init).value_.n;
                                    1i32
                                } else {
                                    luaV_tonumber_(init, &mut ninit)
                                } {
                                    luaG_runerror(
                                        L,
                                        b"\'for\' initial value must be a number\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    let mut io_30: *mut TValue = init;
                                    (*io_30).value_.n = ninit - nstep;
                                    (*io_30).tt_ = 3i32 | 0i32 << 4i32
                                }
                            }
                        }
                    }
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                        ((i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                            as libc::c_int
                            - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32))
                            as isize,
                    );
                    continue;
                }
                41 => {
                    /* call base */
                    let mut cb: StkId = ra.offset(3isize);
                    let mut io1_11: *mut TValue = cb.offset(2isize);
                    *io1_11 = *ra.offset(2isize);
                    if 0 == (*io1_11).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_11).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1245i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_11).tt_ & 0x3fi32 == (*(*io1_11).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_11).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        1245i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_11).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1245i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    let mut io1_12: *mut TValue = cb.offset(1isize);
                    *io1_12 = *ra.offset(1isize);
                    if 0 == (*io1_12).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_12).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1246i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_12).tt_ & 0x3fi32 == (*(*io1_12).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_12).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        1246i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_12).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1246i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    let mut io1_13: *mut TValue = cb;
                    *io1_13 = *ra;
                    if 0 == (*io1_13).tt_ & 1i32 << 6i32 || {
                        if 0 != (*io1_13).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1247i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io1_13).tt_ & 0x3fi32 == (*(*io1_13).value_.gc).tt as libc::c_int
                            && (L.is_null() || {
                                if 0 != (*io1_13).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        1247i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != ((*(*io1_13).value_.gc).marked as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                                    & ((*(*L).l_G).currentwhite as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                            })
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1247i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    /* func. + 2 args (state and index) */
                    (*L).top = cb.offset(3isize);
                    luaD_call(
                        L,
                        cb,
                        (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int,
                    );
                    base = (*ci).u.l.base;
                    (*L).top = (*ci).top;
                    /* go to next instruction */
                    let fresh10 = (*ci).u.l.savedpc;
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1);
                    i = *fresh10;
                    ra = base.offset(
                        (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    if (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
                        as libc::c_uint
                        == OP_TFORLOOP as libc::c_int as libc::c_uint
                    {
                    } else {
                        __assert_fail(b"(((OpCode)(((i)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) == OP_TFORLOOP\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1253i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                }
                42 => {}
                43 => {
                    let mut n: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    let mut c_1: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    let mut last: libc::c_uint = 0;
                    let mut h: *mut Table = 0 as *mut Table;
                    if n == 0i32 {
                        n = (*L).top.wrapping_offset_from(ra) as libc::c_long as libc::c_int - 1i32
                    }
                    if c_1 == 0i32 {
                        if (*(*ci).u.l.savedpc >> 0i32
                            & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                            as OpCode as libc::c_uint
                            == OP_EXTRAARG as libc::c_int as libc::c_uint
                        {
                        } else {
                            __assert_fail(b"(((OpCode)(((*ci->u.l.savedpc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) == OP_EXTRAARG\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lvm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              1271i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 31],
                                                                        &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                        };
                        let fresh11 = (*ci).u.l.savedpc;
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1);
                        c_1 = (*fresh11 >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
                            as libc::c_int
                    }
                    if (*ra).tt_ == 5i32 | 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"((((ra))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                                as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            1274i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                b"void luaV_execute(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*ra).value_.gc).tt as libc::c_int == 5i32 {
                    } else {
                        __assert_fail(
                            b"(((ra)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            1274i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                b"void luaV_execute(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    h = &mut (*((*ra).value_.gc as *mut GCUnion)).h;
                    last = ((c_1 - 1i32) * 50i32 + n) as libc::c_uint;
                    /* needs more space? */
                    if last > (*h).sizearray {
                        /* preallocate it at once */
                        luaH_resizearray(L, h, last);
                    }
                    while n > 0i32 {
                        let mut val: *mut TValue = ra.offset(n as isize);
                        let fresh12 = last;
                        last = last.wrapping_sub(1);
                        luaH_setint(L, h, fresh12 as lua_Integer, val);
                        if 0 != (*val).tt_ & 1i32 << 6i32
                            && 0 != (*h).marked as libc::c_int & 1i32 << 2i32
                            && {
                                if 0 != (*val).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(
                                        b"(((val)->tt_) & (1 << 6))\x00" as *const u8
                                            as *const libc::c_char,
                                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                        1281i32 as libc::c_uint,
                                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                            b"void luaV_execute(lua_State *)\x00",
                                        )).as_ptr(),
                                    );
                                };
                                0 != (*(*val).value_.gc).marked as libc::c_int
                                    & (1i32 << 0i32 | 1i32 << 1i32)
                            } {
                            luaC_barrierback_(L, h);
                        } else {
                        };
                        n -= 1
                    }
                    /* correct top (in case of previous open call) */
                    (*L).top = (*ci).top;
                    continue;
                }
                44 => {
                    let mut p: *mut Proto_0 = *(*(*cl).p).p.offset(
                        (i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                            as libc::c_int as isize,
                    );
                    /* cached closure */
                    let mut ncl: *mut LClosure_0 = getcached(p, (*cl).upvals.as_mut_ptr(), base);
                    /* no match? */
                    if ncl.is_null() {
                        /* create a new one */
                        pushclosure(L, p, (*cl).upvals.as_mut_ptr(), base, ra);
                    } else {
                        io_31 = ra;
                        x__0 = ncl;
                        if (*x__0).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                        } else {
                            __assert_fail(
                                b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1292i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*io_31).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
                        (*io_31).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32;
                        if 0 == (*io_31).tt_ & 1i32 << 6i32 || {
                            if 0 != (*io_31).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(
                                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1292i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*io_31).tt_ & 0x3fi32 == (*(*io_31).value_.gc).tt as libc::c_int
                                && (L.is_null() || {
                                    if 0 != (*io_31).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(
                                            b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                                as *const libc::c_char,
                                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                            1292i32 as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 31],
                                                &[libc::c_char; 31],
                                            >(
                                                b"void luaV_execute(lua_State *)\x00"
                                            )).as_ptr(),
                                        );
                                    };
                                    0 != ((*(*io_31).value_.gc).marked as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                                        & ((*(*L).l_G).currentwhite as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                })
                        } {
                        } else {
                            if 0 != 0i32 {
                            } else {
                                __assert_fail(
                                    b"0\x00" as *const u8 as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1292i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                        };
                    }
                    /* push cashed closure */
                    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                        (*L).top = ra.offset(1isize);
                        luaC_step(L);
                        (*L).top = (*ci).top;
                        base = (*ci).u.l.base
                    }
                    let ref mut fresh13 = *(*((L as *mut libc::c_char)
                        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                        as *mut libc::c_void
                        as *mut L_EXTRA))
                        .plock;
                    *fresh13 -= 1;
                    if *fresh13 == 0i32 {
                    } else {
                        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1293i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    let ref mut fresh14 = *(*((L as *mut libc::c_char)
                        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                        as *mut libc::c_void
                        as *mut L_EXTRA))
                        .plock;
                    let fresh15 = *fresh14;
                    *fresh14 = *fresh14 + 1;
                    if fresh15 == 0i32 {
                    } else {
                        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lvm.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          1293i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 31],
                                                                    &[libc::c_char; 31]>(b"void luaV_execute(lua_State *)\x00")).as_ptr());
                    };
                    continue;
                }
                45 => {
                    /* required results */
                    let mut b_6: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int - 1i32;
                    let mut j: libc::c_int = 0;
                    let mut n_0: libc::c_int = base.wrapping_offset_from((*ci).func) as libc::c_long
                        as libc::c_int
                        - (*(*cl).p).numparams as libc::c_int
                        - 1i32;
                    /* less arguments than parameters? */
                    if n_0 < 0i32 {
                        /* no vararg arguments */
                        n_0 = 0i32
                    }
                    if b_6 < 0i32 {
                        /* B == 0? */
                        /* get all var. arguments */
                        b_6 = n_0;
                        if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long
                            <= n_0 as libc::c_long
                        {
                            luaD_growstack(L, n_0);
                        }
                        base = (*ci).u.l.base;
                        /* previous call may change the stack */
                        ra = base.offset(
                            (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                                as libc::c_int as isize,
                        );
                        (*L).top = ra.offset(n_0 as isize)
                    }
                    j = 0i32;
                    while j < b_6 && j < n_0 {
                        let mut io1_15: *mut TValue = ra.offset(j as isize);
                        *io1_15 = *base.offset(-(n_0 as isize)).offset(j as isize);
                        if 0 == (*io1_15).tt_ & 1i32 << 6i32 || {
                            if 0 != (*io1_15).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(
                                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1309i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            (*io1_15).tt_ & 0x3fi32 == (*(*io1_15).value_.gc).tt as libc::c_int
                                && (L.is_null() || {
                                    if 0 != (*io1_15).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(
                                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                                as *const libc::c_char,
                                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                            1309i32 as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 31],
                                                &[libc::c_char; 31],
                                            >(
                                                b"void luaV_execute(lua_State *)\x00"
                                            )).as_ptr(),
                                        );
                                    };
                                    0 != ((*(*io1_15).value_.gc).marked as libc::c_int
                                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                                        & ((*(*L).l_G).currentwhite as libc::c_int
                                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                                })
                        } {
                        } else {
                            if 0 != 0i32 {
                            } else {
                                __assert_fail(
                                    b"0\x00" as *const u8 as *const libc::c_char,
                                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                    1309i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"void luaV_execute(lua_State *)\x00",
                                    )).as_ptr(),
                                );
                            };
                        };
                        j += 1
                    }
                    /* complete required results with nil */
                    loop {
                        if !(j < b_6) {
                            continue 's_17;
                        }
                        (*ra.offset(j as isize)).tt_ = 0i32;
                        j += 1
                    }
                }
                46 => {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(
                            b"0\x00" as *const u8 as *const libc::c_char,
                            b"lvm.c\x00" as *const u8 as *const libc::c_char,
                            1315i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                b"void luaV_execute(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    continue;
                }
                _ => {
                    continue;
                }
            }
            if (*ra.offset(1isize)).tt_ == 0i32 {
                continue;
            }
            /* continue loop? */
            let mut io1_14: *mut TValue = ra;
            *io1_14 = *ra.offset(1isize);
            if 0 == (*io1_14).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1_14).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        1259i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                            b"void luaV_execute(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                (*io1_14).tt_ & 0x3fi32 == (*(*io1_14).value_.gc).tt as libc::c_int
                    && (L.is_null() || {
                        if 0 != (*io1_14).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                                1259i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"void luaV_execute(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        0 != ((*(*io1_14).value_.gc).marked as libc::c_int
                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                            & ((*(*L).l_G).currentwhite as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                    })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lvm.c\x00" as *const u8 as *const libc::c_char,
                        1259i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                            b"void luaV_execute(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            /* save control variable */
            /* jump back */
            (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                    as libc::c_int - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32))
                    as isize,
            )
        }
    }
}
/*
** check whether cached closure in prototype 'p' may be reused, that is,
** whether there is a cached closure with the same upvalues needed by
** new closure to be created.
*/
unsafe extern "C" fn getcached(
    mut p: *mut Proto_0,
    mut encup: *mut *mut UpVal,
    mut base: StkId,
) -> *mut LClosure_0 {
    let mut c: *mut LClosure_0 = (*p).cache;
    if !c.is_null() {
        /* is there a cached closure? */
        let mut nup: libc::c_int = (*p).sizeupvalues;
        let mut uv: *mut Upvaldesc_0 = (*p).upvalues;
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < nup {
            /* check whether it has right upvalues */
            let mut v: *mut TValue = if 0 != (*uv.offset(i as isize)).instack as libc::c_int {
                base.offset((*uv.offset(i as isize)).idx as libc::c_int as isize)
            } else {
                (**encup.offset((*uv.offset(i as isize)).idx as isize)).v
            };
            if (**(*c).upvals.as_mut_ptr().offset(i as isize)).v != v {
                /* wrong upvalue; cannot reuse closure */
                return 0 as *mut LClosure_0;
            } else {
                i += 1
            }
        }
    }
    /* return cached closure (or NULL if no cached closure) */
    return c;
}
/*
** create a new Lua closure, push it in the stack, and initialize
** its upvalues. Note that the closure is not cached if prototype is
** already black (which means that 'cache' was already cleared by the
** GC).
*/
unsafe extern "C" fn pushclosure(
    mut L: *mut lua_State_0,
    mut p: *mut Proto_0,
    mut encup: *mut *mut UpVal,
    mut base: StkId,
    mut ra: StkId,
) -> () {
    let mut nup: libc::c_int = (*p).sizeupvalues;
    let mut uv: *mut Upvaldesc_0 = (*p).upvalues;
    let mut i: libc::c_int = 0;
    let mut ncl: *mut LClosure_0 = luaF_newLclosure(L, nup);
    (*ncl).p = p;
    let mut io: *mut TValue = ra;
    let mut x_: *mut LClosure_0 = ncl;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lvm.c\x00" as *const u8 as *const libc::c_char,
            640i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"void pushclosure(lua_State *, Proto *, UpVal **, StkId, StkId)\x00",
            )).as_ptr(),
        );
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 || {
        if 0 != (*io).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                640i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"void pushclosure(lua_State *, Proto *, UpVal **, StkId, StkId)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    640i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                        b"void pushclosure(lua_State *, Proto *, UpVal **, StkId, StkId)\x00",
                    )).as_ptr(),
                );
            };
            0 != ((*(*io).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"lvm.c\x00" as *const u8 as *const libc::c_char,
                640i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"void pushclosure(lua_State *, Proto *, UpVal **, StkId, StkId)\x00",
                )).as_ptr(),
            );
        };
    };
    /* anchor new closure in stack */
    i = 0i32;
    while i < nup {
        /* fill in its upvalues */
        /* upvalue refers to local variable? */
        if 0 != (*uv.offset(i as isize)).instack {
            let ref mut fresh16 = *(*ncl).upvals.as_mut_ptr().offset(i as isize);
            *fresh16 = luaF_findupval(
                L,
                base.offset((*uv.offset(i as isize)).idx as libc::c_int as isize),
            )
        } else {
            /* get upvalue from enclosing function */
            let ref mut fresh17 = *(*ncl).upvals.as_mut_ptr().offset(i as isize);
            *fresh17 = *encup.offset((*uv.offset(i as isize)).idx as isize)
        }
        let ref mut fresh18 = (**(*ncl).upvals.as_mut_ptr().offset(i as isize)).refcount;
        *fresh18 = (*fresh18).wrapping_add(1);
        i += 1
    }
    /* new closure is white, so we do not need a barrier here */
    /* cache will not break GC invariant? */
    if 0 == (*p).marked as libc::c_int & 1i32 << 2i32 {
        /* save it on cache for reuse */
        (*p).cache = ncl
    };
}
/*
** Try to convert a 'for' limit to an integer, preserving the
** semantics of the loop.
** (The following explanation assumes a non-negative step; it is valid
** for negative steps mutatis mutandis.)
** If the limit can be converted to an integer, rounding down, that is
** it.
** Otherwise, check whether the limit can be converted to a number.  If
** the number is too large, it is OK to set the limit as LUA_MAXINTEGER,
** which means no limit.  If the number is too negative, the loop
** should not run, because any initial integer value is larger than the
** limit. So, it sets the limit to LUA_MININTEGER. 'stopnow' corrects
** the extreme case when the initial value is LUA_MININTEGER, in which
** case the LUA_MININTEGER limit would still run the loop once.
*/
unsafe extern "C" fn forlimit(
    mut obj: *const TValue,
    mut p: *mut lua_Integer,
    mut step: lua_Integer,
    mut stopnow: *mut libc::c_int,
) -> libc::c_int {
    /* usually, let loops run */
    *stopnow = 0i32;
    if 0 == luaV_tointeger(
        obj,
        p,
        if step < 0i32 as libc::c_longlong {
            2i32
        } else {
            1i32
        },
    ) {
        /* not fit in integer? */
        /* try to convert to float */
        let mut n: lua_Number = 0.;
        /* cannot convert to float? */
        if 0 == if (*obj).tt_ == 3i32 | 0i32 << 4i32 {
            if (*obj).tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((obj))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    141i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                        b"int forlimit(const TValue *, lua_Integer *, lua_Integer, int *)\x00",
                    )).as_ptr(),
                );
            };
            n = (*obj).value_.n;
            1i32
        } else {
            luaV_tonumber_(obj, &mut n)
        } {
            /* not a number */
            return 0i32;
        } else if (0i32 as libc::c_double) < n {
            /* if true, float is larger than max integer */
            *p = 9223372036854775807i64;
            if step < 0i32 as libc::c_longlong {
                *stopnow = 1i32
            }
        } else {
            /* float is smaller than min integer */
            *p = -9223372036854775807i64 - 1i64;
            if step >= 0i32 as libc::c_longlong {
                *stopnow = 1i32
            }
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_objlen(
    mut L: *mut lua_State_0,
    mut ra: StkId,
    mut rb: *const TValue,
) -> () {
    let mut tm: *const TValue = 0 as *const TValue;
    match (*rb).tt_ & 0x3fi32 {
        5 => {
            if (*rb).tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((rb))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    522i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                        b"void luaV_objlen(lua_State *, StkId, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*rb).value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(((rb)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    522i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                        b"void luaV_objlen(lua_State *, StkId, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            let mut h: *mut Table = &mut (*((*rb).value_.gc as *mut GCUnion)).h;
            tm = if (*h).metatable.is_null() {
                0 as *const TValue
            } else if 0 != (*(*h).metatable).flags as libc::c_uint & 1u32 << TM_LEN as libc::c_int {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*h).metatable,
                    TM_LEN,
                    (*(*L).l_G).tmname[TM_LEN as libc::c_int as usize],
                )
            };
            if tm.is_null() {
                /* metamethod? break switch to call it */
                let mut io: *mut TValue = ra;
                (*io).value_.i = luaH_getn(h) as lua_Integer;
                (*io).tt_ = 3i32 | 1i32 << 4i32;
                /* else primitive len */
                return;
            }
        }
        4 => {
            let mut io_0: *mut TValue = ra;
            if (*rb).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((rb))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    529i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                        b"void luaV_objlen(lua_State *, StkId, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*rb).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((rb)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    529i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                        b"void luaV_objlen(lua_State *, StkId, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            (*io_0).value_.i =
                (*&mut (*((*rb).value_.gc as *mut GCUnion)).ts).shrlen as lua_Integer;
            (*io_0).tt_ = 3i32 | 1i32 << 4i32;
            return;
        }
        20 => {
            let mut io_1: *mut TValue = ra;
            if (*rb).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((rb))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    533i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                        b"void luaV_objlen(lua_State *, StkId, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*rb).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((rb)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lvm.c\x00" as *const u8 as *const libc::c_char,
                    533i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                        b"void luaV_objlen(lua_State *, StkId, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            (*io_1).value_.i =
                (*&mut (*((*rb).value_.gc as *mut GCUnion)).ts).u.lnglen as lua_Integer;
            (*io_1).tt_ = 3i32 | 1i32 << 4i32;
            return;
        }
        _ => {
            /* try metamethod */
            tm = luaT_gettmbyobj(L, rb, TM_LEN);
            /* no metamethod? */
            if (*tm).tt_ == 0i32 {
                luaG_typeerror(
                    L,
                    rb,
                    b"get length of\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    luaT_callTM(L, tm, rb, rb, ra, 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaV_div(
    mut L: *mut lua_State_0,
    mut m: lua_Integer,
    mut n: lua_Integer,
) -> lua_Integer {
    if (n as lua_Unsigned).wrapping_add(1u32 as libc::c_ulonglong) <= 1u32 as libc::c_ulonglong {
        /* special cases: -1 or 0 */
        if n == 0i32 as libc::c_longlong {
            luaG_runerror(
                L,
                b"attempt to divide by zero\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            return (0i32 as lua_Unsigned).wrapping_sub(m as lua_Unsigned) as lua_Integer;
        }
    } else {
        /* perform C division */
        let mut q: lua_Integer = m / n;
        /* 'm/n' would be negative non-integer? */
        if m ^ n < 0i32 as libc::c_longlong && m % n != 0i32 as libc::c_longlong {
            /* correct result for different rounding */
            q -= 1i32 as libc::c_longlong
        }
        return q;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_mod(
    mut L: *mut lua_State_0,
    mut m: lua_Integer,
    mut n: lua_Integer,
) -> lua_Integer {
    if (n as lua_Unsigned).wrapping_add(1u32 as libc::c_ulonglong) <= 1u32 as libc::c_ulonglong {
        /* special cases: -1 or 0 */
        if n == 0i32 as libc::c_longlong {
            luaG_runerror(
                L,
                b"attempt to perform \'n%%0\'\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            return 0i32 as lua_Integer;
        }
    } else {
        let mut r: lua_Integer = m % n;
        /* 'm/n' would be non-integer negative? */
        if r != 0i32 as libc::c_longlong && m ^ n < 0i32 as libc::c_longlong {
            /* correct result for different rounding */
            r += n
        }
        return r;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_shiftl(mut x: lua_Integer, mut y: lua_Integer) -> lua_Integer {
    if y < 0i32 as libc::c_longlong {
        /* shift right? */
        if y <= -((::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong) as libc::c_int) as libc::c_longlong
        {
            return 0i32 as lua_Integer;
        } else {
            return (x as lua_Unsigned >> -y as lua_Unsigned) as lua_Integer;
        }
    } else if y >= (::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
        .wrapping_mul(8i32 as libc::c_ulong) as libc::c_int as libc::c_longlong
    {
        return 0i32 as lua_Integer;
    } else {
        return ((x as lua_Unsigned) << y as lua_Unsigned) as lua_Integer;
    };
}
