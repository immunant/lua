use lobject::{
    lua_TValue, CClosure, Closure, GCObject, LClosure, Proto, TString, TValue, Udata, Value,
};
use lstate::{global_State, lua_State, CallInfo, GCUnion};
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn luaH_new(L: *mut lua_State_0) -> *mut Table_0;
    #[no_mangle]
    fn luaH_setint(
        L: *mut lua_State_0,
        t: *mut Table_0,
        key: lua_Integer,
        value: *mut TValue,
    ) -> ();
    #[no_mangle]
    fn luaF_getlocalname(
        func: *const Proto_0,
        local_number: libc::c_int,
        pc: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    fn luaO_chunkid(out: *mut libc::c_char, source: *const libc::c_char, len: size_t) -> ();
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaT_objtypename(L: *mut lua_State_0, o: *const TValue) -> *const libc::c_char;
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn luaG_runerror(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State_0, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State_0, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_hook(L: *mut lua_State_0, event: libc::c_int, line: libc::c_int) -> ();
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
#[derive(Clone)]
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
    pub i_ci: Option<Rc<RefCell<CallInfo>>>,
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
pub type TString_0 = TString;
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
pub type Closure_0 = Closure;
pub type LClosure_0 = LClosure;
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
** Closures
*/
pub type CClosure_0 = CClosure;
pub type Table_0 = Table;
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
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
/*	A B C	R(A) := R(B).. ... ..R(C)			*/
pub const OP_CONCAT: OpCode = 29;
/*	A B	R(A) := length of R(B)				*/
pub const OP_LEN: OpCode = 28;
/*	A B	R(A) := ~R(B)					*/
pub const OP_BNOT: OpCode = 26;
/*	A B	R(A) := -R(B)					*/
pub const OP_UNM: OpCode = 25;
/*	A B C	R(A) := RK(B) + RK(C)				*/
pub const OP_ADD: OpCode = 13;
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
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/*	A B	R(A) := not R(B)				*/
pub const OP_NOT: OpCode = 27;
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
/*
** Function Prototypes
*/
pub type Proto_0 = Proto;
#[no_mangle]
pub unsafe extern "C" fn lua_getstack(
    mut L: *mut lua_State_0,
    mut level: libc::c_int,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut ci: Option<Rc<RefCell<CallInfo>>> = Some(CallInfo::new());
    if level < 0i32 {
        /* invalid (negative) level */
        return 0i32;
    } else {
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
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          115i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"int lua_getstack(lua_State *, int, lua_Debug *)\x00")).as_ptr());
        };
        ci = (*L).ci.as_ref().cloned();
        while level > 0i32 && ci.as_ref().unwrap().as_ptr() != &mut (*L).base_ci as *mut CallInfo {
            level -= 1;
            let previous = ci.as_ref().unwrap().borrow_mut().previous.as_ref().cloned();
            ci = previous
        }
        if level == 0i32 && ci.as_ref().unwrap().as_ptr() != &mut (*L).base_ci as *mut CallInfo {
            /* level found? */
            status = 1i32;
            (*ar).i_ci = ci.as_ref().cloned()
        } else {
            status = 0i32
        }
        let ref mut fresh2 = *(*((L as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void as *mut L_EXTRA))
            .plock;
        *fresh2 -= 1;
        if *fresh2 == 0i32 {
        } else {
            __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          123i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"int lua_getstack(lua_State *, int, lua_Debug *)\x00")).as_ptr());
        };
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_getinfo(
    mut L: *mut lua_State_0,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut cl: *mut Closure = 0 as *mut Closure;
    let mut ci: Option<Rc<RefCell<CallInfo>>> = Some(CallInfo::new());
    let mut func: StkId = 0 as *mut TValue;
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
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      315i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
    };
    swapextra(L);
    if *what as libc::c_int == '>' as i32 {
        ci = Some(CallInfo::new());
        func = (*L).top.offset(-1isize);
        if (*func).tt_ & 0xfi32 == 6i32
            && !(b"function expected\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(((((((func)->tt_)) & 0x0F)) == (6))) && \"function expected\"\x00" as *const u8
                    as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                320i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00",
                )).as_ptr(),
            );
        };
        /* skip the '>' */
        what = what.offset(1isize);
        /* pop function */
        (*L).top = (*L).top.offset(-1isize)
    } else {
        ci = (*ar).i_ci.as_ref().cloned();

        func = (ci.as_ref().unwrap().borrow_mut()).func;
        if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ & 0xfi32 == 6i32 {
        } else {
            __assert_fail(
                b"((((((ci->func)->tt_)) & 0x0F)) == (6))\x00" as *const u8 as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                327i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00",
                )).as_ptr(),
            );
        };
    }
    cl = if (*func).tt_ & 0x1fi32 == 6i32 {
        if (*func).tt_ & 0x1fi32 == 6i32 {
        } else {
            __assert_fail(
                b"((((func)->tt_) & 0x1F) == 6)\x00" as *const u8 as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                329i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*func).value_.gc).tt as libc::c_int & 0xfi32 == 6i32 {
        } else {
            __assert_fail(
                b"(((((func)->value_).gc)->tt) & 0x0F) == 6\x00" as *const u8
                    as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                329i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00",
                )).as_ptr(),
            );
        };
        &mut (*((*func).value_.gc as *mut GCUnion)).cl
    } else {
        0 as *mut Closure_0
    };
    status = auxgetinfo(L, what, ar, cl, ci);
    if !strchr(what, 'f' as i32).is_null() {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *func;
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    332i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                        b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int
                && (L.is_null() || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                            332i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                                b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00",
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
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    332i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                        b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00",
                    )).as_ptr(),
                );
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= ((*L).ci.as_ref().unwrap().borrow_mut()).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                333i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00",
                )).as_ptr(),
            );
        };
    }
    /* correct before option 'L', which can raise a mem. error */
    swapextra(L);
    if !strchr(what, 'L' as i32).is_null() {
        collectvalidlines(L, cl);
    }
    let ref mut fresh5 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh5 -= 1;
    if *fresh5 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      338i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
    };
    return status;
}
unsafe extern "C" fn collectvalidlines(mut L: *mut lua_State_0, mut f: *mut Closure) -> () {
    if f.is_null() || (*f).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
        (*(*L).top).tt_ = 0i32;
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= ((*L).ci.as_ref().unwrap().borrow_mut()).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                233i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"void collectvalidlines(lua_State *, Closure *)\x00",
                )).as_ptr(),
            );
        };
    } else {
        let mut i: libc::c_int = 0;
        let mut v: TValue = lua_TValue {
            value_: Value_0 {
                gc: 0 as *mut GCObject,
            },
            tt_: 0,
        };
        let mut lineinfo: *mut libc::c_int = (*(*f).l.p).lineinfo;
        /* new table to store active lines */
        let mut t: *mut Table_0 = luaH_new(L);
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut Table_0 = t;
        if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                240i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"void collectvalidlines(lua_State *, Closure *)\x00",
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
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    240i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"void collectvalidlines(lua_State *, Closure *)\x00",
                    )).as_ptr(),
                );
            };
            (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
                && (L.is_null() || {
                    if 0 != (*io).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                            240i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                                b"void collectvalidlines(lua_State *, Closure *)\x00",
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
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    240i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"void collectvalidlines(lua_State *, Closure *)\x00",
                    )).as_ptr(),
                );
            };
        };
        /* push it on stack */
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= ((*L).ci.as_ref().unwrap().borrow_mut()).top
            && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                    as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                241i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"void collectvalidlines(lua_State *, Closure *)\x00",
                )).as_ptr(),
            );
        };
        let mut io_0: *mut TValue = &mut v;
        (*io_0).value_.b = 1i32;
        (*io_0).tt_ = 1i32;
        /* boolean 'true' to be the value of all indices */
        /* for all lines with code */
        i = 0i32;
        while i < (*(*f).l.p).sizelineinfo {
            /* table[line] = true */
            luaH_setint(L, t, *lineinfo.offset(i as isize) as lua_Integer, &mut v);
            i += 1
        }
    };
}
/*
** If function yielded, its 'func' can be in the 'extra' field. The
** next function restores 'func' to its correct value for debugging
** purposes. (It exchanges 'func' and 'extra'; so, when called again,
** after debugging, it also "re-restores" ** 'func' to its altered value.
*/
unsafe extern "C" fn swapextra(mut L: *mut lua_State_0) -> () {
    if (*L).status as libc::c_int == 1i32 {
        /* get function that yielded */
        let mut ci: Option<Rc<RefCell<CallInfo>>> = (*L).ci.as_ref().cloned();
        /* exchange its 'func' and 'extra' values */
        let mut temp: StkId = (ci.as_ref().unwrap().borrow_mut()).func;
        ci.as_ref().unwrap().borrow_mut().func = ((*L).stack as *mut libc::c_char).offset((ci.as_ref().unwrap().borrow_mut()).extra as isize) as *mut TValue;
        let some_value = (temp as *mut libc::c_char).wrapping_offset_from((*L).stack as *mut libc::c_char) as libc::c_long;
        ci.as_ref().unwrap().borrow_mut().extra = some_value;
    };
}
unsafe extern "C" fn auxgetinfo(
    mut L: *mut lua_State_0,
    mut what: *const libc::c_char,
    mut ar: *mut lua_Debug,
    mut f: *mut Closure,
    mut ci: Option<Rc<RefCell<CallInfo>>>,
) -> libc::c_int {
    let mut status: libc::c_int = 1i32;
    while 0 != *what {
        match *what as libc::c_int {
            83 => {
                funcinfo(ar, f);
            }
            108 => {
                (*ar).currentline =
                    if !ci.is_none() && 0 != (ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & 1i32 << 1i32 {
                        currentline(ci.as_ref().cloned())
                    } else {
                        -1i32
                    }
            }
            117 => {
                (*ar).nups = (if f.is_null() {
                    0i32
                } else {
                    (*f).c.nupvalues as libc::c_int
                }) as libc::c_uchar;
                if f.is_null() || (*f).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                    (*ar).isvararg = 1i32 as libc::c_char;
                    (*ar).nparams = 0i32 as libc::c_uchar
                } else {
                    (*ar).isvararg = (*(*f).l.p).is_vararg as libc::c_char;
                    (*ar).nparams = (*(*f).l.p).numparams
                }
            }
            116 => {
                (*ar).istailcall = (if !ci.is_none() {
                    (ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & 1i32 << 5i32
                } else {
                    0i32
                }) as libc::c_char
            }
            110 => {
                (*ar).namewhat = getfuncname(L, ci.as_ref().cloned(), &mut (*ar).name);
                if (*ar).namewhat.is_null() {
                    /* not found */
                    (*ar).namewhat = b"\x00" as *const u8 as *const libc::c_char;
                    (*ar).name = 0 as *const libc::c_char
                }
            }
            76 => {}
            102 => {}
            _ => {
                /* handled by lua_getinfo */
                /* invalid option */
                status = 0i32
            }
        }
        what = what.offset(1isize)
    }
    return status;
}
unsafe extern "C" fn getfuncname(
    mut L: *mut lua_State_0,
    mut ci: Option<Rc<RefCell<CallInfo>>>,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    /* no 'ci'? */
    let callstatus = ci.as_ref().unwrap().borrow_mut().callstatus;
    let previous = ci.as_ref().unwrap().borrow_mut().previous.as_ref().cloned();
    let previous_callstatus = previous.as_ref().unwrap().borrow_mut().callstatus;
    if ci.is_none() {
        /* no info */
        return 0 as *const libc::c_char;
    } else if 0 != callstatus as libc::c_int & 1i32 << 8i32 {
        /* is this a finalizer? */
        *name = b"__gc\x00" as *const u8 as *const libc::c_char;
        /* report it as such */
        return b"metamethod\x00" as *const u8 as *const libc::c_char;
    } else if 0 == callstatus as libc::c_int & 1i32 << 5i32
        && 0 != previous_callstatus as libc::c_int & 1i32 << 1i32
    {
        return funcnamefromcode(L, previous, name);
    } else {
        return 0 as *const libc::c_char;
    };
}
/*
** $Id: ldebug.c,v 2.120 2016/03/31 19:01:21 roberto Exp roberto $
** Debug Interface
** See Copyright Notice in lua.h
*/
/* Active Lua function (given call info) */
unsafe extern "C" fn funcnamefromcode(
    mut L: *mut lua_State_0,
    mut ci: Option<Rc<RefCell<CallInfo>>>,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    /* (initial value avoids warnings) */
    let mut tm: TMS = TM_INDEX;
    /* calling function */
    if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            495i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                b"const char *funcnamefromcode(lua_State *, CallInfo *, const char **)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            495i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                b"const char *funcnamefromcode(lua_State *, CallInfo *, const char **)\x00",
            )).as_ptr(),
        );
    };
    let mut p: *mut Proto_0 = (*&mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l).p;
    /* calling instruction index */
    let mut pc: libc::c_int = currentpc(ci.as_ref().cloned());
    /* calling instruction */
    let mut i: Instruction = *(*p).code.offset(pc as isize);
    if 0 != (ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & 1i32 << 2i32 {
        /* was it called inside a hook? */
        *name = b"?\x00" as *const u8 as *const libc::c_char;
        return b"hook\x00" as *const u8 as *const libc::c_char;
    } else {
        match (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint {
            36 | 37 => {
                /* get function name */
                return getobjname(
                    p,
                    pc,
                    (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int,
                    name,
                );
            }
            41 => {
                /* for iterator */
                *name = b"for iterator\x00" as *const u8 as *const libc::c_char;
                return b"for iterator\x00" as *const u8 as *const libc::c_char;
            }
            12 | 6 | 7 => tm = TM_INDEX,
            8 | 10 => tm = TM_NEWINDEX,
            13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => {
                /* ORDER OP */
                let mut offset: libc::c_int = (i >> 0i32
                    & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                    as OpCode as libc::c_int
                    - OP_ADD as libc::c_int;
                /* ORDER TM */
                tm = (offset + TM_ADD as libc::c_int) as TMS
            }
            25 => tm = TM_UNM,
            26 => tm = TM_BNOT,
            28 => tm = TM_LEN,
            29 => tm = TM_CONCAT,
            31 => tm = TM_EQ,
            32 => tm = TM_LT,
            33 => tm = TM_LE,
            _ => {
                /* cannot find a reasonable name */
                return 0 as *const libc::c_char;
            }
        }
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof(((L->l_G)->tmname[tm])->extra)\x00" as *const u8 as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                534i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"const char *funcnamefromcode(lua_State *, CallInfo *, const char **)\x00",
                )).as_ptr(),
            );
        };
        *name = ((*(*L).l_G).tmname[tm as usize] as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
        return b"metamethod\x00" as *const u8 as *const libc::c_char;
    };
}
unsafe extern "C" fn currentpc(mut ci: Option<Rc<RefCell<CallInfo>>>) -> libc::c_int {
    if 0 != (ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & 1i32 << 1i32 {
    } else {
        __assert_fail(
            b"((ci)->callstatus & (1<<1))\x00" as *const u8 as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            47i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"int currentpc(CallInfo *)\x00",
            )).as_ptr(),
        );
    };
    if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            48i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"int currentpc(CallInfo *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            48i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"int currentpc(CallInfo *)\x00",
            )).as_ptr(),
        );
    };
    return (ci.as_ref().unwrap().borrow_mut())
        .u
        .l
        .savedpc
        .wrapping_offset_from((*(*&mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l).p).code)
        as libc::c_long as libc::c_int
        - 1i32;
}
/*
** {======================================================
** Symbolic Execution
** =======================================================
*/
unsafe extern "C" fn getobjname(
    mut p: *mut Proto_0,
    mut lastpc: libc::c_int,
    mut reg: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut pc: libc::c_int = 0;
    *name = luaF_getlocalname(p, reg + 1i32, lastpc);
    /* is a local? */
    if !(*name).is_null() {
        return b"local\x00" as *const u8 as *const libc::c_char;
    } else {
        /* else try symbolic execution */
        pc = findsetreg(p, lastpc, reg);
        if pc != -1i32 {
            /* could find instruction? */
            let mut i: Instruction = *(*p).code.offset(pc as isize);
            let mut op: OpCode =
                (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
            match op as libc::c_uint {
                0 => {
                    /* move from 'b' to 'a' */
                    let mut b: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    if b < (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                        as libc::c_int
                    {
                        /* get name for 'b' */
                        return getobjname(p, pc, b, name);
                    }
                }
                6 | 7 => {
                    /* key index */
                    let mut k: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    /* table index */
                    let mut t: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    /* name of indexed variable */
                    let mut vn: *const libc::c_char =
                        if op as libc::c_uint == OP_GETTABLE as libc::c_int as libc::c_uint {
                            luaF_getlocalname(p, t + 1i32, pc)
                        } else {
                            upvalname(p, t)
                        };
                    kname(p, pc, k, name);
                    return if !vn.is_null()
                        && strcmp(vn, b"_ENV\x00" as *const u8 as *const libc::c_char) == 0i32
                    {
                        b"global\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"field\x00" as *const u8 as *const libc::c_char
                    };
                }
                5 => {
                    *name = upvalname(
                        p,
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as libc::c_int,
                    );
                    return b"upvalue\x00" as *const u8 as *const libc::c_char;
                }
                1 | 2 => {
                    let mut b_0: libc::c_int =
                        if op as libc::c_uint == OP_LOADK as libc::c_int as libc::c_uint {
                            (i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as libc::c_int
                        } else {
                            (*(*p).code.offset((pc + 1i32) as isize) >> 0i32 + 6i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
                                as libc::c_int
                        };
                    if (*(*p).k.offset(b_0 as isize)).tt_ & 0xfi32 == 4i32 {
                        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
                        } else {
                            __assert_fail(b"sizeof((((((((((((&p->k[b]))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (((((((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((&p->k[b])->value_).gc))))->ts))))))->extra)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"ldebug.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          469i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 57],
                                                                    &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                        };
                        if (*(*p).k.offset(b_0 as isize)).tt_ & 0xfi32 == 4i32 {
                        } else {
                            __assert_fail(
                                b"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                                    as *const libc::c_char,
                                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                                469i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                                    b"const char *getobjname(Proto *, int, int, const char **)\x00",
                                )).as_ptr(),
                            );
                        };
                        if (*(*(*p).k.offset(b_0 as isize)).value_.gc).tt as libc::c_int & 0xfi32
                            == 4i32
                        {
                        } else {
                            __assert_fail(
                                b"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                                    as *const libc::c_char,
                                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                                469i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                                    b"const char *getobjname(Proto *, int, int, const char **)\x00",
                                )).as_ptr(),
                            );
                        };
                        *name = (&mut (*((*(*p).k.offset(b_0 as isize)).value_.gc as *mut GCUnion))
                            .ts as *mut TString_0
                            as *mut libc::c_char)
                            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
                        return b"constant\x00" as *const u8 as *const libc::c_char;
                    }
                }
                12 => {
                    /* key index */
                    let mut k_0: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int;
                    kname(p, pc, k_0, name);
                    return b"method\x00" as *const u8 as *const libc::c_char;
                }
                _ => {}
            }
        }
        /* go through to return NULL */
        /* could not find reasonable name */
        return 0 as *const libc::c_char;
    };
}
/*
** find a "name" for the RK value 'c'
*/
unsafe extern "C" fn kname(
    mut p: *mut Proto_0,
    mut pc: libc::c_int,
    mut c: libc::c_int,
    mut name: *mut *const libc::c_char,
) -> () {
    if 0 != c & 1i32 << 9i32 - 1i32 {
        /* is 'c' a constant? */
        let mut kvalue: *mut TValue =
            &mut *(*p).k.offset((c & !(1i32 << 9i32 - 1i32)) as isize) as *mut TValue;
        if (*kvalue).tt_ & 0xfi32 == 4i32 {
            /* literal constant? */
            /* it is its own name */
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(b"sizeof((((((((((((kvalue))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((kvalue))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 360, __extension__ __PRETTY_FUNCTION__)), (((((((((kvalue)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((kvalue)->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 360, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((kvalue)->value_).gc))))->ts))))))->extra)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 360i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 45],
                                                        &[libc::c_char; 45]>(b"void kname(Proto *, int, int, const char **)\x00")).as_ptr());
            };
            if (*kvalue).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((kvalue))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    360i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                        b"void kname(Proto *, int, int, const char **)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*kvalue).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((kvalue)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    360i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                        b"void kname(Proto *, int, int, const char **)\x00",
                    )).as_ptr(),
                );
            };
            *name = (&mut (*((*kvalue).value_.gc as *mut GCUnion)).ts as *mut TString_0
                as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
            return;
        }
    } else {
        /* else no reasonable name found */
        /* 'c' is a register */
        /* search for 'c' */
        let mut what: *const libc::c_char = getobjname(p, pc, c, name);
        if !what.is_null() && *what as libc::c_int == 'c' as i32 {
            /* found a constant name? */
            /* 'name' already filled */
            return;
        }
    }
    /* else no reasonable name found */
    /* no reasonable name found */
    *name = b"?\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn upvalname(mut p: *mut Proto_0, mut uv: libc::c_int) -> *const libc::c_char {
    if uv < (*p).sizeupvalues {
    } else {
        __assert_fail(
            b"uv < p->sizeupvalues\x00" as *const u8 as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            129i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"const char *upvalname(Proto *, int)\x00",
            )).as_ptr(),
        );
    };
    let mut s: *mut TString = (*(*p).upvalues.offset(uv as isize)).name;
    if s.is_null() {
        return b"?\x00" as *const u8 as *const libc::c_char;
    } else {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((s)->extra)\x00" as *const u8 as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                131i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                    b"const char *upvalname(Proto *, int)\x00",
                )).as_ptr(),
            );
        };
        return (s as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
    };
}
/*
** try to find last instruction before 'lastpc' that modified register 'reg'
*/
unsafe extern "C" fn findsetreg(
    mut p: *mut Proto_0,
    mut lastpc: libc::c_int,
    mut reg: libc::c_int,
) -> libc::c_int {
    let mut pc: libc::c_int = 0;
    /* keep last instruction that changed 'reg' */
    let mut setreg: libc::c_int = -1i32;
    /* any code before this address is conditional */
    let mut jmptarget: libc::c_int = 0i32;
    pc = 0i32;
    while pc < lastpc {
        let mut i: Instruction = *(*p).code.offset(pc as isize);
        let mut op: OpCode = (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
        let mut a: libc::c_int =
            (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int;
        match op as libc::c_uint {
            4 => {
                let mut b: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                    as libc::c_int;
                /* set registers from 'a' to 'a+b' */
                if a <= reg && reg <= a + b {
                    setreg = filterpc(pc, jmptarget)
                }
            }
            41 => {
                /* affect all regs above its base */
                if reg >= a + 2i32 {
                    setreg = filterpc(pc, jmptarget)
                }
            }
            36 | 37 => {
                /* affect all registers above base */
                if reg >= a {
                    setreg = filterpc(pc, jmptarget)
                }
            }
            30 => {
                let mut b_0: libc::c_int = (i >> 0i32 + 6i32 + 8i32
                    & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                    as libc::c_int
                    - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32);
                let mut dest: libc::c_int = pc + 1i32 + b_0;
                /* jump is forward and do not skip 'lastpc'? */
                if pc < dest && dest <= lastpc {
                    if dest > jmptarget {
                        /* update 'jmptarget' */
                        jmptarget = dest
                    }
                }
            }
            _ => {
                /* any instruction that set A */
                if 0 != luaP_opmodes[op as usize] as libc::c_int & 1i32 << 6i32 && reg == a {
                    setreg = filterpc(pc, jmptarget)
                }
            }
        }
        pc += 1
    }
    return setreg;
}
unsafe extern "C" fn filterpc(mut pc: libc::c_int, mut jmptarget: libc::c_int) -> libc::c_int {
    /* is code conditional (inside a jump)? */
    if pc < jmptarget {
        /* cannot know who sets that register */
        return -1i32;
    } else {
        return pc;
    };
}
#[no_mangle]
pub unsafe extern "C" fn currentline(mut ci: Option<Rc<RefCell<CallInfo>>>) -> libc::c_int {
    if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            53i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"int currentline(CallInfo *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            53i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"int currentline(CallInfo *)\x00",
            )).as_ptr(),
        );
    };
    return if !(*(*(&mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l as *mut LClosure)).p)
        .lineinfo
        .is_null()
    {
        if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                    as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                53i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"int currentline(CallInfo *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                    as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                53i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"int currentline(CallInfo *)\x00",
                )).as_ptr(),
            );
        };
        *(*(*&mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l).p)
            .lineinfo
            .offset(currentpc(ci.as_ref().cloned()) as isize)
    } else {
        -1i32
    };
}
unsafe extern "C" fn funcinfo(mut ar: *mut lua_Debug, mut cl: *mut Closure) -> () {
    let mut p: *mut Proto_0 = 0 as *mut Proto_0;
    if cl.is_null() || (*cl).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
        (*ar).source = b"=[C]\x00" as *const u8 as *const libc::c_char;
        (*ar).linedefined = -1i32;
        (*ar).lastlinedefined = -1i32;
        (*ar).what = b"C\x00" as *const u8 as *const libc::c_char
    } else {
        p = (*cl).l.p;
        (*ar).source = if !(*p).source.is_null() {
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(
                    b"sizeof((p->source)->extra)\x00" as *const u8 as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    221i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                        b"void funcinfo(lua_Debug *, Closure *)\x00",
                    )).as_ptr(),
                );
            };
            ((*p).source as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
        } else {
            b"=?\x00" as *const u8 as *const libc::c_char
        };
        (*ar).linedefined = (*p).linedefined;
        (*ar).lastlinedefined = (*p).lastlinedefined;
        (*ar).what = if (*ar).linedefined == 0i32 {
            b"main\x00" as *const u8 as *const libc::c_char
        } else {
            b"Lua\x00" as *const u8 as *const libc::c_char
        }
    }
    luaO_chunkid((*ar).short_src.as_mut_ptr(), (*ar).source, 60i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn lua_getlocal(
    mut L: *mut lua_State_0,
    mut ar: *const lua_Debug,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let ref mut fresh6 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    if fresh7 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      174i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
    };
    swapextra(L);
    if ar.is_null() {
        /* information about non-active function? */
        /* not a Lua function? */
        if !((*(*L).top.offset(-1isize)).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32) {
            name = 0 as *const libc::c_char
        } else {
            /* consider live variables at function start (parameters) */
            if (*(*L).top.offset(-1isize)).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"((((L->top - 1))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    180i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                        b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(*L).top.offset(-1isize)).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"(((L->top - 1)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    180i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                        b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00",
                    )).as_ptr(),
                );
            };
            name = luaF_getlocalname(
                (*&mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion))
                    .cl
                    .l)
                    .p,
                n,
                0i32,
            )
        }
    } else {
        /* active function; get information through 'ar' */
        /* to avoid warnings */
        let mut pos: StkId = 0 as StkId;
        name = findlocal(L, (*ar).i_ci.as_ref().cloned(), n, &mut pos);
        if !name.is_null() {
            let mut io1: *mut TValue = (*L).top;
            *io1 = *pos;
            if 0 == (*io1).tt_ & 1i32 << 6i32
                || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                        186i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                            b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00",
                        )).as_ptr(),
                    );
                    };
                    (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int
                        && (L.is_null()
                            || {
                                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ldebug.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      186i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 62],
                                                                                &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
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
                        b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                        186i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                            b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00",
                        )).as_ptr(),
                    );
                };
            };
            (*L).top = (*L).top.offset(1isize);
            if (*L).top <= ((*L).ci.as_ref().unwrap().borrow_mut()).top
                && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
            {
            } else {
                __assert_fail(
                    b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    187i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                        b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00",
                    )).as_ptr(),
                );
            };
        }
    }
    swapextra(L);
    let ref mut fresh8 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh8 -= 1;
    if *fresh8 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      191i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
    };
    return name;
}
unsafe extern "C" fn findlocal(
    mut L: *mut lua_State_0,
    mut ptr_ci: Option<Rc<RefCell<CallInfo>>>,
    mut n: libc::c_int,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    let mut ci: Option<Rc<RefCell<CallInfo>>> = ptr_ci.as_ref().cloned();
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut base: StkId = 0 as *mut TValue;
    if 0 != (ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & 1i32 << 1i32 {
        /* access to vararg values? */
        if n < 0i32 {
            return findvararg(ci, -n, pos);
        } else {
            base = (ci.as_ref().unwrap().borrow_mut()).u.l.base;
            if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    155i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                        b"const char *findlocal(lua_State *, CallInfo *, int, StkId *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    155i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                        b"const char *findlocal(lua_State *, CallInfo *, int, StkId *)\x00",
                    )).as_ptr(),
                );
            };
            name = luaF_getlocalname(
                (*&mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l).p,
                n,
                currentpc(ci.as_ref().cloned()),
            )
        }
    } else {
        base = (ci.as_ref().unwrap().borrow_mut()).func.offset(1isize)
    }
    if name.is_null() {
        /* no 'standard' name? */
        let mut limit: StkId = if ci.as_ref().unwrap().as_ptr() == (*L).ci.as_ref().unwrap().as_ptr() {
            (*L).top
        } else {
            let next = ci.as_ref().unwrap().borrow_mut().next.as_ref().cloned();
            let func = next.as_ref().unwrap().borrow_mut().func;
            func
        };
        /* is 'n' inside 'ci' stack? */
        if limit.wrapping_offset_from(base) as libc::c_long >= n as libc::c_long && n > 0i32 {
            /* generic name for any valid slot */
            name = b"(*temporary)\x00" as *const u8 as *const libc::c_char
        } else {
            return 0 as *const libc::c_char;
        }
    }
    *pos = base.offset((n - 1i32) as isize);
    return name;
}
unsafe extern "C" fn findvararg(
    mut ci: Option<Rc<RefCell<CallInfo>>>,
    mut n: libc::c_int,
    mut pos: *mut StkId,
) -> *const libc::c_char {
    if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"((((ci->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            136i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"const char *findvararg(CallInfo *, int, StkId *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"(((ci->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            136i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                b"const char *findvararg(CallInfo *, int, StkId *)\x00",
            )).as_ptr(),
        );
    };
    let mut nparams: libc::c_int =
        (*(*&mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l).p).numparams as libc::c_int;
    if n >= (ci.as_ref().unwrap().borrow_mut()).u.l.base.wrapping_offset_from((ci.as_ref().unwrap().borrow_mut()).func) as libc::c_long as libc::c_int - nparams
    {
        /* no such vararg */
        return 0 as *const libc::c_char;
    } else {
        *pos = (ci.as_ref().unwrap().borrow_mut()).func.offset(nparams as isize).offset(n as isize);
        /* generic name for any vararg */
        return b"(*vararg)\x00" as *const u8 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setlocal(
    mut L: *mut lua_State_0,
    mut ar: *const lua_Debug,
    mut n: libc::c_int,
) -> *const libc::c_char {
    /* to avoid warnings */
    let mut pos: StkId = 0 as StkId;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let ref mut fresh9 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    let fresh10 = *fresh9;
    *fresh9 = *fresh9 + 1;
    if fresh10 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      199i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
    };
    swapextra(L);
    name = findlocal(L, (*ar).i_ci.as_ref().cloned(), n, &mut pos);
    if !name.is_null() {
        let mut io1: *mut TValue = pos;
        *io1 = *(*L).top.offset(-1isize);
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    203i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                        b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int
                && (L.is_null() || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                        203i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                            b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00",
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
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    203i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                        b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00",
                    )).as_ptr(),
                );
            };
        };
        /* pop value */
        (*L).top = (*L).top.offset(-1isize)
    }
    swapextra(L);
    let ref mut fresh11 = *(*((L as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
        as *mut libc::c_void as *mut L_EXTRA))
        .plock;
    *fresh11 -= 1;
    if *fresh11 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      207i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
    };
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_sethook(
    mut L: *mut lua_State_0,
    mut func: lua_Hook,
    mut mask: libc::c_int,
    mut count: libc::c_int,
) -> () {
    if func.is_none() || mask == 0i32 {
        /* turn off hooks? */
        mask = 0i32;
        func = None
    }
    if 0 != ((*L).ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & 1i32 << 1i32 {
        (*L).oldpc = ((*L).ci.as_ref().unwrap().borrow_mut()).u.l.savedpc
    }
    ::std::ptr::write_volatile(&mut (*L).hook as *mut lua_Hook, func);
    (*L).basehookcount = count;
    (*L).hookcount = (*L).basehookcount;
    (*L).hookmask = mask as lu_byte as sig_atomic_t;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethook(mut L: *mut lua_State_0) -> lua_Hook {
    return (*L).hook;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethookmask(mut L: *mut lua_State_0) -> libc::c_int {
    return (*L).hookmask;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethookcount(mut L: *mut lua_State_0) -> libc::c_int {
    return (*L).basehookcount;
}
#[no_mangle]
pub unsafe extern "C" fn luaG_typeerror(
    mut L: *mut lua_State_0,
    mut o: *const TValue,
    mut op: *const libc::c_char,
) -> ! {
    let mut t: *const libc::c_char = luaT_objtypename(L, o);
    luaG_runerror(
        L,
        b"attempt to %s a %s value%s\x00" as *const u8 as *const libc::c_char,
        op,
        t,
        varinfo(L, o),
    );
}
unsafe extern "C" fn varinfo(mut L: *mut lua_State_0, mut o: *const TValue) -> *const libc::c_char {
    /* to avoid warnings */
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ci: Option<Rc<RefCell<CallInfo>>> = (*L).ci.as_ref().cloned();
    let mut kind: *const libc::c_char = 0 as *const libc::c_char;
    if 0 != (ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & 1i32 << 1i32 {
        /* check whether 'o' is an upvalue */
        kind = getupvalname(ci.as_ref().cloned(), o, &mut name);
        /* no? try a register */
        if kind.is_null() && 0 != isinstack(ci.as_ref().cloned(), o) {
            if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    579i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                        b"const char *varinfo(lua_State *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    579i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                        b"const char *varinfo(lua_State *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            kind = getobjname(
                (*&mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l).p,
                currentpc(ci.as_ref().cloned()),
                o.wrapping_offset_from((ci.as_ref().unwrap().borrow_mut()).u.l.base) as libc::c_long as libc::c_int,
                &mut name,
            )
        }
    }
    return if !kind.is_null() {
        luaO_pushfstring(
            L,
            b" (%s \'%s\')\x00" as *const u8 as *const libc::c_char,
            kind,
            name,
        )
    } else {
        b"\x00" as *const u8 as *const libc::c_char
    };
}
/* }====================================================== */
/*
** The subtraction of two potentially unrelated pointers is
** not ISO C, but it should not crash a program; the subsequent
** checks are ISO C and ensure a correct result.
*/
unsafe extern "C" fn isinstack(mut ci: Option<Rc<RefCell<CallInfo>>>, mut o: *const TValue) -> libc::c_int {
    let mut i: ptrdiff_t = o.wrapping_offset_from((ci.as_ref().unwrap().borrow_mut()).u.l.base) as libc::c_long;
    return (0i32 as libc::c_long <= i
        && i < (ci.as_ref().unwrap().borrow_mut()).top.wrapping_offset_from((ci.as_ref().unwrap().borrow_mut()).u.l.base) as libc::c_long
        && (ci.as_ref().unwrap().borrow_mut()).u.l.base.offset(i as isize) == o as StkId) as libc::c_int;
}
/*
** Checks whether value 'o' came from an upvalue. (That can only happen
** with instructions OP_GETTABUP/OP_SETTABUP, which operate directly on
** upvalues.)
*/
unsafe extern "C" fn getupvalname(
    mut ci: Option<Rc<RefCell<CallInfo>>>,
    mut o: *const TValue,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            560i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 68], &[libc::c_char; 68]>(
                b"const char *getupvalname(CallInfo *, const TValue *, const char **)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            560i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 68], &[libc::c_char; 68]>(
                b"const char *getupvalname(CallInfo *, const TValue *, const char **)\x00",
            )).as_ptr(),
        );
    };
    let mut c: *mut LClosure = &mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*c).nupvalues as libc::c_int {
        if (**(*c).upvals.as_mut_ptr().offset(i as isize)).v == o as *mut TValue {
            *name = upvalname((*c).p, i);
            return b"upvalue\x00" as *const u8 as *const libc::c_char;
        } else {
            i += 1
        }
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn luaG_concaterror(
    mut L: *mut lua_State_0,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    if (*p1).tt_ & 0xfi32 == 4i32 || (*p1).tt_ & 0xfi32 == 3i32 {
        p1 = p2
    }
    luaG_typeerror(
        L,
        p1,
        b"concatenate\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_opinterror(
    mut L: *mut lua_State_0,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut msg: *const libc::c_char,
) -> ! {
    let mut temp: lua_Number = 0.;
    /* first operand is wrong? */
    if 0 == if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
        if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(b"((((p1))->tt_) == ((3 | (0 << 4))))\x00" as
                                     *const u8 as *const libc::c_char,
                                 b"ldebug.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 601i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 80],
                                                           &[libc::c_char; 80]>(b"void luaG_opinterror(lua_State *, const TValue *, const TValue *, const char *)\x00")).as_ptr());
        };
        temp = (*p1).value_.n;
        1i32
    } else {
        luaV_tonumber_(p1, &mut temp)
    } {
        /* now second is wrong */
        p2 = p1
    }
    luaG_typeerror(L, p2, msg);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_tointerror(
    mut L: *mut lua_State_0,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut temp: lua_Integer = 0;
    if 0 == if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
        if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((p1))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                612i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                    b"void luaG_tointerror(lua_State *, const TValue *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        temp = (*p1).value_.i;
        1i32
    } else {
        luaV_tointeger(p1, &mut temp, 0i32)
    } {
        p2 = p1
    }
    luaG_runerror(
        L,
        b"number%s has no integer representation\x00" as *const u8 as *const libc::c_char,
        varinfo(L, p2),
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_ordererror(
    mut L: *mut lua_State_0,
    mut p1: *const TValue,
    mut p2: *const TValue,
) -> ! {
    let mut t1: *const libc::c_char = luaT_objtypename(L, p1);
    let mut t2: *const libc::c_char = luaT_objtypename(L, p2);
    if strcmp(t1, t2) == 0i32 {
        luaG_runerror(
            L,
            b"attempt to compare two %s values\x00" as *const u8 as *const libc::c_char,
            t1,
        );
    } else {
        luaG_runerror(
            L,
            b"attempt to compare %s with %s\x00" as *const u8 as *const libc::c_char,
            t1,
            t2,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaG_addinfo(
    mut L: *mut lua_State_0,
    mut msg: *const libc::c_char,
    mut src: *mut TString,
    mut line: libc::c_int,
) -> *const libc::c_char {
    let mut buff: [libc::c_char; 60] = [0; 60];
    if !src.is_null() {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(
                b"sizeof((src)->extra)\x00" as *const u8 as *const libc::c_char,
                b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                633i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 68], &[libc::c_char; 68]>(
                    b"const char *luaG_addinfo(lua_State *, const char *, TString *, int)\x00",
                )).as_ptr(),
            );
        };
        luaO_chunkid(
            buff.as_mut_ptr(),
            (src as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize),
            60i32 as size_t,
        );
    } else {
        /* no source available; use "?" instead */
        buff[0usize] = '?' as i32 as libc::c_char;
        buff[1usize] = '\u{0}' as i32 as libc::c_char
    }
    return luaO_pushfstring(
        L,
        b"%s:%d: %s\x00" as *const u8 as *const libc::c_char,
        buff.as_mut_ptr(),
        line,
        msg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaG_errormsg(mut L: *mut lua_State_0) -> ! {
    if (*L).errfunc != 0i32 as libc::c_long {
        /* is there an error handling function? */
        let mut errfunc: StkId =
            ((*L).stack as *mut libc::c_char).offset((*L).errfunc as isize) as *mut TValue;
        let mut io1: *mut TValue = (*L).top;
        *io1 = *(*L).top.offset(-1isize);
        if 0 == (*io1).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    644i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"void luaG_errormsg(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int
                && (L.is_null() || {
                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                            644i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                                b"void luaG_errormsg(lua_State *)\x00",
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
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    644i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"void luaG_errormsg(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
        };
        /* move argument */
        let mut io1_0: *mut TValue = (*L).top.offset(-1isize);
        *io1_0 = *errfunc;
        if 0 == (*io1_0).tt_ & 1i32 << 6i32 || {
            if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    645i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"void luaG_errormsg(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            (*io1_0).tt_ & 0x3fi32 == (*(*io1_0).value_.gc).tt as libc::c_int
                && (L.is_null() || {
                    if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                            645i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                                b"void luaG_errormsg(lua_State *)\x00",
                            )).as_ptr(),
                        );
                    };
                    0 != ((*(*io1_0).value_.gc).marked as libc::c_int
                        ^ (1i32 << 0i32 | 1i32 << 1i32))
                        & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                })
        } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    645i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"void luaG_errormsg(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
        };
        /* push function */
        /* assume EXTRA_STACK */
        (*L).top = (*L).top.offset(1isize);
        /* call it */
        luaD_callnoyield(L, (*L).top.offset(-2isize), 1i32);
    }
    luaD_throw(L, 2i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_traceexec(mut L: *mut lua_State_0) -> () {
    let mut ci: Option<Rc<RefCell<CallInfo>>> = (*L).ci.as_ref().cloned();
    let mut mask: lu_byte = (*L).hookmask as lu_byte;
    (*L).hookcount -= 1;
    let mut counthook: libc::c_int =
        ((*L).hookcount == 0i32 && 0 != mask as libc::c_int & 1i32 << 3i32) as libc::c_int;
    if 0 != counthook {
        (*L).hookcount = (*L).basehookcount
    } else if 0 == mask as libc::c_int & 1i32 << 2i32 {
        /* no line hook and count != 0; nothing to be done */
        return;
    }
    if 0 != (ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & 1i32 << 6i32 {
        /* called hook last time? */
        /* erase mark */
        (ci.as_ref().unwrap().borrow_mut()).callstatus = ((ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int & !(1i32 << 6i32)) as libc::c_ushort;
        /* do not call hook again (VM yielded, so it did not move) */
        return;
    } else {
        if 0 != counthook {
            /* call count hook */
            luaD_hook(L, 3i32, -1i32);
        }
        if 0 != mask as libc::c_int & 1i32 << 2i32 {
            if (*(ci.as_ref().unwrap().borrow_mut()).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    668i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                        b"void luaG_traceexec(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                        as *const libc::c_char,
                    b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                    668i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                        b"void luaG_traceexec(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            let mut p: *mut Proto_0 = (*&mut (*((*(ci.as_ref().unwrap().borrow_mut()).func).value_.gc as *mut GCUnion)).cl.l).p;
            let mut npc: libc::c_int = (ci.as_ref().unwrap().borrow_mut()).u.l.savedpc.wrapping_offset_from((*p).code)
                as libc::c_long as libc::c_int
                - 1i32;
            let mut newline: libc::c_int = if !(*p).lineinfo.is_null() {
                *(*p).lineinfo.offset(npc as isize)
            } else {
                -1i32
            };
            /* call linehook when enter a new function, */
            if npc == 0i32 || (ci.as_ref().unwrap().borrow_mut()).u.l.savedpc <= (*L).oldpc || newline != if !(*p)
                .lineinfo
                .is_null()
            {
                *(*p).lineinfo.offset(
                    ((*L).oldpc.wrapping_offset_from((*p).code) as libc::c_long as libc::c_int
                        - 1i32) as isize,
                )
            } else {
                -1i32
            } {
                /* when jump back (loop), or when */
                /* enter a new line */
                /* call line hook */
                luaD_hook(L, 2i32, newline);
            }
        }
        (*L).oldpc = (ci.as_ref().unwrap().borrow_mut()).u.l.savedpc;
        if (*L).status as libc::c_int == 1i32 {
            /* did hook yield? */
            if 0 != counthook {
                /* undo decrement to zero */
                (*L).hookcount = 1i32
            }
            /* undo increment (resume will increment it again) */
            (ci.as_ref().unwrap().borrow_mut()).u.l.savedpc = (ci.as_ref().unwrap().borrow_mut()).u.l.savedpc.offset(-1isize);
            /* mark that it yielded */
            (ci.as_ref().unwrap().borrow_mut()).callstatus = ((ci.as_ref().unwrap().borrow_mut()).callstatus as libc::c_int | 1i32 << 6i32) as libc::c_ushort;
            /* protect stack below results */
            (ci.as_ref().unwrap().borrow_mut()).func = (*L).top.offset(-1isize);
            luaD_throw(L, 1i32);
        } else {
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn svalue_wrapper(mut top: StkId) -> *const libc::c_char {
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof((((((((((((top))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((top))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 688, __extension__ __PRETTY_FUNCTION__)), (((((((((top)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((top)->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 688, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((top)->value_).gc))))->ts))))))->extra)\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      688i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"const char *svalue_wrapper(StkId)\x00")).as_ptr());
    };
    if (*top).tt_ & 0xfi32 == 4i32 {
    } else {
        __assert_fail(
            b"(((((((top))->tt_)) & 0x0F)) == (4))\x00" as *const u8 as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            688i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"const char *svalue_wrapper(StkId)\x00",
            )).as_ptr(),
        );
    };
    if (*(*top).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
    } else {
        __assert_fail(
            b"(((((top)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8 as *const libc::c_char,
            b"ldebug.c\x00" as *const u8 as *const libc::c_char,
            688i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"const char *svalue_wrapper(StkId)\x00",
            )).as_ptr(),
        );
    };
    return (&mut (*((*top).value_.gc as *mut GCUnion)).ts as *mut TString_0 as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}
