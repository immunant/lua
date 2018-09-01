use lstate::{CallInfo, lua_State, global_State};
use lobject::{TValue, lua_TValue, Value, GCObject};

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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State_0, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State_0, tp: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
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
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_geti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State_0, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_seti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_callk(
        L: *mut lua_State_0,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ();
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
    fn luaL_len(L: *mut lua_State_0, idx: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State_0, B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_addlstring(B: *mut luaL_Buffer_0, s: *const libc::c_char, l: size_t) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    static mut tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut daylight: libc::c_int;
    #[no_mangle]
    static mut timezone: libc::c_long;
    #[no_mangle]
    static mut getdate_err: libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
/* }====================================================== */
/*
** {======================================================
** Quicksort
** (based on 'Algorithms in MODULA-3', Robert Sedgewick;
**  Addison-Wesley, 1993.)
** =======================================================
*/
/* type for array indices */
pub type IdxT = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
/* unsigned integer type */
pub type lua_Unsigned = libc::c_ulonglong;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
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
** $Id: lauxlib.h,v 1.130 2016/12/04 20:17:24 roberto Exp roberto $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/
/* extra error code for 'luaL_loadfilex' */
/* key, in the registry, for table of loaded modules */
/* key, in the registry, for table of preloaded loaders */
pub type luaL_Reg = luaL_Reg_0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg_0 {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
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
#[no_mangle]
pub unsafe extern "C" fn luaopen_table(mut L: *mut lua_State_0) -> libc::c_int {
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
        (::std::mem::size_of::<[luaL_Reg; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, tab_funcs.as_ptr(), 0i32);
    return 1i32;
}
/* }====================================================== */
static mut tab_funcs: [luaL_Reg; 8] = unsafe {
    [
        luaL_Reg_0 {
            name: b"concat\x00" as *const u8 as *const libc::c_char,
            func: Some(tconcat),
        },
        luaL_Reg_0 {
            name: b"insert\x00" as *const u8 as *const libc::c_char,
            func: Some(tinsert),
        },
        luaL_Reg_0 {
            name: b"pack\x00" as *const u8 as *const libc::c_char,
            func: Some(pack),
        },
        luaL_Reg_0 {
            name: b"unpack\x00" as *const u8 as *const libc::c_char,
            func: Some(unpack),
        },
        luaL_Reg_0 {
            name: b"remove\x00" as *const u8 as *const libc::c_char,
            func: Some(tremove),
        },
        luaL_Reg_0 {
            name: b"move\x00" as *const u8 as *const libc::c_char,
            func: Some(tmove),
        },
        luaL_Reg_0 {
            name: b"sort\x00" as *const u8 as *const libc::c_char,
            func: Some(sort),
        },
        luaL_Reg_0 {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
/* tail call auxsort(L, lo, up, rnd) */
unsafe extern "C" fn sort(mut L: *mut lua_State_0) -> libc::c_int {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut n: lua_Integer = luaL_len(L, 1i32);
    if n > 1i32 as libc::c_longlong {
        /* non-trivial interval? */
        (n < 2147483647i32 as libc::c_longlong
            || 0 != luaL_argerror(
                L,
                1i32,
                b"array too big\x00" as *const u8 as *const libc::c_char,
            )) as libc::c_int;
        /* is there a 2nd argument? */
        if !(lua_type(L, 2i32) <= 0i32) {
            /* must be a function */
            luaL_checktype(L, 2i32, 6i32);
        }
        /* make sure there are two arguments */
        lua_settop(L, 2i32);
        auxsort(L, 1i32 as IdxT, n as IdxT, 0i32 as libc::c_uint);
    }
    return 0i32;
}
/*
** Check that 'arg' either is a table or can behave like one (that is,
** has a metatable with the required metamethods)
*/
unsafe extern "C" fn checktab(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut what: libc::c_int,
) -> () {
    if lua_type(L, arg) != 5i32 {
        /* is it not a table? */
        /* number of elements to pop */
        let mut n: libc::c_int = 1i32;
        /* must have metatable */
        if 0 != lua_getmetatable(L, arg) && (0 == what & 1i32 || {
            n += 1;
            0 != checkfield(L, b"__index\x00" as *const u8 as *const libc::c_char, n)
        }) && (0 == what & 2i32 || {
            n += 1;
            0 != checkfield(L, b"__newindex\x00" as *const u8 as *const libc::c_char, n)
        }) && (0 == what & 4i32 || {
            n += 1;
            0 != checkfield(L, b"__len\x00" as *const u8 as *const libc::c_char, n)
        }) {
            lua_settop(L, -n - 1i32);
        } else {
            /* pop metatable and tested metamethods */
            /* force an error */
            luaL_checktype(L, arg, 5i32);
        }
    };
}
/*
** $Id: ltablib.c,v 1.92 2016/02/08 12:55:19 roberto Exp roberto $
** Library for Table Manipulation
** See Copyright Notice in lua.h
*/
/*
** Operations that an object must define to mimic a table
** (some functions only need some of them)
*/
/* read */
/* write */
/* length */
/* read/write */
unsafe extern "C" fn checkfield(
    mut L: *mut lua_State_0,
    mut key: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    lua_pushstring(L, key);
    return (lua_rawget(L, -n) != 0i32) as libc::c_int;
}
/*
** QuickSort algorithm (recursive function)
*/
unsafe extern "C" fn auxsort(
    mut L: *mut lua_State_0,
    mut lo: IdxT,
    mut up: IdxT,
    mut rnd: libc::c_uint,
) -> () {
    while lo < up {
        /* loop for tail recursion */
        /* Pivot index */
        let mut p: IdxT = 0;
        /* to be used later */
        let mut n: IdxT = 0;
        /* sort elements 'lo', 'p', and 'up' */
        lua_geti(L, 1i32, lo as lua_Integer);
        lua_geti(L, 1i32, up as lua_Integer);
        /* a[up] < a[lo]? */
        if 0 != sort_comp(L, -1i32, -2i32) {
            /* swap a[lo] - a[up] */
            set2(L, lo, up);
        } else {
            lua_settop(L, -2i32 - 1i32);
        }
        /* remove both values */
        /* only 2 elements? */
        if up.wrapping_sub(lo) == 1i32 as libc::c_uint {
            /* already sorted */
            return;
        } else {
            /* small interval or no randomize? */
            if up.wrapping_sub(lo) < 100u32 || rnd == 0i32 as libc::c_uint {
                /* middle element is a good pivot */
                p = lo.wrapping_add(up).wrapping_div(2i32 as libc::c_uint)
            } else {
                p = choosePivot(lo, up, rnd)
            }
            lua_geti(L, 1i32, p as lua_Integer);
            lua_geti(L, 1i32, lo as lua_Integer);
            /* a[p] < a[lo]? */
            if 0 != sort_comp(L, -2i32, -1i32) {
                /* swap a[p] - a[lo] */
                set2(L, p, lo);
            } else {
                lua_settop(L, -1i32 - 1i32);
                /* remove a[lo] */
                lua_geti(L, 1i32, up as lua_Integer);
                /* a[up] < a[p]? */
                if 0 != sort_comp(L, -1i32, -2i32) {
                    /* swap a[up] - a[p] */
                    set2(L, p, up);
                } else {
                    lua_settop(L, -2i32 - 1i32);
                }
            }
            /* only 3 elements? */
            if up.wrapping_sub(lo) == 2i32 as libc::c_uint {
                /* already sorted */
                return;
            } else {
                /* get middle element (Pivot) */
                lua_geti(L, 1i32, p as lua_Integer);
                /* push Pivot */
                lua_pushvalue(L, -1i32);
                /* push a[up - 1] */
                lua_geti(
                    L,
                    1i32,
                    up.wrapping_sub(1i32 as libc::c_uint) as lua_Integer,
                );
                /* swap Pivot (a[p]) with a[up - 1] */
                set2(L, p, up.wrapping_sub(1i32 as libc::c_uint));
                p = partition(L, lo, up);
                /* a[lo .. p - 1] <= a[p] == P <= a[p + 1 .. up] */
                if p.wrapping_sub(lo) < up.wrapping_sub(p) {
                    /* lower interval is smaller? */
                    /* call recursively for lower interval */
                    auxsort(L, lo, p.wrapping_sub(1i32 as libc::c_uint), rnd);
                    /* size of smaller interval */
                    n = p.wrapping_sub(lo);
                    /* tail call for [p + 1 .. up] (upper interval) */
                    lo = p.wrapping_add(1i32 as libc::c_uint)
                } else {
                    /* call recursively for upper interval */
                    auxsort(L, p.wrapping_add(1i32 as libc::c_uint), up, rnd);
                    /* size of smaller interval */
                    n = up.wrapping_sub(p);
                    /* tail call for [lo .. p - 1]  (lower interval) */
                    up = p.wrapping_sub(1i32 as libc::c_uint)
                }
                /* partition too imbalanced? */
                if !(up.wrapping_sub(lo).wrapping_div(128i32 as libc::c_uint) > n) {
                    continue;
                }
                /* try a new randomization */
                rnd = l_randomizePivot()
            }
        }
    }
}
/*
** Produce a "random" 'unsigned int' to randomize pivot choice. This
** macro is used only when 'sort' detects a big imbalance in the result
** of a partition. (If you don't want/need this "randomness", ~0 is a
** good choice.)
*/
/* { */
/* size of 'e' measured in number of 'unsigned int's */
/*
** Use 'time' and 'clock' as sources of "randomness". Because we don't
** know the types 'clock_t' and 'time_t', we cannot cast them to
** anything without risking overflows. A safe way to use their values
** is to copy them to an array of a known type and use the array values.
*/
unsafe extern "C" fn l_randomizePivot() -> libc::c_uint {
    let mut c: clock_t = clock();
    let mut t: time_t = time(0 as *mut time_t);
    let mut buff: [libc::c_uint; 4] = [0; 4];
    let mut i: libc::c_uint = 0;
    let mut rnd: libc::c_uint = 0i32 as libc::c_uint;
    memcpy(
        buff.as_mut_ptr() as *mut libc::c_void,
        &mut c as *mut clock_t as *const libc::c_void,
        (::std::mem::size_of::<clock_t>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    memcpy(
        buff.as_mut_ptr().offset(
            (::std::mem::size_of::<clock_t>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                as isize,
        ) as *mut libc::c_void,
        &mut t as *mut time_t as *const libc::c_void,
        (::std::mem::size_of::<time_t>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    {
        rnd = rnd.wrapping_add(buff[i as usize]);
        i = i.wrapping_add(1)
    }
    return rnd;
}
/*
** Does the partition: Pivot P is at the top of the stack.
** precondition: a[lo] <= P == a[up-1] <= a[up],
** so it only needs to do the partition from lo + 1 to up - 2.
** Pos-condition: a[lo .. i - 1] <= a[i] == P <= a[i + 1 .. up]
** returns 'i'.
*/
unsafe extern "C" fn partition(mut L: *mut lua_State_0, mut lo: IdxT, mut up: IdxT) -> IdxT {
    /* will be incremented before first use */
    let mut i: IdxT = lo;
    /* will be decremented before first use */
    let mut j: IdxT = up.wrapping_sub(1i32 as libc::c_uint);
    /* loop invariant: a[lo .. i] <= P <= a[j .. up] */
    loop {
        /* next loop: repeat ++i while a[i] < P */
        loop {
            i = i.wrapping_add(1);
            lua_geti(L, 1i32, i as lua_Integer);
            if !(0 != sort_comp(L, -1i32, -2i32)) {
                break;
            }
            /* a[i] < P  but a[up - 1] == P  ?? */
            if i == up.wrapping_sub(1i32 as libc::c_uint) {
                luaL_error(
                    L,
                    b"invalid order function for sorting\x00" as *const u8 as *const libc::c_char,
                );
            }
            lua_settop(L, -1i32 - 1i32);
        }
        /* remove a[i] */
        /* after the loop, a[i] >= P and a[lo .. i - 1] < P */
        /* next loop: repeat --j while P < a[j] */
        loop {
            j = j.wrapping_sub(1);
            lua_geti(L, 1i32, j as lua_Integer);
            if !(0 != sort_comp(L, -3i32, -1i32)) {
                break;
            }
            /* j < i  but  a[j] > P ?? */
            if j < i {
                luaL_error(
                    L,
                    b"invalid order function for sorting\x00" as *const u8 as *const libc::c_char,
                );
            }
            lua_settop(L, -1i32 - 1i32);
        }
        /* remove a[j] */
        /* after the loop, a[j] <= P and a[j + 1 .. up] >= P */
        if j < i {
            /* no elements out of place? */
            lua_settop(L, -1i32 - 1i32);
            /* a[lo .. i - 1] <= P <= a[j + 1 .. i .. up] */
            /* pop a[j] */
            /* swap pivot (a[up - 1]) with a[i] to satisfy pos-condition */
            set2(L, up.wrapping_sub(1i32 as libc::c_uint), i);
            return i;
        } else {
            /* otherwise, swap a[i] - a[j] to restore invariant and repeat */
            set2(L, i, j);
        }
    }
}
/* } */
/* arrays larger than 'RANLIMIT' may use randomized pivots */
unsafe extern "C" fn set2(mut L: *mut lua_State_0, mut i: IdxT, mut j: IdxT) -> () {
    lua_seti(L, 1i32, i as lua_Integer);
    lua_seti(L, 1i32, j as lua_Integer);
}
/*
** Return true iff value at stack index 'a' is less than the value at
** index 'b' (according to the order of the sort).
*/
unsafe extern "C" fn sort_comp(
    mut L: *mut lua_State_0,
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    /* no function? */
    if lua_type(L, 2i32) == 0i32 {
        /* a < b */
        return lua_compare(L, a, b, 1i32);
    } else {
        /* function */
        let mut res: libc::c_int = 0;
        /* push function */
        lua_pushvalue(L, 2i32);
        /* -1 to compensate function */
        lua_pushvalue(L, a - 1i32);
        /* -2 to compensate function and 'a' */
        lua_pushvalue(L, b - 2i32);
        lua_callk(L, 2i32, 1i32, 0i32 as lua_KContext, None);
        /* call function */
        /* get result */
        res = lua_toboolean(L, -1i32);
        lua_settop(L, -1i32 - 1i32);
        /* pop result */
        return res;
    };
}
/*
** Choose an element in the middle (2nd-3th quarters) of [lo,up]
** "randomized" by 'rnd'
*/
unsafe extern "C" fn choosePivot(mut lo: IdxT, mut up: IdxT, mut rnd: libc::c_uint) -> IdxT {
    /* range/4 */
    let mut r4: IdxT = up.wrapping_sub(lo).wrapping_div(4i32 as libc::c_uint);
    let mut p: IdxT = rnd
        .wrapping_rem(r4.wrapping_mul(2i32 as libc::c_uint))
        .wrapping_add(lo.wrapping_add(r4));
    if lo.wrapping_add(r4) <= p && p <= up.wrapping_sub(r4) {
    } else {
        __assert_fail(
            b"lo + r4 <= p && p <= up - r4\x00" as *const u8 as *const libc::c_char,
            b"ltablib.c\x00" as *const u8 as *const libc::c_char,
            350i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"IdxT choosePivot(IdxT, IdxT, unsigned int)\x00",
            )).as_ptr(),
        );
    };
    return p;
}
/*
** Copy elements (1[f], ..., 1[e]) into (tt[t], tt[t+1], ...). Whenever
** possible, copy in increasing order, which is better for rehashing.
** "possible" means destination after original range, or smaller
** than origin, or copying to another table.
*/
unsafe extern "C" fn tmove(mut L: *mut lua_State_0) -> libc::c_int {
    let mut f: lua_Integer = luaL_checkinteger(L, 2i32);
    let mut e: lua_Integer = luaL_checkinteger(L, 3i32);
    let mut t: lua_Integer = luaL_checkinteger(L, 4i32);
    /* destination table */
    let mut tt: libc::c_int = if !(lua_type(L, 5i32) <= 0i32) {
        5i32
    } else {
        1i32
    };
    checktab(L, 1i32, 1i32);
    checktab(L, tt, 2i32);
    if e >= f {
        /* otherwise, nothing to move */
        let mut n: lua_Integer = 0;
        let mut i: lua_Integer = 0;
        (f > 0i32 as libc::c_longlong || e < 9223372036854775807i64 + f
            || 0 != luaL_argerror(
                L,
                3i32,
                b"too many elements to move\x00" as *const u8 as *const libc::c_char,
            )) as libc::c_int;
        /* number of elements to move */
        n = e - f + 1i32 as libc::c_longlong;
        (t <= 9223372036854775807i64 - n + 1i32 as libc::c_longlong
            || 0 != luaL_argerror(
                L,
                4i32,
                b"destination wrap around\x00" as *const u8 as *const libc::c_char,
            )) as libc::c_int;
        if t > e || t <= f || tt != 1i32 && 0 == lua_compare(L, 1i32, tt, 0i32) {
            i = 0i32 as lua_Integer;
            while i < n {
                lua_geti(L, 1i32, f + i);
                lua_seti(L, tt, t + i);
                i += 1
            }
        } else {
            i = n - 1i32 as libc::c_longlong;
            while i >= 0i32 as libc::c_longlong {
                lua_geti(L, 1i32, f + i);
                lua_seti(L, tt, t + i);
                i -= 1
            }
        }
    }
    /* return destination table */
    lua_pushvalue(L, tt);
    return 1i32;
}
unsafe extern "C" fn tremove(mut L: *mut lua_State_0) -> libc::c_int {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut size: lua_Integer = luaL_len(L, 1i32);
    let mut pos: lua_Integer = luaL_optinteger(L, 2i32, size);
    /* validate 'pos' if given */
    if pos != size {
        (1i32 as libc::c_longlong <= pos && pos <= size + 1i32 as libc::c_longlong
            || 0 != luaL_argerror(
                L,
                1i32,
                b"position out of bounds\x00" as *const u8 as *const libc::c_char,
            )) as libc::c_int;
    }
    /* result = t[pos] */
    lua_geti(L, 1i32, pos);
    while pos < size {
        lua_geti(L, 1i32, pos + 1i32 as libc::c_longlong);
        /* t[pos] = t[pos + 1] */
        lua_seti(L, 1i32, pos);
        pos += 1
    }
    lua_pushnil(L);
    /* t[pos] = nil */
    lua_seti(L, 1i32, pos);
    return 1i32;
}
unsafe extern "C" fn unpack(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: lua_Unsigned = 0;
    let mut i: lua_Integer = luaL_optinteger(L, 2i32, 1i32 as lua_Integer);
    let mut e: lua_Integer = if lua_type(L, 3i32) <= 0i32 {
        luaL_len(L, 1i32)
    } else {
        luaL_checkinteger(L, 3i32)
    };
    if i > e {
        /* empty range */
        return 0i32;
    } else {
        /* number of elements minus 1 (avoid overflows) */
        n = (e as lua_Unsigned).wrapping_sub(i as libc::c_ulonglong);
        if n >= 2147483647i32 as libc::c_uint as libc::c_ulonglong || {
            n = n.wrapping_add(1);
            0 == lua_checkstack(L, n as libc::c_int)
        } {
            return luaL_error(
                L,
                b"too many results to unpack\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            while i < e {
                /* push arg[i..e - 1] (to avoid overflows) */
                lua_geti(L, 1i32, i);
                i += 1
            }
            /* push last element */
            lua_geti(L, 1i32, e);
            return n as libc::c_int;
        }
    };
}
/*
** {======================================================
** Pack/unpack
** =======================================================
*/
unsafe extern "C" fn pack(mut L: *mut lua_State_0) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* number of elements to pack */
    let mut n: libc::c_int = lua_gettop(L);
    /* create result table */
    lua_createtable(L, n, 1i32);
    lua_rotate(L, 1i32, 1i32);
    /* put it at index 1 */
    /* assign elements */
    i = n;
    while i >= 1i32 {
        lua_seti(L, 1i32, i as lua_Integer);
        i -= 1
    }
    lua_pushinteger(L, n as lua_Integer);
    /* t.n = number of elements */
    lua_setfield(L, 1i32, b"n\x00" as *const u8 as *const libc::c_char);
    /* return table */
    return 1i32;
}
unsafe extern "C" fn tinsert(mut L: *mut lua_State_0) -> libc::c_int {
    /* first empty element */
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut e: lua_Integer = luaL_len(L, 1i32) + 1i32 as libc::c_longlong;
    /* where to insert new element */
    let mut pos: lua_Integer = 0;
    match lua_gettop(L) {
        2 => {
            /* called with only 2 arguments */
            /* insert new element at the end */
            pos = e
        }
        3 => {
            let mut i: lua_Integer = 0;
            /* 2nd argument is the position */
            pos = luaL_checkinteger(L, 2i32);
            (1i32 as libc::c_longlong <= pos && pos <= e
                || 0 != luaL_argerror(
                    L,
                    2i32,
                    b"position out of bounds\x00" as *const u8 as *const libc::c_char,
                )) as libc::c_int;
            i = e;
            while i > pos {
                /* move up elements */
                lua_geti(L, 1i32, i - 1i32 as libc::c_longlong);
                /* t[i] = t[i - 1] */
                lua_seti(L, 1i32, i);
                i -= 1
            }
        }
        _ => {
            return luaL_error(
                L,
                b"wrong number of arguments to \'insert\'\x00" as *const u8 as *const libc::c_char,
            )
        }
    }
    /* t[pos] = v */
    lua_seti(L, 1i32, pos);
    return 0i32;
}
unsafe extern "C" fn tconcat(mut L: *mut lua_State_0) -> libc::c_int {
    let mut b: luaL_Buffer_0 = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State_0,
        initb: [0; 23],
    };
    checktab(L, 1i32, 1i32 | 4i32);
    let mut last: lua_Integer = luaL_len(L, 1i32);
    let mut lsep: size_t = 0;
    let mut sep: *const libc::c_char = luaL_optlstring(
        L,
        2i32,
        b"\x00" as *const u8 as *const libc::c_char,
        &mut lsep,
    );
    let mut i: lua_Integer = luaL_optinteger(L, 3i32, 1i32 as lua_Integer);
    last = luaL_optinteger(L, 4i32, last);
    luaL_buffinit(L, &mut b);
    while i < last {
        addfield(L, &mut b, i);
        luaL_addlstring(&mut b, sep, lsep);
        i += 1
    }
    /* add last value (if interval was not empty) */
    if i == last {
        addfield(L, &mut b, i);
    }
    luaL_pushresult(&mut b);
    return 1i32;
}
unsafe extern "C" fn addfield(
    mut L: *mut lua_State_0,
    mut b: *mut luaL_Buffer_0,
    mut i: lua_Integer,
) -> () {
    lua_geti(L, 1i32, i);
    if 0 == lua_isstring(L, -1i32) {
        luaL_error(
            L,
            b"invalid value (%s) at index %d in table for \'concat\'\x00" as *const u8
                as *const libc::c_char,
            lua_typename(L, lua_type(L, -1i32)),
            i,
        );
    }
    luaL_addvalue(b);
}
