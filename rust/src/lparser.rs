use lobject::{lua_TValue, GCObject, TValue, Value};
use lstate::{global_State, lua_State, CallInfo, GCUnion};

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
    fn luaO_int2fb(x: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn luaM_toobig(L: *mut lua_State_0) -> !;
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State_0,
        block_0: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaM_growaux_(
        L: *mut lua_State_0,
        block_0: *mut libc::c_void,
        size: *mut libc::c_int,
        size_elem: size_t,
        limit: libc::c_int,
        what: *const libc::c_char,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaX_setinput(
        L: *mut lua_State_0,
        ls: *mut LexState_0,
        z: *mut ZIO,
        source: *mut TString,
        firstchar: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn luaX_newstring(ls: *mut LexState_0, str: *const libc::c_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaX_next(ls: *mut LexState_0) -> ();
    #[no_mangle]
    fn luaX_lookahead(ls: *mut LexState_0) -> libc::c_int;
    #[no_mangle]
    fn luaX_syntaxerror(ls: *mut LexState_0, s: *const libc::c_char) -> !;
    #[no_mangle]
    fn luaX_token2str(ls: *mut LexState_0, token: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
    #[no_mangle]
    fn luaF_newLclosure(L: *mut lua_State_0, nelems: libc::c_int) -> *mut LClosure_0;
    #[no_mangle]
    fn luaC_step(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn luaK_patchlist(fs: *mut FuncState_0, list: libc::c_int, target: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_patchclose(fs: *mut FuncState_0, list: libc::c_int, level: libc::c_int) -> ();
    #[no_mangle]
    fn luaS_new(L: *mut lua_State_0, str: *const libc::c_char) -> *mut TString;
    #[no_mangle]
    fn luaK_jump(fs: *mut FuncState_0) -> libc::c_int;
    #[no_mangle]
    fn luaK_patchtohere(fs: *mut FuncState_0, list: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_ret(fs: *mut FuncState_0, first: libc::c_int, nret: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_storevar(fs: *mut FuncState_0, var: *mut expdesc_0, e: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_setoneret(fs: *mut FuncState_0, e: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_nil(fs: *mut FuncState_0, from: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_reserveregs(fs: *mut FuncState_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_exp2nextreg(fs: *mut FuncState_0, e: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_setreturns(fs: *mut FuncState_0, e: *mut expdesc_0, nresults: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_posfix(
        fs: *mut FuncState_0,
        op: BinOpr,
        v1: *mut expdesc_0,
        v2: *mut expdesc_0,
        line: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn luaK_infix(fs: *mut FuncState_0, op: BinOpr, v: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_fixline(fs: *mut FuncState_0, line: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_codeABC(
        fs: *mut FuncState_0,
        o: OpCode,
        A: libc::c_int,
        B: libc::c_int,
        C: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaK_stringK(fs: *mut FuncState_0, s: *mut TString) -> libc::c_int;
    #[no_mangle]
    fn luaK_setlist(
        fs: *mut FuncState_0,
        base: libc::c_int,
        nelems: libc::c_int,
        tostore: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn luaK_exp2RK(fs: *mut FuncState_0, e: *mut expdesc_0) -> libc::c_int;
    #[no_mangle]
    fn luaK_exp2val(fs: *mut FuncState_0, e: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_self(fs: *mut FuncState_0, e: *mut expdesc_0, key: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_indexed(fs: *mut FuncState_0, t: *mut expdesc_0, k: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_exp2anyregup(fs: *mut FuncState_0, e: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaC_barrier_(L: *mut lua_State_0, o: *mut GCObject, v: *mut GCObject) -> ();
    #[no_mangle]
    fn luaK_dischargevars(fs: *mut FuncState_0, e: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_codeABx(
        fs: *mut FuncState_0,
        o: OpCode,
        A: libc::c_int,
        Bx: libc::c_uint,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaF_newproto(L: *mut lua_State_0) -> *mut Proto_0;
    #[no_mangle]
    fn luaK_prefix(fs: *mut FuncState_0, op: UnOpr, v: *mut expdesc_0, line: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_exp2anyreg(fs: *mut FuncState_0, e: *mut expdesc_0) -> libc::c_int;
    #[no_mangle]
    fn luaK_getlabel(fs: *mut FuncState_0) -> libc::c_int;
    #[no_mangle]
    fn luaK_goiftrue(fs: *mut FuncState_0, e: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaK_checkstack(fs: *mut FuncState_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_intK(fs: *mut FuncState_0, n: lua_Integer) -> libc::c_int;
    #[no_mangle]
    fn luaK_codek(fs: *mut FuncState_0, reg: libc::c_int, k: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaK_concat(fs: *mut FuncState_0, l1: *mut libc::c_int, l2: libc::c_int) -> ();
    #[no_mangle]
    fn luaK_goiffalse(fs: *mut FuncState_0, e: *mut expdesc_0) -> ();
    #[no_mangle]
    fn luaD_inctop(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaH_new(L: *mut lua_State_0) -> *mut Table_0;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type size_t = libc::c_ulong;
pub const OPR_LEN: UnOpr_0 = 3;
pub type ptrdiff_t = libc::c_long;
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
pub type UnOpr = UnOpr_0;
pub const OPR_NOUNOPR: UnOpr_0 = 4;
pub const OPR_BNOT: UnOpr_0 = 1;
pub const OPR_NOT: UnOpr_0 = 2;
pub type UnOpr_0 = libc::c_uint;
pub const OPR_MINUS: UnOpr_0 = 0;
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
** $Id: lzio.h,v 1.30 2014/12/19 17:26:14 roberto Exp roberto $
** Buffered streams
** See Copyright Notice in lua.h
*/
/* end of stream */
pub type ZIO = Zio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut libc::c_char,
    pub n: size_t,
    pub buffsize: size_t,
}
pub type Mbuffer_0 = Mbuffer;
/*
** $Id: llex.h,v 1.78 2014/10/29 15:38:24 roberto Exp roberto $
** Lexical Analyzer
** See Copyright Notice in lua.h
*/
/*
* WARNING: if you change the order of this enumeration,
* grep "ORDER RESERVED"
*/
pub type RESERVED = libc::c_uint;
pub const TK_STRING: RESERVED = 293;
pub const TK_NAME: RESERVED = 292;
pub const TK_INT: RESERVED = 291;
pub const TK_FLT: RESERVED = 290;
pub const TK_EOS: RESERVED = 289;
pub const TK_DBCOLON: RESERVED = 288;
pub const TK_SHR: RESERVED = 287;
pub const TK_SHL: RESERVED = 286;
pub const TK_NE: RESERVED = 285;
pub const TK_LE: RESERVED = 284;
pub const TK_GE: RESERVED = 283;
pub const TK_EQ: RESERVED = 282;
pub const TK_DOTS: RESERVED = 281;
pub const TK_CONCAT: RESERVED = 280;
/* other terminal symbols */
pub const TK_IDIV: RESERVED = 279;
pub const TK_WHILE: RESERVED = 278;
pub const TK_UNTIL: RESERVED = 277;
pub const TK_TRUE: RESERVED = 276;
pub const TK_THEN: RESERVED = 275;
pub const TK_RETURN: RESERVED = 274;
pub const TK_REPEAT: RESERVED = 273;
pub const TK_OR: RESERVED = 272;
pub const TK_NOT: RESERVED = 271;
pub const TK_NIL: RESERVED = 270;
pub const TK_LOCAL: RESERVED = 269;
pub const TK_IN: RESERVED = 268;
pub const TK_IF: RESERVED = 267;
pub const TK_GOTO: RESERVED = 266;
pub const TK_FUNCTION: RESERVED = 265;
pub const TK_FOR: RESERVED = 264;
pub const TK_FALSE: RESERVED = 263;
pub const TK_END: RESERVED = 262;
pub const TK_ELSEIF: RESERVED = 261;
pub const TK_ELSE: RESERVED = 260;
pub const TK_DO: RESERVED = 259;
pub const TK_BREAK: RESERVED = 258;
/* terminal symbols denoted by reserved words */
pub const TK_AND: RESERVED = 257;
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    r: lua_Number,
    i: lua_Integer,
    ts: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
/* semantics information */
pub type Token_0 = Token;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: libc::c_int,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
    pub t: Token_0,
    pub lookahead: Token_0,
    pub fs: *mut FuncState,
    pub L: *mut lua_State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer_0,
    pub h: *mut Table_0,
    pub dyd: *mut Dyndata,
    pub source: *mut TString,
    pub envn: *mut TString,
}
/* dynamic structures used by the parser */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    pub actvar: unnamed_6,
    pub gt: Labellist,
    pub label: Labellist,
}
/* list of labels or gotos */
pub type Labellist = Labellist_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist_0 {
    pub arr: *mut Labeldesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
/* description of pending goto statements and label statements */
pub type Labeldesc = Labeldesc_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc_0 {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: lu_byte,
}
/* list of active local variables */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_6 {
    pub arr: *mut Vardesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
/* description of active local variable */
pub type Vardesc = Vardesc_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vardesc_0 {
    pub idx: libc::c_short,
}
/* current function (parser) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto_0,
    pub prev: *mut FuncState,
    pub ls: *mut LexState,
    pub bl: *mut BlockCnt,
    pub pc: libc::c_int,
    pub lasttarget: libc::c_int,
    pub jpc: libc::c_int,
    pub nk: libc::c_int,
    pub np: libc::c_int,
    pub firstlocal: libc::c_int,
    pub nlocvars: libc::c_short,
    pub nactvar: lu_byte,
    pub nups: lu_byte,
    pub freereg: lu_byte,
}
/* control of blocks */
/* defined in lparser.c */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockCnt {
    pub previous: *mut BlockCnt,
    pub firstlabel: libc::c_int,
    pub firstgoto: libc::c_int,
    pub nactvar: lu_byte,
    pub upval: lu_byte,
    pub isloop: lu_byte,
}
/* state of the lexer plus state of the parser when shared by all
   functions */
pub type LexState_0 = LexState;
pub type OpCode = libc::c_uint;
/*	Ax	extra (larger) argument for previous opcode	*/
pub const OP_EXTRAARG: OpCode = 46;
/*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/
pub const OP_VARARG: OpCode = 45;
/*	A Bx	R(A) := closure(KPROTO[Bx])			*/
pub const OP_CLOSURE: OpCode = 44;
/*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/
pub const OP_SETLIST: OpCode = 43;
/*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/
pub const OP_TFORLOOP: OpCode = 42;
/*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
pub const OP_TFORCALL: OpCode = 41;
/*	A sBx	R(A)-=R(A+2); pc+=sBx				*/
pub const OP_FORPREP: OpCode = 40;
/*	A sBx	R(A)+=R(A+2);
			if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
pub const OP_FORLOOP: OpCode = 39;
/*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/
pub const OP_RETURN: OpCode = 38;
/*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
pub const OP_TAILCALL: OpCode = 37;
/*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_CALL: OpCode = 36;
/*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/
pub const OP_TESTSET: OpCode = 35;
/*	A C	if not (R(A) <=> C) then pc++			*/
pub const OP_TEST: OpCode = 34;
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/*	A B C	R(A) := R(B).. ... ..R(C)			*/
pub const OP_CONCAT: OpCode = 29;
/*	A B	R(A) := length of R(B)				*/
pub const OP_LEN: OpCode = 28;
/*	A B	R(A) := not R(B)				*/
pub const OP_NOT: OpCode = 27;
/*	A B	R(A) := ~R(B)					*/
pub const OP_BNOT: OpCode = 26;
/*	A B	R(A) := -R(B)					*/
pub const OP_UNM: OpCode = 25;
/*	A B C	R(A) := RK(B) >> RK(C)				*/
pub const OP_SHR: OpCode = 24;
/*	A B C	R(A) := RK(B) << RK(C)				*/
pub const OP_SHL: OpCode = 23;
/*	A B C	R(A) := RK(B) ~ RK(C)				*/
pub const OP_BXOR: OpCode = 22;
/*	A B C	R(A) := RK(B) | RK(C)				*/
pub const OP_BOR: OpCode = 21;
/*	A B C	R(A) := RK(B) & RK(C)				*/
pub const OP_BAND: OpCode = 20;
/*	A B C	R(A) := RK(B) // RK(C)				*/
pub const OP_IDIV: OpCode = 19;
/*	A B C	R(A) := RK(B) / RK(C)				*/
pub const OP_DIV: OpCode = 18;
/*	A B C	R(A) := RK(B) ^ RK(C)				*/
pub const OP_POW: OpCode = 17;
/*	A B C	R(A) := RK(B) % RK(C)				*/
pub const OP_MOD: OpCode = 16;
/*	A B C	R(A) := RK(B) * RK(C)				*/
pub const OP_MUL: OpCode = 15;
/*	A B C	R(A) := RK(B) - RK(C)				*/
pub const OP_SUB: OpCode = 14;
/*	A B C	R(A) := RK(B) + RK(C)				*/
pub const OP_ADD: OpCode = 13;
/*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/
pub const OP_SELF: OpCode = 12;
/*	A B C	R(A) := {} (size = B,C)				*/
pub const OP_NEWTABLE: OpCode = 11;
/*	A B C	R(A)[RK(B)] := RK(C)				*/
pub const OP_SETTABLE: OpCode = 10;
/*	A B	UpValue[B] := R(A)				*/
pub const OP_SETUPVAL: OpCode = 9;
/*	A B C	UpValue[A][RK(B)] := RK(C)			*/
pub const OP_SETTABUP: OpCode = 8;
/*	A B C	R(A) := R(B)[RK(C)]				*/
pub const OP_GETTABLE: OpCode = 7;
/*	A B C	R(A) := UpValue[B][RK(C)]			*/
pub const OP_GETTABUP: OpCode = 6;
/*	A B	R(A) := UpValue[B]				*/
pub const OP_GETUPVAL: OpCode = 5;
/*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
pub const OP_LOADNIL: OpCode = 4;
/*	A B C	R(A) := (Bool)B; if (C) pc++			*/
pub const OP_LOADBOOL: OpCode = 3;
/*	A 	R(A) := Kst(extra arg)				*/
pub const OP_LOADKX: OpCode = 2;
/*	A Bx	R(A) := Kst(Bx)					*/
pub const OP_LOADK: OpCode = 1;
/*----------------------------------------------------------------------
name		args	description
------------------------------------------------------------------------*/
/*	A B	R(A) := R(B)					*/
pub const OP_MOVE: OpCode = 0;
pub type expkind = libc::c_uint;
/* vararg expression; info = instruction pc */
pub const VVARARG: expkind = 14;
/* expression is a function call; info = instruction pc */
pub const VCALL: expkind = 13;
/* expression can put result in any register;
                  info = instruction pc */
pub const VRELOCABLE: expkind = 12;
/* expression is a test/comparison;
            info = pc of corresponding jump instruction */
pub const VJMP: expkind = 11;
/* indexed variable;
                ind.vt = whether 't' is register or upvalue;
                ind.t = table register or upvalue;
                ind.idx = key's R/K index */
pub const VINDEXED: expkind = 10;
/* upvalue variable; info = index of upvalue in 'upvalues' */
pub const VUPVAL: expkind = 9;
/* local variable; info = local register */
pub const VLOCAL: expkind = 8;
/* expression has its value in a fixed register;
                 info = result register */
pub const VNONRELOC: expkind = 7;
/* integer constant; nval = numerical integer value */
pub const VKINT: expkind = 6;
/* floating constant; nval = numerical float value */
pub const VKFLT: expkind = 5;
/* constant in 'k'; info = index of constant in 'k' */
pub const VK: expkind = 4;
/* constant false */
pub const VFALSE: expkind = 3;
/* constant true */
pub const VTRUE: expkind = 2;
/* constant nil */
pub const VNIL: expkind = 1;
/* when 'expdesc' describes the last expression a list,
             this kind means an empty list (so, no expression) */
pub const VVOID: expkind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc {
    pub k: expkind,
    pub u: unnamed_7,
    pub t: libc::c_int,
    pub f: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_7 {
    ival: lua_Integer,
    nval: lua_Number,
    info: libc::c_int,
    ind: unnamed_8,
}
/* for indexed variables (VINDEXED) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_8 {
    pub idx: libc::c_short,
    pub t: lu_byte,
    pub vt: lu_byte,
}
pub type expdesc_0 = expdesc;
/* dynamic structures used by the parser */
pub type Dyndata_0 = Dyndata;
/* state needed to generate code for a given function */
pub type FuncState_0 = FuncState;
/*
** $Id: lparser.c,v 2.154 2016/06/22 15:48:25 roberto Exp roberto $
** Lua Parser
** See Copyright Notice in lua.h
*/
/* maximum number of local variables per function (must be smaller
   than 250, due to the bytecode format) */
/* because all strings are unified by the scanner, the parser
   can use pointer equality for string equality */
/*
** nodes for block list (list of active blocks)
*/
pub type BlockCnt_0 = BlockCnt;
/*
** structure to chain all variables in the left-hand side of an
** assignment
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LHS_assign {
    pub prev: *mut LHS_assign,
    pub v: expdesc_0,
}
/*
** $Id: lcode.h,v 1.63 2013/12/30 20:47:58 roberto Exp roberto $
** Code generator for Lua
** See Copyright Notice in lua.h
*/
/*
** Marks the end of a patch list. It is an invalid value both as an absolute
** address, and as a list link (would link an element to itself).
*/
/*
** grep "ORDER OPR" if you change these enums  (ORDER OP)
*/
pub type BinOpr = BinOpr_0;
pub type BinOpr_0 = libc::c_uint;
pub const OPR_NOBINOPR: BinOpr_0 = 21;
pub const OPR_OR: BinOpr_0 = 20;
pub const OPR_AND: BinOpr_0 = 19;
pub const OPR_GE: BinOpr_0 = 18;
pub const OPR_GT: BinOpr_0 = 17;
pub const OPR_NE: BinOpr_0 = 16;
pub const OPR_LE: BinOpr_0 = 15;
pub const OPR_LT: BinOpr_0 = 14;
pub const OPR_EQ: BinOpr_0 = 13;
pub const OPR_CONCAT: BinOpr_0 = 12;
pub const OPR_SHR: BinOpr_0 = 11;
pub const OPR_SHL: BinOpr_0 = 10;
pub const OPR_BXOR: BinOpr_0 = 9;
pub const OPR_BOR: BinOpr_0 = 8;
pub const OPR_BAND: BinOpr_0 = 7;
pub const OPR_IDIV: BinOpr_0 = 6;
pub const OPR_DIV: BinOpr_0 = 5;
pub const OPR_POW: BinOpr_0 = 4;
pub const OPR_MOD: BinOpr_0 = 3;
pub const OPR_MUL: BinOpr_0 = 2;
pub const OPR_SUB: BinOpr_0 = 1;
pub const OPR_ADD: BinOpr_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_9 {
    pub left: lu_byte,
    pub right: lu_byte,
}
/*
** {======================================================================
** Rules for Constructors
** =======================================================================
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConsControl {
    pub v: expdesc_0,
    pub t: *mut expdesc_0,
    pub nh: libc::c_int,
    pub na: libc::c_int,
    pub tostore: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn luaY_parser(
    mut L: *mut lua_State_0,
    mut z: *mut ZIO,
    mut buff: *mut Mbuffer_0,
    mut dyd: *mut Dyndata_0,
    mut name: *const libc::c_char,
    mut firstchar: libc::c_int,
) -> *mut LClosure_0 {
    let mut lexstate: LexState_0 = LexState {
        current: 0,
        linenumber: 0,
        lastline: 0,
        t: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        lookahead: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        fs: 0 as *mut FuncState,
        L: 0 as *mut lua_State,
        z: 0 as *mut ZIO,
        buff: 0 as *mut Mbuffer_0,
        h: 0 as *mut Table_0,
        dyd: 0 as *mut Dyndata,
        source: 0 as *mut TString,
        envn: 0 as *mut TString,
    };
    let mut funcstate: FuncState_0 = FuncState {
        f: 0 as *mut Proto_0,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        jpc: 0,
        nk: 0,
        np: 0,
        firstlocal: 0,
        nlocvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
    };
    /* create main closure */
    let mut cl: *mut LClosure_0 = luaF_newLclosure(L, 1i32);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut LClosure_0 = cl;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lparser.c\x00" as *const u8 as *const libc::c_char,
                      1632i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32
        || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lparser.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 1632i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 83],
                                                           &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
            };
            (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
                && (L.is_null()
                    || {
                        if 0 != (*io).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lparser.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              1632i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 83],
                                                                        &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
                        };
                        0 != ((*(*io).value_.gc).marked as libc::c_int
                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                            & ((*(*L).l_G).currentwhite as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                    })
        } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lparser.c\x00" as *const u8 as
                              *const libc::c_char, 1632i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 83],
                                                    &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
        };
    };
    /* anchor it (to avoid being collected) */
    luaD_inctop(L);
    /* create table for scanner */
    lexstate.h = luaH_new(L);
    let mut io_0: *mut TValue = (*L).top;
    let mut x__0: *mut Table_0 = lexstate.h;
    if (*x__0).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lparser.c\x00" as *const u8 as *const libc::c_char,
                      1635i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
    };
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
    (*io_0).tt_ = 5i32 | 1i32 << 6i32;
    if 0 == (*io_0).tt_ & 1i32 << 6i32
        || {
            if 0 != (*io_0).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lparser.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 1635i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 83],
                                                           &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
            };
            (*io_0).tt_ & 0x3fi32 == (*(*io_0).value_.gc).tt as libc::c_int
                && (L.is_null()
                    || {
                        if 0 != (*io_0).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lparser.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              1635i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 83],
                                                                        &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
                        };
                        0 != ((*(*io_0).value_.gc).marked as libc::c_int
                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                            & ((*(*L).l_G).currentwhite as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                    })
        } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lparser.c\x00" as *const u8 as
                              *const libc::c_char, 1635i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 83],
                                                    &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
        };
    };
    /* anchor it */
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    funcstate.f = (*cl).p;
    /* create and anchor TString */
    (*funcstate.f).source = luaS_new(L, name);
    if 0 != (*funcstate.f).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
    } else {
        __assert_fail(b"(((funcstate.f)->marked) & (((1<<(0)) | (1<<(1)))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lparser.c\x00" as *const u8 as *const libc::c_char,
                      1639i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
    };
    /* do not need barrier here */
    lexstate.buff = buff;
    lexstate.dyd = dyd;
    (*dyd).label.n = 0i32;
    (*dyd).gt.n = (*dyd).label.n;
    (*dyd).actvar.n = (*dyd).gt.n;
    luaX_setinput(L, &mut lexstate, z, (*funcstate.f).source, firstchar);
    mainfunc(&mut lexstate, &mut funcstate);
    if funcstate.prev.is_null() && funcstate.nups as libc::c_int == 1i32 && lexstate.fs.is_null() {
    } else {
        __assert_fail(b"!funcstate.prev && funcstate.nups == 1 && !lexstate.fs\x00"
                          as *const u8 as *const libc::c_char,
                      b"lparser.c\x00" as *const u8 as *const libc::c_char,
                      1645i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
    };
    if (*dyd).actvar.n == 0i32 && (*dyd).gt.n == 0i32 && (*dyd).label.n == 0i32 {
    } else {
        __assert_fail(b"dyd->actvar.n == 0 && dyd->gt.n == 0 && dyd->label.n == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lparser.c\x00" as *const u8 as *const libc::c_char,
                      1647i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"LClosure *luaY_parser(lua_State *, ZIO *, Mbuffer *, Dyndata *, const char *, int)\x00")).as_ptr());
    };
    /* all scopes should be correctly finished */
    /* remove scanner's table */
    (*L).top = (*L).top.offset(-1isize);
    /* closure is on the stack, too */
    return cl;
}
/* }====================================================================== */
/*
** compiles the main function, which is a regular vararg function with an
** upvalue named LUA_ENV
*/
unsafe extern "C" fn mainfunc(mut ls: *mut LexState_0, mut fs: *mut FuncState_0) -> () {
    let mut bl: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut v: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    open_func(ls, fs, &mut bl);
    /* main function is always declared vararg */
    (*(*fs).f).is_vararg = 1i32 as lu_byte;
    /* create and... */
    init_exp(&mut v, VLOCAL, 0i32);
    /* ...set environment upvalue */
    newupvalue(fs, (*ls).envn, &mut v);
    /* read first token */
    luaX_next(ls);
    /* parse main body */
    statlist(ls);
    check(ls, TK_EOS as libc::c_int);
    close_func(ls);
}
unsafe extern "C" fn close_func(mut ls: *mut LexState_0) -> () {
    let mut L: *mut lua_State_0 = (*ls).L;
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut f: *mut Proto_0 = (*fs).f;
    /* final return */
    luaK_ret(fs, 0i32, 0i32);
    leaveblock(fs);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).pc as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t))
                .wrapping_div(::std::mem::size_of::<Instruction>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).code = luaM_realloc_(
        L,
        (*f).code as *mut libc::c_void,
        ((*f).sizecode as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Instruction>() as libc::c_ulong),
        ((*fs).pc as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Instruction>() as libc::c_ulong),
    ) as *mut Instruction;
    (*f).sizecode = (*fs).pc;
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).pc as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t))
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).lineinfo = luaM_realloc_(
        L,
        (*f).lineinfo as *mut libc::c_void,
        ((*f).sizelineinfo as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ((*fs).pc as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*f).sizelineinfo = (*fs).pc;
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).nk as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).k = luaM_realloc_(
        L,
        (*f).k as *mut libc::c_void,
        ((*f).sizek as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
        ((*fs).nk as libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    (*f).sizek = (*fs).nk;
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).np as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t))
                .wrapping_div(::std::mem::size_of::<*mut Proto_0>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).p = luaM_realloc_(
        L,
        (*f).p as *mut libc::c_void,
        ((*f).sizep as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut Proto_0>() as libc::c_ulong),
        ((*fs).np as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut Proto_0>() as libc::c_ulong),
    ) as *mut *mut Proto_0;
    (*f).sizep = (*fs).np;
    if ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).nlocvars as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<LocVar_0>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).locvars = luaM_realloc_(
        L,
        (*f).locvars as *mut libc::c_void,
        ((*f).sizelocvars as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<LocVar_0>() as libc::c_ulong),
        ((*fs).nlocvars as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<LocVar_0>() as libc::c_ulong),
    ) as *mut LocVar_0;
    (*f).sizelocvars = (*fs).nlocvars as libc::c_int;
    if ::std::mem::size_of::<lu_byte>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && ((*fs).nups as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t))
                .wrapping_div(::std::mem::size_of::<Upvaldesc_0>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).upvalues = luaM_realloc_(
        L,
        (*f).upvalues as *mut libc::c_void,
        ((*f).sizeupvalues as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Upvaldesc_0>() as libc::c_ulong),
        ((*fs).nups as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Upvaldesc_0>() as libc::c_ulong),
    ) as *mut Upvaldesc_0;
    (*f).sizeupvalues = (*fs).nups as libc::c_int;
    if (*fs).bl.is_null() {
    } else {
        __assert_fail(
            b"fs->bl == ((void*)0)\x00" as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            571i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"void close_func(LexState *)\x00",
            )).as_ptr(),
        );
    };
    (*ls).fs = (*fs).prev;
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    };
}
unsafe extern "C" fn leaveblock(mut fs: *mut FuncState_0) -> () {
    let mut j: libc::c_int = 0;
    let mut bl: *mut BlockCnt_0 = (*fs).bl;
    let mut ls: *mut LexState_0 = (*fs).ls;
    if !(*bl).previous.is_null() && 0 != (*bl).upval as libc::c_int {
        /* create a 'jump to here' to close upvalues */
        j = luaK_jump(fs);
        luaK_patchclose(fs, j, (*bl).nactvar as libc::c_int);
        luaK_patchtohere(fs, j);
    }
    if 0 != (*bl).isloop {
        /* close pending breaks */
        breaklabel(ls);
    }
    (*fs).bl = (*bl).previous;
    removevars(fs, (*bl).nactvar as libc::c_int);
    if (*bl).nactvar as libc::c_int == (*fs).nactvar as libc::c_int {
    } else {
        __assert_fail(
            b"bl->nactvar == fs->nactvar\x00" as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            487i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"void leaveblock(FuncState *)\x00",
            )).as_ptr(),
        );
    };
    /* free registers */
    (*fs).freereg = (*fs).nactvar;
    /* remove local labels */
    (*(*ls).dyd).label.n = (*bl).firstlabel;
    /* inner block? */
    if !(*bl).previous.is_null() {
        /* update pending gotos to outer block */
        movegotosout(fs, bl);
    } else if (*bl).firstgoto < (*(*ls).dyd).gt.n {
        /* error */
        undefgoto(
            ls,
            &mut *(*(*ls).dyd).gt.arr.offset((*bl).firstgoto as isize),
        );
    };
}
/*
** generates an error for an undefined 'goto'; choose appropriate
** message when label name is a reserved word (which can only be 'break')
*/
unsafe extern "C" fn undefgoto(mut ls: *mut LexState_0, mut gt: *mut Labeldesc) -> ! {
    let mut msg: *const libc::c_char = if (*(*gt).name).tt as libc::c_int == 4i32 | 0i32 << 4i32
        && (*(*gt).name).extra as libc::c_int > 0i32
    {
        b"<%s> at line %d not inside a loop\x00" as *const u8 as *const libc::c_char
    } else {
        b"no visible label \'%s\' for <goto> at line %d\x00" as *const u8 as *const libc::c_char
    };
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof((gt->name)->extra)\x00" as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            469i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"void undefgoto(LexState *, Labeldesc *)\x00",
            )).as_ptr(),
        );
    };
    msg = luaO_pushfstring(
        (*ls).L,
        msg,
        ((*gt).name as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString_0>() as libc::c_ulong as isize),
        (*gt).line,
    );
    semerror(ls, msg);
}
/* semantic error */
unsafe extern "C" fn semerror(mut ls: *mut LexState_0, mut msg: *const libc::c_char) -> ! {
    /* remove "near <token>" from final message */
    (*ls).t.token = 0i32;
    luaX_syntaxerror(ls, msg);
}
/*
** export pending gotos to outer level, to check them against
** outer labels; if the block being exited has upvalues, and
** the goto exits the scope of any variable (which can be the
** upvalue), close those variables being exited.
*/
unsafe extern "C" fn movegotosout(mut fs: *mut FuncState_0, mut bl: *mut BlockCnt_0) -> () {
    let mut i: libc::c_int = (*bl).firstgoto;
    let mut gl: *mut Labellist = &mut (*(*(*fs).ls).dyd).gt;
    /* correct pending gotos to current block and try to close it
     with visible labels */
    while i < (*gl).n {
        let mut gt: *mut Labeldesc = &mut *(*gl).arr.offset(i as isize) as *mut Labeldesc;
        if (*gt).nactvar as libc::c_int > (*bl).nactvar as libc::c_int {
            if 0 != (*bl).upval {
                luaK_patchclose(fs, (*gt).pc, (*bl).nactvar as libc::c_int);
            }
            (*gt).nactvar = (*bl).nactvar
        }
        if !(0 == findlabel((*fs).ls, i)) {
            continue;
        }
        /* move to next one */
        i += 1
    }
}
/*
** try to close a goto with existing labels; this solves backward jumps
*/
unsafe extern "C" fn findlabel(mut ls: *mut LexState_0, mut g: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bl: *mut BlockCnt_0 = (*(*ls).fs).bl;
    let mut dyd: *mut Dyndata_0 = (*ls).dyd;
    let mut gt: *mut Labeldesc = &mut *(*dyd).gt.arr.offset(g as isize) as *mut Labeldesc;
    /* check labels in current block for a match */
    i = (*bl).firstlabel;
    while i < (*dyd).label.n {
        let mut lb: *mut Labeldesc = &mut *(*dyd).label.arr.offset(i as isize) as *mut Labeldesc;
        if (*lb).name == (*gt).name {
            /* correct label? */
            if (*gt).nactvar as libc::c_int > (*lb).nactvar as libc::c_int
                && (0 != (*bl).upval as libc::c_int || (*dyd).label.n > (*bl).firstlabel)
            {
                luaK_patchclose((*ls).fs, (*gt).pc, (*lb).nactvar as libc::c_int);
            }
            /* close it */
            closegoto(ls, g, lb);
            return 1i32;
        } else {
            i += 1
        }
    }
    /* label not found; cannot close goto */
    return 0i32;
}
unsafe extern "C" fn closegoto(
    mut ls: *mut LexState_0,
    mut g: libc::c_int,
    mut label: *mut Labeldesc,
) -> () {
    let mut i: libc::c_int = 0;
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut gt: *mut Labeldesc = &mut *(*gl).arr.offset(g as isize) as *mut Labeldesc;
    if (*gt).name == (*label).name {
    } else {
        __assert_fail(
            b"((gt->name) == (label->name))\x00" as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            347i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"void closegoto(LexState *, int, Labeldesc *)\x00",
            )).as_ptr(),
        );
    };
    if ((*gt).nactvar as libc::c_int) < (*label).nactvar as libc::c_int {
        let mut vname: *mut TString = (*getlocvar(fs, (*gt).nactvar as libc::c_int)).varname;
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((gt->name)->extra)\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                352i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void closegoto(LexState *, int, Labeldesc *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((vname)->extra)\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                352i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void closegoto(LexState *, int, Labeldesc *)\x00",
                )).as_ptr(),
            );
        };
        let mut msg: *const libc::c_char = luaO_pushfstring(
            (*ls).L,
            b"<goto %s> at line %d jumps into the scope of local \'%s\'\x00" as *const u8
                as *const libc::c_char,
            ((*gt).name as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString_0>() as libc::c_ulong as isize),
            (*gt).line,
            (vname as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString_0>() as libc::c_ulong as isize),
        );
        semerror(ls, msg);
    } else {
        luaK_patchlist(fs, (*gt).pc, (*label).pc);
        /* remove goto from pending list */
        i = g;
        while i < (*gl).n - 1i32 {
            *(*gl).arr.offset(i as isize) = *(*gl).arr.offset((i + 1i32) as isize);
            i += 1
        }
        (*gl).n -= 1;
        return;
    };
}
unsafe extern "C" fn getlocvar(mut fs: *mut FuncState_0, mut i: libc::c_int) -> *mut LocVar_0 {
    let mut idx: libc::c_int = (*(*(*(*fs).ls).dyd)
        .actvar
        .arr
        .offset(((*fs).firstlocal + i) as isize)).idx as libc::c_int;
    if idx < (*fs).nlocvars as libc::c_int {
    } else {
        __assert_fail(
            b"idx < fs->nlocvars\x00" as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            198i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"LocVar *getlocvar(FuncState *, int)\x00",
            )).as_ptr(),
        );
    };
    return &mut *(*(*fs).f).locvars.offset(idx as isize) as *mut LocVar_0;
}
unsafe extern "C" fn removevars(mut fs: *mut FuncState_0, mut tolevel: libc::c_int) -> () {
    (*(*(*fs).ls).dyd).actvar.n -= (*fs).nactvar as libc::c_int - tolevel;
    while (*fs).nactvar as libc::c_int > tolevel {
        (*fs).nactvar = (*fs).nactvar.wrapping_sub(1);
        (*getlocvar(fs, (*fs).nactvar as libc::c_int)).endpc = (*fs).pc
    }
}
/*
** create a label named 'break' to resolve break statements
*/
unsafe extern "C" fn breaklabel(mut ls: *mut LexState_0) -> () {
    let mut n: *mut TString = luaS_new((*ls).L, b"break\x00" as *const u8 as *const libc::c_char);
    let mut l: libc::c_int = newlabelentry(ls, &mut (*(*ls).dyd).label, n, 0i32, (*(*ls).fs).pc);
    findgotos(ls, &mut *(*(*ls).dyd).label.arr.offset(l as isize));
}
unsafe extern "C" fn newlabelentry(
    mut ls: *mut LexState_0,
    mut l: *mut Labellist,
    mut name: *mut TString,
    mut line: libc::c_int,
    mut pc: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = (*l).n;
    if n + 1i32 > (*l).size {
        (*l).arr = luaM_growaux_(
            (*ls).L,
            (*l).arr as *mut libc::c_void,
            &mut (*l).size,
            ::std::mem::size_of::<Labeldesc>() as libc::c_ulong,
            32767i32,
            b"labels/gotos\x00" as *const u8 as *const libc::c_char,
        ) as *mut Labeldesc
    }
    let ref mut fresh0 = (*(*l).arr.offset(n as isize)).name;
    *fresh0 = name;
    (*(*l).arr.offset(n as isize)).line = line;
    (*(*l).arr.offset(n as isize)).nactvar = (*(*ls).fs).nactvar;
    (*(*l).arr.offset(n as isize)).pc = pc;
    (*l).n = n + 1i32;
    return n;
}
/*
** check whether new label 'lb' matches any pending gotos in current
** block; solves forward jumps
*/
unsafe extern "C" fn findgotos(mut ls: *mut LexState_0, mut lb: *mut Labeldesc) -> () {
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut i: libc::c_int = (*(*(*ls).fs).bl).firstgoto;
    while i < (*gl).n {
        if (*(*gl).arr.offset(i as isize)).name == (*lb).name {
            closegoto(ls, i, lb);
        } else {
            i += 1
        }
    }
}
unsafe extern "C" fn check(mut ls: *mut LexState_0, mut c: libc::c_int) -> () {
    if (*ls).t.token != c {
        error_expected(ls, c);
    } else {
        return;
    };
}
unsafe extern "C" fn error_expected(mut ls: *mut LexState_0, mut token: libc::c_int) -> ! {
    luaX_syntaxerror(
        ls,
        luaO_pushfstring(
            (*ls).L,
            b"%s expected\x00" as *const u8 as *const libc::c_char,
            luaX_token2str(ls, token),
        ),
    );
}
unsafe extern "C" fn statlist(mut ls: *mut LexState_0) -> () {
    /* statlist -> { stat [';'] } */
    while 0 == block_follow(ls, 1i32) {
        if (*ls).t.token == TK_RETURN as libc::c_int {
            statement(ls);
            /* 'return' must be last statement */
            return;
        } else {
            statement(ls);
        }
    }
}
/*
** prototypes for recursive non-terminal functions
*/
unsafe extern "C" fn statement(mut ls: *mut LexState_0) -> () {
    /* may be needed for error messages */
    let mut line: libc::c_int = (*ls).linenumber;
    enterlevel(ls);
    match (*ls).t.token {
        59 => {
            /* stat -> ';' (empty statement) */
            /* skip ';' */
            luaX_next(ls);
        }
        267 => {
            /* stat -> ifstat */
            ifstat(ls, line);
        }
        278 => {
            /* stat -> whilestat */
            whilestat(ls, line);
        }
        259 => {
            /* stat -> DO block END */
            /* skip DO */
            luaX_next(ls);
            block(ls);
            check_match(ls, TK_END as libc::c_int, TK_DO as libc::c_int, line);
        }
        264 => {
            /* stat -> forstat */
            forstat(ls, line);
        }
        273 => {
            /* stat -> repeatstat */
            repeatstat(ls, line);
        }
        265 => {
            /* stat -> funcstat */
            funcstat(ls, line);
        }
        269 => {
            /* stat -> localstat */
            /* skip LOCAL */
            luaX_next(ls);
            /* local function? */
            if 0 != testnext(ls, TK_FUNCTION as libc::c_int) {
                localfunc(ls);
            } else {
                localstat(ls);
            }
        }
        288 => {
            /* stat -> label */
            /* skip double colon */
            luaX_next(ls);
            labelstat(ls, str_checkname(ls), line);
        }
        274 => {
            /* stat -> retstat */
            /* skip RETURN */
            luaX_next(ls);
            retstat(ls);
        }
        258 | 266 => {
            /* stat -> 'goto' NAME */
            gotostat(ls, luaK_jump((*ls).fs));
        }
        _ => {
            /* stat -> func | assignment */
            exprstat(ls);
        }
    }
    if (*(*(*ls).fs).f).maxstacksize as libc::c_int >= (*(*ls).fs).freereg as libc::c_int
        && (*(*ls).fs).freereg as libc::c_int >= (*(*ls).fs).nactvar as libc::c_int
    {
    } else {
        __assert_fail(
            b"ls->fs->f->maxstacksize >= ls->fs->freereg && ls->fs->freereg >= ls->fs->nactvar\x00"
                as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            1601i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void statement(LexState *)\x00",
            )).as_ptr(),
        );
    };
    /* free registers */
    (*(*ls).fs).freereg = (*(*ls).fs).nactvar;
    (*(*ls).L).nCcalls = (*(*ls).L).nCcalls.wrapping_sub(1);
}
unsafe extern "C" fn exprstat(mut ls: *mut LexState_0) -> () {
    /* stat -> func | assignment */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut v: LHS_assign = LHS_assign {
        prev: 0 as *mut LHS_assign,
        v: expdesc {
            k: VVOID,
            u: unnamed_7 { ival: 0 },
            t: 0,
            f: 0,
        },
    };
    suffixedexp(ls, &mut v.v);
    if (*ls).t.token == '=' as i32 || (*ls).t.token == ',' as i32 {
        /* stat -> assignment ? */
        v.prev = 0 as *mut LHS_assign;
        assignment(ls, &mut v, 1i32);
    } else if !(v.v.k as libc::c_uint == VCALL as libc::c_int as libc::c_uint) {
        luaX_syntaxerror(ls, b"syntax error\x00" as *const u8 as *const libc::c_char);
    } else {
        *(*(*fs).f).code.offset(v.v.u.info as isize) = *(*(*fs).f).code.offset(v.v.u.info as isize)
            & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32)
            | (1i32 as Instruction) << 0i32 + 6i32 + 8i32
                & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32
    };
}
unsafe extern "C" fn assignment(
    mut ls: *mut LexState_0,
    mut lh: *mut LHS_assign,
    mut nvars: libc::c_int,
) -> () {
    let mut e: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    if !(VLOCAL as libc::c_int as libc::c_uint <= (*lh).v.k as libc::c_uint
        && (*lh).v.k as libc::c_uint <= VINDEXED as libc::c_int as libc::c_uint)
    {
        luaX_syntaxerror(ls, b"syntax error\x00" as *const u8 as *const libc::c_char);
    } else {
        if 0 != testnext(ls, ',' as i32) {
            /* assignment -> ',' suffixedexp assignment */
            let mut nv: LHS_assign = LHS_assign {
                prev: 0 as *mut LHS_assign,
                v: expdesc {
                    k: VVOID,
                    u: unnamed_7 { ival: 0 },
                    t: 0,
                    f: 0,
                },
            };
            nv.prev = lh;
            suffixedexp(ls, &mut nv.v);
            if nv.v.k as libc::c_uint != VINDEXED as libc::c_int as libc::c_uint {
                check_conflict(ls, lh, &mut nv.v);
            }
            checklimit(
                (*ls).fs,
                nvars + (*(*ls).L).nCcalls as libc::c_int,
                200i32,
                b"C levels\x00" as *const u8 as *const libc::c_char,
            );
            assignment(ls, &mut nv, nvars + 1i32);
        } else {
            /* assignment -> '=' explist */
            let mut nexps: libc::c_int = 0;
            checknext(ls, '=' as i32);
            nexps = explist(ls, &mut e);
            if nexps != nvars {
                adjust_assign(ls, nvars, nexps, &mut e);
            } else {
                /* close last expression */
                luaK_setoneret((*ls).fs, &mut e);
                luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
                /* avoid default */
                return;
            }
        }
        /* default assignment */
        init_exp(&mut e, VNONRELOC, (*(*ls).fs).freereg as libc::c_int - 1i32);
        luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
        return;
    };
}
unsafe extern "C" fn init_exp(mut e: *mut expdesc_0, mut k: expkind, mut i: libc::c_int) -> () {
    (*e).t = -1i32;
    (*e).f = (*e).t;
    (*e).k = k;
    (*e).u.info = i;
}
unsafe extern "C" fn adjust_assign(
    mut ls: *mut LexState_0,
    mut nvars: libc::c_int,
    mut nexps: libc::c_int,
    mut e: *mut expdesc_0,
) -> () {
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut extra: libc::c_int = nvars - nexps;
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint
        || (*e).k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint
    {
        /* includes call itself */
        extra += 1;
        if extra < 0i32 {
            extra = 0i32
        }
        /* last exp. provides the difference */
        luaK_setreturns(fs, e, extra);
        if extra > 1i32 {
            luaK_reserveregs(fs, extra - 1i32);
        }
    } else {
        if (*e).k as libc::c_uint != VVOID as libc::c_int as libc::c_uint {
            /* close last expression */
            luaK_exp2nextreg(fs, e);
        }
        if extra > 0i32 {
            let mut reg: libc::c_int = (*fs).freereg as libc::c_int;
            luaK_reserveregs(fs, extra);
            luaK_nil(fs, reg, extra);
        }
    }
    if nexps > nvars {
        /* remove extra values */
        (*(*ls).fs).freereg = ((*(*ls).fs).freereg as libc::c_int - (nexps - nvars)) as lu_byte
    };
}
unsafe extern "C" fn explist(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> libc::c_int {
    /* explist -> expr { ',' expr } */
    /* at least one expression */
    let mut n: libc::c_int = 1i32;
    expr(ls, v);
    while 0 != testnext(ls, ',' as i32) {
        luaK_exp2nextreg((*ls).fs, v);
        expr(ls, v);
        n += 1
    }
    return n;
}
unsafe extern "C" fn expr(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> () {
    subexpr(ls, v, 0i32);
}
/* ORDER OPR */
/* '+' '-' */
/* '*' '%' */
/* '^' (right associative) */
/* '/' '//' */
/* '&' '|' '~' */
/* '<<' '>>' */
/* '..' (right associative) */
/* ==, <, <= */
/* ~=, >, >= */
/* and, or */
/* priority for unary operators */
/*
** subexpr -> (simpleexp | unop subexpr) { binop subexpr }
** where 'binop' is any binary operator with a priority higher than 'limit'
*/
unsafe extern "C" fn subexpr(
    mut ls: *mut LexState_0,
    mut v: *mut expdesc_0,
    mut limit: libc::c_int,
) -> BinOpr {
    let mut v2: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut line_0: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    let mut nextop: BinOpr = OPR_ADD;
    let mut op: BinOpr = OPR_ADD;
    let mut uop: UnOpr = OPR_MINUS;
    enterlevel(ls);
    uop = getunopr((*ls).t.token);
    if uop as libc::c_uint != OPR_NOUNOPR as libc::c_int as libc::c_uint {
        line = (*ls).linenumber;
        luaX_next(ls);
        subexpr(ls, v, 12i32);
        luaK_prefix((*ls).fs, uop, v, line);
    } else {
        simpleexp(ls, v);
    }
    /* expand while operators have priorities higher than 'limit' */
    op = getbinopr((*ls).t.token);
    while op as libc::c_uint != OPR_NOBINOPR as libc::c_int as libc::c_uint
        && priority[op as usize].left as libc::c_int > limit
    {
        v2 = expdesc {
            k: VVOID,
            u: unnamed_7 { ival: 0 },
            t: 0,
            f: 0,
        };
        nextop = OPR_ADD;
        line_0 = (*ls).linenumber;
        luaX_next(ls);
        luaK_infix((*ls).fs, op, v);
        /* read sub-expression with higher priority */
        nextop = subexpr(ls, &mut v2, priority[op as usize].right as libc::c_int);
        luaK_posfix((*ls).fs, op, v, &mut v2, line_0);
        op = nextop
    }
    (*(*ls).L).nCcalls = (*(*ls).L).nCcalls.wrapping_sub(1);
    /* return first untreated operator */
    return op;
}
static mut priority: [unnamed_9; 21] = unsafe {
    [
        unnamed_9 {
            left: 10i32 as lu_byte,
            right: 10i32 as lu_byte,
        },
        unnamed_9 {
            left: 10i32 as lu_byte,
            right: 10i32 as lu_byte,
        },
        unnamed_9 {
            left: 11i32 as lu_byte,
            right: 11i32 as lu_byte,
        },
        unnamed_9 {
            left: 11i32 as lu_byte,
            right: 11i32 as lu_byte,
        },
        unnamed_9 {
            left: 14i32 as lu_byte,
            right: 13i32 as lu_byte,
        },
        unnamed_9 {
            left: 11i32 as lu_byte,
            right: 11i32 as lu_byte,
        },
        unnamed_9 {
            left: 11i32 as lu_byte,
            right: 11i32 as lu_byte,
        },
        unnamed_9 {
            left: 6i32 as lu_byte,
            right: 6i32 as lu_byte,
        },
        unnamed_9 {
            left: 4i32 as lu_byte,
            right: 4i32 as lu_byte,
        },
        unnamed_9 {
            left: 5i32 as lu_byte,
            right: 5i32 as lu_byte,
        },
        unnamed_9 {
            left: 7i32 as lu_byte,
            right: 7i32 as lu_byte,
        },
        unnamed_9 {
            left: 7i32 as lu_byte,
            right: 7i32 as lu_byte,
        },
        unnamed_9 {
            left: 9i32 as lu_byte,
            right: 8i32 as lu_byte,
        },
        unnamed_9 {
            left: 3i32 as lu_byte,
            right: 3i32 as lu_byte,
        },
        unnamed_9 {
            left: 3i32 as lu_byte,
            right: 3i32 as lu_byte,
        },
        unnamed_9 {
            left: 3i32 as lu_byte,
            right: 3i32 as lu_byte,
        },
        unnamed_9 {
            left: 3i32 as lu_byte,
            right: 3i32 as lu_byte,
        },
        unnamed_9 {
            left: 3i32 as lu_byte,
            right: 3i32 as lu_byte,
        },
        unnamed_9 {
            left: 3i32 as lu_byte,
            right: 3i32 as lu_byte,
        },
        unnamed_9 {
            left: 2i32 as lu_byte,
            right: 2i32 as lu_byte,
        },
        unnamed_9 {
            left: 1i32 as lu_byte,
            right: 1i32 as lu_byte,
        },
    ]
};
unsafe extern "C" fn getbinopr(mut op: libc::c_int) -> BinOpr {
    match op {
        43 => return OPR_ADD,
        45 => return OPR_SUB,
        42 => return OPR_MUL,
        37 => return OPR_MOD,
        94 => return OPR_POW,
        47 => return OPR_DIV,
        279 => return OPR_IDIV,
        38 => return OPR_BAND,
        124 => return OPR_BOR,
        126 => return OPR_BXOR,
        286 => return OPR_SHL,
        287 => return OPR_SHR,
        280 => return OPR_CONCAT,
        285 => return OPR_NE,
        282 => return OPR_EQ,
        60 => return OPR_LT,
        284 => return OPR_LE,
        62 => return OPR_GT,
        283 => return OPR_GE,
        257 => return OPR_AND,
        272 => return OPR_OR,
        _ => return OPR_NOBINOPR,
    };
}
unsafe extern "C" fn simpleexp(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> () {
    /* simpleexp -> FLT | INT | STRING | NIL | TRUE | FALSE | ... |
                  constructor | FUNCTION body | suffixedexp */
    match (*ls).t.token {
        290 => {
            init_exp(v, VKFLT, 0i32);
            (*v).u.nval = (*ls).t.seminfo.r
        }
        291 => {
            init_exp(v, VKINT, 0i32);
            (*v).u.ival = (*ls).t.seminfo.i
        }
        293 => {
            codestring(ls, v, (*ls).t.seminfo.ts);
        }
        270 => {
            init_exp(v, VNIL, 0i32);
        }
        276 => {
            init_exp(v, VTRUE, 0i32);
        }
        263 => {
            init_exp(v, VFALSE, 0i32);
        }
        281 => {
            /* vararg */
            let mut fs: *mut FuncState_0 = (*ls).fs;
            if 0 == (*(*fs).f).is_vararg {
                luaX_syntaxerror(
                    ls,
                    b"cannot use \'...\' outside a vararg function\x00" as *const u8
                        as *const libc::c_char,
                );
            } else {
                init_exp(v, VVARARG, luaK_codeABC(fs, OP_VARARG, 0i32, 1i32, 0i32));
            }
        }
        123 => {
            /* constructor */
            constructor(ls, v);
            return;
        }
        265 => {
            luaX_next(ls);
            body(ls, v, 0i32, (*ls).linenumber);
            return;
        }
        _ => {
            suffixedexp(ls, v);
            return;
        }
    }
    luaX_next(ls);
}
unsafe extern "C" fn suffixedexp(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> () {
    /* suffixedexp ->
       primaryexp { '.' NAME | '[' exp ']' | ':' NAME funcargs | funcargs } */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut line: libc::c_int = (*ls).linenumber;
    primaryexp(ls, v);
    loop {
        match (*ls).t.token {
            46 => {
                /* fieldsel */
                fieldsel(ls, v);
            }
            91 => {
                /* '[' exp1 ']' */
                let mut key: expdesc_0 = expdesc {
                    k: VVOID,
                    u: unnamed_7 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaK_exp2anyregup(fs, v);
                yindex(ls, &mut key);
                luaK_indexed(fs, v, &mut key);
            }
            58 => {
                /* ':' NAME funcargs */
                let mut key_0: expdesc_0 = expdesc {
                    k: VVOID,
                    u: unnamed_7 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaX_next(ls);
                checkname(ls, &mut key_0);
                luaK_self(fs, v, &mut key_0);
                funcargs(ls, v, line);
            }
            40 | 293 | 123 => {
                /* funcargs */
                luaK_exp2nextreg(fs, v);
                funcargs(ls, v, line);
            }
            _ => return,
        }
    }
}
unsafe extern "C" fn funcargs(
    mut ls: *mut LexState_0,
    mut f: *mut expdesc_0,
    mut line: libc::c_int,
) -> () {
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut args: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut base: libc::c_int = 0;
    let mut nparams: libc::c_int = 0;
    match (*ls).t.token {
        40 => {
            /* funcargs -> '(' [ explist ] ')' */
            luaX_next(ls);
            /* arg list is empty? */
            if (*ls).t.token == ')' as i32 {
                args.k = VVOID
            } else {
                explist(ls, &mut args);
                luaK_setreturns(fs, &mut args, -1i32);
            }
            check_match(ls, ')' as i32, '(' as i32, line);
        }
        123 => {
            /* funcargs -> constructor */
            constructor(ls, &mut args);
        }
        293 => {
            /* funcargs -> STRING */
            codestring(ls, &mut args, (*ls).t.seminfo.ts);
            /* must use 'seminfo' before 'next' */
            luaX_next(ls);
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"function arguments expected\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*f).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"f->k == VNONRELOC\x00" as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            847i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"void funcargs(LexState *, expdesc *, int)\x00",
            )).as_ptr(),
        );
    };
    /* base register for call */
    base = (*f).u.info;
    if args.k as libc::c_uint == VCALL as libc::c_int as libc::c_uint
        || args.k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint
    {
        /* open call */
        nparams = -1i32
    } else {
        if args.k as libc::c_uint != VVOID as libc::c_int as libc::c_uint {
            /* close last argument */
            luaK_exp2nextreg(fs, &mut args);
        }
        nparams = (*fs).freereg as libc::c_int - (base + 1i32)
    }
    init_exp(
        f,
        VCALL,
        luaK_codeABC(fs, OP_CALL, base, nparams + 1i32, 2i32),
    );
    luaK_fixline(fs, line);
    /* call remove function and arguments and leaves
                            (unless changed) one result */
    (*fs).freereg = (base + 1i32) as lu_byte;
}
unsafe extern "C" fn codestring(
    mut ls: *mut LexState_0,
    mut e: *mut expdesc_0,
    mut s: *mut TString,
) -> () {
    init_exp(e, VK, luaK_stringK((*ls).fs, s));
}
unsafe extern "C" fn constructor(mut ls: *mut LexState_0, mut t: *mut expdesc_0) -> () {
    /* constructor -> '{' [ field { sep field } [sep] ] '}'
     sep -> ',' | ';' */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut line: libc::c_int = (*ls).linenumber;
    let mut pc: libc::c_int = luaK_codeABC(fs, OP_NEWTABLE, 0i32, 0i32, 0i32);
    let mut cc: ConsControl = ConsControl {
        v: expdesc {
            k: VVOID,
            u: unnamed_7 { ival: 0 },
            t: 0,
            f: 0,
        },
        t: 0 as *mut expdesc_0,
        nh: 0,
        na: 0,
        tostore: 0,
    };
    cc.tostore = 0i32;
    cc.nh = cc.tostore;
    cc.na = cc.nh;
    cc.t = t;
    init_exp(t, VRELOCABLE, pc);
    /* no value (yet) */
    init_exp(&mut cc.v, VVOID, 0i32);
    /* fix it at stack top */
    luaK_exp2nextreg((*ls).fs, t);
    checknext(ls, '{' as i32);
    loop {
        if cc.v.k as libc::c_uint == VVOID as libc::c_int as libc::c_uint || cc.tostore > 0i32 {
        } else {
            __assert_fail(
                b"cc.v.k == VVOID || cc.tostore > 0\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                739i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"void constructor(LexState *, expdesc *)\x00",
                )).as_ptr(),
            );
        };
        if (*ls).t.token == '}' as i32 {
            break;
        }
        closelistfield(fs, &mut cc);
        field(ls, &mut cc);
        if !(0 != testnext(ls, ',' as i32) || 0 != testnext(ls, ';' as i32)) {
            break;
        }
    }
    check_match(ls, '}' as i32, '{' as i32, line);
    lastlistfield(fs, &mut cc);
    *(*(*fs).f).code.offset(pc as isize) =
        *(*(*fs).f).code.offset(pc as isize)
            & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32)
            | (luaO_int2fb(cc.na as libc::c_uint) as Instruction) << 0i32 + 6i32 + 8i32 + 9i32
                & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32;
    *(*(*fs).f).code.offset(pc as isize) =
        *(*(*fs).f).code.offset(pc as isize)
            & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32)
            | (luaO_int2fb(cc.nh as libc::c_uint) as Instruction) << 0i32 + 6i32 + 8i32
                & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32;
}
unsafe extern "C" fn lastlistfield(mut fs: *mut FuncState_0, mut cc: *mut ConsControl) -> () {
    if (*cc).tostore == 0i32 {
        return;
    } else {
        if (*cc).v.k as libc::c_uint == VCALL as libc::c_int as libc::c_uint
            || (*cc).v.k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint
        {
            luaK_setreturns(fs, &mut (*cc).v, -1i32);
            luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, -1i32);
            /* do not count last expression (unknown number of elements) */
            (*cc).na -= 1
        } else {
            if (*cc).v.k as libc::c_uint != VVOID as libc::c_int as libc::c_uint {
                luaK_exp2nextreg(fs, &mut (*cc).v);
            }
            luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
        }
        return;
    };
}
unsafe extern "C" fn check_match(
    mut ls: *mut LexState_0,
    mut what: libc::c_int,
    mut who: libc::c_int,
    mut where_0: libc::c_int,
) -> () {
    if 0 == testnext(ls, what) {
        if where_0 == (*ls).linenumber {
            error_expected(ls, what);
        } else {
            luaX_syntaxerror(
                ls,
                luaO_pushfstring(
                    (*ls).L,
                    b"%s expected (to close %s at line %d)\x00" as *const u8 as *const libc::c_char,
                    luaX_token2str(ls, what),
                    luaX_token2str(ls, who),
                    where_0,
                ),
            );
        }
    } else {
        return;
    };
}
unsafe extern "C" fn testnext(mut ls: *mut LexState_0, mut c: libc::c_int) -> libc::c_int {
    if (*ls).t.token == c {
        luaX_next(ls);
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn field(mut ls: *mut LexState_0, mut cc: *mut ConsControl) -> () {
    /* field -> listfield | recfield */
    match (*ls).t.token {
        292 => {
            /* may be 'listfield' or 'recfield' */
            /* expression? */
            if luaX_lookahead(ls) != '=' as i32 {
                listfield(ls, cc);
            } else {
                recfield(ls, cc);
            }
        }
        91 => {
            recfield(ls, cc);
        }
        _ => {
            listfield(ls, cc);
        }
    };
}
unsafe extern "C" fn listfield(mut ls: *mut LexState_0, mut cc: *mut ConsControl) -> () {
    /* listfield -> exp */
    expr(ls, &mut (*cc).v);
    checklimit(
        (*ls).fs,
        (*cc).na,
        2147483647i32,
        b"items in a constructor\x00" as *const u8 as *const libc::c_char,
    );
    (*cc).na += 1;
    (*cc).tostore += 1;
}
unsafe extern "C" fn checklimit(
    mut fs: *mut FuncState_0,
    mut v: libc::c_int,
    mut l: libc::c_int,
    mut what: *const libc::c_char,
) -> () {
    if v > l {
        errorlimit(fs, l, what);
    } else {
        return;
    };
}
unsafe extern "C" fn errorlimit(
    mut fs: *mut FuncState_0,
    mut limit: libc::c_int,
    mut what: *const libc::c_char,
) -> ! {
    let mut L: *mut lua_State_0 = (*(*fs).ls).L;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: libc::c_int = (*(*fs).f).linedefined;
    let mut where_0: *const libc::c_char = if line == 0i32 {
        b"main function\x00" as *const u8 as *const libc::c_char
    } else {
        luaO_pushfstring(
            L,
            b"function at line %d\x00" as *const u8 as *const libc::c_char,
            line,
        )
    };
    msg = luaO_pushfstring(
        L,
        b"too many %s (limit is %d) in %s\x00" as *const u8 as *const libc::c_char,
        what,
        limit,
        where_0,
    );
    luaX_syntaxerror((*fs).ls, msg);
}
unsafe extern "C" fn recfield(mut ls: *mut LexState_0, mut cc: *mut ConsControl) -> () {
    /* recfield -> (NAME | '['exp1']') = exp1 */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut reg: libc::c_int = (*(*ls).fs).freereg as libc::c_int;
    let mut key: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut val: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut rkkey: libc::c_int = 0;
    if (*ls).t.token == TK_NAME as libc::c_int {
        checklimit(
            fs,
            (*cc).nh,
            2147483647i32,
            b"items in a constructor\x00" as *const u8 as *const libc::c_char,
        );
        checkname(ls, &mut key);
    } else {
        /* ls->t.token == '[' */
        yindex(ls, &mut key);
    }
    (*cc).nh += 1;
    checknext(ls, '=' as i32);
    rkkey = luaK_exp2RK(fs, &mut key);
    expr(ls, &mut val);
    luaK_codeABC(
        fs,
        OP_SETTABLE,
        (*(*cc).t).u.info,
        rkkey,
        luaK_exp2RK(fs, &mut val),
    );
    /* free registers */
    (*fs).freereg = reg as lu_byte;
}
unsafe extern "C" fn checknext(mut ls: *mut LexState_0, mut c: libc::c_int) -> () {
    check(ls, c);
    luaX_next(ls);
}
unsafe extern "C" fn yindex(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> () {
    /* index -> '[' expr ']' */
    /* skip the '[' */
    luaX_next(ls);
    expr(ls, v);
    luaK_exp2val((*ls).fs, v);
    checknext(ls, ']' as i32);
}
unsafe extern "C" fn checkname(mut ls: *mut LexState_0, mut e: *mut expdesc_0) -> () {
    codestring(ls, e, str_checkname(ls));
}
unsafe extern "C" fn str_checkname(mut ls: *mut LexState_0) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    check(ls, TK_NAME as libc::c_int);
    ts = (*ls).t.seminfo.ts;
    luaX_next(ls);
    return ts;
}
unsafe extern "C" fn closelistfield(mut fs: *mut FuncState_0, mut cc: *mut ConsControl) -> () {
    if (*cc).v.k as libc::c_uint == VVOID as libc::c_int as libc::c_uint {
        /* there is no list item */
        return;
    } else {
        luaK_exp2nextreg(fs, &mut (*cc).v);
        (*cc).v.k = VVOID;
        if (*cc).tostore == 50i32 {
            /* flush */
            luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
            /* no more items pending */
            (*cc).tostore = 0i32
        }
        return;
    };
}
unsafe extern "C" fn fieldsel(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> () {
    /* fieldsel -> ['.' | ':'] NAME */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut key: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    luaK_exp2anyregup(fs, v);
    /* skip the dot or colon */
    luaX_next(ls);
    checkname(ls, &mut key);
    luaK_indexed(fs, v, &mut key);
}
/*
** {======================================================================
** Expression parsing
** =======================================================================
*/
unsafe extern "C" fn primaryexp(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> () {
    /* primaryexp -> NAME | '(' expr ')' */
    match (*ls).t.token {
        40 => {
            let mut line: libc::c_int = (*ls).linenumber;
            luaX_next(ls);
            expr(ls, v);
            check_match(ls, ')' as i32, '(' as i32, line);
            luaK_dischargevars((*ls).fs, v);
            return;
        }
        292 => {
            singlevar(ls, v);
            return;
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"unexpected symbol\x00" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn singlevar(mut ls: *mut LexState_0, mut var: *mut expdesc_0) -> () {
    let mut varname: *mut TString = str_checkname(ls);
    let mut fs: *mut FuncState_0 = (*ls).fs;
    singlevaraux(fs, varname, var, 1i32);
    if (*var).k as libc::c_uint == VVOID as libc::c_int as libc::c_uint {
        /* global name? */
        let mut key: expdesc_0 = expdesc {
            k: VVOID,
            u: unnamed_7 { ival: 0 },
            t: 0,
            f: 0,
        };
        /* get environment variable */
        singlevaraux(fs, (*ls).envn, var, 1i32);
        if (*var).k as libc::c_uint != VVOID as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(
                b"var->k != VVOID\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                303i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                    b"void singlevar(LexState *, expdesc *)\x00",
                )).as_ptr(),
            );
        };
        /* this one must exist */
        /* key is variable name */
        codestring(ls, &mut key, varname);
        /* env[varname] */
        luaK_indexed(fs, var, &mut key);
    };
}
/*
  Find variable with given name 'n'. If it is an upvalue, add this
  upvalue into all intermediate functions.
*/
unsafe extern "C" fn singlevaraux(
    mut fs: *mut FuncState_0,
    mut n: *mut TString,
    mut var: *mut expdesc_0,
    mut base: libc::c_int,
) -> () {
    /* no more levels? */
    if fs.is_null() {
        /* default is global */
        init_exp(var, VVOID, 0i32);
    } else {
        /* look up locals at current level */
        let mut v: libc::c_int = searchvar(fs, n);
        if v >= 0i32 {
            /* found? */
            /* variable is local */
            init_exp(var, VLOCAL, v);
            if 0 == base {
                /* local will be used as an upval */
                markupval(fs, v);
            }
        } else {
            /* not found as local at current level; try upvalues */
            /* try existing upvalues */
            let mut idx: libc::c_int = searchupvalue(fs, n);
            if idx < 0i32 {
                /* not found? */
                /* try upper levels */
                singlevaraux((*fs).prev, n, var, 0i32);
                /* not found? */
                if (*var).k as libc::c_uint == VVOID as libc::c_int as libc::c_uint {
                    /* it is a global */
                    return;
                } else {
                    idx = newupvalue(fs, n, var)
                }
            }
            /* new or old upvalue */
            init_exp(var, VUPVAL, idx);
        }
    };
}
unsafe extern "C" fn searchupvalue(
    mut fs: *mut FuncState_0,
    mut name: *mut TString,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut up: *mut Upvaldesc_0 = (*(*fs).f).upvalues;
    i = 0i32;
    while i < (*fs).nups as libc::c_int {
        if (*up.offset(i as isize)).name == name {
            return i;
        } else {
            i += 1
        }
    }
    /* not found */
    return -1i32;
}
unsafe extern "C" fn newupvalue(
    mut fs: *mut FuncState_0,
    mut name: *mut TString,
    mut v: *mut expdesc_0,
) -> libc::c_int {
    let mut f: *mut Proto_0 = (*fs).f;
    let mut oldsize: libc::c_int = (*f).sizeupvalues;
    checklimit(
        fs,
        (*fs).nups as libc::c_int + 1i32,
        255i32,
        b"upvalues\x00" as *const u8 as *const libc::c_char,
    );
    if (*fs).nups as libc::c_int + 1i32 > (*f).sizeupvalues {
        (*f).upvalues = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).upvalues as *mut libc::c_void,
            &mut (*f).sizeupvalues,
            ::std::mem::size_of::<Upvaldesc_0>() as libc::c_ulong,
            255i32,
            b"upvalues\x00" as *const u8 as *const libc::c_char,
        ) as *mut Upvaldesc_0
    }
    while oldsize < (*f).sizeupvalues {
        let fresh1 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh2 = (*(*f).upvalues.offset(fresh1 as isize)).name;
        *fresh2 = 0 as *mut TString
    }
    (*(*f).upvalues.offset((*fs).nups as isize)).instack =
        ((*v).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint) as libc::c_int as lu_byte;
    (*(*f).upvalues.offset((*fs).nups as isize)).idx = (*v).u.info as lu_byte;
    let ref mut fresh3 = (*(*f).upvalues.offset((*fs).nups as isize)).name;
    *fresh3 = name;
    if 0 != (*f).marked as libc::c_int & 1i32 << 2i32
        && 0 != (*name).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        if (*f).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((f)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                240i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"int newupvalue(FuncState *, TString *, expdesc *)\x00",
                )).as_ptr(),
            );
        };
        if (*name).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((name)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                240i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"int newupvalue(FuncState *, TString *, expdesc *)\x00",
                )).as_ptr(),
            );
        };
        luaC_barrier_(
            (*(*fs).ls).L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(name as *mut GCUnion)).gc,
        );
    } else {
    };
    let fresh4 = (*fs).nups;
    (*fs).nups = (*fs).nups.wrapping_add(1);
    return fresh4 as libc::c_int;
}
unsafe extern "C" fn searchvar(mut fs: *mut FuncState_0, mut n: *mut TString) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = (*fs).nactvar as libc::c_int - 1i32;
    while i >= 0i32 {
        if n == (*getlocvar(fs, i)).varname {
            return i;
        } else {
            i -= 1
        }
    }
    /* not found */
    return -1i32;
}
/*
  Mark block where variable at given level was defined
  (to emit close instructions later).
*/
unsafe extern "C" fn markupval(mut fs: *mut FuncState_0, mut level: libc::c_int) -> () {
    let mut bl: *mut BlockCnt_0 = (*fs).bl;
    while (*bl).nactvar as libc::c_int > level {
        bl = (*bl).previous
    }
    (*bl).upval = 1i32 as lu_byte;
}
unsafe extern "C" fn body(
    mut ls: *mut LexState_0,
    mut e: *mut expdesc_0,
    mut ismethod: libc::c_int,
    mut line: libc::c_int,
) -> () {
    /* body ->  '(' parlist ')' block END */
    let mut new_fs: FuncState_0 = FuncState {
        f: 0 as *mut Proto_0,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        jpc: 0,
        nk: 0,
        np: 0,
        firstlocal: 0,
        nlocvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
    };
    let mut bl: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    new_fs.f = addprototype(ls);
    (*new_fs.f).linedefined = line;
    open_func(ls, &mut new_fs, &mut bl);
    checknext(ls, '(' as i32);
    if 0 != ismethod {
        new_localvarliteral_(
            ls,
            b"self\x00" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong),
        );
        /* create 'self' parameter */
        adjustlocalvars(ls, 1i32);
    }
    parlist(ls);
    checknext(ls, ')' as i32);
    statlist(ls);
    (*new_fs.f).lastlinedefined = (*ls).linenumber;
    check_match(ls, TK_END as libc::c_int, TK_FUNCTION as libc::c_int, line);
    codeclosure(ls, e);
    close_func(ls);
}
/*
** codes instruction to create new closure in parent function.
** The OP_CLOSURE instruction must use the last available register,
** so that, if it invokes the GC, the GC knows which registers
** are in use at that time.
*/
unsafe extern "C" fn codeclosure(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> () {
    let mut fs: *mut FuncState_0 = (*(*ls).fs).prev;
    init_exp(
        v,
        VRELOCABLE,
        luaK_codeABx(fs, OP_CLOSURE, 0i32, ((*fs).np - 1i32) as libc::c_uint),
    );
    /* fix it at the last register */
    luaK_exp2nextreg(fs, v);
}
/* set initial array size */
/* set initial table size */
/* }====================================================================== */
unsafe extern "C" fn parlist(mut ls: *mut LexState_0) -> () {
    /* parlist -> [ param { ',' param } ] */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut f: *mut Proto_0 = (*fs).f;
    let mut nparams: libc::c_int = 0i32;
    (*f).is_vararg = 0i32 as lu_byte;
    if (*ls).t.token != ')' as i32 {
        /* is 'parlist' not empty? */
        loop {
            match (*ls).t.token {
                292 => {
                    /* param -> NAME */
                    new_localvar(ls, str_checkname(ls));
                    nparams += 1
                }
                281 => {
                    /* param -> '...' */
                    luaX_next(ls);
                    /* declared vararg */
                    (*f).is_vararg = 1i32 as lu_byte
                }
                _ => {
                    luaX_syntaxerror(
                        ls,
                        b"<name> or \'...\' expected\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
            if !(0 == (*f).is_vararg && 0 != testnext(ls, ',' as i32)) {
                break;
            }
        }
    }
    adjustlocalvars(ls, nparams);
    (*f).numparams = (*fs).nactvar;
    /* reserve register for parameters */
    luaK_reserveregs(fs, (*fs).nactvar as libc::c_int);
}
unsafe extern "C" fn adjustlocalvars(mut ls: *mut LexState_0, mut nvars: libc::c_int) -> () {
    let mut fs: *mut FuncState_0 = (*ls).fs;
    (*fs).nactvar = ((*fs).nactvar as libc::c_int + nvars) as lu_byte;
    while 0 != nvars {
        (*getlocvar(fs, (*fs).nactvar as libc::c_int - nvars)).startpc = (*fs).pc;
        nvars -= 1
    }
}
unsafe extern "C" fn new_localvar(mut ls: *mut LexState_0, mut name: *mut TString) -> () {
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut dyd: *mut Dyndata_0 = (*ls).dyd;
    let mut reg: libc::c_int = registerlocalvar(ls, name);
    checklimit(
        fs,
        (*dyd).actvar.n + 1i32 - (*fs).firstlocal,
        200i32,
        b"local variables\x00" as *const u8 as *const libc::c_char,
    );
    if (*dyd).actvar.n + 1i32 + 1i32 > (*dyd).actvar.size {
        (*dyd).actvar.arr = luaM_growaux_(
            (*ls).L,
            (*dyd).actvar.arr as *mut libc::c_void,
            &mut (*dyd).actvar.size,
            ::std::mem::size_of::<Vardesc>() as libc::c_ulong,
            2147483647i32,
            b"local variables\x00" as *const u8 as *const libc::c_char,
        ) as *mut Vardesc
    }
    let fresh5 = (*dyd).actvar.n;
    (*dyd).actvar.n = (*dyd).actvar.n + 1;
    (*(*dyd).actvar.arr.offset(fresh5 as isize)).idx = reg as libc::c_short;
}
unsafe extern "C" fn registerlocalvar(
    mut ls: *mut LexState_0,
    mut varname: *mut TString,
) -> libc::c_int {
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut f: *mut Proto_0 = (*fs).f;
    let mut oldsize: libc::c_int = (*f).sizelocvars;
    if (*fs).nlocvars as libc::c_int + 1i32 > (*f).sizelocvars {
        (*f).locvars = luaM_growaux_(
            (*ls).L,
            (*f).locvars as *mut libc::c_void,
            &mut (*f).sizelocvars,
            ::std::mem::size_of::<LocVar_0>() as libc::c_ulong,
            32767i32,
            b"local variables\x00" as *const u8 as *const libc::c_char,
        ) as *mut LocVar_0
    }
    while oldsize < (*f).sizelocvars {
        let fresh6 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh7 = (*(*f).locvars.offset(fresh6 as isize)).varname;
        *fresh7 = 0 as *mut TString
    }
    let ref mut fresh8 = (*(*f).locvars.offset((*fs).nlocvars as isize)).varname;
    *fresh8 = varname;
    if 0 != (*f).marked as libc::c_int & 1i32 << 2i32
        && 0 != (*varname).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        if (*f).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((f)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                171i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"int registerlocalvar(LexState *, TString *)\x00",
                )).as_ptr(),
            );
        };
        if (*varname).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((varname)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                171i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"int registerlocalvar(LexState *, TString *)\x00",
                )).as_ptr(),
            );
        };
        luaC_barrier_(
            (*ls).L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(varname as *mut GCUnion)).gc,
        );
    } else {
    };
    let fresh9 = (*fs).nlocvars;
    (*fs).nlocvars = (*fs).nlocvars + 1;
    return fresh9 as libc::c_int;
}
unsafe extern "C" fn new_localvarliteral_(
    mut ls: *mut LexState_0,
    mut name: *const libc::c_char,
    mut sz: size_t,
) -> () {
    new_localvar(ls, luaX_newstring(ls, name, sz));
}
unsafe extern "C" fn open_func(
    mut ls: *mut LexState_0,
    mut fs: *mut FuncState_0,
    mut bl: *mut BlockCnt_0,
) -> () {
    let mut f: *mut Proto_0 = 0 as *mut Proto_0;
    /* linked list of funcstates */
    (*fs).prev = (*ls).fs;
    (*fs).ls = ls;
    (*ls).fs = fs;
    (*fs).pc = 0i32;
    (*fs).lasttarget = 0i32;
    (*fs).jpc = -1i32;
    (*fs).freereg = 0i32 as lu_byte;
    (*fs).nk = 0i32;
    (*fs).np = 0i32;
    (*fs).nups = 0i32 as lu_byte;
    (*fs).nlocvars = 0i32 as libc::c_short;
    (*fs).nactvar = 0i32 as lu_byte;
    (*fs).firstlocal = (*(*ls).dyd).actvar.n;
    (*fs).bl = 0 as *mut BlockCnt;
    f = (*fs).f;
    (*f).source = (*ls).source;
    /* registers 0/1 are always valid */
    (*f).maxstacksize = 2i32 as lu_byte;
    enterblock(fs, bl, 0i32 as lu_byte);
}
unsafe extern "C" fn enterblock(
    mut fs: *mut FuncState_0,
    mut bl: *mut BlockCnt_0,
    mut isloop: lu_byte,
) -> () {
    (*bl).isloop = isloop;
    (*bl).nactvar = (*fs).nactvar;
    (*bl).firstlabel = (*(*(*fs).ls).dyd).label.n;
    (*bl).firstgoto = (*(*(*fs).ls).dyd).gt.n;
    (*bl).upval = 0i32 as lu_byte;
    (*bl).previous = (*fs).bl;
    (*fs).bl = bl;
    if (*fs).freereg as libc::c_int == (*fs).nactvar as libc::c_int {
    } else {
        __assert_fail(
            b"fs->freereg == fs->nactvar\x00" as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            448i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"void enterblock(FuncState *, BlockCnt *, lu_byte)\x00",
            )).as_ptr(),
        );
    };
}
/*
** adds a new prototype into list of prototypes
*/
unsafe extern "C" fn addprototype(mut ls: *mut LexState_0) -> *mut Proto_0 {
    let mut clp: *mut Proto_0 = 0 as *mut Proto_0;
    let mut L: *mut lua_State_0 = (*ls).L;
    let mut fs: *mut FuncState_0 = (*ls).fs;
    /* prototype of current function */
    let mut f: *mut Proto_0 = (*fs).f;
    if (*fs).np >= (*f).sizep {
        let mut oldsize: libc::c_int = (*f).sizep;
        if (*fs).np + 1i32 > (*f).sizep {
            (*f).p = luaM_growaux_(
                L,
                (*f).p as *mut libc::c_void,
                &mut (*f).sizep,
                ::std::mem::size_of::<*mut Proto_0>() as libc::c_ulong,
                (1i32 << 9i32 + 9i32) - 1i32,
                b"functions\x00" as *const u8 as *const libc::c_char,
            ) as *mut *mut Proto_0
        }
        while oldsize < (*f).sizep {
            let fresh10 = oldsize;
            oldsize = oldsize + 1;
            let ref mut fresh11 = *(*f).p.offset(fresh10 as isize);
            *fresh11 = 0 as *mut Proto
        }
    }
    let fresh12 = (*fs).np;
    (*fs).np = (*fs).np + 1;
    let ref mut fresh13 = *(*f).p.offset(fresh12 as isize);
    clp = luaF_newproto(L);
    *fresh13 = clp;
    if 0 != (*f).marked as libc::c_int & 1i32 << 2i32
        && 0 != (*clp).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        if (*f).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((f)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                512i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"Proto *addprototype(LexState *)\x00",
                )).as_ptr(),
            );
        };
        if (*clp).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((clp)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lparser.c\x00" as *const u8 as *const libc::c_char,
                512i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"Proto *addprototype(LexState *)\x00",
                )).as_ptr(),
            );
        };
        luaC_barrier_(
            L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(clp as *mut GCUnion)).gc,
        );
    } else {
    };
    return clp;
}
unsafe extern "C" fn getunopr(mut op: libc::c_int) -> UnOpr {
    match op {
        271 => return OPR_NOT,
        45 => return OPR_MINUS,
        126 => return OPR_BNOT,
        35 => return OPR_LEN,
        _ => return OPR_NOUNOPR,
    };
}
unsafe extern "C" fn enterlevel(mut ls: *mut LexState_0) -> () {
    let mut L: *mut lua_State_0 = (*ls).L;
    (*L).nCcalls = (*L).nCcalls.wrapping_add(1);
    checklimit(
        (*ls).fs,
        (*L).nCcalls as libc::c_int,
        200i32,
        b"C levels\x00" as *const u8 as *const libc::c_char,
    );
}
/*
** check whether, in an assignment to an upvalue/local variable, the
** upvalue/local variable is begin used in a previous assignment to a
** table. If so, save original upvalue/local value in a safe place and
** use this safe copy in the previous assignment.
*/
unsafe extern "C" fn check_conflict(
    mut ls: *mut LexState_0,
    mut lh: *mut LHS_assign,
    mut v: *mut expdesc_0,
) -> () {
    let mut fs: *mut FuncState_0 = (*ls).fs;
    /* eventual position to save local variable */
    let mut extra: libc::c_int = (*fs).freereg as libc::c_int;
    let mut conflict: libc::c_int = 0i32;
    while !lh.is_null() {
        /* check all previous assignments */
        if (*lh).v.k as libc::c_uint == VINDEXED as libc::c_int as libc::c_uint {
            /* assigning to a table? */
            /* table is the upvalue/local being assigned now? */
            if (*lh).v.u.ind.vt as libc::c_uint == (*v).k as libc::c_uint
                && (*lh).v.u.ind.t as libc::c_int == (*v).u.info
            {
                conflict = 1i32;
                (*lh).v.u.ind.vt = VLOCAL as libc::c_int as lu_byte;
                /* previous assignment will use safe copy */
                (*lh).v.u.ind.t = extra as lu_byte
            }
            /* index is the local being assigned? (index cannot be upvalue) */
            if (*v).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint
                && (*lh).v.u.ind.idx as libc::c_int == (*v).u.info
            {
                conflict = 1i32;
                /* previous assignment will use safe copy */
                (*lh).v.u.ind.idx = extra as libc::c_short
            }
        }
        lh = (*lh).prev
    }
    if 0 != conflict {
        /* copy upvalue/local value to a temporary (in position 'extra') */
        let mut op: OpCode = (if (*v).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint {
            OP_MOVE as libc::c_int
        } else {
            OP_GETUPVAL as libc::c_int
        }) as OpCode;
        luaK_codeABC(fs, op, extra, (*v).u.info, 0i32);
        luaK_reserveregs(fs, 1i32);
    };
}
unsafe extern "C" fn gotostat(mut ls: *mut LexState_0, mut pc: libc::c_int) -> () {
    let mut line: libc::c_int = (*ls).linenumber;
    let mut label: *mut TString = 0 as *mut TString;
    let mut g: libc::c_int = 0;
    if 0 != testnext(ls, TK_GOTO as libc::c_int) {
        label = str_checkname(ls)
    } else {
        /* skip break */
        luaX_next(ls);
        label = luaS_new((*ls).L, b"break\x00" as *const u8 as *const libc::c_char)
    }
    g = newlabelentry(ls, &mut (*(*ls).dyd).gt, label, line, pc);
    /* close it if label already defined */
    findlabel(ls, g);
}
/* call statement uses no results */
unsafe extern "C" fn retstat(mut ls: *mut LexState_0) -> () {
    /* stat -> RETURN [explist] [';'] */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut e: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* registers with returned values */
    let mut first: libc::c_int = 0;
    let mut nret: libc::c_int = 0;
    if 0 != block_follow(ls, 1i32) || (*ls).t.token == ';' as i32 {
        /* return no values */
        nret = 0i32;
        first = nret
    } else {
        /* optional return values */
        nret = explist(ls, &mut e);
        if e.k as libc::c_uint == VCALL as libc::c_int as libc::c_uint
            || e.k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint
        {
            luaK_setreturns(fs, &mut e, -1i32);
            if e.k as libc::c_uint == VCALL as libc::c_int as libc::c_uint && nret == 1i32 {
                /* tail call? */
                *(*(*fs).f).code.offset(e.u.info as isize) =
                    *(*(*fs).f).code.offset(e.u.info as isize)
                        & !(!((!(0i32 as Instruction)) << 6i32) << 0i32)
                        | (OP_TAILCALL as libc::c_int as Instruction) << 0i32
                            & !((!(0i32 as Instruction)) << 6i32) << 0i32;
                if (*(*(*fs).f).code.offset(e.u.info as isize) >> 0i32 + 6i32
                    & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int
                    == (*fs).nactvar as libc::c_int
                {
                } else {
                    __assert_fail(b"(((int)(((((fs)->f->code[(&e)->u.info]))>>(0 + 6)) & ((~((~(Instruction)0)<<(8)))<<(0))))) == fs->nactvar\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lparser.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1518i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 25],
                                                            &[libc::c_char; 25]>(b"void retstat(LexState *)\x00")).as_ptr());
                };
            }
            first = (*fs).nactvar as libc::c_int;
            /* return all values */
            nret = -1i32
        } else if nret == 1i32 {
            first = luaK_exp2anyreg(fs, &mut e)
        } else {
            /* values must go to the stack */
            luaK_exp2nextreg(fs, &mut e);
            /* return all active values */
            first = (*fs).nactvar as libc::c_int;
            if nret == (*fs).freereg as libc::c_int - first {
            } else {
                __assert_fail(
                    b"nret == fs->freereg - first\x00" as *const u8 as *const libc::c_char,
                    b"lparser.c\x00" as *const u8 as *const libc::c_char,
                    1529i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                        b"void retstat(LexState *)\x00",
                    )).as_ptr(),
                );
            };
        }
    }
    luaK_ret(fs, first, nret);
    /* skip optional semicolon */
    testnext(ls, ';' as i32);
}
/*============================================================*/
/* GRAMMAR RULES */
/*============================================================*/
/*
** check whether current token is in the follow set of a block.
** 'until' closes syntactical blocks, but do not close scope,
** so it is handled in separate.
*/
unsafe extern "C" fn block_follow(
    mut ls: *mut LexState_0,
    mut withuntil: libc::c_int,
) -> libc::c_int {
    match (*ls).t.token {
        260 | 261 | 262 | 289 => return 1i32,
        277 => return withuntil,
        _ => return 0i32,
    };
}
unsafe extern "C" fn labelstat(
    mut ls: *mut LexState_0,
    mut label: *mut TString,
    mut line: libc::c_int,
) -> () {
    /* label -> '::' NAME '::' */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut ll: *mut Labellist = &mut (*(*ls).dyd).label;
    /* index of new label being created */
    let mut l: libc::c_int = 0;
    /* check for repeated labels */
    checkrepeated(fs, ll, label);
    /* skip double colon */
    checknext(ls, TK_DBCOLON as libc::c_int);
    /* create new entry for this label */
    l = newlabelentry(ls, ll, label, line, luaK_getlabel(fs));
    /* skip other no-op statements */
    skipnoopstat(ls);
    if 0 != block_follow(ls, 0i32) {
        /* label is last no-op statement in the block? */
        /* assume that locals are already out of scope */
        (*(*ll).arr.offset(l as isize)).nactvar = (*(*fs).bl).nactvar
    }
    findgotos(ls, &mut *(*ll).arr.offset(l as isize));
}
/* skip no-op statements */
unsafe extern "C" fn skipnoopstat(mut ls: *mut LexState_0) -> () {
    while (*ls).t.token == ';' as i32 || (*ls).t.token == TK_DBCOLON as libc::c_int {
        statement(ls);
    }
}
/* check for repeated labels on the same block */
unsafe extern "C" fn checkrepeated(
    mut fs: *mut FuncState_0,
    mut ll: *mut Labellist,
    mut label: *mut TString,
) -> () {
    let mut i: libc::c_int = 0;
    i = (*(*fs).bl).firstlabel;
    while i < (*ll).n {
        if label == (*(*ll).arr.offset(i as isize)).name {
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(
                    b"sizeof((label)->extra)\x00" as *const u8 as *const libc::c_char,
                    b"lparser.c\x00" as *const u8 as *const libc::c_char,
                    1210i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                        b"void checkrepeated(FuncState *, Labellist *, TString *)\x00",
                    )).as_ptr(),
                );
            };
            let mut msg: *const libc::c_char = luaO_pushfstring(
                (*(*fs).ls).L,
                b"label \'%s\' already defined on line %d\x00" as *const u8 as *const libc::c_char,
                (label as *mut libc::c_char)
                    .offset(::std::mem::size_of::<UTString_0>() as libc::c_ulong as isize),
                (*(*ll).arr.offset(i as isize)).line,
            );
            semerror((*fs).ls, msg);
        } else {
            i += 1
        }
    }
}
unsafe extern "C" fn localstat(mut ls: *mut LexState_0) -> () {
    /* stat -> LOCAL NAME {',' NAME} ['=' explist] */
    let mut nvars: libc::c_int = 0i32;
    let mut nexps: libc::c_int = 0;
    let mut e: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    loop {
        new_localvar(ls, str_checkname(ls));
        nvars += 1;
        if !(0 != testnext(ls, ',' as i32)) {
            break;
        }
    }
    if 0 != testnext(ls, '=' as i32) {
        nexps = explist(ls, &mut e)
    } else {
        e.k = VVOID;
        nexps = 0i32
    }
    adjust_assign(ls, nvars, nexps, &mut e);
    adjustlocalvars(ls, nvars);
}
unsafe extern "C" fn localfunc(mut ls: *mut LexState_0) -> () {
    let mut b: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut fs: *mut FuncState_0 = (*ls).fs;
    /* new local variable */
    new_localvar(ls, str_checkname(ls));
    /* enter its scope */
    adjustlocalvars(ls, 1i32);
    /* function created in next register */
    body(ls, &mut b, 0i32, (*ls).linenumber);
    /* debug information will only see the variable after this point! */
    (*getlocvar(fs, b.u.info)).startpc = (*fs).pc;
}
unsafe extern "C" fn funcstat(mut ls: *mut LexState_0, mut line: libc::c_int) -> () {
    /* funcstat -> FUNCTION funcname body */
    let mut ismethod: libc::c_int = 0;
    let mut v: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut b: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* skip FUNCTION */
    luaX_next(ls);
    ismethod = funcname(ls, &mut v);
    body(ls, &mut b, ismethod, line);
    luaK_storevar((*ls).fs, &mut v, &mut b);
    /* definition "happens" in the first line */
    luaK_fixline((*ls).fs, line);
}
unsafe extern "C" fn funcname(mut ls: *mut LexState_0, mut v: *mut expdesc_0) -> libc::c_int {
    /* funcname -> NAME {fieldsel} [':' NAME] */
    let mut ismethod: libc::c_int = 0i32;
    singlevar(ls, v);
    while (*ls).t.token == '.' as i32 {
        fieldsel(ls, v);
    }
    if (*ls).t.token == ':' as i32 {
        ismethod = 1i32;
        fieldsel(ls, v);
    }
    return ismethod;
}
unsafe extern "C" fn repeatstat(mut ls: *mut LexState_0, mut line: libc::c_int) -> () {
    /* repeatstat -> REPEAT block UNTIL cond */
    let mut condexit: libc::c_int = 0;
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut repeat_init: libc::c_int = luaK_getlabel(fs);
    let mut bl1: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut bl2: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    /* loop block */
    enterblock(fs, &mut bl1, 1i32 as lu_byte);
    /* scope block */
    enterblock(fs, &mut bl2, 0i32 as lu_byte);
    /* skip REPEAT */
    luaX_next(ls);
    statlist(ls);
    check_match(ls, TK_UNTIL as libc::c_int, TK_REPEAT as libc::c_int, line);
    /* read condition (inside scope block) */
    condexit = cond(ls);
    /* upvalues? */
    if 0 != bl2.upval {
        luaK_patchclose(fs, condexit, bl2.nactvar as libc::c_int);
    }
    /* finish scope */
    leaveblock(fs);
    /* close the loop */
    luaK_patchlist(fs, condexit, repeat_init);
    /* finish loop */
    leaveblock(fs);
}
unsafe extern "C" fn cond(mut ls: *mut LexState_0) -> libc::c_int {
    /* cond -> exp */
    let mut v: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* read condition */
    expr(ls, &mut v);
    if v.k as libc::c_uint == VNIL as libc::c_int as libc::c_uint {
        /* 'falses' are all equal here */
        v.k = VFALSE
    }
    luaK_goiftrue((*ls).fs, &mut v);
    return v.f;
}
unsafe extern "C" fn forstat(mut ls: *mut LexState_0, mut line: libc::c_int) -> () {
    /* forstat -> FOR (fornum | forlist) END */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut varname: *mut TString = 0 as *mut TString;
    let mut bl: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    /* scope for loop and control variables */
    enterblock(fs, &mut bl, 1i32 as lu_byte);
    /* skip 'for' */
    luaX_next(ls);
    /* first variable name */
    varname = str_checkname(ls);
    match (*ls).t.token {
        61 => {
            fornum(ls, varname, line);
        }
        44 | 268 => {
            forlist(ls, varname);
        }
        _ => {
            luaX_syntaxerror(
                ls,
                b"\'=\' or \'in\' expected\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    check_match(ls, TK_END as libc::c_int, TK_FOR as libc::c_int, line);
    /* loop scope ('break' jumps to this point) */
    leaveblock(fs);
}
unsafe extern "C" fn forlist(mut ls: *mut LexState_0, mut indexname: *mut TString) -> () {
    /* forlist -> NAME {,NAME} IN explist forbody */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut e: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* gen, state, control, plus at least one declared var */
    let mut nvars: libc::c_int = 4i32;
    let mut line: libc::c_int = 0;
    let mut base: libc::c_int = (*fs).freereg as libc::c_int;
    new_localvarliteral_(
        ls,
        b"(for generator)\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    new_localvarliteral_(
        ls,
        b"(for state)\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    new_localvarliteral_(
        ls,
        b"(for control)\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    /* create control variables */
    /* create declared variables */
    new_localvar(ls, indexname);
    while 0 != testnext(ls, ',' as i32) {
        new_localvar(ls, str_checkname(ls));
        nvars += 1
    }
    checknext(ls, TK_IN as libc::c_int);
    line = (*ls).linenumber;
    adjust_assign(ls, 3i32, explist(ls, &mut e), &mut e);
    /* extra space to call generator */
    luaK_checkstack(fs, 3i32);
    forbody(ls, base, line, nvars - 3i32, 0i32);
}
unsafe extern "C" fn forbody(
    mut ls: *mut LexState_0,
    mut base: libc::c_int,
    mut line: libc::c_int,
    mut nvars: libc::c_int,
    mut isnum: libc::c_int,
) -> () {
    /* forbody -> DO block */
    let mut bl: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut prep: libc::c_int = 0;
    let mut endfor: libc::c_int = 0;
    /* control variables */
    adjustlocalvars(ls, 3i32);
    checknext(ls, TK_DO as libc::c_int);
    prep = if 0 != isnum {
        luaK_codeABx(
            fs,
            OP_FORPREP,
            base,
            (-1i32 + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as libc::c_uint,
        )
    } else {
        luaK_jump(fs)
    };
    /* scope for declared variables */
    enterblock(fs, &mut bl, 0i32 as lu_byte);
    adjustlocalvars(ls, nvars);
    luaK_reserveregs(fs, nvars);
    block(ls);
    /* end of scope for declared variables */
    leaveblock(fs);
    luaK_patchtohere(fs, prep);
    /* numeric for? */
    if 0 != isnum {
        endfor = luaK_codeABx(
            fs,
            OP_FORLOOP,
            base,
            (-1i32 + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as libc::c_uint,
        )
    } else {
        /* generic for */
        luaK_codeABC(fs, OP_TFORCALL, base, 0i32, nvars);
        luaK_fixline(fs, line);
        endfor = luaK_codeABx(
            fs,
            OP_TFORLOOP,
            base + 2i32,
            (-1i32 + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as libc::c_uint,
        )
    }
    luaK_patchlist(fs, endfor, prep + 1i32);
    luaK_fixline(fs, line);
}
/* }==================================================================== */
/*
** {======================================================================
** Rules for Statements
** =======================================================================
*/
unsafe extern "C" fn block(mut ls: *mut LexState_0) -> () {
    /* block -> statlist */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut bl: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    enterblock(fs, &mut bl, 0i32 as lu_byte);
    statlist(ls);
    leaveblock(fs);
}
unsafe extern "C" fn fornum(
    mut ls: *mut LexState_0,
    mut varname: *mut TString,
    mut line: libc::c_int,
) -> () {
    /* fornum -> NAME = exp1,exp1[,exp1] forbody */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut base: libc::c_int = (*fs).freereg as libc::c_int;
    new_localvarliteral_(
        ls,
        b"(for index)\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    new_localvarliteral_(
        ls,
        b"(for limit)\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    new_localvarliteral_(
        ls,
        b"(for step)\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    new_localvar(ls, varname);
    checknext(ls, '=' as i32);
    /* initial value */
    exp1(ls);
    checknext(ls, ',' as i32);
    /* limit */
    exp1(ls);
    if 0 != testnext(ls, ',' as i32) {
        /* optional step */
        exp1(ls);
    } else {
        /* default step = 1 */
        luaK_codek(
            fs,
            (*fs).freereg as libc::c_int,
            luaK_intK(fs, 1i32 as lua_Integer),
        );
        luaK_reserveregs(fs, 1i32);
    }
    forbody(ls, base, line, 1i32, 1i32);
}
unsafe extern "C" fn exp1(mut ls: *mut LexState_0) -> libc::c_int {
    let mut e: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut reg: libc::c_int = 0;
    expr(ls, &mut e);
    luaK_exp2nextreg((*ls).fs, &mut e);
    if e.k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"e.k == VNONRELOC\x00" as *const u8 as *const libc::c_char,
            b"lparser.c\x00" as *const u8 as *const libc::c_char,
            1286i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"int exp1(LexState *)\x00"))
                .as_ptr(),
        );
    };
    reg = e.u.info;
    return reg;
}
unsafe extern "C" fn whilestat(mut ls: *mut LexState_0, mut line: libc::c_int) -> () {
    /* whilestat -> WHILE cond DO block END */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut whileinit: libc::c_int = 0;
    let mut condexit: libc::c_int = 0;
    let mut bl: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    /* skip WHILE */
    luaX_next(ls);
    whileinit = luaK_getlabel(fs);
    condexit = cond(ls);
    enterblock(fs, &mut bl, 1i32 as lu_byte);
    checknext(ls, TK_DO as libc::c_int);
    block(ls);
    luaK_patchlist(fs, luaK_jump(fs), whileinit);
    check_match(ls, TK_END as libc::c_int, TK_WHILE as libc::c_int, line);
    leaveblock(fs);
    /* false conditions finish the loop */
    luaK_patchtohere(fs, condexit);
}
unsafe extern "C" fn ifstat(mut ls: *mut LexState_0, mut line: libc::c_int) -> () {
    /* ifstat -> IF cond THEN block {ELSEIF cond THEN block} [ELSE block] END */
    let mut fs: *mut FuncState_0 = (*ls).fs;
    /* exit list for finished parts */
    let mut escapelist: libc::c_int = -1i32;
    /* IF cond THEN block */
    test_then_block(ls, &mut escapelist);
    while (*ls).t.token == TK_ELSEIF as libc::c_int {
        /* ELSEIF cond THEN block */
        test_then_block(ls, &mut escapelist);
    }
    if 0 != testnext(ls, TK_ELSE as libc::c_int) {
        /* 'else' part */
        block(ls);
    }
    check_match(ls, TK_END as libc::c_int, TK_IF as libc::c_int, line);
    /* patch escape list to 'if' end */
    luaK_patchtohere(fs, escapelist);
}
unsafe extern "C" fn test_then_block(
    mut ls: *mut LexState_0,
    mut escapelist: *mut libc::c_int,
) -> () {
    /* test_then_block -> [IF | ELSEIF] cond THEN block */
    let mut bl: BlockCnt_0 = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut fs: *mut FuncState_0 = (*ls).fs;
    let mut v: expdesc_0 = expdesc {
        k: VVOID,
        u: unnamed_7 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* instruction to skip 'then' code (if condition is false) */
    let mut jf: libc::c_int = 0;
    /* skip IF or ELSEIF */
    luaX_next(ls);
    /* read condition */
    expr(ls, &mut v);
    checknext(ls, TK_THEN as libc::c_int);
    if (*ls).t.token == TK_GOTO as libc::c_int || (*ls).t.token == TK_BREAK as libc::c_int {
        /* will jump to label if condition is true */
        luaK_goiffalse((*ls).fs, &mut v);
        /* must enter block before 'goto' */
        enterblock(fs, &mut bl, 0i32 as lu_byte);
        /* handle goto/break */
        gotostat(ls, v.t);
        /* skip other no-op statements */
        skipnoopstat(ls);
        if 0 != block_follow(ls, 0i32) {
            /* 'goto' is the entire block? */
            leaveblock(fs);
            /* and that is it */
            return;
        } else {
            jf = luaK_jump(fs)
        }
    } else {
        /* regular case (not goto/break) */
        /* skip over block if condition is false */
        luaK_goiftrue((*ls).fs, &mut v);
        enterblock(fs, &mut bl, 0i32 as lu_byte);
        jf = v.f
    }
    /* 'then' part */
    statlist(ls);
    leaveblock(fs);
    if (*ls).t.token == TK_ELSE as libc::c_int || (*ls).t.token == TK_ELSEIF as libc::c_int {
        /* followed by 'else'/'elseif'? */
        /* must jump over it */
        luaK_concat(fs, escapelist, luaK_jump(fs));
    }
    luaK_patchtohere(fs, jf);
}
