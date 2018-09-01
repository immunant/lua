use lstate::{CallInfo, lua_State, global_State};
use lobject::{TValue, lua_TValue, Value, GCObject};

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
    fn random() -> libc::c_long;
    #[no_mangle]
    fn srandom(__seed: libc::c_uint) -> ();
    #[no_mangle]
    fn acos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn asin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tan(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn exp(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log10(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log2(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut signgam: libc::c_int;
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
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tointegerx(
        L: *mut lua_State_0,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
    #[no_mangle]
    fn lua_compare(
        L: *mut lua_State_0,
        idx1: libc::c_int,
        idx2: libc::c_int,
        op: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State_0, n: lua_Number) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> ();
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
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State_0,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_checknumber(L: *mut lua_State_0, arg: libc::c_int) -> lua_Number;
    #[no_mangle]
    fn luaL_optnumber(L: *mut lua_State_0, arg: libc::c_int, def: lua_Number) -> lua_Number;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State_0, arg: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State_0, arg: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void, _: size_t, _: size_t)
        -> *mut libc::c_void,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub hash: *mut *mut TString,
    pub nuse: libc::c_int,
    pub size: libc::c_int,
}
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
/* unsigned integer type */
pub type lua_Unsigned = libc::c_ulonglong;
pub type _IO_lock_t = ();
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
/*
** Atomic type (relative to signals) to better ensure that 'lua_sethook'
** is thread safe
*/
/* extra stack space to handle TM calls and some other extras */
/* kinds of Garbage Collection */
/* gc was forced by an allocation failure */
pub type stringtable_0 = stringtable;
pub type l_mem = ptrdiff_t;
pub type ptrdiff_t = libc::c_long;
pub type intptr_t = libc::c_long;
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
pub type Value_0 = Value;
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
pub type GCObject_0 = GCObject;
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
pub type global_State_0 = global_State;
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
#[no_mangle]
pub unsafe extern "C" fn luaopen_math(mut L: *mut lua_State_0) -> libc::c_int {
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
        (::std::mem::size_of::<[luaL_Reg_0; 28]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg_0>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, mathlib.as_ptr(), 0i32);
    lua_pushnumber(L, 3.141592653589793f64);
    lua_setfield(L, -2i32, b"pi\x00" as *const u8 as *const libc::c_char);
    lua_pushnumber(L, ::std::f64::INFINITY);
    lua_setfield(L, -2i32, b"huge\x00" as *const u8 as *const libc::c_char);
    lua_pushinteger(L, 9223372036854775807i64);
    lua_setfield(
        L,
        -2i32,
        b"maxinteger\x00" as *const u8 as *const libc::c_char,
    );
    lua_pushinteger(L, -9223372036854775807i64 - 1i64);
    lua_setfield(
        L,
        -2i32,
        b"mininteger\x00" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
/*
** {==================================================================
** Deprecated functions (for compatibility only)
** ===================================================================
*/
/* }================================================================== */
static mut mathlib: [luaL_Reg_0; 28] = unsafe {
    [
        luaL_Reg {
            name: b"abs\x00" as *const u8 as *const libc::c_char,
            func: Some(math_abs),
        },
        luaL_Reg {
            name: b"acos\x00" as *const u8 as *const libc::c_char,
            func: Some(math_acos),
        },
        luaL_Reg {
            name: b"asin\x00" as *const u8 as *const libc::c_char,
            func: Some(math_asin),
        },
        luaL_Reg {
            name: b"atan\x00" as *const u8 as *const libc::c_char,
            func: Some(math_atan),
        },
        luaL_Reg {
            name: b"ceil\x00" as *const u8 as *const libc::c_char,
            func: Some(math_ceil),
        },
        luaL_Reg {
            name: b"cos\x00" as *const u8 as *const libc::c_char,
            func: Some(math_cos),
        },
        luaL_Reg {
            name: b"deg\x00" as *const u8 as *const libc::c_char,
            func: Some(math_deg),
        },
        luaL_Reg {
            name: b"exp\x00" as *const u8 as *const libc::c_char,
            func: Some(math_exp),
        },
        luaL_Reg {
            name: b"tointeger\x00" as *const u8 as *const libc::c_char,
            func: Some(math_toint),
        },
        luaL_Reg {
            name: b"floor\x00" as *const u8 as *const libc::c_char,
            func: Some(math_floor),
        },
        luaL_Reg {
            name: b"fmod\x00" as *const u8 as *const libc::c_char,
            func: Some(math_fmod),
        },
        luaL_Reg {
            name: b"ult\x00" as *const u8 as *const libc::c_char,
            func: Some(math_ult),
        },
        luaL_Reg {
            name: b"log\x00" as *const u8 as *const libc::c_char,
            func: Some(math_log),
        },
        luaL_Reg {
            name: b"max\x00" as *const u8 as *const libc::c_char,
            func: Some(math_max),
        },
        luaL_Reg {
            name: b"min\x00" as *const u8 as *const libc::c_char,
            func: Some(math_min),
        },
        luaL_Reg {
            name: b"modf\x00" as *const u8 as *const libc::c_char,
            func: Some(math_modf),
        },
        luaL_Reg {
            name: b"rad\x00" as *const u8 as *const libc::c_char,
            func: Some(math_rad),
        },
        luaL_Reg {
            name: b"random\x00" as *const u8 as *const libc::c_char,
            func: Some(math_random),
        },
        luaL_Reg {
            name: b"randomseed\x00" as *const u8 as *const libc::c_char,
            func: Some(math_randomseed),
        },
        luaL_Reg {
            name: b"sin\x00" as *const u8 as *const libc::c_char,
            func: Some(math_sin),
        },
        luaL_Reg {
            name: b"sqrt\x00" as *const u8 as *const libc::c_char,
            func: Some(math_sqrt),
        },
        luaL_Reg {
            name: b"tan\x00" as *const u8 as *const libc::c_char,
            func: Some(math_tan),
        },
        luaL_Reg {
            name: b"type\x00" as *const u8 as *const libc::c_char,
            func: Some(math_type),
        },
        luaL_Reg {
            name: b"pi\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"huge\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"maxinteger\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"mininteger\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn math_type(mut L: *mut lua_State_0) -> libc::c_int {
    if lua_type(L, 1i32) == 3i32 {
        if 0 != lua_isinteger(L, 1i32) {
            lua_pushstring(L, b"integer\x00" as *const u8 as *const libc::c_char);
        } else {
            lua_pushstring(L, b"float\x00" as *const u8 as *const libc::c_char);
        }
    } else {
        luaL_checkany(L, 1i32);
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn math_tan(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, tan(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_sqrt(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, sqrt(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_sin(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, sin(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_randomseed(mut L: *mut lua_State_0) -> libc::c_int {
    srandom(luaL_checknumber(L, 1i32) as lua_Integer as libc::c_uint);
    /* discard first value to avoid undesirable correlations */
    random();
    return 0i32;
}
/*
** This function uses 'double' (instead of 'lua_Number') to ensure that
** all bits from 'l_rand' can be represented, and that 'RANDMAX + 1.0'
** will keep full precision (ensuring that 'r' is always less than 1.0.)
*/
unsafe extern "C" fn math_random(mut L: *mut lua_State_0) -> libc::c_int {
    let mut low: lua_Integer = 0;
    let mut up: lua_Integer = 0;
    let mut r: libc::c_double =
        random() as libc::c_double * (1.0f64 / (2147483647i32 as libc::c_double + 1.0f64));
    match lua_gettop(L) {
        0 => {
            /* no arguments */
            /* Number between 0 and 1 */
            lua_pushnumber(L, r);
            return 1i32;
        }
        1 => {
            /* only upper limit */
            low = 1i32 as lua_Integer;
            up = luaL_checkinteger(L, 1i32)
        }
        2 => {
            /* lower and upper limits */
            low = luaL_checkinteger(L, 1i32);
            up = luaL_checkinteger(L, 2i32)
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments\x00" as *const u8 as *const libc::c_char,
            )
        }
    }
    (low <= up
        || 0 != luaL_argerror(
            L,
            1i32,
            b"interval is empty\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    (low >= 0i32 as libc::c_longlong || up <= 9223372036854775807i64 + low
        || 0 != luaL_argerror(
            L,
            1i32,
            b"interval too large\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    /* random integer in the interval [low, up] */
    r *= (up - low) as libc::c_double + 1.0f64;
    lua_pushinteger(L, r as lua_Integer + low);
    return 1i32;
}
unsafe extern "C" fn math_rad(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1i32) * (3.141592653589793f64 / 180.0f64),
    );
    return 1i32;
}
/*
** next function does not use 'modf', avoiding problems with 'double*'
** (which is not compatible with 'float*') when lua_Number is not
** 'double'.
*/
unsafe extern "C" fn math_modf(mut L: *mut lua_State_0) -> libc::c_int {
    let mut ip: lua_Number = 0.;
    let mut n: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        /* number is its own integer part */
        lua_settop(L, 1i32);
        /* no fractional part */
        lua_pushnumber(L, 0i32 as lua_Number);
    } else {
        n = luaL_checknumber(L, 1i32);
        /* integer part (rounds toward zero) */
        ip = if n < 0i32 as libc::c_double {
            ceil(n)
        } else {
            floor(n)
        };
        pushnumint(L, ip);
        /* fractional part (test needed for inf/-inf) */
        lua_pushnumber(L, if n == ip { 0.0f64 } else { n - ip });
    }
    return 2i32;
}
unsafe extern "C" fn pushnumint(mut L: *mut lua_State_0, mut d: lua_Number) -> () {
    let mut n: lua_Integer = 0;
    /* does 'd' fit in an integer? */
    if d >= (-9223372036854775807i64 - 1i64) as libc::c_double
        && d < -((-9223372036854775807i64 - 1i64) as libc::c_double) && {
        n = d as libc::c_longlong;
        0 != 1i32
    } {
        /* result is integer */
        lua_pushinteger(L, n);
    } else {
        /* result is float */
        lua_pushnumber(L, d);
    };
}
unsafe extern "C" fn math_min(mut L: *mut lua_State_0) -> libc::c_int {
    /* number of arguments */
    let mut n: libc::c_int = lua_gettop(L);
    /* index of current minimum value */
    let mut imin: libc::c_int = 1i32;
    let mut i: libc::c_int = 0;
    (n >= 1i32
        || 0 != luaL_argerror(
            L,
            1i32,
            b"value expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    i = 2i32;
    while i <= n {
        if 0 != lua_compare(L, i, imin, 1i32) {
            imin = i
        }
        i += 1
    }
    lua_pushvalue(L, imin);
    return 1i32;
}
unsafe extern "C" fn math_max(mut L: *mut lua_State_0) -> libc::c_int {
    /* number of arguments */
    let mut n: libc::c_int = lua_gettop(L);
    /* index of current maximum value */
    let mut imax: libc::c_int = 1i32;
    let mut i: libc::c_int = 0;
    (n >= 1i32
        || 0 != luaL_argerror(
            L,
            1i32,
            b"value expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    i = 2i32;
    while i <= n {
        if 0 != lua_compare(L, imax, i, 1i32) {
            imax = i
        }
        i += 1
    }
    lua_pushvalue(L, imax);
    return 1i32;
}
unsafe extern "C" fn math_log(mut L: *mut lua_State_0) -> libc::c_int {
    let mut x: lua_Number = luaL_checknumber(L, 1i32);
    let mut res: lua_Number = 0.;
    if lua_type(L, 2i32) <= 0i32 {
        res = log(x)
    } else {
        let mut base: lua_Number = luaL_checknumber(L, 2i32);
        if base == 2.0f64 {
            res = log2(x)
        } else if base == 10.0f64 {
            res = log10(x)
        } else {
            res = log(x) / log(base)
        }
    }
    lua_pushnumber(L, res);
    return 1i32;
}
unsafe extern "C" fn math_ult(mut L: *mut lua_State_0) -> libc::c_int {
    let mut a: lua_Integer = luaL_checkinteger(L, 1i32);
    let mut b: lua_Integer = luaL_checkinteger(L, 2i32);
    lua_pushboolean(L, ((a as lua_Unsigned) < b as lua_Unsigned) as libc::c_int);
    return 1i32;
}
unsafe extern "C" fn math_fmod(mut L: *mut lua_State_0) -> libc::c_int {
    if 0 != lua_isinteger(L, 1i32) && 0 != lua_isinteger(L, 2i32) {
        let mut d: lua_Integer = lua_tointegerx(L, 2i32, 0 as *mut libc::c_int);
        if (d as lua_Unsigned).wrapping_add(1u32 as libc::c_ulonglong) <= 1u32 as libc::c_ulonglong
        {
            /* special cases: -1 or 0 */
            (d != 0i32 as libc::c_longlong
                || 0 != luaL_argerror(L, 2i32, b"zero\x00" as *const u8 as *const libc::c_char))
                as libc::c_int;
            /* avoid overflow with 0x80000... / -1 */
            lua_pushinteger(L, 0i32 as lua_Integer);
        } else {
            lua_pushinteger(L, lua_tointegerx(L, 1i32, 0 as *mut libc::c_int) % d);
        }
    } else {
        lua_pushnumber(
            L,
            fmod(luaL_checknumber(L, 1i32), luaL_checknumber(L, 2i32)),
        );
    }
    return 1i32;
}
unsafe extern "C" fn math_floor(mut L: *mut lua_State_0) -> libc::c_int {
    let mut d: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        /* integer is its own floor */
        lua_settop(L, 1i32);
    } else {
        d = floor(luaL_checknumber(L, 1i32));
        pushnumint(L, d);
    }
    return 1i32;
}
unsafe extern "C" fn math_toint(mut L: *mut lua_State_0) -> libc::c_int {
    let mut valid: libc::c_int = 0;
    let mut n: lua_Integer = lua_tointegerx(L, 1i32, &mut valid);
    if 0 != valid {
        lua_pushinteger(L, n);
    } else {
        luaL_checkany(L, 1i32);
        /* value is not convertible to integer */
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn math_exp(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, exp(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_deg(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1i32) * (180.0f64 / 3.141592653589793f64),
    );
    return 1i32;
}
unsafe extern "C" fn math_cos(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, cos(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_ceil(mut L: *mut lua_State_0) -> libc::c_int {
    let mut d: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        /* integer is its own ceil */
        lua_settop(L, 1i32);
    } else {
        d = ceil(luaL_checknumber(L, 1i32));
        pushnumint(L, d);
    }
    return 1i32;
}
unsafe extern "C" fn math_atan(mut L: *mut lua_State_0) -> libc::c_int {
    let mut y: lua_Number = luaL_checknumber(L, 1i32);
    let mut x: lua_Number = luaL_optnumber(L, 2i32, 1i32 as lua_Number);
    lua_pushnumber(L, atan2(y, x));
    return 1i32;
}
unsafe extern "C" fn math_asin(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, asin(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_acos(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, acos(luaL_checknumber(L, 1i32)));
    return 1i32;
}
/*
** $Id: lmathlib.c,v 1.118 2016/12/20 18:37:00 roberto Exp roberto $
** Standard mathematical library
** See Copyright Notice in lua.h
*/
/* { */
/* (2^31 - 1), following POSIX */
/* } */
unsafe extern "C" fn math_abs(mut L: *mut lua_State_0) -> libc::c_int {
    if 0 != lua_isinteger(L, 1i32) {
        let mut n: lua_Integer = lua_tointegerx(L, 1i32, 0 as *mut libc::c_int);
        if n < 0i32 as libc::c_longlong {
            n = (0u32 as libc::c_ulonglong).wrapping_sub(n as lua_Unsigned) as lua_Integer
        }
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, fabs(luaL_checknumber(L, 1i32)));
    }
    return 1i32;
}
