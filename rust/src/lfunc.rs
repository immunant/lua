use lobject::{
    unnamed, CClosure, Closure, GCObject, LClosure, LocVar, Proto, TString, TValue, Upvaldesc,
    Value,
};
use lstate::{global_State, lua_State, stringtable, CallInfo, GCUnion};

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
    fn luaC_newobj(L: *mut lua_State_0, tt: libc::c_int, sz: size_t) -> *mut GCObject;
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State_0,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaC_upvalbarrier_(L: *mut lua_State_0, uv: *mut UpVal) -> ();
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
    pub open: unnamed_3,
    pub value: TValue,
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
/*
** Header for string value; string bytes follow the end of this structure
** (aligned according to 'UTString'; see next).
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_4 {
    lnglen: size_t,
    hnext: *mut TString,
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
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void, _: size_t, _: size_t)
        -> *mut libc::c_void,
>;
/*
** Function Prototypes
*/
pub type Proto_0 = Proto;
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
pub type LocVar_0 = LocVar;
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
pub type CClosure_0 = CClosure;
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
    pub user_: Value,
}
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
pub type Upvaldesc_0 = Upvaldesc;
#[no_mangle]
pub unsafe extern "C" fn luaF_newproto(mut L: *mut lua_State_0) -> *mut Proto_0 {
    let mut o: *mut GCObject =
        luaC_newobj(L, 9i32, ::std::mem::size_of::<Proto_0>() as libc::c_ulong);
    if (*o).tt as libc::c_int == 9i32 {
    } else {
        __assert_fail(
            b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
            b"lfunc.c\x00" as *const u8 as *const libc::c_char,
            101i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"Proto *luaF_newproto(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    let mut f: *mut Proto_0 = &mut (*(o as *mut GCUnion)).p;
    (*f).k = 0 as *mut TValue;
    (*f).sizek = 0i32;
    (*f).p = 0 as *mut *mut Proto;
    (*f).sizep = 0i32;
    (*f).code = 0 as *mut Instruction;
    (*f).cache = 0 as *mut LClosure_0;
    (*f).sizecode = 0i32;
    (*f).lineinfo = 0 as *mut libc::c_int;
    (*f).sizelineinfo = 0i32;
    (*f).upvalues = 0 as *mut Upvaldesc_0;
    (*f).sizeupvalues = 0i32;
    (*f).numparams = 0i32 as lu_byte;
    (*f).is_vararg = 0i32 as lu_byte;
    (*f).maxstacksize = 0i32 as lu_byte;
    (*f).locvars = 0 as *mut LocVar;
    (*f).sizelocvars = 0i32;
    (*f).linedefined = 0i32;
    (*f).lastlinedefined = 0i32;
    (*f).source = 0 as *mut TString;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newCclosure(
    mut L: *mut lua_State_0,
    mut n: libc::c_int,
) -> *mut CClosure_0 {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        6i32 | 2i32 << 4i32,
        (::std::mem::size_of::<CClosure_0>() as libc::c_ulong as libc::c_int
            + (::std::mem::size_of::<TValue>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong)
                as libc::c_int) as size_t,
    );
    if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
    } else {
        __assert_fail(
            b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
            b"lfunc.c\x00" as *const u8 as *const libc::c_char,
            27i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"CClosure *luaF_newCclosure(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let mut c: *mut CClosure_0 = &mut (*(o as *mut GCUnion)).cl.c;
    (*c).nupvalues = n as lu_byte;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newLclosure(
    mut L: *mut lua_State_0,
    mut n: libc::c_int,
) -> *mut LClosure {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        6i32 | 0i32 << 4i32,
        (::std::mem::size_of::<LClosure>() as libc::c_ulong as libc::c_int
            + (::std::mem::size_of::<*mut TValue>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong) as libc::c_int) as size_t,
    );
    if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
            b"lfunc.c\x00" as *const u8 as *const libc::c_char,
            35i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"LClosure *luaF_newLclosure(lua_State *, int)\x00",
            )).as_ptr(),
        );
    };
    let mut c: *mut LClosure = &mut (*(o as *mut GCUnion)).cl.l;
    (*c).p = 0 as *mut Proto;
    (*c).nupvalues = n as lu_byte;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(0 != fresh0) {
            break;
        }
        let ref mut fresh1 = *(*c).upvals.as_mut_ptr().offset(n as isize);
        *fresh1 = 0 as *mut UpVal
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_initupvals(mut L: *mut lua_State_0, mut cl: *mut LClosure) -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv: *mut UpVal = luaM_realloc_(
            L,
            0 as *mut libc::c_void,
            0i32 as size_t,
            ::std::mem::size_of::<UpVal>() as libc::c_ulong,
        ) as *mut UpVal;
        (*uv).refcount = 1i32 as lu_mem;
        /* make it closed */
        (*uv).v = &mut (*uv).u.value;
        (*(*uv).v).tt_ = 0i32;
        let ref mut fresh2 = *(*cl).upvals.as_mut_ptr().offset(i as isize);
        *fresh2 = uv;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaF_findupval(mut L: *mut lua_State_0, mut level: StkId) -> *mut UpVal {
    let mut pp: *mut *mut UpVal = &mut (*L).openupval;
    let mut p: *mut UpVal = 0 as *mut UpVal;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    if (*L).twups != L || (*L).openupval.is_null() {
    } else {
        __assert_fail(
            b"(L->twups != L) || L->openupval == ((void*)0)\x00" as *const u8
                as *const libc::c_char,
            b"lfunc.c\x00" as *const u8 as *const libc::c_char,
            61i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"UpVal *luaF_findupval(lua_State *, StkId)\x00",
            )).as_ptr(),
        );
    };
    while !(*pp).is_null() && {
        p = *pp;
        (*p).v >= level
    } {
        if (*p).v != &mut (*p).u.value as *mut TValue {
        } else {
            __assert_fail(
                b"((p)->v != &(p)->u.value)\x00" as *const u8 as *const libc::c_char,
                b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                63i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"UpVal *luaF_findupval(lua_State *, StkId)\x00",
                )).as_ptr(),
            );
        };
        /* found a corresponding upvalue? */
        if (*p).v == level {
            /* return it */
            return p;
        } else {
            pp = &mut (*p).u.open.next
        }
    }
    /* not found: create a new upvalue */
    uv = luaM_realloc_(
        L,
        0 as *mut libc::c_void,
        0i32 as size_t,
        ::std::mem::size_of::<UpVal>() as libc::c_ulong,
    ) as *mut UpVal;
    (*uv).refcount = 0i32 as lu_mem;
    /* link it to list of open upvalues */
    (*uv).u.open.next = *pp;
    (*uv).u.open.touched = 1i32;
    *pp = uv;
    /* current value lives in the stack */
    (*uv).v = level;
    if !((*L).twups != L) {
        /* thread not in list of threads with upvalues? */
        /* link it to the list */
        (*L).twups = (*(*L).l_G).twups;
        (*(*L).l_G).twups = L
    }
    return uv;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_close(mut L: *mut lua_State_0, mut level: StkId) -> () {
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    while !(*L).openupval.is_null() && {
        uv = (*L).openupval;
        (*uv).v >= level
    } {
        if (*uv).v != &mut (*uv).u.value as *mut TValue {
        } else {
            __assert_fail(
                b"((uv)->v != &(uv)->u.value)\x00" as *const u8 as *const libc::c_char,
                b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                86i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                    b"void luaF_close(lua_State *, StkId)\x00",
                )).as_ptr(),
            );
        };
        /* remove from 'open' list */
        (*L).openupval = (*uv).u.open.next;
        /* no references? */
        if (*uv).refcount == 0i32 as libc::c_ulong {
            luaM_realloc_(
                L,
                uv as *mut libc::c_void,
                ::std::mem::size_of::<UpVal>() as libc::c_ulong,
                0i32 as size_t,
            );
        } else {
            /* free upvalue */
            let mut io1: *mut TValue = &mut (*uv).u.value;
            *io1 = *(*uv).v;
            if 0 == (*io1).tt_ & 1i32 << 6i32 || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                        91i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void luaF_close(lua_State *, StkId)\x00",
                        )).as_ptr(),
                    );
                };
                (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int
                    && (L.is_null() || {
                        if 0 != (*io1).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                    as *const libc::c_char,
                                b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                                91i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                                    b"void luaF_close(lua_State *, StkId)\x00",
                                )).as_ptr(),
                            );
                        };
                        0 != ((*(*io1).value_.gc).marked as libc::c_int
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
                        b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                        91i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void luaF_close(lua_State *, StkId)\x00",
                        )).as_ptr(),
                    );
                };
            };
            /* move value to upvalue slot */
            /* now current value lives here */
            (*uv).v = &mut (*uv).u.value;
            if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 && !((*uv).v != &mut (*uv).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, uv);
            } else {
            };
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaF_freeproto(mut L: *mut lua_State_0, mut f: *mut Proto_0) -> () {
    luaM_realloc_(
        L,
        (*f).code as *mut libc::c_void,
        ((*f).sizecode as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Instruction>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).p as *mut libc::c_void,
        ((*f).sizep as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut Proto>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).k as *mut libc::c_void,
        ((*f).sizek as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).lineinfo as *mut libc::c_void,
        ((*f).sizelineinfo as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).locvars as *mut libc::c_void,
        ((*f).sizelocvars as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<LocVar>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).upvalues as *mut libc::c_void,
        ((*f).sizeupvalues as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Upvaldesc_0>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        f as *mut libc::c_void,
        ::std::mem::size_of::<Proto_0>() as libc::c_ulong,
        0i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaF_getlocalname(
    mut f: *const Proto_0,
    mut local_number: libc::c_int,
    mut pc: libc::c_int,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*f).sizelocvars && (*(*f).locvars.offset(i as isize)).startpc <= pc {
        if pc < (*(*f).locvars.offset(i as isize)).endpc {
            /* is variable active? */
            local_number -= 1;
            if local_number == 0i32 {
                if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
                } else {
                    __assert_fail(
                        b"sizeof((f->locvars[i].varname)->extra)\x00" as *const u8
                            as *const libc::c_char,
                        b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                        146i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                            b"const char *luaF_getlocalname(const Proto *, int, int)\x00",
                        )).as_ptr(),
                    );
                };
                return ((*(*f).locvars.offset(i as isize)).varname as *mut libc::c_char)
                    .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
            }
        }
        i += 1
    }
    /* not found */
    return 0 as *const libc::c_char;
}
