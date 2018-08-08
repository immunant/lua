#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type BlockCnt;
    pub type lua_longjmp;
    pub type UpVal_0;
    #[no_mangle]
    static mut signgam: libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaO_arith(L: *mut lua_State_0, op: libc::c_int, p1: *const TValue,
                  p2: *const TValue, res: *mut TValue) -> ();
    #[no_mangle]
    fn luaM_growaux_(L: *mut lua_State_0, block: *mut libc::c_void,
                     size: *mut libc::c_int, size_elem: size_t,
                     limit: libc::c_int, what: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn luaX_syntaxerror(ls: *mut LexState, s: *const libc::c_char) -> !;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
    #[no_mangle]
    fn luaC_barrier_(L: *mut lua_State_0, o: *mut GCObject, v: *mut GCObject)
     -> ();
    #[no_mangle]
    fn luaH_set(L: *mut lua_State_0, t: *mut Table, key: *const TValue)
     -> *mut TValue;
    #[no_mangle]
    fn luaV_equalobj(L: *mut lua_State_0, t1: *const TValue,
                     t2: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer,
                      mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type __sig_atomic_t = libc::c_int;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
pub const OP_IDIV: OpCode = 19;
pub const OP_CLOSURE: OpCode = 44;
pub const iABx: OpMode = 1;
pub const OPR_EQ: BinOpr_0 = 13;
pub const OP_SUB: OpCode = 14;
pub type Mbuffer = Mbuffer_0;
pub type BinOpr = BinOpr_0;
pub const VLOCAL: expkind = 8;
pub const VTRUE: expkind = 2;
pub const OPR_NE: BinOpr_0 = 16;
pub const VUPVAL: expkind = 9;
pub type expdesc = expdesc_0;
pub const VKFLT: expkind = 5;
pub const OP_MOVE: OpCode = 0;
pub const VNIL: expkind = 1;
pub const OPR_BNOT: UnOpr = 1;
pub const OP_LE: OpCode = 33;
pub const OP_LOADNIL: OpCode = 4;
pub type FuncState = FuncState_0;
pub const OPR_OR: BinOpr_0 = 20;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata,
    cl: Closure,
    h: Table_0,
    p: Proto_0,
    th: lua_State,
}
pub const VJMP: expkind = 11;
pub const iAx: OpMode = 3;
pub const VK: expkind = 4;
pub type UnOpr = libc::c_uint;
pub type BinOpr_0 = libc::c_uint;
pub type LexState = LexState_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: lu_byte,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed {
    ival: lua_Integer,
    nval: lua_Number,
    info: libc::c_int,
    ind: unnamed_1,
}
pub const VNONRELOC: expkind = 7;
pub const OP_LT: OpCode = 32;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union SemInfo {
    r: lua_Number,
    i: lua_Integer,
    ts: *mut TString,
}
pub const OP_SETTABLE: OpCode = 10;
pub const OP_CALL: OpCode = 36;
pub const VINDEXED: expkind = 10;
pub type Table = Table_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_0 {
    pub arr: *mut Vardesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
pub const OPR_LT: BinOpr_0 = 14;
pub type Proto = Proto_0;
pub type Labeldesc_0 = Labeldesc;
pub const OP_CONCAT: OpCode = 29;
pub const OP_LOADK: OpCode = 1;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LexState_0 {
    pub current: libc::c_int,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
    pub t: Token_0,
    pub lookahead: Token_0,
    pub fs: *mut FuncState_0,
    pub L: *mut lua_State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer,
    pub h: *mut Table,
    pub dyd: *mut Dyndata,
    pub source: *mut TString,
    pub envn: *mut TString,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct expdesc_0 {
    pub k: expkind,
    pub u: unnamed,
    pub t: libc::c_int,
    pub f: libc::c_int,
}
pub const OP_GETUPVAL: OpCode = 5;
pub type OpMode = libc::c_uint;
pub const OpArgN: OpArgMask = 0;
pub const VRELOCABLE: expkind = 12;
pub const OP_LEN: OpCode = 28;
pub const OPR_GT: BinOpr_0 = 17;
pub const OP_FORLOOP: OpCode = 39;
pub const OP_VARARG: OpCode = 45;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State_0,
}
pub const OP_TAILCALL: OpCode = 37;
pub const VVARARG: expkind = 14;
pub const OP_SETLIST: OpCode = 43;
pub const iAsBx: OpMode = 2;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FuncState_0 {
    pub f: *mut Proto,
    pub prev: *mut FuncState_0,
    pub ls: *mut LexState_0,
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
pub const OPR_DIV: BinOpr_0 = 5;
pub const OP_SETTABUP: OpCode = 8;
pub const OP_EQ: OpCode = 31;
pub const OP_JMP: OpCode = 30;
pub const OP_LOADKX: OpCode = 2;
pub const OPR_CONCAT: BinOpr_0 = 12;
pub const OP_GETTABLE: OpCode = 7;
pub const OP_MUL: OpCode = 15;
pub const OPR_SHL: BinOpr_0 = 10;
pub const VFALSE: expkind = 3;
pub type Vardesc = Vardesc_0;
pub const OP_SELF: OpCode = 12;
pub const OP_TESTSET: OpCode = 35;
pub const OP_GETTABUP: OpCode = 6;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Mbuffer_0 {
    pub buffer: *mut libc::c_char,
    pub n: size_t,
    pub buffsize: size_t,
}
pub const OPR_AND: BinOpr_0 = 19;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Token {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
pub const OP_NOT: OpCode = 27;
pub const OPR_ADD: BinOpr_0 = 0;
pub const OPR_BAND: BinOpr_0 = 7;
pub type expkind = libc::c_uint;
pub const OP_TEST: OpCode = 34;
pub const OP_SETUPVAL: OpCode = 9;
pub const OP_TFORCALL: OpCode = 41;
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure_0,
    l: LClosure,
}
pub const OPR_LEN: UnOpr = 3;
pub const OPR_GE: BinOpr_0 = 18;
pub const OP_BOR: OpCode = 21;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Vardesc_0 {
    pub idx: libc::c_short,
}
pub const OPR_LE: BinOpr_0 = 15;
pub const OP_EXTRAARG: OpCode = 46;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_1 {
    pub idx: libc::c_short,
    pub t: lu_byte,
    pub vt: lu_byte,
}
pub const OP_LOADBOOL: OpCode = 3;
pub type UnOpr_0 = UnOpr;
pub const iABC: OpMode = 0;
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
    pub u: unnamed_2,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
    l: unnamed_4,
    c: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_3 {
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
pub struct unnamed_4 {
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
pub type UpVal = UpVal_0;
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
    pub mt: [*mut Table_0; 9],
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
    pub u: unnamed_5,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    lnglen: size_t,
    hnext: *mut TString_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Table_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,
    pub lsizenode: lu_byte,
    pub sizearray: libc::c_uint,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table_0,
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
    nk: unnamed_6,
    tvk: TValue,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_6 {
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
pub type lu_mem = size_t;
pub type l_mem = ptrdiff_t;
pub type lua_Alloc =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: size_t, _: size_t) -> *mut libc::c_void>;
pub type lua_Reader =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void,
                                _: *mut size_t) -> *const libc::c_char>;
pub type LClosure = LClosure_0;
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
pub const OPR_SUB: BinOpr_0 = 1;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Dyndata {
    pub actvar: unnamed_0,
    pub gt: Labellist,
    pub label: Labellist,
}
pub const OPR_MUL: BinOpr_0 = 2;
pub type Labellist = Labellist_0;
pub const OpArgR: OpArgMask = 2;
pub const OpArgU: OpArgMask = 1;
pub const OP_RETURN: OpCode = 38;
pub const OP_SHL: OpCode = 23;
pub const OP_MOD: OpCode = 16;
pub const OpArgK: OpArgMask = 3;
pub const OP_BNOT: OpCode = 26;
pub const OP_TFORLOOP: OpCode = 42;
pub type Token_0 = Token;
pub const OPR_BXOR: BinOpr_0 = 9;
pub const OP_POW: OpCode = 17;
pub const OPR_NOUNOPR: UnOpr = 4;
pub const OP_ADD: OpCode = 13;
pub const OP_BAND: OpCode = 20;
pub const OP_BXOR: OpCode = 22;
pub const OPR_BOR: BinOpr_0 = 8;
pub const OP_DIV: OpCode = 18;
pub const OPR_IDIV: BinOpr_0 = 6;
pub type ZIO = Zio;
pub const OP_NEWTABLE: OpCode = 11;
pub const VKINT: expkind = 6;
pub const OPR_POW: BinOpr_0 = 4;
pub const OPR_SHR: BinOpr_0 = 11;
pub const OP_FORPREP: OpCode = 40;
pub type CClosure_0 = CClosure;
pub type OpArgMask = libc::c_uint;
pub const OPR_MINUS: UnOpr = 0;
pub const VVOID: expkind = 0;
pub const OP_UNM: OpCode = 25;
pub const OP_SHR: OpCode = 24;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Labellist_0 {
    pub arr: *mut Labeldesc_0,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
pub const OPR_NOT: UnOpr = 2;
pub const VCALL: expkind = 13;
pub const OPR_MOD: BinOpr_0 = 3;
pub type OpCode = libc::c_uint;
pub const OPR_NOBINOPR: BinOpr_0 = 21;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table_0,
    pub len: size_t,
    pub user_: Value_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
pub type Upvaldesc_0 = Upvaldesc;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type LocVar_0 = LocVar;
#[derive ( Copy , Clone )]
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto_0,
    pub upvals: [*mut UpVal; 1],
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codeABx(mut fs: *mut FuncState, mut o: OpCode,
                                      mut a: libc::c_int,
                                      mut bc: libc::c_uint) -> libc::c_int {
    if (luaP_opmodes[o as usize] as libc::c_int & 3i32) as OpMode as
           libc::c_uint == iABx as libc::c_int as libc::c_uint ||
           (luaP_opmodes[o as usize] as libc::c_int & 3i32) as OpMode as
               libc::c_uint == iAsBx as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"(((enum OpMode)(luaP_opmodes[o] & 3))) == iABx || (((enum OpMode)(luaP_opmodes[o] & 3))) == iAsBx\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      325i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"int luaK_codeABx(FuncState *, OpCode, int, unsigned int)\x00")).as_ptr());
    };
    if (luaP_opmodes[o as usize] as libc::c_int >> 2i32 & 3i32) as OpArgMask
           as libc::c_uint == OpArgN as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[o] >> 2) & 3))) == OpArgN\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      326i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"int luaK_codeABx(FuncState *, OpCode, int, unsigned int)\x00")).as_ptr());
    };
    if a <= (1i32 << 8i32) - 1i32 &&
           bc <= ((1i32 << 9i32 + 9i32) - 1i32) as libc::c_uint {
    } else {
        __assert_fail(b"a <= ((1<<8)-1) && bc <= ((1<<(9 + 9))-1)\x00" as
                          *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      327i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"int luaK_codeABx(FuncState *, OpCode, int, unsigned int)\x00")).as_ptr());
    };
    return luaK_code(fs,
                     (o as Instruction) << 0i32 |
                         (a as Instruction) << 0i32 + 6i32 |
                         bc << 0i32 + 6i32 + 8i32);
}
unsafe extern "C" fn luaK_code(mut fs: *mut FuncState, mut i: Instruction)
 -> libc::c_int {
    let mut f: *mut Proto = (*fs).f;
    dischargejpc(fs);
    if (*fs).pc + 1i32 > (*f).sizecode {
        (*f).code =
            luaM_growaux_((*(*fs).ls).L, (*f).code as *mut libc::c_void,
                          &mut (*f).sizecode,
                          ::std::mem::size_of::<Instruction>() as
                              libc::c_ulong, 2147483647i32,
                          b"opcodes\x00" as *const u8 as *const libc::c_char)
                as *mut Instruction
    }
    *(*f).code.offset((*fs).pc as isize) = i;
    if (*fs).pc + 1i32 > (*f).sizelineinfo {
        (*f).lineinfo =
            luaM_growaux_((*(*fs).ls).L, (*f).lineinfo as *mut libc::c_void,
                          &mut (*f).sizelineinfo,
                          ::std::mem::size_of::<libc::c_int>() as
                              libc::c_ulong, 2147483647i32,
                          b"opcodes\x00" as *const u8 as *const libc::c_char)
                as *mut libc::c_int
    }
    *(*f).lineinfo.offset((*fs).pc as isize) = (*(*fs).ls).lastline;
    let fresh0 = (*fs).pc;
    (*fs).pc = (*fs).pc + 1;
    return fresh0;
}
unsafe extern "C" fn dischargejpc(mut fs: *mut FuncState) -> () {
    patchlistaux(fs, (*fs).jpc, (*fs).pc, (1i32 << 8i32) - 1i32, (*fs).pc);
    (*fs).jpc = -1i32;
}
unsafe extern "C" fn patchlistaux(mut fs: *mut FuncState,
                                  mut list: libc::c_int,
                                  mut vtarget: libc::c_int,
                                  mut reg: libc::c_int,
                                  mut dtarget: libc::c_int) -> () {
    while list != -1i32 {
        let mut next: libc::c_int = getjump(fs, list);
        if 0 != patchtestreg(fs, list, reg) {
            fixjump(fs, list, vtarget);
        } else { fixjump(fs, list, dtarget); }
        list = next
    };
}
unsafe extern "C" fn getjump(mut fs: *mut FuncState, mut pc: libc::c_int)
 -> libc::c_int {
    let mut offset: libc::c_int =
        (*(*(*fs).f).code.offset(pc as isize) >> 0i32 + 6i32 + 8i32 &
             !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32) as
            libc::c_int - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32);
    if offset == -1i32 { return -1i32 } else { return pc + 1i32 + offset };
}
unsafe extern "C" fn fixjump(mut fs: *mut FuncState, mut pc: libc::c_int,
                             mut dest: libc::c_int) -> () {
    let mut jmp: *mut Instruction =
        &mut *(*(*fs).f).code.offset(pc as isize) as *mut Instruction;
    let mut offset: libc::c_int = dest - (pc + 1i32);
    if dest != -1i32 {
    } else {
        __assert_fail(b"dest != (-1)\x00" as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      106i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void fixjump(FuncState *, int, int)\x00")).as_ptr());
    };
    if abs(offset) > (1i32 << 9i32 + 9i32) - 1i32 >> 1i32 {
        luaX_syntaxerror((*fs).ls,
                         b"control structure too long\x00" as *const u8 as
                             *const libc::c_char);
    } else {
        *jmp =
            *jmp &
                !(!((!(0i32 as Instruction)) << 9i32 + 9i32) <<
                      0i32 + 6i32 + 8i32) |
                ((offset + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as
                     libc::c_uint) << 0i32 + 6i32 + 8i32 &
                    !((!(0i32 as Instruction)) << 9i32 + 9i32) <<
                        0i32 + 6i32 + 8i32;
        return;
    };
}
unsafe extern "C" fn patchtestreg(mut fs: *mut FuncState,
                                  mut node: libc::c_int, mut reg: libc::c_int)
 -> libc::c_int {
    let mut i: *mut Instruction = getjumpcontrol(fs, node);
    if (*i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as
           libc::c_uint != OP_TESTSET as libc::c_int as libc::c_uint {
        return 0i32
    } else {
        if reg != (1i32 << 8i32) - 1i32 &&
               reg !=
                   (*i >> 0i32 + 6i32 + 8i32 + 9i32 &
                        !((!(0i32 as Instruction)) << 9i32) << 0i32) as
                       libc::c_int {
            *i =
                *i & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32) |
                    (reg as Instruction) << 0i32 + 6i32 &
                        !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32
        } else {
            *i =
                (OP_TEST as libc::c_int as Instruction) << 0i32 |
                    ((*i >> 0i32 + 6i32 + 8i32 + 9i32 &
                          !((!(0i32 as Instruction)) << 9i32) << 0i32) as
                         libc::c_int as Instruction) << 0i32 + 6i32 |
                    (0i32 as Instruction) << 0i32 + 6i32 + 8i32 + 9i32 |
                    ((*i >> 0i32 + 6i32 + 8i32 &
                          !((!(0i32 as Instruction)) << 9i32) << 0i32) as
                         libc::c_int as Instruction) << 0i32 + 6i32 + 8i32
        }
        return 1i32
    };
}
unsafe extern "C" fn getjumpcontrol(mut fs: *mut FuncState,
                                    mut pc: libc::c_int) -> *mut Instruction {
    let mut pi: *mut Instruction =
        &mut *(*(*fs).f).code.offset(pc as isize) as *mut Instruction;
    if pc >= 1i32 &&
           0 !=
               luaP_opmodes[(*pi.offset(-1isize) >> 0i32 &
                                 !((!(0i32 as Instruction)) << 6i32) << 0i32)
                                as OpCode as usize] as libc::c_int &
                   1i32 << 7i32 {
        return pi.offset(-1isize)
    } else { return pi };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codeABC(mut fs: *mut FuncState, mut o: OpCode,
                                      mut a: libc::c_int, mut b: libc::c_int,
                                      mut c: libc::c_int) -> libc::c_int {
    if (luaP_opmodes[o as usize] as libc::c_int & 3i32) as OpMode as
           libc::c_uint == iABC as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"(((enum OpMode)(luaP_opmodes[o] & 3))) == iABC\x00" as
                          *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      313i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"int luaK_codeABC(FuncState *, OpCode, int, int, int)\x00")).as_ptr());
    };
    if (luaP_opmodes[o as usize] as libc::c_int >> 4i32 & 3i32) as OpArgMask
           as libc::c_uint != OpArgN as libc::c_int as libc::c_uint ||
           b == 0i32 {
    } else {
        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[o] >> 4) & 3))) != OpArgN || b == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      314i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"int luaK_codeABC(FuncState *, OpCode, int, int, int)\x00")).as_ptr());
    };
    if (luaP_opmodes[o as usize] as libc::c_int >> 2i32 & 3i32) as OpArgMask
           as libc::c_uint != OpArgN as libc::c_int as libc::c_uint ||
           c == 0i32 {
    } else {
        __assert_fail(b"(((enum OpArgMask)((luaP_opmodes[o] >> 2) & 3))) != OpArgN || c == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      315i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"int luaK_codeABC(FuncState *, OpCode, int, int, int)\x00")).as_ptr());
    };
    if a <= (1i32 << 8i32) - 1i32 && b <= (1i32 << 9i32) - 1i32 &&
           c <= (1i32 << 9i32) - 1i32 {
    } else {
        __assert_fail(b"a <= ((1<<8)-1) && b <= ((1<<9)-1) && c <= ((1<<9)-1)\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      316i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"int luaK_codeABC(FuncState *, OpCode, int, int, int)\x00")).as_ptr());
    };
    return luaK_code(fs,
                     (o as Instruction) << 0i32 |
                         (a as Instruction) << 0i32 + 6i32 |
                         (b as Instruction) << 0i32 + 6i32 + 8i32 + 9i32 |
                         (c as Instruction) << 0i32 + 6i32 + 8i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_codek(mut fs: *mut FuncState,
                                    mut reg: libc::c_int, mut k: libc::c_int)
 -> libc::c_int {
    if k <= (1i32 << 9i32 + 9i32) - 1i32 {
        return luaK_codeABx(fs, OP_LOADK, reg, k as libc::c_uint)
    } else {
        let mut p: libc::c_int =
            luaK_codeABx(fs, OP_LOADKX, reg, 0i32 as libc::c_uint);
        codeextraarg(fs, k);
        return p
    };
}
unsafe extern "C" fn codeextraarg(mut fs: *mut FuncState, mut a: libc::c_int)
 -> libc::c_int {
    if a <= (1i32 << 9i32 + 9i32 + 8i32) - 1i32 {
    } else {
        __assert_fail(b"a <= ((1<<(9 + 9 + 8))-1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      336i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"int codeextraarg(FuncState *, int)\x00")).as_ptr());
    };
    return luaK_code(fs,
                     (OP_EXTRAARG as libc::c_int as Instruction) << 0i32 |
                         (a as Instruction) << 0i32 + 6i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_fixline(mut fs: *mut FuncState,
                                      mut line: libc::c_int) -> () {
    *(*(*fs).f).lineinfo.offset(((*fs).pc - 1i32) as isize) = line;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_nil(mut fs: *mut FuncState,
                                  mut from: libc::c_int, mut n: libc::c_int)
 -> () {
    let mut previous: *mut Instruction = 0 as *mut Instruction;
    let mut l: libc::c_int = from + n - 1i32;
    if (*fs).pc > (*fs).lasttarget {
        previous =
            &mut *(*(*fs).f).code.offset(((*fs).pc - 1i32) as isize) as
                *mut Instruction;
        if (*previous >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
               as OpCode as libc::c_uint ==
               OP_LOADNIL as libc::c_int as libc::c_uint {
            let mut pfrom: libc::c_int =
                (*previous >> 0i32 + 6i32 &
                     !((!(0i32 as Instruction)) << 8i32) << 0i32) as
                    libc::c_int;
            let mut pl: libc::c_int =
                pfrom +
                    (*previous >> 0i32 + 6i32 + 8i32 + 9i32 &
                         !((!(0i32 as Instruction)) << 9i32) << 0i32) as
                        libc::c_int;
            if pfrom <= from && from <= pl + 1i32 ||
                   from <= pfrom && pfrom <= l + 1i32 {
                if pfrom < from { from = pfrom }
                if pl > l { l = pl }
                *previous =
                    *previous &
                        !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32)
                        |
                        (from as Instruction) << 0i32 + 6i32 &
                            !((!(0i32 as Instruction)) << 8i32) <<
                                0i32 + 6i32;
                *previous =
                    *previous &
                        !(!((!(0i32 as Instruction)) << 9i32) <<
                              0i32 + 6i32 + 8i32 + 9i32) |
                        ((l - from) as Instruction) <<
                            0i32 + 6i32 + 8i32 + 9i32 &
                            !((!(0i32 as Instruction)) << 9i32) <<
                                0i32 + 6i32 + 8i32 + 9i32;
                return
            }
        }
    }
    luaK_codeABC(fs, OP_LOADNIL, from, n - 1i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_reserveregs(mut fs: *mut FuncState,
                                          mut n: libc::c_int) -> () {
    luaK_checkstack(fs, n);
    (*fs).freereg = ((*fs).freereg as libc::c_int + n) as lu_byte;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_checkstack(mut fs: *mut FuncState,
                                         mut n: libc::c_int) -> () {
    let mut newstack: libc::c_int = (*fs).freereg as libc::c_int + n;
    if newstack > (*(*fs).f).maxstacksize as libc::c_int {
        if newstack >= 255i32 {
            luaX_syntaxerror((*fs).ls,
                             b"function or expression needs too many registers\x00"
                                 as *const u8 as *const libc::c_char);
        } else { (*(*fs).f).maxstacksize = newstack as lu_byte }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_stringK(mut fs: *mut FuncState,
                                      mut s: *mut TString) -> libc::c_int {
    let mut o: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut io: *mut TValue = &mut o;
    let mut x_: *mut TString = s;
    if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      460i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"int luaK_stringK(FuncState *, TString *)\x00")).as_ptr());
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
    (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lcode.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 460i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 41],
                                                           &[libc::c_char; 41]>(b"int luaK_stringK(FuncState *, TString *)\x00")).as_ptr());
               };
               (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int &&
                   ((*(*fs).ls).L.is_null() ||
                        {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lcode.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              460i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 41],
                                                                        &[libc::c_char; 41]>(b"int luaK_stringK(FuncState *, TString *)\x00")).as_ptr());
                            };
                            0 !=
                                ((*(*io).value_.gc).marked as libc::c_int ^
                                     (1i32 << 0i32 | 1i32 << 1i32)) &
                                    ((*(*(*(*fs).ls).L).l_G).currentwhite as
                                         libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32))
                        })
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          460i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 41],
                                                    &[libc::c_char; 41]>(b"int luaK_stringK(FuncState *, TString *)\x00")).as_ptr());
        };
    };
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn addk(mut fs: *mut FuncState, mut key: *mut TValue,
                          mut v: *mut TValue) -> libc::c_int {
    let mut L: *mut lua_State_0 = (*(*fs).ls).L;
    let mut f: *mut Proto = (*fs).f;
    let mut idx: *mut TValue = luaH_set(L, (*(*fs).ls).h, key);
    let mut k: libc::c_int = 0;
    let mut oldsize: libc::c_int = 0;
    if (*idx).tt_ == 3i32 | 1i32 << 4i32 {
        if (*idx).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(b"((((idx))->tt_) == ((3 | (1 << 4))))\x00" as
                              *const u8 as *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          434i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 42],
                                                    &[libc::c_char; 42]>(b"int addk(FuncState *, TValue *, TValue *)\x00")).as_ptr());
        };
        k = (*idx).value_.i as libc::c_int;
        if k < (*fs).nk &&
               (*(*f).k.offset(k as isize)).tt_ & 63i32 == (*v).tt_ & 63i32 &&
               0 !=
                   luaV_equalobj(0 as *mut lua_State_0,
                                 &mut *(*f).k.offset(k as isize), v) {
            return k
        }
    }
    oldsize = (*f).sizek;
    k = (*fs).nk;
    let mut io: *mut TValue = idx;
    (*io).value_.i = k as lua_Integer;
    (*io).tt_ = 3i32 | 1i32 << 4i32;
    if k + 1i32 > (*f).sizek {
        (*f).k =
            luaM_growaux_(L, (*f).k as *mut libc::c_void, &mut (*f).sizek,
                          ::std::mem::size_of::<TValue>() as libc::c_ulong,
                          (1i32 << 9i32 + 9i32 + 8i32) - 1i32,
                          b"constants\x00" as *const u8 as
                              *const libc::c_char) as *mut TValue
    }
    while oldsize < (*f).sizek {
        let fresh1 = oldsize;
        oldsize = oldsize + 1;
        (*(*f).k.offset(fresh1 as isize)).tt_ = 0i32
    }
    let mut io1: *mut TValue = &mut *(*f).k.offset(k as isize) as *mut TValue;
    *io1 = *v;
    if 0 == (*io1).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io1).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lcode.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 448i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 42],
                                                           &[libc::c_char; 42]>(b"int addk(FuncState *, TValue *, TValue *)\x00")).as_ptr());
               };
               (*io1).tt_ & 63i32 == (*(*io1).value_.gc).tt as libc::c_int &&
                   (L.is_null() ||
                        {
                            if 0 != (*io1).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lcode.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              448i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 42],
                                                                        &[libc::c_char; 42]>(b"int addk(FuncState *, TValue *, TValue *)\x00")).as_ptr());
                            };
                            0 !=
                                ((*(*io1).value_.gc).marked as libc::c_int ^
                                     (1i32 << 0i32 | 1i32 << 1i32)) &
                                    ((*(*L).l_G).currentwhite as libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32))
                        })
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          448i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 42],
                                                    &[libc::c_char; 42]>(b"int addk(FuncState *, TValue *, TValue *)\x00")).as_ptr());
        };
    };
    (*fs).nk += 1;
    if 0 != (*v).tt_ & 1i32 << 6i32 &&
           0 != (*f).marked as libc::c_int & 1i32 << 2i32 &&
           {
               if 0 != (*v).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((v)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lcode.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 450i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 42],
                                                           &[libc::c_char; 42]>(b"int addk(FuncState *, TValue *, TValue *)\x00")).as_ptr());
               };
               0 !=
                   (*(*v).value_.gc).marked as libc::c_int &
                       (1i32 << 0i32 | 1i32 << 1i32)
           } {
        if (*f).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((f)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          450i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 42],
                                                    &[libc::c_char; 42]>(b"int addk(FuncState *, TValue *, TValue *)\x00")).as_ptr());
        };
        if 0 != (*v).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(b"(((v)->tt_) & (1 << 6))\x00" as *const u8 as
                              *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          450i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 42],
                                                    &[libc::c_char; 42]>(b"int addk(FuncState *, TValue *, TValue *)\x00")).as_ptr());
        };
        luaC_barrier_(L, &mut (*(f as *mut GCUnion)).gc, (*v).value_.gc);
    } else { };
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_intK(mut fs: *mut FuncState, mut n: lua_Integer)
 -> libc::c_int {
    let mut k: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut o: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = n as size_t as *mut libc::c_void;
    (*io).tt_ = 2i32;
    let mut io_0: *mut TValue = &mut o;
    (*io_0).value_.i = n;
    (*io_0).tt_ = 3i32 | 1i32 << 4i32;
    return addk(fs, &mut k, &mut o);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_dischargevars(mut fs: *mut FuncState,
                                            mut e: *mut expdesc) -> () {
    match (*e).k as libc::c_uint {
        8 => { (*e).k = VNONRELOC }
        9 => {
            (*e).u.info =
                luaK_codeABC(fs, OP_GETUPVAL, 0i32, (*e).u.info, 0i32);
            (*e).k = VRELOCABLE
        }
        10 => {
            let mut op: OpCode = OP_MOVE;
            freereg(fs, (*e).u.ind.idx as libc::c_int);
            if (*e).u.ind.vt as libc::c_int == VLOCAL as libc::c_int {
                freereg(fs, (*e).u.ind.t as libc::c_int);
                op = OP_GETTABLE
            } else {
                if (*e).u.ind.vt as libc::c_int == VUPVAL as libc::c_int {
                } else {
                    __assert_fail(b"e->u.ind.vt == VUPVAL\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lcode.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  575i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 48],
                                                            &[libc::c_char; 48]>(b"void luaK_dischargevars(FuncState *, expdesc *)\x00")).as_ptr());
                };
                op = OP_GETTABUP
            }
            (*e).u.info =
                luaK_codeABC(fs, op, 0i32, (*e).u.ind.t as libc::c_int,
                             (*e).u.ind.idx as libc::c_int);
            (*e).k = VRELOCABLE
        }
        14 | 13 => { luaK_setoneret(fs, e); }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setoneret(mut fs: *mut FuncState,
                                        mut e: *mut expdesc) -> () {
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint {
        if (*(*(*fs).f).code.offset((*e).u.info as isize) >>
                0i32 + 6i32 + 8i32 &
                !((!(0i32 as Instruction)) << 9i32) << 0i32) as libc::c_int ==
               2i32 {
        } else {
            __assert_fail(b"(((int)(((((fs)->f->code[(e)->u.info]))>>((0 + 6) + 8)) & ((~((~(Instruction)0)<<(9)))<<(0))))) == 2\x00"
                              as *const u8 as *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          542i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 44],
                                                    &[libc::c_char; 44]>(b"void luaK_setoneret(FuncState *, expdesc *)\x00")).as_ptr());
        };
        (*e).k = VNONRELOC;
        (*e).u.info =
            (*(*(*fs).f).code.offset((*e).u.info as isize) >> 0i32 + 6i32 &
                 !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int
    } else if (*e).k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint
     {
        *(*(*fs).f).code.offset((*e).u.info as isize) =
            *(*(*fs).f).code.offset((*e).u.info as isize) &
                !(!((!(0i32 as Instruction)) << 9i32) <<
                      0i32 + 6i32 + 8i32 + 9i32) |
                (2i32 as Instruction) << 0i32 + 6i32 + 8i32 + 9i32 &
                    !((!(0i32 as Instruction)) << 9i32) <<
                        0i32 + 6i32 + 8i32 + 9i32;
        (*e).k = VRELOCABLE
    };
}
unsafe extern "C" fn freereg(mut fs: *mut FuncState, mut reg: libc::c_int)
 -> () {
    if 0 == reg & 1i32 << 9i32 - 1i32 && reg >= (*fs).nactvar as libc::c_int {
        (*fs).freereg = (*fs).freereg.wrapping_sub(1);
        if reg == (*fs).freereg as libc::c_int {
        } else {
            __assert_fail(b"reg == fs->freereg\x00" as *const u8 as
                              *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          389i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 31],
                                                    &[libc::c_char; 31]>(b"void freereg(FuncState *, int)\x00")).as_ptr());
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2anyreg(mut fs: *mut FuncState,
                                         mut e: *mut expdesc) -> libc::c_int {
    luaK_dischargevars(fs, e);
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        if !((*e).t != (*e).f) {
            return (*e).u.info
        } else if (*e).u.info >= (*fs).nactvar as libc::c_int {
            exp2reg(fs, e, (*e).u.info);
            return (*e).u.info
        }
    }
    luaK_exp2nextreg(fs, e);
    return (*e).u.info;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2nextreg(mut fs: *mut FuncState,
                                          mut e: *mut expdesc) -> () {
    luaK_dischargevars(fs, e);
    freeexp(fs, e);
    luaK_reserveregs(fs, 1i32);
    exp2reg(fs, e, (*fs).freereg as libc::c_int - 1i32);
}
unsafe extern "C" fn exp2reg(mut fs: *mut FuncState, mut e: *mut expdesc,
                             mut reg: libc::c_int) -> () {
    let mut fj: libc::c_int = 0;
    discharge2reg(fs, e, reg);
    if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint {
        luaK_concat(fs, &mut (*e).t, (*e).u.info);
    }
    if (*e).t != (*e).f {
        let mut final_0: libc::c_int = 0;
        let mut p_f: libc::c_int = -1i32;
        let mut p_t: libc::c_int = -1i32;
        if 0 != need_value(fs, (*e).t) || 0 != need_value(fs, (*e).f) {
            fj =
                if (*e).k as libc::c_uint ==
                       VJMP as libc::c_int as libc::c_uint {
                    -1i32
                } else { luaK_jump(fs) };
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
pub unsafe extern "C" fn luaK_getlabel(mut fs: *mut FuncState)
 -> libc::c_int {
    (*fs).lasttarget = (*fs).pc;
    return (*fs).pc;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_jump(mut fs: *mut FuncState) -> libc::c_int {
    let mut jpc: libc::c_int = (*fs).jpc;
    let mut j: libc::c_int = 0;
    (*fs).jpc = -1i32;
    j =
        luaK_codeABx(fs, OP_JMP, 0i32,
                     (-1i32 + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as
                         libc::c_uint);
    luaK_concat(fs, &mut j, jpc);
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_concat(mut fs: *mut FuncState,
                                     mut l1: *mut libc::c_int,
                                     mut l2: libc::c_int) -> () {
    if l2 == -1i32 {
        return
    } else {
        if *l1 == -1i32 {
            *l1 = l2
        } else {
            let mut list: libc::c_int = *l1;
            let mut next: libc::c_int = 0;
            loop  {
                next = getjump(fs, list);
                if !(next != -1i32) { break ; }
                list = next
            }
            fixjump(fs, list, l2);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchtohere(mut fs: *mut FuncState,
                                          mut list: libc::c_int) -> () {
    luaK_getlabel(fs);
    luaK_concat(fs, &mut (*fs).jpc, list);
}
unsafe extern "C" fn code_loadbool(mut fs: *mut FuncState, mut A: libc::c_int,
                                   mut b: libc::c_int, mut jump: libc::c_int)
 -> libc::c_int {
    luaK_getlabel(fs);
    return luaK_codeABC(fs, OP_LOADBOOL, A, b, jump);
}
unsafe extern "C" fn need_value(mut fs: *mut FuncState, mut list: libc::c_int)
 -> libc::c_int {
    while list != -1i32 {
        let mut i: Instruction = *getjumpcontrol(fs, list);
        if (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
               as libc::c_uint != OP_TESTSET as libc::c_int as libc::c_uint {
            return 1i32
        } else { list = getjump(fs, list) }
    }
    return 0i32;
}
unsafe extern "C" fn discharge2reg(mut fs: *mut FuncState,
                                   mut e: *mut expdesc, mut reg: libc::c_int)
 -> () {
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        1 => { luaK_nil(fs, reg, 1i32); }
        3 | 2 => {
            luaK_codeABC(fs, OP_LOADBOOL, reg,
                         ((*e).k as libc::c_uint ==
                              VTRUE as libc::c_int as libc::c_uint) as
                             libc::c_int, 0i32);
        }
        4 => { luaK_codek(fs, reg, (*e).u.info); }
        5 => { luaK_codek(fs, reg, luaK_numberK(fs, (*e).u.nval)); }
        6 => { luaK_codek(fs, reg, luaK_intK(fs, (*e).u.ival)); }
        12 => {
            let mut pc: *mut Instruction =
                &mut *(*(*fs).f).code.offset((*e).u.info as isize) as
                    *mut Instruction;
            *pc =
                *pc & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32) |
                    (reg as Instruction) << 0i32 + 6i32 &
                        !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32
        }
        7 => {
            if reg != (*e).u.info {
                luaK_codeABC(fs, OP_MOVE, reg, (*e).u.info, 0i32);
            }
        }
        _ => {
            if (*e).k as libc::c_uint == VJMP as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"e->k == VJMP\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char, 629i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 48],
                                                        &[libc::c_char; 48]>(b"void discharge2reg(FuncState *, expdesc *, int)\x00")).as_ptr());
            };
            return
        }
    }
    (*e).u.info = reg;
    (*e).k = VNONRELOC;
}
unsafe extern "C" fn luaK_numberK(mut fs: *mut FuncState, mut r: lua_Number)
 -> libc::c_int {
    let mut o: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut io: *mut TValue = &mut o;
    (*io).value_.n = r;
    (*io).tt_ = 3i32 | 0i32 << 4i32;
    return addk(fs, &mut o, &mut o);
}
unsafe extern "C" fn freeexp(mut fs: *mut FuncState, mut e: *mut expdesc)
 -> () {
    if (*e).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint {
        freereg(fs, (*e).u.info);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2anyregup(mut fs: *mut FuncState,
                                           mut e: *mut expdesc) -> () {
    if (*e).k as libc::c_uint != VUPVAL as libc::c_int as libc::c_uint ||
           (*e).t != (*e).f {
        luaK_exp2anyreg(fs, e);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2val(mut fs: *mut FuncState,
                                      mut e: *mut expdesc) -> () {
    if (*e).t != (*e).f {
        luaK_exp2anyreg(fs, e);
    } else { luaK_dischargevars(fs, e); };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_exp2RK(mut fs: *mut FuncState,
                                     mut e: *mut expdesc) -> libc::c_int {
    let mut current_block: u64;
    luaK_exp2val(fs, e);
    match (*e).k as libc::c_uint {
        2 => {
            (*e).u.info = boolK(fs, 1i32);
            current_block = 7795507790600949094;
        }
        3 => {
            (*e).u.info = boolK(fs, 0i32);
            current_block = 7795507790600949094;
        }
        1 => { (*e).u.info = nilK(fs); current_block = 7795507790600949094; }
        6 => {
            (*e).u.info = luaK_intK(fs, (*e).u.ival);
            current_block = 7795507790600949094;
        }
        5 => {
            (*e).u.info = luaK_numberK(fs, (*e).u.nval);
            current_block = 7795507790600949094;
        }
        4 => { current_block = 7795507790600949094; }
        _ => { current_block = 792017965103506125; }
    }
    match current_block {
        7795507790600949094 => {
            (*e).k = VK;
            if (*e).u.info <= 1i32 {
                return (*e).u.info | 1i32 << 9i32 - 1i32
            }
        }
        _ => { }
    }
    return luaK_exp2anyreg(fs, e);
}
unsafe extern "C" fn nilK(mut fs: *mut FuncState) -> libc::c_int {
    let mut k: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut v: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    v.tt_ = 0i32;
    let mut io: *mut TValue = &mut k;
    let mut x_: *mut Table = (*(*fs).ls).h;
    if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      505i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"int nilK(FuncState *)\x00")).as_ptr());
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
    (*io).tt_ = 5i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lcode.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 505i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 22],
                                                           &[libc::c_char; 22]>(b"int nilK(FuncState *)\x00")).as_ptr());
               };
               (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int &&
                   ((*(*fs).ls).L.is_null() ||
                        {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lcode.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              505i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 22],
                                                                        &[libc::c_char; 22]>(b"int nilK(FuncState *)\x00")).as_ptr());
                            };
                            0 !=
                                ((*(*io).value_.gc).marked as libc::c_int ^
                                     (1i32 << 0i32 | 1i32 << 1i32)) &
                                    ((*(*(*(*fs).ls).L).l_G).currentwhite as
                                         libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32))
                        })
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          505i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 22],
                                                    &[libc::c_char; 22]>(b"int nilK(FuncState *)\x00")).as_ptr());
        };
    };
    return addk(fs, &mut k, &mut v);
}
unsafe extern "C" fn boolK(mut fs: *mut FuncState, mut b: libc::c_int)
 -> libc::c_int {
    let mut o: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut io: *mut TValue = &mut o;
    (*io).value_.b = b;
    (*io).tt_ = 1i32;
    return addk(fs, &mut o, &mut o);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_self(mut fs: *mut FuncState,
                                   mut e: *mut expdesc, mut key: *mut expdesc)
 -> () {
    let mut ereg: libc::c_int = 0;
    luaK_exp2anyreg(fs, e);
    ereg = (*e).u.info;
    freeexp(fs, e);
    (*e).u.info = (*fs).freereg as libc::c_int;
    (*e).k = VNONRELOC;
    luaK_reserveregs(fs, 2i32);
    luaK_codeABC(fs, OP_SELF, (*e).u.info, ereg, luaK_exp2RK(fs, key));
    freeexp(fs, key);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_indexed(mut fs: *mut FuncState,
                                      mut t: *mut expdesc,
                                      mut k: *mut expdesc) -> () {
    if !((*t).t != (*t).f) &&
           ((*t).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint
                ||
                (*t).k as libc::c_uint ==
                    VLOCAL as libc::c_int as libc::c_uint ||
                (*t).k as libc::c_uint ==
                    VUPVAL as libc::c_int as libc::c_uint) {
    } else {
        __assert_fail(b"!((t)->t != (t)->f) && (((t->k) == VNONRELOC || (t->k) == VLOCAL) || t->k == VUPVAL)\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      947i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"void luaK_indexed(FuncState *, expdesc *, expdesc *)\x00")).as_ptr());
    };
    (*t).u.ind.t = (*t).u.info as lu_byte;
    (*t).u.ind.idx = luaK_exp2RK(fs, k) as libc::c_short;
    (*t).u.ind.vt =
        (if (*t).k as libc::c_uint == VUPVAL as libc::c_int as libc::c_uint {
             VUPVAL as libc::c_int
         } else { VLOCAL as libc::c_int }) as lu_byte;
    (*t).k = VINDEXED;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_goiftrue(mut fs: *mut FuncState,
                                       mut e: *mut expdesc) -> () {
    let mut pc: libc::c_int = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        11 => { negatecondition(fs, e); pc = (*e).u.info }
        4 | 5 | 6 | 2 => { pc = -1i32 }
        _ => { pc = jumponcond(fs, e, 0i32) }
    }
    luaK_concat(fs, &mut (*e).f, pc);
    luaK_patchtohere(fs, (*e).t);
    (*e).t = -1i32;
}
unsafe extern "C" fn jumponcond(mut fs: *mut FuncState, mut e: *mut expdesc,
                                mut cond: libc::c_int) -> libc::c_int {
    if (*e).k as libc::c_uint == VRELOCABLE as libc::c_int as libc::c_uint {
        let mut ie: Instruction =
            *(*(*fs).f).code.offset((*e).u.info as isize);
        if (ie >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as
               OpCode as libc::c_uint == OP_NOT as libc::c_int as libc::c_uint
           {
            (*fs).pc -= 1;
            return condjump(fs, OP_TEST,
                            (ie >> 0i32 + 6i32 + 8i32 + 9i32 &
                                 !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as libc::c_int, 0i32,
                            (0 == cond) as libc::c_int)
        }
    }
    discharge2anyreg(fs, e);
    freeexp(fs, e);
    return condjump(fs, OP_TESTSET, (1i32 << 8i32) - 1i32, (*e).u.info, cond);
}
unsafe extern "C" fn condjump(mut fs: *mut FuncState, mut op: OpCode,
                              mut A: libc::c_int, mut B: libc::c_int,
                              mut C: libc::c_int) -> libc::c_int {
    luaK_codeABC(fs, op, A, B, C);
    return luaK_jump(fs);
}
unsafe extern "C" fn discharge2anyreg(mut fs: *mut FuncState,
                                      mut e: *mut expdesc) -> () {
    if (*e).k as libc::c_uint != VNONRELOC as libc::c_int as libc::c_uint {
        luaK_reserveregs(fs, 1i32);
        discharge2reg(fs, e, (*fs).freereg as libc::c_int - 1i32);
    };
}
unsafe extern "C" fn negatecondition(mut fs: *mut FuncState,
                                     mut e: *mut expdesc) -> () {
    let mut pc: *mut Instruction = getjumpcontrol(fs, (*e).u.info);
    if 0 !=
           luaP_opmodes[(*pc >> 0i32 &
                             !((!(0i32 as Instruction)) << 6i32) << 0i32) as
                            OpCode as usize] as libc::c_int & 1i32 << 7i32 &&
           (*pc >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as
               OpCode as libc::c_uint !=
               OP_TESTSET as libc::c_int as libc::c_uint &&
           (*pc >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as
               OpCode as libc::c_uint !=
               OP_TEST as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"(luaP_opmodes[(((OpCode)(((*pc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0)))))] & (1 << 7)) && (((OpCode)(((*pc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) != OP_TESTSET && (((OpCode)(((*pc)>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) != OP_TEST\x00"
                          as *const u8 as *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      828i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void negatecondition(FuncState *, expdesc *)\x00")).as_ptr());
    };
    *pc =
        *pc & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32) |
            ((0 ==
                  (*pc >> 0i32 + 6i32 &
                       !((!(0i32 as Instruction)) << 8i32) << 0i32) as
                      libc::c_int) as libc::c_int as Instruction) <<
                0i32 + 6i32 &
                !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_goiffalse(mut fs: *mut FuncState,
                                        mut e: *mut expdesc) -> () {
    let mut pc: libc::c_int = 0;
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        11 => { pc = (*e).u.info }
        1 | 3 => { pc = -1i32 }
        _ => { pc = jumponcond(fs, e, 1i32) }
    }
    luaK_concat(fs, &mut (*e).t, pc);
    luaK_patchtohere(fs, (*e).f);
    (*e).f = -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_storevar(mut fs: *mut FuncState,
                                       mut var: *mut expdesc,
                                       mut ex: *mut expdesc) -> () {
    match (*var).k as libc::c_uint {
        8 => { freeexp(fs, ex); exp2reg(fs, ex, (*var).u.info); return }
        9 => {
            let mut e: libc::c_int = luaK_exp2anyreg(fs, ex);
            luaK_codeABC(fs, OP_SETUPVAL, e, (*var).u.info, 0i32);
        }
        10 => {
            let mut op: OpCode =
                (if (*var).u.ind.vt as libc::c_int == VLOCAL as libc::c_int {
                     OP_SETTABLE as libc::c_int
                 } else { OP_SETTABUP as libc::c_int }) as OpCode;
            let mut e_0: libc::c_int = luaK_exp2RK(fs, ex);
            luaK_codeABC(fs, op, (*var).u.ind.t as libc::c_int,
                         (*var).u.ind.idx as libc::c_int, e_0);
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char, 800i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 54],
                                                        &[libc::c_char; 54]>(b"void luaK_storevar(FuncState *, expdesc *, expdesc *)\x00")).as_ptr());
            };
        }
    }
    freeexp(fs, ex);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setreturns(mut fs: *mut FuncState,
                                         mut e: *mut expdesc,
                                         mut nresults: libc::c_int) -> () {
    let mut pc: *mut Instruction = 0 as *mut Instruction;
    if (*e).k as libc::c_uint == VCALL as libc::c_int as libc::c_uint {
        *(*(*fs).f).code.offset((*e).u.info as isize) =
            *(*(*fs).f).code.offset((*e).u.info as isize) &
                !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32) |
                ((nresults + 1i32) as Instruction) << 0i32 + 6i32 + 8i32 &
                    !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32
    } else if (*e).k as libc::c_uint == VVARARG as libc::c_int as libc::c_uint
     {
        pc =
            &mut *(*(*fs).f).code.offset((*e).u.info as isize) as
                *mut Instruction;
        *pc =
            *pc &
                !(!((!(0i32 as Instruction)) << 9i32) <<
                      0i32 + 6i32 + 8i32 + 9i32) |
                ((nresults + 1i32) as Instruction) <<
                    0i32 + 6i32 + 8i32 + 9i32 &
                    !((!(0i32 as Instruction)) << 9i32) <<
                        0i32 + 6i32 + 8i32 + 9i32;
        *pc =
            *pc & !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32) |
                ((*fs).freereg as Instruction) << 0i32 + 6i32 &
                    !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32;
        luaK_reserveregs(fs, 1i32);
    } else {
        if nresults == -1i32 {
        } else {
            __assert_fail(b"nresults == (-1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          525i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 50],
                                                    &[libc::c_char; 50]>(b"void luaK_setreturns(FuncState *, expdesc *, int)\x00")).as_ptr());
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_ret(mut fs: *mut FuncState,
                                  mut first: libc::c_int,
                                  mut nret: libc::c_int) -> () {
    luaK_codeABC(fs, OP_RETURN, first, nret + 1i32, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchlist(mut fs: *mut FuncState,
                                        mut list: libc::c_int,
                                        mut target: libc::c_int) -> () {
    if target == (*fs).pc {
        luaK_patchtohere(fs, list);
    } else {
        if target < (*fs).pc {
        } else {
            __assert_fail(b"target < fs->pc\x00" as *const u8 as
                              *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          267i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 43],
                                                    &[libc::c_char; 43]>(b"void luaK_patchlist(FuncState *, int, int)\x00")).as_ptr());
        };
        patchlistaux(fs, list, target, (1i32 << 8i32) - 1i32, target);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchclose(mut fs: *mut FuncState,
                                         mut list: libc::c_int,
                                         mut level: libc::c_int) -> () {
    level += 1;
    while list != -1i32 {
        if (*(*(*fs).f).code.offset(list as isize) >> 0i32 &
                !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as
               libc::c_uint == OP_JMP as libc::c_int as libc::c_uint &&
               ((*(*(*fs).f).code.offset(list as isize) >> 0i32 + 6i32 &
                     !((!(0i32 as Instruction)) << 8i32) << 0i32) as
                    libc::c_int == 0i32 ||
                    (*(*(*fs).f).code.offset(list as isize) >> 0i32 + 6i32 &
                         !((!(0i32 as Instruction)) << 8i32) << 0i32) as
                        libc::c_int >= level) {
        } else {
            __assert_fail(b"(((OpCode)(((fs->f->code[list])>>0) & ((~((~(Instruction)0)<<(6)))<<(0))))) == OP_JMP && ((((int)(((fs->f->code[list])>>(0 + 6)) & ((~((~(Instruction)0)<<(8)))<<(0))))) == 0 || (((int)(((fs->f->code[list])>>(0 + 6)) & ((~((~(Instruction)0)<<(8)))<<(0))))) >= level)\x00"
                              as *const u8 as *const libc::c_char,
                          b"lcode.c\x00" as *const u8 as *const libc::c_char,
                          283i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 44],
                                                    &[libc::c_char; 44]>(b"void luaK_patchclose(FuncState *, int, int)\x00")).as_ptr());
        };
        *(*(*fs).f).code.offset(list as isize) =
            *(*(*fs).f).code.offset(list as isize) &
                !(!((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32) |
                (level as Instruction) << 0i32 + 6i32 &
                    !((!(0i32 as Instruction)) << 8i32) << 0i32 + 6i32;
        list = getjump(fs, list)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_prefix(mut fs: *mut FuncState, mut op: UnOpr_0,
                                     mut e: *mut expdesc,
                                     mut line: libc::c_int) -> () {
    let mut current_block: u64;
    static mut ef: expdesc =
        unsafe {
            expdesc_0{k: VKINT,
                      u: unnamed{ival: 0i32 as lua_Integer,},
                      t: -1i32,
                      f: -1i32,}
        };
    match op as libc::c_uint {
        0 | 1 => {
            if 0 !=
                   constfolding(fs,
                                (op as
                                     libc::c_uint).wrapping_add(12i32 as
                                                                    libc::c_uint)
                                    as libc::c_int, e, &ef) {
                current_block = 12517898123489920830;
            } else { current_block = 9048422517200034782; }
        }
        3 => { current_block = 9048422517200034782; }
        2 => { codenot(fs, e); current_block = 12517898123489920830; }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1077i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 53],
                                                        &[libc::c_char; 53]>(b"void luaK_prefix(FuncState *, UnOpr, expdesc *, int)\x00")).as_ptr());
            };
            current_block = 12517898123489920830;
        }
    }
    match current_block {
        9048422517200034782 => {
            codeunexpval(fs,
                         (op as
                              libc::c_uint).wrapping_add(OP_UNM as libc::c_int
                                                             as libc::c_uint)
                             as OpCode, e, line);
        }
        _ => { }
    };
}
unsafe extern "C" fn codenot(mut fs: *mut FuncState, mut e: *mut expdesc)
 -> () {
    luaK_dischargevars(fs, e);
    match (*e).k as libc::c_uint {
        1 | 3 => { (*e).k = VTRUE }
        4 | 5 | 6 | 2 => { (*e).k = VFALSE }
        11 => { negatecondition(fs, e); }
        12 | 7 => {
            discharge2anyreg(fs, e);
            freeexp(fs, e);
            (*e).u.info = luaK_codeABC(fs, OP_NOT, 0i32, (*e).u.info, 0i32);
            (*e).k = VRELOCABLE
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char, 933i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 37],
                                                        &[libc::c_char; 37]>(b"void codenot(FuncState *, expdesc *)\x00")).as_ptr());
            };
        }
    }
    let mut temp: libc::c_int = (*e).f;
    (*e).f = (*e).t;
    (*e).t = temp;
    removevalues(fs, (*e).f);
    removevalues(fs, (*e).t);
}
unsafe extern "C" fn removevalues(mut fs: *mut FuncState,
                                  mut list: libc::c_int) -> () {
    while list != -1i32 {
        patchtestreg(fs, list, (1i32 << 8i32) - 1i32);
        list = getjump(fs, list)
    };
}
unsafe extern "C" fn codeunexpval(mut fs: *mut FuncState, mut op: OpCode,
                                  mut e: *mut expdesc, mut line: libc::c_int)
 -> () {
    let mut r: libc::c_int = luaK_exp2anyreg(fs, e);
    freeexp(fs, e);
    (*e).u.info = luaK_codeABC(fs, op, 0i32, r, 0i32);
    (*e).k = VRELOCABLE;
    luaK_fixline(fs, line);
}
unsafe extern "C" fn constfolding(mut fs: *mut FuncState, mut op: libc::c_int,
                                  mut e1: *mut expdesc,
                                  mut e2: *const expdesc) -> libc::c_int {
    let mut v1: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut v2: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut res: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    if 0 == tonumeral(e1, &mut v1) || 0 == tonumeral(e2, &mut v2) ||
           0 == validop(op, &mut v1, &mut v2) {
        return 0i32
    } else {
        luaO_arith((*(*fs).ls).L, op, &mut v1, &mut v2, &mut res);
        if res.tt_ == 3i32 | 1i32 << 4i32 {
            (*e1).k = VKINT;
            if res.tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(b"((((&res))->tt_) == ((3 | (1 << 4))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char, 986i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 63],
                                                        &[libc::c_char; 63]>(b"int constfolding(FuncState *, int, expdesc *, const expdesc *)\x00")).as_ptr());
            };
            (*e1).u.ival = res.value_.i
        } else {
            if res.tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"((((&res))->tt_) == ((3 | (0 << 4))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char, 989i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 63],
                                                        &[libc::c_char; 63]>(b"int constfolding(FuncState *, int, expdesc *, const expdesc *)\x00")).as_ptr());
            };
            let mut n: lua_Number = res.value_.n;
            if !(n == n) || n == 0i32 as libc::c_double {
                return 0i32
            } else { (*e1).k = VKFLT; (*e1).u.nval = n }
        }
        return 1i32
    };
}
unsafe extern "C" fn validop(mut op: libc::c_int, mut v1: *mut TValue,
                             mut v2: *mut TValue) -> libc::c_int {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            let mut i: lua_Integer = 0;
            return (0 !=
                        if (*v1).tt_ == 3i32 | 1i32 << 4i32 {
                            if (*v1).tt_ == 3i32 | 1i32 << 4i32 {
                            } else {
                                __assert_fail(b"((((v1))->tt_) == ((3 | (1 << 4))))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lcode.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              965i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 37],
                                                                        &[libc::c_char; 37]>(b"int validop(int, TValue *, TValue *)\x00")).as_ptr());
                            };
                            i = (*v1).value_.i;
                            1i32
                        } else { luaV_tointeger(v1, &mut i, 0i32) } &&
                        0 !=
                            if (*v2).tt_ == 3i32 | 1i32 << 4i32 {
                                if (*v2).tt_ == 3i32 | 1i32 << 4i32 {
                                } else {
                                    __assert_fail(b"((((v2))->tt_) == ((3 | (1 << 4))))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"lcode.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  965i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 37],
                                                                            &[libc::c_char; 37]>(b"int validop(int, TValue *, TValue *)\x00")).as_ptr());
                                };
                                i = (*v2).value_.i;
                                1i32
                            } else { luaV_tointeger(v2, &mut i, 0i32) }) as
                       libc::c_int
        }
        5 | 6 | 3 => {
            if (*v2).tt_ & 15i32 == 3i32 {
            } else {
                __assert_fail(b"(((((((v2))->tt_)) & 0x0F)) == (3))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char, 968i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 37],
                                                        &[libc::c_char; 37]>(b"int validop(int, TValue *, TValue *)\x00")).as_ptr());
            };
            return (if (*v2).tt_ == 3i32 | 1i32 << 4i32 {
                        if (*v2).tt_ == 3i32 | 1i32 << 4i32 {
                        } else {
                            __assert_fail(b"((((v2))->tt_) == ((3 | (1 << 4))))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lcode.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          968i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 37],
                                                                    &[libc::c_char; 37]>(b"int validop(int, TValue *, TValue *)\x00")).as_ptr());
                        };
                        (*v2).value_.i as lua_Number
                    } else {
                        if (*v2).tt_ == 3i32 | 0i32 << 4i32 {
                        } else {
                            __assert_fail(b"((((v2))->tt_) == ((3 | (0 << 4))))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lcode.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          968i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 37],
                                                                    &[libc::c_char; 37]>(b"int validop(int, TValue *, TValue *)\x00")).as_ptr());
                        };
                        (*v2).value_.n
                    } != 0i32 as libc::c_double) as libc::c_int
        }
        _ => { return 1i32 }
    };
}
unsafe extern "C" fn tonumeral(mut e: *const expdesc, mut v: *mut TValue)
 -> libc::c_int {
    let mut io_0: *mut TValue = 0 as *mut TValue;
    let mut io: *mut TValue = 0 as *mut TValue;
    if (*e).t != (*e).f {
        return 0i32
    } else {
        match (*e).k as libc::c_uint {
            6 => {
                if !v.is_null() {
                    io = v;
                    (*io).value_.i = (*e).u.ival;
                    (*io).tt_ = 3i32 | 1i32 << 4i32
                }
                return 1i32
            }
            5 => {
                if !v.is_null() {
                    io_0 = v;
                    (*io_0).value_.n = (*e).u.nval;
                    (*io_0).tt_ = 3i32 | 0i32 << 4i32
                }
                return 1i32
            }
            _ => { return 0i32 }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_infix(mut fs: *mut FuncState, mut op: BinOpr,
                                    mut v: *mut expdesc) -> () {
    match op as libc::c_uint {
        19 => { luaK_goiftrue(fs, v); }
        20 => { luaK_goiffalse(fs, v); }
        12 => { luaK_exp2nextreg(fs, v); }
        0 | 1 | 2 | 5 | 6 | 3 | 4 | 7 | 8 | 9 | 10 | 11 => {
            if 0 == tonumeral(v, 0 as *mut TValue) { luaK_exp2RK(fs, v); }
        }
        _ => { luaK_exp2RK(fs, v); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_posfix(mut fs: *mut FuncState, mut op: BinOpr,
                                     mut e1: *mut expdesc,
                                     mut e2: *mut expdesc,
                                     mut line: libc::c_int) -> () {
    match op as libc::c_uint {
        19 => {
            if (*e1).t == -1i32 {
            } else {
                __assert_fail(b"e1->t == (-1)\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1128i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 65],
                                                        &[libc::c_char; 65]>(b"void luaK_posfix(FuncState *, BinOpr, expdesc *, expdesc *, int)\x00")).as_ptr());
            };
            luaK_dischargevars(fs, e2);
            luaK_concat(fs, &mut (*e2).f, (*e1).f);
            *e1 = *e2
        }
        20 => {
            if (*e1).f == -1i32 {
            } else {
                __assert_fail(b"e1->f == (-1)\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1135i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 65],
                                                        &[libc::c_char; 65]>(b"void luaK_posfix(FuncState *, BinOpr, expdesc *, expdesc *, int)\x00")).as_ptr());
            };
            luaK_dischargevars(fs, e2);
            luaK_concat(fs, &mut (*e2).t, (*e1).t);
            *e1 = *e2
        }
        12 => {
            luaK_exp2val(fs, e2);
            if (*e2).k as libc::c_uint ==
                   VRELOCABLE as libc::c_int as libc::c_uint &&
                   (*(*(*fs).f).code.offset((*e2).u.info as isize) >> 0i32 &
                        !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
                       as libc::c_uint ==
                       OP_CONCAT as libc::c_int as libc::c_uint {
                if (*e1).u.info ==
                       (*(*(*fs).f).code.offset((*e2).u.info as isize) >>
                            0i32 + 6i32 + 8i32 + 9i32 &
                            !((!(0i32 as Instruction)) << 9i32) << 0i32) as
                           libc::c_int - 1i32 {
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
                    *(*(*fs).f).code.offset((*e2).u.info as isize) &
                        !(!((!(0i32 as Instruction)) << 9i32) <<
                              0i32 + 6i32 + 8i32 + 9i32) |
                        ((*e1).u.info as Instruction) <<
                            0i32 + 6i32 + 8i32 + 9i32 &
                            !((!(0i32 as Instruction)) << 9i32) <<
                                0i32 + 6i32 + 8i32 + 9i32;
                (*e1).k = VRELOCABLE;
                (*e1).u.info = (*e2).u.info
            } else {
                luaK_exp2nextreg(fs, e2);
                codebinexpval(fs, OP_CONCAT, e1, e2, line);
            }
        }
        0 | 1 | 2 | 5 | 6 | 3 | 4 | 7 | 8 | 9 | 10 | 11 => {
            if 0 ==
                   constfolding(fs,
                                (op as
                                     libc::c_uint).wrapping_add(0i32 as
                                                                    libc::c_uint)
                                    as libc::c_int, e1, e2) {
                codebinexpval(fs,
                              (op as
                                   libc::c_uint).wrapping_add(OP_ADD as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                  as OpCode, e1, e2, line);
            }
        }
        13 | 14 | 15 | 16 | 17 | 18 => { codecomp(fs, op, e1, e2); }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1169i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 65],
                                                        &[libc::c_char; 65]>(b"void luaK_posfix(FuncState *, BinOpr, expdesc *, expdesc *, int)\x00")).as_ptr());
            };
        }
    };
}
unsafe extern "C" fn codecomp(mut fs: *mut FuncState, mut opr: BinOpr,
                              mut e1: *mut expdesc, mut e2: *mut expdesc)
 -> () {
    let mut rk1: libc::c_int =
        if (*e1).k as libc::c_uint == VK as libc::c_int as libc::c_uint {
            (*e1).u.info | 1i32 << 9i32 - 1i32
        } else {
            if (*e1).k as libc::c_uint ==
                   VNONRELOC as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"e1->k == VNONRELOC\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lcode.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1039i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 57],
                                                        &[libc::c_char; 57]>(b"void codecomp(FuncState *, BinOpr, expdesc *, expdesc *)\x00")).as_ptr());
            };
            (*e1).u.info
        };
    let mut rk2: libc::c_int = luaK_exp2RK(fs, e2);
    freeexps(fs, e1, e2);
    match opr as libc::c_uint {
        16 => { (*e1).u.info = condjump(fs, OP_EQ, 0i32, rk1, rk2) }
        17 | 18 => {
            let mut op: OpCode =
                (opr as
                     libc::c_uint).wrapping_sub(OPR_NE as libc::c_int as
                                                    libc::c_uint).wrapping_add(OP_EQ
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                    as OpCode;
            (*e1).u.info = condjump(fs, op, 1i32, rk2, rk1)
        }
        _ => {
            let mut op_0: OpCode =
                (opr as
                     libc::c_uint).wrapping_sub(OPR_EQ as libc::c_int as
                                                    libc::c_uint).wrapping_add(OP_EQ
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                    as OpCode;
            (*e1).u.info = condjump(fs, op_0, 1i32, rk1, rk2)
        }
    }
    (*e1).k = VJMP;
}
unsafe extern "C" fn freeexps(mut fs: *mut FuncState, mut e1: *mut expdesc,
                              mut e2: *mut expdesc) -> () {
    let mut r1: libc::c_int =
        if (*e1).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint
           {
            (*e1).u.info
        } else { -1i32 };
    let mut r2: libc::c_int =
        if (*e2).k as libc::c_uint == VNONRELOC as libc::c_int as libc::c_uint
           {
            (*e2).u.info
        } else { -1i32 };
    if r1 > r2 {
        freereg(fs, r1);
        freereg(fs, r2);
    } else { freereg(fs, r2); freereg(fs, r1); };
}
unsafe extern "C" fn codebinexpval(mut fs: *mut FuncState, mut op: OpCode,
                                   mut e1: *mut expdesc, mut e2: *mut expdesc,
                                   mut line: libc::c_int) -> () {
    let mut rk2: libc::c_int = luaK_exp2RK(fs, e2);
    let mut rk1: libc::c_int = luaK_exp2RK(fs, e1);
    freeexps(fs, e1, e2);
    (*e1).u.info = luaK_codeABC(fs, op, 0i32, rk1, rk2);
    (*e1).k = VRELOCABLE;
    luaK_fixline(fs, line);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setlist(mut fs: *mut FuncState,
                                      mut base: libc::c_int,
                                      mut nelems: libc::c_int,
                                      mut tostore: libc::c_int) -> () {
    let mut c: libc::c_int = (nelems - 1i32) / 50i32 + 1i32;
    let mut b: libc::c_int = if tostore == -1i32 { 0i32 } else { tostore };
    if tostore != 0i32 && tostore <= 50i32 {
    } else {
        __assert_fail(b"tostore != 0 && tostore <= 50\x00" as *const u8 as
                          *const libc::c_char,
                      b"lcode.c\x00" as *const u8 as *const libc::c_char,
                      1192i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"void luaK_setlist(FuncState *, int, int, int)\x00")).as_ptr());
    };
    if c <= (1i32 << 9i32) - 1i32 {
        luaK_codeABC(fs, OP_SETLIST, base, b, c);
    } else if c <= (1i32 << 9i32 + 9i32 + 8i32) - 1i32 {
        luaK_codeABC(fs, OP_SETLIST, base, b, 0i32);
        codeextraarg(fs, c);
    } else {
        luaX_syntaxerror((*fs).ls,
                         b"constructor too long\x00" as *const u8 as
                             *const libc::c_char);
    }
    (*fs).freereg = (base + 1i32) as lu_byte;
}
