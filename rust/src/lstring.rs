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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaM_toobig(L: *mut lua_State_0) -> !;
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State_0,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaC_fix(L: *mut lua_State_0, o: *mut GCObject) -> ();
    #[no_mangle]
    fn luaC_newobj(L: *mut lua_State_0, tt: libc::c_int, sz: size_t) -> *mut GCObject;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    c: CClosure_0,
    l: LClosure_0,
}
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
** Description of a local variable for function prototypes
** (used for debug information)
*/
pub type LocVar = LocVar_0;
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
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
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
pub type Udata = Udata_0;
/*
** Union of all collectable objects (only for conversions)
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata_0,
    cl: Closure,
    h: Table,
    p: Proto,
    th: lua_State,
}
/*
** Closures
*/
pub type CClosure_0 = CClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar_0 {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
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
pub type LClosure_0 = LClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table,
    pub len: size_t,
    pub user_: Value_0,
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
pub union UUdata {
    dummy: L_Umaxalign,
    uv: Udata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString_0 {
    dummy: L_Umaxalign,
    tsv: TString,
}
#[no_mangle]
pub unsafe extern "C" fn luaS_hash(
    mut str: *const libc::c_char,
    mut l: size_t,
    mut seed: libc::c_uint,
) -> libc::c_uint {
    let mut h: libc::c_uint = seed ^ l as libc::c_uint;
    let mut step: size_t = (l >> 5i32).wrapping_add(1i32 as libc::c_ulong);
    while l >= step {
        h ^= (h << 5i32).wrapping_add(h >> 2i32).wrapping_add(
            *str.offset(l.wrapping_sub(1i32 as libc::c_ulong) as isize) as lu_byte as libc::c_uint,
        );
        l = (l as libc::c_ulong).wrapping_sub(step) as size_t as size_t
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_hashlongstr(mut ts: *mut TString) -> libc::c_uint {
    if (*ts).tt as libc::c_int == 4i32 | 1i32 << 4i32 {
    } else {
        __assert_fail(
            b"ts->tt == (4 | (1 << 4))\x00" as *const u8 as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            59i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"unsigned int luaS_hashlongstr(TString *)\x00",
            )).as_ptr(),
        );
    };
    if (*ts).extra as libc::c_int == 0i32 {
        /* no hash? */
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((ts)->extra)\x00" as *const u8 as *const libc::c_char,
                b"lstring.c\x00" as *const u8 as *const libc::c_char,
                61i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"unsigned int luaS_hashlongstr(TString *)\x00",
                )).as_ptr(),
            );
        };
        (*ts).hash = luaS_hash(
            (ts as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize),
            (*ts).u.lnglen,
            (*ts).hash,
        );
        /* now it has its hash */
        (*ts).extra = 1i32 as lu_byte
    }
    return (*ts).hash;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_eqlngstr(mut a: *mut TString, mut b: *mut TString) -> libc::c_int {
    let mut len: size_t = (*a).u.lnglen;
    if (*a).tt as libc::c_int == 4i32 | 1i32 << 4i32
        && (*b).tt as libc::c_int == 4i32 | 1i32 << 4i32
    {
    } else {
        __assert_fail(
            b"a->tt == (4 | (1 << 4)) && b->tt == (4 | (1 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            42i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"int luaS_eqlngstr(TString *, TString *)\x00",
            )).as_ptr(),
        );
    };
    /* same instance or... */
    return (a == b || len == (*b).u.lnglen && {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((a)->extra)\x00" as *const u8 as *const libc::c_char,
                b"lstring.c\x00" as *const u8 as *const libc::c_char,
                45i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"int luaS_eqlngstr(TString *, TString *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((b)->extra)\x00" as *const u8 as *const libc::c_char,
                b"lstring.c\x00" as *const u8 as *const libc::c_char,
                45i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                    b"int luaS_eqlngstr(TString *, TString *)\x00",
                )).as_ptr(),
            );
        };
        memcmp(
            (a as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                as *const libc::c_void,
            (b as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                as *const libc::c_void,
            len,
        ) == 0i32
    }) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_resize(mut L: *mut lua_State_0, mut newsize: libc::c_int) -> () {
    let mut i: libc::c_int = 0;
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    if newsize > (*tb).size {
        /* grow table if needed */
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            >= ::std::mem::size_of::<size_t>() as libc::c_ulong
            && (newsize as size_t).wrapping_add(1i32 as libc::c_ulong)
                > (!(0i32 as size_t))
                    .wrapping_div(::std::mem::size_of::<*mut TString>() as libc::c_ulong)
        {
            luaM_toobig(L);
        } else {
        };
        (*tb).hash = luaM_realloc_(
            L,
            (*tb).hash as *mut libc::c_void,
            ((*tb).size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TString>() as libc::c_ulong),
            (newsize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TString>() as libc::c_ulong),
        ) as *mut *mut TString;
        i = (*tb).size;
        while i < newsize {
            let ref mut fresh0 = *(*tb).hash.offset(i as isize);
            *fresh0 = 0 as *mut TString;
            i += 1
        }
    }
    i = 0i32;
    while i < (*tb).size {
        /* rehash */
        let mut p: *mut TString = *(*tb).hash.offset(i as isize);
        let ref mut fresh1 = *(*tb).hash.offset(i as isize);
        *fresh1 = 0 as *mut TString;
        while !p.is_null() {
            /* for each node in the list */
            /* save next */
            let mut hnext: *mut TString = (*p).u.hnext;
            /* new position */
            if newsize & newsize - 1i32 == 0i32 {
            } else {
                __assert_fail(
                    b"(newsize&(newsize-1))==0\x00" as *const u8 as *const libc::c_char,
                    b"lstring.c\x00" as *const u8 as *const libc::c_char,
                    84i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                        b"void luaS_resize(lua_State *, int)\x00",
                    )).as_ptr(),
                );
            };
            let mut h: libc::c_uint =
                ((*p).hash & (newsize - 1i32) as libc::c_uint) as libc::c_int as libc::c_uint;
            /* chain it */
            (*p).u.hnext = *(*tb).hash.offset(h as isize);
            let ref mut fresh2 = *(*tb).hash.offset(h as isize);
            *fresh2 = p;
            p = hnext
        }
        i += 1
    }
    if newsize < (*tb).size {
        /* shrink table if needed */
        if (*(*tb).hash.offset(newsize as isize)).is_null()
            && (*(*tb).hash.offset(((*tb).size - 1i32) as isize)).is_null()
        {
        } else {
            __assert_fail(
                b"tb->hash[newsize] == ((void*)0) && tb->hash[tb->size - 1] == ((void*)0)\x00"
                    as *const u8 as *const libc::c_char,
                b"lstring.c\x00" as *const u8 as *const libc::c_char,
                92i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"void luaS_resize(lua_State *, int)\x00",
                )).as_ptr(),
            );
        };
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            >= ::std::mem::size_of::<size_t>() as libc::c_ulong
            && (newsize as size_t).wrapping_add(1i32 as libc::c_ulong)
                > (!(0i32 as size_t))
                    .wrapping_div(::std::mem::size_of::<*mut TString>() as libc::c_ulong)
        {
            luaM_toobig(L);
        } else {
        };
        (*tb).hash = luaM_realloc_(
            L,
            (*tb).hash as *mut libc::c_void,
            ((*tb).size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TString>() as libc::c_ulong),
            (newsize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TString>() as libc::c_ulong),
        ) as *mut *mut TString
    }
    /* vanishing slice should be empty */
    (*tb).size = newsize;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_clearcache(mut g: *mut global_State) -> () {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0i32;
    while i < 23i32 {
        j = 0i32;
        while j < 5i32 {
            /* will entry be collected? */
            if 0 != (*(*g).strcache[i as usize][j as usize]).marked as libc::c_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                /* replace it with something fixed */
                (*g).strcache[i as usize][j as usize] = (*g).memerrmsg
            }
            j += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaS_init(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* initial size of string table */
    luaS_resize(L, 2i32);
    /* pre-create memory-error message */
    (*g).memerrmsg = luaS_newlstr(
        L,
        b"not enough memory\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    /* it should never be collected */
    if (*(*g).memerrmsg).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((g->memerrmsg)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            122i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"void luaS_init(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    luaC_fix(L, &mut (*((*g).memerrmsg as *mut GCUnion)).gc);
    /* fill cache with valid strings */
    i = 0i32;
    while i < 23i32 {
        j = 0i32;
        while j < 5i32 {
            (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            j += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newlstr(
    mut L: *mut lua_State_0,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> *mut TString {
    /* short string? */
    if l <= 40i32 as libc::c_ulong {
        return internshrstr(L, str, l);
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        if l >= if (::std::mem::size_of::<size_t>() as libc::c_ulong)
            < ::std::mem::size_of::<lua_Integer>() as libc::c_ulong
        {
            !(0i32 as size_t)
        } else {
            9223372036854775807i64 as size_t
        }.wrapping_sub(::std::mem::size_of::<TString>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        {
            luaM_toobig(L);
        } else {
            ts = luaS_createlngstrobj(L, l);
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(
                    b"sizeof((ts)->extra)\x00" as *const u8 as *const libc::c_char,
                    b"lstring.c\x00" as *const u8 as *const libc::c_char,
                    207i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                        b"TString *luaS_newlstr(lua_State *, const char *, size_t)\x00",
                    )).as_ptr(),
                );
            };
            memcpy(
                (ts as *mut libc::c_char)
                    .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                    as *mut libc::c_void,
                str as *const libc::c_void,
                l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            return ts;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_createlngstrobj(
    mut L: *mut lua_State_0,
    mut l: size_t,
) -> *mut TString {
    let mut ts: *mut TString = createstrobj(L, l, 4i32 | 1i32 << 4i32, (*(*L).l_G).seed);
    (*ts).u.lnglen = l;
    return ts;
}
/*
** creates a new string object
*/
unsafe extern "C" fn createstrobj(
    mut L: *mut lua_State_0,
    mut l: size_t,
    mut tag: libc::c_int,
    mut h: libc::c_uint,
) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    /* total size of TString object */
    let mut totalsize: size_t = 0;
    totalsize = (::std::mem::size_of::<UTString_0>() as libc::c_ulong).wrapping_add(
        l.wrapping_add(1i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    o = luaC_newobj(L, tag, totalsize);
    if (*o).tt as libc::c_int & 0xfi32 == 4i32 {
    } else {
        __assert_fail(
            b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            139i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                b"TString *createstrobj(lua_State *, size_t, int, unsigned int)\x00",
            )).as_ptr(),
        );
    };
    ts = &mut (*(o as *mut GCUnion)).ts;
    (*ts).hash = h;
    (*ts).extra = 0i32 as lu_byte;
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof((ts)->extra)\x00" as *const u8 as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            142i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                b"TString *createstrobj(lua_State *, size_t, int, unsigned int)\x00",
            )).as_ptr(),
        );
    };
    *(ts as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
        .offset(l as isize) = '\u{0}' as i32 as libc::c_char;
    /* ending 0 */
    return ts;
}
/*
** checks whether short string exists and reuses it or creates a new one
*/
unsafe extern "C" fn internshrstr(
    mut L: *mut lua_State_0,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut g: *mut global_State = (*L).l_G;
    let mut h: libc::c_uint = luaS_hash(str, l, (*g).seed);
    if (*g).strt.size & (*g).strt.size - 1i32 == 0i32 {
    } else {
        __assert_fail(
            b"(g->strt.size&(g->strt.size-1))==0\x00" as *const u8 as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            171i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"TString *internshrstr(lua_State *, const char *, size_t)\x00",
            )).as_ptr(),
        );
    };
    let mut list: *mut *mut TString = &mut *(*g)
        .strt
        .hash
        .offset((h & ((*g).strt.size - 1i32) as libc::c_uint) as libc::c_int as isize)
        as *mut *mut TString;
    if !str.is_null() {
    } else {
        __assert_fail(
            b"str != ((void*)0)\x00" as *const u8 as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            172i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"TString *internshrstr(lua_State *, const char *, size_t)\x00",
            )).as_ptr(),
        );
    };
    /* otherwise 'memcmp'/'memcpy' are undefined */
    ts = *list;
    while !ts.is_null() {
        if l == (*ts).shrlen as libc::c_ulong && {
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(
                    b"sizeof((ts)->extra)\x00" as *const u8 as *const libc::c_char,
                    b"lstring.c\x00" as *const u8 as *const libc::c_char,
                    175i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                        b"TString *internshrstr(lua_State *, const char *, size_t)\x00",
                    )).as_ptr(),
                );
            };
            memcmp(
                str as *const libc::c_void,
                (ts as *mut libc::c_char)
                    .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                    as *const libc::c_void,
                l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) == 0i32
        } {
            /* found! */
            /* dead (but not collected yet)? */
            if 0 == ((*ts).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            {
                (*ts).marked =
                    ((*ts).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
            }
            /* resurrect it */
            return ts;
        } else {
            ts = (*ts).u.hnext
        }
    }
    if (*g).strt.nuse >= (*g).strt.size && (*g).strt.size <= 2147483647i32 / 2i32 {
        luaS_resize(L, (*g).strt.size * 2i32);
        /* recompute with new size */
        if (*g).strt.size & (*g).strt.size - 1i32 == 0i32 {
        } else {
            __assert_fail(
                b"(g->strt.size&(g->strt.size-1))==0\x00" as *const u8 as *const libc::c_char,
                b"lstring.c\x00" as *const u8 as *const libc::c_char,
                184i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"TString *internshrstr(lua_State *, const char *, size_t)\x00",
                )).as_ptr(),
            );
        };
        list = &mut *(*g)
            .strt
            .hash
            .offset((h & ((*g).strt.size - 1i32) as libc::c_uint) as libc::c_int as isize)
            as *mut *mut TString
    }
    ts = createstrobj(L, l, 4i32 | 0i32 << 4i32, h);
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof((ts)->extra)\x00" as *const u8 as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            187i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"TString *internshrstr(lua_State *, const char *, size_t)\x00",
            )).as_ptr(),
        );
    };
    memcpy(
        (ts as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        str as *const libc::c_void,
        l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*ts).shrlen = l as lu_byte;
    (*ts).u.hnext = *list;
    *list = ts;
    (*g).strt.nuse += 1;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_remove(mut L: *mut lua_State_0, mut ts: *mut TString) -> () {
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    if (*tb).size & (*tb).size - 1i32 == 0i32 {
    } else {
        __assert_fail(
            b"(tb->size&(tb->size-1))==0\x00" as *const u8 as *const libc::c_char,
            b"lstring.c\x00" as *const u8 as *const libc::c_char,
            156i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"void luaS_remove(lua_State *, TString *)\x00",
            )).as_ptr(),
        );
    };
    let mut p: *mut *mut TString = &mut *(*tb)
        .hash
        .offset(((*ts).hash & ((*tb).size - 1i32) as libc::c_uint) as libc::c_int as isize)
        as *mut *mut TString;
    /* find previous element */
    while *p != ts {
        p = &mut (**p).u.hnext
    }
    /* remove element from its list */
    *p = (**p).u.hnext;
    (*tb).nuse -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newudata(mut L: *mut lua_State_0, mut s: size_t) -> *mut Udata {
    let mut u: *mut Udata = 0 as *mut Udata;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    if s > if (::std::mem::size_of::<size_t>() as libc::c_ulong)
        < ::std::mem::size_of::<lua_Integer>() as libc::c_ulong
    {
        !(0i32 as size_t)
    } else {
        9223372036854775807i64 as size_t
    }.wrapping_sub(::std::mem::size_of::<Udata>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
        o = luaC_newobj(
            L,
            7i32,
            (::std::mem::size_of::<UUdata>() as libc::c_ulong).wrapping_add(s),
        );
        if (*o).tt as libc::c_int == 7i32 {
        } else {
            __assert_fail(
                b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                b"lstring.c\x00" as *const u8 as *const libc::c_char,
                242i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"Udata *luaS_newudata(lua_State *, size_t)\x00",
                )).as_ptr(),
            );
        };
        u = &mut (*(o as *mut GCUnion)).u;
        (*u).len = s;
        (*u).metatable = 0 as *mut Table;
        let mut io: *const TValue = &luaO_nilobject_;
        let mut iu: *mut Udata = u;
        (*iu).user_ = (*io).value_;
        (*iu).ttuv_ = (*io).tt_ as lu_byte;
        if 0 == (*io).tt_ & 1i32 << 6i32 || {
            if 0 != (*io).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lstring.c\x00" as *const u8 as *const libc::c_char,
                    245i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"Udata *luaS_newudata(lua_State *, size_t)\x00",
                    )).as_ptr(),
                );
            };
            (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int && (L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lstring.c\x00" as *const u8 as *const libc::c_char,
                        245i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                            b"Udata *luaS_newudata(lua_State *, size_t)\x00",
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
                    b"lstring.c\x00" as *const u8 as *const libc::c_char,
                    245i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"Udata *luaS_newudata(lua_State *, size_t)\x00",
                    )).as_ptr(),
                );
            };
        };
        return u;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_new(
    mut L: *mut lua_State_0,
    mut str: *const libc::c_char,
) -> *mut TString {
    /* hash */
    let mut i: libc::c_uint = ((str as size_t
        & (2147483647i32 as libc::c_uint)
            .wrapping_mul(2u32)
            .wrapping_add(1u32) as libc::c_ulong) as libc::c_uint)
        .wrapping_rem(23i32 as libc::c_uint);
    let mut j: libc::c_int = 0;
    let mut p: *mut *mut TString = (*(*L).l_G).strcache[i as usize].as_mut_ptr();
    j = 0i32;
    while j < 5i32 {
        /* hit? */
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((p[j])->extra)\x00" as *const u8 as *const libc::c_char,
                b"lstring.c\x00" as *const u8 as *const libc::c_char,
                224i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"TString *luaS_new(lua_State *, const char *)\x00",
                )).as_ptr(),
            );
        };
        if strcmp(
            str,
            (*p.offset(j as isize) as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize),
        ) == 0i32
        {
            /* that is it */
            return *p.offset(j as isize);
        } else {
            j += 1
        }
    }
    /* normal route */
    j = 5i32 - 1i32;
    while j > 0i32 {
        /* move out last element */
        let ref mut fresh3 = *p.offset(j as isize);
        *fresh3 = *p.offset((j - 1i32) as isize);
        j -= 1
    }
    /* new element is first in the list */
    let ref mut fresh4 = *p.offset(0isize);
    *fresh4 = luaS_newlstr(L, str, strlen(str));
    return *p.offset(0isize);
}
