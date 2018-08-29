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
    pub type UpVal_0;
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(
        L: *mut lua_State_0,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State_0, idx: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_pushlstring(
        L: *mut lua_State_0,
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State_0, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State_0, p: *mut libc::c_void) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgetp(L: *mut lua_State_0, idx: libc::c_int, p: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_rawsetp(L: *mut lua_State_0, idx: libc::c_int, p: *const libc::c_void) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State_0, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_callk(
        L: *mut lua_State_0,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_checklstring(
        L: *mut lua_State_0,
        arg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_optlstring(
        L: *mut lua_State_0,
        arg: libc::c_int,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_loadfilex(
        L: *mut lua_State_0,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_len(L: *mut lua_State_0, idx: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_gsub(
        L: *mut lua_State_0,
        s: *const libc::c_char,
        p: *const libc::c_char,
        r: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_getsubtable(
        L: *mut lua_State_0,
        idx: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State_0, B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn dlerror() -> *mut libc::c_char;
    #[no_mangle]
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type FILE = _IO_FILE;
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
    pub mt: [*mut Table; 9],
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
    pub u: unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    lnglen: size_t,
    hnext: *mut TString_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,
    pub lsizenode: lu_byte,
    pub sizearray: libc::c_uint,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table,
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
    nk: unnamed_3,
    tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
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
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void, _: size_t, _: size_t)
        -> *mut libc::c_void,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State_0,
    pub initb: [libc::c_char; 23],
}
/*
** ===============================================================
** some useful macros
** ===============================================================
*/
/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/
pub type luaL_Buffer_0 = luaL_Buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
/*
** $Id: lauxlib.h,v 1.130 2016/12/04 20:17:24 roberto Exp roberto $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/
/* extra error code for 'luaL_loadfilex' */
/* key, in the registry, for table of loaded modules */
/* key, in the registry, for table of preloaded loaders */
pub type luaL_Reg_0 = luaL_Reg;
#[no_mangle]
pub unsafe extern "C" fn luaopen_package(mut L: *mut lua_State_0) -> libc::c_int {
    createclibstable(L);
    luaL_checkversion_(
        L,
        503i32 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::std::mem::size_of::<[luaL_Reg_0; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg_0>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, pk_funcs.as_ptr(), 0i32);
    /* create 'package' table */
    createsearcherstable(L);
    /* set paths */
    setpath(L, b"path\x00" as *const u8 as *const libc::c_char,
            b"LUA_PATH\x00" as *const u8 as *const libc::c_char,
            b"/usr/local/share/lua/5.3/?.lua;/usr/local/share/lua/5.3/?/init.lua;/usr/local/lib/lua/5.3/?.lua;/usr/local/lib/lua/5.3/?/init.lua;./?.lua;./?/init.lua\x00"
                as *const u8 as *const libc::c_char);
    setpath(
        L,
        b"cpath\x00" as *const u8 as *const libc::c_char,
        b"LUA_CPATH\x00" as *const u8 as *const libc::c_char,
        b"/usr/local/lib/lua/5.3/?.so;/usr/local/lib/lua/5.3/loadall.so;./?.so\x00" as *const u8
            as *const libc::c_char,
    );
    lua_pushstring(
        L,
        b"/\n;\n?\n!\n-\n\x00" as *const u8 as *const libc::c_char,
    );
    /* store config information */
    lua_setfield(L, -2i32, b"config\x00" as *const u8 as *const libc::c_char);
    /* set field 'loaded' */
    luaL_getsubtable(
        L,
        -50000i32 - 1000i32,
        b"_LOADED\x00" as *const u8 as *const libc::c_char,
    );
    lua_setfield(L, -2i32, b"loaded\x00" as *const u8 as *const libc::c_char);
    /* set field 'preload' */
    luaL_getsubtable(
        L,
        -50000i32 - 1000i32,
        b"_PRELOAD\x00" as *const u8 as *const libc::c_char,
    );
    lua_setfield(L, -2i32, b"preload\x00" as *const u8 as *const libc::c_char);
    lua_rawgeti(L, -50000i32 - 1000i32, 2i32 as lua_Integer);
    /* set 'package' as upvalue for next lib */
    lua_pushvalue(L, -2i32);
    /* open lib into global table */
    luaL_setfuncs(L, ll_funcs.as_ptr(), 1i32);
    lua_settop(L, -1i32 - 1i32);
    /* pop global table */
    /* return 'package' table */
    return 1i32;
}
/* placeholders */
static mut ll_funcs: [luaL_Reg_0; 2] = unsafe {
    [
        luaL_Reg {
            name: b"require\x00" as *const u8 as *const libc::c_char,
            func: Some(ll_require),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
/* remove both returns */
unsafe extern "C" fn ll_require(mut L: *mut lua_State_0) -> libc::c_int {
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    /* LOADED table will be at index 2 */
    lua_settop(L, 1i32);
    lua_getfield(
        L,
        -50000i32 - 1000i32,
        b"_LOADED\x00" as *const u8 as *const libc::c_char,
    );
    /* LOADED[name] */
    lua_getfield(L, 2i32, name);
    /* is it there? */
    if 0 != lua_toboolean(L, -1i32) {
        /* package is already loaded */
        return 1i32;
    } else {
        lua_settop(L, -1i32 - 1i32);
        /* else must load package */
        /* remove 'getfield' result */
        findloader(L, name);
        /* pass name as argument to module loader */
        lua_pushstring(L, name);
        lua_rotate(L, -2i32, 1i32);
        lua_callk(L, 2i32, 1i32, 0i32 as lua_KContext, None);
        /* name is 1st argument (before search data) */
        /* run loader to load module */
        /* non-nil return? */
        if !(lua_type(L, -1i32) == 0i32) {
            /* LOADED[name] = returned value */
            lua_setfield(L, 2i32, name);
        }
        if lua_getfield(L, 2i32, name) == 0i32 {
            /* module set no value? */
            /* use true as result */
            lua_pushboolean(L, 1i32);
            /* extra copy to be returned */
            lua_pushvalue(L, -1i32);
            /* LOADED[name] = true */
            lua_setfield(L, 2i32, name);
        }
        return 1i32;
    };
}
unsafe extern "C" fn findloader(mut L: *mut lua_State_0, mut name: *const libc::c_char) -> () {
    let mut i: libc::c_int = 0;
    /* to build error message */
    let mut msg: luaL_Buffer_0 = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State_0,
        initb: [0; 23],
    };
    luaL_buffinit(L, &mut msg);
    /* push 'package.searchers' to index 3 in the stack */
    if lua_getfield(
        L,
        -50000i32 - 1000i32 - 1i32,
        b"searchers\x00" as *const u8 as *const libc::c_char,
    ) != 5i32
    {
        luaL_error(
            L,
            b"\'package.searchers\' must be a table\x00" as *const u8 as *const libc::c_char,
        );
    }
    /*  iterate over available searchers to find a loader */
    i = 1i32;
    loop {
        if lua_rawgeti(L, 3i32, i as lua_Integer) == 0i32 {
            /* no more searchers? */
            lua_settop(L, -1i32 - 1i32);
            /* remove nil */
            /* create error message */
            luaL_pushresult(&mut msg);
            luaL_error(
                L,
                b"module \'%s\' not found:%s\x00" as *const u8 as *const libc::c_char,
                name,
                lua_tolstring(L, -1i32, 0 as *mut size_t),
            );
        }
        lua_pushstring(L, name);
        lua_callk(L, 1i32, 2i32, 0i32 as lua_KContext, None);
        /* call it */
        /* did it find a loader? */
        if lua_type(L, -2i32) == 6i32 {
            /* module loader found */
            return;
        } else {
            if 0 != lua_isstring(L, -2i32) {
                /* searcher returned error message? */
                lua_settop(L, -1i32 - 1i32);
                /* remove extra return */
                /* concatenate error message */
                luaL_addvalue(&mut msg);
            } else {
                lua_settop(L, -2i32 - 1i32);
            }
            i += 1
        }
    }
}
/*
** Set a path
*/
unsafe extern "C" fn setpath(
    mut L: *mut lua_State_0,
    mut fieldname: *const libc::c_char,
    mut envname: *const libc::c_char,
    mut dft: *const libc::c_char,
) -> () {
    let mut nver: *const libc::c_char = lua_pushfstring(
        L,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        envname,
        b"_5_3\x00" as *const u8 as *const libc::c_char,
    );
    /* use versioned name */
    let mut path: *const libc::c_char = getenv(nver);
    /* no environment variable? */
    if path.is_null() {
        /* try unversioned name */
        path = getenv(envname)
    }
    /* no environment variable? */
    if path.is_null() || 0 != noenv(L) {
        /* use default */
        lua_pushstring(L, dft);
    } else {
        /* replace ";;" by ";AUXMARK;" and then AUXMARK by default path */
        path = luaL_gsub(
            L,
            path,
            b";;\x00" as *const u8 as *const libc::c_char,
            b";\x01;\x00" as *const u8 as *const libc::c_char,
        );
        luaL_gsub(
            L,
            path,
            b"\x01\x00" as *const u8 as *const libc::c_char,
            dft,
        );
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    }
    /* remove result from 1st 'gsub' */
    /* package[fieldname] = path value */
    lua_setfield(L, -3i32, fieldname);
    lua_settop(L, -1i32 - 1i32);
}
/* }====================================================== */
/* }{ */
/* } */
/*
** {==================================================================
** Set Paths
** ===================================================================
*/
/*
** LUA_PATH_VAR and LUA_CPATH_VAR are the names of the environment
** variables that Lua check to set its paths.
*/
/* auxiliary mark */
/*
** return registry.LUA_NOENV as a boolean
*/
unsafe extern "C" fn noenv(mut L: *mut lua_State_0) -> libc::c_int {
    let mut b: libc::c_int = 0;
    lua_getfield(
        L,
        -50000i32 - 1000i32,
        b"LUA_NOENV\x00" as *const u8 as *const libc::c_char,
    );
    b = lua_toboolean(L, -1i32);
    lua_settop(L, -1i32 - 1i32);
    /* remove value */
    return b;
}
unsafe extern "C" fn createsearcherstable(mut L: *mut lua_State_0) -> () {
    static mut searchers: [lua_CFunction; 5] = unsafe {
        [
            Some(searcher_preload),
            Some(searcher_Lua),
            Some(searcher_C),
            Some(searcher_Croot),
            None,
        ]
    };
    let mut i: libc::c_int = 0;
    /* create 'searchers' table */
    lua_createtable(
        L,
        (::std::mem::size_of::<[lua_CFunction; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<lua_CFunction>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
        0i32,
    );
    /* fill it with predefined searchers */
    i = 0i32;
    while searchers[i as usize].is_some() {
        /* set 'package' as upvalue for all searchers */
        lua_pushvalue(L, -2i32);
        lua_pushcclosure(L, searchers[i as usize], 1i32);
        lua_rawseti(L, -2i32, (i + 1i32) as lua_Integer);
        i += 1
    }
    /* put it in field 'searchers' */
    lua_setfield(
        L,
        -2i32,
        b"searchers\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn searcher_Croot(mut L: *mut lua_State_0) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut p: *const libc::c_char = strchr(name, '.' as i32);
    let mut stat: libc::c_int = 0;
    if p.is_null() {
        /* is root */
        return 0i32;
    } else {
        lua_pushlstring(
            L,
            name,
            p.wrapping_offset_from(name) as libc::c_long as size_t,
        );
        filename = findfile(
            L,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
            b"cpath\x00" as *const u8 as *const libc::c_char,
            b"/\x00" as *const u8 as *const libc::c_char,
        );
        if filename.is_null() {
            /* root not found */
            return 1i32;
        } else {
            stat = loadfunc(L, filename, name);
            if stat != 0i32 {
                if stat != 2i32 {
                    /* real error */
                    return checkload(L, 0i32, filename);
                } else {
                    /* open function not found */
                    lua_pushfstring(
                        L,
                        b"\n\tno module \'%s\' in file \'%s\'\x00" as *const u8
                            as *const libc::c_char,
                        name,
                        filename,
                    );
                    return 1i32;
                }
            } else {
                /* will be 2nd argument to module */
                lua_pushstring(L, filename);
                return 2i32;
            }
        }
    };
}
unsafe extern "C" fn checkload(
    mut L: *mut lua_State_0,
    mut stat: libc::c_int,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    if 0 != stat {
        /* module loaded successfully? */
        /* will be 2nd argument to module */
        lua_pushstring(L, filename);
        /* return open function and file name */
        return 2i32;
    } else {
        return luaL_error(
            L,
            b"error loading module \'%s\' from file \'%s\':\n\t%s\x00" as *const u8
                as *const libc::c_char,
            lua_tolstring(L, 1i32, 0 as *mut size_t),
            filename,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
        );
    };
}
/*
** Try to find a load function for module 'modname' at file 'filename'.
** First, change '.' to '_' in 'modname'; then, if 'modname' has
** the form X-Y (that is, it has an "ignore mark"), build a function
** name "luaopen_X" and look for it. (For compatibility, if that
** fails, it also tries "luaopen_Y".) If there is no ignore mark,
** look for a function named "luaopen_modname".
*/
unsafe extern "C" fn loadfunc(
    mut L: *mut lua_State_0,
    mut filename: *const libc::c_char,
    mut modname: *const libc::c_char,
) -> libc::c_int {
    let mut openfunc: *const libc::c_char = 0 as *const libc::c_char;
    let mut mark: *const libc::c_char = 0 as *const libc::c_char;
    modname = luaL_gsub(
        L,
        modname,
        b".\x00" as *const u8 as *const libc::c_char,
        b"_\x00" as *const u8 as *const libc::c_char,
    );
    mark = strchr(
        modname,
        *(b"-\x00" as *const u8 as *const libc::c_char) as libc::c_int,
    );
    if !mark.is_null() {
        let mut stat: libc::c_int = 0;
        openfunc = lua_pushlstring(
            L,
            modname,
            mark.wrapping_offset_from(modname) as libc::c_long as size_t,
        );
        openfunc = lua_pushfstring(
            L,
            b"luaopen_%s\x00" as *const u8 as *const libc::c_char,
            openfunc,
        );
        stat = lookforfunc(L, filename, openfunc);
        if stat != 2i32 {
            return stat;
        } else {
            modname = mark.offset(1isize)
        }
    }
    openfunc = lua_pushfstring(
        L,
        b"luaopen_%s\x00" as *const u8 as *const libc::c_char,
        modname,
    );
    return lookforfunc(L, filename, openfunc);
}
/* error codes for 'lookforfunc' */
/*
** Look for a C function named 'sym' in a dynamically loaded library
** 'path'.
** First, check whether the library is already loaded; if not, try
** to load it.
** Then, if 'sym' is '*', return true (as library has been loaded).
** Otherwise, look for symbol 'sym' in the library and push a
** C function with that symbol.
** Return 0 and 'true' or a function in the stack; in case of
** errors, return an error code and an error message in the stack.
*/
unsafe extern "C" fn lookforfunc(
    mut L: *mut lua_State_0,
    mut path: *const libc::c_char,
    mut sym: *const libc::c_char,
) -> libc::c_int {
    /* check loaded C libraries */
    let mut reg: *mut libc::c_void = checkclib(L, path);
    if reg.is_null() {
        /* must load library? */
        /* global symbols if 'sym'=='*' */
        reg = lsys_load(L, path, (*sym as libc::c_int == '*' as i32) as libc::c_int);
        if reg.is_null() {
            /* unable to load library */
            return 1i32;
        } else {
            addtoclib(L, path, reg);
        }
    }
    if *sym as libc::c_int == '*' as i32 {
        /* loading only library (no function)? */
        /* return 'true' */
        lua_pushboolean(L, 1i32);
        /* no errors */
        return 0i32;
    } else {
        let mut f: lua_CFunction = lsys_sym(L, reg, sym);
        if f.is_none() {
            /* unable to find function */
            return 2i32;
        } else {
            lua_pushcclosure(L, f, 0i32);
            /* else create new function */
            /* no errors */
            return 0i32;
        }
    };
}
/* pop versioned variable name */
/* }================================================================== */
/*
** return registry.CLIBS[path]
*/
unsafe extern "C" fn checkclib(
    mut L: *mut lua_State_0,
    mut path: *const libc::c_char,
) -> *mut libc::c_void {
    let mut plib: *mut libc::c_void = 0 as *mut libc::c_void;
    lua_rawgetp(
        L,
        -50000i32 - 1000i32,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
    lua_getfield(L, -1i32, path);
    /* plib = CLIBS[path] */
    plib = lua_touserdata(L, -1i32);
    lua_settop(L, -2i32 - 1i32);
    /* pop CLIBS table and 'plib' */
    return plib;
}
/*
** $Id: loadlib.c,v 1.129 2016/12/04 20:17:24 roberto Exp roberto $
** Dynamic library loader for Lua
** See Copyright Notice in lua.h
**
** This module contains an implementation of loadlib for Unix systems
** that have dlfcn, an implementation for Windows, and a stub for other
** systems.
*/
/*
** LUA_IGMARK is a mark to ignore all before it when building the
** luaopen_ function name.
*/
/*
** LUA_CSUBSEP is the character that replaces dots in submodule names
** when searching for a C loader.
** LUA_LSUBSEP is the character that replaces dots in submodule names
** when searching for a Lua loader.
*/
/* prefix for open functions in C libraries */
/* separator for open functions in C libraries */
/*
** unique key for table in the registry that keeps handles
** for all loaded C libraries
*/
static mut CLIBS: libc::c_int = unsafe { 0i32 };
/*
** Try to find a function named 'sym' in library 'lib'.
** Returns the function; in case of error, returns NULL plus an
** error string in the stack.
*/
unsafe extern "C" fn lsys_sym(
    mut L: *mut lua_State_0,
    mut lib: *mut libc::c_void,
    mut sym: *const libc::c_char,
) -> lua_CFunction {
    let mut f: lua_CFunction =
        ::std::mem::transmute::<*mut libc::c_void, lua_CFunction>(dlsym(lib, sym));
    if f.is_none() {
        lua_pushstring(L, dlerror());
    }
    return f;
}
/*
** registry.CLIBS[path] = plib        -- for queries
** registry.CLIBS[#CLIBS + 1] = plib  -- also keep a list of all libraries
*/
unsafe extern "C" fn addtoclib(
    mut L: *mut lua_State_0,
    mut path: *const libc::c_char,
    mut plib: *mut libc::c_void,
) -> () {
    lua_rawgetp(
        L,
        -50000i32 - 1000i32,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
    lua_pushlightuserdata(L, plib);
    lua_pushvalue(L, -1i32);
    /* CLIBS[path] = plib */
    lua_setfield(L, -3i32, path);
    /* CLIBS[#CLIBS + 1] = plib */
    lua_rawseti(L, -2i32, luaL_len(L, -2i32) + 1i32 as libc::c_longlong);
    lua_settop(L, -1i32 - 1i32);
}
/*
** load C library in file 'path'. If 'seeglb', load with all names in
** the library global.
** Returns the library; in case of error, returns NULL plus an
** error string in the stack.
*/
unsafe extern "C" fn lsys_load(
    mut L: *mut lua_State_0,
    mut path: *const libc::c_char,
    mut seeglb: libc::c_int,
) -> *mut libc::c_void {
    let mut lib: *mut libc::c_void =
        dlopen(path, 0x2i32 | if 0 != seeglb { 0x100i32 } else { 0i32 });
    if lib.is_null() {
        lua_pushstring(L, dlerror());
    }
    return lib;
}
unsafe extern "C" fn findfile(
    mut L: *mut lua_State_0,
    mut name: *const libc::c_char,
    mut pname: *const libc::c_char,
    mut dirsep: *const libc::c_char,
) -> *const libc::c_char {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    lua_getfield(L, -50000i32 - 1000i32 - 1i32, pname);
    path = lua_tolstring(L, -1i32, 0 as *mut size_t);
    if path.is_null() {
        luaL_error(
            L,
            b"\'package.%s\' must be a string\x00" as *const u8 as *const libc::c_char,
            pname,
        );
    }
    return searchpath(
        L,
        name,
        path,
        b".\x00" as *const u8 as *const libc::c_char,
        dirsep,
    );
}
unsafe extern "C" fn searchpath(
    mut L: *mut lua_State_0,
    mut name: *const libc::c_char,
    mut path: *const libc::c_char,
    mut sep: *const libc::c_char,
    mut dirsep: *const libc::c_char,
) -> *const libc::c_char {
    /* to build error message */
    let mut msg: luaL_Buffer_0 = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State_0,
        initb: [0; 23],
    };
    luaL_buffinit(L, &mut msg);
    /* non-empty separator? */
    if *sep as libc::c_int != '\u{0}' as i32 {
        /* replace it by 'dirsep' */
        name = luaL_gsub(L, name, sep, dirsep)
    }
    loop {
        path = pushnexttemplate(L, path);
        if path.is_null() {
            break;
        }
        let mut filename: *const libc::c_char = luaL_gsub(
            L,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
            b"?\x00" as *const u8 as *const libc::c_char,
            name,
        );
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        /* remove path template */
        /* does file exist and is readable? */
        if 0 != readable(filename) {
            /* return that file name */
            return filename;
        } else {
            lua_pushfstring(
                L,
                b"\n\tno file \'%s\'\x00" as *const u8 as *const libc::c_char,
                filename,
            );
            lua_rotate(L, -2i32, -1i32);
            lua_settop(L, -1i32 - 1i32);
            /* remove file name */
            /* concatenate error msg. entry */
            luaL_addvalue(&mut msg);
        }
    }
    /* create error message */
    luaL_pushresult(&mut msg);
    /* not found */
    return 0 as *const libc::c_char;
}
/*
** {======================================================
** 'require' function
** =======================================================
*/
unsafe extern "C" fn readable(mut filename: *const libc::c_char) -> libc::c_int {
    /* try to open file */
    let mut f: *mut FILE = fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        /* open failed */
        return 0i32;
    } else {
        fclose(f);
        return 1i32;
    };
}
unsafe extern "C" fn pushnexttemplate(
    mut L: *mut lua_State_0,
    mut path: *const libc::c_char,
) -> *const libc::c_char {
    let mut l: *const libc::c_char = 0 as *const libc::c_char;
    while *path as libc::c_int == *(b";\x00" as *const u8 as *const libc::c_char) as libc::c_int {
        /* skip separators */
        path = path.offset(1isize)
    }
    if *path as libc::c_int == '\u{0}' as i32 {
        /* no more templates */
        return 0 as *const libc::c_char;
    } else {
        /* find next separator */
        l = strchr(
            path,
            *(b";\x00" as *const u8 as *const libc::c_char) as libc::c_int,
        );
        if l.is_null() {
            l = path.offset(strlen(path) as isize)
        }
        /* template */
        lua_pushlstring(
            L,
            path,
            l.wrapping_offset_from(path) as libc::c_long as size_t,
        );
        return l;
    };
}
unsafe extern "C" fn searcher_C(mut L: *mut lua_State_0) -> libc::c_int {
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut filename: *const libc::c_char = findfile(
        L,
        name,
        b"cpath\x00" as *const u8 as *const libc::c_char,
        b"/\x00" as *const u8 as *const libc::c_char,
    );
    if filename.is_null() {
        /* module not found in this path */
        return 1i32;
    } else {
        return checkload(
            L,
            (loadfunc(L, filename, name) == 0i32) as libc::c_int,
            filename,
        );
    };
}
unsafe extern "C" fn searcher_Lua(mut L: *mut lua_State_0) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    filename = findfile(
        L,
        name,
        b"path\x00" as *const u8 as *const libc::c_char,
        b"/\x00" as *const u8 as *const libc::c_char,
    );
    if filename.is_null() {
        /* module not found in this path */
        return 1i32;
    } else {
        return checkload(
            L,
            (luaL_loadfilex(L, filename, 0 as *const libc::c_char) == 0i32) as libc::c_int,
            filename,
        );
    };
}
unsafe extern "C" fn searcher_preload(mut L: *mut lua_State_0) -> libc::c_int {
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_getfield(
        L,
        -50000i32 - 1000i32,
        b"_PRELOAD\x00" as *const u8 as *const libc::c_char,
    );
    /* not found? */
    if lua_getfield(L, -1i32, name) == 0i32 {
        lua_pushfstring(
            L,
            b"\n\tno field package.preload[\'%s\']\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return 1i32;
}
/* }====================================================== */
/*
** {======================================================
** 'module' function
** =======================================================
*/
/* }====================================================== */
static mut pk_funcs: [luaL_Reg_0; 8] = unsafe {
    [
        luaL_Reg {
            name: b"loadlib\x00" as *const u8 as *const libc::c_char,
            func: Some(ll_loadlib),
        },
        luaL_Reg {
            name: b"searchpath\x00" as *const u8 as *const libc::c_char,
            func: Some(ll_searchpath),
        },
        luaL_Reg {
            name: b"preload\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"cpath\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"path\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"searchers\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"loaded\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn ll_searchpath(mut L: *mut lua_State_0) -> libc::c_int {
    let mut f: *const libc::c_char = searchpath(
        L,
        luaL_checklstring(L, 1i32, 0 as *mut size_t),
        luaL_checklstring(L, 2i32, 0 as *mut size_t),
        luaL_optlstring(
            L,
            3i32,
            b".\x00" as *const u8 as *const libc::c_char,
            0 as *mut size_t,
        ),
        luaL_optlstring(
            L,
            4i32,
            b"/\x00" as *const u8 as *const libc::c_char,
            0 as *mut size_t,
        ),
    );
    if !f.is_null() {
        return 1i32;
    } else {
        /* error message is on top of the stack */
        lua_pushnil(L);
        lua_rotate(L, -2i32, 1i32);
        /* return nil + error message */
        return 2i32;
    };
}
unsafe extern "C" fn ll_loadlib(mut L: *mut lua_State_0) -> libc::c_int {
    let mut path: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut init: *const libc::c_char = luaL_checklstring(L, 2i32, 0 as *mut size_t);
    let mut stat: libc::c_int = lookforfunc(L, path, init);
    /* no errors? */
    if stat == 0i32 {
        /* return the loaded function */
        return 1i32;
    } else {
        /* error; error message is on stack top */
        lua_pushnil(L);
        lua_rotate(L, -2i32, 1i32);
        lua_pushstring(
            L,
            if stat == 1i32 {
                b"open\x00" as *const u8 as *const libc::c_char
            } else {
                b"init\x00" as *const u8 as *const libc::c_char
            },
        );
        /* return nil, error message, and where */
        return 3i32;
    };
}
/*
** create table CLIBS to keep track of loaded C libraries,
** setting a finalizer to close all libraries when closing state.
*/
unsafe extern "C" fn createclibstable(mut L: *mut lua_State_0) -> () {
    lua_createtable(L, 0i32, 0i32);
    /* create CLIBS table */
    /* create metatable for CLIBS */
    lua_createtable(L, 0i32, 1i32);
    lua_pushcclosure(L, Some(gctm), 0i32);
    /* set finalizer for CLIBS table */
    lua_setfield(L, -2i32, b"__gc\x00" as *const u8 as *const libc::c_char);
    lua_setmetatable(L, -2i32);
    /* set CLIBS table in registry */
    lua_rawsetp(
        L,
        -50000i32 - 1000i32,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
}
/* pop CLIBS table */
/*
** __gc tag method for CLIBS table: calls 'lsys_unloadlib' for all lib
** handles in list CLIBS
*/
unsafe extern "C" fn gctm(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: lua_Integer = luaL_len(L, 1i32);
    while n >= 1i32 as libc::c_longlong {
        /* for each handle, in reverse order */
        /* get handle CLIBS[n] */
        lua_rawgeti(L, 1i32, n);
        lsys_unloadlib(lua_touserdata(L, -1i32));
        lua_settop(L, -1i32 - 1i32);
        n -= 1
    }
    /* pop handle */
    return 0i32;
}
/*
** system-dependent functions
*/
/*
** unload library 'lib'
*/
unsafe extern "C" fn lsys_unloadlib(mut lib: *mut libc::c_void) -> () {
    dlclose(lib);
}
