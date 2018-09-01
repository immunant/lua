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
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State_0, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_xmove(from: *mut lua_State_0, to: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_iscfunction(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(
        L: *mut lua_State_0,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_tothread(L: *mut lua_State_0, idx: libc::c_int) -> *mut lua_State_0;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State_0, p: *mut libc::c_void) -> ();
    #[no_mangle]
    fn lua_pushthread(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgetp(L: *mut lua_State_0, idx: libc::c_int, p: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State_0, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_getuservalue(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_rawset(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rawsetp(L: *mut lua_State_0, idx: libc::c_int, p: *const libc::c_void) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State_0, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_setuservalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_callk(
        L: *mut lua_State_0,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ();
    #[no_mangle]
    fn lua_pcallk(
        L: *mut lua_State_0,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State_0, level: libc::c_int, ar: *mut lua_Debug) -> libc::c_int;
    #[no_mangle]
    fn lua_getinfo(
        L: *mut lua_State_0,
        what: *const libc::c_char,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_getlocal(
        L: *mut lua_State_0,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_setlocal(
        L: *mut lua_State_0,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_getupvalue(
        L: *mut lua_State_0,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_setupvalue(
        L: *mut lua_State_0,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_upvalueid(L: *mut lua_State_0, fidx: libc::c_int, n: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_upvaluejoin(
        L: *mut lua_State_0,
        fidx1: libc::c_int,
        n1: libc::c_int,
        fidx2: libc::c_int,
        n2: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn lua_sethook(
        L: *mut lua_State_0,
        func: lua_Hook,
        mask: libc::c_int,
        count: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn lua_gethook(L: *mut lua_State_0) -> lua_Hook;
    #[no_mangle]
    fn lua_gethookmask(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_gethookcount(L: *mut lua_State_0) -> libc::c_int;
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
    fn luaL_optlstring(
        L: *mut lua_State_0,
        arg: libc::c_int,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State_0, arg: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State_0, arg: libc::c_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State_0, arg: libc::c_int, t: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State_0, arg: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_loadbufferx(
        L: *mut lua_State_0,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_traceback(
        L: *mut lua_State_0,
        L1: *mut lua_State_0,
        msg: *const libc::c_char,
        level: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> *const libc::c_char;
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
/* only for Lua functions */
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
pub unsafe extern "C" fn luaopen_debug(mut L: *mut lua_State_0) -> libc::c_int {
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
        (::std::mem::size_of::<[luaL_Reg_0; 17]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg_0>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, dblib.as_ptr(), 0i32);
    return 1i32;
}
static mut dblib: [luaL_Reg_0; 17] = unsafe {
    [
        luaL_Reg {
            name: b"debug\x00" as *const u8 as *const libc::c_char,
            func: Some(db_debug),
        },
        luaL_Reg {
            name: b"getuservalue\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getuservalue),
        },
        luaL_Reg {
            name: b"gethook\x00" as *const u8 as *const libc::c_char,
            func: Some(db_gethook),
        },
        luaL_Reg {
            name: b"getinfo\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getinfo),
        },
        luaL_Reg {
            name: b"getlocal\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getlocal),
        },
        luaL_Reg {
            name: b"getregistry\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getregistry),
        },
        luaL_Reg {
            name: b"getmetatable\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getmetatable),
        },
        luaL_Reg {
            name: b"getupvalue\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getupvalue),
        },
        luaL_Reg {
            name: b"upvaluejoin\x00" as *const u8 as *const libc::c_char,
            func: Some(db_upvaluejoin),
        },
        luaL_Reg {
            name: b"upvalueid\x00" as *const u8 as *const libc::c_char,
            func: Some(db_upvalueid),
        },
        luaL_Reg {
            name: b"setuservalue\x00" as *const u8 as *const libc::c_char,
            func: Some(db_setuservalue),
        },
        luaL_Reg {
            name: b"sethook\x00" as *const u8 as *const libc::c_char,
            func: Some(db_sethook),
        },
        luaL_Reg {
            name: b"setlocal\x00" as *const u8 as *const libc::c_char,
            func: Some(db_setlocal),
        },
        luaL_Reg {
            name: b"setmetatable\x00" as *const u8 as *const libc::c_char,
            func: Some(db_setmetatable),
        },
        luaL_Reg {
            name: b"setupvalue\x00" as *const u8 as *const libc::c_char,
            func: Some(db_setupvalue),
        },
        luaL_Reg {
            name: b"traceback\x00" as *const u8 as *const libc::c_char,
            func: Some(db_traceback),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn db_traceback(mut L: *mut lua_State_0) -> libc::c_int {
    let mut level: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State_0 = getthread(L, &mut arg);
    let mut msg: *const libc::c_char = lua_tolstring(L, arg + 1i32, 0 as *mut size_t);
    /* non-string 'msg'? */
    if msg.is_null() && !(lua_type(L, arg + 1i32) <= 0i32) {
        /* return it untouched */
        lua_pushvalue(L, arg + 1i32);
    } else {
        level = luaL_optinteger(
            L,
            arg + 2i32,
            (if L == L1 { 1i32 } else { 0i32 }) as lua_Integer,
        ) as libc::c_int;
        luaL_traceback(L, L1, msg, level);
    }
    return 1i32;
}
/*
** Auxiliary function used by several library functions: check for
** an optional thread as function's first argument and set 'arg' with
** 1 if this argument is present (so that functions can skip it to
** access their other arguments)
*/
unsafe extern "C" fn getthread(
    mut L: *mut lua_State_0,
    mut arg: *mut libc::c_int,
) -> *mut lua_State_0 {
    if lua_type(L, 1i32) == 8i32 {
        *arg = 1i32;
        return lua_tothread(L, 1i32);
    } else {
        *arg = 0i32;
        /* function will operate over current thread */
        return L;
    };
}
unsafe extern "C" fn db_setupvalue(mut L: *mut lua_State_0) -> libc::c_int {
    luaL_checkany(L, 3i32);
    return auxupvalue(L, 0i32);
}
/*
** get (if 'get' is true) or set an upvalue from a closure
*/
unsafe extern "C" fn auxupvalue(mut L: *mut lua_State_0, mut get: libc::c_int) -> libc::c_int {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* upvalue index */
    let mut n: libc::c_int = luaL_checkinteger(L, 2i32) as libc::c_int;
    /* closure */
    luaL_checktype(L, 1i32, 6i32);
    name = if 0 != get {
        lua_getupvalue(L, 1i32, n)
    } else {
        lua_setupvalue(L, 1i32, n)
    };
    if name.is_null() {
        return 0i32;
    } else {
        lua_pushstring(L, name);
        lua_rotate(L, -(get + 1i32), 1i32);
        /* no-op if get is false */
        return get + 1i32;
    };
}
unsafe extern "C" fn db_setmetatable(mut L: *mut lua_State_0) -> libc::c_int {
    let mut t: libc::c_int = lua_type(L, 2i32);
    (t == 0i32 || t == 5i32
        || 0 != luaL_argerror(
            L,
            2i32,
            b"nil or table expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    lua_settop(L, 2i32);
    lua_setmetatable(L, 1i32);
    /* return 1st argument */
    return 1i32;
}
unsafe extern "C" fn db_setlocal(mut L: *mut lua_State_0) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut L1: *mut lua_State_0 = getthread(L, &mut arg);
    let mut ar: lua_Debug = lua_Debug_0 {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut level: libc::c_int = luaL_checkinteger(L, arg + 1i32) as libc::c_int;
    let mut nvar: libc::c_int = luaL_checkinteger(L, arg + 2i32) as libc::c_int;
    /* out of range? */
    if 0 == lua_getstack(L1, level, &mut ar) {
        return luaL_argerror(
            L,
            arg + 1i32,
            b"level out of range\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        luaL_checkany(L, arg + 3i32);
        lua_settop(L, arg + 3i32);
        checkstack(L, L1, 1i32);
        lua_xmove(L, L1, 1i32);
        name = lua_setlocal(L1, &mut ar, nvar);
        if name.is_null() {
            lua_settop(L1, -1i32 - 1i32);
        }
        /* pop value (if not popped by 'lua_setlocal') */
        lua_pushstring(L, name);
        return 1i32;
    };
}
/*
** If L1 != L, L1 can be in any state, and therefore there are no
** guarantees about its stack space; any push in L1 must be
** checked.
*/
unsafe extern "C" fn checkstack(
    mut L: *mut lua_State_0,
    mut L1: *mut lua_State_0,
    mut n: libc::c_int,
) -> () {
    if L != L1 && 0 == lua_checkstack(L1, n) {
        luaL_error(L, b"stack overflow\x00" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn db_sethook(mut L: *mut lua_State_0) -> libc::c_int {
    let mut smask: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut func: lua_Hook = None;
    let mut L1: *mut lua_State_0 = getthread(L, &mut arg);
    if lua_type(L, arg + 1i32) <= 0i32 {
        /* no hook? */
        lua_settop(L, arg + 1i32);
        func = None;
        mask = 0i32;
        /* turn off hooks */
        count = 0i32
    } else {
        smask = luaL_checklstring(L, arg + 2i32, 0 as *mut size_t);
        luaL_checktype(L, arg + 1i32, 6i32);
        count = luaL_optinteger(L, arg + 3i32, 0i32 as lua_Integer) as libc::c_int;
        func = Some(hookf);
        mask = makemask(smask, count)
    }
    if lua_rawgetp(
        L,
        -50000i32 - 1000i32,
        &HOOKKEY as *const libc::c_int as *const libc::c_void,
    ) == 0i32
    {
        /* create a hook table */
        lua_createtable(L, 0i32, 2i32);
        lua_pushvalue(L, -1i32);
        /* set it in position */
        lua_rawsetp(
            L,
            -50000i32 - 1000i32,
            &HOOKKEY as *const libc::c_int as *const libc::c_void,
        );
        lua_pushstring(L, b"k\x00" as *const u8 as *const libc::c_char);
        /* * hooktable.__mode = "k" */
        lua_setfield(L, -2i32, b"__mode\x00" as *const u8 as *const libc::c_char);
        lua_pushvalue(L, -1i32);
        /* setmetatable(hooktable) = hooktable */
        lua_setmetatable(L, -2i32);
    }
    checkstack(L, L1, 1i32);
    lua_pushthread(L1);
    /* key (thread) */
    lua_xmove(L1, L, 1i32);
    /* value (hook function) */
    lua_pushvalue(L, arg + 1i32);
    /* hooktable[L1] = new Lua hook */
    lua_rawset(L, -3i32);
    lua_sethook(L1, func, mask, count);
    return 0i32;
}
/*
** $Id: ldblib.c,v 1.150 2015/11/19 19:16:22 roberto Exp roberto $
** Interface from Lua to its debug API
** See Copyright Notice in lua.h
*/
/*
** The hook table at registry[&HOOKKEY] maps threads to their current
** hook function. (We only need the unique address of 'HOOKKEY'.)
*/
static mut HOOKKEY: libc::c_int = unsafe { 0i32 };
/* call hook function */
/*
** Convert a string mask (for 'sethook') into a bit mask
*/
unsafe extern "C" fn makemask(
    mut smask: *const libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut mask: libc::c_int = 0i32;
    if !strchr(smask, 'c' as i32).is_null() {
        mask |= 1i32 << 0i32
    }
    if !strchr(smask, 'r' as i32).is_null() {
        mask |= 1i32 << 1i32
    }
    if !strchr(smask, 'l' as i32).is_null() {
        mask |= 1i32 << 2i32
    }
    if count > 0i32 {
        mask |= 1i32 << 3i32
    }
    return mask;
}
/*
** Call hook function registered at hook table for the current
** thread (if there is one)
*/
unsafe extern "C" fn hookf(mut L: *mut lua_State_0, mut ar: *mut lua_Debug) -> () {
    static mut hooknames: [*const libc::c_char; 5] = unsafe {
        [
            b"call\x00" as *const u8 as *const libc::c_char,
            b"return\x00" as *const u8 as *const libc::c_char,
            b"line\x00" as *const u8 as *const libc::c_char,
            b"count\x00" as *const u8 as *const libc::c_char,
            b"tail call\x00" as *const u8 as *const libc::c_char,
        ]
    };
    lua_rawgetp(
        L,
        -50000i32 - 1000i32,
        &HOOKKEY as *const libc::c_int as *const libc::c_void,
    );
    lua_pushthread(L);
    if lua_rawget(L, -2i32) == 6i32 {
        /* is there a hook function? */
        /* push event name */
        lua_pushstring(L, hooknames[(*ar).event as usize]);
        if (*ar).currentline >= 0i32 {
            /* push current line */
            lua_pushinteger(L, (*ar).currentline as lua_Integer);
        } else {
            lua_pushnil(L);
        }
        if 0 != lua_getinfo(L, b"lS\x00" as *const u8 as *const libc::c_char, ar) {
        } else {
            __assert_fail(
                b"lua_getinfo(L, \"lS\", ar)\x00" as *const u8 as *const libc::c_char,
                b"ldblib.c\x00" as *const u8 as *const libc::c_char,
                316i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"void hookf(lua_State *, lua_Debug *)\x00",
                )).as_ptr(),
            );
        };
        lua_callk(L, 2i32, 0i32, 0i32 as lua_KContext, None);
    };
}
unsafe extern "C" fn db_setuservalue(mut L: *mut lua_State_0) -> libc::c_int {
    luaL_checktype(L, 1i32, 7i32);
    luaL_checkany(L, 2i32);
    lua_settop(L, 2i32);
    lua_setuservalue(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn db_upvalueid(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: libc::c_int = checkupval(L, 1i32, 2i32);
    lua_pushlightuserdata(L, lua_upvalueid(L, 1i32, n));
    return 1i32;
}
/*
** Check whether a given upvalue from a given closure exists and
** returns its index
*/
unsafe extern "C" fn checkupval(
    mut L: *mut lua_State_0,
    mut argf: libc::c_int,
    mut argnup: libc::c_int,
) -> libc::c_int {
    /* upvalue index */
    let mut nup: libc::c_int = luaL_checkinteger(L, argnup) as libc::c_int;
    /* closure */
    luaL_checktype(L, argf, 6i32);
    (!lua_getupvalue(L, argf, nup).is_null()
        || 0 != luaL_argerror(
            L,
            argnup,
            b"invalid upvalue index\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    return nup;
}
unsafe extern "C" fn db_upvaluejoin(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n1: libc::c_int = checkupval(L, 1i32, 2i32);
    let mut n2: libc::c_int = checkupval(L, 3i32, 4i32);
    (0 == lua_iscfunction(L, 1i32)
        || 0 != luaL_argerror(
            L,
            1i32,
            b"Lua function expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    (0 == lua_iscfunction(L, 3i32)
        || 0 != luaL_argerror(
            L,
            3i32,
            b"Lua function expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    lua_upvaluejoin(L, 1i32, n1, 3i32, n2);
    return 0i32;
}
unsafe extern "C" fn db_getupvalue(mut L: *mut lua_State_0) -> libc::c_int {
    return auxupvalue(L, 1i32);
}
unsafe extern "C" fn db_getmetatable(mut L: *mut lua_State_0) -> libc::c_int {
    luaL_checkany(L, 1i32);
    if 0 == lua_getmetatable(L, 1i32) {
        /* no metatable */
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn db_getregistry(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushvalue(L, -50000i32 - 1000i32);
    return 1i32;
}
unsafe extern "C" fn db_getlocal(mut L: *mut lua_State_0) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State_0 = getthread(L, &mut arg);
    let mut ar: lua_Debug = lua_Debug_0 {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* local-variable index */
    let mut nvar: libc::c_int = luaL_checkinteger(L, arg + 2i32) as libc::c_int;
    if lua_type(L, arg + 1i32) == 6i32 {
        /* function argument? */
        /* push function */
        lua_pushvalue(L, arg + 1i32);
        /* push local name */
        lua_pushstring(L, lua_getlocal(L, 0 as *const lua_Debug, nvar));
        /* return only name (there is no value) */
        return 1i32;
    } else {
        /* stack-level argument */
        let mut level: libc::c_int = luaL_checkinteger(L, arg + 1i32) as libc::c_int;
        /* out of range? */
        if 0 == lua_getstack(L1, level, &mut ar) {
            return luaL_argerror(
                L,
                arg + 1i32,
                b"level out of range\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            checkstack(L, L1, 1i32);
            name = lua_getlocal(L1, &mut ar, nvar);
            if !name.is_null() {
                /* move local value */
                lua_xmove(L1, L, 1i32);
                /* push name */
                lua_pushstring(L, name);
                /* re-order */
                lua_rotate(L, -2i32, 1i32);
                return 2i32;
            } else {
                /* no name (nor value) */
                lua_pushnil(L);
                return 1i32;
            }
        }
    };
}
/*
** Calls 'lua_getinfo' and collects all results in a new table.
** L1 needs stack space for an optional input (function) plus
** two optional outputs (function and line table) from function
** 'lua_getinfo'.
*/
unsafe extern "C" fn db_getinfo(mut L: *mut lua_State_0) -> libc::c_int {
    let mut ar: lua_Debug = lua_Debug_0 {
        event: 0,
        name: 0 as *const libc::c_char,
        namewhat: 0 as *const libc::c_char,
        what: 0 as *const libc::c_char,
        source: 0 as *const libc::c_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State_0 = getthread(L, &mut arg);
    let mut options: *const libc::c_char = luaL_optlstring(
        L,
        arg + 2i32,
        b"flnStu\x00" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    checkstack(L, L1, 3i32);
    if lua_type(L, arg + 1i32) == 6i32 {
        /* info about a function? */
        /* add '>' to 'options' */
        options = lua_pushfstring(L, b">%s\x00" as *const u8 as *const libc::c_char, options);
        /* move function to 'L1' stack */
        lua_pushvalue(L, arg + 1i32);
        lua_xmove(L, L1, 1i32);
    } else if 0 == lua_getstack(L1, luaL_checkinteger(L, arg + 1i32) as libc::c_int, &mut ar) {
        /* level out of range */
        lua_pushnil(L);
        return 1i32;
    }
    if 0 == lua_getinfo(L1, options, &mut ar) {
        return luaL_argerror(
            L,
            arg + 2i32,
            b"invalid option\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        lua_createtable(L, 0i32, 0i32);
        /* table to collect results */
        if !strchr(options, 'S' as i32).is_null() {
            settabss(
                L,
                b"source\x00" as *const u8 as *const libc::c_char,
                ar.source,
            );
            settabss(
                L,
                b"short_src\x00" as *const u8 as *const libc::c_char,
                ar.short_src.as_mut_ptr(),
            );
            settabsi(
                L,
                b"linedefined\x00" as *const u8 as *const libc::c_char,
                ar.linedefined,
            );
            settabsi(
                L,
                b"lastlinedefined\x00" as *const u8 as *const libc::c_char,
                ar.lastlinedefined,
            );
            settabss(L, b"what\x00" as *const u8 as *const libc::c_char, ar.what);
        }
        if !strchr(options, 'l' as i32).is_null() {
            settabsi(
                L,
                b"currentline\x00" as *const u8 as *const libc::c_char,
                ar.currentline,
            );
        }
        if !strchr(options, 'u' as i32).is_null() {
            settabsi(
                L,
                b"nups\x00" as *const u8 as *const libc::c_char,
                ar.nups as libc::c_int,
            );
            settabsi(
                L,
                b"nparams\x00" as *const u8 as *const libc::c_char,
                ar.nparams as libc::c_int,
            );
            settabsb(
                L,
                b"isvararg\x00" as *const u8 as *const libc::c_char,
                ar.isvararg as libc::c_int,
            );
        }
        if !strchr(options, 'n' as i32).is_null() {
            settabss(L, b"name\x00" as *const u8 as *const libc::c_char, ar.name);
            settabss(
                L,
                b"namewhat\x00" as *const u8 as *const libc::c_char,
                ar.namewhat,
            );
        }
        if !strchr(options, 't' as i32).is_null() {
            settabsb(
                L,
                b"istailcall\x00" as *const u8 as *const libc::c_char,
                ar.istailcall as libc::c_int,
            );
        }
        if !strchr(options, 'L' as i32).is_null() {
            treatstackoption(
                L,
                L1,
                b"activelines\x00" as *const u8 as *const libc::c_char,
            );
        }
        if !strchr(options, 'f' as i32).is_null() {
            treatstackoption(L, L1, b"func\x00" as *const u8 as *const libc::c_char);
        }
        /* return table */
        return 1i32;
    };
}
/*
** In function 'db_getinfo', the call to 'lua_getinfo' may push
** results on the stack; later it creates the result table to put
** these objects. Function 'treatstackoption' puts the result from
** 'lua_getinfo' on top of the result table so that it can call
** 'lua_setfield'.
*/
unsafe extern "C" fn treatstackoption(
    mut L: *mut lua_State_0,
    mut L1: *mut lua_State_0,
    mut fname: *const libc::c_char,
) -> () {
    if L == L1 {
        /* exchange object and table */
        lua_rotate(L, -2i32, 1i32);
    } else {
        /* move object to the "main" stack */
        lua_xmove(L1, L, 1i32);
    }
    /* put object into table */
    lua_setfield(L, -2i32, fname);
}
unsafe extern "C" fn settabsb(
    mut L: *mut lua_State_0,
    mut k: *const libc::c_char,
    mut v: libc::c_int,
) -> () {
    lua_pushboolean(L, v);
    lua_setfield(L, -2i32, k);
}
/*
** Variations of 'lua_settable', used by 'db_getinfo' to put results
** from 'lua_getinfo' into result table. Key is always a string;
** value can be a string, an int, or a boolean.
*/
unsafe extern "C" fn settabss(
    mut L: *mut lua_State_0,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
) -> () {
    lua_pushstring(L, v);
    lua_setfield(L, -2i32, k);
}
unsafe extern "C" fn settabsi(
    mut L: *mut lua_State_0,
    mut k: *const libc::c_char,
    mut v: libc::c_int,
) -> () {
    lua_pushinteger(L, v as lua_Integer);
    lua_setfield(L, -2i32, k);
}
unsafe extern "C" fn db_gethook(mut L: *mut lua_State_0) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State_0 = getthread(L, &mut arg);
    let mut buff: [libc::c_char; 5] = [0; 5];
    let mut mask: libc::c_int = lua_gethookmask(L1);
    let mut hook: lua_Hook = lua_gethook(L1);
    /* no hook? */
    if hook.is_none() {
        lua_pushnil(L);
    } else if hook != Some(hookf) {
        lua_pushstring(L, b"external hook\x00" as *const u8 as *const libc::c_char);
    } else {
        /* hook table must exist */
        lua_rawgetp(
            L,
            -50000i32 - 1000i32,
            &HOOKKEY as *const libc::c_int as *const libc::c_void,
        );
        checkstack(L, L1, 1i32);
        lua_pushthread(L1);
        lua_xmove(L1, L, 1i32);
        /* 1st result = hooktable[L1] */
        lua_rawget(L, -2i32);
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    }
    /* remove hook table */
    /* 2nd result = mask */
    lua_pushstring(L, unmakemask(mask, buff.as_mut_ptr()));
    /* 3rd result = count */
    lua_pushinteger(L, lua_gethookcount(L1) as lua_Integer);
    return 3i32;
}
/*
** Convert a bit mask (for 'gethook') into a string mask
*/
unsafe extern "C" fn unmakemask(
    mut mask: libc::c_int,
    mut smask: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0i32;
    if 0 != mask & 1i32 << 0i32 {
        let fresh0 = i;
        i = i + 1;
        *smask.offset(fresh0 as isize) = 'c' as i32 as libc::c_char
    }
    if 0 != mask & 1i32 << 1i32 {
        let fresh1 = i;
        i = i + 1;
        *smask.offset(fresh1 as isize) = 'r' as i32 as libc::c_char
    }
    if 0 != mask & 1i32 << 2i32 {
        let fresh2 = i;
        i = i + 1;
        *smask.offset(fresh2 as isize) = 'l' as i32 as libc::c_char
    }
    *smask.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    return smask;
}
unsafe extern "C" fn db_getuservalue(mut L: *mut lua_State_0) -> libc::c_int {
    if lua_type(L, 1i32) != 7i32 {
        lua_pushnil(L);
    } else {
        lua_getuservalue(L, 1i32);
    }
    return 1i32;
}
unsafe extern "C" fn db_debug(mut L: *mut lua_State_0) -> libc::c_int {
    loop {
        let mut buffer: [libc::c_char; 250] = [0; 250];
        fprintf(
            stderr,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"lua_debug> \x00" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
        if fgets(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 250]>() as libc::c_ulong as libc::c_int,
            stdin,
        ).is_null()
            || strcmp(
                buffer.as_mut_ptr(),
                b"cont\n\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
        {
            return 0i32;
        } else {
            if 0 != luaL_loadbufferx(
                L,
                buffer.as_mut_ptr(),
                strlen(buffer.as_mut_ptr()),
                b"=(debug command)\x00" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            ) || 0 != lua_pcallk(L, 0i32, 0i32, 0i32, 0i32 as lua_KContext, None)
            {
                fprintf(
                    stderr,
                    b"%s\n\x00" as *const u8 as *const libc::c_char,
                    lua_tolstring(L, -1i32, 0 as *mut size_t),
                );
                fflush(stderr);
            }
            /* remove eventual returns */
            lua_settop(L, 0i32);
        }
    }
}
