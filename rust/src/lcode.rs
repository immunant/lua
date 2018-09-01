use lstate::{CallInfo, lua_State, global_State};
use lobject::{TValue, lua_TValue, Value, GCObject};

extern crate libc;
extern "C" {
    /* control of blocks */
    /* defined in lparser.c */
    pub type BlockCnt;
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
    static mut signgam: libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn luaO_arith(
        L: *mut lua_State_0,
        op: libc::c_int,
        p1: *const TValue,
        p2: *const TValue,
        res: *mut TValue,
    ) -> ();
    #[no_mangle]
    fn luaM_growaux_(
        L: *mut lua_State_0,
        block: *mut libc::c_void,
        size: *mut libc::c_int,
        size_elem: size_t,
        limit: libc::c_int,
        what: *const libc::c_char,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaX_syntaxerror(ls: *mut LexState_0, s: *const libc::c_char) -> !;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
    #[no_mangle]
    fn luaC_barrier_(L: *mut lua_State_0, o: *mut GCObject, v: *mut GCObject) -> ();
    #[no_mangle]
    fn luaH_set(L: *mut lua_State_0, t: *mut Table_0, key: *const TValue) -> *mut TValue;
    #[no_mangle]
    fn luaV_equalobj(L: *mut lua_State_0, t1: *const TValue, t2: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type __sig_atomic_t = libc::c_int;
/* constant true */
pub const VTRUE: expkind = 2;
/* argument is not used */
pub const OpArgN: OpArgMask = 0;
/*
** Closures
*/
pub type CClosure = CClosure_0;
pub const OPR_AND: BinOpr = 19;
/* expression is a test/comparison;
            info = pc of corresponding jump instruction */
pub const VJMP: expkind = 11;
pub const OPR_BXOR: BinOpr = 9;
/*	A B	R(A) := not R(B)				*/
pub const OP_NOT: OpCode = 27;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
/* current function (parser) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto,
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
/*===========================================================================
  Notes:
  (*) In OP_CALL, if (B == 0) then B = top. If (C == 0), then 'top' is
  set to last_result+1, so next open instruction (OP_CALL, OP_RETURN,
  OP_SETLIST) may use 'top'.

  (*) In OP_VARARG, if (B == 0) then use actual number of varargs and
  set top (like in OP_CALL with C == 0).

  (*) In OP_RETURN, if (B == 0) then return up to 'top'.

  (*) In OP_SETLIST, if (B == 0) then B = 'top'; if (C == 0) then next
  'instruction' is EXTRAARG(real C).

  (*) In OP_LOADKX, the next 'instruction' is always EXTRAARG.

  (*) For comparisons, A specifies what condition the test should accept
  (true or false).

  (*) All 'skips' (pc++) assume that next instruction is a jump.

===========================================================================*/
/*
** masks for instruction properties. The format is:
** bits 0-1: op mode
** bits 2-3: C arg mode
** bits 4-5: B arg mode
** bit 6: instruction set register A
** bit 7: operator is a test (next instruction must be a jump)
*/
pub type OpArgMask = libc::c_uint;
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
pub const OPR_SUB: BinOpr = 1;
/* integer constant; nval = numerical integer value */
pub const VKINT: expkind = 6;
/*	A B	UpValue[B] := R(A)				*/
pub const OP_SETUPVAL: OpCode = 9;
pub type BinOpr = libc::c_uint;
/* constant nil */
pub const VNIL: expkind = 1;
/*	A B C	R(A) := UpValue[B][RK(C)]			*/
pub const OP_GETTABUP: OpCode = 6;
/*	A B C	R(A) := R(B)[RK(C)]				*/
pub const OP_GETTABLE: OpCode = 7;
/*	A B	R(A) := UpValue[B]				*/
pub const OP_GETUPVAL: OpCode = 5;
pub type UnOpr = UnOpr_0;
/*	Ax	extra (larger) argument for previous opcode	*/
pub const OP_EXTRAARG: OpCode = 46;
/* list of labels or gotos */
pub type Labellist = Labellist_0;
/*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/
pub const OP_SELF: OpCode = 12;
pub const OPR_OR: BinOpr = 20;
pub const OPR_BNOT: UnOpr_0 = 1;
/*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/
pub const OP_TFORLOOP: OpCode = 42;
pub type LClosure = LClosure_0;
pub const iABC: OpMode = 0;
/*----------------------------------------------------------------------
name		args	description
------------------------------------------------------------------------*/
/*	A B	R(A) := R(B)					*/
pub const OP_MOVE: OpCode = 0;
/*	A C	if not (R(A) <=> C) then pc++			*/
pub const OP_TEST: OpCode = 34;
/* basic instruction format */
pub const iAx: OpMode = 3;
/*	A B C	R(A) := RK(B) * RK(C)				*/
pub const OP_MUL: OpCode = 15;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc {
    pub k: expkind,
    pub u: unnamed_6,
    pub t: libc::c_int,
    pub f: libc::c_int,
}
/*	A B C	R(A) := RK(B) | RK(C)				*/
pub const OP_BOR: OpCode = 21;
/* dynamic structures used by the parser */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    pub actvar: unnamed,
    pub gt: Labellist,
    pub label: Labellist,
}
/* list of active local variables */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed {
    pub arr: *mut Vardesc_0,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
pub const OPR_EQ: BinOpr = 13;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist_0 {
    pub arr: *mut Labeldesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
/*	A B C	R(A) := {} (size = B,C)				*/
pub const OP_NEWTABLE: OpCode = 11;
/*	A B C	R(A) := RK(B) % RK(C)				*/
pub const OP_MOD: OpCode = 16;
pub type expkind = libc::c_uint;
/* semantics information */
pub type Token = Token_0;
pub const OPR_ADD: BinOpr = 0;
pub const OPR_MINUS: UnOpr_0 = 0;
/* description of pending goto statements and label statements */
pub type Labeldesc = Labeldesc_0;
/*
** $Id: lzio.h,v 1.30 2014/12/19 17:26:14 roberto Exp roberto $
** Buffered streams
** See Copyright Notice in lua.h
*/
/* end of stream */
pub type ZIO = Zio;
pub const OPR_IDIV: BinOpr = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut libc::c_char,
    pub n: size_t,
    pub buffsize: size_t,
}
/*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
pub const OP_TAILCALL: OpCode = 37;
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
/* argument is used */
pub const OpArgU: OpArgMask = 1;
/*	A sBx	R(A)+=R(A+2);
			if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
pub const OP_FORLOOP: OpCode = 39;
/*	A B C	R(A) := RK(B) << RK(C)				*/
pub const OP_SHL: OpCode = 23;
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
pub type BinOpr_0 = BinOpr;
/*	A sBx	R(A)-=R(A+2); pc+=sBx				*/
pub const OP_FORPREP: OpCode = 40;
/*	A B C	R(A) := RK(B) // RK(C)				*/
pub const OP_IDIV: OpCode = 19;
/*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/
pub const OP_VARARG: OpCode = 45;
/*	A B C	R(A) := RK(B) ^ RK(C)				*/
pub const OP_POW: OpCode = 17;
/*	A B C	R(A)[RK(B)] := RK(C)				*/
pub const OP_SETTABLE: OpCode = 10;
pub type expdesc_0 = expdesc;
/*	A B C	UpValue[A][RK(B)] := RK(C)			*/
pub const OP_SETTABUP: OpCode = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    r: lua_Number,
    i: lua_Integer,
    ts: *mut TString,
}
/*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
pub const OP_LOADNIL: OpCode = 4;
/*	A B	R(A) := length of R(B)				*/
pub const OP_LEN: OpCode = 28;
pub const OPR_BAND: BinOpr = 7;
/* upvalue variable; info = index of upvalue in 'upvalues' */
pub const VUPVAL: expkind = 9;
pub const OPR_LEN: UnOpr_0 = 3;
/* constant in 'k'; info = index of constant in 'k' */
pub const VK: expkind = 4;
/*	A B	R(A) := -R(B)					*/
pub const OP_UNM: OpCode = 25;
pub const OPR_NOUNOPR: UnOpr_0 = 4;
/*	A 	R(A) := Kst(extra arg)				*/
pub const OP_LOADKX: OpCode = 2;
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	R(A) := RK(B) / RK(C)				*/
pub const OP_DIV: OpCode = 18;
pub const iAsBx: OpMode = 2;
pub const OPR_MOD: BinOpr = 3;
pub const OPR_GE: BinOpr = 18;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: libc::c_int,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FuncState,
    pub L: *mut lua_State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer_0,
    pub h: *mut Table_0,
    pub dyd: *mut Dyndata,
    pub source: *mut TString,
    pub envn: *mut TString,
}
pub const OPR_NOT: UnOpr_0 = 2;
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
    p: Proto_0,
    th: lua_State,
}
pub type size_t = libc::c_ulong;
/*	A B C	R(A) := (Bool)B; if (C) pc++			*/
pub const OP_LOADBOOL: OpCode = 3;
/* indexed variable;
                ind.vt = whether 't' is register or upvalue;
                ind.t = table register or upvalue;
                ind.idx = key's R/K index */
