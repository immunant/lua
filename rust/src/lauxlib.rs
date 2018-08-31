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
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE_0) -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn lua_newstate(f: lua_Alloc, ud: *mut libc::c_void) -> *mut lua_State_0;
    #[no_mangle]
    fn lua_atpanic(L: *mut lua_State_0, panicf: lua_CFunction) -> lua_CFunction;
    #[no_mangle]
    fn lua_version(L: *mut lua_State_0) -> *const lua_Number;
    #[no_mangle]
    fn lua_absindex(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_copy(L: *mut lua_State_0, fromidx: libc::c_int, toidx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State_0, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State_0, tp: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn lua_tonumberx(L: *mut lua_State_0, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Number;
    #[no_mangle]
    fn lua_tointegerx(
        L: *mut lua_State_0,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(
        L: *mut lua_State_0,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_rawlen(L: *mut lua_State_0, idx: libc::c_int) -> size_t;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State_0, idx: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_topointer(L: *mut lua_State_0, idx: libc::c_int) -> *const libc::c_void;
    #[no_mangle]
    fn lua_rawequal(L: *mut lua_State_0, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int;
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
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State_0, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State_0, sz: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State_0, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State_0, name: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> ();
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
    fn lua_load(
        L: *mut lua_State_0,
        reader: lua_Reader,
        dt: *mut libc::c_void,
        chunkname: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_next(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_concat(L: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_len(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getallocf(L: *mut lua_State_0, ud: *mut *mut libc::c_void) -> lua_Alloc;
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State_0, level: libc::c_int, ar: *mut lua_Debug) -> libc::c_int;
    #[no_mangle]
    fn lua_getinfo(
        L: *mut lua_State_0,
        what: *const libc::c_char,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
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
pub type _IO_FILE_0 = _IO_FILE;
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
/*
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
pub type lua_Reader = Option<
    unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void, _: *mut size_t)
        -> *const libc::c_char,
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
/* }====================================================== */
/*
** {======================================================
** Load functions
** =======================================================
*/
pub type LoadF = LoadF_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadF_0 {
    pub n: libc::c_int,
    pub f: *mut FILE,
    pub buff: [libc::c_char; 8192],
}
pub type LoadS = LoadS_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadS_0 {
    pub s: *const libc::c_char,
    pub size: size_t,
}
/* predefined references */
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
pub type luaL_Buffer = luaL_Buffer_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer_0 {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State_0,
    pub initb: [libc::c_char; 23],
}
/* }====================================================== */
/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/
/* userdata to box arbitrary data */
pub type UBox = UBox_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UBox_0 {
    pub box_0: *mut libc::c_void,
    pub bsize: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkversion_(
    mut L: *mut lua_State_0,
    mut ver: lua_Number,
    mut sz: size_t,
) -> () {
    let mut v: *const lua_Number = lua_version(L);
    /* check numeric types */
    if sz != (::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
        .wrapping_mul(16i32 as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<lua_Number>() as libc::c_ulong)
    {
        luaL_error(
            L,
            b"core and library have incompatible numeric types\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if v != lua_version(0 as *mut lua_State_0) {
        luaL_error(
            L,
            b"multiple Lua VMs detected\x00" as *const u8 as *const libc::c_char,
        );
    } else if *v != ver {
        luaL_error(
            L,
            b"version mismatch: app. needs %f, Lua core provides %f\x00" as *const u8
                as *const libc::c_char,
            ver,
            *v,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_getmetafield(
    mut L: *mut lua_State_0,
    mut obj: libc::c_int,
    mut event: *const libc::c_char,
) -> libc::c_int {
    /* no metatable? */
    if 0 == lua_getmetatable(L, obj) {
        return 0i32;
    } else {
        let mut tt: libc::c_int = 0;
        lua_pushstring(L, event);
        tt = lua_rawget(L, -2i32);
        /* is metafield nil? */
        if tt == 0i32 {
            lua_settop(L, -2i32 - 1i32);
        } else {
            lua_rotate(L, -2i32, -1i32);
            lua_settop(L, -1i32 - 1i32);
        }
        /* remove metatable and metafield */
        /* remove only metatable */
        /* return metafield type */
        return tt;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_callmeta(
    mut L: *mut lua_State_0,
    mut obj: libc::c_int,
    mut event: *const libc::c_char,
) -> libc::c_int {
    obj = lua_absindex(L, obj);
    /* no metafield? */
    if luaL_getmetafield(L, obj, event) == 0i32 {
        return 0i32;
    } else {
        lua_pushvalue(L, obj);
        lua_callk(L, 1i32, 1i32, 0i32 as lua_KContext, None);
        return 1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_tolstring(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    if 0 != luaL_callmeta(
        L,
        idx,
        b"__tostring\x00" as *const u8 as *const libc::c_char,
    ) {
        /* metafield? */
        if 0 == lua_isstring(L, -1i32) {
            luaL_error(
                L,
                b"\'__tostring\' must return a string\x00" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        match lua_type(L, idx) {
            3 => {
                if 0 != lua_isinteger(L, idx) {
                    lua_pushfstring(
                        L,
                        b"%I\x00" as *const u8 as *const libc::c_char,
                        lua_tointegerx(L, idx, 0 as *mut libc::c_int),
                    );
                } else {
                    lua_pushfstring(
                        L,
                        b"%f\x00" as *const u8 as *const libc::c_char,
                        lua_tonumberx(L, idx, 0 as *mut libc::c_int),
                    );
                }
            }
            4 => {
                lua_pushvalue(L, idx);
            }
            1 => {
                lua_pushstring(
                    L,
                    if 0 != lua_toboolean(L, idx) {
                        b"true\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"false\x00" as *const u8 as *const libc::c_char
                    },
                );
            }
            0 => {
                lua_pushstring(L, b"nil\x00" as *const u8 as *const libc::c_char);
            }
            _ => {
                /* try name */
                let mut tt: libc::c_int =
                    luaL_getmetafield(L, idx, b"__name\x00" as *const u8 as *const libc::c_char);
                let mut kind: *const libc::c_char = if tt == 4i32 {
                    lua_tolstring(L, -1i32, 0 as *mut size_t)
                } else {
                    lua_typename(L, lua_type(L, idx))
                };
                lua_pushfstring(
                    L,
                    b"%s: %p\x00" as *const u8 as *const libc::c_char,
                    kind,
                    lua_topointer(L, idx),
                );
                if tt != 0i32 {
                    lua_rotate(L, -2i32, -1i32);
                    lua_settop(L, -1i32 - 1i32);
                }
            }
        }
    }
    /* remove '__name' */
    return lua_tolstring(L, -1i32, len);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_argerror(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut extramsg: *const libc::c_char,
) -> libc::c_int {
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
    /* no stack frame? */
    if 0 == lua_getstack(L, 0i32, &mut ar) {
        return luaL_error(
            L,
            b"bad argument #%d (%s)\x00" as *const u8 as *const libc::c_char,
            arg,
            extramsg,
        );
    } else {
        lua_getinfo(L, b"n\x00" as *const u8 as *const libc::c_char, &mut ar);
        if strcmp(
            ar.namewhat,
            b"method\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            /* do not count 'self' */
            arg -= 1;
            /* error is in the self argument itself? */
            if arg == 0i32 {
                return luaL_error(
                    L,
                    b"calling \'%s\' on bad self (%s)\x00" as *const u8 as *const libc::c_char,
                    ar.name,
                    extramsg,
                );
            }
        }
        if ar.name.is_null() {
            ar.name = if 0 != pushglobalfuncname(L, &mut ar) {
                lua_tolstring(L, -1i32, 0 as *mut size_t)
            } else {
                b"?\x00" as *const u8 as *const libc::c_char
            }
        }
        return luaL_error(
            L,
            b"bad argument #%d to \'%s\' (%s)\x00" as *const u8 as *const libc::c_char,
            arg,
            ar.name,
            extramsg,
        );
    };
}
/*
** Search for a name for a function in all loaded modules
*/
unsafe extern "C" fn pushglobalfuncname(
    mut L: *mut lua_State_0,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut top: libc::c_int = lua_gettop(L);
    /* push function */
    lua_getinfo(L, b"f\x00" as *const u8 as *const libc::c_char, ar);
    lua_getfield(
        L,
        -50000i32 - 1000i32,
        b"_LOADED\x00" as *const u8 as *const libc::c_char,
    );
    if 0 != findfield(L, top + 1i32, 2i32) {
        let mut name: *const libc::c_char = lua_tolstring(L, -1i32, 0 as *mut size_t);
        if strncmp(
            name,
            b"_G.\x00" as *const u8 as *const libc::c_char,
            3i32 as libc::c_ulong,
        ) == 0i32
        {
            /* name start with '_G.'? */
            /* push name without prefix */
            lua_pushstring(L, name.offset(3isize));
            lua_rotate(L, -2i32, -1i32);
            lua_settop(L, -1i32 - 1i32);
        }
        /* remove original name */
        /* move name to proper place */
        lua_copy(L, -1i32, top + 1i32);
        lua_settop(L, -2i32 - 1i32);
        /* remove pushed values */
        return 1i32;
    } else {
        /* remove function and global table */
        lua_settop(L, top);
        return 0i32;
    };
}
/*
** $Id: lauxlib.c,v 1.288 2016/12/04 20:17:24 roberto Exp roberto $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/
/*
** This file uses only the official API of Lua.
** Any function declared here could be written as an application function.
*/
/*
** {======================================================
** Traceback
** =======================================================
*/
/* size of the first part of the stack */
/* size of the second part of the stack */
/*
** search for 'objidx' in table at index -1.
** return 1 + string at top if find a good name.
*/
unsafe extern "C" fn findfield(
    mut L: *mut lua_State_0,
    mut objidx: libc::c_int,
    mut level: libc::c_int,
) -> libc::c_int {
    if level == 0i32 || !(lua_type(L, -1i32) == 5i32) {
        /* not found */
        return 0i32;
    } else {
        /* start 'next' loop */
        lua_pushnil(L);
        while 0 != lua_next(L, -2i32) {
            /* for each pair in table */
            if lua_type(L, -2i32) == 4i32 {
                /* ignore non-string keys */
                if 0 != lua_rawequal(L, objidx, -1i32) {
                    /* found object? */
                    lua_settop(L, -1i32 - 1i32);
                    /* remove value (but keep name) */
                    return 1i32;
                } else if 0 != findfield(L, objidx, level - 1i32) {
                    /* try recursively */
                    lua_rotate(L, -2i32, -1i32);
                    lua_settop(L, -1i32 - 1i32);
                    lua_pushstring(L, b".\x00" as *const u8 as *const libc::c_char);
                    lua_rotate(L, -2i32, 1i32);
                    /* remove table (but keep name) */
                    /* place '.' between the two names */
                    lua_concat(L, 3i32);
                    return 1i32;
                }
            }
            lua_settop(L, -1i32 - 1i32);
        }
        /* remove value */
        /* not found */
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checklstring(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = lua_tolstring(L, arg, len);
    if s.is_null() {
        tag_error(L, arg, 4i32);
    }
    return s;
}
unsafe extern "C" fn tag_error(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut tag: libc::c_int,
) -> () {
    typeerror(L, arg, lua_typename(L, tag));
}
unsafe extern "C" fn typeerror(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut tname: *const libc::c_char,
) -> libc::c_int {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    /* name for the type of the actual argument */
    let mut typearg: *const libc::c_char = 0 as *const libc::c_char;
    if luaL_getmetafield(L, arg, b"__name\x00" as *const u8 as *const libc::c_char) == 4i32 {
        /* use the given type name */
        typearg = lua_tolstring(L, -1i32, 0 as *mut size_t)
    } else if lua_type(L, arg) == 2i32 {
        /* special name for messages */
        typearg = b"light userdata\x00" as *const u8 as *const libc::c_char
    } else {
        typearg = lua_typename(L, lua_type(L, arg))
    }
    msg = lua_pushfstring(
        L,
        b"%s expected, got %s\x00" as *const u8 as *const libc::c_char,
        tname,
        typearg,
    );
    return luaL_argerror(L, arg, msg);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optlstring(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut def: *const libc::c_char,
    mut len: *mut size_t,
) -> *const libc::c_char {
    if lua_type(L, arg) <= 0i32 {
        if !len.is_null() {
            *len = if !def.is_null() {
                strlen(def)
            } else {
                0i32 as libc::c_ulong
            }
        }
        return def;
    } else {
        return luaL_checklstring(L, arg, len);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checknumber(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
) -> lua_Number {
    let mut isnum: libc::c_int = 0;
    let mut d: lua_Number = lua_tonumberx(L, arg, &mut isnum);
    if 0 == isnum {
        tag_error(L, arg, 3i32);
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optnumber(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut def: lua_Number,
) -> lua_Number {
    return if lua_type(L, arg) <= 0i32 {
        def
    } else {
        luaL_checknumber(L, arg)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkinteger(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
) -> lua_Integer {
    let mut isnum: libc::c_int = 0;
    let mut d: lua_Integer = lua_tointegerx(L, arg, &mut isnum);
    if 0 == isnum {
        interror(L, arg);
    }
    return d;
}
unsafe extern "C" fn interror(mut L: *mut lua_State_0, mut arg: libc::c_int) -> () {
    if 0 != lua_isnumber(L, arg) {
        luaL_argerror(
            L,
            arg,
            b"number has no integer representation\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        tag_error(L, arg, 3i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optinteger(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut def: lua_Integer,
) -> lua_Integer {
    return if lua_type(L, arg) <= 0i32 {
        def
    } else {
        luaL_checkinteger(L, arg)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkstack(
    mut L: *mut lua_State_0,
    mut space: libc::c_int,
    mut msg: *const libc::c_char,
) -> () {
    if 0 == lua_checkstack(L, space) {
        if !msg.is_null() {
            luaL_error(
                L,
                b"stack overflow (%s)\x00" as *const u8 as *const libc::c_char,
                msg,
            );
        } else {
            luaL_error(L, b"stack overflow\x00" as *const u8 as *const libc::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checktype(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut t: libc::c_int,
) -> () {
    if lua_type(L, arg) != t {
        tag_error(L, arg, t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkany(mut L: *mut lua_State_0, mut arg: libc::c_int) -> () {
    if lua_type(L, arg) == -1i32 {
        luaL_argerror(
            L,
            arg,
            b"value expected\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_newmetatable(
    mut L: *mut lua_State_0,
    mut tname: *const libc::c_char,
) -> libc::c_int {
    /* name already in use? */
    if lua_getfield(L, -50000i32 - 1000i32, tname) != 0i32 {
        /* leave previous value on top, but return 0 */
        return 0i32;
    } else {
        lua_settop(L, -1i32 - 1i32);
        /* create metatable */
        lua_createtable(L, 0i32, 2i32);
        lua_pushstring(L, tname);
        /* metatable.__name = tname */
        lua_setfield(L, -2i32, b"__name\x00" as *const u8 as *const libc::c_char);
        lua_pushvalue(L, -1i32);
        /* registry.name = metatable */
        lua_setfield(L, -50000i32 - 1000i32, tname);
        return 1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_setmetatable(
    mut L: *mut lua_State_0,
    mut tname: *const libc::c_char,
) -> () {
    lua_getfield(L, -50000i32 - 1000i32, tname);
    lua_setmetatable(L, -2i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_testudata(
    mut L: *mut lua_State_0,
    mut ud: libc::c_int,
    mut tname: *const libc::c_char,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = lua_touserdata(L, ud);
    if !p.is_null() {
        /* value is a userdata? */
        if 0 != lua_getmetatable(L, ud) {
            /* does it have a metatable? */
            lua_getfield(L, -50000i32 - 1000i32, tname);
            /* get correct metatable */
            /* not the same? */
            if 0 == lua_rawequal(L, -1i32, -2i32) {
                /* value is a userdata with wrong metatable */
                p = 0 as *mut libc::c_void
            }
            lua_settop(L, -2i32 - 1i32);
            /* remove both metatables */
            return p;
        }
    }
    /* value is not a userdata with a metatable */
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkudata(
    mut L: *mut lua_State_0,
    mut ud: libc::c_int,
    mut tname: *const libc::c_char,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = luaL_testudata(L, ud, tname);
    if p.is_null() {
        typeerror(L, ud, tname);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_where(mut L: *mut lua_State_0, mut level: libc::c_int) -> () {
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
    if 0 != lua_getstack(L, level, &mut ar) {
        /* check function at level */
        /* get info about it */
        lua_getinfo(L, b"Sl\x00" as *const u8 as *const libc::c_char, &mut ar);
        if ar.currentline > 0i32 {
            /* is there info? */
            lua_pushfstring(
                L,
                b"%s:%d: \x00" as *const u8 as *const libc::c_char,
                ar.short_src.as_mut_ptr(),
                ar.currentline,
            );
            return;
        }
    }
    /* else, no information available... */
    lua_pushfstring(L, b"\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkoption(
    mut L: *mut lua_State_0,
    mut arg: libc::c_int,
    mut def: *const libc::c_char,
    mut lst: *const *const libc::c_char,
) -> libc::c_int {
    let mut name: *const libc::c_char = if !def.is_null() {
        luaL_optlstring(L, arg, def, 0 as *mut size_t)
    } else {
        luaL_checklstring(L, arg, 0 as *mut size_t)
    };
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !(*lst.offset(i as isize)).is_null() {
        if strcmp(*lst.offset(i as isize), name) == 0i32 {
            return i;
        } else {
            i += 1
        }
    }
    return luaL_argerror(
        L,
        arg,
        lua_pushfstring(
            L,
            b"invalid option \'%s\'\x00" as *const u8 as *const libc::c_char,
            name,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaL_fileresult(
    mut L: *mut lua_State_0,
    mut stat: libc::c_int,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    /* calls to Lua API may change this value */
    let mut en: libc::c_int = *__errno_location();
    if 0 != stat {
        lua_pushboolean(L, 1i32);
        return 1i32;
    } else {
        lua_pushnil(L);
        if !fname.is_null() {
            lua_pushfstring(
                L,
                b"%s: %s\x00" as *const u8 as *const libc::c_char,
                fname,
                strerror(en),
            );
        } else {
            lua_pushstring(L, strerror(en));
        }
        lua_pushinteger(L, en as lua_Integer);
        return 3i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_execresult(
    mut L: *mut lua_State_0,
    mut stat: libc::c_int,
) -> libc::c_int {
    /* type of termination */
    let mut what: *const libc::c_char = b"exit\x00" as *const u8 as *const libc::c_char;
    /* error? */
    if stat == -1i32 {
        return luaL_fileresult(L, 0i32, 0 as *const libc::c_char);
    } else {
        if stat & 0x7fi32 == 0i32 {
            stat = (stat & 0xff00i32) >> 8i32
        } else if ((stat & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
            stat = stat & 0x7fi32;
            what = b"signal\x00" as *const u8 as *const libc::c_char
        }
        /* interpret result */
        /* successful termination? */
        if *what as libc::c_int == 'e' as i32 && stat == 0i32 {
            lua_pushboolean(L, 1i32);
        } else {
            lua_pushnil(L);
        }
        lua_pushstring(L, what);
        lua_pushinteger(L, stat as lua_Integer);
        /* return true/nil,what,code */
        return 3i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_ref(mut L: *mut lua_State_0, mut t: libc::c_int) -> libc::c_int {
    let mut ref_0: libc::c_int = 0;
    if lua_type(L, -1i32) == 0i32 {
        lua_settop(L, -1i32 - 1i32);
        /* remove from stack */
        /* 'nil' has a unique fixed reference */
        return -1i32;
    } else {
        t = lua_absindex(L, t);
        /* get first free element */
        lua_rawgeti(L, t, 0i32 as lua_Integer);
        /* ref = t[freelist] */
        ref_0 = lua_tointegerx(L, -1i32, 0 as *mut libc::c_int) as libc::c_int;
        lua_settop(L, -1i32 - 1i32);
        /* remove it from stack */
        if ref_0 != 0i32 {
            /* any free element? */
            /* remove it from list */
            lua_rawgeti(L, t, ref_0 as lua_Integer);
            /* (t[freelist] = t[ref]) */
            lua_rawseti(L, t, 0i32 as lua_Integer);
        } else {
            ref_0 = lua_rawlen(L, t) as libc::c_int + 1i32
        }
        lua_rawseti(L, t, ref_0 as lua_Integer);
        return ref_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_unref(
    mut L: *mut lua_State_0,
    mut t: libc::c_int,
    mut ref_0: libc::c_int,
) -> () {
    if ref_0 >= 0i32 {
        t = lua_absindex(L, t);
        lua_rawgeti(L, t, 0i32 as lua_Integer);
        /* t[ref] = t[freelist] */
        lua_rawseti(L, t, ref_0 as lua_Integer);
        lua_pushinteger(L, ref_0 as lua_Integer);
        /* t[freelist] = ref */
        lua_rawseti(L, t, 0i32 as lua_Integer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadfilex(
    mut L: *mut lua_State_0,
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut lf: LoadF = LoadF_0 {
        n: 0,
        f: 0 as *mut FILE,
        buff: [0; 8192],
    };
    let mut status: libc::c_int = 0;
    let mut readstatus: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    /* index of filename on the stack */
    let mut fnameindex: libc::c_int = lua_gettop(L) + 1i32;
    if filename.is_null() {
        lua_pushstring(L, b"=stdin\x00" as *const u8 as *const libc::c_char);
        lf.f = stdin
    } else {
        lua_pushfstring(L, b"@%s\x00" as *const u8 as *const libc::c_char, filename);
        lf.f = fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
        if lf.f.is_null() {
            return errfile(
                L,
                b"open\x00" as *const u8 as *const libc::c_char,
                fnameindex,
            );
        }
    }
    /* read initial portion */
    if 0 != skipcomment(&mut lf, &mut c) {
        /* add line to correct line numbers */
        let fresh0 = lf.n;
        lf.n = lf.n + 1;
        lf.buff[fresh0 as usize] = '\n' as i32 as libc::c_char
    }
    if c == (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\x1bLua\x00"))[0usize]
        as libc::c_int && !filename.is_null()
    {
        /* binary file? */
        /* reopen in binary mode */
        lf.f = freopen(
            filename,
            b"rb\x00" as *const u8 as *const libc::c_char,
            lf.f,
        );
        if lf.f.is_null() {
            return errfile(
                L,
                b"reopen\x00" as *const u8 as *const libc::c_char,
                fnameindex,
            );
        } else {
            /* re-read initial portion */
            skipcomment(&mut lf, &mut c);
        }
    }
    if c != -1i32 {
        /* 'c' is the first character of the stream */
        let fresh1 = lf.n;
        lf.n = lf.n + 1;
        lf.buff[fresh1 as usize] = c as libc::c_char
    }
    status = lua_load(
        L,
        Some(getF),
        &mut lf as *mut LoadF as *mut libc::c_void,
        lua_tolstring(L, -1i32, 0 as *mut size_t),
        mode,
    );
    readstatus = ferror(lf.f);
    if !filename.is_null() {
        /* close file (even in case of errors) */
        fclose(lf.f);
    }
    if 0 != readstatus {
        /* ignore results from 'lua_load' */
        lua_settop(L, fnameindex);
        return errfile(
            L,
            b"read\x00" as *const u8 as *const libc::c_char,
            fnameindex,
        );
    } else {
        lua_rotate(L, fnameindex, -1i32);
        lua_settop(L, -1i32 - 1i32);
        return status;
    };
}
unsafe extern "C" fn errfile(
    mut L: *mut lua_State_0,
    mut what: *const libc::c_char,
    mut fnameindex: libc::c_int,
) -> libc::c_int {
    let mut serr: *const libc::c_char = strerror(*__errno_location());
    let mut filename: *const libc::c_char =
        lua_tolstring(L, fnameindex, 0 as *mut size_t).offset(1isize);
    lua_pushfstring(
        L,
        b"cannot %s %s: %s\x00" as *const u8 as *const libc::c_char,
        what,
        filename,
        serr,
    );
    lua_rotate(L, fnameindex, -1i32);
    lua_settop(L, -1i32 - 1i32);
    return 6i32 + 1i32;
}
unsafe extern "C" fn getF(
    mut L: *mut lua_State_0,
    mut ud: *mut libc::c_void,
    mut size: *mut size_t,
) -> *const libc::c_char {
    let mut lf: *mut LoadF = ud as *mut LoadF;
    /* not used */
    if (*lf).n > 0i32 {
        /* are there pre-read characters to be read? */
        /* return them (chars already in buffer) */
        *size = (*lf).n as size_t;
        /* no more pre-read characters */
        (*lf).n = 0i32
    } else if 0 != feof((*lf).f) {
        return 0 as *const libc::c_char;
    } else {
        *size = fread(
            (*lf).buff.as_mut_ptr() as *mut libc::c_void,
            1i32 as size_t,
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            (*lf).f,
        )
    }
    return (*lf).buff.as_mut_ptr();
}
/*
** reads the first character of file 'f' and skips an optional BOM mark
** in its beginning plus its first line if it starts with '#'. Returns
** true if it skipped the first line.  In any case, '*cp' has the
** first "valid" character of the file (after the optional BOM and
** a first-line comment).
*/
unsafe extern "C" fn skipcomment(mut lf: *mut LoadF, mut cp: *mut libc::c_int) -> libc::c_int {
    *cp = skipBOM(lf);
    let mut c: libc::c_int = *cp;
    if c == '#' as i32 {
        /* first line is a comment (Unix exec. file)? */
        loop {
            /* skip first line */
            c = _IO_getc((*lf).f);
            if !(c != -1i32 && c != '\n' as i32) {
                break;
            }
        }
        /* skip end-of-line, if present */
        *cp = _IO_getc((*lf).f);
        /* there was a comment */
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn skipBOM(mut lf: *mut LoadF) -> libc::c_int {
    /* UTF-8 BOM mark */
    let mut p: *const libc::c_char = b"\xef\xbb\xbf\x00" as *const u8 as *const libc::c_char;
    let mut c: libc::c_int = 0;
    (*lf).n = 0i32;
    loop {
        c = _IO_getc((*lf).f);
        if c == -1i32 || {
            let fresh2 = p;
            p = p.offset(1);
            c != *(fresh2 as *const libc::c_uchar) as libc::c_int
        } {
            return c;
        } else {
            /* to be read by the parser */
            let fresh3 = (*lf).n;
            (*lf).n = (*lf).n + 1;
            (*lf).buff[fresh3 as usize] = c as libc::c_char;
            if !(*p as libc::c_int != '\u{0}' as i32) {
                break;
            }
        }
    }
    /* prefix matched; discard it */
    (*lf).n = 0i32;
    /* return next character */
    return _IO_getc((*lf).f);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadbufferx(
    mut L: *mut lua_State_0,
    mut buff: *const libc::c_char,
    mut size: size_t,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut ls: LoadS = LoadS_0 {
        s: 0 as *const libc::c_char,
        size: 0,
    };
    ls.s = buff;
    ls.size = size;
    return lua_load(
        L,
        Some(getS),
        &mut ls as *mut LoadS as *mut libc::c_void,
        name,
        mode,
    );
}
unsafe extern "C" fn getS(
    mut L: *mut lua_State_0,
    mut ud: *mut libc::c_void,
    mut size: *mut size_t,
) -> *const libc::c_char {
    let mut ls: *mut LoadS = ud as *mut LoadS;
    /* not used */
    if (*ls).size == 0i32 as libc::c_ulong {
        return 0 as *const libc::c_char;
    } else {
        *size = (*ls).size;
        (*ls).size = 0i32 as size_t;
        return (*ls).s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadstring(
    mut L: *mut lua_State_0,
    mut s: *const libc::c_char,
) -> libc::c_int {
    return luaL_loadbufferx(L, s, strlen(s), s, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_newstate() -> *mut lua_State_0 {
    let mut L: *mut lua_State_0 = lua_newstate(Some(l_alloc), 0 as *mut libc::c_void);
    if !L.is_null() {
        lua_atpanic(L, Some(panic));
    }
    return L;
}
unsafe extern "C" fn l_alloc(
    mut ud: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    /* not used */
    if nsize == 0i32 as libc::c_ulong {
        free(ptr);
        return 0 as *mut libc::c_void;
    } else {
        return realloc(ptr, nsize);
    };
}
unsafe extern "C" fn panic(mut L: *mut lua_State_0) -> libc::c_int {
    fprintf(
        stderr,
        b"PANIC: unprotected error in call to Lua API (%s)\n\x00" as *const u8
            as *const libc::c_char,
        lua_tolstring(L, -1i32, 0 as *mut size_t),
    );
    fflush(stderr);
    /* return to Lua to abort */
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_len(mut L: *mut lua_State_0, mut idx: libc::c_int) -> lua_Integer {
    let mut l: lua_Integer = 0;
    let mut isnum: libc::c_int = 0;
    lua_len(L, idx);
    l = lua_tointegerx(L, -1i32, &mut isnum);
    if 0 == isnum {
        luaL_error(
            L,
            b"object length is not an integer\x00" as *const u8 as *const libc::c_char,
        );
    }
    lua_settop(L, -1i32 - 1i32);
    /* remove object */
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_gsub(
    mut L: *mut lua_State_0,
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
    mut r: *const libc::c_char,
) -> *const libc::c_char {
    let mut wild: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: size_t = strlen(p);
    let mut b: luaL_Buffer = luaL_Buffer_0 {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State_0,
        initb: [0; 23],
    };
    luaL_buffinit(L, &mut b);
    loop {
        wild = strstr(s, p);
        if wild.is_null() {
            break;
        }
        /* push prefix */
        luaL_addlstring(
            &mut b,
            s,
            wild.wrapping_offset_from(s) as libc::c_long as size_t,
        );
        /* push replacement in place of pattern */
        luaL_addstring(&mut b, r);
        /* continue after 'p' */
        s = wild.offset(l as isize)
    }
    /* push last suffix */
    luaL_addstring(&mut b, s);
    luaL_pushresult(&mut b);
    return lua_tolstring(L, -1i32, 0 as *mut size_t);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_pushresult(mut B: *mut luaL_Buffer) -> () {
    let mut L: *mut lua_State_0 = (*B).L;
    lua_pushlstring(L, (*B).b, (*B).n);
    if (*B).b != (*B).initb.as_mut_ptr() {
        /* delete old buffer */
        resizebox(L, -2i32, 0i32 as size_t);
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    };
}
unsafe extern "C" fn resizebox(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut newsize: size_t,
) -> *mut libc::c_void {
    let mut ud: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut allocf: lua_Alloc = lua_getallocf(L, &mut ud);
    let mut box_0: *mut UBox = lua_touserdata(L, idx) as *mut UBox;
    let mut temp: *mut libc::c_void =
        allocf.expect("non-null function pointer")(ud, (*box_0).box_0, (*box_0).bsize, newsize);
    if temp.is_null() && newsize > 0i32 as libc::c_ulong {
        /* allocation error? */
        /* free buffer */
        resizebox(L, idx, 0i32 as size_t);
        luaL_error(
            L,
            b"not enough memory for buffer allocation\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*box_0).box_0 = temp;
    (*box_0).bsize = newsize;
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addstring(mut B: *mut luaL_Buffer, mut s: *const libc::c_char) -> () {
    luaL_addlstring(B, s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addlstring(
    mut B: *mut luaL_Buffer,
    mut s: *const libc::c_char,
    mut l: size_t,
) -> () {
    if l > 0i32 as libc::c_ulong {
        /* avoid 'memcpy' when 's' can be NULL */
        let mut b: *mut libc::c_char = luaL_prepbuffsize(B, l);
        memcpy(
            b as *mut libc::c_void,
            s as *const libc::c_void,
            l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        (*B).n = ((*B).n as libc::c_ulong).wrapping_add(l) as size_t as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_prepbuffsize(
    mut B: *mut luaL_Buffer,
    mut sz: size_t,
) -> *mut libc::c_char {
    let mut L: *mut lua_State_0 = (*B).L;
    if (*B).size.wrapping_sub((*B).n) < sz {
        /* not enough space? */
        let mut newbuff: *mut libc::c_char = 0 as *mut libc::c_char;
        /* double buffer size */
        let mut newsize: size_t = (*B).size.wrapping_mul(2i32 as libc::c_ulong);
        /* not big enough? */
        if newsize.wrapping_sub((*B).n) < sz {
            newsize = (*B).n.wrapping_add(sz)
        }
        if newsize < (*B).n || newsize.wrapping_sub((*B).n) < sz {
            luaL_error(
                L,
                b"buffer too large\x00" as *const u8 as *const libc::c_char,
            );
        }
        /* create larger buffer */
        if (*B).b != (*B).initb.as_mut_ptr() {
            newbuff = resizebox(L, -1i32, newsize) as *mut libc::c_char
        } else {
            /* no buffer yet */
            newbuff = newbox(L, newsize) as *mut libc::c_char;
            /* copy original content */
            memcpy(
                newbuff as *mut libc::c_void,
                (*B).b as *const libc::c_void,
                (*B).n
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        }
        (*B).b = newbuff;
        (*B).size = newsize
    }
    return &mut *(*B).b.offset((*B).n as isize) as *mut libc::c_char;
}
unsafe extern "C" fn newbox(mut L: *mut lua_State_0, mut newsize: size_t) -> *mut libc::c_void {
    let mut box_0: *mut UBox =
        lua_newuserdata(L, ::std::mem::size_of::<UBox>() as libc::c_ulong) as *mut UBox;
    (*box_0).box_0 = 0 as *mut libc::c_void;
    (*box_0).bsize = 0i32 as size_t;
    if 0 != luaL_newmetatable(L, b"LUABOX\x00" as *const u8 as *const libc::c_char) {
        /* creating metatable? */
        lua_pushcclosure(L, Some(boxgc), 0i32);
        /* metatable.__gc = boxgc */
        lua_setfield(L, -2i32, b"__gc\x00" as *const u8 as *const libc::c_char);
    }
    lua_setmetatable(L, -2i32);
    return resizebox(L, -1i32, newsize);
}
unsafe extern "C" fn boxgc(mut L: *mut lua_State_0) -> libc::c_int {
    resizebox(L, 1i32, 0i32 as size_t);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_buffinit(mut L: *mut lua_State_0, mut B: *mut luaL_Buffer) -> () {
    (*B).L = L;
    (*B).b = (*B).initb.as_mut_ptr();
    (*B).n = 0i32 as size_t;
    (*B).size = 23i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_setfuncs(
    mut L: *mut lua_State_0,
    mut l: *const luaL_Reg_0,
    mut nup: libc::c_int,
) -> () {
    luaL_checkstack(
        L,
        nup,
        b"too many upvalues\x00" as *const u8 as *const libc::c_char,
    );
    while !(*l).name.is_null() {
        /* fill the table with given functions */
        let mut i: libc::c_int = 0;
        /* copy upvalues to the top */
        i = 0i32;
        while i < nup {
            lua_pushvalue(L, -nup);
            i += 1
        }
        /* closure with those upvalues */
        lua_pushcclosure(L, (*l).func, nup);
        lua_setfield(L, -(nup + 2i32), (*l).name);
        l = l.offset(1isize)
    }
    lua_settop(L, -nup - 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_getsubtable(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    if lua_getfield(L, idx, fname) == 5i32 {
        /* table already there */
        return 1i32;
    } else {
        lua_settop(L, -1i32 - 1i32);
        /* remove previous result */
        idx = lua_absindex(L, idx);
        lua_createtable(L, 0i32, 0i32);
        /* copy to be left at top */
        lua_pushvalue(L, -1i32);
        /* assign new table to field */
        lua_setfield(L, idx, fname);
        /* false, because did not find table there */
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_traceback(
    mut L: *mut lua_State_0,
    mut L1: *mut lua_State_0,
    mut msg: *const libc::c_char,
    mut level: libc::c_int,
) -> () {
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
    let mut top: libc::c_int = lua_gettop(L);
    let mut last: libc::c_int = lastlevel(L1);
    let mut n1: libc::c_int = if last - level > 10i32 + 11i32 {
        10i32
    } else {
        -1i32
    };
    if !msg.is_null() {
        lua_pushfstring(L, b"%s\n\x00" as *const u8 as *const libc::c_char, msg);
    }
    luaL_checkstack(L, 10i32, 0 as *const libc::c_char);
    lua_pushstring(
        L,
        b"stack traceback:\x00" as *const u8 as *const libc::c_char,
    );
    loop {
        let fresh4 = level;
        level = level + 1;
        if !(0 != lua_getstack(L1, fresh4, &mut ar)) {
            break;
        }
        let fresh5 = n1;
        n1 = n1 - 1;
        if fresh5 == 0i32 {
            /* too many levels? */
            lua_pushstring(L, b"\n\t...\x00" as *const u8 as *const libc::c_char);
            /* add a '...' */
            /* and skip to last ones */
            level = last - 11i32 + 1i32
        } else {
            lua_getinfo(L1, b"Slnt\x00" as *const u8 as *const libc::c_char, &mut ar);
            lua_pushfstring(
                L,
                b"\n\t%s:\x00" as *const u8 as *const libc::c_char,
                ar.short_src.as_mut_ptr(),
            );
            if ar.currentline > 0i32 {
                lua_pushfstring(
                    L,
                    b"%d:\x00" as *const u8 as *const libc::c_char,
                    ar.currentline,
                );
            }
            lua_pushstring(L, b" in \x00" as *const u8 as *const libc::c_char);
            pushfuncname(L, &mut ar);
            if 0 != ar.istailcall {
                lua_pushstring(
                    L,
                    b"\n\t(...tail calls...)\x00" as *const u8 as *const libc::c_char,
                );
            }
            lua_concat(L, lua_gettop(L) - top);
        }
    }
    lua_concat(L, lua_gettop(L) - top);
}
unsafe extern "C" fn pushfuncname(mut L: *mut lua_State_0, mut ar: *mut lua_Debug) -> () {
    if 0 != pushglobalfuncname(L, ar) {
        /* try first a global name */
        lua_pushfstring(
            L,
            b"function \'%s\'\x00" as *const u8 as *const libc::c_char,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
        );
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    } else if *(*ar).namewhat as libc::c_int != '\u{0}' as i32 {
        /* use it */
        lua_pushfstring(
            L,
            b"%s \'%s\'\x00" as *const u8 as *const libc::c_char,
            (*ar).namewhat,
            (*ar).name,
        );
    } else if *(*ar).what as libc::c_int == 'm' as i32 {
        lua_pushstring(L, b"main chunk\x00" as *const u8 as *const libc::c_char);
    } else if *(*ar).what as libc::c_int != 'C' as i32 {
        lua_pushfstring(
            L,
            b"function <%s:%d>\x00" as *const u8 as *const libc::c_char,
            (*ar).short_src.as_mut_ptr(),
            (*ar).linedefined,
        );
    } else {
        lua_pushstring(L, b"?\x00" as *const u8 as *const libc::c_char);
    };
}
/* nothing left... */
unsafe extern "C" fn lastlevel(mut L: *mut lua_State_0) -> libc::c_int {
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
    let mut li: libc::c_int = 1i32;
    let mut le: libc::c_int = 1i32;
    /* find an upper bound */
    while 0 != lua_getstack(L, le, &mut ar) {
        li = le;
        le *= 2i32
    }
    /* do a binary search */
    while li < le {
        let mut m: libc::c_int = (li + le) / 2i32;
        if 0 != lua_getstack(L, m, &mut ar) {
            li = m + 1i32
        } else {
            le = m
        }
    }
    return le - 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_requiref(
    mut L: *mut lua_State_0,
    mut modname: *const libc::c_char,
    mut openf: lua_CFunction,
    mut glb: libc::c_int,
) -> () {
    luaL_getsubtable(
        L,
        -50000i32 - 1000i32,
        b"_LOADED\x00" as *const u8 as *const libc::c_char,
    );
    /* LOADED[modname] */
    lua_getfield(L, -1i32, modname);
    if 0 == lua_toboolean(L, -1i32) {
        /* package not already loaded? */
        lua_settop(L, -1i32 - 1i32);
        lua_pushcclosure(L, openf, 0i32);
        /* remove field */
        /* argument to open function */
        lua_pushstring(L, modname);
        lua_callk(L, 1i32, 1i32, 0i32 as lua_KContext, None);
        /* call 'openf' to open module */
        /* make copy of module (call result) */
        lua_pushvalue(L, -1i32);
        /* LOADED[modname] = module */
        lua_setfield(L, -3i32, modname);
    }
    lua_rotate(L, -2i32, -1i32);
    lua_settop(L, -1i32 - 1i32);
    /* remove LOADED table */
    if 0 != glb {
        /* copy of module */
        lua_pushvalue(L, -1i32);
        /* _G[modname] = module */
        lua_setglobal(L, modname);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addvalue(mut B: *mut luaL_Buffer) -> () {
    let mut L: *mut lua_State_0 = (*B).L;
    let mut l: size_t = 0;
    let mut s: *const libc::c_char = lua_tolstring(L, -1i32, &mut l);
    if (*B).b != (*B).initb.as_mut_ptr() {
        lua_rotate(L, -2i32, 1i32);
    }
    /* put value below buffer */
    luaL_addlstring(B, s, l);
    lua_rotate(
        L,
        if (*B).b != (*B).initb.as_mut_ptr() {
            -2i32
        } else {
            -1i32
        },
        -1i32,
    );
    lua_settop(L, -1i32 - 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_pushresultsize(mut B: *mut luaL_Buffer, mut sz: size_t) -> () {
    (*B).n = ((*B).n as libc::c_ulong).wrapping_add(sz) as size_t as size_t;
    luaL_pushresult(B);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_buffinitsize(
    mut L: *mut lua_State_0,
    mut B: *mut luaL_Buffer,
    mut sz: size_t,
) -> *mut libc::c_char {
    luaL_buffinit(L, B);
    return luaL_prepbuffsize(B, sz);
}
