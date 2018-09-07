use lobject::{lua_TValue, GCObject, TValue, Value};
use lstate::{global_State, lua_State, CallInfo};

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
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaG_runerror(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State_0, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaC_fullgc(L: *mut lua_State_0, isemergency: libc::c_int) -> ();
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
#[no_mangle]
pub unsafe extern "C" fn luaM_toobig(mut L: *mut lua_State_0) -> ! {
    luaG_runerror(
        L,
        b"memory allocation error: block too big\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaM_realloc_(
    mut L: *mut lua_State_0,
    mut block: *mut libc::c_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut g: *mut global_State = (*L).l_G;
    let mut realosize: size_t = if !block.is_null() {
        osize
    } else {
        0i32 as libc::c_ulong
    };
    if (realosize == 0i32 as libc::c_ulong) as libc::c_int
        == (block == 0 as *mut libc::c_void) as libc::c_int
    {
    } else {
        __assert_fail(
            b"(realosize == 0) == (block == ((void*)0))\x00" as *const u8 as *const libc::c_char,
            b"lmem.c\x00" as *const u8 as *const libc::c_char,
            82i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"void *luaM_realloc_(lua_State *, void *, size_t, size_t)\x00",
            )).as_ptr(),
        );
    };
    newblock = (*g).frealloc.expect("non-null function pointer")((*g).ud, block, osize, nsize);
    if newblock.is_null() && nsize > 0i32 as libc::c_ulong {
        if nsize > realosize {
        } else {
            __assert_fail(
                b"nsize > realosize\x00" as *const u8 as *const libc::c_char,
                b"lmem.c\x00" as *const u8 as *const libc::c_char,
                89i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"void *luaM_realloc_(lua_State *, void *, size_t, size_t)\x00",
                )).as_ptr(),
            );
        };
        /* cannot fail when shrinking a block */
        if !(*g).version.is_null() {
            /* is state fully built? */
            /* try to free some memory... */
            luaC_fullgc(L, 1i32);
            /* try again */
            newblock =
                (*g).frealloc.expect("non-null function pointer")((*g).ud, block, osize, nsize)
        }
        if newblock.is_null() {
            luaD_throw(L, 4i32);
        }
    }
    if (nsize == 0i32 as libc::c_ulong) as libc::c_int
        == (newblock == 0 as *mut libc::c_void) as libc::c_int
    {
    } else {
        __assert_fail(
            b"(nsize == 0) == (newblock == ((void*)0))\x00" as *const u8 as *const libc::c_char,
            b"lmem.c\x00" as *const u8 as *const libc::c_char,
            97i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"void *luaM_realloc_(lua_State *, void *, size_t, size_t)\x00",
            )).as_ptr(),
        );
    };
    (*g).GCdebt = ((*g).GCdebt as libc::c_ulong)
        .wrapping_add(nsize)
        .wrapping_sub(realosize) as l_mem;
    return newblock;
}
#[no_mangle]
pub unsafe extern "C" fn luaM_growaux_(
    mut L: *mut lua_State_0,
    mut block: *mut libc::c_void,
    mut size: *mut libc::c_int,
    mut size_elems: size_t,
    mut limit: libc::c_int,
    mut what: *const libc::c_char,
) -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut newsize: libc::c_int = 0;
    if *size >= limit / 2i32 {
        /* cannot double it? */
        /* cannot grow even a little? */
        if *size >= limit {
            luaG_runerror(
                L,
                b"too many %s (limit is %d)\x00" as *const u8 as *const libc::c_char,
                what,
                limit,
            );
        } else {
            newsize = limit
        }
    } else {
        newsize = *size * 2i32;
        if newsize < 4i32 {
            /* minimum size */
            newsize = 4i32
        }
    }
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (newsize as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(size_elems)
    {
        luaM_toobig(L);
    } else {
    };
    newblock = luaM_realloc_(
        L,
        block,
        (*size as libc::c_ulong).wrapping_mul(size_elems),
        (newsize as libc::c_ulong).wrapping_mul(size_elems),
    );
    /* update only when everything else is OK */
    *size = newsize;
    return newblock;
}
