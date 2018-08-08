#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(libc, offset_to)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[no_mangle]
    fn _longjmp(__env: *mut __jmp_buf_tag, __val: libc::c_int) -> !;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    static mut l_memcontrol: Memcontrol_0;
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State_0, str: *const libc::c_char, l: size_t)
     -> *mut TString;
    #[no_mangle]
    fn luaG_runerror(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn luaV_execute(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaV_finishOp(L: *mut lua_State_0) -> ();
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaM_realloc_(L: *mut lua_State_0, block: *mut libc::c_void,
                     oldsize: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn luaM_toobig(L: *mut lua_State_0) -> !;
    #[no_mangle]
    fn luaE_shrinkCI(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaE_freeCI(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaF_close(L: *mut lua_State_0, level: StkId) -> ();
    #[no_mangle]
    fn luaT_gettmbyobj(L: *mut lua_State_0, o: *const TValue, event: TMS)
     -> *const TValue;
    #[no_mangle]
    fn luaG_typeerror(L: *mut lua_State_0, o: *const TValue,
                      opname: *const libc::c_char) -> !;
    #[no_mangle]
    fn luaC_step(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaE_extendCI(L: *mut lua_State_0) -> *mut CallInfo_0;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State_0, str: *const libc::c_char)
     -> *mut TString;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaZ_fill(z: *mut ZIO) -> libc::c_int;
    #[no_mangle]
    fn luaF_initupvals(L: *mut lua_State_0, cl: *mut LClosure_0) -> ();
    #[no_mangle]
    fn luaY_parser(L: *mut lua_State_0, z: *mut ZIO, buff: *mut Mbuffer_0,
                   dyd: *mut Dyndata_0, name: *const libc::c_char,
                   firstchar: libc::c_int) -> *mut LClosure_0;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaU_undump(L: *mut lua_State_0, Z: *mut ZIO,
                   name: *const libc::c_char) -> *mut LClosure_0;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut Labeldesc_0,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Dyndata {
    pub actvar: unnamed_6,
    pub gt: Labellist_0,
    pub label: Labellist_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Mbuffer_0,
    pub dyd: Dyndata_0,
    pub mode: *const libc::c_char,
    pub name: *const libc::c_char,
}
pub type __sig_atomic_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: lu_byte,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Vardesc {
    pub idx: libc::c_short,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut libc::c_char,
    pub n: size_t,
    pub buffsize: size_t,
}
pub type ZIO = Zio;
pub type ptrdiff_t = libc::c_long;
pub type intptr_t = libc::c_long;
#[derive ( Copy , Clone )]
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
pub type lu_byte = libc::c_uchar;
pub type sig_atomic_t = __sig_atomic_t;
pub type lua_Hook =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut lua_Debug)
               -> ()>;
pub type lua_Debug = lua_Debug_0;
#[derive ( Copy , Clone )]
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
#[derive ( Copy , Clone )]
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    l: unnamed_1,
    c: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_0 {
    pub k: lua_KFunction,
    pub old_errfunc: ptrdiff_t,
    pub ctx: lua_KContext,
}
pub type lua_KContext = intptr_t;
pub type lua_KFunction =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: libc::c_int,
                                _: lua_KContext) -> libc::c_int>;
pub type lua_State_0 = lua_State;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_1 {
    pub base: StkId,
    pub savedpc: *const Instruction,
}
pub type Instruction = libc::c_uint;
pub type StkId = *mut TValue;
pub type TValue = lua_TValue;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct lua_TValue {
    pub value_: Value,
    pub tt_: libc::c_int,
}
pub type Value = Value_0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Value_0 {
    gc: *mut GCObject,
    p: *mut libc::c_void,
    b: libc::c_int,
    f: lua_CFunction,
    i: lua_Integer,
    n: lua_Number,
}
pub type lua_Number = libc::c_double;
pub type lua_Integer = libc::c_longlong;
pub type lua_CFunction =
    Option<unsafe extern "C" fn(_: *mut lua_State_0) -> libc::c_int>;
pub type GCObject = GCObject_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct GCObject_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
}
pub type CallInfo_0 = CallInfo;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct lua_longjmp {
    pub previous: *mut lua_longjmp,
    pub b: jmp_buf,
    pub status: libc::c_int,
}
pub type UpVal = UpVal_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct UpVal_0 {
    pub v: *mut TValue,
    pub refcount: lu_mem,
    pub u: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    open: unnamed_3,
    value: TValue,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_3 {
    pub next: *mut UpVal,
    pub touched: libc::c_int,
}
pub type lu_mem = size_t;
pub type global_State = global_State_0;
#[derive ( Copy , Clone )]
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
pub type TString = TString_0;
#[derive ( Copy , Clone )]
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_4 {
    lnglen: size_t,
    hnext: *mut TString_0,
}
#[derive ( Copy , Clone )]
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
pub type Node = Node_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Node_0 {
    pub i_val: TValue,
    pub i_key: TKey,
}
pub type TKey = TKey_0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union TKey_0 {
    nk: unnamed_5,
    tvk: TValue,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_5 {
    pub value_: Value,
    pub tt_: libc::c_int,
    pub next: libc::c_int,
}
pub type stringtable = stringtable_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct stringtable_0 {
    pub hash: *mut *mut TString,
    pub nuse: libc::c_int,
    pub size: libc::c_int,
}
pub type l_mem = ptrdiff_t;
pub type lua_Alloc =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: size_t, _: size_t) -> *mut libc::c_void>;
pub type lua_Reader =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void,
                                _: *mut size_t) -> *const libc::c_char>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
pub type Memcontrol_0 = Memcontrol;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct L_EXTRA {
    pub lock: libc::c_int,
    pub plock: *mut libc::c_int,
}
pub type Vardesc_0 = Vardesc;
pub type Dyndata_0 = Dyndata;
pub type Mbuffer_0 = Mbuffer;
pub type Labeldesc_0 = Labeldesc;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_6 {
    pub arr: *mut Vardesc_0,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
pub type Labellist_0 = Labellist;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata,
    cl: Closure,
    h: Table,
    p: Proto,
    th: lua_State,
}
#[derive ( Copy , Clone )]
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
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
pub type Upvaldesc = Upvaldesc_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Upvaldesc_0 {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
pub type LocVar = LocVar_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LocVar_0 {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure,
    l: LClosure_0,
}
pub type LClosure_0 = LClosure;
pub type CClosure = CClosure_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
#[derive ( Copy , Clone )]
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
pub type Pfunc =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void)
               -> ()>;
pub type TMS = libc::c_uint;
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
pub const TM_EQ: TMS = 5;
pub const TM_LEN: TMS = 4;
pub const TM_MODE: TMS = 3;
pub const TM_GC: TMS = 2;
pub const TM_NEWINDEX: TMS = 1;
pub const TM_INDEX: TMS = 0;
pub const OP_TAILCALL: OpCode = 37;
pub type OpCode = libc::c_uint;
pub const OP_EXTRAARG: OpCode = 46;
pub const OP_VARARG: OpCode = 45;
pub const OP_CLOSURE: OpCode = 44;
pub const OP_SETLIST: OpCode = 43;
pub const OP_TFORLOOP: OpCode = 42;
pub const OP_TFORCALL: OpCode = 41;
pub const OP_FORPREP: OpCode = 40;
pub const OP_FORLOOP: OpCode = 39;
pub const OP_RETURN: OpCode = 38;
pub const OP_CALL: OpCode = 36;
pub const OP_TESTSET: OpCode = 35;
pub const OP_TEST: OpCode = 34;
pub const OP_LE: OpCode = 33;
pub const OP_LT: OpCode = 32;
pub const OP_EQ: OpCode = 31;
pub const OP_JMP: OpCode = 30;
pub const OP_CONCAT: OpCode = 29;
pub const OP_LEN: OpCode = 28;
pub const OP_NOT: OpCode = 27;
pub const OP_BNOT: OpCode = 26;
pub const OP_UNM: OpCode = 25;
pub const OP_SHR: OpCode = 24;
pub const OP_SHL: OpCode = 23;
pub const OP_BXOR: OpCode = 22;
pub const OP_BOR: OpCode = 21;
pub const OP_BAND: OpCode = 20;
pub const OP_IDIV: OpCode = 19;
pub const OP_DIV: OpCode = 18;
pub const OP_POW: OpCode = 17;
pub const OP_MOD: OpCode = 16;
pub const OP_MUL: OpCode = 15;
pub const OP_SUB: OpCode = 14;
pub const OP_ADD: OpCode = 13;
pub const OP_SELF: OpCode = 12;
pub const OP_NEWTABLE: OpCode = 11;
pub const OP_SETTABLE: OpCode = 10;
pub const OP_SETUPVAL: OpCode = 9;
pub const OP_SETTABUP: OpCode = 8;
pub const OP_GETTABLE: OpCode = 7;
pub const OP_GETTABUP: OpCode = 6;
pub const OP_GETUPVAL: OpCode = 5;
pub const OP_LOADNIL: OpCode = 4;
pub const OP_LOADBOOL: OpCode = 3;
pub const OP_LOADKX: OpCode = 2;
pub const OP_LOADK: OpCode = 1;
pub const OP_MOVE: OpCode = 0;
pub type Proto_0 = Proto;
#[no_mangle]
pub unsafe extern "C" fn lua_yieldk(mut L: *mut lua_State_0,
                                    mut nresults: libc::c_int,
                                    mut ctx: lua_KContext,
                                    mut k: lua_KFunction) -> libc::c_int {
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let ref mut fresh0 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
    let fresh1 = *fresh0;
    *fresh0 = *fresh0 + 1;
    if fresh1 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      697i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"int lua_yieldk(lua_State *, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    if (nresults as libc::c_long) <
           (*(*L).ci).func.offset_to((*L).top).expect("bad offset_to") as
               libc::c_long &&
           !(b"not enough elements in the stack\x00" as *const u8 as
                 *const libc::c_char).is_null() {
    } else {
        __assert_fail(b"((nresults) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      698i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"int lua_yieldk(lua_State *, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
    };
    if (*L).nny as libc::c_int > 0i32 {
        if L != (*(*L).l_G).mainthread {
            luaG_runerror(L,
                          b"attempt to yield across a C-call boundary\x00" as
                              *const u8 as *const libc::c_char);
        } else {
            luaG_runerror(L,
                          b"attempt to yield from outside a coroutine\x00" as
                              *const u8 as *const libc::c_char);
        }
    } else {
        (*L).status = 1i32 as lu_byte;
        (*ci).extra =
            ((*L).stack as
                 *mut libc::c_char).offset_to((*ci).func as
                                                  *mut libc::c_char).expect("bad offset_to")
                as libc::c_long;
        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
            if k.is_none() &&
                   !(b"hooks cannot continue after yielding\x00" as *const u8
                         as *const libc::c_char).is_null() {
            } else {
                __assert_fail(b"(k == ((void*)0)) && \"hooks cannot continue after yielding\"\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 708i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"int lua_yieldk(lua_State *, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
            };
            if 0 != (*ci).callstatus as libc::c_int & 1i32 << 2i32 {
            } else {
                __assert_fail(b"ci->callstatus & (1<<2)\x00" as *const u8 as
                                  *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 716i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"int lua_yieldk(lua_State *, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
            };
            let ref mut fresh2 =
                *(*((L as
                         *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                         as libc::c_ulong as
                                                         isize)) as
                        *mut libc::c_void as *mut L_EXTRA)).plock;
            *fresh2 -= 1;
            if *fresh2 == 0i32 {
            } else {
                __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 717i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"int lua_yieldk(lua_State *, int, lua_KContext, lua_KFunction)\x00")).as_ptr());
            };
            return 0i32
        } else {
            (*ci).u.c.k = k;
            if (*ci).u.c.k.is_some() { (*ci).u.c.ctx = ctx }
            (*ci).func =
                (*L).top.offset(-(nresults as isize)).offset(-1isize);
            luaD_throw(L, 1i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_throw(mut L: *mut lua_State_0,
                                    mut errcode: libc::c_int) -> ! {
    if !(*L).errorJmp.is_null() {
        ::std::ptr::write_volatile(&mut (*(*L).errorJmp).status as
                                       *mut libc::c_int, errcode);
        _longjmp((*(*L).errorJmp).b.as_mut_ptr(), 1i32);
    } else {
        let mut g: *mut global_State = (*L).l_G;
        (*L).status = errcode as lu_byte;
        if !(*(*g).mainthread).errorJmp.is_null() {
            let fresh3 = (*(*g).mainthread).top;
            (*(*g).mainthread).top = (*(*g).mainthread).top.offset(1);
            let mut io1: *mut TValue = fresh3;
            *io1 = *(*L).top.offset(-1isize);
            if 0 == (*io1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ldo.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         120i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 34],
                                                                   &[libc::c_char; 34]>(b"void luaD_throw(lua_State *, int)\x00")).as_ptr());
                       };
                       (*io1).tt_ & 63i32 ==
                           (*(*io1).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ldo.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      120i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 34],
                                                                                &[libc::c_char; 34]>(b"void luaD_throw(lua_State *, int)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io1).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  120i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 34],
                                                            &[libc::c_char; 34]>(b"void luaD_throw(lua_State *, int)\x00")).as_ptr());
                };
            };
            luaD_throw((*g).mainthread, errcode);
        } else {
            if (*g).panic.is_some() {
                seterrorobj(L, errcode, (*L).top);
                if (*(*L).ci).top < (*L).top { (*(*L).ci).top = (*L).top }
                let ref mut fresh4 =
                    *(*((L as
                             *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                             as libc::c_ulong
                                                             as isize)) as
                            *mut libc::c_void as *mut L_EXTRA)).plock;
                *fresh4 -= 1;
                if *fresh4 == 0i32 {
                } else {
                    __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  128i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 34],
                                                            &[libc::c_char; 34]>(b"void luaD_throw(lua_State *, int)\x00")).as_ptr());
                };
                (*g).panic.expect("non-null function pointer")(L);
            }
            abort();
        }
    };
}
unsafe extern "C" fn seterrorobj(mut L: *mut lua_State_0,
                                 mut errcode: libc::c_int, mut oldtop: StkId)
 -> () {
    match errcode {
        4 => {
            let mut io: *mut TValue = oldtop;
            let mut x_: *mut TString = (*(*L).l_G).memerrmsg;
            if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8
                                  as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 95i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
            };
            (*io).value_.gc =
                &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
            (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
            if 0 == (*io).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ldo.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         95i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 42],
                                                                   &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                       };
                       (*io).tt_ & 63i32 ==
                           (*(*io).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ldo.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      95i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 42],
                                                                                &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  95i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 42],
                                                            &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                };
            };
        }
        6 => {
            let mut io_0: *mut TValue = oldtop;
            let mut x__0: *mut TString =
                luaS_newlstr(L,
                             b"error in error handling\x00" as *const u8 as
                                 *const libc::c_char,
                             (::std::mem::size_of::<[libc::c_char; 24]>() as
                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                                  as
                                                                  libc::c_ulong).wrapping_sub(1i32
                                                                                                  as
                                                                                                  libc::c_ulong));
            if (*x__0).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8
                                  as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 99i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
            };
            (*io_0).value_.gc =
                &mut (*(x__0 as *mut GCUnion)).gc as *mut GCObject;
            (*io_0).tt_ = (*x__0).tt as libc::c_int | 1i32 << 6i32;
            if 0 == (*io_0).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io_0).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ldo.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         99i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 42],
                                                                   &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                       };
                       (*io_0).tt_ & 63i32 ==
                           (*(*io_0).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io_0).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ldo.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      99i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 42],
                                                                                &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io_0).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  99i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 42],
                                                            &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                };
            };
        }
        _ => {
            let mut io1: *mut TValue = oldtop;
            *io1 = *(*L).top.offset(-1isize);
            if 0 == (*io1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ldo.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         103i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 42],
                                                                   &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                       };
                       (*io1).tt_ & 63i32 ==
                           (*(*io1).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ldo.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      103i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 42],
                                                                                &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io1).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  103i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 42],
                                                            &[libc::c_char; 42]>(b"void seterrorobj(lua_State *, int, StkId)\x00")).as_ptr());
                };
            };
        }
    }
    (*L).top = oldtop.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_resume(mut L: *mut lua_State_0,
                                    mut from: *mut lua_State_0,
                                    mut nargs: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut oldnny: libc::c_ushort = (*L).nny;
    let ref mut fresh5 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
    let fresh6 = *fresh5;
    *fresh5 = *fresh5 + 1;
    if fresh6 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      652i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"int lua_resume(lua_State *, lua_State *, int)\x00")).as_ptr());
    };
    if (*L).status as libc::c_int == 0i32 {
        if (*L).ci != &mut (*L).base_ci as *mut CallInfo_0 {
            return resume_error(L,
                                b"cannot resume non-suspended coroutine\x00"
                                    as *const u8 as *const libc::c_char,
                                nargs)
        }
    } else if (*L).status as libc::c_int != 1i32 {
        return resume_error(L,
                            b"cannot resume dead coroutine\x00" as *const u8
                                as *const libc::c_char, nargs)
    }
    (*L).nCcalls =
        (if !from.is_null() {
             (*from).nCcalls as libc::c_int + 1i32
         } else { 1i32 }) as libc::c_ushort;
    if (*L).nCcalls as libc::c_int >= 200i32 {
        return resume_error(L,
                            b"C stack overflow\x00" as *const u8 as
                                *const libc::c_char, nargs)
    } else {
        (*L).nny = 0i32 as libc::c_ushort;
        if ((if (*L).status as libc::c_int == 0i32 {
                 nargs + 1i32
             } else { nargs }) as libc::c_long) <
               (*(*L).ci).func.offset_to((*L).top).expect("bad offset_to") as
                   libc::c_long &&
               !(b"not enough elements in the stack\x00" as *const u8 as
                     *const libc::c_char).is_null() {
        } else {
            __assert_fail(b"(((L->status == 0) ? nargs + 1 : nargs) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          664i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"int lua_resume(lua_State *, lua_State *, int)\x00")).as_ptr());
        };
        status =
            luaD_rawrunprotected(L, Some(resume),
                                 &mut nargs as *mut libc::c_int as
                                     *mut libc::c_void);
        if status == -1i32 {
            status = 2i32
        } else {
            while status > 1i32 && 0 != recover(L, status) {
                status =
                    luaD_rawrunprotected(L, Some(unroll),
                                         &mut status as *mut libc::c_int as
                                             *mut libc::c_void)
            }
            if status > 1i32 {
                (*L).status = status as lu_byte;
                seterrorobj(L, status, (*L).top);
                (*(*L).ci).top = (*L).top
            } else {
                if status == (*L).status as libc::c_int {
                } else {
                    __assert_fail(b"status == L->status\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  678i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 46],
                                                            &[libc::c_char; 46]>(b"int lua_resume(lua_State *, lua_State *, int)\x00")).as_ptr());
                };
            }
        }
        (*L).nny = oldnny;
        (*L).nCcalls = (*L).nCcalls.wrapping_sub(1);
        if (*L).nCcalls as libc::c_int ==
               if !from.is_null() {
                   (*from).nCcalls as libc::c_int
               } else { 0i32 } {
        } else {
            __assert_fail(b"L->nCcalls == ((from) ? from->nCcalls : 0)\x00" as
                              *const u8 as *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          682i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"int lua_resume(lua_State *, lua_State *, int)\x00")).as_ptr());
        };
        let ref mut fresh7 =
            *(*((L as
                     *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                     as libc::c_ulong as
                                                     isize)) as
                    *mut libc::c_void as *mut L_EXTRA)).plock;
        *fresh7 -= 1;
        if *fresh7 == 0i32 {
        } else {
            __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          683i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"int lua_resume(lua_State *, lua_State *, int)\x00")).as_ptr());
        };
        return status
    };
}
unsafe extern "C" fn unroll(mut L: *mut lua_State_0,
                            mut ud: *mut libc::c_void) -> () {
    if !ud.is_null() { finishCcall(L, *(ud as *mut libc::c_int)); }
    while (*L).ci != &mut (*L).base_ci as *mut CallInfo_0 {
        if 0 == (*(*L).ci).callstatus as libc::c_int & 1i32 << 1i32 {
            finishCcall(L, 1i32);
        } else { luaV_finishOp(L); luaV_execute(L); }
    };
}
unsafe extern "C" fn finishCcall(mut L: *mut lua_State_0,
                                 mut status: libc::c_int) -> () {
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let mut n: libc::c_int = 0;
    if (*ci).u.c.k.is_some() && (*L).nny as libc::c_int == 0i32 {
    } else {
        __assert_fail(b"ci->u.c.k != ((void*)0) && L->nny == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      523i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void finishCcall(lua_State *, int)\x00")).as_ptr());
    };
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 4i32 || status == 1i32 {
    } else {
        __assert_fail(b"(ci->callstatus & (1<<4)) || status == 1\x00" as
                          *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      525i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void finishCcall(lua_State *, int)\x00")).as_ptr());
    };
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 4i32 {
        (*ci).callstatus =
            ((*ci).callstatus as libc::c_int & !(1i32 << 4i32)) as
                libc::c_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc
    }
    if (*ci).nresults as libc::c_int == -1i32 && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top
    }
    let ref mut fresh8 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
    *fresh8 -= 1;
    if *fresh8 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      533i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void finishCcall(lua_State *, int)\x00")).as_ptr());
    };
    n =
        (*ci).u.c.k.expect("non-null function pointer")(L, status,
                                                        (*ci).u.c.ctx);
    let ref mut fresh9 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
    let fresh10 = *fresh9;
    *fresh9 = *fresh9 + 1;
    if fresh10 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      535i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void finishCcall(lua_State *, int)\x00")).as_ptr());
    };
    if (n as libc::c_long) <
           (*(*L).ci).func.offset_to((*L).top).expect("bad offset_to") as
               libc::c_long &&
           !(b"not enough elements in the stack\x00" as *const u8 as
                 *const libc::c_char).is_null() {
    } else {
        __assert_fail(b"((n) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      536i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void finishCcall(lua_State *, int)\x00")).as_ptr());
    };
    luaD_poscall(L, ci, (*L).top.offset(-(n as isize)), n);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_poscall(mut L: *mut lua_State_0,
                                      mut ci: *mut CallInfo_0,
                                      mut firstResult: StkId,
                                      mut nres: libc::c_int) -> libc::c_int {
    let mut fr: ptrdiff_t = 0;
    let mut res: StkId = 0 as *mut TValue;
    let mut wanted: libc::c_int = (*ci).nresults as libc::c_int;
    if 0 != (*L).hookmask & (1i32 << 1i32 | 1i32 << 2i32) {
        if 0 != (*L).hookmask & 1i32 << 1i32 {
            fr =
                ((*L).stack as
                     *mut libc::c_char).offset_to(firstResult as
                                                      *mut libc::c_char).expect("bad offset_to")
                    as libc::c_long;
            luaD_hook(L, 1i32, -1i32);
            firstResult =
                ((*L).stack as *mut libc::c_char).offset(fr as isize) as
                    *mut TValue
        }
        (*L).oldpc = (*(*ci).previous).u.l.savedpc
    }
    res = (*ci).func;
    (*L).ci = (*ci).previous;
    return moveresults(L, firstResult as *const TValue, res, nres, wanted);
}
unsafe extern "C" fn moveresults(mut L: *mut lua_State_0,
                                 mut firstResult: *const TValue,
                                 mut res: StkId, mut nres: libc::c_int,
                                 mut wanted: libc::c_int) -> libc::c_int {
    match wanted {
        0 => { }
        1 => {
            if nres == 0i32 { firstResult = &luaO_nilobject_ }
            let mut io1: *mut TValue = res;
            *io1 = *firstResult;
            if 0 == (*io1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ldo.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         342i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 62],
                                                                   &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                       };
                       (*io1).tt_ & 63i32 ==
                           (*(*io1).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ldo.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      342i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 62],
                                                                                &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io1).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  342i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 62],
                                                            &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                };
            };
        }
        -1 => {
            let mut i: libc::c_int = 0;
            i = 0i32;
            while i < nres {
                let mut io1_0: *mut TValue = res.offset(i as isize);
                *io1_0 = *firstResult.offset(i as isize);
                if 0 == (*io1_0).tt_ & 1i32 << 6i32 ||
                       {
                           if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                           } else {
                               __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"ldo.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             348i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 62],
                                                                       &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                           };
                           (*io1_0).tt_ & 63i32 ==
                               (*(*io1_0).value_.gc).tt as libc::c_int &&
                               (L.is_null() ||
                                    {
                                        if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                                        } else {
                                            __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          b"ldo.c\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          348i32 as
                                                              libc::c_uint,
                                                          (*::std::mem::transmute::<&[u8; 62],
                                                                                    &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                                        };
                                        0 !=
                                            ((*(*io1_0).value_.gc).marked as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32)) &
                                                ((*(*L).l_G).currentwhite as
                                                     libc::c_int ^
                                                     (1i32 << 0i32 |
                                                          1i32 << 1i32))
                                    })
                       } {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(b"0\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"ldo.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      348i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 62],
                                                                &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                    };
                };
                i += 1
            }
            (*L).top = res.offset(nres as isize);
            return 0i32
        }
        _ => {
            let mut i_0: libc::c_int = 0;
            if wanted <= nres {
                i_0 = 0i32;
                while i_0 < wanted {
                    let mut io1_1: *mut TValue = res.offset(i_0 as isize);
                    *io1_1 = *firstResult.offset(i_0 as isize);
                    if 0 == (*io1_1).tt_ & 1i32 << 6i32 ||
                           {
                               if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
                               } else {
                                   __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"ldo.c\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 356i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 62],
                                                                           &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                               };
                               (*io1_1).tt_ & 63i32 ==
                                   (*(*io1_1).value_.gc).tt as libc::c_int &&
                                   (L.is_null() ||
                                        {
                                            if 0 !=
                                                   (*io1_1).tt_ & 1i32 << 6i32
                                               {
                                            } else {
                                                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              b"ldo.c\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              356i32 as
                                                                  libc::c_uint,
                                                              (*::std::mem::transmute::<&[u8; 62],
                                                                                        &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                                            };
                                            0 !=
                                                ((*(*io1_1).value_.gc).marked
                                                     as libc::c_int ^
                                                     (1i32 << 0i32 |
                                                          1i32 << 1i32)) &
                                                    ((*(*L).l_G).currentwhite
                                                         as libc::c_int ^
                                                         (1i32 << 0i32 |
                                                              1i32 << 1i32))
                                        })
                           } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(b"0\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"ldo.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          356i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 62],
                                                                    &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                        };
                    };
                    i_0 += 1
                }
            } else {
                i_0 = 0i32;
                while i_0 < nres {
                    let mut io1_2: *mut TValue = res.offset(i_0 as isize);
                    *io1_2 = *firstResult.offset(i_0 as isize);
                    if 0 == (*io1_2).tt_ & 1i32 << 6i32 ||
                           {
                               if 0 != (*io1_2).tt_ & 1i32 << 6i32 {
                               } else {
                                   __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"ldo.c\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 360i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 62],
                                                                           &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                               };
                               (*io1_2).tt_ & 63i32 ==
                                   (*(*io1_2).value_.gc).tt as libc::c_int &&
                                   (L.is_null() ||
                                        {
                                            if 0 !=
                                                   (*io1_2).tt_ & 1i32 << 6i32
                                               {
                                            } else {
                                                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              b"ldo.c\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              360i32 as
                                                                  libc::c_uint,
                                                              (*::std::mem::transmute::<&[u8; 62],
                                                                                        &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                                            };
                                            0 !=
                                                ((*(*io1_2).value_.gc).marked
                                                     as libc::c_int ^
                                                     (1i32 << 0i32 |
                                                          1i32 << 1i32)) &
                                                    ((*(*L).l_G).currentwhite
                                                         as libc::c_int ^
                                                         (1i32 << 0i32 |
                                                              1i32 << 1i32))
                                        })
                           } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(b"0\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"ldo.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          360i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 62],
                                                                    &[libc::c_char; 62]>(b"int moveresults(lua_State *, const TValue *, StkId, int, int)\x00")).as_ptr());
                        };
                    };
                    i_0 += 1
                }
                while i_0 < wanted {
                    (*res.offset(i_0 as isize)).tt_ = 0i32;
                    i_0 += 1
                }
            }
        }
    }
    (*L).top = res.offset(wanted as isize);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_hook(mut L: *mut lua_State_0,
                                   mut event: libc::c_int,
                                   mut line: libc::c_int) -> () {
    let mut hook: lua_Hook = (*L).hook;
    if hook.is_some() && 0 != (*L).allowhook as libc::c_int {
        let mut ci: *mut CallInfo_0 = (*L).ci;
        let mut top: ptrdiff_t =
            ((*L).stack as
                 *mut libc::c_char).offset_to((*L).top as
                                                  *mut libc::c_char).expect("bad offset_to")
                as libc::c_long;
        let mut ci_top: ptrdiff_t =
            ((*L).stack as
                 *mut libc::c_char).offset_to((*ci).top as
                                                  *mut libc::c_char).expect("bad offset_to")
                as libc::c_long;
        let mut ar: lua_Debug =
            lua_Debug_0{event: 0,
                        name: 0 as *const libc::c_char,
                        namewhat: 0 as *const libc::c_char,
                        what: 0 as *const libc::c_char,
                        source: 0 as *const libc::c_char,
                        currentline: 0,
                        linedefined: 0,
                        lastlinedefined: 0,
                        nups: 0,
                        nparams: 0,
                        isvararg: 0,
                        istailcall: 0,
                        short_src: [0; 60],
                        i_ci: 0 as *mut CallInfo,};
        ar.event = event;
        ar.currentline = line;
        ar.i_ci = ci;
        if (*L).top.offset_to((*L).stack_last).expect("bad offset_to") as
               libc::c_long <= 20i32 as libc::c_long {
            luaD_growstack(L, 20i32);
        }
        (*ci).top = (*L).top.offset(20isize);
        if (*ci).top <= (*L).stack_last {
        } else {
            __assert_fail(b"ci->top <= L->stack_last\x00" as *const u8 as
                              *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          266i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"void luaD_hook(lua_State *, int, int)\x00")).as_ptr());
        };
        (*L).allowhook = 0i32 as lu_byte;
        (*ci).callstatus =
            ((*ci).callstatus as libc::c_int | 1i32 << 2i32) as
                libc::c_ushort;
        let ref mut fresh11 =
            *(*((L as
                     *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                     as libc::c_ulong as
                                                     isize)) as
                    *mut libc::c_void as *mut L_EXTRA)).plock;
        *fresh11 -= 1;
        if *fresh11 == 0i32 {
        } else {
            __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          269i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"void luaD_hook(lua_State *, int, int)\x00")).as_ptr());
        };
        hook.expect("non-null function pointer")(L, &mut ar);
        let ref mut fresh12 =
            *(*((L as
                     *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                     as libc::c_ulong as
                                                     isize)) as
                    *mut libc::c_void as *mut L_EXTRA)).plock;
        let fresh13 = *fresh12;
        *fresh12 = *fresh12 + 1;
        if fresh13 == 0i32 {
        } else {
            __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          271i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"void luaD_hook(lua_State *, int, int)\x00")).as_ptr());
        };
        if 0 == (*L).allowhook {
        } else {
            __assert_fail(b"!L->allowhook\x00" as *const u8 as
                              *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          272i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"void luaD_hook(lua_State *, int, int)\x00")).as_ptr());
        };
        (*L).allowhook = 1i32 as lu_byte;
        (*ci).top =
            ((*L).stack as *mut libc::c_char).offset(ci_top as isize) as
                *mut TValue;
        (*L).top =
            ((*L).stack as *mut libc::c_char).offset(top as isize) as
                *mut TValue;
        (*ci).callstatus =
            ((*ci).callstatus as libc::c_int & !(1i32 << 2i32)) as
                libc::c_ushort
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_growstack(mut L: *mut lua_State_0,
                                        mut n: libc::c_int) -> () {
    let mut size: libc::c_int = (*L).stacksize;
    if size > 50000i32 {
        luaD_throw(L, 6i32);
    } else {
        let mut needed: libc::c_int =
            (*L).stack.offset_to((*L).top).expect("bad offset_to") as
                libc::c_long as libc::c_int + n + 5i32;
        let mut newsize: libc::c_int = 2i32 * size;
        if newsize > 50000i32 { newsize = 50000i32 }
        if newsize < needed { newsize = needed }
        if newsize > 50000i32 {
            luaD_reallocstack(L, 50000i32 + 200i32);
            luaG_runerror(L,
                          b"stack overflow\x00" as *const u8 as
                              *const libc::c_char);
        } else { luaD_reallocstack(L, newsize); return; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_reallocstack(mut L: *mut lua_State_0,
                                           mut newsize: libc::c_int) -> () {
    let mut oldstack: *mut TValue = (*L).stack;
    let mut lim: libc::c_int = (*L).stacksize;
    if newsize <= 50000i32 || newsize == 50000i32 + 200i32 {
    } else {
        __assert_fail(b"newsize <= 50000 || newsize == (50000 + 200)\x00" as
                          *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      181i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void luaD_reallocstack(lua_State *, int)\x00")).as_ptr());
    };
    if (*L).stack.offset_to((*L).stack_last).expect("bad offset_to") as
           libc::c_long == ((*L).stacksize - 5i32) as libc::c_long {
    } else {
        __assert_fail(b"L->stack_last - L->stack == L->stacksize - 5\x00" as
                          *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      182i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void luaD_reallocstack(lua_State *, int)\x00")).as_ptr());
    };
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (newsize as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<TValue>() as
                                                libc::c_ulong) {
        luaM_toobig(L);
    } else { };
    (*L).stack =
        luaM_realloc_(L, (*L).stack as *mut libc::c_void,
                      ((*L).stacksize as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong),
                      (newsize as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong))
            as *mut TValue;
    while lim < newsize {
        (*(*L).stack.offset(lim as isize)).tt_ = 0i32;
        lim += 1
    }
    (*L).stacksize = newsize;
    (*L).stack_last = (*L).stack.offset(newsize as isize).offset(-5isize);
    correctstack(L, oldstack);
}
unsafe extern "C" fn correctstack(mut L: *mut lua_State_0,
                                  mut oldstack: *mut TValue) -> () {
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    let mut up: *mut UpVal = 0 as *mut UpVal;
    (*L).top =
        (*L).stack.offset(oldstack.offset_to((*L).top).expect("bad offset_to")
                              as libc::c_long as isize);
    up = (*L).openupval;
    while !up.is_null() {
        (*up).v =
            (*L).stack.offset(oldstack.offset_to((*up).v).expect("bad offset_to")
                                  as libc::c_long as isize);
        up = (*up).u.open.next
    }
    ci = (*L).ci;
    while !ci.is_null() {
        (*ci).top =
            (*L).stack.offset(oldstack.offset_to((*ci).top).expect("bad offset_to")
                                  as libc::c_long as isize);
        (*ci).func =
            (*L).stack.offset(oldstack.offset_to((*ci).func).expect("bad offset_to")
                                  as libc::c_long as isize);
        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
            (*ci).u.l.base =
                (*L).stack.offset(oldstack.offset_to((*ci).u.l.base).expect("bad offset_to")
                                      as libc::c_long as isize)
        }
        ci = (*ci).previous
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_rawrunprotected(mut L: *mut lua_State_0,
                                              mut f: Pfunc,
                                              mut ud: *mut libc::c_void)
 -> libc::c_int {
    let mut oldnCcalls: libc::c_ushort = (*L).nCcalls;
    let mut lj: lua_longjmp =
        lua_longjmp{previous: 0 as *mut lua_longjmp,
                    b:
                        [__jmp_buf_tag{__jmpbuf: [0; 8],
                                       __mask_was_saved: 0,
                                       __saved_mask:
                                           __sigset_t{__val: [0; 16],},}; 1],
                    status: 0,};
    ::std::ptr::write_volatile(&mut lj.status as *mut libc::c_int, 0i32);
    lj.previous = (*L).errorJmp;
    (*L).errorJmp = &mut lj;
    if _setjmp(lj.b.as_mut_ptr()) == 0i32 {
        f.expect("non-null function pointer")(L, ud);
    }
    (*L).errorJmp = lj.previous;
    (*L).nCcalls = oldnCcalls;
    return lj.status;
}
unsafe extern "C" fn recover(mut L: *mut lua_State_0, mut status: libc::c_int)
 -> libc::c_int {
    let mut oldtop: StkId = 0 as *mut TValue;
    let mut ci: *mut CallInfo_0 = findpcall(L);
    if ci.is_null() {
        return 0i32
    } else {
        oldtop =
            ((*L).stack as *mut libc::c_char).offset((*ci).extra as isize) as
                *mut TValue;
        luaF_close(L, oldtop);
        seterrorobj(L, status, oldtop);
        (*L).ci = ci;
        (*L).allowhook =
            ((*ci).callstatus as libc::c_int & 1i32 << 0i32) as lu_byte;
        (*L).nny = 0i32 as libc::c_ushort;
        luaD_shrinkstack(L);
        (*L).errfunc = (*ci).u.c.old_errfunc;
        return 1i32
    };
}
unsafe extern "C" fn findpcall(mut L: *mut lua_State_0) -> *mut CallInfo_0 {
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    ci = (*L).ci;
    while !ci.is_null() {
        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 4i32 {
            return ci
        } else { ci = (*ci).previous }
    }
    return 0 as *mut CallInfo_0;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_shrinkstack(mut L: *mut lua_State_0) -> () {
    let mut inuse: libc::c_int = stackinuse(L);
    let mut goodsize: libc::c_int = inuse + inuse / 8i32 + 2i32 * 5i32;
    if goodsize > 50000i32 { goodsize = 50000i32 }
    if (*L).stacksize > 50000i32 {
        luaE_freeCI(L);
    } else { luaE_shrinkCI(L); }
    if inuse <= 50000i32 - 5i32 && goodsize < (*L).stacksize {
        luaD_reallocstack(L, goodsize);
    };
}
unsafe extern "C" fn stackinuse(mut L: *mut lua_State_0) -> libc::c_int {
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    let mut lim: StkId = (*L).top;
    ci = (*L).ci;
    while !ci.is_null() {
        if lim < (*ci).top { lim = (*ci).top }
        ci = (*ci).previous
    }
    if lim <= (*L).stack_last {
    } else {
        __assert_fail(b"lim <= L->stack_last\x00" as *const u8 as
                          *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      217i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"int stackinuse(lua_State *)\x00")).as_ptr());
    };
    return (*L).stack.offset_to(lim).expect("bad offset_to") as libc::c_long
               as libc::c_int + 1i32;
}
unsafe extern "C" fn resume(mut L: *mut lua_State_0,
                            mut ud: *mut libc::c_void) -> () {
    let mut n: libc::c_int = *(ud as *mut libc::c_int);
    let mut firstArg: StkId = (*L).top.offset(-(n as isize));
    let mut ci: *mut CallInfo_0 = (*L).ci;
    if (*L).status as libc::c_int == 0i32 {
        if 0 == luaD_precall(L, firstArg.offset(-1isize), -1i32) {
            luaV_execute(L);
        }
    } else {
        if (*L).status as libc::c_int == 1i32 {
        } else {
            __assert_fail(b"L->status == 1\x00" as *const u8 as
                              *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          629i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void resume(lua_State *, void *)\x00")).as_ptr());
        };
        (*L).status = 0i32 as lu_byte;
        (*ci).func =
            ((*L).stack as *mut libc::c_char).offset((*ci).extra as isize) as
                *mut TValue;
        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
            luaV_execute(L);
        } else {
            if (*ci).u.c.k.is_some() {
                let ref mut fresh14 =
                    *(*((L as
                             *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                             as libc::c_ulong
                                                             as isize)) as
                            *mut libc::c_void as *mut L_EXTRA)).plock;
                *fresh14 -= 1;
                if *fresh14 == 0i32 {
                } else {
                    __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  636i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 33],
                                                            &[libc::c_char; 33]>(b"void resume(lua_State *, void *)\x00")).as_ptr());
                };
                n =
                    (*ci).u.c.k.expect("non-null function pointer")(L, 1i32,
                                                                    (*ci).u.c.ctx);
                let ref mut fresh15 =
                    *(*((L as
                             *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                             as libc::c_ulong
                                                             as isize)) as
                            *mut libc::c_void as *mut L_EXTRA)).plock;
                let fresh16 = *fresh15;
                *fresh15 = *fresh15 + 1;
                if fresh16 == 0i32 {
                } else {
                    __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  638i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 33],
                                                            &[libc::c_char; 33]>(b"void resume(lua_State *, void *)\x00")).as_ptr());
                };
                if (n as libc::c_long) <
                       (*(*L).ci).func.offset_to((*L).top).expect("bad offset_to")
                           as libc::c_long &&
                       !(b"not enough elements in the stack\x00" as *const u8
                             as *const libc::c_char).is_null() {
                } else {
                    __assert_fail(b"((n) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  639i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 33],
                                                            &[libc::c_char; 33]>(b"void resume(lua_State *, void *)\x00")).as_ptr());
                };
                firstArg = (*L).top.offset(-(n as isize))
            }
            luaD_poscall(L, ci, firstArg, n);
        }
        unroll(L, 0 as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_precall(mut L: *mut lua_State_0,
                                      mut func: StkId,
                                      mut nresults: libc::c_int)
 -> libc::c_int {
    let mut f: lua_CFunction = None;
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    match (*func).tt_ & 63i32 {
        38 => {
            if (*func).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((func))->tt_) == ((((6 | (2 << 4))) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 419i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
            };
            if (*(*func).value_.gc).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(b"(((func)->value_).gc)->tt == (6 | (2 << 4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 419i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
            };
            f =
                (*(&mut (*((*func).value_.gc as *mut GCUnion)).cl.c as
                       *mut CClosure)).f
        }
        22 => {
            if (*func).tt_ == 6i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(b"((((func))->tt_) == ((6 | (1 << 4))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 422i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
            };
            f = (*func).value_.f
        }
        6 => {
            let mut base: StkId = 0 as *mut TValue;
            if (*func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 443i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
            };
            if (*(*func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"(((func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 443i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
            };
            let mut p: *mut Proto_0 =
                (*(&mut (*((*func).value_.gc as *mut GCUnion)).cl.l as
                       *mut LClosure_0)).p;
            let mut n_0: libc::c_int =
                func.offset_to((*L).top).expect("bad offset_to") as
                    libc::c_long as libc::c_int - 1i32;
            let mut fsize: libc::c_int = (*p).maxstacksize as libc::c_int;
            if (*L).top.offset_to((*L).stack_last).expect("bad offset_to") as
                   libc::c_long <= fsize as libc::c_long {
                let mut t___0: ptrdiff_t =
                    ((*L).stack as
                         *mut libc::c_char).offset_to(func as
                                                          *mut libc::c_char).expect("bad offset_to")
                        as libc::c_long;
                if (*(*L).l_G).GCdebt > 0i32 as libc::c_long { luaC_step(L); }
                luaD_growstack(L, fsize);
                func =
                    ((*L).stack as *mut libc::c_char).offset(t___0 as isize)
                        as *mut TValue
            }
            if 0 != (*p).is_vararg {
                base = adjust_varargs(L, p, n_0)
            } else {
                while n_0 < (*p).numparams as libc::c_int {
                    let fresh20 = (*L).top;
                    (*L).top = (*L).top.offset(1);
                    (*fresh20).tt_ = 0i32;
                    n_0 += 1
                }
                base = func.offset(1isize)
            }
            (*L).ci =
                if !(*(*L).ci).next.is_null() {
                    (*(*L).ci).next
                } else { luaE_extendCI(L) };
            ci = (*L).ci;
            (*ci).nresults = nresults as libc::c_short;
            (*ci).func = func;
            (*ci).u.l.base = base;
            (*ci).top = base.offset(fsize as isize);
            (*L).top = (*ci).top;
            if (*ci).top <= (*L).stack_last {
            } else {
                __assert_fail(b"ci->top <= L->stack_last\x00" as *const u8 as
                                  *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 459i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
            };
            (*ci).u.l.savedpc = (*p).code;
            (*ci).callstatus = (1i32 << 1i32) as libc::c_ushort;
            if 0 != (*L).hookmask & 1i32 << 0i32 { callhook(L, ci); }
            return 0i32
        }
        _ => {
            if (*L).top.offset_to((*L).stack_last).expect("bad offset_to") as
                   libc::c_long <= 1i32 as libc::c_long {
                let mut t___1: ptrdiff_t =
                    ((*L).stack as
                         *mut libc::c_char).offset_to(func as
                                                          *mut libc::c_char).expect("bad offset_to")
                        as libc::c_long;
                if (*(*L).l_G).GCdebt > 0i32 as libc::c_long { luaC_step(L); }
                luaD_growstack(L, 1i32);
                func =
                    ((*L).stack as *mut libc::c_char).offset(t___1 as isize)
                        as *mut TValue
            }
            tryfuncTM(L, func);
            return luaD_precall(L, func, nresults)
        }
    }
    let mut n: libc::c_int = 0;
    if (*L).top.offset_to((*L).stack_last).expect("bad offset_to") as
           libc::c_long <= 20i32 as libc::c_long {
        let mut t__: ptrdiff_t =
            ((*L).stack as
                 *mut libc::c_char).offset_to(func as
                                                  *mut libc::c_char).expect("bad offset_to")
                as libc::c_long;
        if (*(*L).l_G).GCdebt > 0i32 as libc::c_long { luaC_step(L); }
        luaD_growstack(L, 20i32);
        func =
            ((*L).stack as *mut libc::c_char).offset(t__ as isize) as
                *mut TValue
    }
    (*L).ci =
        if !(*(*L).ci).next.is_null() {
            (*(*L).ci).next
        } else { luaE_extendCI(L) };
    ci = (*L).ci;
    (*ci).nresults = nresults as libc::c_short;
    (*ci).func = func;
    (*ci).top = (*L).top.offset(20isize);
    if (*ci).top <= (*L).stack_last {
    } else {
        __assert_fail(b"ci->top <= L->stack_last\x00" as *const u8 as
                          *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      430i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
    };
    (*ci).callstatus = 0i32 as libc::c_ushort;
    if 0 != (*L).hookmask & 1i32 << 0i32 { luaD_hook(L, 0i32, -1i32); }
    let ref mut fresh17 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
    *fresh17 -= 1;
    if *fresh17 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      434i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
    };
    n = f.expect("non-null function pointer")(L);
    let ref mut fresh18 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
    let fresh19 = *fresh18;
    *fresh18 = *fresh18 + 1;
    if fresh19 == 0i32 {
    } else {
        __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      436i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
    };
    if (n as libc::c_long) <
           (*(*L).ci).func.offset_to((*L).top).expect("bad offset_to") as
               libc::c_long &&
           !(b"not enough elements in the stack\x00" as *const u8 as
                 *const libc::c_char).is_null() {
    } else {
        __assert_fail(b"((n) < (L->top - L->ci->func)) && \"not enough elements in the stack\"\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      437i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"int luaD_precall(lua_State *, StkId, int)\x00")).as_ptr());
    };
    luaD_poscall(L, ci, (*L).top.offset(-(n as isize)), n);
    return 1i32;
}
unsafe extern "C" fn tryfuncTM(mut L: *mut lua_State_0, mut func: StkId)
 -> () {
    let mut tm: *const TValue =
        luaT_gettmbyobj(L, func as *const TValue, TM_CALL);
    let mut p: StkId = 0 as *mut TValue;
    if !((*tm).tt_ & 15i32 == 6i32) {
        luaG_typeerror(L, func as *const TValue,
                       b"call\x00" as *const u8 as *const libc::c_char);
    } else {
        p = (*L).top;
        while p > func {
            let mut io1: *mut TValue = p;
            *io1 = *p.offset(-1isize);
            if 0 == (*io1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ldo.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         323i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 35],
                                                                   &[libc::c_char; 35]>(b"void tryfuncTM(lua_State *, StkId)\x00")).as_ptr());
                       };
                       (*io1).tt_ & 63i32 ==
                           (*(*io1).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ldo.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      323i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 35],
                                                                                &[libc::c_char; 35]>(b"void tryfuncTM(lua_State *, StkId)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io1).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldo.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  323i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 35],
                                                            &[libc::c_char; 35]>(b"void tryfuncTM(lua_State *, StkId)\x00")).as_ptr());
                };
            };
            p = p.offset(-1isize)
        }
        (*L).top = (*L).top.offset(1isize);
        let mut io1_0: *mut TValue = func;
        *io1_0 = *tm;
        if 0 == (*io1_0).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ldo.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     325i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 35],
                                                               &[libc::c_char; 35]>(b"void tryfuncTM(lua_State *, StkId)\x00")).as_ptr());
                   };
                   (*io1_0).tt_ & 63i32 ==
                       (*(*io1_0).value_.gc).tt as libc::c_int &&
                       (L.is_null() ||
                            {
                                if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldo.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  325i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 35],
                                                                            &[libc::c_char; 35]>(b"void tryfuncTM(lua_State *, StkId)\x00")).as_ptr());
                                };
                                0 !=
                                    ((*(*io1_0).value_.gc).marked as
                                         libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32)) &
                                        ((*(*L).l_G).currentwhite as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32))
                            })
               } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 325i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void tryfuncTM(lua_State *, StkId)\x00")).as_ptr());
            };
        };
        return;
    };
}
unsafe extern "C" fn callhook(mut L: *mut lua_State_0,
                              mut ci: *mut CallInfo_0) -> () {
    let mut hook: libc::c_int = 0i32;
    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize);
    if 0 != (*(*ci).previous).callstatus as libc::c_int & 1i32 << 1i32 &&
           (*(*(*ci).previous).u.l.savedpc.offset(-1isize) >> 0i32 &
                !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as
               libc::c_uint == OP_TAILCALL as libc::c_int as libc::c_uint {
        (*ci).callstatus =
            ((*ci).callstatus as libc::c_int | 1i32 << 5i32) as
                libc::c_ushort;
        hook = 4i32
    }
    luaD_hook(L, hook, -1i32);
    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(-1isize);
}
unsafe extern "C" fn adjust_varargs(mut L: *mut lua_State_0,
                                    mut p: *mut Proto_0,
                                    mut actual: libc::c_int) -> StkId {
    let mut i: libc::c_int = 0;
    let mut nfixargs: libc::c_int = (*p).numparams as libc::c_int;
    let mut base: StkId = 0 as *mut TValue;
    let mut fixed: StkId = 0 as *mut TValue;
    fixed = (*L).top.offset(-(actual as isize));
    base = (*L).top;
    i = 0i32;
    while i < nfixargs && i < actual {
        let fresh21 = (*L).top;
        (*L).top = (*L).top.offset(1);
        let mut io1: *mut TValue = fresh21;
        *io1 = *fixed.offset(i as isize);
        if 0 == (*io1).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io1).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ldo.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     302i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 48],
                                                               &[libc::c_char; 48]>(b"StkId adjust_varargs(lua_State *, Proto *, int)\x00")).as_ptr());
                   };
                   (*io1).tt_ & 63i32 == (*(*io1).value_.gc).tt as libc::c_int
                       &&
                       (L.is_null() ||
                            {
                                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldo.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  302i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 48],
                                                                            &[libc::c_char; 48]>(b"StkId adjust_varargs(lua_State *, Proto *, int)\x00")).as_ptr());
                                };
                                0 !=
                                    ((*(*io1).value_.gc).marked as libc::c_int
                                         ^ (1i32 << 0i32 | 1i32 << 1i32)) &
                                        ((*(*L).l_G).currentwhite as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32))
                            })
               } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"ldo.c\x00" as *const u8 as
                                  *const libc::c_char, 302i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 48],
                                                        &[libc::c_char; 48]>(b"StkId adjust_varargs(lua_State *, Proto *, int)\x00")).as_ptr());
            };
        };
        (*fixed.offset(i as isize)).tt_ = 0i32;
        i += 1
    }
    while i < nfixargs {
        let fresh22 = (*L).top;
        (*L).top = (*L).top.offset(1);
        (*fresh22).tt_ = 0i32;
        i += 1
    }
    return base;
}
unsafe extern "C" fn resume_error(mut L: *mut lua_State_0,
                                  mut msg: *const libc::c_char,
                                  mut narg: libc::c_int) -> libc::c_int {
    (*L).top = (*L).top.offset(-(narg as isize));
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = luaS_new(L, msg);
    if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      606i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"int resume_error(lua_State *, const char *, int)\x00")).as_ptr());
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
    (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"ldo.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 606i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 49],
                                                           &[libc::c_char; 49]>(b"int resume_error(lua_State *, const char *, int)\x00")).as_ptr());
               };
               (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int &&
                   (L.is_null() ||
                        {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"ldo.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              606i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 49],
                                                                        &[libc::c_char; 49]>(b"int resume_error(lua_State *, const char *, int)\x00")).as_ptr());
                            };
                            0 !=
                                ((*(*io).value_.gc).marked as libc::c_int ^
                                     (1i32 << 0i32 | 1i32 << 1i32)) &
                                    ((*(*L).l_G).currentwhite as libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32))
                        })
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ldo.c\x00" as *const u8 as *const libc::c_char,
                          606i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 49],
                                                    &[libc::c_char; 49]>(b"int resume_error(lua_State *, const char *, int)\x00")).as_ptr());
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top &&
           !(b"stack overflow\x00" as *const u8 as
                 *const libc::c_char).is_null() {
    } else {
        __assert_fail(b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as
                          *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      607i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"int resume_error(lua_State *, const char *, int)\x00")).as_ptr());
    };
    let ref mut fresh23 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
    *fresh23 -= 1;
    if *fresh23 == 0i32 {
    } else {
        __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      608i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"int resume_error(lua_State *, const char *, int)\x00")).as_ptr());
    };
    return 2i32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isyieldable(mut L: *mut lua_State_0)
 -> libc::c_int {
    return ((*L).nny as libc::c_int == 0i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_protectedparser(mut L: *mut lua_State_0,
                                              mut z: *mut ZIO,
                                              mut name: *const libc::c_char,
                                              mut mode: *const libc::c_char)
 -> libc::c_int {
    let mut p: SParser =
        SParser{z: 0 as *mut ZIO,
                buff:
                    Mbuffer{buffer: 0 as *mut libc::c_char,
                            n: 0,
                            buffsize: 0,},
                dyd:
                    Dyndata{actvar:
                                unnamed_6{arr: 0 as *mut Vardesc_0,
                                          n: 0,
                                          size: 0,},
                            gt:
                                Labellist{arr: 0 as *mut Labeldesc_0,
                                          n: 0,
                                          size: 0,},
                            label:
                                Labellist{arr: 0 as *mut Labeldesc_0,
                                          n: 0,
                                          size: 0,},},
                mode: 0 as *const libc::c_char,
                name: 0 as *const libc::c_char,};
    let mut status: libc::c_int = 0;
    (*L).nny = (*L).nny.wrapping_add(1);
    p.z = z;
    p.name = name;
    p.mode = mode;
    p.dyd.actvar.arr = 0 as *mut Vardesc_0;
    p.dyd.actvar.size = 0i32;
    p.dyd.gt.arr = 0 as *mut Labeldesc_0;
    p.dyd.gt.size = 0i32;
    p.dyd.label.arr = 0 as *mut Labeldesc_0;
    p.dyd.label.size = 0i32;
    p.buff.buffer = 0 as *mut libc::c_char;
    p.buff.buffsize = 0i32 as size_t;
    status =
        luaD_pcall(L, Some(f_parser),
                   &mut p as *mut SParser as *mut libc::c_void,
                   ((*L).stack as
                        *mut libc::c_char).offset_to((*L).top as
                                                         *mut libc::c_char).expect("bad offset_to")
                       as libc::c_long, (*L).errfunc);
    p.buff.buffer =
        luaM_realloc_(L, p.buff.buffer as *mut libc::c_void,
                      p.buff.buffsize.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                       as libc::c_ulong),
                      (0i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
            as *mut libc::c_char;
    p.buff.buffsize = 0i32 as size_t;
    luaM_realloc_(L, p.dyd.actvar.arr as *mut libc::c_void,
                  (p.dyd.actvar.size as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<Vardesc_0>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, p.dyd.gt.arr as *mut libc::c_void,
                  (p.dyd.gt.size as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<Labeldesc_0>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, p.dyd.label.arr as *mut libc::c_void,
                  (p.dyd.label.size as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<Labeldesc_0>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    (*L).nny = (*L).nny.wrapping_sub(1);
    return status;
}
unsafe extern "C" fn f_parser(mut L: *mut lua_State_0,
                              mut ud: *mut libc::c_void) -> () {
    let mut cl: *mut LClosure_0 = 0 as *mut LClosure_0;
    let mut p: *mut SParser = ud as *mut SParser;
    let fresh24 = (*(*p).z).n;
    (*(*p).z).n = (*(*p).z).n.wrapping_sub(1);
    let mut c: libc::c_int =
        if fresh24 > 0i32 as libc::c_ulong {
            let fresh25 = (*(*p).z).p;
            (*(*p).z).p = (*(*p).z).p.offset(1);
            *fresh25 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*p).z) };
    if c ==
           (*::std::mem::transmute::<&[u8; 5],
                                     &[libc::c_char; 5]>(b"\x1bLua\x00"))[0usize]
               as libc::c_int {
        checkmode(L, (*p).mode,
                  b"binary\x00" as *const u8 as *const libc::c_char);
        cl = luaU_undump(L, (*p).z, (*p).name)
    } else {
        checkmode(L, (*p).mode,
                  b"text\x00" as *const u8 as *const libc::c_char);
        cl =
            luaY_parser(L, (*p).z, &mut (*p).buff, &mut (*p).dyd, (*p).name,
                        c)
    }
    if (*cl).nupvalues as libc::c_int == (*(*cl).p).sizeupvalues {
    } else {
        __assert_fail(b"cl->nupvalues == cl->p->sizeupvalues\x00" as *const u8
                          as *const libc::c_char,
                      b"ldo.c\x00" as *const u8 as *const libc::c_char,
                      779i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void f_parser(lua_State *, void *)\x00")).as_ptr());
    };
    luaF_initupvals(L, cl);
}
unsafe extern "C" fn checkmode(mut L: *mut lua_State_0,
                               mut mode: *const libc::c_char,
                               mut x: *const libc::c_char) -> () {
    if !mode.is_null() &&
           strchr(mode, *x.offset(0isize) as libc::c_int).is_null() {
        luaO_pushfstring(L,
                         b"attempt to load a %s chunk (mode is \'%s\')\x00" as
                             *const u8 as *const libc::c_char, x, mode);
        luaD_throw(L, 3i32);
    } else { return; };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_pcall(mut L: *mut lua_State_0, mut func: Pfunc,
                                    mut u: *mut libc::c_void,
                                    mut old_top: ptrdiff_t, mut ef: ptrdiff_t)
 -> libc::c_int {
    let mut oldtop: StkId = 0 as *mut TValue;
    let mut status: libc::c_int = 0;
    let mut old_ci: *mut CallInfo_0 = (*L).ci;
    let mut old_allowhooks: lu_byte = (*L).allowhook;
    let mut old_nny: libc::c_ushort = (*L).nny;
    let mut old_errfunc: ptrdiff_t = (*L).errfunc;
    (*L).errfunc = ef;
    status = luaD_rawrunprotected(L, func, u);
    if status != 0i32 {
        oldtop =
            ((*L).stack as *mut libc::c_char).offset(old_top as isize) as
                *mut TValue;
        luaF_close(L, oldtop);
        seterrorobj(L, status, oldtop);
        (*L).ci = old_ci;
        (*L).allowhook = old_allowhooks;
        (*L).nny = old_nny;
        luaD_shrinkstack(L);
    }
    (*L).errfunc = old_errfunc;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_call(mut L: *mut lua_State_0, mut func: StkId,
                                   mut nResults: libc::c_int) -> () {
    (*L).nCcalls = (*L).nCcalls.wrapping_add(1);
    if (*L).nCcalls as libc::c_int >= 200i32 { stackerror(L); }
    if 0 == luaD_precall(L, func, nResults) { luaV_execute(L); }
    (*L).nCcalls = (*L).nCcalls.wrapping_sub(1);
}
unsafe extern "C" fn stackerror(mut L: *mut lua_State_0) -> () {
    if (*L).nCcalls as libc::c_int == 200i32 {
        luaG_runerror(L,
                      b"C stack overflow\x00" as *const u8 as
                          *const libc::c_char);
    } else if (*L).nCcalls as libc::c_int >= 200i32 + (200i32 >> 3i32) {
        luaD_throw(L, 6i32);
    } else { return; };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_callnoyield(mut L: *mut lua_State_0,
                                          mut func: StkId,
                                          mut nResults: libc::c_int) -> () {
    (*L).nny = (*L).nny.wrapping_add(1);
    luaD_call(L, func, nResults);
    (*L).nny = (*L).nny.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_inctop(mut L: *mut lua_State_0) -> () {
    if (*L).top.offset_to((*L).stack_last).expect("bad offset_to") as
           libc::c_long <= 1i32 as libc::c_long {
        luaD_growstack(L, 1i32);
    }
    (*L).top = (*L).top.offset(1isize);
}
