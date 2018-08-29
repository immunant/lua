#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(extern_types, libc, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
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
    pub type _IO_FILE_plus;
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
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_tointegerx(
        L: *mut lua_State_0,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushlstring(
        L: *mut lua_State_0,
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State_0, fn_0: lua_CFunction, n: libc::c_int) -> ();
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
    fn luaL_checklstring(
        L: *mut lua_State_0,
        arg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State_0, arg: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State_0, arg: libc::c_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State_0, sz: libc::c_int, msg: *const libc::c_char) -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State_0, B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
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
pub unsafe extern "C" fn luaopen_utf8(mut L: *mut lua_State_0) -> libc::c_int {
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
        (::std::mem::size_of::<[luaL_Reg_0; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg_0>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, funcs.as_ptr(), 0i32);
    lua_pushlstring(
        L,
        b"[\x00-\x7f\xc2-\xf4][\x80-\xbf]*\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    lua_setfield(
        L,
        -2i32,
        b"charpattern\x00" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
/* pattern to match a single UTF-8 character */
static mut funcs: [luaL_Reg_0; 7] = unsafe {
    [
        luaL_Reg {
            name: b"offset\x00" as *const u8 as *const libc::c_char,
            func: Some(byteoffset),
        },
        luaL_Reg {
            name: b"codepoint\x00" as *const u8 as *const libc::c_char,
            func: Some(codepoint),
        },
        luaL_Reg {
            name: b"char\x00" as *const u8 as *const libc::c_char,
            func: Some(utfchar),
        },
        luaL_Reg {
            name: b"len\x00" as *const u8 as *const libc::c_char,
            func: Some(utflen),
        },
        luaL_Reg {
            name: b"codes\x00" as *const u8 as *const libc::c_char,
            func: Some(iter_codes),
        },
        luaL_Reg {
            name: b"charpattern\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn iter_codes(mut L: *mut lua_State_0) -> libc::c_int {
    luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_pushcclosure(L, Some(iter_aux), 0i32);
    lua_pushvalue(L, 1i32);
    lua_pushinteger(L, 0i32 as lua_Integer);
    return 3i32;
}
unsafe extern "C" fn iter_aux(mut L: *mut lua_State_0) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut n: lua_Integer =
        lua_tointegerx(L, 2i32, 0 as *mut libc::c_int) - 1i32 as libc::c_longlong;
    /* first iteration? */
    if n < 0i32 as libc::c_longlong {
        /* start from here */
        n = 0i32 as lua_Integer
    } else if n < len as lua_Integer {
        /* skip current byte */
        n += 1;
        while *s.offset(n as isize) as libc::c_int & 0xc0i32 == 0x80i32 {
            /* and its continuations */
            n += 1
        }
    }
    if n >= len as lua_Integer {
        /* no more codepoints */
        return 0i32;
    } else {
        let mut code: libc::c_int = 0;
        let mut next: *const libc::c_char = utf8_decode(s.offset(n as isize), &mut code);
        if next.is_null() || *next as libc::c_int & 0xc0i32 == 0x80i32 {
            return luaL_error(
                L,
                b"invalid UTF-8 code\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            lua_pushinteger(L, n + 1i32 as libc::c_longlong);
            lua_pushinteger(L, code as lua_Integer);
            return 2i32;
        }
    };
}
/*
** Decode one UTF-8 sequence, returning NULL if byte sequence is invalid.
*/
unsafe extern "C" fn utf8_decode(
    mut o: *const libc::c_char,
    mut val: *mut libc::c_int,
) -> *const libc::c_char {
    static mut limits: [libc::c_uint; 4] = unsafe {
        [
            0xffi32 as libc::c_uint,
            0x7fi32 as libc::c_uint,
            0x7ffi32 as libc::c_uint,
            0xffffi32 as libc::c_uint,
        ]
    };
    let mut s: *const libc::c_uchar = o as *const libc::c_uchar;
    let mut c: libc::c_uint = *s.offset(0isize) as libc::c_uint;
    /* final result */
    let mut res: libc::c_uint = 0i32 as libc::c_uint;
    /* ascii? */
    if c < 0x80i32 as libc::c_uint {
        res = c
    } else {
        /* to count number of continuation bytes */
        let mut count: libc::c_int = 0i32;
        while 0 != c & 0x40i32 as libc::c_uint {
            /* still have continuation bytes? */
            /* read next byte */
            count += 1;
            let mut cc: libc::c_int = *s.offset(count as isize) as libc::c_int;
            /* not a continuation byte? */
            if cc & 0xc0i32 != 0x80i32 {
                /* invalid byte sequence */
                return 0 as *const libc::c_char;
            } else {
                /* add lower 6 bits from cont. byte */
                res = res << 6i32 | (cc & 0x3fi32) as libc::c_uint;
                /* to test next bit */
                c <<= 1i32
            }
        }
        /* add first byte */
        res |= (c & 0x7fi32 as libc::c_uint) << count * 5i32;
        if count > 3i32 || res > 0x10ffffi32 as libc::c_uint || res <= limits[count as usize] {
            /* invalid byte sequence */
            return 0 as *const libc::c_char;
        } else {
            s = s.offset(count as isize)
        }
    }
    if !val.is_null() {
        *val = res as libc::c_int
    }
    /* +1 to include first byte */
    return (s as *const libc::c_char).offset(1isize);
}
/*
** utf8len(s [, i [, j]]) --> number of characters that start in the
** range [i,j], or nil + current position if 's' is not well formed in
** that interval
*/
unsafe extern "C" fn utflen(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: libc::c_int = 0i32;
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut posi: lua_Integer = u_posrelat(luaL_optinteger(L, 2i32, 1i32 as lua_Integer), len);
    let mut posj: lua_Integer = u_posrelat(luaL_optinteger(L, 3i32, -1i32 as lua_Integer), len);
    (1i32 as libc::c_longlong <= posi && {
        posi -= 1;
        posi <= len as lua_Integer
    }
        || 0 != luaL_argerror(
            L,
            2i32,
            b"initial position out of string\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    posj -= 1;
    (posj < len as lua_Integer
        || 0 != luaL_argerror(
            L,
            3i32,
            b"final position out of string\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    while posi <= posj {
        let mut s1: *const libc::c_char =
            utf8_decode(s.offset(posi as isize), 0 as *mut libc::c_int);
        if s1.is_null() {
            /* conversion error? */
            /* return nil ... */
            lua_pushnil(L);
            /* ... and current position */
            lua_pushinteger(L, posi + 1i32 as libc::c_longlong);
            return 2i32;
        } else {
            posi = s1.wrapping_offset_from(s) as libc::c_long as lua_Integer;
            n += 1
        }
    }
    lua_pushinteger(L, n as lua_Integer);
    return 1i32;
}
/*
** $Id: lutf8lib.c,v 1.15 2015/03/28 19:16:55 roberto Exp roberto $
** Standard library for UTF-8 manipulation
** See Copyright Notice in lua.h
*/
/* from strlib */
/* translate a relative string position: negative means back from end */
unsafe extern "C" fn u_posrelat(mut pos: lua_Integer, mut len: size_t) -> lua_Integer {
    if pos >= 0i32 as libc::c_longlong {
        return pos;
    } else if (0u32 as libc::c_ulong).wrapping_sub(pos as size_t) > len {
        return 0i32 as lua_Integer;
    } else {
        return len as lua_Integer + pos + 1i32 as libc::c_longlong;
    };
}
/*
** utfchar(n1, n2, ...)  -> char(n1)..char(n2)...
*/
unsafe extern "C" fn utfchar(mut L: *mut lua_State_0) -> libc::c_int {
    /* number of arguments */
    let mut n: libc::c_int = lua_gettop(L);
    /* optimize common case of single char */
    if n == 1i32 {
        pushutfchar(L, 1i32);
    } else {
        let mut i: libc::c_int = 0;
        let mut b: luaL_Buffer_0 = luaL_Buffer {
            b: 0 as *mut libc::c_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State_0,
            initb: [0; 23],
        };
        luaL_buffinit(L, &mut b);
        i = 1i32;
        while i <= n {
            pushutfchar(L, i);
            luaL_addvalue(&mut b);
            i += 1
        }
        luaL_pushresult(&mut b);
    }
    return 1i32;
}
unsafe extern "C" fn pushutfchar(mut L: *mut lua_State_0, mut arg: libc::c_int) -> () {
    let mut code: lua_Integer = luaL_checkinteger(L, arg);
    (0i32 as libc::c_longlong <= code && code <= 0x10ffffi32 as libc::c_longlong
        || 0 != luaL_argerror(
            L,
            arg,
            b"value out of range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    lua_pushfstring(
        L,
        b"%U\x00" as *const u8 as *const libc::c_char,
        code as libc::c_long,
    );
}
/*
** codepoint(s, [i, [j]])  -> returns codepoints for all characters
** that start in the range [i,j]
*/
unsafe extern "C" fn codepoint(mut L: *mut lua_State_0) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut posi: lua_Integer = u_posrelat(luaL_optinteger(L, 2i32, 1i32 as lua_Integer), len);
    let mut pose: lua_Integer = u_posrelat(luaL_optinteger(L, 3i32, posi), len);
    let mut n: libc::c_int = 0;
    let mut se: *const libc::c_char = 0 as *const libc::c_char;
    (posi >= 1i32 as libc::c_longlong
        || 0 != luaL_argerror(
            L,
            2i32,
            b"out of range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    (pose <= len as lua_Integer
        || 0 != luaL_argerror(
            L,
            3i32,
            b"out of range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    if posi > pose {
        /* empty interval; return no values */
        return 0i32;
    } else if pose - posi >= 2147483647i32 as libc::c_longlong {
        return luaL_error(
            L,
            b"string slice too long\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        n = (pose - posi) as libc::c_int + 1i32;
        luaL_checkstack(
            L,
            n,
            b"string slice too long\x00" as *const u8 as *const libc::c_char,
        );
        n = 0i32;
        se = s.offset(pose as isize);
        s = s.offset((posi - 1i32 as libc::c_longlong) as isize);
        while s < se {
            let mut code: libc::c_int = 0;
            s = utf8_decode(s, &mut code);
            if s.is_null() {
                return luaL_error(
                    L,
                    b"invalid UTF-8 code\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                lua_pushinteger(L, code as lua_Integer);
                n += 1
            }
        }
        return n;
    };
}
/*
** offset(s, n, [i])  -> index where n-th character counting from
**   position 'i' starts; 0 means character at 'i'.
*/
unsafe extern "C" fn byteoffset(mut L: *mut lua_State_0) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut n: lua_Integer = luaL_checkinteger(L, 2i32);
    let mut posi: lua_Integer = (if n >= 0i32 as libc::c_longlong {
        1i32 as libc::c_ulong
    } else {
        len.wrapping_add(1i32 as libc::c_ulong)
    }) as lua_Integer;
    posi = u_posrelat(luaL_optinteger(L, 3i32, posi), len);
    (1i32 as libc::c_longlong <= posi && {
        posi -= 1;
        posi <= len as lua_Integer
    }
        || 0 != luaL_argerror(
            L,
            3i32,
            b"position out of range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    if n == 0i32 as libc::c_longlong {
        /* find beginning of current byte sequence */
        while posi > 0i32 as libc::c_longlong
            && *s.offset(posi as isize) as libc::c_int & 0xc0i32 == 0x80i32
        {
            posi -= 1
        }
    } else {
        if *s.offset(posi as isize) as libc::c_int & 0xc0i32 == 0x80i32 {
            luaL_error(
                L,
                b"initial position is a continuation byte\x00" as *const u8 as *const libc::c_char,
            );
        }
        if n < 0i32 as libc::c_longlong {
            while n < 0i32 as libc::c_longlong && posi > 0i32 as libc::c_longlong {
                /* move back */
                loop {
                    /* find beginning of previous character */
                    posi -= 1;
                    if !(posi > 0i32 as libc::c_longlong
                        && *s.offset(posi as isize) as libc::c_int & 0xc0i32 == 0x80i32)
                    {
                        break;
                    }
                }
                n += 1
            }
        } else {
            /* do not move for 1st character */
            n -= 1;
            while n > 0i32 as libc::c_longlong && posi < len as lua_Integer {
                loop {
                    /* find beginning of next character */
                    posi += 1;
                    if !(*s.offset(posi as isize) as libc::c_int & 0xc0i32 == 0x80i32) {
                        break;
                    }
                }
                /* (cannot pass final '\0') */
                n -= 1
            }
        }
    }
    /* did it find given character? */
    if n == 0i32 as libc::c_longlong {
        lua_pushinteger(L, posi + 1i32 as libc::c_longlong);
    } else {
        /* no such character */
        lua_pushnil(L);
    }
    return 1i32;
}
