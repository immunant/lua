#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
#![feature(const_slice_as_ptr, extern_types, libc, ptr_wrapping_offset_from)]
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
    fn luaS_new(L: *mut lua_State_0, str: *const libc::c_char) -> *mut TString;
    #[no_mangle]
    fn luaH_getshortstr(t: *mut Table_0, key: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaC_fix(L: *mut lua_State_0, o: *mut GCObject) -> ();
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State_0, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_call(L: *mut lua_State_0, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaG_opinterror(
        L: *mut lua_State_0,
        p1: *const TValue,
        p2: *const TValue,
        msg: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn luaG_tointerror(L: *mut lua_State_0, p1: *const TValue, p2: *const TValue) -> !;
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    #[no_mangle]
    fn luaG_concaterror(L: *mut lua_State_0, p1: *const TValue, p2: *const TValue) -> !;
}
pub type size_t = libc::c_ulong;
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
pub const TM_INDEX: TMS = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    c: CClosure_0,
    l: LClosure_0,
}
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
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table,
    pub len: size_t,
    pub user_: Value_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub const TM_CALL: TMS = 23;
pub const TM_LE: TMS = 21;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
pub const TM_BOR: TMS = 14;
pub const TM_SHR: TMS = 17;
pub const TM_MOD: TMS = 9;
pub const TM_LEN: TMS = 4;
pub const TM_BAND: TMS = 13;
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
pub type TMS = libc::c_uint;
pub const TM_MUL: TMS = 8;
pub type Table_0 = Table;
pub const TM_UNM: TMS = 18;
/*
** Ensures that address after this type is always fully aligned.
*/
pub type UTString = UTString_0;
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
pub const TM_BNOT: TMS = 19;
pub const TM_SUB: TMS = 7;
pub const TM_DIV: TMS = 11;
pub const TM_SHL: TMS = 16;
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
pub type LocVar_0 = LocVar;
/*
** Closures
*/
pub type CClosure_0 = CClosure;
/* last tag method with fast access */
pub const TM_EQ: TMS = 5;
pub const TM_NEWINDEX: TMS = 1;
pub type LClosure_0 = LClosure;
pub const TM_MODE: TMS = 3;
pub const TM_CONCAT: TMS = 22;
pub const TM_ADD: TMS = 6;
pub const TM_POW: TMS = 10;
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
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
pub type Upvaldesc_0 = Upvaldesc;
pub const TM_GC: TMS = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString_0 {
    dummy: L_Umaxalign,
    tsv: TString,
}
pub const TM_IDIV: TMS = 12;
pub const TM_LT: TMS = 20;
/* number of elements in the enum */
pub const TM_N: TMS = 24;
pub const TM_BXOR: TMS = 15;
#[no_mangle]
pub static mut luaT_typenames_: [*const libc::c_char; 11] = unsafe {
    [
        b"no value\x00" as *const u8 as *const libc::c_char,
        b"nil\x00" as *const u8 as *const libc::c_char,
        b"boolean\x00" as *const u8 as *const libc::c_char,
        udatatypename.as_ptr(),
        b"number\x00" as *const u8 as *const libc::c_char,
        b"string\x00" as *const u8 as *const libc::c_char,
        b"table\x00" as *const u8 as *const libc::c_char,
        b"function\x00" as *const u8 as *const libc::c_char,
        udatatypename.as_ptr(),
        b"thread\x00" as *const u8 as *const libc::c_char,
        b"proto\x00" as *const u8 as *const libc::c_char,
    ]
};
/*
** $Id: ltm.c,v 2.37 2016/02/26 19:20:15 roberto Exp roberto $
** Tag methods
** See Copyright Notice in lua.h
*/
static mut udatatypename: [libc::c_char; 9] = unsafe { [117, 115, 101, 114, 100, 97, 116, 97, 0] };
#[no_mangle]
pub unsafe extern "C" fn luaT_objtypename(
    mut L: *mut lua_State_0,
    mut o: *const TValue,
) -> *const libc::c_char {
    let mut mt: *mut Table_0 = 0 as *mut Table_0;
    if (*o).tt_ == 5i32 | 1i32 << 6i32 && {
        if (*o).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"ltm.c\x00" as *const u8 as *const libc::c_char,
                92i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"const char *luaT_objtypename(lua_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"ltm.c\x00" as *const u8 as *const libc::c_char,
                92i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"const char *luaT_objtypename(lua_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        mt = (*&mut (*((*o).value_.gc as *mut GCUnion)).h).metatable;
        !mt.is_null()
    } || (*o).tt_ == 7i32 | 1i32 << 6i32 && {
        if (*o).tt_ == 7i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((o))->tt_) == (((7) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                b"ltm.c\x00" as *const u8 as *const libc::c_char,
                93i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"const char *luaT_objtypename(lua_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*o).value_.gc).tt as libc::c_int == 7i32 {
        } else {
            __assert_fail(
                b"(((o)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                b"ltm.c\x00" as *const u8 as *const libc::c_char,
                93i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                    b"const char *luaT_objtypename(lua_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        mt = (*&mut (*((*o).value_.gc as *mut GCUnion)).u).metatable;
        !mt.is_null()
    } {
        let mut name: *const TValue = luaH_getshortstr(
            mt,
            luaS_new(L, b"__name\x00" as *const u8 as *const libc::c_char),
        );
        /* is '__name' a string? */
        if (*name).tt_ & 0xfi32 == 4i32 {
            /* use it as type name */
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(b"sizeof((((((((((((name))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((name))->tt_)) & 0x0F)) == (4))\", \"ltm.c\", 96, __extension__ __PRETTY_FUNCTION__)), (((((((((name)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((name)->value_).gc)->tt) & 0x0F) == 4\", \"ltm.c\", 96, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((name)->value_).gc))))->ts))))))->extra)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ltm.c\x00" as *const u8 as
                                  *const libc::c_char, 96i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 58],
                                                        &[libc::c_char; 58]>(b"const char *luaT_objtypename(lua_State *, const TValue *)\x00")).as_ptr());
            };
            if (*name).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((name))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                        as *const libc::c_char,
                    b"ltm.c\x00" as *const u8 as *const libc::c_char,
                    96i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                        b"const char *luaT_objtypename(lua_State *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*name).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((name)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"ltm.c\x00" as *const u8 as *const libc::c_char,
                    96i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 58], &[libc::c_char; 58]>(
                        b"const char *luaT_objtypename(lua_State *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            return (&mut (*((*name).value_.gc as *mut GCUnion)).ts as *mut TString_0
                as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
        }
    }
    /* else use standard type name */
    return luaT_typenames_[(((*o).tt_ & 0xfi32) + 1i32) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn luaT_gettm(
    mut events: *mut Table_0,
    mut event: TMS,
    mut ename: *mut TString,
) -> *const TValue {
    let mut tm: *const TValue = luaH_getshortstr(events, ename);
    if event as libc::c_uint <= TM_EQ as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"event <= TM_EQ\x00" as *const u8 as *const libc::c_char,
            b"ltm.c\x00" as *const u8 as *const libc::c_char,
            61i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"const TValue *luaT_gettm(Table *, TMS, TString *)\x00",
            )).as_ptr(),
        );
    };
    if (*tm).tt_ == 0i32 {
        /* no tag method? */
        /* cache this fact */
        (*events).flags = ((*events).flags as libc::c_int
            | (1u32 << event as libc::c_uint) as lu_byte as libc::c_int)
            as lu_byte;
        return 0 as *const TValue;
    } else {
        return tm;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_gettmbyobj(
    mut L: *mut lua_State_0,
    mut o: *const TValue,
    mut event: TMS,
) -> *const TValue {
    let mut mt: *mut Table_0 = 0 as *mut Table_0;
    match (*o).tt_ & 0xfi32 {
        5 => {
            if (*o).tt_ == 5i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (((5) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                    b"ltm.c\x00" as *const u8 as *const libc::c_char,
                    74i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                        b"const TValue *luaT_gettmbyobj(lua_State *, const TValue *, TMS)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == 5\x00" as *const u8 as *const libc::c_char,
                    b"ltm.c\x00" as *const u8 as *const libc::c_char,
                    74i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                        b"const TValue *luaT_gettmbyobj(lua_State *, const TValue *, TMS)\x00",
                    )).as_ptr(),
                );
            };
            mt = (*&mut (*((*o).value_.gc as *mut GCUnion)).h).metatable
        }
        7 => {
            if (*o).tt_ == 7i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((o))->tt_) == (((7) | (1 << 6))))\x00" as *const u8 as *const libc::c_char,
                    b"ltm.c\x00" as *const u8 as *const libc::c_char,
                    77i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                        b"const TValue *luaT_gettmbyobj(lua_State *, const TValue *, TMS)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*o).value_.gc).tt as libc::c_int == 7i32 {
            } else {
                __assert_fail(
                    b"(((o)->value_).gc)->tt == 7\x00" as *const u8 as *const libc::c_char,
                    b"ltm.c\x00" as *const u8 as *const libc::c_char,
                    77i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                        b"const TValue *luaT_gettmbyobj(lua_State *, const TValue *, TMS)\x00",
                    )).as_ptr(),
                );
            };
            mt = (*&mut (*((*o).value_.gc as *mut GCUnion)).u).metatable
        }
        _ => mt = (*(*L).l_G).mt[((*o).tt_ & 0xfi32) as usize],
    }
    return if !mt.is_null() {
        luaH_getshortstr(mt, (*(*L).l_G).tmname[event as usize])
    } else {
        &luaO_nilobject_
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_init(mut L: *mut lua_State_0) -> () {
    /* ORDER TM */
    static mut luaT_eventname: [*const libc::c_char; 24] = unsafe {
        [
            b"__index\x00" as *const u8 as *const libc::c_char,
            b"__newindex\x00" as *const u8 as *const libc::c_char,
            b"__gc\x00" as *const u8 as *const libc::c_char,
            b"__mode\x00" as *const u8 as *const libc::c_char,
            b"__len\x00" as *const u8 as *const libc::c_char,
            b"__eq\x00" as *const u8 as *const libc::c_char,
            b"__add\x00" as *const u8 as *const libc::c_char,
            b"__sub\x00" as *const u8 as *const libc::c_char,
            b"__mul\x00" as *const u8 as *const libc::c_char,
            b"__mod\x00" as *const u8 as *const libc::c_char,
            b"__pow\x00" as *const u8 as *const libc::c_char,
            b"__div\x00" as *const u8 as *const libc::c_char,
            b"__idiv\x00" as *const u8 as *const libc::c_char,
            b"__band\x00" as *const u8 as *const libc::c_char,
            b"__bor\x00" as *const u8 as *const libc::c_char,
            b"__bxor\x00" as *const u8 as *const libc::c_char,
            b"__shl\x00" as *const u8 as *const libc::c_char,
            b"__shr\x00" as *const u8 as *const libc::c_char,
            b"__unm\x00" as *const u8 as *const libc::c_char,
            b"__bnot\x00" as *const u8 as *const libc::c_char,
            b"__lt\x00" as *const u8 as *const libc::c_char,
            b"__le\x00" as *const u8 as *const libc::c_char,
            b"__concat\x00" as *const u8 as *const libc::c_char,
            b"__call\x00" as *const u8 as *const libc::c_char,
        ]
    };
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < TM_N as libc::c_int {
        (*(*L).l_G).tmname[i as usize] = luaS_new(L, luaT_eventname[i as usize]);
        /* never collect these names */
        if (*(*(*L).l_G).tmname[i as usize]).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"((((L->l_G)->tmname[i])->tt) & 0x0F) < (9+1)\x00" as *const u8
                    as *const libc::c_char,
                b"ltm.c\x00" as *const u8 as *const libc::c_char,
                50i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"void luaT_init(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        luaC_fix(
            L,
            &mut (*((*(*L).l_G).tmname[i as usize] as *mut GCUnion)).gc,
        );
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callTM(
    mut L: *mut lua_State_0,
    mut f: *const TValue,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut p3: *mut TValue,
    mut hasres: libc::c_int,
) -> () {
    let mut io1_2: *mut TValue = 0 as *mut TValue;
    let mut result: ptrdiff_t = (p3 as *mut libc::c_char)
        .wrapping_offset_from((*L).stack as *mut libc::c_char)
        as libc::c_long;
    let mut func: StkId = (*L).top;
    let mut io1: *mut TValue = func;
    *io1 = *f;
    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"ltm.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 106i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 93],
                                                           &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
        };
        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"ltm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              106i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 93],
                                                                        &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
            };
            0 != ((*(*io1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ltm.c\x00" as *const u8 as *const libc::c_char,
                          106i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 93],
                                                    &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
        };
    };
    /* push function (assume EXTRA_STACK) */
    let mut io1_0: *mut TValue = func.offset(1isize);
    *io1_0 = *p1;
    if 0 == (*io1_0).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"ltm.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 107i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 93],
                                                           &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
        };
        (*io1_0).tt_ & 0x3fi32 == (*(*io1_0).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"ltm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              107i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 93],
                                                                        &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
            };
            0 != ((*(*io1_0).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ltm.c\x00" as *const u8 as *const libc::c_char,
                          107i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 93],
                                                    &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
        };
    };
    /* 1st argument */
    let mut io1_1: *mut TValue = func.offset(2isize);
    *io1_1 = *p2;
    if 0 == (*io1_1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"ltm.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 108i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 93],
                                                           &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
        };
        (*io1_1).tt_ & 0x3fi32 == (*(*io1_1).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"ltm.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              108i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 93],
                                                                        &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
            };
            0 != ((*(*io1_1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ltm.c\x00" as *const u8 as *const libc::c_char,
                          108i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 93],
                                                    &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
        };
    };
    /* 2nd argument */
    (*L).top = (*L).top.offset(3isize);
    /* no result? 'p3' is third argument */
    if 0 == hasres {
        let fresh0 = (*L).top;
        (*L).top = (*L).top.offset(1);
        io1_2 = fresh0;
        *io1_2 = *p3;
        if 0 == (*io1_2).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1_2).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ltm.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     111i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 93],
                                                               &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
            };
            (*io1_2).tt_ & 0x3fi32 == (*(*io1_2).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1_2).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ltm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  111i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 93],
                                                                            &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
                };
                0 != ((*(*io1_2).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"ltm.c\x00" as *const u8 as
                                  *const libc::c_char, 111i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 93],
                                                        &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
            };
        };
    }
    /* 3rd argument */
    /* metamethod may yield only when called from Lua code */
    if 0 != (*(*L).ci).callstatus as libc::c_int & 1i32 << 1i32 {
        luaD_call(L, func, hasres);
    } else {
        luaD_callnoyield(L, func, hasres);
    }
    if 0 != hasres {
        /* if has result, move it to its place */
        p3 = ((*L).stack as *mut libc::c_char).offset(result as isize) as *mut TValue;
        let mut io1_3: *mut TValue = p3;
        (*L).top = (*L).top.offset(-1isize);
        *io1_3 = *(*L).top;
        if 0 == (*io1_3).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1_3).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ltm.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     119i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 93],
                                                               &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
            };
            (*io1_3).tt_ & 0x3fi32 == (*(*io1_3).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io1_3).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ltm.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  119i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 93],
                                                                            &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
                };
                0 != ((*(*io1_3).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"ltm.c\x00" as *const u8 as
                                  *const libc::c_char, 119i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 93],
                                                        &[libc::c_char; 93]>(b"void luaT_callTM(lua_State *, const TValue *, const TValue *, const TValue *, TValue *, int)\x00")).as_ptr());
            };
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callbinTM(
    mut L: *mut lua_State_0,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) -> libc::c_int {
    /* try first operand */
    let mut tm: *const TValue = luaT_gettmbyobj(L, p1, event);
    if (*tm).tt_ == 0i32 {
        /* try second operand */
        tm = luaT_gettmbyobj(L, p2, event)
    }
    if (*tm).tt_ == 0i32 {
        return 0i32;
    } else {
        luaT_callTM(L, tm, p1, p2, res, 1i32);
        return 1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_trybinTM(
    mut L: *mut lua_State_0,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) -> () {
    if 0 == luaT_callbinTM(L, p1, p2, res, event) {
        match event as libc::c_uint {
            22 => {
                luaG_concaterror(L, p1, p2);
            }
            13 | 14 | 15 | 16 | 17 | 19 => {
                let mut dummy: lua_Number = 0.;
                if 0 != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                    if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                    } else {
                        __assert_fail(b"((((p1))->tt_) == ((3 | (0 << 4))))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"ltm.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             145i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 76],
                                                                       &[libc::c_char; 76]>(b"void luaT_trybinTM(lua_State *, const TValue *, const TValue *, StkId, TMS)\x00")).as_ptr());
                    };
                    dummy = (*p1).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p1, &mut dummy)
                } && 0 != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                    if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                    } else {
                        __assert_fail(b"((((p2))->tt_) == ((3 | (0 << 4))))\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"ltm.c\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 145i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 76],
                                                                           &[libc::c_char; 76]>(b"void luaT_trybinTM(lua_State *, const TValue *, const TValue *, StkId, TMS)\x00")).as_ptr());
                    };
                    dummy = (*p2).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p2, &mut dummy)
                } {
                    luaG_tointerror(L, p1, p2);
                } else {
                    luaG_opinterror(
                        L,
                        p1,
                        p2,
                        b"perform bitwise operation on\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
            _ => {
                luaG_opinterror(
                    L,
                    p1,
                    p2,
                    b"perform arithmetic on\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    } else {
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callorderTM(
    mut L: *mut lua_State_0,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut event: TMS,
) -> libc::c_int {
    if 0 == luaT_callbinTM(L, p1, p2, (*L).top, event) {
        /* no metamethod */
        return -1i32;
    } else {
        return !((*(*L).top).tt_ == 0i32 || (*(*L).top).tt_ == 1i32 && {
            if (*(*L).top).tt_ == 1i32 {
            } else {
                __assert_fail(b"((((L->top))->tt_) == (1))\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               b"ltm.c\x00" as *const u8 as
                                                   *const libc::c_char,
                                               163i32 as libc::c_uint,
                                               (*::std::mem::transmute::<&[u8; 71],
                                                                         &[libc::c_char; 71]>(b"int luaT_callorderTM(lua_State *, const TValue *, const TValue *, TMS)\x00")).as_ptr());
            };
            (*(*L).top).value_.b == 0i32
        }) as libc::c_int;
    };
}