pub const VINDEXED: expkind = 10;
/* floating constant; nval = numerical float value */
pub const VKFLT: expkind = 5;
/*	A Bx	R(A) := closure(KPROTO[Bx])			*/
pub const OP_CLOSURE: OpCode = 44;
/*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/
pub const OP_TESTSET: OpCode = 35;
/*	A B	R(A) := ~R(B)					*/
pub const OP_BNOT: OpCode = 26;
/*
** Function Prototypes
*/
pub type Proto = Proto_0;
pub const OPR_NE: BinOpr = 16;
pub type UnOpr_0 = libc::c_uint;
pub const OPR_NOBINOPR: BinOpr = 21;
/*
** $Id: lopcodes.h,v 1.148 2014/10/25 11:50:46 roberto Exp roberto $
** Opcodes for Lua virtual machine
** See Copyright Notice in lua.h
*/
/*===========================================================================
  We assume that instructions are unsigned numbers.
  All instructions have an opcode in the first 6 bits.
  Instructions can have the following fields:
	'A' : 8 bits
	'B' : 9 bits
	'C' : 9 bits
	'Ax' : 26 bits ('A', 'B', and 'C' together)
	'Bx' : 18 bits ('B' and 'C' together)
	'sBx' : signed Bx

  A signed argument is represented in excess K; that is, the number
  value is the unsigned value minus K. K is exactly the maximum value
  for that argument (so that -max is represented by 0, and +max is
  represented by 2*max), which is half the maximum for the corresponding
  unsigned argument.
===========================================================================*/
pub type OpMode = libc::c_uint;
/* state of the lexer plus state of the parser when shared by all
   functions */
