extern crate libc;
extern "C" {
    pub type UpVal;
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
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn localeconv() -> *mut lconv;
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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong, _: *const libc::c_char, ...)
        -> libc::c_int;
    #[no_mangle]
    fn strtod(__nptr: *const libc::c_char, __endptr: *mut *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    static luai_ctype_: [lu_byte; 257];
    #[no_mangle]
    fn luaT_trybinTM(
        L: *mut lua_State_0,
        p1: *const TValue,
        p2: *const TValue,
        res: StkId,
        event: TMS,
    ) -> ();
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    #[no_mangle]
    fn luaV_shiftl(x: lua_Integer, y: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaV_div(L: *mut lua_State_0, x: lua_Integer, y: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaV_mod(L: *mut lua_State_0, x: lua_Integer, y: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State_0, str: *const libc::c_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaD_inctop(L: *mut lua_State_0) -> ();
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub const TM_ADD: TMS = 6;
pub const TM_LE: TMS = 21;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
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
    pub ci: *mut CallInfo,
    pub oldpc: *const Instruction,
    pub stack_last: StkId,
    pub stack: StkId,
    pub openupval: *mut UpVal_0,
    pub gclist: *mut GCObject,
    pub twups: *mut lua_State,
    pub errorJmp: *mut lua_longjmp,
    pub base_ci: CallInfo,
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
pub const TM_MUL: TMS = 8;
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
/* only for Lua functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed {
    pub base: StkId,
    pub savedpc: *const Instruction,
}
pub const TM_BAND: TMS = 13;
pub const TM_LT: TMS = 20;
/* unsigned integer type */
pub type lua_Unsigned = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    c: CClosure,
    l: LClosure_0,
}
/* 16-bit ints */
 /* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = libc::c_uchar;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void, _: size_t, _: size_t)
        -> *mut libc::c_void,
>;
/* last tag method with fast access */
pub const TM_EQ: TMS = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_0 {
    l: unnamed,
    c: unnamed_3,
}
pub type sig_atomic_t = __sig_atomic_t;
/*
** Lua Upvalues
*/
pub type UpVal_0 = UpVal;
pub const TM_SHL: TMS = 16;
pub const TM_BXOR: TMS = 15;
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
/*
** Header for string value; string bytes follow the end of this structure
** (aligned according to 'UTString'; see next).
*/
pub type TString = TString_0;
pub const TM_UNM: TMS = 18;
pub type TValue = lua_TValue;
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
pub const TM_POW: TMS = 10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_TValue {
    pub value_: Value,
    pub tt_: libc::c_int,
}
/* copy a value into a key without messing up field 'next' */
pub type Node = Node_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union TKey {
    nk: unnamed_1,
    tvk: TValue,
}
pub const TM_INDEX: TMS = 0;
/*
** Type for continuation functions
*/
pub type lua_KFunction = Option<
    unsafe extern "C" fn(_: *mut lua_State_0, _: libc::c_int, _: lua_KContext) -> libc::c_int,
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
pub struct unnamed_1 {
    pub value_: Value,
    pub tt_: libc::c_int,
    pub next: libc::c_int,
}
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
    pub upvalues: *mut Upvaldesc_0,
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    lnglen: size_t,
    hnext: *mut TString_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node_0 {
    pub i_val: TValue,
    pub i_key: TKey_0,
}
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
pub const TM_LEN: TMS = 4;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCObject_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
}
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
/* only for C functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
    pub k: lua_KFunction,
    pub old_errfunc: ptrdiff_t,
    pub ctx: lua_KContext,
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
    pub upvals: [*mut UpVal_0; 0],
}
pub const TM_SUB: TMS = 7;
/*
** Information about a call.
** When a thread yields, 'func' is adjusted to pretend that the
** top function has only the yielded values in its stack; in that
** case, the actual 'func' value is saved in field 'extra'.
** When a function calls another with a continuation, 'extra' keeps
** the function index so that, in case of errors, the continuation
** function can be called with the correct top.
*/
pub type CallInfo = CallInfo_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
pub const TM_IDIV: TMS = 12;
pub const TM_SHR: TMS = 17;
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
pub const TM_GC: TMS = 2;
pub const TM_MOD: TMS = 9;
/* active function */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo_0 {
    pub func: StkId,
    pub top: StkId,
    pub previous: *mut CallInfo_0,
    pub next: *mut CallInfo_0,
    pub u: unnamed_0,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
pub const TM_NEWINDEX: TMS = 1;
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
/* Functions to be called by the debugger in specific events */
pub type lua_Hook = Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut lua_Debug) -> ()>;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
pub type LClosure_0 = LClosure;
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
** Tables
*/
pub type TKey_0 = TKey;
pub const TM_MODE: TMS = 3;
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
pub const TM_CALL: TMS = 23;
pub type intptr_t = libc::c_long;
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
pub type LocVar = LocVar_0;
pub type ptrdiff_t = libc::c_long;
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
pub const TM_BOR: TMS = 14;
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
/* number of elements in the enum */
pub const TM_N: TMS = 24;
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
pub type Upvaldesc_0 = Upvaldesc;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State_0) -> libc::c_int>;
pub const TM_BNOT: TMS = 19;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar_0 {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type TMS = libc::c_uint;
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
    pub i_ci: *mut CallInfo_0,
}
/*
** Closures
*/
pub type CClosure = CClosure_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable_0 {
    pub hash: *mut *mut TString,
    pub nuse: libc::c_int,
    pub size: libc::c_int,
}
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
/* type for continuation-function contexts */
pub type lua_KContext = intptr_t;
pub type l_mem = ptrdiff_t;
pub const TM_CONCAT: TMS = 22;
pub const TM_DIV: TMS = 11;
#[no_mangle]
pub static mut luaO_nilobject_: TValue = unsafe {
    lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0i32,
    }
};
#[no_mangle]
pub unsafe extern "C" fn luaO_int2fb(mut x: libc::c_uint) -> libc::c_int {
    /* exponent */
    let mut e: libc::c_int = 0i32;
    if x < 8i32 as libc::c_uint {
        return x as libc::c_int;
    } else {
        while x >= (8i32 << 4i32) as libc::c_uint {
            /* coarse steps */
            /* x = ceil(x / 16) */
            x = x.wrapping_add(0xfi32 as libc::c_uint) >> 4i32;
            e += 4i32
        }
        while x >= (8i32 << 1i32) as libc::c_uint {
            /* fine steps */
            /* x = ceil(x / 2) */
            x = x.wrapping_add(1i32 as libc::c_uint) >> 1i32;
            e += 1
        }
        return e + 1i32 << 3i32 | x as libc::c_int - 8i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_fb2int(mut x: libc::c_int) -> libc::c_int {
    return if x < 8i32 {
        x
    } else {
        (x & 7i32) + 8i32 << (x >> 3i32) - 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_utf8esc(
    mut buff: *mut libc::c_char,
    mut x: libc::c_ulong,
) -> libc::c_int {
    /* number of bytes put in buffer (backwards) */
    let mut n: libc::c_int = 1i32;
    if x <= 0x10ffffi32 as libc::c_ulong {
    } else {
        __assert_fail(
            b"x <= 0x10FFFF\x00" as *const u8 as *const libc::c_char,
            b"lobject.c\x00" as *const u8 as *const libc::c_char,
            349i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"int luaO_utf8esc(char *, unsigned long)\x00",
            )).as_ptr(),
        );
    };
    /* ascii? */
    if x < 0x80i32 as libc::c_ulong {
        *buff.offset((8i32 - 1i32) as isize) = x as libc::c_char
    } else {
        /* need continuation bytes */
        /* maximum that fits in first byte */
        let mut mfb: libc::c_uint = 0x3fi32 as libc::c_uint;
        loop {
            /* add continuation bytes */
            let fresh0 = n;
            n = n + 1;
            *buff.offset((8i32 - fresh0) as isize) =
                (0x80i32 as libc::c_ulong | x & 0x3fi32 as libc::c_ulong) as libc::c_char;
            /* remove added bits */
            x >>= 6i32;
            /* now there is one less bit available in first byte */
            mfb >>= 1i32;
            if !(x > mfb as libc::c_ulong) {
                break;
            }
        }
        /* still needs continuation byte? */
        /* add first byte */
        *buff.offset((8i32 - n) as isize) = ((!mfb << 1i32) as libc::c_ulong | x) as libc::c_char
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_ceillog2(mut x: libc::c_uint) -> libc::c_int {
    /* log_2[i] = ceil(log2(i - 1)) */
    static mut log_2: [lu_byte; 256] = unsafe {
        [
            0i32 as lu_byte,
            1i32 as lu_byte,
            2i32 as lu_byte,
            2i32 as lu_byte,
            3i32 as lu_byte,
            3i32 as lu_byte,
            3i32 as lu_byte,
            3i32 as lu_byte,
            4i32 as lu_byte,
            4i32 as lu_byte,
            4i32 as lu_byte,
            4i32 as lu_byte,
            4i32 as lu_byte,
            4i32 as lu_byte,
            4i32 as lu_byte,
            4i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            5i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            6i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            7i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
            8i32 as lu_byte,
        ]
    };
    let mut l: libc::c_int = 0i32;
    x = x.wrapping_sub(1);
    while x >= 256i32 as libc::c_uint {
        l += 8i32;
        x >>= 8i32
    }
    return l + log_2[x as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_arith(
    mut L: *mut lua_State_0,
    mut op: libc::c_int,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: *mut TValue,
) -> () {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            /* operate only on integers */
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            if 0 != if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
                if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(b"((((p1))->tt_) == ((3 | (1 << 4))))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lobject.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         131i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 76],
                                                                   &[libc::c_char; 76]>(b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00")).as_ptr());
                };
                i1 = (*p1).value_.i;
                1i32
            } else {
                luaV_tointeger(p1, &mut i1, 0i32)
            } && 0 != if (*p2).tt_ == 3i32 | 1i32 << 4i32 {
                if (*p2).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(b"((((p2))->tt_) == ((3 | (1 << 4))))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lobject.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             131i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 76],
                                                                       &[libc::c_char; 76]>(b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00")).as_ptr());
                };
                i2 = (*p2).value_.i;
                1i32
            } else {
                luaV_tointeger(p2, &mut i2, 0i32)
            } {
                let mut io: *mut TValue = res;
                (*io).value_.i = intarith(L, op, i1, i2);
                (*io).tt_ = 3i32 | 1i32 << 4i32;
                return;
            }
        }
        5 | 4 => {
            /* go to the end */
            /* operate only on floats */
            let mut n1: lua_Number = 0.;
            let mut n2: lua_Number = 0.;
            if 0 != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"((((p1))->tt_) == ((3 | (0 << 4))))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lobject.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         139i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 76],
                                                                   &[libc::c_char; 76]>(b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00")).as_ptr());
                };
                n1 = (*p1).value_.n;
                1i32
            } else {
                luaV_tonumber_(p1, &mut n1)
            } && 0 != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"((((p2))->tt_) == ((3 | (0 << 4))))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lobject.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             139i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 76],
                                                                       &[libc::c_char; 76]>(b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00")).as_ptr());
                };
                n2 = (*p2).value_.n;
                1i32
            } else {
                luaV_tonumber_(p2, &mut n2)
            } {
                let mut io_0: *mut TValue = res;
                (*io_0).value_.n = numarith(L, op, n1, n2);
                (*io_0).tt_ = 3i32 | 0i32 << 4i32;
                return;
            }
        }
        _ => {
            /* go to the end */
            /* other operations */
            let mut n1_0: lua_Number = 0.;
            let mut n2_0: lua_Number = 0.;
            if (*p1).tt_ == 3i32 | 1i32 << 4i32 && (*p2).tt_ == 3i32 | 1i32 << 4i32 {
                let mut io_1: *mut TValue = res;
                if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(b"((((p1))->tt_) == ((3 | (1 << 4))))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lobject.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  148i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 76],
                                                            &[libc::c_char; 76]>(b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00")).as_ptr());
                };
                if (*p2).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(b"((((p2))->tt_) == ((3 | (1 << 4))))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lobject.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  148i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 76],
                                                            &[libc::c_char; 76]>(b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00")).as_ptr());
                };
                (*io_1).value_.i = intarith(L, op, (*p1).value_.i, (*p2).value_.i);
                (*io_1).tt_ = 3i32 | 1i32 << 4i32;
                return;
            } else if 0 != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"((((p1))->tt_) == ((3 | (0 << 4))))\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                b"lobject.c\x00" as *const u8
                                                    as *const libc::c_char,
                                                151i32 as libc::c_uint,
                                                (*::std::mem::transmute::<&[u8; 76],
                                                                          &[libc::c_char; 76]>(b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00")).as_ptr());
                };
                n1_0 = (*p1).value_.n;
                1i32
            } else {
                luaV_tonumber_(p1, &mut n1_0)
            } && 0 != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"((((p2))->tt_) == ((3 | (0 << 4))))\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    b"lobject.c\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    151i32 as libc::c_uint,
                                                    (*::std::mem::transmute::<&[u8; 76],
                                                                              &[libc::c_char; 76]>(b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00")).as_ptr());
                };
                n2_0 = (*p2).value_.n;
                1i32
            } else {
                luaV_tonumber_(p2, &mut n2_0)
            } {
                let mut io_2: *mut TValue = res;
                (*io_2).value_.n = numarith(L, op, n1_0, n2_0);
                (*io_2).tt_ = 3i32 | 0i32 << 4i32;
                return;
            }
        }
    }
    /* go to the end */
    if !L.is_null() {
    } else {
        __assert_fail(
            b"L != ((void*)0)\x00" as *const u8 as *const libc::c_char,
            b"lobject.c\x00" as *const u8 as *const libc::c_char,
            159i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 76], &[libc::c_char; 76]>(
                b"void luaO_arith(lua_State *, int, const TValue *, const TValue *, TValue *)\x00",
            )).as_ptr(),
        );
    };
    /* could not perform raw operation; try metamethod */
    /* should not fail when folding (compile time) */
    luaT_trybinTM(L, p1, p2, res, (op - 0i32 + TM_ADD as libc::c_int) as TMS);
}
unsafe extern "C" fn numarith(
    mut L: *mut lua_State_0,
    mut op: libc::c_int,
    mut v1: lua_Number,
    mut v2: lua_Number,
) -> lua_Number {
    match op {
        0 => return v1 + v2,
        1 => return v1 - v2,
        2 => return v1 * v2,
        5 => return v1 / v2,
        4 => return pow(v1, v2),
        6 => return floor(v1 / v2),
        12 => return -v1,
        3 => {
            let mut m: lua_Number = 0.;
            m = fmod(v1, v2);
            if m * v2 < 0i32 as libc::c_double {
                m += v2
            }
            return m;
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lobject.c\x00" as *const u8 as *const libc::c_char,
                    119i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                        b"lua_Number numarith(lua_State *, int, lua_Number, lua_Number)\x00",
                    )).as_ptr(),
                );
            };
            return 0i32 as lua_Number;
        }
    };
}
unsafe extern "C" fn intarith(
    mut L: *mut lua_State_0,
    mut op: libc::c_int,
    mut v1: lua_Integer,
    mut v2: lua_Integer,
) -> lua_Integer {
    match op {
        0 => return (v1 as lua_Unsigned).wrapping_add(v2 as lua_Unsigned) as lua_Integer,
        1 => return (v1 as lua_Unsigned).wrapping_sub(v2 as lua_Unsigned) as lua_Integer,
        2 => return (v1 as lua_Unsigned).wrapping_mul(v2 as lua_Unsigned) as lua_Integer,
        3 => return luaV_mod(L, v1, v2),
        6 => return luaV_div(L, v1, v2),
        7 => return (v1 as lua_Unsigned & v2 as lua_Unsigned) as lua_Integer,
        8 => return (v1 as lua_Unsigned | v2 as lua_Unsigned) as lua_Integer,
        9 => return (v1 as lua_Unsigned ^ v2 as lua_Unsigned) as lua_Integer,
        10 => return luaV_shiftl(v1, v2),
        11 => return luaV_shiftl(v1, -v2),
        12 => return (0i32 as lua_Unsigned).wrapping_sub(v1 as lua_Unsigned) as lua_Integer,
        13 => return (!(0i32 as lua_Unsigned) ^ v1 as lua_Unsigned) as lua_Integer,
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lobject.c\x00" as *const u8 as *const libc::c_char,
                    99i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                        b"lua_Integer intarith(lua_State *, int, lua_Integer, lua_Integer)\x00",
                    )).as_ptr(),
                );
            };
            return 0i32 as lua_Integer;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_str2num(mut s: *const libc::c_char, mut o: *mut TValue) -> size_t {
    let mut i: lua_Integer = 0;
    let mut n: lua_Number = 0.;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    e = l_str2int(s, &mut i);
    if !e.is_null() {
        /* try as an integer */
        let mut io: *mut TValue = o;
        (*io).value_.i = i;
        (*io).tt_ = 3i32 | 1i32 << 4i32
    } else {
        e = l_str2d(s, &mut n);
        if !e.is_null() {
            /* else try as a float */
            let mut io_0: *mut TValue = o;
            (*io_0).value_.n = n;
            (*io_0).tt_ = 3i32 | 0i32 << 4i32
        } else {
            return 0i32 as size_t;
        }
    }
    /* success; return string size */
    return (e.wrapping_offset_from(s) as libc::c_long + 1i32 as libc::c_long) as size_t;
}
/*
** Convert string 's' to a Lua number (put in 'result'). Return NULL
** on fail or the address of the ending '\0' on success.
** 'pmode' points to (and 'mode' contains) special things in the string:
** - 'x'/'X' means an hexadecimal numeral
** - 'n'/'N' means 'inf' or 'nan' (which should be rejected)
** - '.' just optimizes the search for the common case (nothing special)
** This function accepts both the current locale or a dot as the radix
** mark. If the convertion fails, it may mean number has a dot but
** locale accepts something else. In that case, the code copies 's'
** to a buffer (because 's' is read-only), changes the dot to the
** current locale radix mark, and tries to convert again.
*/
unsafe extern "C" fn l_str2d(
    mut s: *const libc::c_char,
    mut result: *mut lua_Number,
) -> *const libc::c_char {
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut pmode: *const libc::c_char =
        strpbrk(s, b".xXnN\x00" as *const u8 as *const libc::c_char);
    let mut mode: libc::c_int = if !pmode.is_null() {
        *pmode as libc::c_uchar as libc::c_int | 'A' as i32 ^ 'a' as i32
    } else {
        0i32
    };
    /* reject 'inf' and 'nan' */
    if mode == 'n' as i32 {
        return 0 as *const libc::c_char;
    } else {
        /* try to convert */
        endptr = l_str2dloc(s, result, mode);
        if endptr.is_null() {
            /* failed? may be a different locale */
            let mut buff: [libc::c_char; 201] = [0; 201];
            let mut pdot: *const libc::c_char = strchr(s, '.' as i32);
            if strlen(s) > 200i32 as libc::c_ulong || pdot.is_null() {
                /* string too long or no dot; fail */
                return 0 as *const libc::c_char;
            } else {
                /* copy string to buffer */
                strcpy(buff.as_mut_ptr(), s);
                /* correct decimal point */
                buff[pdot.wrapping_offset_from(s) as libc::c_long as usize] =
                    *(*localeconv()).decimal_point.offset(0isize);
                /* try again */
                endptr = l_str2dloc(buff.as_mut_ptr(), result, mode);
                if !endptr.is_null() {
                    /* make relative to 's' */
                    endptr = s.offset(
                        endptr.wrapping_offset_from(buff.as_mut_ptr()) as libc::c_long as isize,
                    )
                }
            }
        }
        return endptr;
    };
}
/*
** {==================================================================
** Lua's implementation for 'lua_strx2number'
** ===================================================================
*/
/* }====================================================== */
/* maximum length of a numeral */
unsafe extern "C" fn l_str2dloc(
    mut s: *const libc::c_char,
    mut result: *mut lua_Number,
    mut mode: libc::c_int,
) -> *const libc::c_char {
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    /* try to convert */
    *result = if mode == 'x' as i32 {
        strtod(s, &mut endptr)
    } else {
        strtod(s, &mut endptr)
    };
    if endptr == s as *mut libc::c_char {
        /* nothing recognized? */
        return 0 as *const libc::c_char;
    } else {
        while 0 != luai_ctype_[(*endptr as libc::c_uchar as libc::c_int + 1i32) as usize]
            as libc::c_int & 1i32 << 3i32
        {
            /* skip trailing spaces */
            endptr = endptr.offset(1isize)
        }
        /* OK if no trailing characters */
        return if *endptr as libc::c_int == '\u{0}' as i32 {
            endptr
        } else {
            0 as *mut libc::c_char
        };
    };
}
unsafe extern "C" fn l_str2int(
    mut s: *const libc::c_char,
    mut result: *mut lua_Integer,
) -> *const libc::c_char {
    let mut a: lua_Unsigned = 0i32 as lua_Unsigned;
    let mut empty: libc::c_int = 1i32;
    let mut neg: libc::c_int = 0;
    while 0 != luai_ctype_[(*s as libc::c_uchar as libc::c_int + 1i32) as usize] as libc::c_int
        & 1i32 << 3i32
    {
        /* skip initial spaces */
        s = s.offset(1isize)
    }
    neg = isneg(&mut s);
    if *s.offset(0isize) as libc::c_int == '0' as i32
        && (*s.offset(1isize) as libc::c_int == 'x' as i32
            || *s.offset(1isize) as libc::c_int == 'X' as i32)
    {
        /* hex? */
        /* skip '0x' */
        s = s.offset(2isize);
        while 0 != luai_ctype_[(*s as libc::c_uchar as libc::c_int + 1i32) as usize] as libc::c_int
            & 1i32 << 4i32
        {
            a = a
                .wrapping_mul(16i32 as libc::c_ulonglong)
                .wrapping_add(luaO_hexavalue(*s as libc::c_int) as libc::c_ulonglong);
            empty = 0i32;
            s = s.offset(1isize)
        }
    } else {
        while 0 != luai_ctype_[(*s as libc::c_uchar as libc::c_int + 1i32) as usize] as libc::c_int
            & 1i32 << 1i32
        {
            let mut d: libc::c_int = *s as libc::c_int - '0' as i32;
            /* overflow? */
            if a >= (9223372036854775807i64 / 10i32 as libc::c_longlong) as lua_Unsigned
                && (a > (9223372036854775807i64 / 10i32 as libc::c_longlong) as lua_Unsigned
                    || d
                        > (9223372036854775807i64 % 10i32 as libc::c_longlong) as libc::c_int + neg)
            {
                /* do not accept it (as integer) */
                return 0 as *const libc::c_char;
            } else {
                a = a
                    .wrapping_mul(10i32 as libc::c_ulonglong)
                    .wrapping_add(d as libc::c_ulonglong);
                empty = 0i32;
                s = s.offset(1isize)
            }
        }
    }
    while 0 != luai_ctype_[(*s as libc::c_uchar as libc::c_int + 1i32) as usize] as libc::c_int
        & 1i32 << 3i32
    {
        /* skip trailing spaces */
        s = s.offset(1isize)
    }
    if 0 != empty || *s as libc::c_int != '\u{0}' as i32 {
        /* something wrong in the numeral */
        return 0 as *const libc::c_char;
    } else {
        *result = (if 0 != neg {
            (0u32 as libc::c_ulonglong).wrapping_sub(a)
        } else {
            a
        }) as lua_Integer;
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_hexavalue(mut c: libc::c_int) -> libc::c_int {
    if 0 != luai_ctype_[(c + 1i32) as usize] as libc::c_int & 1i32 << 1i32 {
        return c - '0' as i32;
    } else {
        return (c | 'A' as i32 ^ 'a' as i32) - 'a' as i32 + 10i32;
    };
}
unsafe extern "C" fn isneg(mut s: *mut *const libc::c_char) -> libc::c_int {
    if **s as libc::c_int == '-' as i32 {
        *s = (*s).offset(1isize);
        return 1i32;
    } else {
        if **s as libc::c_int == '+' as i32 {
            *s = (*s).offset(1isize)
        }
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_tostring(mut L: *mut lua_State_0, mut obj: StkId) -> () {
    let mut buff: [libc::c_char; 50] = [0; 50];
    let mut len: size_t = 0;
    if (*obj).tt_ & 0xfi32 == 3i32 {
    } else {
        __assert_fail(
            b"(((((((obj))->tt_)) & 0x0F)) == (3))\x00" as *const u8 as *const libc::c_char,
            b"lobject.c\x00" as *const u8 as *const libc::c_char,
            375i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void luaO_tostring(lua_State *, StkId)\x00",
            )).as_ptr(),
        );
    };
    if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
        memset(
            buff.as_mut_ptr() as *mut libc::c_void,
            0xabi32,
            ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
        );
        if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((obj))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lobject.c\x00" as *const u8 as *const libc::c_char,
                377i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void luaO_tostring(lua_State *, StkId)\x00",
                )).as_ptr(),
            );
        };
        len = snprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
            b"%lld\x00" as *const u8 as *const libc::c_char,
            (*obj).value_.i,
        ) as size_t
    } else {
        memset(
            buff.as_mut_ptr() as *mut libc::c_void,
            0xabi32,
            ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
        );
        if (*obj).tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((obj))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lobject.c\x00" as *const u8 as *const libc::c_char,
                379i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void luaO_tostring(lua_State *, StkId)\x00",
                )).as_ptr(),
            );
        };
        len = snprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
            b"%.14g\x00" as *const u8 as *const libc::c_char,
            (*obj).value_.n,
        ) as size_t;
        if buff[strspn(
            buff.as_mut_ptr(),
            b"-0123456789\x00" as *const u8 as *const libc::c_char,
        ) as usize] as libc::c_int == '\u{0}' as i32
        {
            /* looks like an int? */
            let fresh1 = len;
            len = len.wrapping_add(1);
            buff[fresh1 as usize] = *(*localeconv()).decimal_point.offset(0isize);
            /* adds '.0' to result */
            let fresh2 = len;
            len = len.wrapping_add(1);
            buff[fresh2 as usize] = '0' as i32 as libc::c_char
        }
    }
    let mut io: *mut TValue = obj;
    let mut x_: *mut TString = luaS_newlstr(L, buff.as_mut_ptr(), len);
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lobject.c\x00" as *const u8 as *const libc::c_char,
            387i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void luaO_tostring(lua_State *, StkId)\x00",
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
                b"lobject.c\x00" as *const u8 as *const libc::c_char,
                387i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void luaO_tostring(lua_State *, StkId)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lobject.c\x00" as *const u8 as *const libc::c_char,
                    387i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"void luaO_tostring(lua_State *, StkId)\x00",
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
                b"lobject.c\x00" as *const u8 as *const libc::c_char,
                387i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void luaO_tostring(lua_State *, StkId)\x00",
                )).as_ptr(),
            );
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_chunkid(
    mut out: *mut libc::c_char,
    mut source: *const libc::c_char,
    mut bufflen: size_t,
) -> () {
    let mut l: size_t = strlen(source);
    if *source as libc::c_int == '=' as i32 {
        /* 'literal' source */
        /* small enough? */
        if l <= bufflen {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1isize) as *const libc::c_void,
                l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        } else {
            /* truncate it */
            memcpy(
                out as *mut libc::c_void,
                source.offset(1isize) as *const libc::c_void,
                bufflen
                    .wrapping_sub(1i32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(bufflen.wrapping_sub(1i32 as libc::c_ulong) as isize);
            *out = '\u{0}' as i32 as libc::c_char
        }
    } else if *source as libc::c_int == '@' as i32 {
        /* file name */
        /* small enough? */
        if l <= bufflen {
            memcpy(
                out as *mut libc::c_void,
                source.offset(1isize) as *const libc::c_void,
                l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        } else {
            /* add '...' before rest of name */
            memcpy(
                out as *mut libc::c_void,
                b"...\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as isize,
            );
            bufflen = (bufflen as libc::c_ulong).wrapping_sub(
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong),
            ) as size_t as size_t;
            memcpy(
                out as *mut libc::c_void,
                source
                    .offset(1isize)
                    .offset(l as isize)
                    .offset(-(bufflen as isize)) as *const libc::c_void,
                bufflen.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
        }
    } else {
        /* string; format as [string "source"] */
        /* find first new line (if any) */
        let mut nl: *const libc::c_char = strchr(source, '\n' as i32);
        memcpy(
            out as *mut libc::c_void,
            b"[string \"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        out = out.offset(
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong) as isize,
        );
        /* add prefix */
        /* save space for prefix+suffix+'\0' */
        bufflen = (bufflen as libc::c_ulong).wrapping_sub(
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                .wrapping_add(1i32 as libc::c_ulong),
        ) as size_t as size_t;
        if l < bufflen && nl.is_null() {
            /* small one-line source? */
            memcpy(
                out as *mut libc::c_void,
                source as *const libc::c_void,
                l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(l as isize)
        } else {
            /* keep it */
            if !nl.is_null() {
                /* stop at first newline */
                l = nl.wrapping_offset_from(source) as libc::c_long as size_t
            }
            if l > bufflen {
                l = bufflen
            }
            memcpy(
                out as *mut libc::c_void,
                source as *const libc::c_void,
                l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(l as isize);
            memcpy(
                out as *mut libc::c_void,
                b"...\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            out = out.offset(
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as isize,
            )
        }
        memcpy(
            out as *mut libc::c_void,
            b"\"]\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
                .wrapping_add(1i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn pushstr(
    mut L: *mut lua_State_0,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> () {
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = luaS_newlstr(L, str, l);
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lobject.c\x00" as *const u8 as *const libc::c_char,
            392i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void pushstr(lua_State *, const char *, size_t)\x00",
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
                b"lobject.c\x00" as *const u8 as *const libc::c_char,
                392i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void pushstr(lua_State *, const char *, size_t)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lobject.c\x00" as *const u8 as *const libc::c_char,
                    392i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                        b"void pushstr(lua_State *, const char *, size_t)\x00",
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
                b"lobject.c\x00" as *const u8 as *const libc::c_char,
                392i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void pushstr(lua_State *, const char *, size_t)\x00",
                )).as_ptr(),
            );
        };
    };
    luaD_inctop(L);
}
