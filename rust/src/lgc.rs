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
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaT_gettm(events: *mut Table_0, event: TMS, ename: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaT_gettmbyobj(L: *mut lua_State_0, o: *const TValue, event: TMS) -> *const TValue;
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State_0,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaE_setdebt(g: *mut global_State, debt: l_mem) -> ();
    #[no_mangle]
    fn luaE_freethread(L: *mut lua_State_0, L1: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State_0, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_pcall(
        L: *mut lua_State_0,
        func: Pfunc,
        u: *mut libc::c_void,
        oldtop: ptrdiff_t,
        ef: ptrdiff_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaD_shrinkstack(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State_0, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaF_freeproto(L: *mut lua_State_0, f: *mut Proto_0) -> ();
    #[no_mangle]
    fn luaS_remove(L: *mut lua_State_0, ts: *mut TString) -> ();
    #[no_mangle]
    fn luaH_free(L: *mut lua_State_0, t: *mut Table_0) -> ();
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn luaS_resize(L: *mut lua_State_0, newsize: libc::c_int) -> ();
    #[no_mangle]
    fn luaS_clearcache(g: *mut global_State) -> ();
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
    h: Table,
    p: Proto,
    th: lua_State,
}
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
** $Id: ldo.h,v 2.28 2015/11/23 11:29:43 roberto Exp roberto $
** Stack and Call structure of Lua
** See Copyright Notice in lua.h
*/
/*
** Macro to check stack size and grow stack if needed.  Parameters
** 'pre'/'pos' allow the macro to preserve a pointer into the
** stack across reallocations, doing the work only when needed.
** 'condmovestack' is used in heavy tests to force a stack reallocation
** at every check.
*/
/* In general, 'pre'/'pos' are empty (nothing to save) */
/* type of protected functions, to be ran by 'runprotected' */
pub type Pfunc = Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString {
    dummy: L_Umaxalign,
    tsv: TString,
}
/*
** Ensures that address after this type is always fully aligned.
*/
pub type UTString_0 = UTString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table,
    pub len: size_t,
    pub user_: Value_0,
}
/*
** Get the actual string (array of bytes) from a 'TString'.
** (Access to 'extra' ensures that value is really a 'TString'.)
*/
/* get the actual string (array of bytes) from a Lua value */
/* get string length from 'TString *s' */
/* get string length from 'TValue *o' */
/*
** Header for userdata; memory area follows the end of this structure
** (aligned according to 'UUdata'; see next).
*/
pub type Udata_0 = Udata;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UUdata {
    dummy: L_Umaxalign,
    uv: Udata_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
pub type Upvaldesc_0 = Upvaldesc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
pub type LocVar_0 = LocVar;
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
/*
** Function Prototypes
*/
pub type Proto_0 = Proto;
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
** Closures
*/
pub type CClosure_0 = CClosure;
pub type LClosure_0 = LClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    c: CClosure_0,
    l: LClosure_0,
}
pub type Table_0 = Table;
pub type TMS = libc::c_uint;
/* number of elements in the enum */
pub const TM_N: TMS = 24;
pub const TM_CALL: TMS = 23;
pub const TM_CONCAT: TMS = 22;
pub const TM_LE: TMS = 21;
pub const TM_LT: TMS = 20;
pub const TM_BNOT: TMS = 19;
pub const TM_UNM: TMS = 18;
pub const TM_SHR: TMS = 17;
pub const TM_SHL: TMS = 16;
pub const TM_BXOR: TMS = 15;
pub const TM_BOR: TMS = 14;
pub const TM_BAND: TMS = 13;
pub const TM_IDIV: TMS = 12;
pub const TM_DIV: TMS = 11;
pub const TM_POW: TMS = 10;
pub const TM_MOD: TMS = 9;
pub const TM_MUL: TMS = 8;
pub const TM_SUB: TMS = 7;
pub const TM_ADD: TMS = 6;
/* last tag method with fast access */
pub const TM_EQ: TMS = 5;
pub const TM_LEN: TMS = 4;
pub const TM_MODE: TMS = 3;
pub const TM_GC: TMS = 2;
pub const TM_NEWINDEX: TMS = 1;
pub const TM_INDEX: TMS = 0;
#[no_mangle]
pub unsafe extern "C" fn luaC_fix(mut L: *mut lua_State_0, mut o: *mut GCObject) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if (*g).allgc == o {
    } else {
        __assert_fail(
            b"g->allgc == o\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            197i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void luaC_fix(lua_State *, GCObject *)\x00",
            )).as_ptr(),
        );
    };
    (*o).marked = ((*o).marked as libc::c_int
        & !(1i32 << 0i32 | 1i32 << 1i32) as lu_byte as libc::c_int) as lu_byte;
    /* object must be 1st in 'allgc' list! */
    /* they will be gray forever */
    /* remove object from 'allgc' list */
    (*g).allgc = (*o).next;
    /* link it to 'fixedgc' list */
    (*o).next = (*g).fixedgc;
    (*g).fixedgc = o;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_freeallobjects(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* separate all objects with finalizers */
    separatetobefnz(g, 1i32);
    if (*g).finobj.is_null() {
    } else {
        __assert_fail(
            b"g->finobj == ((void*)0)\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            970i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void luaC_freeallobjects(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    callallpendingfinalizers(L);
    if (*g).tobefnz.is_null() {
    } else {
        __assert_fail(
            b"g->tobefnz == ((void*)0)\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            972i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void luaC_freeallobjects(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    /* this "white" makes all objects look dead */
    (*g).currentwhite = (1i32 << 0i32 | 1i32 << 1i32) as lu_byte;
    (*g).gckind = 0i32 as lu_byte;
    sweeplist(L, &mut (*g).finobj, !(0i32 as lu_mem));
    sweeplist(L, &mut (*g).allgc, !(0i32 as lu_mem));
    sweeplist(L, &mut (*g).fixedgc, !(0i32 as lu_mem));
    if (*g).strt.nuse == 0i32 {
    } else {
        __assert_fail(
            b"g->strt.nuse == 0\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            978i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void luaC_freeallobjects(lua_State *)\x00",
            )).as_ptr(),
        );
    };
}
unsafe extern "C" fn sweeplist(
    mut L: *mut lua_State_0,
    mut p: *mut *mut GCObject,
    mut count: lu_mem,
) -> *mut *mut GCObject {
    let mut g: *mut global_State = (*L).l_G;
    let mut ow: libc::c_int = (*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32);
    /* current white */
    let mut white: libc::c_int = ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32))
        as lu_byte as libc::c_int;
    while !(*p).is_null() && {
        let fresh0 = count;
        count = count.wrapping_sub(1);
        fresh0 > 0i32 as libc::c_ulong
    } {
        let mut curr: *mut GCObject = *p;
        let mut marked: libc::c_int = (*curr).marked as libc::c_int;
        if 0 == (marked ^ (1i32 << 0i32 | 1i32 << 1i32)) & ow {
            /* is 'curr' dead? */
            /* remove 'curr' from list */
            *p = (*curr).next;
            /* erase 'curr' */
            freeobj(L, curr);
        } else {
            /* change mark to 'white' */
            (*curr).marked =
                (marked & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32)) | white) as lu_byte;
            /* go to next element */
            p = &mut (*curr).next
        }
    }
    return if (*p).is_null() {
        0 as *mut *mut GCObject
    } else {
        p
    };
}
unsafe extern "C" fn freeobj(mut L: *mut lua_State_0, mut o: *mut GCObject) -> () {
    match (*o).tt as libc::c_int {
        9 => {
            if (*o).tt as libc::c_int == 9i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    699i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            luaF_freeproto(L, &mut (*(o as *mut GCUnion)).p);
        }
        6 => {
            if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    701i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            freeLclosure(L, &mut (*(o as *mut GCUnion)).cl.l);
        }
        38 => {
            if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    705i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            luaM_realloc_(
                L,
                o as *mut libc::c_void,
                (::std::mem::size_of::<CClosure_0>() as libc::c_ulong as libc::c_int
                    + (::std::mem::size_of::<TValue>() as libc::c_ulong).wrapping_mul(
                        (*&mut (*(o as *mut GCUnion)).cl.c).nupvalues as libc::c_ulong,
                    ) as libc::c_int) as size_t,
                0i32 as size_t,
            );
        }
        5 => {
            if (*o).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    708i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            luaH_free(L, &mut (*(o as *mut GCUnion)).h);
        }
        8 => {
            if (*o).tt as libc::c_int == 8i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == 8\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    709i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            luaE_freethread(L, &mut (*(o as *mut GCUnion)).th);
        }
        7 => {
            if (*o).tt as libc::c_int == 7i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    710i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            luaM_realloc_(
                L,
                o as *mut libc::c_void,
                (::std::mem::size_of::<UUdata>() as libc::c_ulong)
                    .wrapping_add((*&mut (*(o as *mut GCUnion)).u).len),
                0i32 as size_t,
            );
        }
        4 => {
            /* remove it from hash table */
            if (*o).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    712i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            luaS_remove(L, &mut (*(o as *mut GCUnion)).ts);
            if (*o).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    713i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            luaM_realloc_(
                L,
                o as *mut libc::c_void,
                (::std::mem::size_of::<UTString>() as libc::c_ulong).wrapping_add(
                    (((*&mut (*(o as *mut GCUnion)).ts).shrlen as libc::c_int + 1i32)
                        as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
                ),
                0i32 as size_t,
            );
        }
        20 => {
            if (*o).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    716i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            luaM_realloc_(
                L,
                o as *mut libc::c_void,
                (::std::mem::size_of::<UTString>() as libc::c_ulong).wrapping_add(
                    (*&mut (*(o as *mut GCUnion)).ts)
                        .u
                        .lnglen
                        .wrapping_add(1i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
                ),
                0i32 as size_t,
            );
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    719i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void freeobj(lua_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
        }
    };
}
unsafe extern "C" fn freeLclosure(mut L: *mut lua_State_0, mut cl: *mut LClosure_0) -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv: *mut UpVal = *(*cl).upvals.as_mut_ptr().offset(i as isize);
        if !uv.is_null() {
            luaC_upvdeccount(L, uv);
        }
        i += 1
    }
    luaM_realloc_(
        L,
        cl as *mut libc::c_void,
        (::std::mem::size_of::<LClosure_0>() as libc::c_ulong as libc::c_int
            + (::std::mem::size_of::<*mut TValue>() as libc::c_ulong)
                .wrapping_mul((*cl).nupvalues as libc::c_ulong) as libc::c_int) as size_t,
        0i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaC_upvdeccount(mut L: *mut lua_State_0, mut uv: *mut UpVal) -> () {
    if (*uv).refcount > 0i32 as libc::c_ulong {
    } else {
        __assert_fail(
            b"uv->refcount > 0\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            679i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void luaC_upvdeccount(lua_State *, UpVal *)\x00",
            )).as_ptr(),
        );
    };
    (*uv).refcount = (*uv).refcount.wrapping_sub(1);
    if (*uv).refcount == 0i32 as libc::c_ulong && !((*uv).v != &mut (*uv).u.value as *mut TValue) {
        luaM_realloc_(
            L,
            uv as *mut libc::c_void,
            ::std::mem::size_of::<UpVal>() as libc::c_ulong,
            0i32 as size_t,
        );
    };
}
/*
** call all pending finalizers
*/
unsafe extern "C" fn callallpendingfinalizers(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    while !(*g).tobefnz.is_null() {
        GCTM(L, 0i32);
    }
}
unsafe extern "C" fn GCTM(mut L: *mut lua_State_0, mut propagateerrors: libc::c_int) -> () {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut g: *mut global_State = (*L).l_G;
    let mut tm: *const TValue = 0 as *const TValue;
    let mut v: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut v;
    let mut i_g: *mut GCObject = udata2finalize(g);
    (*io).value_.gc = i_g;
    (*io).tt_ = (*i_g).tt as libc::c_int | 1i32 << 6i32;
    tm = luaT_gettmbyobj(L, &mut v, TM_GC);
    if !tm.is_null() && (*tm).tt_ & 0xfi32 == 6i32 {
        /* is there a finalizer? */
        let mut status: libc::c_int = 0;
        let mut oldah: lu_byte = (*L).allowhook;
        let mut running: libc::c_int = (*g).gcrunning as libc::c_int;
        /* stop debug hooks during GC metamethod */
        (*L).allowhook = 0i32 as lu_byte;
        /* avoid GC steps */
        (*g).gcrunning = 0i32 as lu_byte;
        let mut io1: *mut TValue = (*L).top;
        *io1 = *tm;
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    819i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                        b"void GCTM(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        819i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                            b"void GCTM(lua_State *, int)\x00",
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
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    819i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                        b"void GCTM(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
        };
        /* push finalizer... */
        let mut io1_0: *mut TValue = (*L).top.offset(1isize);
        *io1_0 = v;
        if 0 == (*io1_0).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    820i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                        b"void GCTM(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io1_0).tt_ & 0x3fi32 == (*(*io1_0).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        820i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                            b"void GCTM(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                0 != ((*(*io1_0).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    820i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                        b"void GCTM(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
        };
        /* ... and its argument */
        /* and (next line) call the finalizer */
        (*L).top = (*L).top.offset(2isize);
        /* will run a finalizer */
        (*(*L).ci).callstatus =
            ((*(*L).ci).callstatus as libc::c_int | 1i32 << 8i32) as libc::c_ushort;
        status = luaD_pcall(
            L,
            Some(dothecall),
            0 as *mut libc::c_void,
            ((*L).top.offset(-2isize) as *mut libc::c_char)
                .wrapping_offset_from((*L).stack as *mut libc::c_char) as libc::c_long,
            0i32 as ptrdiff_t,
        );
        /* not running a finalizer anymore */
        (*(*L).ci).callstatus =
            ((*(*L).ci).callstatus as libc::c_int & !(1i32 << 8i32)) as libc::c_ushort;
        /* restore hooks */
        (*L).allowhook = oldah;
        /* restore state */
        (*g).gcrunning = running as lu_byte;
        if status != 0i32 && 0 != propagateerrors {
            /* error while running __gc? */
            if status == 2i32 {
                /* is there an error object? */
                msg = if (*(*L).top.offset(-1isize)).tt_ & 0xfi32 == 4i32 {
                    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
                    } else {
                        __assert_fail(b"sizeof((((((((((((L->top - 1))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((L->top - 1))->tt_)) & 0x0F)) == (4))\", \"lgc.c\", 830, __extension__ __PRETTY_FUNCTION__)), (((((((((L->top - 1)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((L->top - 1)->value_).gc)->tt) & 0x0F) == 4\", \"lgc.c\", 830, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((L->top - 1)->value_).gc))))->ts))))))->extra)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          830i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 28],
                                                                    &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
                    };
                    if (*(*L).top.offset(-1isize)).tt_ & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((((L->top - 1))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                                as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            830i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                                b"void GCTM(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*(*L).top.offset(-1isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                    } else {
                        __assert_fail(
                            b"(((((L->top - 1)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                                as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            830i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                                b"void GCTM(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    (&mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion)).ts
                        as *mut TString_0 as *mut libc::c_char)
                        .offset(::std::mem::size_of::<UTString_0>() as libc::c_ulong as isize)
                } else {
                    b"no message\x00" as *const u8 as *const libc::c_char
                };
                luaO_pushfstring(
                    L,
                    b"error in __gc metamethod (%s)\x00" as *const u8 as *const libc::c_char,
                    msg,
                );
                /* error in __gc metamethod */
                status = 5i32
            }
            /* re-throw error */
            luaD_throw(L, status);
        }
    };
}
unsafe extern "C" fn dothecall(mut L: *mut lua_State_0, mut ud: *mut libc::c_void) -> () {
    ud = 0 as *mut libc::c_void;
    luaD_callnoyield(L, (*L).top.offset(-2isize), 0i32);
}
unsafe extern "C" fn udata2finalize(mut g: *mut global_State) -> *mut GCObject {
    /* get first element */
    let mut o: *mut GCObject = (*g).tobefnz;
    if 0 != (*o).marked as libc::c_int & 1i32 << 3i32 {
    } else {
        __assert_fail(
            b"(((o)->marked) & ((1<<(3))))\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            790i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"GCObject *udata2finalize(global_State *)\x00",
            )).as_ptr(),
        );
    };
    /* remove it from 'tobefnz' list */
    (*g).tobefnz = (*o).next;
    /* return it to 'allgc' list */
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    (*o).marked =
        ((*o).marked as libc::c_int & !(1i32 << 3i32) as lu_byte as libc::c_int) as lu_byte;
    /* object is "normal" again */
    if 2i32 <= (*g).gcstate as libc::c_int && (*g).gcstate as libc::c_int <= 5i32 {
        (*o).marked = ((*o).marked as libc::c_int & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32))
            | ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
                as libc::c_int) as lu_byte
    }
    /* "sweep" object */
    return o;
}
/*
** move all unreachable objects (or 'all' objects) that need
** finalization from list 'finobj' to list 'tobefnz' (to be finalized)
*/
unsafe extern "C" fn separatetobefnz(mut g: *mut global_State, mut all: libc::c_int) -> () {
    let mut curr: *mut GCObject = 0 as *mut GCObject;
    let mut p: *mut *mut GCObject = &mut (*g).finobj;
    let mut lastnext: *mut *mut GCObject = findlast(&mut (*g).tobefnz);
    loop {
        curr = *p;
        if curr.is_null() {
            break;
        }
        /* traverse all finalizable objects */
        if 0 != (*curr).marked as libc::c_int & 1i32 << 3i32 {
        } else {
            __assert_fail(
                b"(((curr)->marked) & ((1<<(3))))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                885i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"void separatetobefnz(global_State *, int)\x00",
                )).as_ptr(),
            );
        };
        /* not being collected? */
        if !(0 != (*curr).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) || 0 != all) {
            /* don't bother with it */
            p = &mut (*curr).next
        } else {
            /* remove 'curr' from 'finobj' list */
            *p = (*curr).next;
            /* link at the end of 'tobefnz' list */
            (*curr).next = *lastnext;
            *lastnext = curr;
            lastnext = &mut (*curr).next
        }
    }
}
/*
** find last 'next' field in list 'p' list (to add elements in its end)
*/
unsafe extern "C" fn findlast(mut p: *mut *mut GCObject) -> *mut *mut GCObject {
    while !(*p).is_null() {
        p = &mut (**p).next
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_step(mut L: *mut lua_State_0) -> () {
    let mut work: lu_mem = 0;
    let mut g: *mut global_State = (*L).l_G;
    /* GC deficit (be paid now) */
    let mut debt: l_mem = getdebt(g);
    if 0 == (*g).gcrunning {
        /* not running? */
        /* avoid being called too often */
        luaE_setdebt(
            g,
            (-((100i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TString>() as libc::c_ulong)
                as libc::c_int) * 10i32) as l_mem,
        );
        return;
    } else {
        loop {
            /* repeat until pause or enough "credit" (negative debt) */
            /* perform one single step */
            work = singlestep(L);
            debt = (debt as libc::c_ulong).wrapping_sub(work) as l_mem as l_mem;
            if !(debt > -((100i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TString>() as libc::c_ulong)
                as libc::c_int) as libc::c_long
                && (*g).gcstate as libc::c_int != 7i32)
            {
                break;
            }
        }
        if (*g).gcstate as libc::c_int == 7i32 {
            /* pause until next cycle */
            setpause(g);
        } else {
            /* convert 'work units' to Kb */
            debt = debt / (*g).gcstepmul as libc::c_long * 200i32 as libc::c_long;
            luaE_setdebt(g, debt);
            runafewfinalizers(L);
        }
        return;
    };
}
/*
** call a few (up to 'g->gcfinnum') finalizers
*/
unsafe extern "C" fn runafewfinalizers(mut L: *mut lua_State_0) -> libc::c_int {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: libc::c_uint = 0;
    if (*g).tobefnz.is_null() || (*g).gcfinnum > 0i32 as libc::c_uint {
    } else {
        __assert_fail(
            b"!g->tobefnz || g->gcfinnum > 0\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            847i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"int runafewfinalizers(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    i = 0i32 as libc::c_uint;
    while !(*g).tobefnz.is_null() && i < (*g).gcfinnum {
        /* call one finalizer */
        GCTM(L, 1i32);
        i = i.wrapping_add(1)
    }
    /* nothing more to finalize? */
    (*g).gcfinnum = if (*g).tobefnz.is_null() {
        0i32 as libc::c_uint
    } else {
        (*g).gcfinnum.wrapping_mul(2i32 as libc::c_uint)
    };
    /* else call a few more next time */
    return i as libc::c_int;
}
/*
** get GC debt and convert it from Kb to 'work units' (avoid zero debt
** and overflows)
*/
unsafe extern "C" fn getdebt(mut g: *mut global_State) -> l_mem {
    let mut debt: l_mem = (*g).GCdebt;
    let mut stepmul: libc::c_int = (*g).gcstepmul;
    if debt <= 0i32 as libc::c_long {
        /* minimal debt */
        return 0i32 as l_mem;
    } else {
        debt = debt / 200i32 as libc::c_long + 1i32 as libc::c_long;
        debt = if debt < (!(0i32 as lu_mem) >> 1i32) as l_mem / stepmul as libc::c_long {
            debt * stepmul as libc::c_long
        } else {
            (!(0i32 as lu_mem) >> 1i32) as l_mem
        };
        return debt;
    };
}
/* mark it as such */
/* }====================================================== */
/*
** {======================================================
** GC control
** =======================================================
*/
/*
** Set a reasonable "time" to wait before starting a new GC cycle; cycle
** will start when memory use hits threshold. (Division by 'estimate'
** should be OK: it cannot be zero (because Lua cannot even start with
** less than PAUSEADJ bytes).
*/
unsafe extern "C" fn setpause(mut g: *mut global_State) -> () {
    let mut threshold: l_mem = 0;
    let mut debt: l_mem = 0;
    /* adjust 'estimate' */
    let mut estimate: l_mem = (*g).GCestimate.wrapping_div(100i32 as libc::c_ulong) as l_mem;
    if estimate > 0i32 as libc::c_long {
    } else {
        __assert_fail(
            b"estimate > 0\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            943i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void setpause(global_State *)\x00",
            )).as_ptr(),
        );
    };
    /* overflow? */
    threshold = if ((*g).gcpause as libc::c_long) < (!(0i32 as lu_mem) >> 1i32) as l_mem / estimate
    {
        estimate * (*g).gcpause as libc::c_long
    } else {
        (!(0i32 as lu_mem) >> 1i32) as l_mem
    };
    /* no overflow */
    /* overflow; truncate to maximum */
    debt = (((*g).totalbytes + (*g).GCdebt) as lu_mem).wrapping_sub(threshold as libc::c_ulong)
        as l_mem;
    luaE_setdebt(g, debt);
}
unsafe extern "C" fn singlestep(mut L: *mut lua_State_0) -> lu_mem {
    let mut g: *mut global_State = (*L).l_G;
    match (*g).gcstate as libc::c_int {
        7 => {
            (*g).GCmemtrav = ((*g).strt.size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut GCObject>() as libc::c_ulong);
            restartcollection(g);
            (*g).gcstate = 0i32 as lu_byte;
            return (*g).GCmemtrav;
        }
        0 => {
            (*g).GCmemtrav = 0i32 as lu_mem;
            if !(*g).gray.is_null() {
            } else {
                __assert_fail(
                    b"g->gray\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    1056i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                        b"lu_mem singlestep(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            propagatemark(g);
            /* no more gray objects? */
            if (*g).gray.is_null() {
                /* finish propagate phase */
                (*g).gcstate = 1i32 as lu_byte
            }
            /* memory traversed in this step */
            return (*g).GCmemtrav;
        }
        1 => {
            let mut work: lu_mem = 0;
            /* make sure gray list is empty */
            propagateall(g);
            /* work is what was traversed by 'atomic' */
            work = atomic(L) as lu_mem;
            entersweep(L);
            (*g).GCestimate = ((*g).totalbytes + (*g).GCdebt) as lu_mem;
            /* first estimate */
            return work;
        }
        2 => {
            /* sweep "regular" objects */
            return sweepstep(L, g, 3i32, &mut (*g).finobj);
        }
        3 => {
            /* sweep objects with finalizers */
            return sweepstep(L, g, 4i32, &mut (*g).tobefnz);
        }
        4 => {
            /* sweep objects to be finalized */
            return sweepstep(L, g, 5i32, 0 as *mut *mut GCObject);
        }
        5 => {
            /* finish sweeps */
            (*(*g).mainthread).marked = ((*(*g).mainthread).marked as libc::c_int
                & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32))
                | ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
                    as libc::c_int) as lu_byte;
            /* sweep main thread */
            checkSizes(L, g);
            (*g).gcstate = 6i32 as lu_byte;
            return 0i32 as lu_mem;
        }
        6 => {
            /* call remaining finalizers */
            if !(*g).tobefnz.is_null() && (*g).gckind as libc::c_int != 1i32 {
                let mut n: libc::c_int = runafewfinalizers(L);
                return (n as libc::c_ulong).wrapping_mul(
                    (::std::mem::size_of::<TString>() as libc::c_ulong)
                        .wrapping_add(4i32 as libc::c_ulong)
                        .wrapping_div(4i32 as libc::c_ulong),
                );
            } else {
                /* emergency mode or no more finalizers */
                /* finish collection */
                (*g).gcstate = 7i32 as lu_byte;
                return 0i32 as lu_mem;
            }
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    1095i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                        b"lu_mem singlestep(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            return 0i32 as lu_mem;
        }
    };
}
/* }====================================================== */
/*
** {======================================================
** Finalization
** =======================================================
*/
/*
** If possible, shrink string table
*/
unsafe extern "C" fn checkSizes(mut L: *mut lua_State_0, mut g: *mut global_State) -> () {
    if (*g).gckind as libc::c_int != 1i32 {
        let mut olddebt: l_mem = (*g).GCdebt;
        /* string table too big? */
        if (*g).strt.nuse < (*g).strt.size / 4i32 {
            /* shrink it a little */
            luaS_resize(L, (*g).strt.size / 2i32);
        }
        /* update estimate */
        (*g).GCestimate = ((*g).GCestimate as libc::c_ulong)
            .wrapping_add(((*g).GCdebt - olddebt) as libc::c_ulong)
            as lu_mem as lu_mem
    };
}
unsafe extern "C" fn sweepstep(
    mut L: *mut lua_State_0,
    mut g: *mut global_State,
    mut nextstate: libc::c_int,
    mut nextlist: *mut *mut GCObject,
) -> lu_mem {
    if !(*g).sweepgc.is_null() {
        let mut olddebt: l_mem = (*g).GCdebt;
        (*g).sweepgc = sweeplist(
            L,
            (*g).sweepgc,
            ((100i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TString>() as libc::c_ulong)
                as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<TString>() as libc::c_ulong)
                        .wrapping_add(4i32 as libc::c_ulong)
                        .wrapping_div(4i32 as libc::c_ulong),
                )
                .wrapping_div(4i32 as libc::c_ulong) as libc::c_int as lu_mem,
        );
        /* update estimate */
        (*g).GCestimate = ((*g).GCestimate as libc::c_ulong)
            .wrapping_add(((*g).GCdebt - olddebt) as libc::c_ulong)
            as lu_mem as lu_mem;
        /* is there still something to sweep? */
        if !(*g).sweepgc.is_null() {
            return (((100i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TString>() as libc::c_ulong)
                as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<TString>() as libc::c_ulong)
                        .wrapping_add(4i32 as libc::c_ulong)
                        .wrapping_div(4i32 as libc::c_ulong),
                )
                .wrapping_div(4i32 as libc::c_ulong) as libc::c_int
                as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<TString>() as libc::c_ulong)
                        .wrapping_add(4i32 as libc::c_ulong)
                        .wrapping_div(4i32 as libc::c_ulong),
                );
        }
    }
    /* else enter next state */
    (*g).gcstate = nextstate as lu_byte;
    (*g).sweepgc = nextlist;
    return 0i32 as lu_mem;
}
/*
** Enter first sweep phase.
** The call to 'sweeplist' tries to make pointer point to an object
** inside the list (instead of to the header), so that the real sweep do
** not need to skip objects created between "now" and the start of the
** real sweep.
*/
unsafe extern "C" fn entersweep(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    (*g).gcstate = 2i32 as lu_byte;
    if (*g).sweepgc.is_null() {
    } else {
        __assert_fail(
            b"g->sweepgc == ((void*)0)\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            962i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"void entersweep(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    (*g).sweepgc = sweeplist(L, &mut (*g).allgc, 1i32 as lu_mem);
}
/* collect fixed objects */
unsafe extern "C" fn atomic(mut L: *mut lua_State_0) -> l_mem {
    let mut g: *mut global_State = (*L).l_G;
    let mut work: l_mem = 0;
    let mut origweak: *mut GCObject = 0 as *mut GCObject;
    let mut origall: *mut GCObject = 0 as *mut GCObject;
    /* save original list */
    let mut grayagain: *mut GCObject = (*g).grayagain;
    if (*g).ephemeron.is_null() && (*g).weak.is_null() {
    } else {
        __assert_fail(
            b"g->ephemeron == ((void*)0) && g->weak == ((void*)0)\x00" as *const u8
                as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            987i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"l_mem atomic(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    if 0 == (*(*g).mainthread).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
    } else {
        __assert_fail(
            b"!(((g->mainthread)->marked) & (((1<<(0)) | (1<<(1)))))\x00" as *const u8
                as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            988i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"l_mem atomic(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    (*g).gcstate = (7i32 + 1i32) as lu_byte;
    /* start counting work */
    (*g).GCmemtrav = 0i32 as lu_mem;
    if 0 != (*L).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
        if (*L).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((L)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                991i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"l_mem atomic(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        reallymarkobject(g, &mut (*(L as *mut GCUnion)).gc);
    }
    /* mark running thread */
    if 0 == (*g).l_registry.tt_ & 1i32 << 6i32 || {
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                993i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"l_mem atomic(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        (*g).l_registry.tt_ & 0x3fi32 == (*(*g).l_registry.value_.gc).tt as libc::c_int
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                993i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"l_mem atomic(lua_State *)\x00",
                )).as_ptr(),
            );
        };
    };
    if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 && {
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                993i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"l_mem atomic(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*g).l_registry.value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
    } {
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                993i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"l_mem atomic(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        reallymarkobject(g, (*g).l_registry.value_.gc);
    }
    /* registry and global metatables may be changed by API */
    /* mark global metatables */
    markmt(g);
    /* remark occasional upvalues of (maybe) dead threads */
    remarkupvals(g);
    /* propagate changes */
    propagateall(g);
    /* stop counting (do not recount 'grayagain') */
    work = (*g).GCmemtrav as l_mem;
    (*g).gray = grayagain;
    /* traverse 'grayagain' list */
    propagateall(g);
    /* restart counting */
    (*g).GCmemtrav = 0i32 as lu_mem;
    convergeephemerons(g);
    /* at this point, all strongly accessible objects are marked. */
    /* Clear values from weak tables, before checking finalizers */
    clearvalues(g, (*g).weak, 0 as *mut GCObject);
    clearvalues(g, (*g).allweak, 0 as *mut GCObject);
    origweak = (*g).weak;
    origall = (*g).allweak;
    /* stop counting (objects being finalized) */
    work = (work as libc::c_ulong).wrapping_add((*g).GCmemtrav) as l_mem as l_mem;
    /* separate objects to be finalized */
    separatetobefnz(g, 0i32);
    /* there may be objects to be finalized */
    (*g).gcfinnum = 1i32 as libc::c_uint;
    /* mark objects that will be finalized */
    markbeingfnz(g);
    /* remark, to propagate 'resurrection' */
    propagateall(g);
    /* restart counting */
    (*g).GCmemtrav = 0i32 as lu_mem;
    convergeephemerons(g);
    /* at this point, all resurrected objects are marked. */
    /* remove dead objects from weak tables */
    /* clear keys from all ephemeron tables */
    clearkeys(g, (*g).ephemeron, 0 as *mut GCObject);
    /* clear keys from all 'allweak' tables */
    clearkeys(g, (*g).allweak, 0 as *mut GCObject);
    /* clear values from resurrected weak tables */
    clearvalues(g, (*g).weak, origweak);
    clearvalues(g, (*g).allweak, origall);
    luaS_clearcache(g);
    /* flip current white */
    (*g).currentwhite =
        ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte;
    /* complete counting */
    work = (work as libc::c_ulong).wrapping_add((*g).GCmemtrav) as l_mem as l_mem;
    /* estimate of memory marked by 'atomic' */
    return work;
}
/*
** clear entries with unmarked values from all weaktables in list 'l' up
** to element 'f'
*/
unsafe extern "C" fn clearvalues(
    mut g: *mut global_State,
    mut l: *mut GCObject,
    mut f: *mut GCObject,
) -> () {
    while l != f {
        if (*l).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(l)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                660i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"void clearvalues(global_State *, GCObject *, GCObject *)\x00",
                )).as_ptr(),
            );
        };
        let mut h: *mut Table_0 = &mut (*(l as *mut GCUnion)).h;
        let mut n: *mut Node = 0 as *mut Node;
        let mut limit: *mut Node = &mut *(*h)
            .node
            .offset((1i32 << (*h).lsizenode as libc::c_int) as size_t as isize)
            as *mut Node;
        let mut i: libc::c_uint = 0;
        i = 0i32 as libc::c_uint;
        while i < (*h).sizearray {
            let mut o: *mut TValue = &mut *(*h).array.offset(i as isize) as *mut TValue;
            /* value was collected? */
            if 0 != iscleared(g, o) {
                (*o).tt_ = 0i32
            }
            i = i.wrapping_add(1)
        }
        /* remove value */
        n = &mut *(*h).node.offset(0isize) as *mut Node;
        while n < limit {
            if !((*n).i_val.tt_ == 0i32) && 0 != iscleared(g, &mut (*n).i_val) {
                (*n).i_val.tt_ = 0i32;
                /* remove value ... */
                /* and remove entry from table */
                removeentry(n);
            }
            n = n.offset(1isize)
        }
        if (*l).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(l)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                659i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"void clearvalues(global_State *, GCObject *, GCObject *)\x00",
                )).as_ptr(),
            );
        };
        l = (*&mut (*(l as *mut GCUnion)).h).gclist
    }
}
/*
** {======================================================
** Generic functions
** =======================================================
*/
/*
** one after last element in a hash array
*/
/*
** link collectable object 'o' into list pointed by 'p'
*/
/*
** If key is not marked, mark its entry as dead. This allows key to be
** collected, but keeps its entry in the table.  A dead node is needed
** when Lua looks up for a key (it may be part of a chain) and when
** traversing a weak table (key might be removed from the table during
** traversal). Other places never manipulate dead keys, because its
** associated nil value is enough to signal that the entry is logically
** empty.
*/
unsafe extern "C" fn removeentry(mut n: *mut Node) -> () {
    if (*n).i_val.tt_ == 0i32 {
    } else {
        __assert_fail(
            b"(((((&(n)->i_val)))->tt_) == (0))\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            127i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"void removeentry(Node *)\x00",
            )).as_ptr(),
        );
    };
    if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32 && {
        if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00" as *const u8
                    as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                128i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"void removeentry(Node *)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
            .value_
            .gc)
            .marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
    } {
        (*n).i_key.nk.tt_ = 9i32 + 1i32
    };
}
/* unused and unmarked key; remove it */
/*
** tells whether a key or value can be cleared from a weak
** table. Non-collectable objects are never removed from weak
** tables. Strings behave as 'values', so are never removed too. for
** other objects: if really collected, cannot keep them; for objects
** being finalized, keep them in keys, but not in values
*/
unsafe extern "C" fn iscleared(mut g: *mut global_State, mut o: *const TValue) -> libc::c_int {
    if 0 == (*o).tt_ & 1i32 << 6i32 {
        return 0i32;
    } else if (*o).tt_ & 0xfi32 == 4i32 {
        if (*o).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                143i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"int iscleared(global_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                143i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"int iscleared(global_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != (*(&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString_0)).marked
            as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
        {
            if (*o).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    143i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                        b"int iscleared(global_State *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    143i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                        b"int iscleared(global_State *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString_0)).tt as libc::c_int
                & 0xfi32 < 9i32 + 1i32
            {
            } else {
                __assert_fail(b"(((((((((((((o))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((o))->tt_)) & 0x0F)) == (4))\", \"lgc.c\", 143, __extension__ __PRETTY_FUNCTION__)), (((((((((o)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((o)->value_).gc)->tt) & 0x0F) == 4\", \"lgc.c\", 143, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((o)->value_).gc))))->ts))))))->tt) & 0x0F) < (9+1)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 143i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
            };
            if (*o).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    143i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                        b"int iscleared(global_State *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    143i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                        b"int iscleared(global_State *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(
                g,
                &mut (*(&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString_0
                    as *mut GCUnion))
                    .gc,
            );
        }
        /* strings are 'values', so are never weak */
        return 0i32;
    } else {
        if 0 != (*o).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                146i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"int iscleared(global_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        return (*(*o).value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32);
    };
}
/*
** $Id: lgc.c,v 2.214 2016/11/07 12:38:35 roberto Exp roberto $
** Garbage Collector
** See Copyright Notice in lua.h
*/
/*
** internal state for collector while inside the atomic phase. The
** collector should never be in this state while running regular code.
*/
/*
** cost of sweeping one element (the size of a small object divided
** by some adjust for the sweep speed)
*/
/* maximum number of elements to sweep in each single step */
/* cost of calling one finalizer */
/*
** macro to adjust 'stepmul': 'stepmul' is actually used like
** 'stepmul / STEPMULADJ' (value chosen by tests)
*/
/*
** macro to adjust 'pause': 'pause' is actually used like
** 'pause / PAUSEADJ' (value chosen by tests)
*/
/*
** 'makewhite' erases all color bits then sets only the current white
** bit
*/
/*
** mark an object that can be NULL (either because it is really optional,
** or it was stripped as debug info, or inside an uncompleted structure)
*/
unsafe extern "C" fn reallymarkobject(mut g: *mut global_State, mut o: *mut GCObject) -> () {
    loop {
        (*o).marked = ((*o).marked as libc::c_int
            & !(1i32 << 0i32 | 1i32 << 1i32) as lu_byte as libc::c_int)
            as lu_byte;
        match (*o).tt as libc::c_int {
            4 => {
                (*o).marked = ((*o).marked as libc::c_int | 1i32 << 2i32) as lu_byte;
                if (*o).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        242i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                (*g).GCmemtrav = ((*g).GCmemtrav as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<UTString>() as libc::c_ulong).wrapping_add(
                        (((*&mut (*(o as *mut GCUnion)).ts).shrlen as libc::c_int + 1i32)
                            as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
                    ),
                ) as lu_mem as lu_mem;
                break;
            }
            20 => {
                (*o).marked = ((*o).marked as libc::c_int | 1i32 << 2i32) as lu_byte;
                if (*o).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        247i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                (*g).GCmemtrav = ((*g).GCmemtrav as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<UTString>() as libc::c_ulong).wrapping_add(
                        (*&mut (*(o as *mut GCUnion)).ts)
                            .u
                            .lnglen
                            .wrapping_add(1i32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
                    ),
                ) as lu_mem as lu_mem;
                break;
            }
            7 => {
                let mut uvalue: TValue = lua_TValue {
                    value_: Value_0 {
                        gc: 0 as *const GCObject as *mut GCObject,
                    },
                    tt_: 0,
                };
                if (*o).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        252i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                if !(*(&mut (*(o as *mut GCUnion)).u as *mut Udata))
                    .metatable
                    .is_null()
                {
                    if (*o).tt as libc::c_int == 7i32 {
                    } else {
                        __assert_fail(
                            b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            252i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                b"void reallymarkobject(global_State *, GCObject *)\x00",
                            )).as_ptr(),
                        );
                    };
                    if 0 != (*(*(&mut (*(o as *mut GCUnion)).u as *mut Udata)).metatable).marked
                        as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                    {
                        if (*o).tt as libc::c_int == 7i32 {
                        } else {
                            __assert_fail(
                                b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                                252i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                    b"void reallymarkobject(global_State *, GCObject *)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*(&mut (*(o as *mut GCUnion)).u as *mut Udata)).metatable).tt
                            as libc::c_int & 0xfi32 < 9i32 + 1i32
                        {
                        } else {
                            __assert_fail(b"(((((((o)->tt == 7) ? (void) (0) : __assert_fail (\"(o)->tt == 7\", \"lgc.c\", 252, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->u)))->metatable)->tt) & 0x0F) < (9+1)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          252i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 50],
                                                                    &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                        };
                        if (*o).tt as libc::c_int == 7i32 {
                        } else {
                            __assert_fail(
                                b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                                252i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                    b"void reallymarkobject(global_State *, GCObject *)\x00",
                                )).as_ptr(),
                            );
                        };
                        reallymarkobject(
                            g,
                            &mut (*((*(&mut (*(o as *mut GCUnion)).u as *mut Udata)).metatable
                                as *mut GCUnion))
                                .gc,
                        );
                    }
                }
                /* mark its metatable */
                (*o).marked = ((*o).marked as libc::c_int | 1i32 << 2i32) as lu_byte;
                if (*o).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        254i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                (*g).GCmemtrav = ((*g).GCmemtrav as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<UUdata>() as libc::c_ulong)
                        .wrapping_add((*&mut (*(o as *mut GCUnion)).u).len),
                ) as lu_mem as lu_mem;
                let mut io: *mut TValue = &mut uvalue;
                if (*o).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        255i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                let mut iu: *const Udata_0 = &mut (*(o as *mut GCUnion)).u;
                (*io).value_ = (*iu).user_;
                (*io).tt_ = (*iu).ttuv_ as libc::c_int;
                if 0 == (*io).tt_ & 1i32 << 6i32 || {
                    if 0 != (*io).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            255i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                b"void reallymarkobject(global_State *, GCObject *)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
                        && ((*g).mainthread.is_null() || {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(
                                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                                    255i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                        b"void reallymarkobject(global_State *, GCObject *)\x00",
                                    )).as_ptr(),
                                );
                            };
                            0 != ((*(*io).value_.gc).marked as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                                & ((*(*(*g).mainthread).l_G).currentwhite as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                        })
                } {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(
                            b"0\x00" as *const u8 as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            255i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                b"void reallymarkobject(global_State *, GCObject *)\x00",
                            )).as_ptr(),
                        );
                    };
                };
                if !(0 != uvalue.tt_ & 1i32 << 6i32 && {
                    if 0 != uvalue.tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((&uvalue)->tt_) & (1 << 6))\x00" as *const u8
                                as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            256i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                b"void reallymarkobject(global_State *, GCObject *)\x00",
                            )).as_ptr(),
                        );
                    };
                    0 != (*uvalue.value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                }) {
                    break;
                }
                /* markvalue(g, &uvalue); */
                if 0 != uvalue.tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((&uvalue)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        257i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                o = uvalue.value_.gc
            }
            6 => {
                if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        263i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                let ref mut fresh1 = (*&mut (*(o as *mut GCUnion)).cl.l).gclist;
                *fresh1 = (*g).gray;
                if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        263i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(&mut (*(o as *mut GCUnion)).cl.l as *mut LClosure_0)).tt as libc::c_int
                    & 0xfi32 < 9i32 + 1i32
                {
                } else {
                    __assert_fail(b"(((((((o)->tt == (6 | (0 << 4))) ? (void) (0) : __assert_fail (\"(o)->tt == (6 | (0 << 4))\", \"lgc.c\", 263, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->cl.l))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  263i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        263i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                (*g).gray = &mut (*(&mut (*(o as *mut GCUnion)).cl.l as *mut LClosure_0
                    as *mut GCUnion))
                    .gc;
                break;
            }
            38 => {
                if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        267i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                let ref mut fresh2 = (*&mut (*(o as *mut GCUnion)).cl.c).gclist;
                *fresh2 = (*g).gray;
                if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        267i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(&mut (*(o as *mut GCUnion)).cl.c as *mut CClosure_0)).tt as libc::c_int
                    & 0xfi32 < 9i32 + 1i32
                {
                } else {
                    __assert_fail(b"(((((((o)->tt == (6 | (2 << 4))) ? (void) (0) : __assert_fail (\"(o)->tt == (6 | (2 << 4))\", \"lgc.c\", 267, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->cl.c))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  267i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        267i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                (*g).gray = &mut (*(&mut (*(o as *mut GCUnion)).cl.c as *mut CClosure_0
                    as *mut GCUnion))
                    .gc;
                break;
            }
            5 => {
                if (*o).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        271i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                let ref mut fresh3 = (*&mut (*(o as *mut GCUnion)).h).gclist;
                *fresh3 = (*g).gray;
                if (*o).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        271i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(&mut (*(o as *mut GCUnion)).h as *mut Table)).tt as libc::c_int & 0xfi32
                    < 9i32 + 1i32
                {
                } else {
                    __assert_fail(b"(((((((o)->tt == 5) ? (void) (0) : __assert_fail (\"(o)->tt == 5\", \"lgc.c\", 271, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->h))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  271i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        271i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).h as *mut Table as *mut GCUnion)).gc;
                break;
            }
            8 => {
                if (*o).tt as libc::c_int == 8i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 8\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        275i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                let ref mut fresh4 = (*&mut (*(o as *mut GCUnion)).th).gclist;
                *fresh4 = (*g).gray;
                if (*o).tt as libc::c_int == 8i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 8\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        275i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(&mut (*(o as *mut GCUnion)).th as *mut lua_State)).tt as libc::c_int & 0xfi32
                    < 9i32 + 1i32
                {
                } else {
                    __assert_fail(b"(((((((o)->tt == 8) ? (void) (0) : __assert_fail (\"(o)->tt == 8\", \"lgc.c\", 275, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->th))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  275i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 8i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 8\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        275i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).th as *mut lua_State as *mut GCUnion)).gc;
                break;
            }
            9 => {
                if (*o).tt as libc::c_int == 9i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        279i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                let ref mut fresh5 = (*&mut (*(o as *mut GCUnion)).p).gclist;
                *fresh5 = (*g).gray;
                if (*o).tt as libc::c_int == 9i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        279i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(&mut (*(o as *mut GCUnion)).p as *mut Proto)).tt as libc::c_int & 0xfi32
                    < 9i32 + 1i32
                {
                } else {
                    __assert_fail(b"(((((((o)->tt == 9) ? (void) (0) : __assert_fail (\"(o)->tt == 9\", \"lgc.c\", 279, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->p))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  279i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 9i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        279i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).p as *mut Proto as *mut GCUnion)).gc;
                break;
            }
            _ => {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        282i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void reallymarkobject(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                break;
            }
        }
    }
}
/* }====================================================== */
/*
** {======================================================
** Sweep Functions
** =======================================================
*/
/*
** clear entries with unmarked keys from all weaktables in list 'l' up
** to element 'f'
*/
unsafe extern "C" fn clearkeys(
    mut g: *mut global_State,
    mut l: *mut GCObject,
    mut f: *mut GCObject,
) -> () {
    while l != f {
        if (*l).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(l)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                642i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                    b"void clearkeys(global_State *, GCObject *, GCObject *)\x00",
                )).as_ptr(),
            );
        };
        let mut h: *mut Table_0 = &mut (*(l as *mut GCUnion)).h;
        let mut n: *mut Node = 0 as *mut Node;
        let mut limit: *mut Node = &mut *(*h)
            .node
            .offset((1i32 << (*h).lsizenode as libc::c_int) as size_t as isize)
            as *mut Node;
        n = &mut *(*h).node.offset(0isize) as *mut Node;
        while n < limit {
            if !((*n).i_val.tt_ == 0i32)
                && 0 != iscleared(g, &mut (*n).i_key.tvk as *mut TValue as *const TValue)
            {
                (*n).i_val.tt_ = 0i32;
                /* remove value ... */
                /* and remove entry from table */
                removeentry(n);
            }
            n = n.offset(1isize)
        }
        if (*l).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(l)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                641i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                    b"void clearkeys(global_State *, GCObject *, GCObject *)\x00",
                )).as_ptr(),
            );
        };
        l = (*&mut (*(l as *mut GCUnion)).h).gclist
    }
}
unsafe extern "C" fn convergeephemerons(mut g: *mut global_State) -> () {
    let mut changed: libc::c_int = 0;
    loop {
        let mut w: *mut GCObject = 0 as *mut GCObject;
        /* get ephemeron list */
        let mut next: *mut GCObject = (*g).ephemeron;
        /* tables may return to this list when traversed */
        (*g).ephemeron = 0 as *mut GCObject;
        changed = 0i32;
        loop {
            w = next;
            if w.is_null() {
                break;
            }
            if (*w).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(w)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    617i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void convergeephemerons(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            next = (*&mut (*(w as *mut GCUnion)).h).gclist;
            if (*w).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(w)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    618i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void convergeephemerons(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            if !(0 != traverseephemeron(g, &mut (*(w as *mut GCUnion)).h)) {
                continue;
            }
            /* traverse marked some value? */
            /* propagate changes */
            propagateall(g);
            /* will have to revisit all ephemeron tables */
            changed = 1i32
        }
        if !(0 != changed) {
            break;
        }
    }
}
unsafe extern "C" fn propagateall(mut g: *mut global_State) -> () {
    while !(*g).gray.is_null() {
        propagatemark(g);
    }
}
/*
** traverse one gray object, turning it to black (except for threads,
** which are always gray).
*/
unsafe extern "C" fn propagatemark(mut g: *mut global_State) -> () {
    let mut size: lu_mem = 0;
    let mut o: *mut GCObject = (*g).gray;
    if 0 == (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32) {
    } else {
        __assert_fail(
            b"(!(((o)->marked) & (((1<<(0)) | (1<<(1))) | (1<<(2)))))\x00" as *const u8
                as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            563i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"void propagatemark(global_State *)\x00",
            )).as_ptr(),
        );
    };
    (*o).marked = ((*o).marked as libc::c_int | 1i32 << 2i32) as lu_byte;
    match (*o).tt as libc::c_int {
        5 => {
            if (*o).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    567i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void propagatemark(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            let mut h: *mut Table_0 = &mut (*(o as *mut GCUnion)).h;
            /* remove from 'gray' list */
            (*g).gray = (*h).gclist;
            size = traversetable(g, h)
        }
        6 => {
            if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    573i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void propagatemark(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            let mut cl: *mut LClosure_0 = &mut (*(o as *mut GCUnion)).cl.l;
            /* remove from 'gray' list */
            (*g).gray = (*cl).gclist;
            size = traverseLclosure(g, cl)
        }
        38 => {
            if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    579i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void propagatemark(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            let mut cl_0: *mut CClosure_0 = &mut (*(o as *mut GCUnion)).cl.c;
            /* remove from 'gray' list */
            (*g).gray = (*cl_0).gclist;
            size = traverseCclosure(g, cl_0)
        }
        8 => {
            if (*o).tt as libc::c_int == 8i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == 8\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    585i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void propagatemark(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            let mut th: *mut lua_State_0 = &mut (*(o as *mut GCUnion)).th;
            /* remove from 'gray' list */
            (*g).gray = (*th).gclist;
            (*th).gclist = (*g).grayagain;
            if (*th).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((th)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    587i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void propagatemark(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            (*g).grayagain = &mut (*(th as *mut GCUnion)).gc;
            (*o).marked =
                ((*o).marked as libc::c_int & !(1i32 << 2i32) as lu_byte as libc::c_int) as lu_byte;
            /* insert into 'grayagain' list */
            size = traversethread(g, th)
        }
        9 => {
            if (*o).tt as libc::c_int == 9i32 {
            } else {
                __assert_fail(
                    b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    593i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void propagatemark(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            let mut p: *mut Proto_0 = &mut (*(o as *mut GCUnion)).p;
            /* remove from 'gray' list */
            (*g).gray = (*p).gclist;
            size = traverseproto(g, p) as lu_mem
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    598i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void propagatemark(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            return;
        }
    }
    (*g).GCmemtrav = ((*g).GCmemtrav as libc::c_ulong).wrapping_add(size) as lu_mem as lu_mem;
}
/*
** Traverse a prototype. (While a prototype is being build, its
** arrays can be larger than needed; the extra slots are filled with
** NULL, so the use of 'markobjectN')
*/
unsafe extern "C" fn traverseproto(mut g: *mut global_State, mut f: *mut Proto_0) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if !(*f).cache.is_null()
        && 0 != (*(*f).cache).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        /* allow cache to be collected */
        (*f).cache = 0 as *mut LClosure
    }
    if !(*f).source.is_null() {
        if 0 != (*(*f).source).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
            if (*(*f).source).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((f->source)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    484i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"int traverseproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, &mut (*((*f).source as *mut GCUnion)).gc);
        }
    }
    /* mark literals */
    i = 0i32;
    while i < (*f).sizek {
        if 0 == (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 || {
            if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&f->k[i])->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    486i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"int traverseproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            (*(*f).k.offset(i as isize)).tt_ & 0x3fi32
                == (*(*(*f).k.offset(i as isize)).value_.gc).tt as libc::c_int
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    486i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"int traverseproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
        };
        if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 && {
            if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&f->k[i])->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    486i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"int traverseproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            0 != (*(*(*f).k.offset(i as isize)).value_.gc).marked as libc::c_int
                & (1i32 << 0i32 | 1i32 << 1i32)
        } {
            if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&f->k[i])->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    486i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"int traverseproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, (*(*f).k.offset(i as isize)).value_.gc);
        }
        i += 1
    }
    /* mark upvalue names */
    i = 0i32;
    while i < (*f).sizeupvalues {
        if !(*(*f).upvalues.offset(i as isize)).name.is_null() {
            if 0 != (*(*(*f).upvalues.offset(i as isize)).name).marked as libc::c_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                if (*(*(*f).upvalues.offset(i as isize)).name).tt as libc::c_int & 0xfi32
                    < 9i32 + 1i32
                {
                } else {
                    __assert_fail(
                        b"(((f->upvalues[i].name)->tt) & 0x0F) < (9+1)\x00" as *const u8
                            as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        488i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                            b"int traverseproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
                reallymarkobject(
                    g,
                    &mut (*((*(*f).upvalues.offset(i as isize)).name as *mut GCUnion)).gc,
                );
            }
        }
        i += 1
    }
    /* mark nested protos */
    i = 0i32;
    while i < (*f).sizep {
        if !(*(*f).p.offset(i as isize)).is_null() {
            if 0 != (**(*f).p.offset(i as isize)).marked as libc::c_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                if (**(*f).p.offset(i as isize)).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(
                        b"(((f->p[i])->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        490i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                            b"int traverseproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
                reallymarkobject(g, &mut (*(*(*f).p.offset(i as isize) as *mut GCUnion)).gc);
            }
        }
        i += 1
    }
    /* mark local-variable names */
    i = 0i32;
    while i < (*f).sizelocvars {
        if !(*(*f).locvars.offset(i as isize)).varname.is_null() {
            if 0 != (*(*(*f).locvars.offset(i as isize)).varname).marked as libc::c_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                if (*(*(*f).locvars.offset(i as isize)).varname).tt as libc::c_int & 0xfi32
                    < 9i32 + 1i32
                {
                } else {
                    __assert_fail(
                        b"(((f->locvars[i].varname)->tt) & 0x0F) < (9+1)\x00" as *const u8
                            as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        492i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                            b"int traverseproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
                reallymarkobject(
                    g,
                    &mut (*((*(*f).locvars.offset(i as isize)).varname as *mut GCUnion)).gc,
                );
            }
        }
        i += 1
    }
    return (::std::mem::size_of::<Proto_0>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<Instruction>() as libc::c_ulong)
                .wrapping_mul((*f).sizecode as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<*mut Proto_0>() as libc::c_ulong)
                .wrapping_mul((*f).sizep as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<TValue>() as libc::c_ulong)
                .wrapping_mul((*f).sizek as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul((*f).sizelineinfo as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<LocVar_0>() as libc::c_ulong)
                .wrapping_mul((*f).sizelocvars as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<Upvaldesc_0>() as libc::c_ulong)
                .wrapping_mul((*f).sizeupvalues as libc::c_ulong),
        ) as libc::c_int;
}
unsafe extern "C" fn traversethread(mut g: *mut global_State, mut th: *mut lua_State_0) -> lu_mem {
    let mut o: StkId = (*th).stack;
    if o.is_null() {
        /* stack not completely built yet */
        return 1i32 as lu_mem;
    } else {
        if (*g).gcstate as libc::c_int == 7i32 + 1i32
            || (*th).openupval.is_null()
            || (*th).twups != th
        {
        } else {
            __assert_fail(
                b"g->gcstate == (7 + 1) || th->openupval == ((void*)0) || (th->twups != th)\x00"
                    as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                536i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"lu_mem traversethread(global_State *, lua_State *)\x00",
                )).as_ptr(),
            );
        };
        /* mark live elements in the stack */
        while o < (*th).top {
            if 0 == (*o).tt_ & 1i32 << 6i32 || {
                if 0 != (*o).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        538i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                            b"lu_mem traversethread(global_State *, lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                (*o).tt_ & 0x3fi32 == (*(*o).value_.gc).tt as libc::c_int
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        538i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                            b"lu_mem traversethread(global_State *, lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            if 0 != (*o).tt_ & 1i32 << 6i32 && {
                if 0 != (*o).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        538i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                            b"lu_mem traversethread(global_State *, lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*o).value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                if 0 != (*o).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        538i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                            b"lu_mem traversethread(global_State *, lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                reallymarkobject(g, (*o).value_.gc);
            }
            o = o.offset(1isize)
        }
        if (*g).gcstate as libc::c_int == 7i32 + 1i32 {
            /* final traversal? */
            /* real end of stack */
            let mut lim: StkId = (*th).stack.offset((*th).stacksize as isize);
            /* clear not-marked stack slice */
            while o < lim {
                (*o).tt_ = 0i32;
                o = o.offset(1isize)
            }
            /* 'remarkupvals' may have removed thread from 'twups' list */
            if !((*th).twups != th) && !(*th).openupval.is_null() {
                /* link it back to the list */
                (*th).twups = (*g).twups;
                (*g).twups = th
            }
        } else if (*g).gckind as libc::c_int != 1i32 {
            /* do not change stack in emergency cycle */
            luaD_shrinkstack(th);
        }
        return (::std::mem::size_of::<lua_State_0>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<TValue>() as libc::c_ulong)
                    .wrapping_mul((*th).stacksize as libc::c_ulong),
            )
            .wrapping_add(
                (::std::mem::size_of::<CallInfo_0>() as libc::c_ulong)
                    .wrapping_mul((*th).nci as libc::c_ulong),
            );
    };
}
unsafe extern "C" fn traverseCclosure(mut g: *mut global_State, mut cl: *mut CClosure_0) -> lu_mem {
    let mut i: libc::c_int = 0;
    /* mark its upvalues */
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        if 0 == (*(*cl).upvalue.as_mut_ptr().offset(i as isize)).tt_ & 1i32 << 6i32 || {
            if 0 != (*(*cl).upvalue.as_mut_ptr().offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&cl->upvalue[i])->tt_) & (1 << 6))\x00" as *const u8
                        as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    505i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"lu_mem traverseCclosure(global_State *, CClosure *)\x00",
                    )).as_ptr(),
                );
            };
            (*(*cl).upvalue.as_mut_ptr().offset(i as isize)).tt_ & 0x3fi32
                == (*(*(*cl).upvalue.as_mut_ptr().offset(i as isize)).value_.gc).tt as libc::c_int
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    505i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"lu_mem traverseCclosure(global_State *, CClosure *)\x00",
                    )).as_ptr(),
                );
            };
        };
        if 0 != (*(*cl).upvalue.as_mut_ptr().offset(i as isize)).tt_ & 1i32 << 6i32 && {
            if 0 != (*(*cl).upvalue.as_mut_ptr().offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&cl->upvalue[i])->tt_) & (1 << 6))\x00" as *const u8
                        as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    505i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"lu_mem traverseCclosure(global_State *, CClosure *)\x00",
                    )).as_ptr(),
                );
            };
            0 != (*(*(*cl).upvalue.as_mut_ptr().offset(i as isize)).value_.gc).marked as libc::c_int
                & (1i32 << 0i32 | 1i32 << 1i32)
        } {
            if 0 != (*(*cl).upvalue.as_mut_ptr().offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&cl->upvalue[i])->tt_) & (1 << 6))\x00" as *const u8
                        as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    505i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"lu_mem traverseCclosure(global_State *, CClosure *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(
                g,
                (*(*cl).upvalue.as_mut_ptr().offset(i as isize)).value_.gc,
            );
        }
        i += 1
    }
    return (::std::mem::size_of::<CClosure_0>() as libc::c_ulong as libc::c_int
        + (::std::mem::size_of::<TValue>() as libc::c_ulong)
            .wrapping_mul((*cl).nupvalues as libc::c_ulong) as libc::c_int) as lu_mem;
}
/*
** open upvalues point to values in a thread, so those values should
** be marked when the thread is traversed except in the atomic phase
** (because then the value cannot be changed by the thread and the
** thread may not be traversed again)
*/
unsafe extern "C" fn traverseLclosure(mut g: *mut global_State, mut cl: *mut LClosure_0) -> lu_mem {
    let mut i: libc::c_int = 0;
    if !(*cl).p.is_null() {
        if 0 != (*(*cl).p).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
            if (*(*cl).p).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((cl->p)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    517i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                        b"lu_mem traverseLclosure(global_State *, LClosure *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, &mut (*((*cl).p as *mut GCUnion)).gc);
        }
    }
    /* mark its prototype */
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        /* mark its upvalues */
        let mut uv: *mut UpVal = *(*cl).upvals.as_mut_ptr().offset(i as isize);
        if !uv.is_null() {
            if (*uv).v != &mut (*uv).u.value as *mut TValue
                && (*g).gcstate as libc::c_int != 7i32 + 1i32
            {
                /* can be marked in 'remarkupvals' */
                (*uv).u.open.touched = 1i32
            } else {
                if 0 == (*(*uv).v).tt_ & 1i32 << 6i32 || {
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((uv->v)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            524i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                                b"lu_mem traverseLclosure(global_State *, LClosure *)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*(*uv).v).tt_ & 0x3fi32 == (*(*(*uv).v).value_.gc).tt as libc::c_int
                } {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(
                            b"0\x00" as *const u8 as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            524i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                                b"lu_mem traverseLclosure(global_State *, LClosure *)\x00",
                            )).as_ptr(),
                        );
                    };
                };
                if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 && {
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((uv->v)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            524i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                                b"lu_mem traverseLclosure(global_State *, LClosure *)\x00",
                            )).as_ptr(),
                        );
                    };
                    0 != (*(*(*uv).v).value_.gc).marked as libc::c_int
                        & (1i32 << 0i32 | 1i32 << 1i32)
                } {
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((uv->v)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lgc.c\x00" as *const u8 as *const libc::c_char,
                            524i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                                b"lu_mem traverseLclosure(global_State *, LClosure *)\x00",
                            )).as_ptr(),
                        );
                    };
                    reallymarkobject(g, (*(*uv).v).value_.gc);
                }
            }
        }
        i += 1
    }
    return (::std::mem::size_of::<LClosure_0>() as libc::c_ulong as libc::c_int
        + (::std::mem::size_of::<*mut TValue>() as libc::c_ulong)
            .wrapping_mul((*cl).nupvalues as libc::c_ulong) as libc::c_int) as lu_mem;
}
unsafe extern "C" fn traversetable(mut g: *mut global_State, mut h: *mut Table_0) -> lu_mem {
    let mut weakkey: *const libc::c_char = 0 as *const libc::c_char;
    let mut weakvalue: *const libc::c_char = 0 as *const libc::c_char;
    let mut mode: *const TValue = if (*h).metatable.is_null() {
        0 as *const TValue
    } else if 0 != (*(*h).metatable).flags as libc::c_uint & 1u32 << TM_MODE as libc::c_int {
        0 as *const TValue
    } else {
        luaT_gettm(
            (*h).metatable,
            TM_MODE,
            (*g).tmname[TM_MODE as libc::c_int as usize],
        )
    };
    if !(*h).metatable.is_null() {
        if 0 != (*(*h).metatable).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
            if (*(*h).metatable).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((h->metatable)->tt) & 0x0F) < (9+1)\x00" as *const u8
                        as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    455i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                        b"lu_mem traversetable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, &mut (*((*h).metatable as *mut GCUnion)).gc);
        }
    }
    /* is there a weak mode? */
    if !mode.is_null() && (*mode).tt_ & 0xfi32 == 4i32 && {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((((((((((((mode))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((mode))->tt_)) & 0x0F)) == (4))\", \"lgc.c\", 457, __extension__ __PRETTY_FUNCTION__)), (((((((((mode)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((mode)->value_).gc)->tt) & 0x0F) == 4\", \"lgc.c\", 457, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((mode)->value_).gc))))->ts))))))->extra)\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 457i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 46],
                                                           &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
        };
        if (*mode).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((mode))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                457i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"lu_mem traversetable(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*mode).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((mode)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                    as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                457i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"lu_mem traversetable(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        weakkey = strchr(
            (&mut (*((*mode).value_.gc as *mut GCUnion)).ts as *mut TString_0 as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString_0>() as libc::c_ulong as isize),
            'k' as i32,
        );
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((((((((((((mode))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((mode))->tt_)) & 0x0F)) == (4))\", \"lgc.c\", 458, __extension__ __PRETTY_FUNCTION__)), (((((((((mode)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((mode)->value_).gc)->tt) & 0x0F) == 4\", \"lgc.c\", 458, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((mode)->value_).gc))))->ts))))))->extra)\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 458i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 46],
                                                           &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
        };
        if (*mode).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((mode))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                458i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"lu_mem traversetable(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*mode).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((mode)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                    as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                458i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"lu_mem traversetable(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        weakvalue = strchr(
            (&mut (*((*mode).value_.gc as *mut GCUnion)).ts as *mut TString_0 as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString_0>() as libc::c_ulong as isize),
            'v' as i32,
        );
        !weakkey.is_null() || !weakvalue.is_null()
    } {
        /* is really weak? */
        (*h).marked =
            ((*h).marked as libc::c_int & !(1i32 << 2i32) as lu_byte as libc::c_int) as lu_byte;
        /* keep table gray */
        /* strong keys? */
        if weakkey.is_null() {
            traverseweakvalue(g, h);
        } else if weakvalue.is_null() {
            traverseephemeron(g, h);
        } else {
            (*h).gclist = (*g).allweak;
            if (*h).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    466i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                        b"lu_mem traversetable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            (*g).allweak = &mut (*(h as *mut GCUnion)).gc
        }
    } else {
        /* all weak */
        /* nothing to traverse now */
        /* not weak */
        traversestrongtable(g, h);
    }
    return (::std::mem::size_of::<Table_0>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<TValue>() as libc::c_ulong)
                .wrapping_mul((*h).sizearray as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<Node>() as libc::c_ulong).wrapping_mul(
                (if (*h).lastfree.is_null() {
                    0i32
                } else {
                    1i32 << (*h).lsizenode as libc::c_int
                }) as size_t,
            ),
        );
}
unsafe extern "C" fn traversestrongtable(mut g: *mut global_State, mut h: *mut Table_0) -> () {
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *(*h)
        .node
        .offset((1i32 << (*h).lsizenode as libc::c_int) as size_t as isize)
        as *mut Node;
    let mut i: libc::c_uint = 0;
    /* traverse array part */
    i = 0i32 as libc::c_uint;
    while i < (*h).sizearray {
        if 0 == (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 || {
            if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&h->array[i])->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    438i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void traversestrongtable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            (*(*h).array.offset(i as isize)).tt_ & 0x3fi32
                == (*(*(*h).array.offset(i as isize)).value_.gc).tt as libc::c_int
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    438i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void traversestrongtable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
        };
        if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 && {
            if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&h->array[i])->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    438i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void traversestrongtable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            0 != (*(*(*h).array.offset(i as isize)).value_.gc).marked as libc::c_int
                & (1i32 << 0i32 | 1i32 << 1i32)
        } {
            if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&h->array[i])->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    438i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void traversestrongtable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, (*(*h).array.offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1)
    }
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        /* traverse hash part */
        if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 9i32 + 1i32)
            || (*n).i_val.tt_ == 0i32
        {
        } else {
            __assert_fail(b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == ((9+1))) || (((((&(n)->i_val)))->tt_) == (0))\x00"
                              as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          440i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 50],
                                                    &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
        };
        /* entry is empty? */
        if (*n).i_val.tt_ == 0i32 {
            /* remove it */
            removeentry(n);
        } else {
            if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 0i32) {
            } else {
                __assert_fail(
                    b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == (0))\x00" as *const u8
                        as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    444i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void traversestrongtable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            if 0 == (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32 || {
                if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
                {
                } else {
                    __assert_fail(
                        b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                            as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        445i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void traversestrongtable(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 0x3fi32
                    == (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                        .value_
                        .gc)
                        .tt as libc::c_int
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        445i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void traversestrongtable(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32 && {
                if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
                {
                } else {
                    __assert_fail(
                        b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                            as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        445i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void traversestrongtable(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                    .value_
                    .gc)
                    .marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
                {
                } else {
                    __assert_fail(
                        b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                            as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        445i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void traversestrongtable(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                reallymarkobject(
                    g,
                    (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                        .value_
                        .gc,
                );
            }
            /* mark key */
            if 0 == (*n).i_val.tt_ & 1i32 << 6i32 || {
                if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((&(n)->i_val))->tt_) & (1 << 6))\x00" as *const u8
                            as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        446i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void traversestrongtable(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                (*n).i_val.tt_ & 0x3fi32 == (*(*n).i_val.value_.gc).tt as libc::c_int
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        446i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void traversestrongtable(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32 && {
                if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((&(n)->i_val))->tt_) & (1 << 6))\x00" as *const u8
                            as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        446i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void traversestrongtable(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*n).i_val.value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((&(n)->i_val))->tt_) & (1 << 6))\x00" as *const u8
                            as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        446i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void traversestrongtable(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                reallymarkobject(g, (*n).i_val.value_.gc);
            }
        }
        /* mark value */
        n = n.offset(1isize)
    }
}
/* has to be cleared later */
/*
** Traverse an ephemeron table and link it to proper list. Returns true
** iff any object was marked during this traversal (which implies that
** convergence has to continue). During propagation phase, keep table
** in 'grayagain' list, to be visited again in the atomic phase. In
** the atomic phase, if table has any white->white entry, it has to
** be revisited during ephemeron convergence (as that key may turn
** black). Otherwise, if it has any white key, table has to be cleared
** (in the atomic phase).
*/
unsafe extern "C" fn traverseephemeron(
    mut g: *mut global_State,
    mut h: *mut Table_0,
) -> libc::c_int {
    /* true if an object is marked in this traversal */
    let mut marked: libc::c_int = 0i32;
    /* true if table has white keys */
    let mut hasclears: libc::c_int = 0i32;
    /* true if table has entry "white-key -> white-value" */
    let mut hasww: libc::c_int = 0i32;
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *(*h)
        .node
        .offset((1i32 << (*h).lsizenode as libc::c_int) as size_t as isize)
        as *mut Node;
    let mut i: libc::c_uint = 0;
    /* traverse array part */
    i = 0i32 as libc::c_uint;
    while i < (*h).sizearray {
        if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 && {
            if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&h->array[i])->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    403i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"int traverseephemeron(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            0 != (*(*(*h).array.offset(i as isize)).value_.gc).marked as libc::c_int
                & (1i32 << 0i32 | 1i32 << 1i32)
        } {
            marked = 1i32;
            if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((&h->array[i])->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    405i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"int traverseephemeron(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, (*(*h).array.offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1)
    }
    /* traverse hash part */
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 9i32 + 1i32)
            || (*n).i_val.tt_ == 0i32
        {
        } else {
            __assert_fail(b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == ((9+1))) || (((((&(n)->i_val)))->tt_) == (0))\x00"
                              as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          410i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
        };
        /* entry is empty? */
        if (*n).i_val.tt_ == 0i32 {
            /* remove it */
            removeentry(n);
        } else if 0 != iscleared(g, &mut (*n).i_key.tvk as *mut TValue as *const TValue) {
            /* key is not marked (yet)? */
            /* table must be cleared */
            hasclears = 1i32;
            /* value not marked yet? */
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32 && {
                if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((&(n)->i_val))->tt_) & (1 << 6))\x00" as *const u8
                            as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        415i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                            b"int traverseephemeron(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*n).i_val.value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                /* white-white entry */
                hasww = 1i32
            }
        } else if 0 != (*n).i_val.tt_ & 1i32 << 6i32 && {
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((&(n)->i_val))->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    418i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"int traverseephemeron(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            0 != (*(*n).i_val.value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
        } {
            /* value not marked yet? */
            marked = 1i32;
            /* mark it now */
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((&(n)->i_val))->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    420i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"int traverseephemeron(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, (*n).i_val.value_.gc);
        }
        n = n.offset(1isize)
    }
    /* link table into proper list */
    if (*g).gcstate as libc::c_int == 0i32 {
        (*h).gclist = (*g).grayagain;
        if (*h).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                425i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"int traverseephemeron(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        (*g).grayagain = &mut (*(h as *mut GCUnion)).gc
    } else if 0 != hasww {
        (*h).gclist = (*g).ephemeron;
        if (*h).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                427i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"int traverseephemeron(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        (*g).ephemeron = &mut (*(h as *mut GCUnion)).gc
    } else if 0 != hasclears {
        (*h).gclist = (*g).allweak;
        if (*h).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                429i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"int traverseephemeron(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        (*g).allweak = &mut (*(h as *mut GCUnion)).gc
    }
    /* may have to clean white keys */
    return marked;
}
/* }====================================================== */
/*
** {======================================================
** Traverse functions
** =======================================================
*/
/*
** Traverse a table with weak values and link it to proper list. During
** propagate phase, keep it in 'grayagain' list, to be revisited in the
** atomic phase. In the atomic phase, if table has any white value,
** put it in 'weak' list, to be cleared.
*/
unsafe extern "C" fn traverseweakvalue(mut g: *mut global_State, mut h: *mut Table_0) -> () {
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *(*h)
        .node
        .offset((1i32 << (*h).lsizenode as libc::c_int) as size_t as isize)
        as *mut Node;
    /* if there is array part, assume it may have white values (it is not
     worth traversing it now just to check) */
    let mut hasclears: libc::c_int = ((*h).sizearray > 0i32 as libc::c_uint) as libc::c_int;
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        /* traverse hash part */
        if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 9i32 + 1i32)
            || (*n).i_val.tt_ == 0i32
        {
        } else {
            __assert_fail(b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == ((9+1))) || (((((&(n)->i_val)))->tt_) == (0))\x00"
                              as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          368i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
        };
        /* entry is empty? */
        if (*n).i_val.tt_ == 0i32 {
            /* remove it */
            removeentry(n);
        } else {
            if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 0i32) {
            } else {
                __assert_fail(
                    b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == (0))\x00" as *const u8
                        as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    372i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                        b"void traverseweakvalue(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            if 0 == (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32 || {
                if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
                {
                } else {
                    __assert_fail(
                        b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                            as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        373i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                            b"void traverseweakvalue(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 0x3fi32
                    == (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                        .value_
                        .gc)
                        .tt as libc::c_int
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        373i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                            b"void traverseweakvalue(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32 && {
                if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
                {
                } else {
                    __assert_fail(
                        b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                            as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        373i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                            b"void traverseweakvalue(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                    .value_
                    .gc)
                    .marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
                {
                } else {
                    __assert_fail(
                        b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                            as *const u8 as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        373i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                            b"void traverseweakvalue(global_State *, Table *)\x00",
                        )).as_ptr(),
                    );
                };
                reallymarkobject(
                    g,
                    (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                        .value_
                        .gc,
                );
            }
            /* mark key */
            /* is there a white value? */
            if 0 == hasclears && 0 != iscleared(g, &mut (*n).i_val) {
                /* table will have to be cleared */
                hasclears = 1i32
            }
        }
        n = n.offset(1isize)
    }
    if (*g).gcstate as libc::c_int == 0i32 {
        (*h).gclist = (*g).grayagain;
        if (*h).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                379i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void traverseweakvalue(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        (*g).grayagain = &mut (*(h as *mut GCUnion)).gc
    } else if 0 != hasclears {
        (*h).gclist = (*g).weak;
        if (*h).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                381i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void traverseweakvalue(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        (*g).weak = &mut (*(h as *mut GCUnion)).gc
    };
}
/*
** mark all objects in list of being-finalized
*/
unsafe extern "C" fn markbeingfnz(mut g: *mut global_State) -> () {
    let mut o: *mut GCObject = 0 as *mut GCObject;
    o = (*g).tobefnz;
    while !o.is_null() {
        if 0 != (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
            if (*o).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((o)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    303i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                        b"void markbeingfnz(global_State *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).gc);
        }
        o = (*o).next
    }
}
/*
** Mark all values stored in marked open upvalues from non-marked threads.
** (Values from marked threads were already marked when traversing the
** thread.) Remove from the list threads that no longer have upvalues and
** not-marked threads.
*/
unsafe extern "C" fn remarkupvals(mut g: *mut global_State) -> () {
    let mut thread: *mut lua_State_0 = 0 as *mut lua_State_0;
    let mut p: *mut *mut lua_State_0 = &mut (*g).twups;
    loop {
        thread = *p;
        if thread.is_null() {
            break;
        }
        if 0 == (*thread).marked as libc::c_int & 1i32 << 2i32 {
        } else {
            __assert_fail(
                b"!(((thread)->marked) & ((1<<(2))))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                317i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void remarkupvals(global_State *)\x00",
                )).as_ptr(),
            );
        };
        /* threads are never black */
        if 0 == (*thread).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32)
            && !(*thread).openupval.is_null()
        {
            /* keep marked thread with upvalues in the list */
            p = &mut (*thread).twups
        } else {
            /* thread is not marked or without upvalues */
            let mut uv: *mut UpVal = 0 as *mut UpVal;
            /* remove thread from the list */
            *p = (*thread).twups;
            /* mark that it is out of list */
            (*thread).twups = thread;
            uv = (*thread).openupval;
            while !uv.is_null() {
                if 0 != (*uv).u.open.touched {
                    if 0 == (*(*uv).v).tt_ & 1i32 << 6i32 || {
                        if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((uv->v)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                                326i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                                    b"void remarkupvals(global_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        (*(*uv).v).tt_ & 0x3fi32 == (*(*(*uv).v).value_.gc).tt as libc::c_int
                    } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                                326i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                                    b"void remarkupvals(global_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 && {
                        if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((uv->v)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                                326i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                                    b"void remarkupvals(global_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        0 != (*(*(*uv).v).value_.gc).marked as libc::c_int
                            & (1i32 << 0i32 | 1i32 << 1i32)
                    } {
                        if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((uv->v)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                                326i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                                    b"void remarkupvals(global_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        reallymarkobject(g, (*(*uv).v).value_.gc);
                    }
                    /* remark upvalue's value */
                    (*uv).u.open.touched = 0i32
                }
                uv = (*uv).u.open.next
            }
        }
    }
}
/*
** mark metamethods for basic types
*/
unsafe extern "C" fn markmt(mut g: *mut global_State) -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 9i32 {
        if !(*g).mt[i as usize].is_null() {
            if 0 != (*(*g).mt[i as usize]).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
                if (*(*g).mt[i as usize]).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(
                        b"(((g->mt[i])->tt) & 0x0F) < (9+1)\x00" as *const u8
                            as *const libc::c_char,
                        b"lgc.c\x00" as *const u8 as *const libc::c_char,
                        293i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                            b"void markmt(global_State *)\x00",
                        )).as_ptr(),
                    );
                };
                reallymarkobject(g, &mut (*((*g).mt[i as usize] as *mut GCUnion)).gc);
            }
        }
        i += 1
    }
}
/*
** mark root set and reset all gray lists, to start a new collection
*/
unsafe extern "C" fn restartcollection(mut g: *mut global_State) -> () {
    (*g).grayagain = 0 as *mut GCObject;
    (*g).gray = (*g).grayagain;
    (*g).ephemeron = 0 as *mut GCObject;
    (*g).allweak = (*g).ephemeron;
    (*g).weak = (*g).allweak;
    if 0 != (*(*g).mainthread).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
        if (*(*g).mainthread).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((g->mainthread)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                341i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void restartcollection(global_State *)\x00",
                )).as_ptr(),
            );
        };
        reallymarkobject(g, &mut (*((*g).mainthread as *mut GCUnion)).gc);
    }
    if 0 == (*g).l_registry.tt_ & 1i32 << 6i32 || {
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                342i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void restartcollection(global_State *)\x00",
                )).as_ptr(),
            );
        };
        (*g).l_registry.tt_ & 0x3fi32 == (*(*g).l_registry.value_.gc).tt as libc::c_int
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                342i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void restartcollection(global_State *)\x00",
                )).as_ptr(),
            );
        };
    };
    if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 && {
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                342i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void restartcollection(global_State *)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*g).l_registry.value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
    } {
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                342i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void restartcollection(global_State *)\x00",
                )).as_ptr(),
            );
        };
        reallymarkobject(g, (*g).l_registry.value_.gc);
    }
    markmt(g);
    /* mark any finalizing object left from previous cycle */
    markbeingfnz(g);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_runtilstate(
    mut L: *mut lua_State_0,
    mut statesmask: libc::c_int,
) -> () {
    let mut g: *mut global_State = (*L).l_G;
    while 0 == statesmask & 1i32 << (*g).gcstate as libc::c_int {
        singlestep(L);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaC_fullgc(mut L: *mut lua_State_0, mut isemergency: libc::c_int) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if (*g).gckind as libc::c_int == 0i32 {
    } else {
        __assert_fail(
            b"g->gckind == 0\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            1161i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"void luaC_fullgc(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if 0 != isemergency {
        /* set flag */
        (*g).gckind = 1i32 as lu_byte
    }
    if (*g).gcstate as libc::c_int <= 1i32 {
        /* black objects? */
        /* sweep everything to turn them back to white */
        entersweep(L);
    }
    /* finish any pending sweep phase to start a new cycle */
    luaC_runtilstate(L, 1i32 << 7i32);
    /* start new collection */
    luaC_runtilstate(L, !(1i32 << 7i32));
    /* run up to finalizers */
    luaC_runtilstate(L, 1i32 << 6i32);
    if (*g).GCestimate == ((*g).totalbytes + (*g).GCdebt) as lu_mem {
    } else {
        __assert_fail(
            b"g->GCestimate == ((lu_mem)((g)->totalbytes + (g)->GCdebt))\x00" as *const u8
                as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            1171i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"void luaC_fullgc(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    /* estimate must be correct after a full GC cycle */
    /* finish collection */
    luaC_runtilstate(L, 1i32 << 7i32);
    (*g).gckind = 0i32 as lu_byte;
    setpause(g);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_newobj(
    mut L: *mut lua_State_0,
    mut tt: libc::c_int,
    mut sz: size_t,
) -> *mut GCObject {
    let mut g: *mut global_State = (*L).l_G;
    let mut o: *mut GCObject =
        luaM_realloc_(L, 0 as *mut libc::c_void, (tt & 0xfi32) as size_t, sz) as *mut GCObject;
    (*o).marked = ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte;
    (*o).tt = tt as lu_byte;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_barrier_(
    mut L: *mut lua_State_0,
    mut o: *mut GCObject,
    mut v: *mut GCObject,
) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if 0 != (*o).marked as libc::c_int & 1i32 << 2i32
        && 0 != (*v).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
        && 0 != ((*v).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        && 0 != ((*o).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
    {
    } else {
        __assert_fail(b"(((o)->marked) & ((1<<(2)))) && (((v)->marked) & (((1<<(0)) | (1<<(1))))) && !(!((((v)->marked) ^ ((1<<(0)) | (1<<(1)))) & (((g)->currentwhite ^ ((1<<(0)) | (1<<(1))))))) && !(!((((o)->marked) ^ ((1<<(0)) | (1<<(1)))) & (((g)->currentwhite ^ ((1<<(0)) | (1<<(1)))))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      158i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"void luaC_barrier_(lua_State *, GCObject *, GCObject *)\x00")).as_ptr());
    };
    /* must keep invariant? */
    if (*g).gcstate as libc::c_int <= 1i32 {
        /* restore invariant */
        reallymarkobject(g, v);
    } else {
        /* sweep phase */
        if 2i32 <= (*g).gcstate as libc::c_int && (*g).gcstate as libc::c_int <= 5i32 {
        } else {
            __assert_fail(
                b"(2 <= (g)->gcstate && (g)->gcstate <= 5)\x00" as *const u8 as *const libc::c_char,
                b"lgc.c\x00" as *const u8 as *const libc::c_char,
                162i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"void luaC_barrier_(lua_State *, GCObject *, GCObject *)\x00",
                )).as_ptr(),
            );
        };
        (*o).marked = ((*o).marked as libc::c_int & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32))
            | ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
                as libc::c_int) as lu_byte
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_barrierback_(mut L: *mut lua_State_0, mut t: *mut Table_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if 0 != (*t).marked as libc::c_int & 1i32 << 2i32
        && 0 != ((*t).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
    {
    } else {
        __assert_fail(b"(((t)->marked) & ((1<<(2)))) && !(!((((t)->marked) ^ ((1<<(0)) | (1<<(1)))) & (((g)->currentwhite ^ ((1<<(0)) | (1<<(1)))))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      174i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void luaC_barrierback_(lua_State *, Table *)\x00")).as_ptr());
    };
    (*t).marked =
        ((*t).marked as libc::c_int & !(1i32 << 2i32) as lu_byte as libc::c_int) as lu_byte;
    (*t).gclist = (*g).grayagain;
    if (*t).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((t)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            176i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"void luaC_barrierback_(lua_State *, Table *)\x00",
            )).as_ptr(),
        );
    };
    (*g).grayagain = &mut (*(t as *mut GCUnion)).gc;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_upvalbarrier_(mut L: *mut lua_State_0, mut uv: *mut UpVal) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((uv->v)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            188i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void luaC_upvalbarrier_(lua_State *, UpVal *)\x00",
            )).as_ptr(),
        );
    };
    let mut o: *mut GCObject = (*(*uv).v).value_.gc;
    if !((*uv).v != &mut (*uv).u.value as *mut TValue) {
    } else {
        __assert_fail(
            b"!((uv)->v != &(uv)->u.value)\x00" as *const u8 as *const libc::c_char,
            b"lgc.c\x00" as *const u8 as *const libc::c_char,
            189i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void luaC_upvalbarrier_(lua_State *, UpVal *)\x00",
            )).as_ptr(),
        );
    };
    /* ensured by macro luaC_upvalbarrier */
    if (*g).gcstate as libc::c_int <= 1i32 {
        if 0 != (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
            if (*o).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((o)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"lgc.c\x00" as *const u8 as *const libc::c_char,
                    191i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                        b"void luaC_upvalbarrier_(lua_State *, UpVal *)\x00",
                    )).as_ptr(),
                );
            };
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).gc);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_checkfinalizer(
    mut L: *mut lua_State_0,
    mut o: *mut GCObject,
    mut mt: *mut Table_0,
) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* obj. is already marked... */
    if 0 != (*o).marked as libc::c_int & 1i32 << 3i32 || if mt.is_null() {
        0 as *const TValue
    } else if 0 != (*mt).flags as libc::c_uint & 1u32 << TM_GC as libc::c_int {
        0 as *const TValue
    } else {
        luaT_gettm(mt, TM_GC, (*g).tmname[TM_GC as libc::c_int as usize])
    }.is_null()
    {
        /* or has no finalizer? */
        /* nothing to be done */
        return;
    } else {
        /* move 'o' to 'finobj' list */
        let mut p: *mut *mut GCObject = 0 as *mut *mut GCObject;
        if 2i32 <= (*g).gcstate as libc::c_int && (*g).gcstate as libc::c_int <= 5i32 {
            (*o).marked = ((*o).marked as libc::c_int
                & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32))
                | ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
                    as libc::c_int) as lu_byte;
            /* "sweep" object 'o' */
            /* should not remove 'sweepgc' object */
            if (*g).sweepgc == &mut (*o).next as *mut *mut GCObject {
                /* change 'sweepgc' */
                (*g).sweepgc = sweeptolive(L, (*g).sweepgc)
            }
        }
        /* search for pointer pointing to 'o' */
        p = &mut (*g).allgc;
        while *p != o {
            /* empty */
            p = &mut (**p).next
        }
        /* remove 'o' from 'allgc' list */
        *p = (*o).next;
        /* link it in 'finobj' list */
        (*o).next = (*g).finobj;
        (*g).finobj = o;
        (*o).marked = ((*o).marked as libc::c_int | 1i32 << 3i32) as lu_byte;
        return;
    };
}
/*
** sweep a list until a live object (or end of list)
*/
unsafe extern "C" fn sweeptolive(
    mut L: *mut lua_State_0,
    mut p: *mut *mut GCObject,
) -> *mut *mut GCObject {
    let mut old: *mut *mut GCObject = p;
    loop {
        p = sweeplist(L, p, 1i32 as lu_mem);
        if !(p == old) {
            break;
        }
    }
    return p;
}
