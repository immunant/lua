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
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaC_barrier_(L: *mut lua_State_0, o: *mut GCObject, v: *mut GCObject) -> ();
    #[no_mangle]
    fn luaD_growstack(L: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_rawrunprotected(L: *mut lua_State_0, f: Pfunc, ud: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaC_step(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaO_tostring(L: *mut lua_State_0, obj: StkId) -> ();
    #[no_mangle]
    fn luaH_getn(t: *mut Table_0) -> libc::c_int;
    #[no_mangle]
    fn luaO_arith(
        L: *mut lua_State_0,
        op: libc::c_int,
        p1: *const TValue,
        p2: *const TValue,
        res: *mut TValue,
    ) -> ();
    #[no_mangle]
    fn luaV_equalobj(L: *mut lua_State_0, t1: *const TValue, t2: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaV_lessequal(L: *mut lua_State_0, l: *const TValue, r: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaV_lessthan(L: *mut lua_State_0, l: *const TValue, r: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State_0, str: *const libc::c_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State_0, str: *const libc::c_char) -> *mut TString;
    #[no_mangle]
    fn luaO_pushvfstring(
        L: *mut lua_State_0,
        fmt: *const libc::c_char,
        argp: *mut __va_list_tag,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaF_newCclosure(L: *mut lua_State_0, nelems: libc::c_int) -> *mut CClosure;
    #[no_mangle]
    fn luaH_getint(t: *mut Table_0, key: lua_Integer) -> *const TValue;
    #[no_mangle]
    fn luaV_finishget(
        L: *mut lua_State_0,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    ) -> ();
    #[no_mangle]
    fn luaH_getstr(t: *mut Table_0, key: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaH_get(t: *mut Table_0, key: *const TValue) -> *const TValue;
    #[no_mangle]
    fn luaH_resize(
        L: *mut lua_State_0,
        t: *mut Table_0,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    ) -> ();
    #[no_mangle]
    fn luaH_new(L: *mut lua_State_0) -> *mut Table_0;
    #[no_mangle]
    fn luaS_newudata(L: *mut lua_State_0, s: size_t) -> *mut Udata_0;
    #[no_mangle]
    fn luaV_finishset(
        L: *mut lua_State_0,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    ) -> ();
    #[no_mangle]
    fn luaC_barrierback_(L: *mut lua_State_0, o: *mut Table_0) -> ();
    #[no_mangle]
    fn luaH_set(L: *mut lua_State_0, t: *mut Table_0, key: *const TValue) -> *mut TValue;
    #[no_mangle]
    fn luaH_setint(
        L: *mut lua_State_0,
        t: *mut Table_0,
        key: lua_Integer,
        value: *mut TValue,
    ) -> ();
    #[no_mangle]
    fn luaC_checkfinalizer(L: *mut lua_State_0, o: *mut GCObject, mt: *mut Table_0) -> ();
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State_0, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_call(L: *mut lua_State_0, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_pcall(
        L: *mut lua_State_0,
        func: Pfunc,
        u: *mut libc::c_void,
        oldtop: ptrdiff_t,
        ef: ptrdiff_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaC_upvalbarrier_(L: *mut lua_State_0, uv: *mut UpVal) -> ();
    #[no_mangle]
    fn luaD_protectedparser(
        L: *mut lua_State_0,
        z: *mut ZIO,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaZ_init(
        L: *mut lua_State_0,
        z: *mut ZIO,
        reader: lua_Reader,
        data: *mut libc::c_void,
    ) -> ();
    #[no_mangle]
    fn luaU_dump(
        L: *mut lua_State_0,
        f: *const Proto_0,
        w: lua_Writer,
        data: *mut libc::c_void,
        strip: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaE_setdebt(g: *mut global_State, debt: l_mem) -> ();
    #[no_mangle]
    fn luaC_fullgc(L: *mut lua_State_0, isemergency: libc::c_int) -> ();
    #[no_mangle]
    fn luaG_errormsg(L: *mut lua_State_0) -> !;
    #[no_mangle]
    fn luaH_next(L: *mut lua_State_0, t: *mut Table_0, key: StkId) -> libc::c_int;
    #[no_mangle]
    fn luaV_concat(L: *mut lua_State_0, total: libc::c_int) -> ();
    #[no_mangle]
    fn luaV_objlen(L: *mut lua_State_0, ra: StkId, rb: *const TValue) -> ();
    #[no_mangle]
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> size_t;
    #[no_mangle]
    fn luaC_upvdeccount(L: *mut lua_State_0, uv: *mut UpVal) -> ();
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
** garbage-collection function and options
*/
/*
** miscellaneous functions
*/
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
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
pub type lua_Reader = Option<
    unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void, _: *mut size_t)
        -> *const libc::c_char,
>;
pub type lua_Writer = Option<
    unsafe extern "C" fn(
        _: *mut lua_State_0,
        _: *const libc::c_void,
        _: size_t,
        _: *mut libc::c_void,
    ) -> libc::c_int,
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
/* test for lock/unlock */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct L_EXTRA {
    pub lock: libc::c_int,
    pub plock: *mut libc::c_int,
}
/*
** Closures
*/
pub type CClosure = CClosure_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    c: CClosure,
    l: LClosure,
}
pub type LClosure = LClosure_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 0],
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
    pub locvars: *mut LocVar,
    pub upvalues: *mut Upvaldesc,
    pub cache: *mut LClosure_0,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
pub type Upvaldesc = Upvaldesc_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc_0 {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
pub type LocVar = LocVar_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar_0 {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
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
/*
** Ensures that address after this type is always fully aligned.
*/
pub type UTString = UTString_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString_0 {
    dummy: L_Umaxalign,
    tsv: TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
pub type Table_0 = Table;
/*
** Ensures that address after this type is always fully aligned.
*/
pub type UUdata = UUdata_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UUdata_0 {
    dummy: L_Umaxalign,
    uv: Udata_0,
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
/*
** Execute a protected call.
*/
/* data to 'f_call' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallS {
    pub func: StkId,
    pub nresults: libc::c_int,
}
/*
** $Id: lzio.h,v 1.30 2014/12/19 17:26:14 roberto Exp roberto $
** Buffered streams
** See Copyright Notice in lua.h
*/
/* end of stream */
pub type ZIO = Zio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State_0,
}
/*
** Function Prototypes
*/
pub type Proto_0 = Proto;
/*
** generic extra include file
*/
/*
** RCS ident string
*/
#[no_mangle]
pub static mut lua_ident: [libc::c_char; 129] = unsafe {
    [
        36, 76, 117, 97, 86, 101, 114, 115, 105, 111, 110, 58, 32, 76, 117, 97, 32, 53, 46, 51, 46,
        52, 32, 32, 67, 111, 112, 121, 114, 105, 103, 104, 116, 32, 40, 67, 41, 32, 49, 57, 57, 52,
        45, 50, 48, 49, 55, 32, 76, 117, 97, 46, 111, 114, 103, 44, 32, 80, 85, 67, 45, 82, 105,
        111, 32, 36, 36, 76, 117, 97, 65, 117, 116, 104, 111, 114, 115, 58, 32, 82, 46, 32, 73,
        101, 114, 117, 115, 97, 108, 105, 109, 115, 99, 104, 121, 44, 32, 76, 46, 32, 72, 46, 32,
        100, 101, 32, 70, 105, 103, 117, 101, 105, 114, 101, 100, 111, 44, 32, 87, 46, 32, 67, 101,
        108, 101, 115, 32, 36, 0,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn lua_atpanic(
    mut L: *mut lua_State_0,
    mut panicf: lua_CFunction,
) -> lua_CFunction {
    let mut old: lua_CFunction = None;
    let ref mut fresh0 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh1 = *fresh0;
    *fresh0 = *fresh0 + 1;
    if fresh1 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      137i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"lua_CFunction lua_atpanic(lua_State *, lua_CFunction)\x00")).as_ptr());
    };
    old = (*(*L).l_G).panic;
    (*(*L).l_G).panic = panicf;
    let ref mut fresh2 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh2 -= 1;
    if *fresh2 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      140i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"lua_CFunction lua_atpanic(lua_State *, lua_CFunction)\x00")).as_ptr());
    };
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn lua_version(mut L: *mut lua_State_0) -> *const lua_Number {
    static mut version: lua_Number = unsafe { 503i32 as lua_Number };
    if L.is_null() {
        return &version;
    } else {
        return (*(*L).l_G).version;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_absindex(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    return if idx > 0i32 || idx <= -50000i32 - 1000i32 {
        idx
    } else {
        (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long as libc::c_int + idx
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettop(mut L: *mut lua_State_0) -> libc::c_int {
    return (*L)
        .top
        .wrapping_offset_from((*(*L).ci).func.offset(1isize)) as libc::c_long
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_settop(mut L: *mut lua_State_0, mut idx: libc::c_int) -> () {
    let mut func: StkId = (*(*L).ci).func;
    let ref mut fresh3 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh4 = *fresh3;
    *fresh3 = *fresh3 + 1;
    if fresh4 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      175i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void lua_settop(lua_State *, int)\x00")).as_ptr());
    };
    if idx >= 0i32 {
        if idx as libc::c_long
            <= (*L).stack_last.wrapping_offset_from(func.offset(1isize)) as libc::c_long
            && !(b"new top too large\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(idx <= L->stack_last - (func + 1)) && \"new top too large\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                177i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_settop(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        while (*L).top < func.offset(1isize).offset(idx as isize) {
            let fresh5 = (*L).top;
            (*L).top = (*L).top.offset(1);
            (*fresh5).tt_ = 0i32
        }
        (*L).top = func.offset(1isize).offset(idx as isize)
    } else {
        if -(idx + 1i32) as libc::c_long
            <= (*L).top.wrapping_offset_from(func.offset(1isize)) as libc::c_long
            && !(b"invalid new top\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(-(idx+1) <= (L->top - (func + 1))) && \"invalid new top\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                183i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_settop(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        /* 'subtract' index (index is negative) */
        (*L).top = (*L).top.offset((idx + 1i32) as isize)
    }
    let ref mut fresh6 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh6 -= 1;
    if *fresh6 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      186i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void lua_settop(lua_State *, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvalue(mut L: *mut lua_State_0, mut idx: libc::c_int) -> () {
    let ref mut fresh7 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh8 = *fresh7;
    *fresh7 = *fresh7 + 1;
    if fresh8 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      239i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"void lua_pushvalue(lua_State *, int)\x00")).as_ptr());
    };
    let mut io1: *mut TValue = (*L).top;
    *io1 = *index2addr(L, idx);
    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                240i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"void lua_pushvalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    240i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_pushvalue(lua_State *, int)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                240i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"void lua_pushvalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            241i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"void lua_pushvalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh9 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh9 -= 1;
    if *fresh9 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      242i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"void lua_pushvalue(lua_State *, int)\x00")).as_ptr());
    };
}
/*
** $Id: lapi.c,v 2.258 2016/01/05 16:07:21 roberto Exp roberto $
** Lua API
** See Copyright Notice in lua.h
*/
/* value at a non-valid index */
/* corresponding test */
/* test for pseudo index */
/* test for upvalue */
/* test for valid but not pseudo index */
unsafe extern "C" fn index2addr(mut L: *mut lua_State_0, mut idx: libc::c_int) -> *mut TValue {
    let mut ci: *mut CallInfo_0 = (*L).ci;
    if idx > 0i32 {
        let mut o: *mut TValue = (*ci).func.offset(idx as isize);
        if idx as libc::c_long
            <= (*ci).top.wrapping_offset_from((*ci).func.offset(1isize)) as libc::c_long
            && !(b"unacceptable index\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(idx <= ci->top - (ci->func + 1)) && \"unacceptable index\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                65i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"TValue *index2addr(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if o >= (*L).top {
            return &luaO_nilobject_ as *const TValue as *mut TValue;
        } else {
            return o;
        }
    } else if !(idx <= -50000i32 - 1000i32) {
        /* negative index */
        if idx != 0i32
            && -idx as libc::c_long
                <= (*L).top.wrapping_offset_from((*ci).func.offset(1isize)) as libc::c_long
            && !(b"invalid index\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(idx != 0 && -idx <= L->top - (ci->func + 1)) && \"invalid index\"\x00"
                    as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                70i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"TValue *index2addr(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        return (*L).top.offset(idx as isize);
    } else if idx == -50000i32 - 1000i32 {
        return &mut (*(*L).l_G).l_registry;
    } else {
        /* upvalues */
        idx = -50000i32 - 1000i32 - idx;
        if idx <= 255i32 + 1i32
            && !(b"upvalue index too large\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(idx <= 255 + 1) && \"upvalue index too large\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                77i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"TValue *index2addr(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        /* light C function? */
        if (*(*ci).func).tt_ == 6i32 | 1i32 << 4i32 {
            /* it has no upvalues */
            return &luaO_nilobject_ as *const TValue as *mut TValue;
        } else {
            if (*(*ci).func).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((ci->func))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    81i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"TValue *index2addr(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(*ci).func).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(((ci->func)->value_).gc)->tt == (6 | (2 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    81i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"TValue *index2addr(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            let mut func: *mut CClosure = &mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.c;
            return if idx <= (*func).nupvalues as libc::c_int {
                &mut *(*func).upvalue.as_mut_ptr().offset((idx - 1i32) as isize) as *mut TValue
            } else {
                &luaO_nilobject_ as *const TValue as *mut TValue
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rotate(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut n: libc::c_int,
) -> () {
    let mut p: StkId = 0 as *mut TValue;
    let mut t: StkId = 0 as *mut TValue;
    let mut m: StkId = 0 as *mut TValue;
    let ref mut fresh10 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh11 = *fresh10;
    *fresh10 = *fresh10 + 1;
    if fresh11 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      210i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"void lua_rotate(lua_State *, int, int)\x00")).as_ptr());
    };
    /* end of stack segment being rotated */
    t = (*L).top.offset(-1isize);
    /* start of segment */
    p = index2addr(L, idx);
    if p != &luaO_nilobject_ as *const TValue as StkId
        && !(idx <= -50000i32 - 1000i32)
        && !(b"index not in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(b"((((p) != (&luaO_nilobject_)) && !((idx) <= (-50000 - 1000)))) && \"index not in the stack\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      213i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"void lua_rotate(lua_State *, int, int)\x00")).as_ptr());
    };
    if (if n >= 0i32 { n } else { -n }) as libc::c_long
        <= t.wrapping_offset_from(p) as libc::c_long + 1i32 as libc::c_long
        && !(b"invalid \'n\'\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((n >= 0 ? n : -n) <= (t - p + 1)) && \"invalid \'n\'\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            214i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void lua_rotate(lua_State *, int, int)\x00",
            )).as_ptr(),
        );
    };
    /* end of prefix */
    m = if n >= 0i32 {
        t.offset(-(n as isize))
    } else {
        p.offset(-(n as isize)).offset(-1isize)
    };
    /* reverse the prefix with length 'n' */
    reverse(L, p, m);
    /* reverse the suffix */
    reverse(L, m.offset(1isize), t);
    /* reverse the entire segment */
    reverse(L, p, t);
    let ref mut fresh12 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh12 -= 1;
    if *fresh12 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      219i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"void lua_rotate(lua_State *, int, int)\x00")).as_ptr());
    };
}
/*
** Reverse the stack segment from 'from' to 'to'
** (auxiliary to 'lua_rotate')
*/
unsafe extern "C" fn reverse(mut L: *mut lua_State_0, mut from: StkId, mut to: StkId) -> () {
    while from < to {
        let mut temp: TValue = lua_TValue {
            value_: Value_0 {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io1: *mut TValue = &mut temp;
        *io1 = *from;
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    197i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void reverse(lua_State *, StkId, StkId)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        197i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                            b"void reverse(lua_State *, StkId, StkId)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    197i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void reverse(lua_State *, StkId, StkId)\x00",
                    )).as_ptr(),
                );
            };
        };
        let mut io1_0: *mut TValue = from;
        *io1_0 = *to;
        if 0 == (*io1_0).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    198i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void reverse(lua_State *, StkId, StkId)\x00",
                    )).as_ptr(),
                );
            };
            (*io1_0).tt_ & 0x3fi32 == (*(*io1_0).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        198i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                            b"void reverse(lua_State *, StkId, StkId)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    198i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void reverse(lua_State *, StkId, StkId)\x00",
                    )).as_ptr(),
                );
            };
        };
        let mut io1_1: *mut TValue = to;
        *io1_1 = temp;
        if 0 == (*io1_1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    199i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void reverse(lua_State *, StkId, StkId)\x00",
                    )).as_ptr(),
                );
            };
            (*io1_1).tt_ & 0x3fi32 == (*(*io1_1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        199i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                            b"void reverse(lua_State *, StkId, StkId)\x00",
                        )).as_ptr(),
                    );
                };
                0 != ((*(*io1_1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    199i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void reverse(lua_State *, StkId, StkId)\x00",
                    )).as_ptr(),
                );
            };
        };
        from = from.offset(1isize);
        to = to.offset(-1isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_copy(
    mut L: *mut lua_State_0,
    mut fromidx: libc::c_int,
    mut toidx: libc::c_int,
) -> () {
    let mut fr: *mut TValue = 0 as *mut TValue;
    let mut to: *mut TValue = 0 as *mut TValue;
    let ref mut fresh13 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh14 = *fresh13;
    *fresh13 = *fresh13 + 1;
    if fresh14 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      225i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"void lua_copy(lua_State *, int, int)\x00")).as_ptr());
    };
    fr = index2addr(L, fromidx);
    to = index2addr(L, toidx);
    if to != &luaO_nilobject_ as *const TValue as *mut TValue
        && !(b"invalid index\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((to) != (&luaO_nilobject_))) && \"invalid index\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            228i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"void lua_copy(lua_State *, int, int)\x00",
            )).as_ptr(),
        );
    };
    let mut io1: *mut TValue = to;
    *io1 = *fr;
    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                229i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"void lua_copy(lua_State *, int, int)\x00",
                )).as_ptr(),
            );
        };
        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    229i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                229i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"void lua_copy(lua_State *, int, int)\x00",
                )).as_ptr(),
            );
        };
    };
    /* function upvalue? */
    if toidx < -50000i32 - 1000i32 {
        if 0 != (*fr).tt_ & 1i32 << 6i32 && {
            if (*(*(*L).ci).func).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((L->ci->func))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(*(*L).ci).func).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(((L->ci->func)->value_).gc)->tt == (6 | (2 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            0 != (*(&mut (*((*(*(*L).ci).func).value_.gc as *mut GCUnion)).cl.c as *mut CClosure))
                .marked as libc::c_int & 1i32 << 2i32
        } && {
            if 0 != (*fr).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((fr)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            0 != (*(*fr).value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
        } {
            if (*(*(*L).ci).func).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((L->ci->func))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(*(*L).ci).func).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(((L->ci->func)->value_).gc)->tt == (6 | (2 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(&mut (*((*(*(*L).ci).func).value_.gc as *mut GCUnion)).cl.c as *mut CClosure)).tt
                as libc::c_int & 0xfi32 < 9i32 + 1i32
            {
            } else {
                __assert_fail(b"((((((((((L->ci->func))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))) ? (void) (0) : __assert_fail (\"((((L->ci->func))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\", \"lapi.c\", 231, __extension__ __PRETTY_FUNCTION__)), (((((((L->ci->func)->value_).gc)->tt == (6 | (2 << 4))) ? (void) (0) : __assert_fail (\"(((L->ci->func)->value_).gc)->tt == (6 | (2 << 4))\", \"lapi.c\", 231, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((L->ci->func)->value_).gc))))->cl.c))))))->tt) & 0x0F) < (9+1)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char, 231i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 37],
                                                        &[libc::c_char; 37]>(b"void lua_copy(lua_State *, int, int)\x00")).as_ptr());
            };
            if (*(*(*L).ci).func).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((L->ci->func))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(*(*L).ci).func).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(((L->ci->func)->value_).gc)->tt == (6 | (2 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            if 0 != (*fr).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((fr)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void lua_copy(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            luaC_barrier_(
                L,
                &mut (*(&mut (*((*(*(*L).ci).func).value_.gc as *mut GCUnion)).cl.c as *mut CClosure
                    as *mut GCUnion))
                    .gc,
                (*fr).value_.gc,
            );
        } else {
        };
    }
    let ref mut fresh15 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh15 -= 1;
    if *fresh15 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      234i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"void lua_copy(lua_State *, int, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_checkstack(
    mut L: *mut lua_State_0,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let ref mut fresh16 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh17 = *fresh16;
    *fresh16 = *fresh16 + 1;
    if fresh17 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      101i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"int lua_checkstack(lua_State *, int)\x00")).as_ptr());
    };
    if n >= 0i32 && !(b"negative \'n\'\x00" as *const u8 as *const libc::c_char).is_null() {
    } else {
        __assert_fail(
            b"(n >= 0) && \"negative \'n\'\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            102i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int lua_checkstack(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    /* stack large enough? */
    if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long > n as libc::c_long {
        /* yes; check is OK */
        res = 1i32
    } else {
        /* no; need to grow stack */
        let mut inuse: libc::c_int =
            (*L).top.wrapping_offset_from((*L).stack) as libc::c_long as libc::c_int + 5i32;
        /* can grow without overflow? */
        if inuse > 50000i32 - n {
            /* no */
            res = 0i32
        } else {
            res = (luaD_rawrunprotected(
                L,
                Some(growstack),
                &mut n as *mut libc::c_int as *mut libc::c_void,
            ) == 0i32) as libc::c_int
        }
    }
    if 0 != res && (*ci).top < (*L).top.offset(n as isize) {
        /* adjust frame top */
        (*ci).top = (*L).top.offset(n as isize)
    }
    let ref mut fresh18 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh18 -= 1;
    if *fresh18 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      114i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"int lua_checkstack(lua_State *, int)\x00")).as_ptr());
    };
    return res;
}
/*
** to be called by 'lua_checkstack' in protected mode, to grow stack
** capturing memory errors
*/
unsafe extern "C" fn growstack(mut L: *mut lua_State_0, mut ud: *mut libc::c_void) -> () {
    let mut size: libc::c_int = *(ud as *mut libc::c_int);
    luaD_growstack(L, size);
}
#[no_mangle]
pub unsafe extern "C" fn lua_xmove(
    mut from: *mut lua_State_0,
    mut to: *mut lua_State_0,
    mut n: libc::c_int,
) -> () {
    let mut i: libc::c_int = 0;
    if from == to {
        return;
    } else {
        let ref mut fresh19 = *(*((to as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void as *mut L_EXTRA))
            .plock;
        let fresh20 = *fresh19;
        *fresh19 = *fresh19 + 1;
        if fresh20 == 0i32 {
        } else {
            __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(to) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"lapi.c\x00" as *const u8 as *const libc::c_char,
                          122i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"void lua_xmove(lua_State *, lua_State *, int)\x00")).as_ptr());
        };
        if (n as libc::c_long)
            < (*from).top.wrapping_offset_from((*(*from).ci).func) as libc::c_long
            && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"((n) < (from->top - from->ci->func)) && \"not enough elements in the stack\"\x00"
                    as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                123i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"void lua_xmove(lua_State *, lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*from).l_G == (*to).l_G
            && !(b"moving among independent states\x00" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"((from->l_G) == (to->l_G)) && \"moving among independent states\"\x00"
                    as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                124i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"void lua_xmove(lua_State *, lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*to).ci).top.wrapping_offset_from((*to).top) as libc::c_long >= n as libc::c_long
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(to->ci->top - to->top >= n) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                125i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"void lua_xmove(lua_State *, lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        (*from).top = (*from).top.offset(-(n as isize));
        i = 0i32;
        while i < n {
            let mut io1: *mut TValue = (*to).top;
            *io1 = *(*from).top.offset(i as isize);
            if 0 == (*io1).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        128i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                            b"void lua_xmove(lua_State *, lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (to.is_null() || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            128i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                                b"void lua_xmove(lua_State *, lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    0 != ((*(*io1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                        & ((*(*to).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        128i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                            b"void lua_xmove(lua_State *, lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
            };
            /* stack already checked by previous 'api_check' */
            (*to).top = (*to).top.offset(1isize);
            i += 1
        }
        let ref mut fresh21 = *(*((to as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void as *mut L_EXTRA))
            .plock;
        *fresh21 -= 1;
        if *fresh21 == 0i32 {
        } else {
            __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(to) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"lapi.c\x00" as *const u8 as *const libc::c_char,
                          131i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"void lua_xmove(lua_State *, lua_State *, int)\x00")).as_ptr());
        };
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_isnumber(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    return if (*o).tt_ == 3i32 | 0i32 << 4i32 {
        if (*o).tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                280i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"int lua_isnumber(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        n = (*o).value_.n;
        1i32
    } else {
        luaV_tonumber_(o, &mut n)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_isstring(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ & 0xfi32 == 4i32 || (*o).tt_ & 0xfi32 == 3i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_iscfunction(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return ((*o).tt_ == 6i32 | 1i32 << 4i32 || (*o).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isinteger(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return ((*o).tt_ == 3i32 | 1i32 << 4i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isuserdata(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ == 7i32 | 1i32 << 6i32 || (*o).tt_ == 2i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_type(mut L: *mut lua_State_0, mut idx: libc::c_int) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return if o != &luaO_nilobject_ as *const TValue as StkId {
        (*o).tt_ & 0xfi32
    } else {
        -1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_typename(
    mut L: *mut lua_State_0,
    mut t: libc::c_int,
) -> *const libc::c_char {
    L = 0 as *mut lua_State_0;
    if -1i32 <= t && t < 9i32 && !(b"invalid tag\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((-1) <= t && t < 9) && \"invalid tag\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            260i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"const char *lua_typename(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    return luaT_typenames_[(t + 1i32) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn lua_tonumberx(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut pisnum: *mut libc::c_int,
) -> lua_Number {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    let mut isnum: libc::c_int = if (*o).tt_ == 3i32 | 0i32 << 4i32 {
        if (*o).tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                349i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"lua_Number lua_tonumberx(lua_State *, int, int *)\x00",
                )).as_ptr(),
            );
        };
        n = (*o).value_.n;
        1i32
    } else {
        luaV_tonumber_(o, &mut n)
    };
    if 0 == isnum {
        /* call to 'tonumber' may change 'n' even if it fails */
        n = 0i32 as lua_Number
    }
    if !pisnum.is_null() {
        *pisnum = isnum
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tointegerx(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut pisnum: *mut libc::c_int,
) -> lua_Integer {
    let mut res: lua_Integer = 0;
    let mut o: *const TValue = index2addr(L, idx);
    let mut isnum: libc::c_int = if (*o).tt_ == 3i32 | 1i32 << 4i32 {
        if (*o).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                360i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                    b"lua_Integer lua_tointegerx(lua_State *, int, int *)\x00",
                )).as_ptr(),
            );
        };
        res = (*o).value_.i;
        1i32
    } else {
        luaV_tointeger(o, &mut res, 0i32)
    };
    if 0 == isnum {
        /* call to 'tointeger' may change 'n' even if it fails */
        res = 0i32 as lua_Integer
    }
    if !pisnum.is_null() {
        *pisnum = isnum
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_toboolean(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return !((*o).tt_ == 0i32 || (*o).tt_ == 1i32 && {
        if (*o).tt_ == 1i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (1))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                370i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                    b"int lua_toboolean(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        (*o).value_.b == 0i32
    }) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tolstring(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut o: StkId = index2addr(L, idx);
    if !((*o).tt_ & 0xfi32 == 4i32) {
        if !((*o).tt_ & 0xfi32 == 3i32) {
            /* not convertible? */
            if !len.is_null() {
                *len = 0i32 as size_t
            }
            return 0 as *const libc::c_char;
        } else {
            let ref mut fresh22 = *(*((L as *mut libc::c_char)
                .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA))
                .plock;
            let fresh23 = *fresh22;
            *fresh22 = *fresh22 + 1;
            if fresh23 == 0i32 {
            } else {
                __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char, 381i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 54],
                                                        &[libc::c_char; 54]>(b"const char *lua_tolstring(lua_State *, int, size_t *)\x00")).as_ptr());
            };
            /* 'luaO_tostring' may create a new string */
            luaO_tostring(L, o);
            if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                luaC_step(L);
            }
            /* previous call may reallocate the stack */
            o = index2addr(L, idx);
            let ref mut fresh24 = *(*((L as *mut libc::c_char)
                .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA))
                .plock;
            *fresh24 -= 1;
            if *fresh24 == 0i32 {
            } else {
                __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char, 385i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 54],
                                                        &[libc::c_char; 54]>(b"const char *lua_tolstring(lua_State *, int, size_t *)\x00")).as_ptr());
            };
        }
    }
    if !len.is_null() {
        if (*o).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                388i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                    b"const char *lua_tolstring(lua_State *, int, size_t *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                388i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                    b"const char *lua_tolstring(lua_State *, int, size_t *)\x00",
                )).as_ptr(),
            );
        };
        *len = if (*(&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString_0)).tt
            as libc::c_int == 4i32 | 0i32 << 4i32
        {
            if (*o).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    388i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                        b"const char *lua_tolstring(lua_State *, int, size_t *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    388i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                        b"const char *lua_tolstring(lua_State *, int, size_t *)\x00",
                    )).as_ptr(),
                );
            };
            (*&mut (*((*o).value_.gc as *mut GCUnion)).ts).shrlen as libc::c_ulong
        } else {
            if (*o).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    388i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                        b"const char *lua_tolstring(lua_State *, int, size_t *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    388i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                        b"const char *lua_tolstring(lua_State *, int, size_t *)\x00",
                    )).as_ptr(),
                );
            };
            (*&mut (*((*o).value_.gc as *mut GCUnion)).ts).u.lnglen
        }
    }
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof((((((((((((o))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((o))->tt_)) & 0x0F)) == (4))\", \"lapi.c\", 389, __extension__ __PRETTY_FUNCTION__)), (((((((((o)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((o)->value_).gc)->tt) & 0x0F) == 4\", \"lapi.c\", 389, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((o)->value_).gc))))->ts))))))->extra)\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      389i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"const char *lua_tolstring(lua_State *, int, size_t *)\x00")).as_ptr());
    };
    if (*o).tt_ & 0xfi32 == 4i32 {
    } else {
        __assert_fail(
            b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            389i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"const char *lua_tolstring(lua_State *, int, size_t *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
    } else {
        __assert_fail(
            b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            389i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"const char *lua_tolstring(lua_State *, int, size_t *)\x00",
            )).as_ptr(),
        );
    };
    return (&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString_0 as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawlen(mut L: *mut lua_State_0, mut idx: libc::c_int) -> size_t {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3fi32 {
        4 => {
            if (*o).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    396i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"size_t lua_rawlen(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    396i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"size_t lua_rawlen(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return (*&mut (*((*o).value_.gc as *mut GCUnion)).ts).shrlen as size_t;
        }
        20 => {
            if (*o).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    397i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"size_t lua_rawlen(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    397i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"size_t lua_rawlen(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return (*&mut (*((*o).value_.gc as *mut GCUnion)).ts).u.lnglen;
        }
        7 => {
            if (*o).tt_ == 7i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (((7) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    398i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"size_t lua_rawlen(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 7i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    398i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"size_t lua_rawlen(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return (*&mut (*((*o).value_.gc as *mut GCUnion)).u).len;
        }
        5 => {
            if (*o).tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    399i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"size_t lua_rawlen(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    399i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                        b"size_t lua_rawlen(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return luaH_getn(&mut (*((*o).value_.gc as *mut GCUnion)).h) as size_t;
        }
        _ => return 0i32 as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tocfunction(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> lua_CFunction {
    let mut o: StkId = index2addr(L, idx);
    if (*o).tt_ == 6i32 | 1i32 << 4i32 {
        if (*o).tt_ == 6i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == ((6 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                407i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"lua_CFunction lua_tocfunction(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        return (*o).value_.f;
    } else if (*o).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
        if (*o).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                409i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"lua_CFunction lua_tocfunction(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                409i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"lua_CFunction lua_tocfunction(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        return (*&mut (*((*o).value_.gc as *mut GCUnion)).cl.c).f;
    } else {
        return None;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_touserdata(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> *mut libc::c_void {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0xfi32 {
        7 => {
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(b"sizeof(((((((((o))->tt_) == (((7) | (1 << 6))))) ? (void) (0) : __assert_fail (\"((((o))->tt_) == (((7) | (1 << 6))))\", \"lapi.c\", 417, __extension__ __PRETTY_FUNCTION__)), (((((((o)->value_).gc)->tt == 7) ? (void) (0) : __assert_fail (\"(((o)->value_).gc)->tt == 7\", \"lapi.c\", 417, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((o)->value_).gc))))->u))))))->ttuv_)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char, 417i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 39],
                                                        &[libc::c_char; 39]>(b"void *lua_touserdata(lua_State *, int)\x00")).as_ptr());
            };
            if (*o).tt_ == 7i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (((7) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    417i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"void *lua_touserdata(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 7i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    417i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"void *lua_touserdata(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata as *mut libc::c_char)
                .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *mut libc::c_void;
        }
        2 => {
            if (*o).tt_ == 2i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (2))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    418i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"void *lua_touserdata(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return (*o).value_.p;
        }
        _ => return 0 as *mut libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tothread(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> *mut lua_State_0 {
    let mut o: StkId = index2addr(L, idx);
    return if !((*o).tt_ == 8i32 | 1i32 << 6i32) {
        0 as *mut lua_State
    } else {
        if (*o).tt_ == 8i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((8) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                426i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"lua_State *lua_tothread(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 8i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 8\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                426i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"lua_State *lua_tothread(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        &mut (*((*o).value_.gc as *mut GCUnion)).th
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_topointer(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> *const libc::c_void {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3fi32 {
        5 => {
            if (*o).tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    433i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    433i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return &mut (*((*o).value_.gc as *mut GCUnion)).h as *mut Table as *const libc::c_void;
        }
        6 => {
            if (*o).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    434i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    434i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.l as *mut LClosure
                as *const libc::c_void;
        }
        38 => {
            if (*o).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    435i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == (6 | (2 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    435i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.c as *mut CClosure
                as *const libc::c_void;
        }
        22 => {
            if (*o).tt_ == 6i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == ((6 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    436i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return ::std::mem::transmute::<lua_CFunction, size_t>((*o).value_.f)
                as *mut libc::c_void;
        }
        8 => {
            if (*o).tt_ == 8i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (((8) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    437i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 8i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == 8\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    437i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return &mut (*((*o).value_.gc as *mut GCUnion)).th as *mut lua_State
                as *const libc::c_void;
        }
        7 => {
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(b"sizeof(((((((((o))->tt_) == (((7) | (1 << 6))))) ? (void) (0) : __assert_fail (\"((((o))->tt_) == (((7) | (1 << 6))))\", \"lapi.c\", 438, __extension__ __PRETTY_FUNCTION__)), (((((((o)->value_).gc)->tt == 7) ? (void) (0) : __assert_fail (\"(((o)->value_).gc)->tt == 7\", \"lapi.c\", 438, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((o)->value_).gc))))->u))))))->ttuv_)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char, 438i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 44],
                                                        &[libc::c_char; 44]>(b"const void *lua_topointer(lua_State *, int)\x00")).as_ptr());
            };
            if (*o).tt_ == 7i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (((7) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    438i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 7i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    438i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata as *mut libc::c_char)
                .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *const libc::c_void;
        }
        2 => {
            if (*o).tt_ == 2i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (2))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    439i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"const void *lua_topointer(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            return (*o).value_.p;
        }
        _ => return 0 as *const libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_arith(mut L: *mut lua_State_0, mut op: libc::c_int) -> () {
    let ref mut fresh25 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh26 = *fresh25;
    *fresh25 = *fresh25 + 1;
    if fresh26 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      304i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void lua_arith(lua_State *, int)\x00")).as_ptr());
    };
    if op != 12i32 && op != 13i32 {
        if (2i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
            && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"((2) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                    as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                306i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"void lua_arith(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    } else {
        /* all other operations expect two operands */
        /* for unary operations, add fake 2nd operand */
        if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
            && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                    as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                308i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"void lua_arith(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        let mut io1: *mut TValue = (*L).top;
        *io1 = *(*L).top.offset(-1isize);
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    309i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                        b"void lua_arith(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        309i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                            b"void lua_arith(lua_State *, int)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    309i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                        b"void lua_arith(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                310i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"void lua_arith(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    }
    /* first operand at top - 2, second at top - 1; result go to top - 2 */
    luaO_arith(
        L,
        op,
        (*L).top.offset(-2isize) as *const TValue,
        (*L).top.offset(-1isize) as *const TValue,
        (*L).top.offset(-2isize),
    );
    /* remove second operand */
    (*L).top = (*L).top.offset(-1isize);
    let ref mut fresh27 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh27 -= 1;
    if *fresh27 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      315i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void lua_arith(lua_State *, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawequal(
    mut L: *mut lua_State_0,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
) -> libc::c_int {
    let mut o1: StkId = index2addr(L, index1);
    let mut o2: StkId = index2addr(L, index2);
    return if o1 != &luaO_nilobject_ as *const TValue as StkId
        && o2 != &luaO_nilobject_ as *const TValue as StkId
    {
        luaV_equalobj(
            0 as *mut lua_State_0,
            o1 as *const TValue,
            o2 as *const TValue,
        )
    } else {
        0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_compare(
    mut L: *mut lua_State_0,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut o1: StkId = 0 as *mut TValue;
    let mut o2: StkId = 0 as *mut TValue;
    let mut i: libc::c_int = 0i32;
    let ref mut fresh28 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh29 = *fresh28;
    *fresh28 = *fresh28 + 1;
    if fresh29 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      322i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"int lua_compare(lua_State *, int, int, int)\x00")).as_ptr());
    };
    /* may call tag method */
    o1 = index2addr(L, index1);
    o2 = index2addr(L, index2);
    if o1 != &luaO_nilobject_ as *const TValue as StkId
        && o2 != &luaO_nilobject_ as *const TValue as StkId
    {
        match op {
            0 => i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue),
            1 => i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue),
            2 => i = luaV_lessequal(L, o1 as *const TValue, o2 as *const TValue),
            _ => {
                if 0 != 0i32
                    && !(b"invalid option\x00" as *const u8 as *const libc::c_char).is_null()
                {
                } else {
                    __assert_fail(
                        b"(0) && \"invalid option\"\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        330i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                            b"int lua_compare(lua_State *, int, int, int)\x00",
                        )).as_ptr(),
                    );
                };
            }
        }
    }
    let ref mut fresh30 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh30 -= 1;
    if *fresh30 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      333i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"int lua_compare(lua_State *, int, int, int)\x00")).as_ptr());
    };
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnil(mut L: *mut lua_State_0) -> () {
    let ref mut fresh31 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh32 = *fresh31;
    *fresh31 = *fresh31 + 1;
    if fresh32 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      452i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void lua_pushnil(lua_State *)\x00")).as_ptr());
    };
    (*(*L).top).tt_ = 0i32;
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            454i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void lua_pushnil(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh33 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh33 -= 1;
    if *fresh33 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      455i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void lua_pushnil(lua_State *)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnumber(mut L: *mut lua_State_0, mut n: lua_Number) -> () {
    let ref mut fresh34 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh35 = *fresh34;
    *fresh34 = *fresh34 + 1;
    if fresh35 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      460i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void lua_pushnumber(lua_State *, lua_Number)\x00")).as_ptr());
    };
    let mut io: *mut TValue = (*L).top;
    (*io).value_.n = n;
    (*io).tt_ = 3i32 | 0i32 << 4i32;
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            462i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"void lua_pushnumber(lua_State *, lua_Number)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh36 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh36 -= 1;
    if *fresh36 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      463i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void lua_pushnumber(lua_State *, lua_Number)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushinteger(mut L: *mut lua_State_0, mut n: lua_Integer) -> () {
    let ref mut fresh37 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh38 = *fresh37;
    *fresh37 = *fresh37 + 1;
    if fresh38 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      468i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void lua_pushinteger(lua_State *, lua_Integer)\x00")).as_ptr());
    };
    let mut io: *mut TValue = (*L).top;
    (*io).value_.i = n;
    (*io).tt_ = 3i32 | 1i32 << 4i32;
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            470i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"void lua_pushinteger(lua_State *, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh39 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh39 -= 1;
    if *fresh39 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      471i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void lua_pushinteger(lua_State *, lua_Integer)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlstring(
    mut L: *mut lua_State_0,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut ts: *mut TString = 0 as *mut TString;
    let ref mut fresh40 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh41 = *fresh40;
    *fresh40 = *fresh40 + 1;
    if fresh41 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      482i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 63],
                                                &[libc::c_char; 63]>(b"const char *lua_pushlstring(lua_State *, const char *, size_t)\x00")).as_ptr());
    };
    ts = if len == 0i32 as libc::c_ulong {
        luaS_new(L, b"\x00" as *const u8 as *const libc::c_char)
    } else {
        luaS_newlstr(L, s, len)
    };
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = ts;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            484i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"const char *lua_pushlstring(lua_State *, const char *, size_t)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                484i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"const char *lua_pushlstring(lua_State *, const char *, size_t)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    484i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                        b"const char *lua_pushlstring(lua_State *, const char *, size_t)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                484i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"const char *lua_pushlstring(lua_State *, const char *, size_t)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            485i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"const char *lua_pushlstring(lua_State *, const char *, size_t)\x00",
            )).as_ptr(),
        );
    };
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    let ref mut fresh42 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh42 -= 1;
    if *fresh42 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      487i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 63],
                                                &[libc::c_char; 63]>(b"const char *lua_pushlstring(lua_State *, const char *, size_t)\x00")).as_ptr());
    };
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof((ts)->extra)\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            488i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"const char *lua_pushlstring(lua_State *, const char *, size_t)\x00",
            )).as_ptr(),
        );
    };
    return (ts as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushstring(
    mut L: *mut lua_State_0,
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    let ref mut fresh43 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh44 = *fresh43;
    *fresh43 = *fresh43 + 1;
    if fresh44 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      493i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"const char *lua_pushstring(lua_State *, const char *)\x00")).as_ptr());
    };
    if s.is_null() {
        (*(*L).top).tt_ = 0i32
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        ts = luaS_new(L, s);
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = ts;
        if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                499i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                    b"const char *lua_pushstring(lua_State *, const char *)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    499i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                        b"const char *lua_pushstring(lua_State *, const char *)\x00",
                    )).as_ptr(),
                );
            };
            (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        499i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                            b"const char *lua_pushstring(lua_State *, const char *)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    499i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                        b"const char *lua_pushstring(lua_State *, const char *)\x00",
                    )).as_ptr(),
                );
            };
        };
        /* internal copy's address */
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((ts)->extra)\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                500i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                    b"const char *lua_pushstring(lua_State *, const char *)\x00",
                )).as_ptr(),
            );
        };
        s = (ts as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
    }
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            502i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"const char *lua_pushstring(lua_State *, const char *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    let ref mut fresh45 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh45 -= 1;
    if *fresh45 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      504i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"const char *lua_pushstring(lua_State *, const char *)\x00")).as_ptr());
    };
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvfstring(
    mut L: *mut lua_State_0,
    mut fmt: *const libc::c_char,
    mut argp: *mut __va_list_tag,
) -> *const libc::c_char {
    let mut ret: *const libc::c_char = 0 as *const libc::c_char;
    let ref mut fresh46 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh47 = *fresh46;
    *fresh46 = *fresh46 + 1;
    if fresh47 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      512i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"const char *lua_pushvfstring(lua_State *, const char *, struct __va_list_tag *)\x00")).as_ptr());
    };
    ret = luaO_pushvfstring(L, fmt, argp);
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    let ref mut fresh48 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh48 -= 1;
    if *fresh48 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      515i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"const char *lua_pushvfstring(lua_State *, const char *, struct __va_list_tag *)\x00")).as_ptr());
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushcclosure(
    mut L: *mut lua_State_0,
    mut fn_0: lua_CFunction,
    mut n: libc::c_int,
) -> () {
    let ref mut fresh49 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh50 = *fresh49;
    *fresh49 = *fresh49 + 1;
    if fresh50 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      521i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00")).as_ptr());
    };
    if n == 0i32 {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.f = fn_0;
        (*io).tt_ = 6i32 | 1i32 << 4i32
    } else {
        let mut cl: *mut CClosure = 0 as *mut CClosure;
        if (n as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
            && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"((n) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                    as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                527i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                    b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
                )).as_ptr(),
            );
        };
        if n <= 255i32
            && !(b"upvalue index too large\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(n <= 255) && \"upvalue index too large\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                528i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                    b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
                )).as_ptr(),
            );
        };
        cl = luaF_newCclosure(L, n);
        (*cl).f = fn_0;
        (*L).top = (*L).top.offset(-(n as isize));
        loop {
            let fresh51 = n;
            n = n - 1;
            if !(0 != fresh51) {
                break;
            }
            let mut io1: *mut TValue =
                &mut *(*cl).upvalue.as_mut_ptr().offset(n as isize) as *mut TValue;
            *io1 = *(*L).top.offset(n as isize);
            if 0 == (*io1).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        533i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                            b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
                        )).as_ptr(),
                    );
                };
                (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            533i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                                b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
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
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        533i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                            b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
                        )).as_ptr(),
                    );
                };
            };
        }
        let mut io_0: *mut TValue = (*L).top;
        let mut x_: *mut CClosure = cl;
        if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                536i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                    b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
                )).as_ptr(),
            );
        };
        (*io_0).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io_0).tt_ = 6i32 | 2i32 << 4i32 | 1i32 << 6i32;
        if 0 == (*io_0).tt_ & 1i32 << 6i32 || {
            if 0 != (*io_0).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    536i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io_0).tt_ & 0x3fi32 == (*(*io_0).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io_0).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        536i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                            b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
                        )).as_ptr(),
                    );
                };
                0 != ((*(*io_0).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    536i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
                    )).as_ptr(),
                );
            };
        };
    }
    /* does not need barrier because closure is white */
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            538i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00",
            )).as_ptr(),
        );
    };
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    let ref mut fresh52 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh52 -= 1;
    if *fresh52 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      540i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void lua_pushcclosure(lua_State *, lua_CFunction, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushboolean(mut L: *mut lua_State_0, mut b: libc::c_int) -> () {
    let ref mut fresh53 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh54 = *fresh53;
    *fresh53 = *fresh53 + 1;
    if fresh54 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      545i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"void lua_pushboolean(lua_State *, int)\x00")).as_ptr());
    };
    let mut io: *mut TValue = (*L).top;
    (*io).value_.b = (b != 0i32) as libc::c_int;
    (*io).tt_ = 1i32;
    /* ensure that true is 1 */
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            547i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void lua_pushboolean(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh55 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh55 -= 1;
    if *fresh55 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      548i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"void lua_pushboolean(lua_State *, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlightuserdata(
    mut L: *mut lua_State_0,
    mut p: *mut libc::c_void,
) -> () {
    let ref mut fresh56 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh57 = *fresh56;
    *fresh56 = *fresh56 + 1;
    if fresh57 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      553i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void lua_pushlightuserdata(lua_State *, void *)\x00")).as_ptr());
    };
    let mut io: *mut TValue = (*L).top;
    (*io).value_.p = p;
    (*io).tt_ = 2i32;
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            555i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void lua_pushlightuserdata(lua_State *, void *)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh58 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh58 -= 1;
    if *fresh58 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      556i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void lua_pushlightuserdata(lua_State *, void *)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushthread(mut L: *mut lua_State_0) -> libc::c_int {
    let ref mut fresh59 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh60 = *fresh59;
    *fresh59 = *fresh59 + 1;
    if fresh60 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      561i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"int lua_pushthread(lua_State *)\x00")).as_ptr());
    };
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut lua_State_0 = L;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            562i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"int lua_pushthread(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 8i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 || {
        if 0 != (*io).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                562i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"int lua_pushthread(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    562i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"int lua_pushthread(lua_State *)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                562i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"int lua_pushthread(lua_State *)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            563i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"int lua_pushthread(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh61 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh61 -= 1;
    if *fresh61 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      564i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"int lua_pushthread(lua_State *)\x00")).as_ptr());
    };
    return ((*(*L).l_G).mainthread == L) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getglobal(
    mut L: *mut lua_State_0,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if (*(*L).l_G).l_registry.tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((&(L->l_G)->l_registry))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            593i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"int lua_getglobal(lua_State *, const char *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(*L).l_G).l_registry.value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((&(L->l_G)->l_registry)->value_).gc)->tt == 5\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            593i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"int lua_getglobal(lua_State *, const char *)\x00",
            )).as_ptr(),
        );
    };
    let mut reg: *mut Table_0 = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
    let ref mut fresh62 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh63 = *fresh62;
    *fresh62 = *fresh62 + 1;
    if fresh63 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      594i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"int lua_getglobal(lua_State *, const char *)\x00")).as_ptr());
    };
    return auxgetstr(L, luaH_getint(reg, 2i32 as lua_Integer), name);
}
/*
** get functions (Lua -> stack)
*/
unsafe extern "C" fn auxgetstr(
    mut L: *mut lua_State_0,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) -> libc::c_int {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        if (*t).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                578i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                578i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
                )).as_ptr(),
            );
        };
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        !((*slot).tt_ == 0i32) as libc::c_int
    } {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *slot;
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    579i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                        b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        579i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                            b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    579i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                        b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
                    )).as_ptr(),
                );
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                580i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
                )).as_ptr(),
            );
        };
    } else {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = str;
        if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                583i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    583i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                        b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
                    )).as_ptr(),
                );
            };
            (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        583i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                            b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    583i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                        b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
                    )).as_ptr(),
                );
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                584i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"int auxgetstr(lua_State *, const TValue *, const char *)\x00",
                )).as_ptr(),
            );
        };
        luaV_finishget(
            L,
            t,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    let ref mut fresh64 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh64 -= 1;
    if *fresh64 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      587i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"int auxgetstr(lua_State *, const TValue *, const char *)\x00")).as_ptr());
    };
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettable(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let ref mut fresh65 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh66 = *fresh65;
    *fresh65 = *fresh65 + 1;
    if fresh66 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      601i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"int lua_gettable(lua_State *, int)\x00")).as_ptr());
    };
    t = index2addr(L, idx);
    let mut slot: *const TValue = 0 as *const TValue;
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        if (*t).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                603i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"int lua_gettable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                603i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"int lua_gettable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            (*L).top.offset(-1isize) as *const TValue,
        );
        !((*slot).tt_ == 0i32) as libc::c_int
    } {
        let mut io1: *mut TValue = (*L).top.offset(-1isize);
        *io1 = *slot;
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    603i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"int lua_gettable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        603i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                            b"int lua_gettable(lua_State *, int)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    603i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"int lua_gettable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
        };
    } else {
        luaV_finishget(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    let ref mut fresh67 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh67 -= 1;
    if *fresh67 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      604i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"int lua_gettable(lua_State *, int)\x00")).as_ptr());
    };
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getfield(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut k: *const libc::c_char,
) -> libc::c_int {
    let ref mut fresh68 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh69 = *fresh68;
    *fresh68 = *fresh68 + 1;
    if fresh69 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      610i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"int lua_getfield(lua_State *, int, const char *)\x00")).as_ptr());
    };
    return auxgetstr(L, index2addr(L, idx), k);
}
#[no_mangle]
pub unsafe extern "C" fn lua_geti(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    let ref mut fresh70 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh71 = *fresh70;
    *fresh70 = *fresh70 + 1;
    if fresh71 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      618i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"int lua_geti(lua_State *, int, lua_Integer)\x00")).as_ptr());
    };
    t = index2addr(L, idx);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        if (*t).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                620i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"int lua_geti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                620i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"int lua_geti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        !((*slot).tt_ == 0i32) as libc::c_int
    } {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *slot;
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    621i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"int lua_geti(lua_State *, int, lua_Integer)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        621i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                            b"int lua_geti(lua_State *, int, lua_Integer)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    621i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"int lua_geti(lua_State *, int, lua_Integer)\x00",
                    )).as_ptr(),
                );
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                622i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"int lua_geti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                626i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"int lua_geti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        luaV_finishget(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    let ref mut fresh72 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh72 -= 1;
    if *fresh72 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      629i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"int lua_geti(lua_State *, int, lua_Integer)\x00")).as_ptr());
    };
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawget(mut L: *mut lua_State_0, mut idx: libc::c_int) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let ref mut fresh73 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh74 = *fresh73;
    *fresh73 = *fresh73 + 1;
    if fresh74 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      636i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int lua_rawget(lua_State *, int)\x00")).as_ptr());
    };
    t = index2addr(L, idx);
    if (*t).tt_ == 5i32 | 1i32 << 6i32
        && !(b"table expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((t))->tt_) == (((5) | (1 << 6))))) && \"table expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            638i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"int lua_rawget(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let mut io1: *mut TValue = (*L).top.offset(-1isize);
    if (*t).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            639i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"int lua_rawget(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            639i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"int lua_rawget(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    *io1 = *luaH_get(
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-1isize) as *const TValue,
    );
    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                639i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_rawget(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    639i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                        b"int lua_rawget(lua_State *, int)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                639i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_rawget(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    };
    let ref mut fresh75 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh75 -= 1;
    if *fresh75 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      640i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int lua_rawget(lua_State *, int)\x00")).as_ptr());
    };
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgeti(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let ref mut fresh76 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh77 = *fresh76;
    *fresh76 = *fresh76 + 1;
    if fresh77 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      647i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00")).as_ptr());
    };
    t = index2addr(L, idx);
    if (*t).tt_ == 5i32 | 1i32 << 6i32
        && !(b"table expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((t))->tt_) == (((5) | (1 << 6))))) && \"table expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            649i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    let mut io1: *mut TValue = (*L).top;
    if (*t).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            650i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            650i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    *io1 = *luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                650i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    650i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                650i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            651i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh78 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh78 -= 1;
    if *fresh78 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      652i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"int lua_rawgeti(lua_State *, int, lua_Integer)\x00")).as_ptr());
    };
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgetp(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut k: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let ref mut fresh79 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh80 = *fresh79;
    *fresh79 = *fresh79 + 1;
    if fresh80 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      660i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"int lua_rawgetp(lua_State *, int, const void *)\x00")).as_ptr());
    };
    t = index2addr(L, idx);
    if (*t).tt_ == 5i32 | 1i32 << 6i32
        && !(b"table expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((t))->tt_) == (((5) | (1 << 6))))) && \"table expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            662i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"int lua_rawgetp(lua_State *, int, const void *)\x00",
            )).as_ptr(),
        );
    };
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut libc::c_void;
    (*io).tt_ = 2i32;
    let mut io1: *mut TValue = (*L).top;
    if (*t).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            664i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"int lua_rawgetp(lua_State *, int, const void *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            664i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"int lua_rawgetp(lua_State *, int, const void *)\x00",
            )).as_ptr(),
        );
    };
    *io1 = *luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, &mut k);
    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                664i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"int lua_rawgetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    664i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                        b"int lua_rawgetp(lua_State *, int, const void *)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                664i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"int lua_rawgetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            665i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"int lua_rawgetp(lua_State *, int, const void *)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh81 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh81 -= 1;
    if *fresh81 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      666i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"int lua_rawgetp(lua_State *, int, const void *)\x00")).as_ptr());
    };
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_createtable(
    mut L: *mut lua_State_0,
    mut narray: libc::c_int,
    mut nrec: libc::c_int,
) -> () {
    let mut t: *mut Table_0 = 0 as *mut Table_0;
    let ref mut fresh82 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh83 = *fresh82;
    *fresh82 = *fresh82 + 1;
    if fresh83 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      673i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"void lua_createtable(lua_State *, int, int)\x00")).as_ptr());
    };
    t = luaH_new(L);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut Table_0 = t;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            675i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void lua_createtable(lua_State *, int, int)\x00",
            )).as_ptr(),
        );
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 || {
        if 0 != (*io).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                675i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"void lua_createtable(lua_State *, int, int)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    675i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"void lua_createtable(lua_State *, int, int)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                675i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"void lua_createtable(lua_State *, int, int)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            676i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void lua_createtable(lua_State *, int, int)\x00",
            )).as_ptr(),
        );
    };
    if narray > 0i32 || nrec > 0i32 {
        luaH_resize(L, t, narray as libc::c_uint, nrec as libc::c_uint);
    }
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    let ref mut fresh84 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh84 -= 1;
    if *fresh84 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      680i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"void lua_createtable(lua_State *, int, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_newuserdata(
    mut L: *mut lua_State_0,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut u: *mut Udata_0 = 0 as *mut Udata_0;
    let ref mut fresh85 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh86 = *fresh85;
    *fresh85 = *fresh85 + 1;
    if fresh86 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1173i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"void *lua_newuserdata(lua_State *, size_t)\x00")).as_ptr());
    };
    u = luaS_newudata(L, size);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut Udata_0 = u;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1175i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"void *lua_newuserdata(lua_State *, size_t)\x00",
            )).as_ptr(),
        );
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 7i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 || {
        if 0 != (*io).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                1175i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                    b"void *lua_newuserdata(lua_State *, size_t)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1175i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"void *lua_newuserdata(lua_State *, size_t)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                1175i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                    b"void *lua_newuserdata(lua_State *, size_t)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1176i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"void *lua_newuserdata(lua_State *, size_t)\x00",
            )).as_ptr(),
        );
    };
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    let ref mut fresh87 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh87 -= 1;
    if *fresh87 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1178i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"void *lua_newuserdata(lua_State *, size_t)\x00")).as_ptr());
    };
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof((u)->ttuv_)\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1179i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"void *lua_newuserdata(lua_State *, size_t)\x00",
            )).as_ptr(),
        );
    };
    return (u as *mut libc::c_char)
        .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getmetatable(
    mut L: *mut lua_State_0,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj: *const TValue = 0 as *const TValue;
    let mut mt: *mut Table_0 = 0 as *mut Table_0;
    let mut res: libc::c_int = 0i32;
    let ref mut fresh88 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh89 = *fresh88;
    *fresh88 = *fresh88 + 1;
    if fresh89 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      688i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"int lua_getmetatable(lua_State *, int)\x00")).as_ptr());
    };
    obj = index2addr(L, objindex);
    match (*obj).tt_ & 0xfi32 {
        5 => {
            if (*obj).tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((obj))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    692i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_getmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*obj).value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(((obj)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    692i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_getmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            mt = (*&mut (*((*obj).value_.gc as *mut GCUnion)).h).metatable
        }
        7 => {
            if (*obj).tt_ == 7i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((obj))->tt_) == (((7) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    695i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_getmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*obj).value_.gc).tt as libc::c_int == 7i32 {
            } else {
                __assert_fail(
                    b"(((obj)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    695i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_getmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            mt = (*&mut (*((*obj).value_.gc as *mut GCUnion)).u).metatable
        }
        _ => mt = (*(*L).l_G).mt[((*obj).tt_ & 0xfi32) as usize],
    }
    if !mt.is_null() {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut Table_0 = mt;
        if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                702i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"int lua_getmetatable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = 5i32 | 1i32 << 6i32;
        if 0 == (*io).tt_ & 1i32 << 6i32 || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    702i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_getmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        702i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                            b"int lua_getmetatable(lua_State *, int)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    702i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_getmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                703i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"int lua_getmetatable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        res = 1i32
    }
    let ref mut fresh90 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh90 -= 1;
    if *fresh90 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      706i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"int lua_getmetatable(lua_State *, int)\x00")).as_ptr());
    };
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getuservalue(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = 0 as *mut TValue;
    let ref mut fresh91 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh92 = *fresh91;
    *fresh91 = *fresh91 + 1;
    if fresh92 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      713i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"int lua_getuservalue(lua_State *, int)\x00")).as_ptr());
    };
    o = index2addr(L, idx);
    if (*o).tt_ == 7i32 | 1i32 << 6i32
        && !(b"full userdata expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((o))->tt_) == (((7) | (1 << 6))))) && \"full userdata expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            715i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"int lua_getuservalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let mut io: *mut TValue = (*L).top;
    if (*o).tt_ == 7i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((o))->tt_) == (((7) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            716i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"int lua_getuservalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if (*(*o).value_.gc).tt as libc::c_int == 7i32 {
    } else {
        __assert_fail(
            b"(((o)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            716i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"int lua_getuservalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let mut iu: *const Udata_0 = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*io).value_ = (*iu).user_;
    (*io).tt_ = (*iu).ttuv_ as libc::c_int;
    if 0 == (*io).tt_ & 1i32 << 6i32 || {
        if 0 != (*io).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                716i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"int lua_getuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    716i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_getuservalue(lua_State *, int)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                716i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"int lua_getuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            717i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"int lua_getuservalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh93 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh93 -= 1;
    if *fresh93 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      718i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"int lua_getuservalue(lua_State *, int)\x00")).as_ptr());
    };
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setglobal(
    mut L: *mut lua_State_0,
    mut name: *const libc::c_char,
) -> () {
    if (*(*L).l_G).l_registry.tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((&(L->l_G)->l_registry))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            747i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void lua_setglobal(lua_State *, const char *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(*L).l_G).l_registry.value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((&(L->l_G)->l_registry)->value_).gc)->tt == 5\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            747i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void lua_setglobal(lua_State *, const char *)\x00",
            )).as_ptr(),
        );
    };
    let mut reg: *mut Table_0 = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
    let ref mut fresh94 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh95 = *fresh94;
    *fresh94 = *fresh94 + 1;
    if fresh95 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      748i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"void lua_setglobal(lua_State *, const char *)\x00")).as_ptr());
    };
    /* unlock done in 'auxsetstr' */
    auxsetstr(L, luaH_getint(reg, 2i32 as lua_Integer), name);
}
/*
** set functions (stack -> Lua)
*/
/*
** t[k] = value at the top of the stack (where 'k' is a string)
*/
unsafe extern "C" fn auxsetstr(
    mut L: *mut lua_State_0,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) -> () {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            733i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
            )).as_ptr(),
        );
    };
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        if (*t).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                734i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                734i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                )).as_ptr(),
            );
        };
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 && {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        734i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                            b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        734i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                            b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(&mut (*((*t).value_.gc as *mut GCUnion)).h as *mut Table)).marked
                    as libc::c_int & 1i32 << 2i32
            } && {
                if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((L->top - 1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        734i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                            b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        734i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                            b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        734i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                            b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                        )).as_ptr(),
                    );
                };
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            if 0 == (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 || {
                if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        734i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                            b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                        )).as_ptr(),
                    );
                };
                (*(slot as *mut TValue)).tt_ & 0x3fi32
                    == (*(*(slot as *mut TValue)).value_.gc).tt as libc::c_int
                    && (L.is_null() || {
                        if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"lapi.c\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char,
                                                             734i32 as
                                                                 libc::c_uint,
                                                             (*::std::mem::transmute::<&[u8; 58],
                                                                                       &[libc::c_char; 58]>(b"void auxsetstr(lua_State *, const TValue *, const char *)\x00")).as_ptr());
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
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        734i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                            b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            1i32
        }
    } {
        /* pop value */
        (*L).top = (*L).top.offset(-1isize)
    } else {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = str;
        if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                737i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    737i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                        b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                    )).as_ptr(),
                );
            };
            (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        737i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                            b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    737i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                        b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                    )).as_ptr(),
                );
            };
        };
        /* push 'str' (to make it a TValue) */
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                738i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"void auxsetstr(lua_State *, const TValue *, const char *)\x00",
                )).as_ptr(),
            );
        };
        luaV_finishset(
            L,
            t,
            (*L).top.offset(-1isize),
            (*L).top.offset(-2isize),
            slot,
        );
        /* pop value and key */
        (*L).top = (*L).top.offset(-2isize)
    }
    let ref mut fresh96 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh96 -= 1;
    if *fresh96 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      742i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"void auxsetstr(lua_State *, const TValue *, const char *)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_settable(mut L: *mut lua_State_0, mut idx: libc::c_int) -> () {
    let mut t: StkId = 0 as *mut TValue;
    let ref mut fresh97 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh98 = *fresh97;
    *fresh97 = *fresh97 + 1;
    if fresh98 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      755i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void lua_settable(lua_State *, int)\x00")).as_ptr());
    };
    if (2i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((2) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            756i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"void lua_settable(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    t = index2addr(L, idx);
    let mut slot: *const TValue = 0 as *const TValue;
    if 0 == if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        if (*t).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                758i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                    b"void lua_settable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                758i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                    b"void lua_settable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            (*L).top.offset(-2isize) as *const TValue,
        );
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 && {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        758i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void lua_settable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        758i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void lua_settable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(&mut (*((*t).value_.gc as *mut GCUnion)).h as *mut Table)).marked
                    as libc::c_int & 1i32 << 2i32
            } && {
                if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((L->top - 1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        758i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void lua_settable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        758i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void lua_settable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        758i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void lua_settable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            if 0 == (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 || {
                if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        758i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void lua_settable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                (*(slot as *mut TValue)).tt_ & 0x3fi32
                    == (*(*(slot as *mut TValue)).value_.gc).tt as libc::c_int
                    && (L.is_null() || {
                        if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                                758i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                                    b"void lua_settable(lua_State *, int)\x00",
                                )).as_ptr(),
                            );
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
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        758i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void lua_settable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
            };
            1i32
        }
    } {
        luaV_finishset(
            L,
            t as *const TValue,
            (*L).top.offset(-2isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    /* pop index and value */
    (*L).top = (*L).top.offset(-2isize);
    let ref mut fresh99 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh99 -= 1;
    if *fresh99 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      760i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void lua_settable(lua_State *, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setfield(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut k: *const libc::c_char,
) -> () {
    let ref mut fresh100 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh101 = *fresh100;
    *fresh100 = *fresh100 + 1;
    if fresh101 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      765i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"void lua_setfield(lua_State *, int, const char *)\x00")).as_ptr());
    };
    /* unlock done in 'auxsetstr' */
    auxsetstr(L, index2addr(L, idx), k);
}
#[no_mangle]
pub unsafe extern "C" fn lua_seti(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> () {
    let mut t: StkId = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    let ref mut fresh102 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh103 = *fresh102;
    *fresh102 = *fresh102 + 1;
    if fresh103 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      773i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void lua_seti(lua_State *, int, lua_Integer)\x00")).as_ptr());
    };
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            774i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"void lua_seti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    t = index2addr(L, idx);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        if (*t).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                776i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                776i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 && {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        776i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        776i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(&mut (*((*t).value_.gc as *mut GCUnion)).h as *mut Table)).marked
                    as libc::c_int & 1i32 << 2i32
            } && {
                if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((L->top - 1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        776i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                if (*t).tt_ == 5i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        776i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        776i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                        )).as_ptr(),
                    );
                };
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            if 0 == (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 || {
                if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        776i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                        )).as_ptr(),
                    );
                };
                (*(slot as *mut TValue)).tt_ & 0x3fi32
                    == (*(*(slot as *mut TValue)).value_.gc).tt as libc::c_int
                    && (L.is_null() || {
                        if 0 != (*(slot as *mut TValue)).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"((((((TValue *)(slot))))->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                                776i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                                    b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                                )).as_ptr(),
                            );
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
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        776i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                        )).as_ptr(),
                    );
                };
            };
            1i32
        }
    } {
        /* pop value */
        (*L).top = (*L).top.offset(-1isize)
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                780i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void lua_seti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        luaV_finishset(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-2isize),
            slot,
        );
        /* pop value and key */
        (*L).top = (*L).top.offset(-2isize)
    }
    let ref mut fresh104 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh104 -= 1;
    if *fresh104 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      784i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void lua_seti(lua_State *, int, lua_Integer)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawset(mut L: *mut lua_State_0, mut idx: libc::c_int) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let mut slot: *mut TValue = 0 as *mut TValue;
    let ref mut fresh105 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh106 = *fresh105;
    *fresh105 = *fresh105 + 1;
    if fresh106 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      791i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void lua_rawset(lua_State *, int)\x00")).as_ptr());
    };
    if (2i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((2) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            792i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void lua_rawset(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    o = index2addr(L, idx);
    if (*o).tt_ == 5i32 | 1i32 << 6i32
        && !(b"table expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((o))->tt_) == (((5) | (1 << 6))))) && \"table expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            794i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void lua_rawset(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if (*o).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            795i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void lua_rawset(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            795i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void lua_rawset(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    slot = luaH_set(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-2isize) as *const TValue,
    );
    *slot = *(*L).top.offset(-1isize);
    if 0 == (*slot).tt_ & 1i32 << 6i32 || {
        if 0 != (*slot).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((slot))->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                796i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_rawset(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        (*slot).tt_ & 0x3fi32 == (*(*slot).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*slot).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((slot))->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    796i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                        b"void lua_rawset(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            0 != ((*(*slot).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                796i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_rawset(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    };
    if (*o).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            797i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void lua_rawset(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            797i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void lua_rawset(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    (*&mut (*((*o).value_.gc as *mut GCUnion)).h).flags = 0i32 as lu_byte;
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 && {
        if (*o).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                798i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_rawset(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                798i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_rawset(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(&mut (*((*o).value_.gc as *mut GCUnion)).h as *mut Table)).marked as libc::c_int
            & 1i32 << 2i32
    } && {
        if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((L->top-1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                798i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_rawset(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    } {
        if (*o).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                798i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_rawset(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                798i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_rawset(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-2isize);
    let ref mut fresh107 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh107 -= 1;
    if *fresh107 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      800i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void lua_rawset(lua_State *, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawseti(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let ref mut fresh108 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh109 = *fresh108;
    *fresh108 = *fresh108 + 1;
    if fresh109 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      806i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void lua_rawseti(lua_State *, int, lua_Integer)\x00")).as_ptr());
    };
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            807i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    o = index2addr(L, idx);
    if (*o).tt_ == 5i32 | 1i32 << 6i32
        && !(b"table expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((o))->tt_) == (((5) | (1 << 6))))) && \"table expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            809i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    if (*o).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            810i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            810i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
            )).as_ptr(),
        );
    };
    luaH_setint(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        n,
        (*L).top.offset(-1isize),
    );
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 && {
        if (*o).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                811i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                811i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(&mut (*((*o).value_.gc as *mut GCUnion)).h as *mut Table)).marked as libc::c_int
            & 1i32 << 2i32
    } && {
        if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((L->top-1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                811i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    } {
        if (*o).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                811i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                811i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void lua_rawseti(lua_State *, int, lua_Integer)\x00",
                )).as_ptr(),
            );
        };
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
    let ref mut fresh110 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh110 -= 1;
    if *fresh110 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      813i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void lua_rawseti(lua_State *, int, lua_Integer)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawsetp(
    mut L: *mut lua_State_0,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let mut k: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut slot: *mut TValue = 0 as *mut TValue;
    let ref mut fresh111 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh112 = *fresh111;
    *fresh111 = *fresh111 + 1;
    if fresh112 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      820i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void lua_rawsetp(lua_State *, int, const void *)\x00")).as_ptr());
    };
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            821i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"void lua_rawsetp(lua_State *, int, const void *)\x00",
            )).as_ptr(),
        );
    };
    o = index2addr(L, idx);
    if (*o).tt_ == 5i32 | 1i32 << 6i32
        && !(b"table expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((o))->tt_) == (((5) | (1 << 6))))) && \"table expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            823i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"void lua_rawsetp(lua_State *, int, const void *)\x00",
            )).as_ptr(),
        );
    };
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut libc::c_void;
    (*io).tt_ = 2i32;
    if (*o).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            825i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"void lua_rawsetp(lua_State *, int, const void *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            825i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"void lua_rawsetp(lua_State *, int, const void *)\x00",
            )).as_ptr(),
        );
    };
    slot = luaH_set(L, &mut (*((*o).value_.gc as *mut GCUnion)).h, &mut k);
    *slot = *(*L).top.offset(-1isize);
    if 0 == (*slot).tt_ & 1i32 << 6i32 || {
        if 0 != (*slot).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((slot))->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                826i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"void lua_rawsetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
        (*slot).tt_ & 0x3fi32 == (*(*slot).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*slot).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((slot))->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    826i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                        b"void lua_rawsetp(lua_State *, int, const void *)\x00",
                    )).as_ptr(),
                );
            };
            0 != ((*(*slot).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                826i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"void lua_rawsetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
    };
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 && {
        if (*o).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                827i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"void lua_rawsetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                827i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"void lua_rawsetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(&mut (*((*o).value_.gc as *mut GCUnion)).h as *mut Table)).marked as libc::c_int
            & 1i32 << 2i32
    } && {
        if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((L->top - 1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                827i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"void lua_rawsetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    } {
        if (*o).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                827i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"void lua_rawsetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                827i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                    b"void lua_rawsetp(lua_State *, int, const void *)\x00",
                )).as_ptr(),
            );
        };
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
    let ref mut fresh113 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh113 -= 1;
    if *fresh113 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      829i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void lua_rawsetp(lua_State *, int, const void *)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setmetatable(
    mut L: *mut lua_State_0,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj: *mut TValue = 0 as *mut TValue;
    let mut mt: *mut Table_0 = 0 as *mut Table_0;
    let ref mut fresh114 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh115 = *fresh114;
    *fresh114 = *fresh114 + 1;
    if fresh115 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      836i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"int lua_setmetatable(lua_State *, int)\x00")).as_ptr());
    };
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            837i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"int lua_setmetatable(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    obj = index2addr(L, objindex);
    if (*(*L).top.offset(-1isize)).tt_ == 0i32 {
        mt = 0 as *mut Table_0
    } else {
        if (*(*L).top.offset(-1isize)).tt_ == 5i32 | 1i32 << 6i32
            && !(b"table expected\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(((((L->top - 1))->tt_) == (((5) | (1 << 6))))) && \"table expected\"\x00"
                    as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                842i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"int lua_setmetatable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*L).top.offset(-1isize)).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((L->top - 1))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                843i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"int lua_setmetatable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*(*L).top.offset(-1isize)).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((L->top - 1)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                843i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"int lua_setmetatable(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        mt = &mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion)).h
    }
    match (*obj).tt_ & 0xfi32 {
        5 => {
            if (*obj).tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((obj))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    847i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_setmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*obj).value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(((obj)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    847i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_setmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            let ref mut fresh116 = (*&mut (*((*obj).value_.gc as *mut GCUnion)).h).metatable;
            *fresh116 = mt;
            if !mt.is_null() {
                if 0 != (*obj).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((obj)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        849i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                            b"int lua_setmetatable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                if 0 != (*(*obj).value_.gc).marked as libc::c_int & 1i32 << 2i32
                    && 0 != (*mt).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                {
                    if 0 != (*obj).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((obj)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            849i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                                b"int lua_setmetatable(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*obj).value_.gc).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                    } else {
                        __assert_fail(b"(((((((((obj)->tt_) & (1 << 6))) ? (void) (0) : __assert_fail (\"(((obj)->tt_) & (1 << 6))\", \"lapi.c\", 849, __extension__ __PRETTY_FUNCTION__)), (((obj)->value_).gc)))->tt) & 0x0F) < (9+1)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"lapi.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      849i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 39],
                                                                &[libc::c_char; 39]>(b"int lua_setmetatable(lua_State *, int)\x00")).as_ptr());
                    };
                    if 0 != (*obj).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((obj)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            849i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                                b"int lua_setmetatable(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*mt).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                    } else {
                        __assert_fail(
                            b"(((mt)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            849i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                                b"int lua_setmetatable(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    luaC_barrier_(
                        L,
                        &mut (*((*obj).value_.gc as *mut GCUnion)).gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {
                };
                if 0 != (*obj).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((obj)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        850i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                            b"int lua_setmetatable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        7 => {
            if (*obj).tt_ == 7i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((obj))->tt_) == (((7) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    855i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_setmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*obj).value_.gc).tt as libc::c_int == 7i32 {
            } else {
                __assert_fail(
                    b"(((obj)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    855i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"int lua_setmetatable(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            let ref mut fresh117 = (*&mut (*((*obj).value_.gc as *mut GCUnion)).u).metatable;
            *fresh117 = mt;
            if !mt.is_null() {
                if (*obj).tt_ == 7i32 | 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"((((obj))->tt_) == (((7) | (1 << 6))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        857i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                            b"int lua_setmetatable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*obj).value_.gc).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(
                        b"(((obj)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        857i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                            b"int lua_setmetatable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                if 0 != (*(&mut (*((*obj).value_.gc as *mut GCUnion)).u as *mut Udata)).marked
                    as libc::c_int & 1i32 << 2i32
                    && 0 != (*mt).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                {
                    if (*obj).tt_ == 7i32 | 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"((((obj))->tt_) == (((7) | (1 << 6))))\x00" as *const u8
                                as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            857i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                                b"int lua_setmetatable(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*obj).value_.gc).tt as libc::c_int == 7i32 {
                    } else {
                        __assert_fail(
                            b"(((obj)->value_).gc)->tt == 7\x00" as *const u8
                                as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            857i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                                b"int lua_setmetatable(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(&mut (*((*obj).value_.gc as *mut GCUnion)).u as *mut Udata)).tt
                        as libc::c_int & 0xfi32 < 9i32 + 1i32
                    {
                    } else {
                        __assert_fail(b"((((((((((obj))->tt_) == (((7) | (1 << 6))))) ? (void) (0) : __assert_fail (\"((((obj))->tt_) == (((7) | (1 << 6))))\", \"lapi.c\", 857, __extension__ __PRETTY_FUNCTION__)), (((((((obj)->value_).gc)->tt == 7) ? (void) (0) : __assert_fail (\"(((obj)->value_).gc)->tt == 7\", \"lapi.c\", 857, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((obj)->value_).gc))))->u))))))->tt) & 0x0F) < (9+1)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"lapi.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      857i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 39],
                                                                &[libc::c_char; 39]>(b"int lua_setmetatable(lua_State *, int)\x00")).as_ptr());
                    };
                    if (*obj).tt_ == 7i32 | 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"((((obj))->tt_) == (((7) | (1 << 6))))\x00" as *const u8
                                as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            857i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                                b"int lua_setmetatable(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*(*obj).value_.gc).tt as libc::c_int == 7i32 {
                    } else {
                        __assert_fail(
                            b"(((obj)->value_).gc)->tt == 7\x00" as *const u8
                                as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            857i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                                b"int lua_setmetatable(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if (*mt).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                    } else {
                        __assert_fail(
                            b"(((mt)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                            b"lapi.c\x00" as *const u8 as *const libc::c_char,
                            857i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                                b"int lua_setmetatable(lua_State *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    luaC_barrier_(
                        L,
                        &mut (*(&mut (*((*obj).value_.gc as *mut GCUnion)).u as *mut Udata
                            as *mut GCUnion))
                            .gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {
                };
                if 0 != (*obj).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((obj)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        858i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                            b"int lua_setmetatable(lua_State *, int)\x00",
                        )).as_ptr(),
                    );
                };
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        _ => (*(*L).l_G).mt[((*obj).tt_ & 0xfi32) as usize] = mt,
    }
    (*L).top = (*L).top.offset(-1isize);
    let ref mut fresh118 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh118 -= 1;
    if *fresh118 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      868i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"int lua_setmetatable(lua_State *, int)\x00")).as_ptr());
    };
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setuservalue(mut L: *mut lua_State_0, mut idx: libc::c_int) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let ref mut fresh119 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh120 = *fresh119;
    *fresh119 = *fresh119 + 1;
    if fresh120 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      875i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void lua_setuservalue(lua_State *, int)\x00")).as_ptr());
    };
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            876i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void lua_setuservalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    o = index2addr(L, idx);
    if (*o).tt_ == 7i32 | 1i32 << 6i32
        && !(b"full userdata expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((o))->tt_) == (((7) | (1 << 6))))) && \"full userdata expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            878i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void lua_setuservalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let mut io: *const TValue = (*L).top.offset(-1isize) as *const TValue;
    if (*o).tt_ == 7i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((o))->tt_) == (((7) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            879i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void lua_setuservalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if (*(*o).value_.gc).tt as libc::c_int == 7i32 {
    } else {
        __assert_fail(
            b"(((o)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            879i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void lua_setuservalue(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let mut iu: *mut Udata_0 = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*iu).user_ = (*io).value_;
    (*iu).ttuv_ = (*io).tt_ as lu_byte;
    if 0 == (*io).tt_ & 1i32 << 6i32 || {
        if 0 != (*io).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                879i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void lua_setuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    879i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                        b"void lua_setuservalue(lua_State *, int)\x00",
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
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                879i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void lua_setuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    };
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 && {
        if 0 != (*o).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                880i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void lua_setuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*o).value_.gc).marked as libc::c_int & 1i32 << 2i32
    } && {
        if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((L->top - 1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                880i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void lua_setuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    } {
        if 0 != (*o).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                880i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void lua_setuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((((((((o)->tt_) & (1 << 6))) ? (void) (0) : __assert_fail (\"(((o)->tt_) & (1 << 6))\", \"lapi.c\", 880, __extension__ __PRETTY_FUNCTION__)), (((o)->value_).gc)))->tt) & 0x0F) < (9+1)\x00"
                              as *const u8 as *const libc::c_char,
                          b"lapi.c\x00" as *const u8 as *const libc::c_char,
                          880i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_char; 40]>(b"void lua_setuservalue(lua_State *, int)\x00")).as_ptr());
        };
        if 0 != (*o).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                880i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void lua_setuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((L->top - 1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                880i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void lua_setuservalue(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        luaC_barrier_(
            L,
            &mut (*((*o).value_.gc as *mut GCUnion)).gc,
            (*(*L).top.offset(-1isize)).value_.gc,
        );
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
    let ref mut fresh121 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh121 -= 1;
    if *fresh121 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      882i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void lua_setuservalue(lua_State *, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_callk(
    mut L: *mut lua_State_0,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> () {
    let mut func: StkId = 0 as *mut TValue;
    let ref mut fresh122 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh123 = *fresh122;
    *fresh122 = *fresh122 + 1;
    if fresh123 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      899i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"void lua_callk(lua_State *, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    if (k.is_none() || 0 == (*(*L).ci).callstatus as libc::c_int & 1i32 << 1i32)
        && !(b"cannot use continuations inside hooks\x00" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(b"(k == ((void*)0) || !((L->ci)->callstatus & (1<<1))) && \"cannot use continuations inside hooks\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      901i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"void lua_callk(lua_State *, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    if ((nargs + 1i32) as libc::c_long)
        < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((nargs+1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            902i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                b"void lua_callk(lua_State *, int, int, lua_KContext, lua_KFunction)\x00",
            )).as_ptr(),
        );
    };
    if (*L).status as libc::c_int == 0i32
        && !(b"cannot do calls on non-normal thread\x00" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"(L->status == 0) && \"cannot do calls on non-normal thread\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            903i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                b"void lua_callk(lua_State *, int, int, lua_KContext, lua_KFunction)\x00",
            )).as_ptr(),
        );
    };
    if (nresults == -1i32
        || (*(*L).ci).top.wrapping_offset_from((*L).top) as libc::c_long
            >= (nresults - nargs) as libc::c_long)
        && !(b"results from function overflow current stack size\x00" as *const u8
            as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(b"((nresults) == (-1) || (L->ci->top - L->top >= (nresults) - (nargs))) && \"results from function overflow current stack size\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      904i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"void lua_callk(lua_State *, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    func = (*L).top.offset(-((nargs + 1i32) as isize));
    if k.is_some() && (*L).nny as libc::c_int == 0i32 {
        /* need to prepare continuation? */
        /* save continuation */
        (*(*L).ci).u.c.k = k;
        /* save context */
        (*(*L).ci).u.c.ctx = ctx;
        /* do the call */
        luaD_call(L, func, nresults);
    } else {
        /* no continuation or no yieldable */
        /* just do the call */
        luaD_callnoyield(L, func, nresults);
    }
    if nresults == -1i32 && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top
    }
    let ref mut fresh124 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh124 -= 1;
    if *fresh124 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      914i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"void lua_callk(lua_State *, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pcallk(
    mut L: *mut lua_State_0,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut errfunc: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> libc::c_int {
    let mut o: StkId = 0 as *mut TValue;
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    let mut c: CallS = CallS {
        func: 0 as *mut TValue,
        nresults: 0,
    };
    let mut status: libc::c_int = 0;
    let mut func: ptrdiff_t = 0;
    let ref mut fresh125 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh126 = *fresh125;
    *fresh125 = *fresh125 + 1;
    if fresh126 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      940i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"int lua_pcallk(lua_State *, int, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    if (k.is_none() || 0 == (*(*L).ci).callstatus as libc::c_int & 1i32 << 1i32)
        && !(b"cannot use continuations inside hooks\x00" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(b"(k == ((void*)0) || !((L->ci)->callstatus & (1<<1))) && \"cannot use continuations inside hooks\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      942i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"int lua_pcallk(lua_State *, int, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    if ((nargs + 1i32) as libc::c_long)
        < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((nargs+1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            943i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 72], &[libc::c_char; 72]>(
                b"int lua_pcallk(lua_State *, int, int, int, lua_KContext, lua_KFunction)\x00",
            )).as_ptr(),
        );
    };
    if (*L).status as libc::c_int == 0i32
        && !(b"cannot do calls on non-normal thread\x00" as *const u8 as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(
            b"(L->status == 0) && \"cannot do calls on non-normal thread\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            944i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 72], &[libc::c_char; 72]>(
                b"int lua_pcallk(lua_State *, int, int, int, lua_KContext, lua_KFunction)\x00",
            )).as_ptr(),
        );
    };
    if (nresults == -1i32
        || (*(*L).ci).top.wrapping_offset_from((*L).top) as libc::c_long
            >= (nresults - nargs) as libc::c_long)
        && !(b"results from function overflow current stack size\x00" as *const u8
            as *const libc::c_char)
            .is_null()
    {
    } else {
        __assert_fail(b"((nresults) == (-1) || (L->ci->top - L->top >= (nresults) - (nargs))) && \"results from function overflow current stack size\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      945i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"int lua_pcallk(lua_State *, int, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    if errfunc == 0i32 {
        func = 0i32 as ptrdiff_t
    } else {
        o = index2addr(L, errfunc);
        if o != &luaO_nilobject_ as *const TValue as StkId
            && !(errfunc <= -50000i32 - 1000i32)
            && !(b"index not in the stack\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(b"((((o) != (&luaO_nilobject_)) && !((errfunc) <= (-50000 - 1000)))) && \"index not in the stack\"\x00"
                              as *const u8 as *const libc::c_char,
                          b"lapi.c\x00" as *const u8 as *const libc::c_char,
                          950i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 72],
                                                    &[libc::c_char; 72]>(b"int lua_pcallk(lua_State *, int, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
        };
        func = (o as *mut libc::c_char).wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long
    }
    /* function to be called */
    c.func = (*L).top.offset(-((nargs + 1i32) as isize));
    if k.is_none() || (*L).nny as libc::c_int > 0i32 {
        /* no continuation or no yieldable? */
        /* do a 'conventional' protected call */
        c.nresults = nresults;
        status = luaD_pcall(
            L,
            Some(f_call),
            &mut c as *mut CallS as *mut libc::c_void,
            (c.func as *mut libc::c_char).wrapping_offset_from((*L).stack as *mut libc::c_char)
                as libc::c_long,
            func,
        )
    } else {
        /* prepare continuation (call is already protected by 'resume') */
        ci = (*L).ci;
        /* save continuation */
        (*ci).u.c.k = k;
        /* save context */
        (*ci).u.c.ctx = ctx;
        /* save information for error recovery */
        (*ci).extra = (c.func as *mut libc::c_char)
            .wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long;
        (*ci).u.c.old_errfunc = (*L).errfunc;
        (*L).errfunc = func;
        (*ci).callstatus = ((*ci).callstatus as libc::c_int & !(1i32 << 0i32)
            | (*L).allowhook as libc::c_int) as libc::c_ushort;
        /* save value of 'allowhook' */
        /* function can do error recovery */
        (*ci).callstatus = ((*ci).callstatus as libc::c_int | 1i32 << 4i32) as libc::c_ushort;
        /* do the call */
        luaD_call(L, c.func, nresults);
        (*ci).callstatus = ((*ci).callstatus as libc::c_int & !(1i32 << 4i32)) as libc::c_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc;
        /* if it is here, there were no errors */
        status = 0i32
    }
    if nresults == -1i32 && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top
    }
    let ref mut fresh127 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh127 -= 1;
    if *fresh127 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      974i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"int lua_pcallk(lua_State *, int, int, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    return status;
}
unsafe extern "C" fn f_call(mut L: *mut lua_State_0, mut ud: *mut libc::c_void) -> () {
    let mut c: *mut CallS = ud as *mut CallS;
    luaD_callnoyield(L, (*c).func, (*c).nresults);
}
#[no_mangle]
pub unsafe extern "C" fn lua_load(
    mut L: *mut lua_State_0,
    mut reader: lua_Reader,
    mut data: *mut libc::c_void,
    mut chunkname: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut z: ZIO = Zio {
        n: 0,
        p: 0 as *const libc::c_char,
        reader: None,
        data: 0 as *mut libc::c_void,
        L: 0 as *mut lua_State_0,
    };
    let mut status: libc::c_int = 0;
    let ref mut fresh128 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh129 = *fresh128;
    *fresh128 = *fresh128 + 1;
    if fresh129 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      983i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
    };
    if chunkname.is_null() {
        chunkname = b"?\x00" as *const u8 as *const libc::c_char
    }
    luaZ_init(L, &mut z, reader, data);
    status = luaD_protectedparser(L, &mut z, chunkname, mode);
    if status == 0i32 {
        /* no errors? */
        /* get newly created function */
        if (*(*L).top.offset(-1isize)).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(b"((((L->top - 1))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                              as *const u8 as *const libc::c_char,
                          b"lapi.c\x00" as *const u8 as *const libc::c_char,
                          988i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 74],
                                                    &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
        };
        if (*(*(*L).top.offset(-1isize)).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(b"(((L->top - 1)->value_).gc)->tt == (6 | (0 << 4))\x00"
                              as *const u8 as *const libc::c_char,
                          b"lapi.c\x00" as *const u8 as *const libc::c_char,
                          988i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 74],
                                                    &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
        };
        let mut f: *mut LClosure = &mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion))
            .cl
            .l;
        if (*f).nupvalues as libc::c_int >= 1i32 {
            /* does it have an upvalue? */
            /* get global table from registry */
            if (*(*L).l_G).l_registry.tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((&(L->l_G)->l_registry))->tt_) == (((5) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char, 991i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 74],
                                                        &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
            };
            if (*(*(*L).l_G).l_registry.value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(b"(((&(L->l_G)->l_registry)->value_).gc)->tt == 5\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char, 991i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 74],
                                                        &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
            };
            let mut reg: *mut Table_0 =
                &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
            let mut gt: *const TValue = luaH_getint(reg, 2i32 as lua_Integer);
            let mut io1: *mut TValue = (**(*f).upvals.as_mut_ptr().offset(0isize)).v;
            *io1 = *gt;
            if 0 == (*io1).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"lapi.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         994i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 74],
                                                                   &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
                };
                (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"lapi.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      994i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 74],
                                                                                &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
                    };
                    0 != ((*(*io1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                        & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lapi.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  994i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 74],
                                                            &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
                };
            };
            /* set global table as 1st upvalue of 'f' (may be LUA_ENV) */
            if 0 != (*(**(*f).upvals.as_mut_ptr().offset(0isize)).v).tt_ & 1i32 << 6i32
                && !((**(*f).upvals.as_mut_ptr().offset(0isize)).v
                    != &mut (**(*f).upvals.as_mut_ptr().offset(0isize)).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, *(*f).upvals.as_mut_ptr().offset(0isize));
            } else {
            };
        }
    }
    let ref mut fresh130 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh130 -= 1;
    if *fresh130 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      998i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"int lua_load(lua_State *, lua_Reader, void *, const char *, const char *)\x00")).as_ptr());
    };
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_dump(
    mut L: *mut lua_State_0,
    mut writer: lua_Writer,
    mut data: *mut libc::c_void,
    mut strip: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut o: *mut TValue = 0 as *mut TValue;
    let ref mut fresh131 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh132 = *fresh131;
    *fresh131 = *fresh131 + 1;
    if fresh132 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1006i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"int lua_dump(lua_State *, lua_Writer, void *, int)\x00")).as_ptr());
    };
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1007i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"int lua_dump(lua_State *, lua_Writer, void *, int)\x00",
            )).as_ptr(),
        );
    };
    o = (*L).top.offset(-1isize);
    if (*o).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
        if (*o).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                1010i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"int lua_dump(lua_State *, lua_Writer, void *, int)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                1010i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"int lua_dump(lua_State *, lua_Writer, void *, int)\x00",
                )).as_ptr(),
            );
        };
        status = luaU_dump(
            L,
            (*&mut (*((*o).value_.gc as *mut GCUnion)).cl.l).p,
            writer,
            data,
            strip,
        )
    } else {
        status = 1i32
    }
    let ref mut fresh133 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh133 -= 1;
    if *fresh133 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1013i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"int lua_dump(lua_State *, lua_Writer, void *, int)\x00")).as_ptr());
    };
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_status(mut L: *mut lua_State_0) -> libc::c_int {
    return (*L).status as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gc(
    mut L: *mut lua_State_0,
    mut what: libc::c_int,
    mut data: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0i32;
    let mut g: *mut global_State = 0 as *mut global_State;
    let ref mut fresh134 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh135 = *fresh134;
    *fresh134 = *fresh134 + 1;
    if fresh135 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1030i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"int lua_gc(lua_State *, int, int)\x00")).as_ptr());
    };
    g = (*L).l_G;
    match what {
        0 => (*g).gcrunning = 0i32 as lu_byte,
        1 => {
            luaE_setdebt(g, 0i32 as l_mem);
            (*g).gcrunning = 1i32 as lu_byte
        }
        2 => {
            luaC_fullgc(L, 0i32);
        }
        3 => {
            /* GC values are expressed in Kbytes: #bytes/2^10 */
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem >> 10i32) as libc::c_int
        }
        4 => {
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem & 0x3ffi32 as libc::c_ulong)
                as libc::c_int
        }
        5 => {
            /* =1 to signal that it did an actual step */
            let mut debt: l_mem = 1i32 as l_mem;
            let mut oldrunning: lu_byte = (*g).gcrunning;
            /* allow GC to run */
            (*g).gcrunning = 1i32 as lu_byte;
            if data == 0i32 {
                /* to do a "small" step */
                luaE_setdebt(
                    g,
                    -((100i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<TString>() as libc::c_ulong)
                        as libc::c_int) as l_mem,
                );
                luaC_step(L);
            } else {
                /* add 'data' to total debt */
                debt = data as l_mem * 1024i32 as libc::c_long + (*g).GCdebt;
                luaE_setdebt(g, debt);
                if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                    luaC_step(L);
                }
            }
            /* restore previous state */
            (*g).gcrunning = oldrunning;
            /* end of cycle? */
            if debt > 0i32 as libc::c_long && (*g).gcstate as libc::c_int == 7i32 {
                /* signal it */
                res = 1i32
            }
        }
        6 => {
            res = (*g).gcpause;
            (*g).gcpause = data
        }
        7 => {
            res = (*g).gcstepmul;
            if data < 40i32 {
                /* avoid ridiculous low values (and 0) */
                data = 40i32
            }
            (*g).gcstepmul = data
        }
        9 => res = (*g).gcrunning as libc::c_int,
        _ => {
            /* invalid option */
            res = -1i32
        }
    }
    let ref mut fresh136 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh136 -= 1;
    if *fresh136 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1090i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"int lua_gc(lua_State *, int, int)\x00")).as_ptr());
    };
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_error(mut L: *mut lua_State_0) -> libc::c_int {
    let ref mut fresh137 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh138 = *fresh137;
    *fresh137 = *fresh137 + 1;
    if fresh138 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1102i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"int lua_error(lua_State *)\x00")).as_ptr());
    };
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1103i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"int lua_error(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    luaG_errormsg(L);
}
#[no_mangle]
pub unsafe extern "C" fn lua_next(mut L: *mut lua_State_0, mut idx: libc::c_int) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut more: libc::c_int = 0;
    let ref mut fresh139 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh140 = *fresh139;
    *fresh139 = *fresh139 + 1;
    if fresh140 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1113i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"int lua_next(lua_State *, int)\x00")).as_ptr());
    };
    t = index2addr(L, idx);
    if (*t).tt_ == 5i32 | 1i32 << 6i32
        && !(b"table expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((t))->tt_) == (((5) | (1 << 6))))) && \"table expected\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1115i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"int lua_next(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if (*t).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((t))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1116i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"int lua_next(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if (*(*t).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"(((t)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1116i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"int lua_next(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    more = luaH_next(
        L,
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-1isize),
    );
    if 0 != more {
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                1118i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"int lua_next(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    } else {
        (*L).top = (*L).top.offset(-1isize)
    }
    let ref mut fresh141 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh141 -= 1;
    if *fresh141 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1122i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"int lua_next(lua_State *, int)\x00")).as_ptr());
    };
    return more;
}
#[no_mangle]
pub unsafe extern "C" fn lua_concat(mut L: *mut lua_State_0, mut n: libc::c_int) -> () {
    let ref mut fresh142 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh143 = *fresh142;
    *fresh142 = *fresh142 + 1;
    if fresh143 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1128i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void lua_concat(lua_State *, int)\x00")).as_ptr());
    };
    if (n as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((n) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1129i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"void lua_concat(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    if n >= 2i32 {
        luaV_concat(L, n);
    } else if n == 0i32 {
        /* push empty string */
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = luaS_newlstr(
            L,
            b"\x00" as *const u8 as *const libc::c_char,
            0i32 as size_t,
        );
        if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                1134i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_concat(lua_State *, int)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1134i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                        b"void lua_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        1134i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                            b"void lua_concat(lua_State *, int)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1134i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                        b"void lua_concat(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                1135i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"void lua_concat(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
    }
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    /* else n == 1; nothing to do */
    let ref mut fresh144 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh144 -= 1;
    if *fresh144 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1139i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"void lua_concat(lua_State *, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_len(mut L: *mut lua_State_0, mut idx: libc::c_int) -> () {
    let mut t: StkId = 0 as *mut TValue;
    let ref mut fresh145 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh146 = *fresh145;
    *fresh145 = *fresh145 + 1;
    if fresh146 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1145i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void lua_len(lua_State *, int)\x00")).as_ptr());
    };
    t = index2addr(L, idx);
    luaV_objlen(L, (*L).top, t as *const TValue);
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1148i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"void lua_len(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let ref mut fresh147 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh147 -= 1;
    if *fresh147 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1149i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void lua_len(lua_State *, int)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_stringtonumber(
    mut L: *mut lua_State_0,
    mut s: *const libc::c_char,
) -> size_t {
    let mut sz: size_t = luaO_str2num(s, (*L).top);
    if sz != 0i32 as libc::c_ulong {
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                341i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                    b"size_t lua_stringtonumber(lua_State *, const char *)\x00",
                )).as_ptr(),
            );
        };
    }
    return sz;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getallocf(
    mut L: *mut lua_State_0,
    mut ud: *mut *mut libc::c_void,
) -> lua_Alloc {
    let mut f: lua_Alloc = None;
    let ref mut fresh148 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh149 = *fresh148;
    *fresh148 = *fresh148 + 1;
    if fresh149 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1155i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"lua_Alloc lua_getallocf(lua_State *, void **)\x00")).as_ptr());
    };
    if !ud.is_null() {
        *ud = (*(*L).l_G).ud
    }
    f = (*(*L).l_G).frealloc;
    let ref mut fresh150 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh150 -= 1;
    if *fresh150 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1158i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"lua_Alloc lua_getallocf(lua_State *, void **)\x00")).as_ptr());
    };
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setallocf(
    mut L: *mut lua_State_0,
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) -> () {
    let ref mut fresh151 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh152 = *fresh151;
    *fresh151 = *fresh151 + 1;
    if fresh152 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1164i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void lua_setallocf(lua_State *, lua_Alloc, void *)\x00")).as_ptr());
    };
    (*(*L).l_G).ud = ud;
    (*(*L).l_G).frealloc = f;
    let ref mut fresh153 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh153 -= 1;
    if *fresh153 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1167i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"void lua_setallocf(lua_State *, lua_Alloc, void *)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_getupvalue(
    mut L: *mut lua_State_0,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* to avoid warnings */
    let mut val: *mut TValue = 0 as *mut TValue;
    let ref mut fresh154 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh155 = *fresh154;
    *fresh154 = *fresh154 + 1;
    if fresh155 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1212i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"const char *lua_getupvalue(lua_State *, int, int)\x00")).as_ptr());
    };
    name = aux_upvalue(
        index2addr(L, funcindex),
        n,
        &mut val,
        0 as *mut *mut CClosure,
        0 as *mut *mut UpVal,
    );
    if !name.is_null() {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *val;
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1215i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"const char *lua_getupvalue(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        1215i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"const char *lua_getupvalue(lua_State *, int, int)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1215i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"const char *lua_getupvalue(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"lapi.c\x00" as *const u8 as *const libc::c_char,
                1216i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"const char *lua_getupvalue(lua_State *, int, int)\x00",
                )).as_ptr(),
            );
        };
    }
    let ref mut fresh156 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh156 -= 1;
    if *fresh156 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1218i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"const char *lua_getupvalue(lua_State *, int, int)\x00")).as_ptr());
    };
    return name;
}
unsafe extern "C" fn aux_upvalue(
    mut fi: StkId,
    mut n: libc::c_int,
    mut val: *mut *mut TValue,
    mut owner: *mut *mut CClosure,
    mut uv: *mut *mut UpVal,
) -> *const libc::c_char {
    match (*fi).tt_ & 0x3fi32 {
        38 => {
            /* C closure */
            if (*fi).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((fi))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1188i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 70],
                                                        &[libc::c_char; 70]>(b"const char *aux_upvalue(StkId, int, TValue **, CClosure **, UpVal **)\x00")).as_ptr());
            };
            if (*(*fi).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(b"(((fi)->value_).gc)->tt == (6 | (2 << 4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1188i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 70],
                                                        &[libc::c_char; 70]>(b"const char *aux_upvalue(StkId, int, TValue **, CClosure **, UpVal **)\x00")).as_ptr());
            };
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            if !(1i32 <= n && n <= (*f).nupvalues as libc::c_int) {
                return 0 as *const libc::c_char;
            } else {
                *val = &mut *(*f).upvalue.as_mut_ptr().offset((n - 1i32) as isize) as *mut TValue;
                if !owner.is_null() {
                    *owner = f
                }
                return b"\x00" as *const u8 as *const libc::c_char;
            }
        }
        6 => {
            /* Lua closure */
            if (*fi).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((fi))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1195i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 70],
                                                        &[libc::c_char; 70]>(b"const char *aux_upvalue(StkId, int, TValue **, CClosure **, UpVal **)\x00")).as_ptr());
            };
            if (*(*fi).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"(((fi)->value_).gc)->tt == (6 | (0 << 4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lapi.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1195i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 70],
                                                        &[libc::c_char; 70]>(b"const char *aux_upvalue(StkId, int, TValue **, CClosure **, UpVal **)\x00")).as_ptr());
            };
            let mut f_0: *mut LClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
            let mut name: *mut TString = 0 as *mut TString;
            let mut p: *mut Proto_0 = (*f_0).p;
            if !(1i32 <= n && n <= (*p).sizeupvalues) {
                return 0 as *const libc::c_char;
            } else {
                *val = (**(*f_0).upvals.as_mut_ptr().offset((n - 1i32) as isize)).v;
                if !uv.is_null() {
                    *uv = *(*f_0).upvals.as_mut_ptr().offset((n - 1i32) as isize)
                }
                name = (*(*p).upvalues.offset((n - 1i32) as isize)).name;
                return if name.is_null() {
                    b"(*no name)\x00" as *const u8 as *const libc::c_char
                } else {
                    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
                    } else {
                        __assert_fail(b"sizeof((name)->extra)\x00" as
                                                 *const u8 as
                                                 *const libc::c_char,
                                             b"lapi.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             1202i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 70],
                                                                       &[libc::c_char; 70]>(b"const char *aux_upvalue(StkId, int, TValue **, CClosure **, UpVal **)\x00")).as_ptr());
                    };
                    (name as *mut libc::c_char)
                        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                };
            }
        }
        _ => {
            /* not a closure */
            return 0 as *const libc::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setupvalue(
    mut L: *mut lua_State_0,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* to avoid warnings */
    let mut val: *mut TValue = 0 as *mut TValue;
    let mut owner: *mut CClosure = 0 as *mut CClosure;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    let mut fi: StkId = 0 as *mut TValue;
    let ref mut fresh157 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh158 = *fresh157;
    *fresh157 = *fresh157 + 1;
    if fresh158 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1229i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"const char *lua_setupvalue(lua_State *, int, int)\x00")).as_ptr());
    };
    fi = index2addr(L, funcindex);
    if (1i32 as libc::c_long) < (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long
        && !(b"not enough elements in the stack\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1231i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"const char *lua_setupvalue(lua_State *, int, int)\x00",
            )).as_ptr(),
        );
    };
    name = aux_upvalue(fi, n, &mut val, &mut owner, &mut uv);
    if !name.is_null() {
        (*L).top = (*L).top.offset(-1isize);
        let mut io1: *mut TValue = val;
        *io1 = *(*L).top;
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1235i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"const char *lua_setupvalue(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        1235i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"const char *lua_setupvalue(lua_State *, int, int)\x00",
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
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1235i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"const char *lua_setupvalue(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
        };
        if !owner.is_null() {
            if 0 != (*(*L).top).tt_ & 1i32 << 6i32
                && 0 != (*owner).marked as libc::c_int & 1i32 << 2i32 && {
                if 0 != (*(*L).top).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((L->top)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        1236i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"const char *lua_setupvalue(lua_State *, int, int)\x00",
                        )).as_ptr(),
                    );
                };
                0 != (*(*(*L).top).value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
            } {
                if (*owner).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(
                        b"(((owner)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        1236i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"const char *lua_setupvalue(lua_State *, int, int)\x00",
                        )).as_ptr(),
                    );
                };
                if 0 != (*(*L).top).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((L->top)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lapi.c\x00" as *const u8 as *const libc::c_char,
                        1236i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"const char *lua_setupvalue(lua_State *, int, int)\x00",
                        )).as_ptr(),
                    );
                };
                luaC_barrier_(L, &mut (*(owner as *mut GCUnion)).gc, (*(*L).top).value_.gc);
            } else {
            };
        } else if !uv.is_null() {
            if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 && !((*uv).v != &mut (*uv).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, uv);
            } else {
            };
        }
    }
    let ref mut fresh159 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh159 -= 1;
    if *fresh159 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lapi.c\x00" as *const u8 as *const libc::c_char,
                      1239i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 50],
                                                &[libc::c_char; 50]>(b"const char *lua_setupvalue(lua_State *, int, int)\x00")).as_ptr());
    };
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvalueid(
    mut L: *mut lua_State_0,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    let mut fi: StkId = index2addr(L, fidx);
    match (*fi).tt_ & 0x3fi32 {
        6 => {
            /* lua closure */
            return *getupvalref(L, fidx, n, 0 as *mut *mut LClosure) as *mut libc::c_void;
        }
        38 => {
            /* C closure */
            if (*fi).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((fi))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1262i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"void *lua_upvalueid(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*fi).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(((fi)->value_).gc)->tt == (6 | (2 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1262i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"void *lua_upvalueid(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            if 1i32 <= n
                && n <= (*f).nupvalues as libc::c_int
                && !(b"invalid upvalue index\x00" as *const u8 as *const libc::c_char).is_null()
            {
            } else {
                __assert_fail(
                    b"(1 <= n && n <= f->nupvalues) && \"invalid upvalue index\"\x00" as *const u8
                        as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1263i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"void *lua_upvalueid(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            return &mut *(*f).upvalue.as_mut_ptr().offset((n - 1i32) as isize) as *mut TValue
                as *mut libc::c_void;
        }
        _ => {
            if 0 != 0i32 && !(b"closure expected\x00" as *const u8 as *const libc::c_char).is_null()
            {
            } else {
                __assert_fail(
                    b"(0) && \"closure expected\"\x00" as *const u8 as *const libc::c_char,
                    b"lapi.c\x00" as *const u8 as *const libc::c_char,
                    1267i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"void *lua_upvalueid(lua_State *, int, int)\x00",
                    )).as_ptr(),
                );
            };
            return 0 as *mut libc::c_void;
        }
    };
}
unsafe extern "C" fn getupvalref(
    mut L: *mut lua_State_0,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
    mut pf: *mut *mut LClosure,
) -> *mut *mut UpVal {
    let mut f: *mut LClosure = 0 as *mut LClosure;
    let mut fi: StkId = index2addr(L, fidx);
    if (*fi).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32
        && !(b"Lua function expected\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(((((fi))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))) && \"Lua function expected\"\x00"
                as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1247i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"UpVal **getupvalref(lua_State *, int, int, LClosure **)\x00",
            )).as_ptr(),
        );
    };
    if (*fi).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((fi))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1248i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"UpVal **getupvalref(lua_State *, int, int, LClosure **)\x00",
            )).as_ptr(),
        );
    };
    if (*(*fi).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"(((fi)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1248i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"UpVal **getupvalref(lua_State *, int, int, LClosure **)\x00",
            )).as_ptr(),
        );
    };
    f = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
    if 1i32 <= n
        && n <= (*(*f).p).sizeupvalues
        && !(b"invalid upvalue index\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"((1 <= n && n <= f->p->sizeupvalues)) && \"invalid upvalue index\"\x00" as *const u8
                as *const libc::c_char,
            b"lapi.c\x00" as *const u8 as *const libc::c_char,
            1249i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"UpVal **getupvalref(lua_State *, int, int, LClosure **)\x00",
            )).as_ptr(),
        );
    };
    if !pf.is_null() {
        *pf = f
    }
    /* get its upvalue pointer */
    return &mut *(*f).upvals.as_mut_ptr().offset((n - 1i32) as isize) as *mut *mut UpVal;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvaluejoin(
    mut L: *mut lua_State_0,
    mut fidx1: libc::c_int,
    mut n1: libc::c_int,
    mut fidx2: libc::c_int,
    mut n2: libc::c_int,
) -> () {
    let mut f1: *mut LClosure = 0 as *mut LClosure;
    let mut up1: *mut *mut UpVal = getupvalref(L, fidx1, n1, &mut f1);
    let mut up2: *mut *mut UpVal = getupvalref(L, fidx2, n2, 0 as *mut *mut LClosure);
    luaC_upvdeccount(L, *up1);
    *up1 = *up2;
    (**up1).refcount = (**up1).refcount.wrapping_add(1);
    if (**up1).v != &mut (**up1).u.value as *mut TValue {
        (**up1).u.open.touched = 1i32
    }
    if 0 != (*(**up1).v).tt_ & 1i32 << 6i32 && !((**up1).v != &mut (**up1).u.value as *mut TValue) {
        luaC_upvalbarrier_(L, *up1);
    } else {
    };
}
