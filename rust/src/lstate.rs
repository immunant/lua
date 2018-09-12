use ldebug::lua_Debug;
use ldo::lua_longjmp;
use lfunc::UpVal;
use lobject::{
    lua_TValue, CClosure, Closure, GCObject, LClosure, Proto, StkId, TString, TValue, Table, Udata,
    Value, Node
};

use std::convert::From;
use std::cell::{Ref, RefMut, RefCell};
use std::rc::Rc;

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
    fn luaM_realloc_(
        L: *mut lua_State_0,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaC_freeallobjects(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaF_close(L: *mut lua_State_0, level: StkId) -> ();
    #[no_mangle]
    fn lua_version(L: *mut lua_State_0) -> *const lua_Number;
    #[no_mangle]
    fn luaX_init(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaT_init(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaS_init(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaH_new(L: *mut lua_State_0) -> *mut Table_0;
    #[no_mangle]
    fn luaH_setint(
        L: *mut lua_State_0,
        t: *mut Table_0,
        key: lua_Integer,
        value: *mut TValue,
    ) -> ();
    #[no_mangle]
    fn luaH_resize(
        L: *mut lua_State_0,
        t: *mut Table_0,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    ) -> ();
    #[no_mangle]
    fn luaM_toobig(L: *mut lua_State_0) -> !;
    #[no_mangle]
    fn luaD_rawrunprotected(L: *mut lua_State_0, f: Pfunc, ud: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn luaS_hash(str: *const libc::c_char, l: size_t, seed: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaC_step(L: *mut lua_State_0) -> ();
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
pub type __time_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type intptr_t = libc::c_long;
pub type Value_0 = Value;

#[derive(Clone)]
#[repr(C)]
pub struct lua_State {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nci: libc::c_ushort,
    pub status: lu_byte,
    pub top: StkId,
    pub l_G: *mut global_State,
    pub ci: Option<Rc<RefCell<CallInfo>>>,
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

pub type lua_Debug_0 = lua_Debug;
/* active function */

#[derive(Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: StkId,
    pub top: StkId,
    pub previous: Option<Rc<RefCell<CallInfo>>>,
    pub next: Option<Rc<RefCell<CallInfo>>>,
    pub u: unnamed,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}

impl CallInfo {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CallInfo {
            func: 0 as StkId,
            top: 0 as StkId,
            previous: None,
            next: None,
            u: unnamed::from(0),
            extra: 0 as ptrdiff_t,
            nresults: 0 as libc::c_short,
            callstatus: 0 as libc::c_ushort,
        }))
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed {
    pub l: unnamed_1,
    pub c: unnamed_0,
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

/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State_0) -> libc::c_int>;

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
pub type UpVal_0 = UpVal;
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
pub type TString_0 = TString;
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_4 {
    pub lnglen: size_t,
    pub hnext: *mut TString_0,
}
/* copy a value into a key without messing up field 'next' */
pub type Node_0 = Node;
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
pub type time_t = __time_t;
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
** Main thread combines a thread state and the global state
*/
pub type LG = LG_0;
#[derive(Clone)]
#[repr(C)]
pub struct LG_0 {
    pub l: LX,
    pub g: global_State,
}
/*
** $Id: lstate.c,v 2.132 2015/11/02 16:01:41 roberto Exp roberto $
** Global State
** See Copyright Notice in lua.h
*/
/* 200% */
/* GC runs 'twice the speed' of memory allocation */
/*
** a macro to help the creation of a unique random seed when a state is
** created; the seed is used to randomize hashes.
*/
/*
** thread state + extra space
*/
pub type LX = LX_0;
#[derive(Clone)]
#[repr(C)]
pub struct LX_0 {
    pub extra_: [lu_byte; 16],
    pub l: lua_State_0,
}
pub type Table_0 = Table;
/*
** Union of all collectable objects (only for conversions)
*/
#[repr(C)]
pub union GCUnion {
    pub gc: GCObject,
    pub ts: TString_0,
    pub u: Udata,
    pub cl: Closure,
    pub h: Table,
    pub p: Proto,
    pub th: lua_State,
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
pub type LClosure_0 = LClosure;
/*
** Closures
*/
pub type CClosure_0 = CClosure;
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
#[no_mangle]
pub unsafe extern "C" fn lua_newstate(
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) -> *mut lua_State_0 {
    let mut i: libc::c_int = 0;
    let mut L: *mut lua_State_0 = 0 as *mut lua_State_0;
    let mut g: *mut global_State = 0 as *mut global_State;
    let mut l: *mut LG = f.expect("non-null function pointer")(
        ud,
        0 as *mut libc::c_void,
        8i32 as size_t,
        ::std::mem::size_of::<LG>() as libc::c_ulong,
    ) as *mut LG;
    if l.is_null() {
        return 0 as *mut lua_State_0;
    } else {
        L = &mut (*l).l.l;
        g = &mut (*l).g;
        (*L).next = 0 as *mut GCObject;
        (*L).tt = 8i32 as lu_byte;
        (*g).currentwhite = (1i32 << 0i32) as lu_byte;
        (*L).marked = ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte;
        preinit_thread(L, g);
        (*g).frealloc = f;
        (*g).ud = ud;
        (*g).mainthread = L;
        (*g).seed = makeseed(L);
        /* no GC while building state */
        (*g).gcrunning = 0i32 as lu_byte;
        (*g).GCestimate = 0i32 as lu_mem;
        (*g).strt.nuse = 0i32;
        (*g).strt.size = (*g).strt.nuse;
        (*g).strt.hash = 0 as *mut *mut TString;
        (*g).l_registry.tt_ = 0i32;
        (*g).panic = None;
        (*g).version = 0 as *const lua_Number;
        (*g).gcstate = 7i32 as lu_byte;
        (*g).gckind = 0i32 as lu_byte;
        (*g).fixedgc = 0 as *mut GCObject;
        (*g).tobefnz = (*g).fixedgc;
        (*g).finobj = (*g).tobefnz;
        (*g).allgc = (*g).finobj;
        (*g).sweepgc = 0 as *mut *mut GCObject;
        (*g).grayagain = 0 as *mut GCObject;
        (*g).gray = (*g).grayagain;
        (*g).allweak = 0 as *mut GCObject;
        (*g).ephemeron = (*g).allweak;
        (*g).weak = (*g).ephemeron;
        (*g).twups = 0 as *mut lua_State;
        (*g).totalbytes = ::std::mem::size_of::<LG>() as libc::c_ulong as l_mem;
        (*g).GCdebt = 0i32 as l_mem;
        (*g).gcfinnum = 0i32 as libc::c_uint;
        (*g).gcpause = 200i32;
        (*g).gcstepmul = 200i32;
        i = 0i32;
        while i < 9i32 {
            (*g).mt[i as usize] = 0 as *mut Table;
            i += 1
        }
        if luaD_rawrunprotected(L, Some(f_luaopen), 0 as *mut libc::c_void) != 0i32 {
            /* memory allocation error: free partial state */
            close_state(L);
            L = 0 as *mut lua_State_0
        }
        return L;
    };
}
unsafe extern "C" fn close_state(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* close all upvalues for this thread */
    luaF_close(L, (*L).stack);
    /* collect all objects */
    luaC_freeallobjects(L);
    /* closing a fully built state? */
    if !(*g).version.is_null() {
        if (*((L as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void as *mut L_EXTRA))
            .lock
            == 1i32
            && (*((L as *mut libc::c_char)
                .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA))
                .plock
                == &mut (*((L as *mut libc::c_char)
                    .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
                    as *mut libc::c_void as *mut L_EXTRA))
                    .lock as *mut libc::c_int
        {
        } else {
            __assert_fail(b"((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->lock == 1 && ((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock == &(((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->lock)\x00"
                              as *const u8 as *const libc::c_char,
                          b"lstate.c\x00" as *const u8 as *const libc::c_char,
                          247i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 30],
                                                    &[libc::c_char; 30]>(b"void close_state(lua_State *)\x00")).as_ptr());
        };
    }
    luaM_realloc_(
        L,
        (*(*L).l_G).strt.hash as *mut libc::c_void,
        ((*(*L).l_G).strt.size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut TString>() as libc::c_ulong),
        0i32 as size_t,
    );
    freestack(L);
    if ((*g).totalbytes + (*g).GCdebt) as lu_mem == ::std::mem::size_of::<LG>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"((lu_mem)((g)->totalbytes + (g)->GCdebt)) == sizeof(LG)\x00" as *const u8
                as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            250i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void close_state(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    /* free main block */
    (*g).frealloc.expect("non-null function pointer")(
        (*g).ud,
        (L as *mut lu_byte).offset(-16isize) as *mut LX as *mut libc::c_void,
        ::std::mem::size_of::<LG>() as libc::c_ulong,
        0i32 as size_t,
    );
}
unsafe extern "C" fn freestack(mut L: *mut lua_State) -> () {
    if (*L).stack.is_null() {
        /* stack not completely built yet */
        return;
    } else {
        /* free the entire 'ci' list */
        *((*L).ci.as_ref().unwrap()).borrow_mut() = (*L).base_ci.clone();
        luaE_freeCI(L);
        if (*L).nci as libc::c_int == 0i32 {
        } else {
            __assert_fail(
                b"L->nci == 0\x00" as *const u8 as *const libc::c_char,
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                176i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"void freestack(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        luaM_realloc_(
            L,
            (*L).stack as *mut libc::c_void,
            ((*L).stacksize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
            0i32 as size_t,
        );
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaE_freeCI(mut L: *mut lua_State_0) -> () {
    let mut ci: Option<Rc<RefCell<CallInfo>>> = (*L).ci.as_ref().cloned();
    let mut next: Option<Rc<RefCell<CallInfo>>> = ci.as_ref().unwrap().borrow_mut().next.as_ref().cloned();
    ci.as_ref().unwrap().borrow_mut().next = None;
    loop {
        ci = next;
        if ci.is_none() {
            break;
        }
        next = ci.as_ref().unwrap().borrow_mut().next.take();
        (*L).nci = (*L).nci.wrapping_sub(1)
    }
}
/*
** open parts of the state that may cause memory-allocation errors.
** ('g->version' != NULL flags that the state was completely build)
*/
unsafe extern "C" fn f_luaopen(mut L: *mut lua_State_0, mut ud: *mut libc::c_void) -> () {
    let mut g: *mut global_State = (*L).l_G;
    ud = 0 as *mut libc::c_void;
    /* init stack */
    stack_init(L, L);
    init_registry(L, g);
    luaS_init(L);
    luaT_init(L);
    luaX_init(L);
    /* allow gc */
    (*g).gcrunning = 1i32 as lu_byte;
    (*g).version = lua_version(0 as *mut lua_State_0);
    (*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .lock = 0i32;
    let ref mut fresh0 = (*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh0 = &mut (*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .lock;
}
/* free stack array */
/*
** Create registry table and its predefined values
*/
unsafe extern "C" fn init_registry(mut L: *mut lua_State_0, mut g: *mut global_State) -> () {
    let mut temp: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    /* create registry */
    let mut registry: *mut Table_0 = luaH_new(L);
    let mut io: *mut TValue = &mut (*g).l_registry;
    let mut x_: *mut Table_0 = registry;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            188i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void init_registry(lua_State *, global_State *)\x00",
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
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                188i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void init_registry(lua_State *, global_State *)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
            && (L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lstate.c\x00" as *const u8 as *const libc::c_char,
                        188i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                            b"void init_registry(lua_State *, global_State *)\x00",
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
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                188i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void init_registry(lua_State *, global_State *)\x00",
                )).as_ptr(),
            );
        };
    };
    luaH_resize(L, registry, 2i32 as libc::c_uint, 0i32 as libc::c_uint);
    let mut io_0: *mut TValue = &mut temp;
    let mut x__0: *mut lua_State_0 = L;
    if (*x__0).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            191i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void init_registry(lua_State *, global_State *)\x00",
            )).as_ptr(),
        );
    };
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
    (*io_0).tt_ = 8i32 | 1i32 << 6i32;
    if 0 == (*io_0).tt_ & 1i32 << 6i32 || {
        if 0 != (*io_0).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                191i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void init_registry(lua_State *, global_State *)\x00",
                )).as_ptr(),
            );
        };
        (*io_0).tt_ & 0x3fi32 == (*(*io_0).value_.gc).tt as libc::c_int
            && (L.is_null() || {
                if 0 != (*io_0).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lstate.c\x00" as *const u8 as *const libc::c_char,
                        191i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                            b"void init_registry(lua_State *, global_State *)\x00",
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
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                191i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void init_registry(lua_State *, global_State *)\x00",
                )).as_ptr(),
            );
        };
    };
    /* registry[LUA_RIDX_MAINTHREAD] = L */
    /* temp = L */
    luaH_setint(L, registry, 1i32 as lua_Integer, &mut temp);
    let mut io_1: *mut TValue = &mut temp;
    let mut x__1: *mut Table_0 = luaH_new(L);
    if (*x__1).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            194i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"void init_registry(lua_State *, global_State *)\x00",
            )).as_ptr(),
        );
    };
    (*io_1).value_.gc = &mut (*(x__1 as *mut GCUnion)).gc;
    (*io_1).tt_ = 5i32 | 1i32 << 6i32;
    if 0 == (*io_1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io_1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                194i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void init_registry(lua_State *, global_State *)\x00",
                )).as_ptr(),
            );
        };
        (*io_1).tt_ & 0x3fi32 == (*(*io_1).value_.gc).tt as libc::c_int
            && (L.is_null() || {
                if 0 != (*io_1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lstate.c\x00" as *const u8 as *const libc::c_char,
                        194i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                            b"void init_registry(lua_State *, global_State *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != ((*(*io_1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                194i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"void init_registry(lua_State *, global_State *)\x00",
                )).as_ptr(),
            );
        };
    };
    /* registry[LUA_RIDX_GLOBALS] = table of globals */
    /* temp = new table (global table) */
    luaH_setint(L, registry, 2i32 as lua_Integer, &mut temp);
}
unsafe extern "C" fn stack_init(mut L1: *mut lua_State_0, mut L: *mut lua_State_0) -> () {
    let mut i: libc::c_int = 0;
    let mut ci: Option<Rc<RefCell<CallInfo>>> = Some(CallInfo::new());
    /* initialize stack array */
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && ((2i32 * 20i32) as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*L1).stack = luaM_realloc_(
        L,
        0 as *mut libc::c_void,
        (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
        ((2i32 * 20i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    (*L1).stacksize = 2i32 * 20i32;
    i = 0i32;
    while i < 2i32 * 20i32 {
        (*(*L1).stack.offset(i as isize)).tt_ = 0i32;
        i += 1
    }
    /* erase new stack */
    (*L1).top = (*L1).stack;
    (*L1).stack_last = (*L1).stack.offset((*L1).stacksize as isize).offset(-5isize);
    /* initialize first ci */
    *ci.as_ref().unwrap().borrow_mut() = (*L1).base_ci.clone();
    ci.as_ref().unwrap().borrow_mut().previous = None;
    ci.as_ref().unwrap().borrow_mut().next = None;
    ci.as_ref().unwrap().borrow_mut().callstatus = 0i32 as libc::c_ushort;
    ci.as_ref().unwrap().borrow_mut().func = (*L1).top;
    let fresh1 = (*L1).top;
    (*L1).top = (*L1).top.offset(1);
    (*fresh1).tt_ = 0i32;
    /* 'function' entry for this 'ci' */
    ci.as_ref().unwrap().borrow_mut().top = (*L1).top.offset(20isize);
    (*L1).ci = ci.as_ref().cloned();
}
/*
** Compute an initial seed as random as possible. Rely on Address Space
** Layout Randomization (if present) to increase randomness..
*/
unsafe extern "C" fn makeseed(mut L: *mut lua_State_0) -> libc::c_uint {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut h: libc::c_uint = time(0 as *mut time_t) as libc::c_uint;
    let mut p: libc::c_int = 0i32;
    let mut t: size_t = L as size_t;
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t as *mut size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    /* heap variable */
    let mut t_0: size_t = &mut h as *mut libc::c_uint as size_t;
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_0 as *mut size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    /* local variable */
    let mut t_1: size_t = &luaO_nilobject_ as *const TValue as size_t;
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_1 as *mut size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    /* global variable */
    let mut t_2: size_t = ::std::mem::transmute::<
        Option<unsafe extern "C" fn(_: lua_Alloc, _: *mut libc::c_void) -> *mut lua_State_0>,
        size_t,
    >(Some(lua_newstate));
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_2 as *mut size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    /* public function */
    if p as libc::c_ulong == ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong {
    } else {
        __assert_fail(
            b"p == sizeof(buff)\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            89i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"unsigned int makeseed(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    return luaS_hash(buff.as_mut_ptr(), p as size_t, h);
}
/*
** preinitialize a thread with consistent values without allocating
** any memory (to avoid errors)
*/
unsafe extern "C" fn preinit_thread(mut L: *mut lua_State_0, mut g: *mut global_State) -> () {
    (*L).l_G = g;
    (*L).stack = 0 as StkId;
    (*L).ci = None;
    (*L).nci = 0i32 as libc::c_ushort;
    (*L).stacksize = 0i32;
    /* thread has no upvalues */
    (*L).twups = L;
    (*L).errorJmp = 0 as *mut lua_longjmp;
    (*L).nCcalls = 0i32 as libc::c_ushort;
    ::std::ptr::write_volatile(&mut (*L).hook as *mut lua_Hook, None);
    (*L).hookmask = 0i32;
    (*L).basehookcount = 0i32;
    (*L).allowhook = 1i32 as lu_byte;
    (*L).hookcount = (*L).basehookcount;
    (*L).openupval = 0 as *mut UpVal;
    (*L).nny = 1i32 as libc::c_ushort;
    (*L).status = 0i32 as lu_byte;
    (*L).errfunc = 0i32 as ptrdiff_t;
}
#[no_mangle]
pub unsafe extern "C" fn lua_close(mut L: *mut lua_State_0) -> () {
    /* only the main thread can be closed */
    L = (*(*L).l_G).mainthread;
    let ref mut fresh2 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh3 = *fresh2;
    *fresh2 = *fresh2 + 1;
    if fresh3 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      343i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"void lua_close(lua_State *)\x00")).as_ptr());
    };
    close_state(L);
}
#[no_mangle]
pub unsafe extern "C" fn lua_newthread(mut L: *mut lua_State_0) -> *mut lua_State_0 {
    let mut g: *mut global_State = (*L).l_G;
    let mut L1: *mut lua_State_0 = 0 as *mut lua_State_0;
    let ref mut fresh4 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh5 = *fresh4;
    *fresh4 = *fresh4 + 1;
    if fresh5 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      258i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
    };
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    /* create new thread */
    L1 = &mut (*(luaM_realloc_(
        L,
        0 as *mut libc::c_void,
        8i32 as size_t,
        ::std::mem::size_of::<LX>() as libc::c_ulong,
    ) as *mut LX))
        .l;
    (*L1).marked = ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte;
    (*L1).tt = 8i32 as lu_byte;
    /* link it on list 'allgc' */
    (*L1).next = (*g).allgc;
    if (*L1).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((L1)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            266i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"lua_State *lua_newthread(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    (*g).allgc = &mut (*(L1 as *mut GCUnion)).gc;
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut lua_State_0 = L1;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            268i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"lua_State *lua_newthread(lua_State *)\x00",
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
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                268i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                    b"lua_State *lua_newthread(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
            && (L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lstate.c\x00" as *const u8 as *const libc::c_char,
                        268i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                            b"lua_State *lua_newthread(lua_State *)\x00",
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
                b"lstate.c\x00" as *const u8 as *const libc::c_char,
                268i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                    b"lua_State *lua_newthread(lua_State *)\x00",
                )).as_ptr(),
            );
        };
    };
    /* anchor it on L stack */
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*L).ci.as_ref().unwrap().borrow_mut().top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            269i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"lua_State *lua_newthread(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    preinit_thread(L1, g);
    (*L1).hookmask = (*L).hookmask;
    (*L1).basehookcount = (*L).basehookcount;
    ::std::ptr::write_volatile(&mut (*L1).hook as *mut lua_Hook, (*L).hook);
    (*L1).hookcount = (*L1).basehookcount;
    /* initialize L1 extra space */
    memcpy(
        (L1 as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void,
        ((*g).mainthread as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void,
        ::std::mem::size_of::<L_EXTRA>() as libc::c_ulong,
    );
    if (*((L1 as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock
        == (*((L as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void as *mut L_EXTRA))
            .plock
    {
    } else {
        __assert_fail(b"((struct L_EXTRA*)(((void *)((char *)(L1) - sizeof(struct L_EXTRA)))))->plock == ((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      278i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
    };
    /* init stack */
    stack_init(L1, L);
    let ref mut fresh6 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh6 -= 1;
    if *fresh6 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      280i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
    };
    return L1;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_setdebt(mut g: *mut global_State, mut debt: l_mem) -> () {
    let mut tb: l_mem = ((*g).totalbytes + (*g).GCdebt) as lu_mem as l_mem;
    if tb > 0i32 as libc::c_long {
    } else {
        __assert_fail(
            b"tb > 0\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            100i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"void luaE_setdebt(global_State *, l_mem)\x00",
            )).as_ptr(),
        );
    };
    if debt < tb - (!(0i32 as lu_mem) >> 1i32) as l_mem {
        /* will make 'totalbytes == MAX_LMEM' */
        debt = tb - (!(0i32 as lu_mem) >> 1i32) as l_mem
    }
    (*g).totalbytes = tb - debt;
    (*g).GCdebt = debt;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_freethread(mut L: *mut lua_State_0, mut L1: *mut lua_State_0) -> () {
    let mut l: *mut LX = (L1 as *mut lu_byte).offset(-16isize) as *mut LX;
    /* close all upvalues for this thread */
    luaF_close(L1, (*L1).stack);
    if (*L1).openupval.is_null() {
    } else {
        __assert_fail(
            b"L1->openupval == ((void*)0)\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            288i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"void luaE_freethread(lua_State *, lua_State *)\x00",
            )).as_ptr(),
        );
    };
    if (*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock
        == (*((L1 as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void as *mut L_EXTRA))
            .plock
    {
    } else {
        __assert_fail(b"((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock == ((struct L_EXTRA*)(((void *)((char *)(L1) - sizeof(struct L_EXTRA)))))->plock\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      289i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void luaE_freethread(lua_State *, lua_State *)\x00")).as_ptr());
    };
    freestack(L1);
    luaM_realloc_(
        L,
        l as *mut libc::c_void,
        ::std::mem::size_of::<LX>() as libc::c_ulong,
        0i32 as size_t,
    );
}

impl From<i32> for unnamed {
    fn from(item: i32) -> Self {
        let l = unnamed_1 {
            base: 0 as StkId,
            savedpc: 0 as *const Instruction,
        };
        unnamed { l }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaE_extendCI(mut L: *mut lua_State) -> Rc<RefCell<CallInfo>> {
    let ci: Rc<RefCell<CallInfo>> = CallInfo::new();
    if (*L).ci.as_ref().unwrap().borrow_mut().next.is_none() {
    } else {
        __assert_fail(
            b"L->ci->next == ((void*)0)\x00" as *const u8 as *const libc::c_char,
            b"lstate.c\x00" as *const u8 as *const libc::c_char,
            110i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"CallInfo *luaE_extendCI(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    (*L).ci.as_ref().unwrap().borrow_mut().next = Some(ci.clone());
    ci.as_ref().borrow_mut().previous = (*L).ci.as_ref().cloned();
    ci.as_ref().borrow_mut().next = None;
    (*L).nci = (*L).nci.wrapping_add(1);
    return ci;
}

#[no_mangle]
pub unsafe extern "C" fn luaE_shrinkCI(mut L: *mut lua_State) -> () {
    let mut ci: Option<Rc<RefCell<CallInfo>>> = (*L).ci.as_ref().cloned();
    /* next's next */
    let mut next2: Option<Rc<RefCell<CallInfo>>> = Some(CallInfo::new());
    /* while there are two nexts */
    while !ci.as_ref().unwrap().borrow().next.is_none() && {
        let mut temp = ci.as_ref().unwrap().borrow().next.as_ref().cloned();
        next2 = temp.as_ref().unwrap().borrow().next.as_ref().cloned();
        // next2 = ci.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().next.as_ref().cloned();
        !next2.is_none()
    } {
        /* free next */
        (*L).nci = (*L).nci.wrapping_sub(1);
        /* remove 'next' from the list */
        ci.as_ref().unwrap().borrow_mut().next = next2.as_ref().cloned();
        next2.as_ref().unwrap().borrow_mut().previous = ci.as_ref().cloned();
        /* keep next's next */
        ci = next2.as_ref().cloned()
    }
}