pub type LexState_0 = LexState;
pub const OPR_LT: BinOpr = 14;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token_0 {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
pub type ptrdiff_t = libc::c_long;
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
    pub u: unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_3 {
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
    nk: unnamed_4,
    tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_4 {
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
/*
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
pub type lua_Reader = Option<
    unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void, _: *mut size_t)
        -> *const libc::c_char,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vardesc {
    pub idx: libc::c_short,
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
pub const OPR_MUL: BinOpr = 2;
/* state needed to generate code for a given function */
pub type FuncState_0 = FuncState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State_0,
}
pub type OpCode = libc::c_uint;
/* for indexed variables (VINDEXED) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_5 {
    pub idx: libc::c_short,
    pub t: lu_byte,
    pub vt: lu_byte,
}
/*	A B C	R(A) := RK(B) + RK(C)				*/
pub const OP_ADD: OpCode = 13;
/* argument is a register or a jump offset */
pub const OpArgR: OpArgMask = 2;
/*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/
pub const OP_SETLIST: OpCode = 43;
pub const OPR_DIV: BinOpr = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc_0 {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: lu_byte,
}
pub const OPR_CONCAT: BinOpr = 12;
/* expression is a function call; info = instruction pc */
pub const VCALL: expkind = 13;
/*	A B C	R(A) := RK(B) - RK(C)				*/
pub const OP_SUB: OpCode = 14;
/* vararg expression; info = instruction pc */
pub const VVARARG: expkind = 14;
pub const OPR_LE: BinOpr = 15;
/* argument is a constant or register/constant */
pub const OpArgK: OpArgMask = 3;
pub type Table_0 = Table;
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/* expression has its value in a fixed register;
                 info = result register */
pub const VNONRELOC: expkind = 7;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    c: CClosure,
    l: LClosure,
}
pub const iABx: OpMode = 1;
pub const OPR_SHL: BinOpr = 10;
pub const OPR_GT: BinOpr = 17;
/*	A B C	R(A) := RK(B) & RK(C)				*/
pub const OP_BAND: OpCode = 20;
/*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/
pub const OP_RETURN: OpCode = 38;
/* expression can put result in any register;
                  info = instruction pc */
pub const VRELOCABLE: expkind = 12;
/* constant false */
pub const VFALSE: expkind = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_6 {
    ival: lua_Integer,
    nval: lua_Number,
    info: libc::c_int,
    ind: unnamed_5,
}
/*	A Bx	R(A) := Kst(Bx)					*/
pub const OP_LOADK: OpCode = 1;
/*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
pub const OP_TFORCALL: OpCode = 41;
pub const OPR_SHR: BinOpr = 11;
/*	A B C	R(A) := RK(B) >> RK(C)				*/
pub const OP_SHR: OpCode = 24;
/*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_CALL: OpCode = 36;
/* local variable; info = local register */
pub const VLOCAL: expkind = 8;
pub type Mbuffer_0 = Mbuffer;
/*	A B C	R(A) := RK(B) ~ RK(C)				*/
pub const OP_BXOR: OpCode = 22;
pub const OPR_BOR: BinOpr = 8;
/* when 'expdesc' describes the last expression a list,
             this kind means an empty list (so, no expression) */
pub const VVOID: expkind = 0;
/* description of active local variable */
pub type Vardesc_0 = Vardesc;
pub const OPR_POW: BinOpr = 4;
/*	A B C	R(A) := R(B).. ... ..R(C)			*/
pub const OP_CONCAT: OpCode = 29;
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
pub struct Proto_0 {
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
    pub p: *mut *mut Proto_0,
    pub lineinfo: *mut libc::c_int,
    pub locvars: *mut LocVar_0,
    pub upvalues: *mut Upvaldesc_0,
    pub cache: *mut LClosure_0,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
/* last-created closure with this prototype */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto_0,
    pub upvals: [*mut UpVal; 0],
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codeABx(
    mut fs: *mut FuncState_0,
    mut o: OpCode,
    mut a: libc::c_int,
    mut bc: libc::c_uint,
) -> libc::c_int {
    if (luaP_opmodes[o as usize] as libc::c_int & 3i32) as OpMode as libc::c_uint
        == iABx as libc::c_int as libc::c_uint
        || (luaP_opmodes[o as usize] as libc::c_int & 3i32) as OpMode as libc::c_uint
            == iAsBx as libc::c_int as libc::c_uint
    {
    } else {
        __assert_fail(b"(((enum OpMode)(luaP_opmodes[o] & 3))) == iABx || (((enum OpMode)(luaP_opmodes[o] & 3))) == iAsBx\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      325i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"int luaK_codeABx(FuncState *, OpCode, int, unsigned int)\x00")).as_ptr());
    };
    if (luaP_opmodes[o as usize] as libc::c_int >> 2i32 & 3i32) as OpArgMask as libc::c_uint
        == OpArgN as libc::c_int as libc::c_uint
    {
    } else {
        __assert_fail(
            b"(((enum OpArgMask)((luaP_opmodes[o] >> 2) & 3))) == OpArgN\x00" as *const u8
                as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            326i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"int luaK_codeABx(FuncState *, OpCode, int, unsigned int)\x00",
            )).as_ptr(),
        );
    };
    if a <= (1i32 << 8i32) - 1i32 && bc <= ((1i32 << 9i32 + 9i32) - 1i32) as libc::c_uint {
    } else {
        __assert_fail(
            b"a <= ((1<<8)-1) && bc <= ((1<<(9 + 9))-1)\x00" as *const u8 as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            327i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"int luaK_codeABx(FuncState *, OpCode, int, unsigned int)\x00",
            )).as_ptr(),
        );
    };
    return luaK_code(
        fs,
        (o as Instruction) << 0i32 | (a as Instruction) << 0i32 + 6i32 | bc << 0i32 + 6i32 + 8i32,
    );
}
/*
** Emit instruction 'i', checking for array sizes and saving also its
** line information. Return 'i' position.
*/
unsafe extern "C" fn luaK_code(mut fs: *mut FuncState_0, mut i: Instruction) -> libc::c_int {
    let mut f: *mut Proto = (*fs).f;
    /* 'pc' will change */
    dischargejpc(fs);
    if (*fs).pc + 1i32 > (*f).sizecode {
        (*f).code = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).code as *mut libc::c_void,
            &mut (*f).sizecode,
            ::std::mem::size_of::<Instruction>() as libc::c_ulong,
            2147483647i32,
            b"opcodes\x00" as *const u8 as *const libc::c_char,
        ) as *mut Instruction
    }
    /* put new instruction in code array */
    *(*f).code.offset((*fs).pc as isize) = i;
    if (*fs).pc + 1i32 > (*f).sizelineinfo {
        (*f).lineinfo = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).lineinfo as *mut libc::c_void,
            &mut (*f).sizelineinfo,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            2147483647i32,
            b"opcodes\x00" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_int
    }
    /* save corresponding line information */
    *(*f).lineinfo.offset((*fs).pc as isize) = (*(*fs).ls).lastline;
    let fresh0 = (*fs).pc;
    (*fs).pc = (*fs).pc + 1;
    return fresh0;
}
/*
** Ensure all pending jumps to current position are fixed (jumping
** to current position with no values) and reset list of pending
** jumps
*/
unsafe extern "C" fn dischargejpc(mut fs: *mut FuncState_0) -> () {
    patchlistaux(fs, (*fs).jpc, (*fs).pc, (1i32 << 8i32) - 1i32, (*fs).pc);
    (*fs).jpc = -1i32;
}
/*
** Traverse a list of tests, patching their destination address and
** registers: tests producing values jump to 'vtarget' (and put their
** values in 'reg'), other tests jump to 'dtarget'.
*/
unsafe extern "C" fn patchlistaux(
    mut fs: *mut FuncState_0,
    mut list: libc::c_int,
    mut vtarget: libc::c_int,
    mut reg: libc::c_int,
    mut dtarget: libc::c_int,
) -> () {
    while list != -1i32 {
        let mut next: libc::c_int = getjump(fs, list);
        if 0 != patchtestreg(fs, list, reg) {
            fixjump(fs, list, vtarget);
        } else {
            /* jump to default target */
            fixjump(fs, list, dtarget);
        }
        list = next
    }
}
/*
** Gets the destination address of a jump instruction. Used to traverse
** a list of jumps.
*/
unsafe extern "C" fn getjump(mut fs: *mut FuncState_0, mut pc: libc::c_int) -> libc::c_int {
    let mut offset: libc::c_int = (*(*(*fs).f).code.offset(pc as isize) >> 0i32 + 6i32 + 8i32
        & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
        as libc::c_int - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32);
    /* point to itself represents end of list */
    if offset == -1i32 {
        /* end of list */
        return -1i32;
    } else {
        return pc + 1i32 + offset;
    };
}
/*
** Fix jump instruction at position 'pc' to jump to 'dest'.
** (Jump addresses are relative in Lua)
*/
unsafe extern "C" fn fixjump(
    mut fs: *mut FuncState_0,
    mut pc: libc::c_int,
    mut dest: libc::c_int,
) -> () {
    let mut jmp: *mut Instruction = &mut *(*(*fs).f).code.offset(pc as isize) as *mut Instruction;
    let mut offset: libc::c_int = dest - (pc + 1i32);
    if dest != -1i32 {
    } else {
        __assert_fail(
            b"dest != (-1)\x00" as *const u8 as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            106i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"void fixjump(FuncState *, int, int)\x00",
            )).as_ptr(),
        );
    };
    if abs(offset) > (1i32 << 9i32 + 9i32) - 1i32 >> 1i32 {
        luaX_syntaxerror(
            (*fs).ls,
            b"control structure too long\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        *jmp = *jmp & !(!((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32 + 6i32 + 8i32)
            | ((offset + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as libc::c_uint)
                << 0i32 + 6i32 + 8i32
                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32 + 6i32 + 8i32;
        return;
    };
}
/*
** Patch destination register for a TESTSET instruction.
** If instruction in position 'node' is not a TESTSET, return 0 ("fails").
** Otherwise, if 'reg' is not 'NO_REG', set it as the destination
** register. Otherwise, change instruction to a simple 'TEST' (produces
** no register value)
*/
unsafe extern "C" fn patchtestreg(
    mut fs: *mut FuncState_0,
    mut node: libc::c_int,
    mut reg: libc::c_int,
) -> libc::c_int {
    let mut i: *mut Instruction = getjumpcontrol(fs, node);
    if (*i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint
        != OP_TESTSET as libc::c_int as libc::c_uint
    {
        /* cannot patch other instructions */
        return 0i32;
    } else {
        if reg != (1i32 << 8i32) - 1i32
            && reg != (*i >> 0i32 + 6i32 + 8i32 + 9i32
                & !((!(0i32 as Instruction)) << 9i32) << 0i32) as libc::c_int
        {
            *i = *i & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32)
                | (reg as Instruction) << 0i32 + 6i32
                    & !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32
        } else {
            *i = (OP_TEST as libc::c_int as Instruction) << 0i32
                | ((*i >> 0i32 + 6i32 + 8i32 + 9i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                    as libc::c_int as Instruction) << 0i32 + 6i32
                | (0i32 as Instruction) << 0i32 + 6i32 + 8i32 + 9i32
                | ((*i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                    as libc::c_int as Instruction) << 0i32 + 6i32 + 8i32
        }
        return 1i32;
    };
}
/*
** Returns the position of the instruction "controlling" a given
** jump (that is, its condition), or the jump itself if it is
** unconditional.
*/
unsafe extern "C" fn getjumpcontrol(
    mut fs: *mut FuncState_0,
    mut pc: libc::c_int,
) -> *mut Instruction {
    let mut pi: *mut Instruction = &mut *(*(*fs).f).code.offset(pc as isize) as *mut Instruction;
    if pc >= 1i32
        && 0 != luaP_opmodes[(*pi.offset(-1isize) >> 0i32
                                 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                 as OpCode as usize] as libc::c_int & 1i32 << 7i32
    {
        return pi.offset(-1isize);
    } else {
        return pi;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codeABC(
    mut fs: *mut FuncState_0,
    mut o: OpCode,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    if (luaP_opmodes[o as usize] as libc::c_int & 3i32) as OpMode as libc::c_uint
        == iABC as libc::c_int as libc::c_uint
    {
    } else {
        __assert_fail(
            b"(((enum OpMode)(luaP_opmodes[o] & 3))) == iABC\x00" as *const u8
                as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            313i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"int luaK_codeABC(FuncState *, OpCode, int, int, int)\x00",
            )).as_ptr(),
        );
    };
    if (luaP_opmodes[o as usize] as libc::c_int >> 4i32 & 3i32) as OpArgMask as libc::c_uint
        != OpArgN as libc::c_int as libc::c_uint || b == 0i32
    {
    } else {
        __assert_fail(
            b"(((enum OpArgMask)((luaP_opmodes[o] >> 4) & 3))) != OpArgN || b == 0\x00" as *const u8
                as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            314i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"int luaK_codeABC(FuncState *, OpCode, int, int, int)\x00",
            )).as_ptr(),
        );
    };
    if (luaP_opmodes[o as usize] as libc::c_int >> 2i32 & 3i32) as OpArgMask as libc::c_uint
        != OpArgN as libc::c_int as libc::c_uint || c == 0i32
    {
    } else {
        __assert_fail(
            b"(((enum OpArgMask)((luaP_opmodes[o] >> 2) & 3))) != OpArgN || c == 0\x00" as *const u8
                as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            315i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"int luaK_codeABC(FuncState *, OpCode, int, int, int)\x00",
            )).as_ptr(),
        );
    };
    if a <= (1i32 << 8i32) - 1i32 && b <= (1i32 << 9i32) - 1i32 && c <= (1i32 << 9i32) - 1i32 {
    } else {
        __assert_fail(
            b"a <= ((1<<8)-1) && b <= ((1<<9)-1) && c <= ((1<<9)-1)\x00" as *const u8
                as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            316i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"int luaK_codeABC(FuncState *, OpCode, int, int, int)\x00",
            )).as_ptr(),
        );
    };
    return luaK_code(
        fs,
        (o as Instruction) << 0i32
            | (a as Instruction) << 0i32 + 6i32
            | (b as Instruction) << 0i32 + 6i32 + 8i32 + 9i32
            | (c as Instruction) << 0i32 + 6i32 + 8i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codek(
    mut fs: *mut FuncState_0,
    mut reg: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    if k <= (1i32 << 9i32 + 9i32) - 1i32 {
        return luaK_codeABx(fs, OP_LOADK, reg, k as libc::c_uint);
    } else {
        let mut p: libc::c_int = luaK_codeABx(fs, OP_LOADKX, reg, 0i32 as libc::c_uint);
        codeextraarg(fs, k);
        return p;
    };
}
/*
** Emit an "extra argument" instruction (format 'iAx')
*/
unsafe extern "C" fn codeextraarg(mut fs: *mut FuncState_0, mut a: libc::c_int) -> libc::c_int {
    if a <= (1i32 << 9i32 + 9i32 + 8i32) - 1i32 {
    } else {
        __assert_fail(
            b"a <= ((1<<(9 + 9 + 8))-1)\x00" as *const u8 as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            336i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"int codeextraarg(FuncState *, int)\x00",
            )).as_ptr(),
        );
    };
    return luaK_code(
        fs,
        (OP_EXTRAARG as libc::c_int as Instruction) << 0i32 | (a as Instruction) << 0i32 + 6i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaK_fixline(mut fs: *mut FuncState_0, mut line: libc::c_int) -> () {
    *(*(*fs).f).lineinfo.offset(((*fs).pc - 1i32) as isize) = line;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_nil(
    mut fs: *mut FuncState_0,
    mut from: libc::c_int,
    mut n: libc::c_int,
) -> () {
    let mut previous: *mut Instruction = 0 as *mut Instruction;
    /* last register to set nil */
    let mut l: libc::c_int = from + n - 1i32;
    if (*fs).pc > (*fs).lasttarget {
        /* no jumps to current position? */
        previous = &mut *(*(*fs).f).code.offset(((*fs).pc - 1i32) as isize) as *mut Instruction;
        if (*previous >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
            as libc::c_uint == OP_LOADNIL as libc::c_int as libc::c_uint
        {
            /* previous is LOADNIL? */
            /* get previous range */
            let mut pfrom: libc::c_int = (*previous >> 0i32 + 6i32
                & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                as libc::c_int;
            let mut pl: libc::c_int = pfrom
                + (*previous >> 0i32 + 6i32 + 8i32 + 9i32
                    & !((!(0i32 as Instruction)) << 9i32) << 0i32) as libc::c_int;
            if pfrom <= from && from <= pl + 1i32 || from <= pfrom && pfrom <= l + 1i32 {
                /* can connect both? */
                if pfrom < from {
                    /* from = min(from, pfrom) */
                    from = pfrom
                }
                if pl > l {
                    /* l = max(l, pl) */
                    l = pl
                }
                *previous = *previous & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32)
                    | (from as Instruction) << 0i32 + 6i32
                        & !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32;
                *previous = *previous
                    & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32)
                    | ((l - from) as Instruction) << 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32;
                return;
            }
        }
    }
    /* else go through */
    /* else no optimization */
    luaK_codeABC(fs, OP_LOADNIL, from, n - 1i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_reserveregs(mut fs: *mut FuncState_0, mut n: libc::c_int) -> () {
    luaK_checkstack(fs, n);
    (*fs).freereg = ((*fs).freereg as libc::c_int + n) as lu_byte;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_checkstack(mut fs: *mut FuncState_0, mut n: libc::c_int) -> () {
    let mut newstack: libc::c_int = (*fs).freereg as libc::c_int + n;
    if newstack > (*(*fs).f).maxstacksize as libc::c_int {
        if newstack >= 255i32 {
            luaX_syntaxerror(
                (*fs).ls,
                b"function or expression needs too many registers\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
            (*(*fs).f).maxstacksize = newstack as lu_byte
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_stringK(
    mut fs: *mut FuncState_0,
    mut s: *mut TString,
) -> libc::c_int {
    let mut o: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut o;
    let mut x_: *mut TString = s;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            460i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"int luaK_stringK(FuncState *, TString *)\x00",
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
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                460i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"int luaK_stringK(FuncState *, TString *)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
            && ((*(*fs).ls).L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lcode.c\x00" as *const u8 as *const libc::c_char,
                        460i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                            b"int luaK_stringK(FuncState *, TString *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != ((*(*io).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*(*(*fs).ls).L).l_G).currentwhite as libc::c_int
                        ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                460i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"int luaK_stringK(FuncState *, TString *)\x00",
                )).as_ptr(),
            );
        };
    };
    /* use string itself as key */
    return addk(fs, &mut o, &mut o);
}
/*
** Add constant 'v' to prototype's list of constants (field 'k').
** Use scanner's table to cache position of constants in constant list
** and try to reuse constants. Because some values should not be used
** as keys (nil cannot be a key, integer keys can collapse with float
** keys), the caller must provide a useful 'key' for indexing the cache.
*/
unsafe extern "C" fn addk(
    mut fs: *mut FuncState_0,
    mut key: *mut TValue,
    mut v: *mut TValue,
) -> libc::c_int {
    let mut L: *mut lua_State_0 = (*(*fs).ls).L;
    let mut f: *mut Proto = (*fs).f;
    /* index scanner table */
    let mut idx: *mut TValue = luaH_set(L, (*(*fs).ls).h, key);
    let mut k: libc::c_int = 0;
    let mut oldsize: libc::c_int = 0;
    if (*idx).tt_ == 3i32 | 1i32 << 4i32 {
        /* is there an index there? */
        if (*idx).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((idx))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                434i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int addk(FuncState *, TValue *, TValue *)\x00",
                )).as_ptr(),
            );
        };
        k = (*idx).value_.i as libc::c_int;
        /* correct value? (warning: must distinguish floats from integers!) */
        if k < (*fs).nk
            && (*(*f).k.offset(k as isize)).tt_ & 0x3fi32 == (*v).tt_ & 0x3fi32
            && 0 != luaV_equalobj(0 as *mut lua_State_0, &mut *(*f).k.offset(k as isize), v)
        {
            /* reuse index */
            return k;
        }
    }
    /* constant not found; create a new entry */
    oldsize = (*f).sizek;
    k = (*fs).nk;
    let mut io: *mut TValue = idx;
    (*io).value_.i = k as lua_Integer;
    (*io).tt_ = 3i32 | 1i32 << 4i32;
    /* numerical value does not need GC barrier;
     table has no metatable, so it does not need to invalidate cache */
    if k + 1i32 > (*f).sizek {
        (*f).k = luaM_growaux_(
            L,
            (*f).k as *mut libc::c_void,
            &mut (*f).sizek,
            ::std::mem::size_of::<TValue>() as libc::c_ulong,
            (1i32 << 9i32 + 9i32 + 8i32) - 1i32,
            b"constants\x00" as *const u8 as *const libc::c_char,
        ) as *mut TValue
    }
    while oldsize < (*f).sizek {
        let fresh1 = oldsize;
        oldsize = oldsize + 1;
        (*(*f).k.offset(fresh1 as isize)).tt_ = 0i32
    }
    let mut io1: *mut TValue = &mut *(*f).k.offset(k as isize) as *mut TValue;
    *io1 = *v;
    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                448i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int addk(FuncState *, TValue *, TValue *)\x00",
                )).as_ptr(),
            );
        };
        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int && (L.is_null() || {
            if 0 != (*io1).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    448i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                        b"int addk(FuncState *, TValue *, TValue *)\x00",
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
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                448i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int addk(FuncState *, TValue *, TValue *)\x00",
                )).as_ptr(),
            );
        };
    };
    (*fs).nk += 1;
    if 0 != (*v).tt_ & 1i32 << 6i32 && 0 != (*f).marked as libc::c_int & 1i32 << 2i32 && {
        if 0 != (*v).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((v)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                450i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int addk(FuncState *, TValue *, TValue *)\x00",
                )).as_ptr(),
            );
        };
        0 != (*(*v).value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
    } {
        if (*f).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((f)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                450i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int addk(FuncState *, TValue *, TValue *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != (*v).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((v)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                450i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                    b"int addk(FuncState *, TValue *, TValue *)\x00",
                )).as_ptr(),
            );
        };
        luaC_barrier_(L, &mut (*(f as *mut GCUnion)).gc, (*v).value_.gc);
    } else {
    };
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_intK(mut fs: *mut FuncState_0, mut n: lua_Integer) -> libc::c_int {
    let mut k: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut o: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = n as size_t as *mut libc::c_void;
    (*io).tt_ = 2i32;
    let mut io_0: *mut TValue = &mut o;
    (*io_0).value_.i = n;
    (*io_0).tt_ = 3i32 | 1i32 << 4i32;
    return addk(fs, &mut k, &mut o);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_dischargevars(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    match (*e).k as libc::c_uint {
        8 => {
            /* already in a register */
            /* becomes a non-relocatable value */
            (*e).k = VNONRELOC
        }
        9 => {
            /* move value to some (pending) register */
            (*e).u.info = luaK_codeABC(fs, OP_GETUPVAL, 0i32, (*e).u.info, 0i32);
            (*e).k = VRELOCABLE
        }
        10 => {
            let mut op: OpCode = OP_MOVE;
            freereg(fs, (*e).u.ind.idx as libc::c_int);
            if (*e).u.ind.vt as libc::c_int == VLOCAL as libc::c_int {
                /* is 't' in a register? */
                freereg(fs, (*e).u.ind.t as libc::c_int);
                op = OP_GETTABLE
            } else {
                if (*e).u.ind.vt as libc::c_int == VUPVAL as libc::c_int {
                } else {
                    __assert_fail(
                        b"e->u.ind.vt == VUPVAL\x00" as *const u8 as *const libc::c_char,
                        b"lcode.c\x00" as *const u8 as *const libc::c_char,
                        575i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                            b"void luaK_dischargevars(FuncState *, expdesc *)\x00",
                        )).as_ptr(),
                    );
                };
                /* 't' is in an upvalue */
                op = OP_GETTABUP
            }
            (*e).u.info = luaK_codeABC(
                fs,
                op,
                0i32,
                (*e).u.ind.t as libc::c_int,
                (*e).u.ind.idx as libc::c_int,
            );
            (*e).k = VRELOCABLE
        }
        14 | 13 => {
            luaK_setoneret(fs, e);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setoneret(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint {
        /* expression is an open function call? */
        if (*(*(*fs).f).code.offset((*e).u.info as isize) >> 0i32 + 6i32 + 8i32
            & !((!(0i32 as Instruction)) << 9i32) << 0i32) as libc::c_int == 2i32
        {
        } else {
            __assert_fail(b"(((int)(((((fs)->f->code[(e)->u.info]))>>((0 + 6) + 8)) & ((~((~(Instruction)0)<<(9)))<<(0))))) == 2\x00"
                              as *const u8 as *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          542i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 44],
                                                    &[libc::c_char; 44]>(b"void luaK_setoneret(FuncState *, expdesc *)\x00")).as_ptr());
        };
        /* already returns 1 value */
        /* result has fixed position */
        (*e).k = VNONRELOC;
        (*e).u.info = (*(*(*fs).f).code.offset((*e).u.info as isize) >> 0i32 + 6i32
            & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int
    } else if (*e).k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint {
        *(*(*fs).f).code.offset((*e).u.info as isize) =
            *(*(*fs).f).code.offset((*e).u.info as isize)
                & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32)
                | (2i32 as Instruction) << 0i32 + 6i32 + 8i32 + 9i32
                    & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32;
        /* can relocate its simple result */
        (*e).k = VRELOCABLE
    };
}
/*
** Free register 'reg', if it is neither a constant index nor
** a local variable.
)
*/
unsafe extern "C" fn freereg(mut fs: *mut FuncState_0, mut reg: libc::c_int) -> () {
    if 0 == reg & 1i32 << 9i32 - 1i32 && reg >= (*fs).nactvar as libc::c_int {
        (*fs).freereg = (*fs).freereg.wrapping_sub(1);
        if reg == (*fs).freereg as libc::c_int {
        } else {
            __assert_fail(
                b"reg == fs->freereg\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                389i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"void freereg(FuncState *, int)\x00",
                )).as_ptr(),
            );
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2anyreg(
    mut fs: *mut FuncState_0,
    mut e: *mut expdesc_0,
) -> libc::c_int {
    luaK_dischargevars(fs, e);
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        /* expression already has a register? */
        /* no jumps? */
        if !((*e).t != (*e).f) {
            /* result is already in a register */
            return (*e).u.info;
        } else if (*e).u.info >= (*fs).nactvar as libc::c_int {
            /* reg. is not a local? */
            /* put final result in it */
            exp2reg(fs, e, (*e).u.info);
            return (*e).u.info;
        }
    }
    /* otherwise, use next available register */
    luaK_exp2nextreg(fs, e);
    return (*e).u.info;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2nextreg(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    luaK_dischargevars(fs, e);
    freeexp(fs, e);
    luaK_reserveregs(fs, 1i32);
    exp2reg(fs, e, (*fs).freereg as libc::c_int - 1i32);
}
/*
** Ensures final expression result (including results from its jump
** lists) is in register 'reg'.
** If expression has jumps, need to patch these jumps either to
** its final position or to "load" instructions (for those tests
** that do not produce values).
*/
unsafe extern "C" fn exp2reg(
    mut fs: *mut FuncState_0,
    mut e: *mut expdesc_0,
    mut reg: libc::c_int,
) -> () {
    let mut fj: libc::c_int = 0;
    discharge2reg(fs, e, reg);
    /* expression itself is a test? */
    if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint {
        /* put this jump in 't' list */
        luaK_concat(fs, &mut (*e).t, (*e).u.info);
    }
    if (*e).t != (*e).f {
        /* position after whole expression */
        let mut final_0: libc::c_int = 0;
        /* position of an eventual LOAD false */
        let mut p_f: libc::c_int = -1i32;
        /* position of an eventual LOAD true */
        let mut p_t: libc::c_int = -1i32;
        if 0 != need_value(fs, (*e).t) || 0 != need_value(fs, (*e).f) {
            fj = if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint {
                -1i32
            } else {
                luaK_jump(fs)
            };
            p_f = code_loadbool(fs, reg, 0i32, 1i32);
            p_t = code_loadbool(fs, reg, 1i32, 0i32);
            luaK_patchtohere(fs, fj);
        }
        final_0 = luaK_getlabel(fs);
        patchlistaux(fs, (*e).f, final_0, reg, p_f);
        patchlistaux(fs, (*e).t, final_0, reg, p_t);
    }
    (*e).t = -1i32;
    (*e).f = (*e).t;
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_getlabel(mut fs: *mut FuncState_0) -> libc::c_int {
    (*fs).lasttarget = (*fs).pc;
    return (*fs).pc;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_jump(mut fs: *mut FuncState_0) -> libc::c_int {
    /* save list of jumps to here */
    let mut jpc: libc::c_int = (*fs).jpc;
    let mut j: libc::c_int = 0;
    /* no more jumps to here */
    (*fs).jpc = -1i32;
    j = luaK_codeABx(
        fs,
        OP_JMP,
        0i32,
        (-1i32 + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as libc::c_uint,
    );
    /* keep them on hold */
    luaK_concat(fs, &mut j, jpc);
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_concat(
    mut fs: *mut FuncState_0,
    mut l1: *mut libc::c_int,
    mut l2: libc::c_int,
) -> () {
    if l2 == -1i32 {
        /* nothing to concatenate? */
        return;
    } else {
        /* no original list? */
        if *l1 == -1i32 {
            /* 'l1' points to 'l2' */
            *l1 = l2
        } else {
            let mut list: libc::c_int = *l1;
            let mut next: libc::c_int = 0;
            /* find last element */
            loop {
                next = getjump(fs, list);
                if !(next != -1i32) {
                    break;
                }
                list = next
            }
            /* last element links to 'l2' */
            fixjump(fs, list, l2);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchtohere(mut fs: *mut FuncState_0, mut list: libc::c_int) -> () {
    /* mark "here" as a jump target */
    luaK_getlabel(fs);
    luaK_concat(fs, &mut (*fs).jpc, list);
}
unsafe extern "C" fn code_loadbool(
    mut fs: *mut FuncState_0,
    mut A: libc::c_int,
    mut b: libc::c_int,
    mut jump: libc::c_int,
) -> libc::c_int {
    /* those instructions may be jump targets */
    luaK_getlabel(fs);
    return luaK_codeABC(fs, OP_LOADBOOL, A, b, jump);
}
/*
** check whether list has any jump that do not produce a value
** or produce an inverted value
*/
unsafe extern "C" fn need_value(mut fs: *mut FuncState_0, mut list: libc::c_int) -> libc::c_int {
    while list != -1i32 {
        let mut i: Instruction = *getjumpcontrol(fs, list);
        if (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint
            != OP_TESTSET as libc::c_int as libc::c_uint
        {
            return 1i32;
        } else {
            list = getjump(fs, list)
        }
    }
    /* not found */
    return 0i32;
}
/*
** Ensures expression value is in register 'reg' (and therefore
** 'e' will become a non-relocatable expression).
*/
unsafe extern "C" fn discharge2reg(
    mut fs: *mut FuncState_0,
    mut e: *mut expdesc_0,
    mut reg: libc::c_int,
) -> () {
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        1 => {
            luaK_nil(fs, reg, 1i32);
        }
        3 | 2 => {
            luaK_codeABC(
                fs,
                OP_LOADBOOL,
                reg,
                ((*e).k as libc::c_uint == VTRUE as libc::c_int as libc::c_uint) as libc::c_int,
                0i32,
            );
        }
        4 => {
            luaK_codek(fs, reg, (*e).u.info);
        }
        5 => {
            luaK_codek(fs, reg, luaK_numberK(fs, (*e).u.nval));
        }
        6 => {
            luaK_codek(fs, reg, luaK_intK(fs, (*e).u.ival));
        }
        12 => {
            let mut pc: *mut Instruction =
                &mut *(*(*fs).f).code.offset((*e).u.info as isize) as *mut Instruction;
            *pc = *pc & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32)
                | (reg as Instruction) << 0i32 + 6i32
                    & !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32
        }
        7 => {
            /* instruction will put result in 'reg' */
            if reg != (*e).u.info {
                luaK_codeABC(fs, OP_MOVE, reg, (*e).u.info, 0i32);
            }
        }
        _ => {
            if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(
                    b"e->k == VJMP\x00" as *const u8 as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    629i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                        b"void discharge2reg(FuncState *, expdesc *, int)\x00",
                    )).as_ptr(),
                );
            };
            /* nothing to do... */
            return;
        }
    }
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}
/*
** Add a float to list of constants and return its index.
*/
unsafe extern "C" fn luaK_numberK(mut fs: *mut FuncState_0, mut r: lua_Number) -> libc::c_int {
    let mut o: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut o;
    (*io).value_.n = r;
    (*io).tt_ = 3i32 | 0i32 << 4i32;
    /* use number itself as key */
    return addk(fs, &mut o, &mut o);
}
/*
** Free register used by expression 'e' (if any)
*/
unsafe extern "C" fn freeexp(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        freereg(fs, (*e).u.info);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2anyregup(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    if (*e).k as libc::c_uint != VUPVAL as libc::c_int as libc::c_uint || (*e).t != (*e).f {
        luaK_exp2anyreg(fs, e);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2val(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    if (*e).t != (*e).f {
        luaK_exp2anyreg(fs, e);
    } else {
        luaK_dischargevars(fs, e);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2RK(
    mut fs: *mut FuncState_0,
    mut e: *mut expdesc_0,
) -> libc::c_int {
    let mut current_block: u64;
    luaK_exp2val(fs, e);
    match (*e).k as libc::c_uint {
        2 => {
            (*e).u.info = boolK(fs, 1i32);
            current_block = 12405266282655138779;
        }
        3 => {
            (*e).u.info = boolK(fs, 0i32);
            current_block = 12405266282655138779;
        }
        1 => {
            (*e).u.info = nilK(fs);
            current_block = 12405266282655138779;
        }
        6 => {
            (*e).u.info = luaK_intK(fs, (*e).u.ival);
            current_block = 12405266282655138779;
        }
        5 => {
            (*e).u.info = luaK_numberK(fs, (*e).u.nval);
            current_block = 12405266282655138779;
        }
        4 => {
            current_block = 12405266282655138779;
        }
        _ => {
            current_block = 792017965103506125;
        }
    }
    match current_block {
        12405266282655138779 => {
            (*e).k = VK;
            /* constant fits in 'argC'? */
            if (*e).u.info <= 1i32 {
                return (*e).u.info | 1i32 << 9i32 - 1i32;
            }
        }
        _ => {}
    }
    /* not a constant in the right range: put it in a register */
    return luaK_exp2anyreg(fs, e);
}
/*
** Add nil to list of constants and return its index.
*/
unsafe extern "C" fn nilK(mut fs: *mut FuncState_0) -> libc::c_int {
    let mut k: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut v: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    v.tt_ = 0i32;
    let mut io: *mut TValue = &mut k;
    let mut x_: *mut Table_0 = (*(*fs).ls).h;
    if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            505i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"int nilK(FuncState *)\x00",
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
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                505i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"int nilK(FuncState *)\x00",
                )).as_ptr(),
            );
        };
        (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
            && ((*(*fs).ls).L.is_null() || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"lcode.c\x00" as *const u8 as *const libc::c_char,
                        505i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                            b"int nilK(FuncState *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != ((*(*io).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*(*(*fs).ls).L).l_G).currentwhite as libc::c_int
                        ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                505i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"int nilK(FuncState *)\x00",
                )).as_ptr(),
            );
        };
    };
    /* cannot use nil as key; instead use table itself to represent nil */
    return addk(fs, &mut k, &mut v);
}
/*
** Add a boolean to list of constants and return its index.
*/
unsafe extern "C" fn boolK(mut fs: *mut FuncState_0, mut b: libc::c_int) -> libc::c_int {
    let mut o: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut o;
    (*io).value_.b = b;
    (*io).tt_ = 1i32;
    /* use boolean itself as key */
    return addk(fs, &mut o, &mut o);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_self(
    mut fs: *mut FuncState_0,
    mut e: *mut expdesc_0,
    mut key: *mut expdesc_0,
) -> () {
    let mut ereg: libc::c_int = 0;
    luaK_exp2anyreg(fs, e);
    /* register where 'e' was placed */
    ereg = (*e).u.info;
    freeexp(fs, e);
    /* base register for op_self */
    (*e).u.info = (*fs).freereg as libc::c_int;
    /* self expression has a fixed register */
    (*e).k = VNONRELOC;
    /* function and 'self' produced by op_self */
    luaK_reserveregs(fs, 2i32);
    luaK_codeABC(fs, OP_SELF, (*e).u.info, ereg, luaK_exp2RK(fs, key));
    freeexp(fs, key);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_indexed(
    mut fs: *mut FuncState_0,
    mut t: *mut expdesc_0,
    mut k: *mut expdesc_0,
) -> () {
    if !((*t).t != (*t).f)
        && ((*t).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint
            || (*t).k as libc::c_uint == VLOCAL as libc::c_int as libc::c_uint
            || (*t).k as libc::c_uint == VUPVAL as libc::c_int as libc::c_uint)
    {
    } else {
        __assert_fail(b"!((t)->t != (t)->f) && (((t->k) == VNONRELOC || (t->k) == VLOCAL) || t->k == VUPVAL)\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      947i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"void luaK_indexed(FuncState *, expdesc *, expdesc *)\x00")).as_ptr());
    };
    /* register or upvalue index */
    (*t).u.ind.t = (*t).u.info as lu_byte;
    /* R/K index for key */
    (*t).u.ind.idx = luaK_exp2RK(fs, k) as libc::c_short;
    (*t).u.ind.vt = (if (*t).k as libc::c_uint == VUPVAL as libc::c_int as libc::c_uint {
        VUPVAL as libc::c_int
    } else {
        VLOCAL as libc::c_int
    }) as lu_byte;
    (*t).k = VINDEXED;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_goiftrue(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    /* pc of new jump */
    let mut pc: libc::c_int = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        11 => {
            /* condition? */
            /* jump when it is false */
            negatecondition(fs, e);
            /* save jump position */
            pc = (*e).u.info
        }
        4 | 5 | 6 | 2 => {
            /* always true; do nothing */
            pc = -1i32
        }
        _ => {
            /* jump when false */
            pc = jumponcond(fs, e, 0i32)
        }
    }
    /* insert new jump in false list */
    luaK_concat(fs, &mut (*e).f, pc);
    /* true list jumps to here (to go through) */
    luaK_patchtohere(fs, (*e).t);
    (*e).t = -1i32;
}
/*
** Emit instruction to jump if 'e' is 'cond' (that is, if 'cond'
** is true, code will jump if 'e' is true.) Return jump position.
** Optimize when 'e' is 'not' something, inverting the condition
** and removing the 'not'.
*/
unsafe extern "C" fn jumponcond(
    mut fs: *mut FuncState_0,
    mut e: *mut expdesc_0,
    mut cond: libc::c_int,
) -> libc::c_int {
    if (*e).k as libc::c_uint == VRELOCABLE as libc::c_int as libc::c_uint {
        let mut ie: Instruction = *(*(*fs).f).code.offset((*e).u.info as isize);
        if (ie >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint
            == OP_NOT as libc::c_int as libc::c_uint
        {
            /* remove previous OP_NOT */
            (*fs).pc -= 1;
            return condjump(
                fs,
                OP_TEST,
                (ie >> 0i32 + 6i32 + 8i32 + 9i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                    as libc::c_int,
                0i32,
                (0 == cond) as libc::c_int,
            );
        }
    }
    /* else go through */
    discharge2anyreg(fs, e);
    freeexp(fs, e);
    return condjump(fs, OP_TESTSET, (1i32 << 8i32) - 1i32, (*e).u.info, cond);
}
/*
** Code a "conditional jump", that is, a test or comparison opcode
** followed by a jump. Return jump position.
*/
unsafe extern "C" fn condjump(
    mut fs: *mut FuncState_0,
    mut op: OpCode,
    mut A: libc::c_int,
    mut B: libc::c_int,
    mut C: libc::c_int,
) -> libc::c_int {
    luaK_codeABC(fs, op, A, B, C);
    return luaK_jump(fs);
}
/*
** Ensures expression value is in any register.
*/
unsafe extern "C" fn discharge2anyreg(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    if (*e).k as libc::c_uint != VNONRELOC as libc::c_int as libc::c_uint {
        /* no fixed register yet? */
        /* get a register */
        luaK_reserveregs(fs, 1i32);
        /* put value there */
        discharge2reg(fs, e, (*fs).freereg as libc::c_int - 1i32);
    };
}
/*
** Negate condition 'e' (where 'e' is a comparison).
*/
unsafe extern "C" fn negatecondition(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    let mut pc: *mut Instruction = getjumpcontrol(fs, (*e).u.info);
    if 0 != luaP_opmodes
        [(*pc >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as usize]
        as libc::c_int & 1i32 << 7i32
        && (*pc >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint
            != OP_TESTSET as libc::c_int as libc::c_uint
        && (*pc >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint
            != OP_TEST as libc::c_int as libc::c_uint
    {
    } else {
        __assert_fail(b"(luaP_opmodes[(((OpCode)(((*pc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] & (1 << 7)) && (((OpCode)(((*pc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) != OP_TESTSET && (((OpCode)(((*pc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) != OP_TEST\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      828i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void negatecondition(FuncState *, expdesc *)\x00")).as_ptr());
    };
    *pc = *pc & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32)
        | ((0 == (*pc >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int)
            as libc::c_int as Instruction) << 0i32 + 6i32
            & !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_goiffalse(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    /* pc of new jump */
    let mut pc: libc::c_int = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        11 => {
            /* already jump if true */
            pc = (*e).u.info
        }
        1 | 3 => {
            /* always false; do nothing */
            pc = -1i32
        }
        _ => {
            /* jump if true */
            pc = jumponcond(fs, e, 1i32)
        }
    }
    /* insert new jump in 't' list */
    luaK_concat(fs, &mut (*e).t, pc);
    /* false list jumps to here (to go through) */
    luaK_patchtohere(fs, (*e).f);
    (*e).f = -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_storevar(
    mut fs: *mut FuncState_0,
    mut var: *mut expdesc_0,
    mut ex: *mut expdesc_0,
) -> () {
    match (*var).k as libc::c_uint {
        8 => {
            freeexp(fs, ex);
            /* compute 'ex' into proper place */
            exp2reg(fs, ex, (*var).u.info);
            return;
        }
        9 => {
            let mut e: libc::c_int = luaK_exp2anyreg(fs, ex);
            luaK_codeABC(fs, OP_SETUPVAL, e, (*var).u.info, 0i32);
        }
        10 => {
            let mut op: OpCode = (if (*var).u.ind.vt as libc::c_int == VLOCAL as libc::c_int {
                OP_SETTABLE as libc::c_int
            } else {
                OP_SETTABUP as libc::c_int
            }) as OpCode;
            let mut e_0: libc::c_int = luaK_exp2RK(fs, ex);
            luaK_codeABC(
                fs,
                op,
                (*var).u.ind.t as libc::c_int,
                (*var).u.ind.idx as libc::c_int,
                e_0,
            );
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    800i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                        b"void luaK_storevar(FuncState *, expdesc *, expdesc *)\x00",
                    )).as_ptr(),
                );
            };
        }
    }
    freeexp(fs, ex);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setreturns(
    mut fs: *mut FuncState_0,
    mut e: *mut expdesc_0,
    mut nresults: libc::c_int,
) -> () {
    let mut pc: *mut Instruction = 0 as *mut Instruction;
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint {
        /* expression is an open function call? */
        *(*(*fs).f).code.offset((*e).u.info as isize) =
            *(*(*fs).f).code.offset((*e).u.info as isize)
                & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32)
                | ((nresults + 1i32) as Instruction) << 0i32 + 6i32 + 8i32
                    & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32
    } else if (*e).k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint {
        pc = &mut *(*(*fs).f).code.offset((*e).u.info as isize) as *mut Instruction;
        *pc = *pc & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32)
            | ((nresults + 1i32) as Instruction) << 0i32 + 6i32 + 8i32 + 9i32
                & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32;
        *pc = *pc & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32)
            | ((*fs).freereg as Instruction) << 0i32 + 6i32
                & !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32;
        luaK_reserveregs(fs, 1i32);
    } else {
        if nresults == -1i32 {
        } else {
            __assert_fail(
                b"nresults == (-1)\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                525i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"void luaK_setreturns(FuncState *, expdesc *, int)\x00",
                )).as_ptr(),
            );
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_ret(
    mut fs: *mut FuncState_0,
    mut first: libc::c_int,
    mut nret: libc::c_int,
) -> () {
    luaK_codeABC(fs, OP_RETURN, first, nret + 1i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchlist(
    mut fs: *mut FuncState_0,
    mut list: libc::c_int,
    mut target: libc::c_int,
) -> () {
    /* 'target' is current position? */
    if target == (*fs).pc {
        /* add list to pending jumps */
        luaK_patchtohere(fs, list);
    } else {
        if target < (*fs).pc {
        } else {
            __assert_fail(
                b"target < fs->pc\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                267i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                    b"void luaK_patchlist(FuncState *, int, int)\x00",
                )).as_ptr(),
            );
        };
        patchlistaux(fs, list, target, (1i32 << 8i32) - 1i32, target);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchclose(
    mut fs: *mut FuncState_0,
    mut list: libc::c_int,
    mut level: libc::c_int,
) -> () {
    /* argument is +1 to reserve 0 as non-op */
    level += 1;
    while list != -1i32 {
        if (*(*(*fs).f).code.offset(list as isize) >> 0i32
            & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint
            == OP_JMP as libc::c_int as libc::c_uint
            && ((*(*(*fs).f).code.offset(list as isize) >> 0i32 + 6i32
                & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int
                == 0i32
                || (*(*(*fs).f).code.offset(list as isize) >> 0i32 + 6i32
                    & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int
                    >= level)
        {
        } else {
            __assert_fail(b"(((OpCode)(((fs->f->code[list])>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) == OP_JMP && ((((int)(((fs->f->code[list])>>(0 + 6)) & ((~((~(Instruction)0)<<(8)))<<(0))))) == 0 || (((int)(((fs->f->code[list])>>(0 + 6)) & ((~((~(Instruction)0)<<(8)))<<(0))))) >= level)\x00"
                              as *const u8 as *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          283i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 44],
                                                    &[libc::c_char; 44]>(b"void luaK_patchclose(FuncState *, int, int)\x00")).as_ptr());
        };
        *(*(*fs).f).code.offset(list as isize) = *(*(*fs).f).code.offset(list as isize)
            & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32)
            | (level as Instruction) << 0i32 + 6i32
                & !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32;
        list = getjump(fs, list)
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaK_prefix(
    mut fs: *mut FuncState_0,
    mut op: UnOpr,
    mut e: *mut expdesc_0,
    mut line: libc::c_int,
) -> () {
    let mut current_block: u64;
    static mut ef: expdesc_0 = unsafe {
        expdesc {
            k: VKINT,
            u: unnamed_6 {
                ival: 0i32 as lua_Integer,
            },
            t: -1i32,
            f: -1i32,
        }
    };
    match op as libc::c_uint {
        0 => {
            /* use 'ef' as fake 2nd operand */
            current_block = 5144668994701107217;
        }
        1 => {
            current_block = 5144668994701107217;
        }
        3 => {
            current_block = 9791213246199671339;
        }
        2 => {
            codenot(fs, e);
            current_block = 12517898123489920830;
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    1077i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                        b"void luaK_prefix(FuncState *, UnOpr, expdesc *, int)\x00",
                    )).as_ptr(),
                );
            };
            current_block = 12517898123489920830;
        }
    }
    match current_block {
        5144668994701107217 => {
            if 0 != constfolding(
                fs,
                (op as libc::c_uint).wrapping_add(12i32 as libc::c_uint) as libc::c_int,
                e,
                &ef,
            ) {
                current_block = 12517898123489920830;
            } else {
                /* FALLTHROUGH */
                current_block = 9791213246199671339;
            }
        }
        _ => {}
    }
    match current_block {
        9791213246199671339 => {
            codeunexpval(
                fs,
                (op as libc::c_uint).wrapping_add(OP_UNM as libc::c_int as libc::c_uint) as OpCode,
                e,
                line,
            );
        }
        _ => {}
    };
}
/*
** Code 'not e', doing constant folding.
*/
unsafe extern "C" fn codenot(mut fs: *mut FuncState_0, mut e: *mut expdesc_0) -> () {
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        1 | 3 => {
            /* true == not nil == not false */
            (*e).k = VTRUE
        }
        4 | 5 | 6 | 2 => {
            /* false == not "x" == not 0.5 == not 1 == not true */
            (*e).k = VFALSE
        }
        11 => {
            negatecondition(fs, e);
        }
        12 | 7 => {
            discharge2anyreg(fs, e);
            freeexp(fs, e);
            (*e).u.info = luaK_codeABC(fs, OP_NOT, 0i32, (*e).u.info, 0i32);
            (*e).k = VRELOCABLE
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    933i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                        b"void codenot(FuncState *, expdesc *)\x00",
                    )).as_ptr(),
                );
            };
        }
    }
    /* interchange true and false lists */
    let mut temp: libc::c_int = (*e).f;
    (*e).f = (*e).t;
    (*e).t = temp;
    /* values are useless when negated */
    removevalues(fs, (*e).f);
    removevalues(fs, (*e).t);
}
/*
** Traverse a list of tests ensuring no one produces a value
*/
unsafe extern "C" fn removevalues(mut fs: *mut FuncState_0, mut list: libc::c_int) -> () {
    while list != -1i32 {
        patchtestreg(fs, list, (1i32 << 8i32) - 1i32);
        list = getjump(fs, list)
    }
}
/*
** Emit code for unary expressions that "produce values"
** (everything but 'not').
** Expression to produce final result will be encoded in 'e'.
*/
unsafe extern "C" fn codeunexpval(
    mut fs: *mut FuncState_0,
    mut op: OpCode,
    mut e: *mut expdesc_0,
    mut line: libc::c_int,
) -> () {
    /* opcodes operate only on registers */
    let mut r: libc::c_int = luaK_exp2anyreg(fs, e);
    freeexp(fs, e);
    /* generate opcode */
    (*e).u.info = luaK_codeABC(fs, op, 0i32, r, 0i32);
    /* all those operations are relocatable */
    (*e).k = VRELOCABLE;
    luaK_fixline(fs, line);
}
/*
** Try to "constant-fold" an operation; return 1 iff successful.
** (In this case, 'e1' has the final result.)
*/
unsafe extern "C" fn constfolding(
    mut fs: *mut FuncState_0,
    mut op: libc::c_int,
    mut e1: *mut expdesc_0,
    mut e2: *const expdesc_0,
) -> libc::c_int {
    let mut v1: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut v2: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut res: TValue = lua_TValue {
        value_: Value_0 {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    if 0 == tonumeral(e1, &mut v1)
        || 0 == tonumeral(e2, &mut v2)
        || 0 == validop(op, &mut v1, &mut v2)
    {
        /* non-numeric operands or not safe to fold */
        return 0i32;
    } else {
        /* does operation */
        luaO_arith((*(*fs).ls).L, op, &mut v1, &mut v2, &mut res);
        if res.tt_ == 3i32 | 1i32 << 4i32 {
            (*e1).k = VKINT;
            if res.tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((&res))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    986i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                        b"int constfolding(FuncState *, int, expdesc *, const expdesc *)\x00",
                    )).as_ptr(),
                );
            };
            (*e1).u.ival = res.value_.i
        } else {
            /* folds neither NaN nor 0.0 (to avoid problems with -0.0) */
            if res.tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(
                    b"((((&res))->tt_) == ((3 | (0 << 4))))\x00" as *const u8
                        as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    989i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                        b"int constfolding(FuncState *, int, expdesc *, const expdesc *)\x00",
                    )).as_ptr(),
                );
            };
            let mut n: lua_Number = res.value_.n;
            if !(n == n) || n == 0i32 as libc::c_double {
                return 0i32;
            } else {
                (*e1).k = VKFLT;
                (*e1).u.nval = n
            }
        }
        return 1i32;
    };
}
/*
** Return false if folding can raise an error.
** Bitwise operations need operands convertible to integers; division
** operations cannot have 0 as divisor.
*/
unsafe extern "C" fn validop(
    mut op: libc::c_int,
    mut v1: *mut TValue,
    mut v2: *mut TValue,
) -> libc::c_int {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            /* conversion errors */
            let mut i: lua_Integer = 0;
            return (0 != if (*v1).tt_ == 3i32 | 1i32 << 4i32 {
                if (*v1).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((v1))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lcode.c\x00" as *const u8 as *const libc::c_char,
                        965i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                            b"int validop(int, TValue *, TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                i = (*v1).value_.i;
                1i32
            } else {
                luaV_tointeger(v1, &mut i, 0i32)
            } && 0 != if (*v2).tt_ == 3i32 | 1i32 << 4i32 {
                if (*v2).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"((((v2))->tt_) == ((3 | (1 << 4))))\x00" as *const u8
                            as *const libc::c_char,
                        b"lcode.c\x00" as *const u8 as *const libc::c_char,
                        965i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                            b"int validop(int, TValue *, TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                i = (*v2).value_.i;
                1i32
            } else {
                luaV_tointeger(v2, &mut i, 0i32)
            }) as libc::c_int;
        }
        5 | 6 => {}
        3 => {}
        _ => {
            /* division by 0 */
            /* everything else is valid */
            return 1i32;
        }
    }
    if (*v2).tt_ & 0xfi32 == 3i32 {
    } else {
        __assert_fail(
            b"(((((((v2))->tt_)) & 0x0F)) == (3))\x00" as *const u8 as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            968i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                b"int validop(int, TValue *, TValue *)\x00",
            )).as_ptr(),
        );
    };
    return (if (*v2).tt_ == 3i32 | 1i32 << 4i32 {
        if (*v2).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((v2))->tt_) == ((3 | (1 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                968i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"int validop(int, TValue *, TValue *)\x00",
                )).as_ptr(),
            );
        };
        (*v2).value_.i as lua_Number
    } else {
        if (*v2).tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"((((v2))->tt_) == ((3 | (0 << 4))))\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                968i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
                    b"int validop(int, TValue *, TValue *)\x00",
                )).as_ptr(),
            );
        };
        (*v2).value_.n
    } != 0i32 as libc::c_double) as libc::c_int;
}
/*
** $Id: lcode.c,v 2.111 2016/07/19 17:12:07 roberto Exp roberto $
** Code generator for Lua
** See Copyright Notice in lua.h
*/
/* Maximum number of registers in a Lua function (must fit in 8 bits) */
/*
** If expression is a numeric constant, fills 'v' with its value
** and returns 1. Otherwise, returns 0.
*/
unsafe extern "C" fn tonumeral(mut e: *const expdesc_0, mut v: *mut TValue) -> libc::c_int {
    let mut io_0: *mut TValue = 0 as *mut TValue;
    let mut io: *mut TValue = 0 as *mut TValue;
    if (*e).t != (*e).f {
        /* not a numeral */
        return 0i32;
    } else {
        match (*e).k as libc::c_uint {
            6 => {
                if !v.is_null() {
                    io = v;
                    (*io).value_.i = (*e).u.ival;
                    (*io).tt_ = 3i32 | 1i32 << 4i32
                }
                return 1i32;
            }
            5 => {
                if !v.is_null() {
                    io_0 = v;
                    (*io_0).value_.n = (*e).u.nval;
                    (*io_0).tt_ = 3i32 | 0i32 << 4i32
                }
                return 1i32;
            }
            _ => return 0i32,
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_infix(
    mut fs: *mut FuncState_0,
    mut op: BinOpr_0,
    mut v: *mut expdesc_0,
) -> () {
    match op as libc::c_uint {
        19 => {
            /* go ahead only if 'v' is true */
            luaK_goiftrue(fs, v);
        }
        20 => {
            /* go ahead only if 'v' is false */
            luaK_goiffalse(fs, v);
        }
        12 => {
            /* operand must be on the 'stack' */
            luaK_exp2nextreg(fs, v);
        }
        0 | 1 | 2 | 5 | 6 | 3 | 4 | 7 | 8 | 9 | 10 | 11 => {
            if 0 == tonumeral(v, 0 as *mut TValue) {
                luaK_exp2RK(fs, v);
            }
        }
        _ => {
            /* else keep numeral, which may be folded with 2nd operand */
            luaK_exp2RK(fs, v);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_posfix(
    mut fs: *mut FuncState_0,
    mut op: BinOpr_0,
    mut e1: *mut expdesc_0,
    mut e2: *mut expdesc_0,
    mut line: libc::c_int,
) -> () {
    match op as libc::c_uint {
        19 => {
            if (*e1).t == -1i32 {
            } else {
                __assert_fail(
                    b"e1->t == (-1)\x00" as *const u8 as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    1128i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                        b"void luaK_posfix(FuncState *, BinOpr, expdesc *, expdesc *, int)\x00",
                    )).as_ptr(),
                );
            };
            /* list closed by 'luK_infix' */
            luaK_dischargevars(fs, e2);
            luaK_concat(fs, &mut (*e2).f, (*e1).f);
            *e1 = *e2
        }
        20 => {
            if (*e1).f == -1i32 {
            } else {
                __assert_fail(
                    b"e1->f == (-1)\x00" as *const u8 as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    1135i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                        b"void luaK_posfix(FuncState *, BinOpr, expdesc *, expdesc *, int)\x00",
                    )).as_ptr(),
                );
            };
            /* list closed by 'luK_infix' */
            luaK_dischargevars(fs, e2);
            luaK_concat(fs, &mut (*e2).t, (*e1).t);
            *e1 = *e2
        }
        12 => {
            luaK_exp2val(fs, e2);
            if (*e2).k as libc::c_uint == VRELOCABLE as libc::c_int as libc::c_uint
                && (*(*(*fs).f).code.offset((*e2).u.info as isize) >> 0i32
                    & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
                    as libc::c_uint == OP_CONCAT as libc::c_int as libc::c_uint
            {
                if (*e1).u.info
                    == (*(*(*fs).f).code.offset((*e2).u.info as isize) >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as libc::c_int - 1i32
                {
                } else {
                    __assert_fail(b"e1->u.info == (((int)(((((fs)->f->code[(e2)->u.info]))>>(((0 + 6) + 8) + 9)) & ((~((~(Instruction)0)<<(9)))<<(0)))))-1\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lcode.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1145i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 65],
                                                            &[libc::c_char; 65]>(b"void luaK_posfix(FuncState *, BinOpr, expdesc *, expdesc *, int)\x00")).as_ptr());
                };
                freeexp(fs, e1);
                *(*(*fs).f).code.offset((*e2).u.info as isize) =
                    *(*(*fs).f).code.offset((*e2).u.info as isize)
                        & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32)
                        | ((*e1).u.info as Instruction) << 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32;
                (*e1).k = VRELOCABLE;
                (*e1).u.info = (*e2).u.info
            } else {
                /* operand must be on the 'stack' */
                luaK_exp2nextreg(fs, e2);
                codebinexpval(fs, OP_CONCAT, e1, e2, line);
            }
        }
        0 | 1 | 2 | 5 | 6 | 3 | 4 | 7 | 8 | 9 | 10 | 11 => {
            if 0 == constfolding(
                fs,
                (op as libc::c_uint).wrapping_add(0i32 as libc::c_uint) as libc::c_int,
                e1,
                e2,
            ) {
                codebinexpval(
                    fs,
                    (op as libc::c_uint).wrapping_add(OP_ADD as libc::c_int as libc::c_uint)
                        as OpCode,
                    e1,
                    e2,
                    line,
                );
            }
        }
        13 | 14 | 15 | 16 | 17 | 18 => {
            codecomp(fs, op, e1, e2);
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"lcode.c\x00" as *const u8 as *const libc::c_char,
                    1169i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                        b"void luaK_posfix(FuncState *, BinOpr, expdesc *, expdesc *, int)\x00",
                    )).as_ptr(),
                );
            };
        }
    };
}
/*
** Emit code for comparisons.
** 'e1' was already put in R/K form by 'luaK_infix'.
*/
unsafe extern "C" fn codecomp(
    mut fs: *mut FuncState_0,
    mut opr: BinOpr_0,
    mut e1: *mut expdesc_0,
    mut e2: *mut expdesc_0,
) -> () {
    let mut rk1: libc::c_int = if (*e1).k as libc::c_uint == VK as libc::c_int as libc::c_uint {
        (*e1).u.info | 1i32 << 9i32 - 1i32
    } else {
        if (*e1).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(
                b"e1->k == VNONRELOC\x00" as *const u8 as *const libc::c_char,
                b"lcode.c\x00" as *const u8 as *const libc::c_char,
                1039i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"void codecomp(FuncState *, BinOpr, expdesc *, expdesc *)\x00",
                )).as_ptr(),
            );
        };
        (*e1).u.info
    };
    let mut rk2: libc::c_int = luaK_exp2RK(fs, e2);
    freeexps(fs, e1, e2);
    match opr as libc::c_uint {
        16 => {
            /* '(a ~= b)' ==> 'not (a == b)' */
            (*e1).u.info = condjump(fs, OP_EQ, 0i32, rk1, rk2)
        }
        17 | 18 => {
            /* '(a > b)' ==> '(b < a)';  '(a >= b)' ==> '(b <= a)' */
            let mut op: OpCode = (opr as libc::c_uint)
                .wrapping_sub(OPR_NE as libc::c_int as libc::c_uint)
                .wrapping_add(OP_EQ as libc::c_int as libc::c_uint)
                as OpCode;
            /* invert operands */
            (*e1).u.info = condjump(fs, op, 1i32, rk2, rk1)
        }
        _ => {
            /* '==', '<', '<=' use their own opcodes */
            let mut op_0: OpCode = (opr as libc::c_uint)
                .wrapping_sub(OPR_EQ as libc::c_int as libc::c_uint)
                .wrapping_add(OP_EQ as libc::c_int as libc::c_uint)
                as OpCode;
            (*e1).u.info = condjump(fs, op_0, 1i32, rk1, rk2)
        }
    }
    (*e1).k = VJMP;
}
/*
** Free registers used by expressions 'e1' and 'e2' (if any) in proper
** order.
*/
unsafe extern "C" fn freeexps(
    mut fs: *mut FuncState_0,
    mut e1: *mut expdesc_0,
    mut e2: *mut expdesc_0,
) -> () {
    let mut r1: libc::c_int = if (*e1).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint
    {
        (*e1).u.info
    } else {
        -1i32
    };
    let mut r2: libc::c_int = if (*e2).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint
    {
        (*e2).u.info
    } else {
        -1i32
    };
    if r1 > r2 {
        freereg(fs, r1);
        freereg(fs, r2);
    } else {
        freereg(fs, r2);
        freereg(fs, r1);
    };
}
/*
** Emit code for binary expressions that "produce values"
** (everything but logical operators 'and'/'or' and comparison
** operators).
** Expression to produce final result will be encoded in 'e1'.
** Because 'luaK_exp2RK' can free registers, its calls must be
** in "stack order" (that is, first on 'e2', which may have more
** recent registers to be released).
*/
unsafe extern "C" fn codebinexpval(
    mut fs: *mut FuncState_0,
    mut op: OpCode,
    mut e1: *mut expdesc_0,
    mut e2: *mut expdesc_0,
    mut line: libc::c_int,
) -> () {
    /* both operands are "RK" */
    let mut rk2: libc::c_int = luaK_exp2RK(fs, e2);
    let mut rk1: libc::c_int = luaK_exp2RK(fs, e1);
    freeexps(fs, e1, e2);
    /* generate opcode */
    (*e1).u.info = luaK_codeABC(fs, op, 0i32, rk1, rk2);
    /* all those operations are relocatable */
    (*e1).k = VRELOCABLE;
    luaK_fixline(fs, line);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setlist(
    mut fs: *mut FuncState_0,
    mut base: libc::c_int,
    mut nelems: libc::c_int,
    mut tostore: libc::c_int,
) -> () {
    let mut c: libc::c_int = (nelems - 1i32) / 50i32 + 1i32;
    let mut b: libc::c_int = if tostore == -1i32 { 0i32 } else { tostore };
    if tostore != 0i32 && tostore <= 50i32 {
    } else {
        __assert_fail(
            b"tostore != 0 && tostore <= 50\x00" as *const u8 as *const libc::c_char,
            b"lcode.c\x00" as *const u8 as *const libc::c_char,
            1192i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void luaK_setlist(FuncState *, int, int, int)\x00",
            )).as_ptr(),
        );
    };
    if c <= (1i32 << 9i32) - 1i32 {
        luaK_codeABC(fs, OP_SETLIST, base, b, c);
    } else if c <= (1i32 << 9i32 + 9i32 + 8i32) - 1i32 {
        luaK_codeABC(fs, OP_SETLIST, base, b, 0i32);
        codeextraarg(fs, c);
    } else {
        luaX_syntaxerror(
            (*fs).ls,
            b"constructor too long\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* free registers with list values */
    (*fs).freereg = (base + 1i32) as lu_byte;
}
