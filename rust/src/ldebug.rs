#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, offset_to)]
extern crate libc;
extern "C" {
    pub type lua_longjmp;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn luaH_new(L: *mut lua_State_0) -> *mut Table_0;
    #[no_mangle]
    fn luaH_setint(L: *mut lua_State_0, t: *mut Table_0, key: lua_Integer,
                   value: *mut TValue) -> ();
    #[no_mangle]
    fn luaF_getlocalname(func: *const Proto_0, local_number: libc::c_int,
                         pc: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    fn luaO_chunkid(out: *mut libc::c_char, source: *const libc::c_char,
                    len: size_t) -> ();
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaT_objtypename(L: *mut lua_State_0, o: *const TValue)
     -> *const libc::c_char;
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaG_runerror(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer,
                      mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State_0, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State_0, func: StkId,
                        nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_hook(L: *mut lua_State_0, event: libc::c_int, line: libc::c_int)
     -> ();
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __sig_atomic_t = libc::c_int;
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
pub type Closure = Closure_0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure_0 {
    c: CClosure,
    l: LClosure,
}
pub type LClosure = LClosure_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
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
    pub cache: *mut LClosure_0,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
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
pub type Table_0 = Table;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata,
    cl: Closure_0,
    h: Table,
    p: Proto,
    th: lua_State,
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
pub type UTString = UTString_0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union UTString_0 {
    dummy: L_Umaxalign,
    tsv: TString,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
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
pub const OP_LE: OpCode = 33;
pub const OP_LT: OpCode = 32;
pub const OP_EQ: OpCode = 31;
pub const OP_CONCAT: OpCode = 29;
pub const OP_LEN: OpCode = 28;
pub const OP_BNOT: OpCode = 26;
pub const OP_UNM: OpCode = 25;
pub const OP_ADD: OpCode = 13;
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
pub const OP_TAILCALL: OpCode = 37;
pub const OP_CALL: OpCode = 36;
pub const OP_TESTSET: OpCode = 35;
pub const OP_TEST: OpCode = 34;
pub const OP_JMP: OpCode = 30;
pub const OP_NOT: OpCode = 27;
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
pub unsafe extern "C" fn lua_getstack(mut L: *mut lua_State_0,
                                      mut level: libc::c_int,
                                      mut ar: *mut lua_Debug) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    if level < 0i32 {
        return 0i32
    } else {
        let ref mut fresh0 =
            *(*((L as
                     *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                     as libc::c_ulong as
                                                     isize)) as
                    *mut libc::c_void as *mut L_EXTRA)).plock;
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
        ci = (*L).ci;
        while level > 0i32 && ci != &mut (*L).base_ci as *mut CallInfo_0 {
            level -= 1;
            ci = (*ci).previous
        }
        if level == 0i32 && ci != &mut (*L).base_ci as *mut CallInfo_0 {
            status = 1i32;
            (*ar).i_ci = ci
        } else { status = 0i32 }
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
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          123i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"int lua_getstack(lua_State *, int, lua_Debug *)\x00")).as_ptr());
        };
        return status
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_getinfo(mut L: *mut lua_State_0,
                                     mut what: *const libc::c_char,
                                     mut ar: *mut lua_Debug) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut cl: *mut Closure = 0 as *mut Closure;
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    let mut func: StkId = 0 as *mut TValue;
    let ref mut fresh3 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
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
        ci = 0 as *mut CallInfo_0;
        func = (*L).top.offset(-1isize);
        if (*func).tt_ & 15i32 == 6i32 &&
               !(b"function expected\x00" as *const u8 as
                     *const libc::c_char).is_null() {
        } else {
            __assert_fail(b"(((((((func)->tt_)) & 0x0F)) == (6))) && \"function expected\"\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          320i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 56],
                                                    &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
        };
        what = what.offset(1isize);
        (*L).top = (*L).top.offset(-1isize)
    } else {
        ci = (*ar).i_ci;
        func = (*ci).func;
        if (*(*ci).func).tt_ & 15i32 == 6i32 {
        } else {
            __assert_fail(b"((((((ci->func)->tt_)) & 0x0F)) == (6))\x00" as
                              *const u8 as *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          327i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 56],
                                                    &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
        };
    }
    cl =
        if (*func).tt_ & 31i32 == 6i32 {
            if (*func).tt_ & 31i32 == 6i32 {
            } else {
                __assert_fail(b"((((func)->tt_) & 0x1F) == 6)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 329i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 56],
                                                        &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
            };
            if (*(*func).value_.gc).tt as libc::c_int & 15i32 == 6i32 {
            } else {
                __assert_fail(b"(((((func)->value_).gc)->tt) & 0x0F) == 6\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 329i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 56],
                                                        &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
            };
            &mut (*((*func).value_.gc as *mut GCUnion)).cl as *mut Closure_0
        } else { 0 as *mut Closure_0 };
    status = auxgetinfo(L, what, ar, cl, ci);
    if !strchr(what, 'f' as i32).is_null() {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *func;
        if 0 == (*io1).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io1).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ldebug.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     332i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 56],
                                                               &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
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
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  332i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 56],
                                                                            &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
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
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 332i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 56],
                                                        &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top &&
               !(b"stack overflow\x00" as *const u8 as
                     *const libc::c_char).is_null() {
        } else {
            __assert_fail(b"(L->top <= L->ci->top) && \"stack overflow\"\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          333i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 56],
                                                    &[libc::c_char; 56]>(b"int lua_getinfo(lua_State *, const char *, lua_Debug *)\x00")).as_ptr());
        };
    }
    swapextra(L);
    if !strchr(what, 'L' as i32).is_null() { collectvalidlines(L, cl); }
    let ref mut fresh5 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
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
unsafe extern "C" fn collectvalidlines(mut L: *mut lua_State_0,
                                       mut f: *mut Closure) -> () {
    if f.is_null() || (*f).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
        (*(*L).top).tt_ = 0i32;
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top &&
               !(b"stack overflow\x00" as *const u8 as
                     *const libc::c_char).is_null() {
        } else {
            __assert_fail(b"(L->top <= L->ci->top) && \"stack overflow\"\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          233i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"void collectvalidlines(lua_State *, Closure *)\x00")).as_ptr());
        };
    } else {
        let mut i: libc::c_int = 0;
        let mut v: TValue =
            lua_TValue{value_: Value_0{gc: 0 as *mut GCObject,}, tt_: 0,};
        let mut lineinfo: *mut libc::c_int = (*(*f).l.p).lineinfo;
        let mut t: *mut Table_0 = luaH_new(L);
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut Table_0 = t;
        if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          240i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"void collectvalidlines(lua_State *, Closure *)\x00")).as_ptr());
        };
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
        (*io).tt_ = 5i32 | 1i32 << 6i32;
        if 0 == (*io).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ldebug.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     240i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 47],
                                                               &[libc::c_char; 47]>(b"void collectvalidlines(lua_State *, Closure *)\x00")).as_ptr());
                   };
                   (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int
                       &&
                       (L.is_null() ||
                            {
                                if 0 != (*io).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  240i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 47],
                                                                            &[libc::c_char; 47]>(b"void collectvalidlines(lua_State *, Closure *)\x00")).as_ptr());
                                };
                                0 !=
                                    ((*(*io).value_.gc).marked as libc::c_int
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
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 240i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 47],
                                                        &[libc::c_char; 47]>(b"void collectvalidlines(lua_State *, Closure *)\x00")).as_ptr());
            };
        };
        (*L).top = (*L).top.offset(1isize);
        if (*L).top <= (*(*L).ci).top &&
               !(b"stack overflow\x00" as *const u8 as
                     *const libc::c_char).is_null() {
        } else {
            __assert_fail(b"(L->top <= L->ci->top) && \"stack overflow\"\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          241i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"void collectvalidlines(lua_State *, Closure *)\x00")).as_ptr());
        };
        let mut io_0: *mut TValue = &mut v;
        (*io_0).value_.b = 1i32;
        (*io_0).tt_ = 1i32;
        i = 0i32;
        while i < (*(*f).l.p).sizelineinfo {
            luaH_setint(L, t, *lineinfo.offset(i as isize) as lua_Integer,
                        &mut v);
            i += 1
        }
    };
}
unsafe extern "C" fn swapextra(mut L: *mut lua_State_0) -> () {
    if (*L).status as libc::c_int == 1i32 {
        let mut ci: *mut CallInfo_0 = (*L).ci;
        let mut temp: StkId = (*ci).func;
        (*ci).func =
            ((*L).stack as *mut libc::c_char).offset((*ci).extra as isize) as
                *mut TValue;
        (*ci).extra =
            ((*L).stack as
                 *mut libc::c_char).offset_to(temp as
                                                  *mut libc::c_char).expect("bad offset_to")
                as libc::c_long
    };
}
unsafe extern "C" fn auxgetinfo(mut L: *mut lua_State_0,
                                mut what: *const libc::c_char,
                                mut ar: *mut lua_Debug, mut f: *mut Closure,
                                mut ci: *mut CallInfo_0) -> libc::c_int {
    let mut status: libc::c_int = 1i32;
    while 0 != *what {
        match *what as libc::c_int {
            83 => { funcinfo(ar, f); }
            108 => {
                (*ar).currentline =
                    if !ci.is_null() &&
                           0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32
                       {
                        currentline(ci)
                    } else { -1i32 }
            }
            117 => {
                (*ar).nups =
                    (if f.is_null() {
                         0i32
                     } else { (*f).c.nupvalues as libc::c_int }) as
                        libc::c_uchar;
                if f.is_null() ||
                       (*f).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                    (*ar).isvararg = 1i32 as libc::c_char;
                    (*ar).nparams = 0i32 as libc::c_uchar
                } else {
                    (*ar).isvararg = (*(*f).l.p).is_vararg as libc::c_char;
                    (*ar).nparams = (*(*f).l.p).numparams
                }
            }
            116 => {
                (*ar).istailcall =
                    (if !ci.is_null() {
                         (*ci).callstatus as libc::c_int & 1i32 << 5i32
                     } else { 0i32 }) as libc::c_char
            }
            110 => {
                (*ar).namewhat = getfuncname(L, ci, &mut (*ar).name);
                if (*ar).namewhat.is_null() {
                    (*ar).namewhat =
                        b"\x00" as *const u8 as *const libc::c_char;
                    (*ar).name = 0 as *const libc::c_char
                }
            }
            76 | 102 => { }
            _ => { status = 0i32 }
        }
        what = what.offset(1isize)
    }
    return status;
}
unsafe extern "C" fn getfuncname(mut L: *mut lua_State_0,
                                 mut ci: *mut CallInfo_0,
                                 mut name: *mut *const libc::c_char)
 -> *const libc::c_char {
    if ci.is_null() {
        return 0 as *const libc::c_char
    } else if 0 != (*ci).callstatus as libc::c_int & 1i32 << 8i32 {
        *name = b"__gc\x00" as *const u8 as *const libc::c_char;
        return b"metamethod\x00" as *const u8 as *const libc::c_char
    } else if 0 == (*ci).callstatus as libc::c_int & 1i32 << 5i32 &&
                  0 !=
                      (*(*ci).previous).callstatus as libc::c_int &
                          1i32 << 1i32 {
        return funcnamefromcode(L, (*ci).previous, name)
    } else { return 0 as *const libc::c_char };
}
unsafe extern "C" fn funcnamefromcode(mut L: *mut lua_State_0,
                                      mut ci: *mut CallInfo_0,
                                      mut name: *mut *const libc::c_char)
 -> *const libc::c_char {
    let mut tm: TMS = TM_INDEX;
    if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      495i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[libc::c_char; 69]>(b"const char *funcnamefromcode(lua_State *, CallInfo *, const char **)\x00")).as_ptr());
    };
    if (*(*(*ci).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      495i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[libc::c_char; 69]>(b"const char *funcnamefromcode(lua_State *, CallInfo *, const char **)\x00")).as_ptr());
    };
    let mut p: *mut Proto_0 =
        (*(&mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l as
               *mut LClosure)).p;
    let mut pc: libc::c_int = currentpc(ci);
    let mut i: Instruction = *(*p).code.offset(pc as isize);
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 2i32 {
        *name = b"?\x00" as *const u8 as *const libc::c_char;
        return b"hook\x00" as *const u8 as *const libc::c_char
    } else {
        match (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as
                  OpCode as libc::c_uint {
            36 | 37 => {
                return getobjname(p, pc,
                                  (i >> 0i32 + 6i32 &
                                       !((!(0i32 as Instruction)) << 8i32) <<
                                           0i32) as libc::c_int, name)
            }
            41 => {
                *name =
                    b"for iterator\x00" as *const u8 as *const libc::c_char;
                return b"for iterator\x00" as *const u8 as *const libc::c_char
            }
            12 | 6 | 7 => { tm = TM_INDEX }
            8 | 10 => { tm = TM_NEWINDEX }
            13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => {
                let mut offset: libc::c_int =
                    (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32)
                        as OpCode as libc::c_int - OP_ADD as libc::c_int;
                tm = (offset + TM_ADD as libc::c_int) as TMS
            }
            25 => { tm = TM_UNM }
            26 => { tm = TM_BNOT }
            28 => { tm = TM_LEN }
            29 => { tm = TM_CONCAT }
            31 => { tm = TM_EQ }
            32 => { tm = TM_LT }
            33 => { tm = TM_LE }
            _ => { return 0 as *const libc::c_char }
        }
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof(((L->l_G)->tmname[tm])->extra)\x00" as
                              *const u8 as *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          534i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 69],
                                                    &[libc::c_char; 69]>(b"const char *funcnamefromcode(lua_State *, CallInfo *, const char **)\x00")).as_ptr());
        };
        *name =
            ((*(*L).l_G).tmname[tm as usize] as
                 *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                               as libc::c_ulong as isize);
        return b"metamethod\x00" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn currentpc(mut ci: *mut CallInfo_0) -> libc::c_int {
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
    } else {
        __assert_fail(b"((ci)->callstatus & (1<<1))\x00" as *const u8 as
                          *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      47i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"int currentpc(CallInfo *)\x00")).as_ptr());
    };
    if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      48i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"int currentpc(CallInfo *)\x00")).as_ptr());
    };
    if (*(*(*ci).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      48i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"int currentpc(CallInfo *)\x00")).as_ptr());
    };
    return (*(*(&mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l as
                    *mut LClosure)).p).code.offset_to((*ci).u.l.savedpc).expect("bad offset_to")
               as libc::c_long as libc::c_int - 1i32;
}
unsafe extern "C" fn getobjname(mut p: *mut Proto_0, mut lastpc: libc::c_int,
                                mut reg: libc::c_int,
                                mut name: *mut *const libc::c_char)
 -> *const libc::c_char {
    let mut current_block: u64;
    let mut pc: libc::c_int = 0;
    *name = luaF_getlocalname(p, reg + 1i32, lastpc);
    if !(*name).is_null() {
        return b"local\x00" as *const u8 as *const libc::c_char
    } else {
        pc = findsetreg(p, lastpc, reg);
        if pc != -1i32 {
            let mut i: Instruction = *(*p).code.offset(pc as isize);
            let mut op: OpCode =
                (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as
                    OpCode;
            match op as libc::c_uint {
                0 => {
                    current_block = 10680521327981672866;
                    match current_block {
                        10680521327981672866 => {
                            let mut b: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            if b <
                                   (i >> 0i32 + 6i32 &
                                        !((!(0i32 as Instruction)) << 8i32) <<
                                            0i32) as libc::c_int {
                                return getobjname(p, pc, b, name)
                            }
                        }
                        820271813250567934 => {
                            let mut k: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut t: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut vn: *const libc::c_char =
                                if op as libc::c_uint ==
                                       OP_GETTABLE as libc::c_int as
                                           libc::c_uint {
                                    luaF_getlocalname(p, t + 1i32, pc)
                                } else { upvalname(p, t) };
                            kname(p, pc, k, name);
                            return if !vn.is_null() &&
                                          strcmp(vn,
                                                 b"_ENV\x00" as *const u8 as
                                                     *const libc::c_char) ==
                                              0i32 {
                                       b"global\x00" as *const u8 as
                                           *const libc::c_char
                                   } else {
                                       b"field\x00" as *const u8 as
                                           *const libc::c_char
                                   }
                        }
                        6483416627284290920 => {
                            *name =
                                upvalname(p,
                                          (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                               !((!(0i32 as Instruction)) <<
                                                     9i32) << 0i32) as
                                              libc::c_int);
                            return b"upvalue\x00" as *const u8 as
                                       *const libc::c_char
                        }
                        11006700562992250127 => {
                            let mut b_0: libc::c_int =
                                if op as libc::c_uint ==
                                       OP_LOADK as libc::c_int as libc::c_uint
                                   {
                                    (i >> 0i32 + 6i32 + 8i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32) << 0i32) as
                                        libc::c_int
                                } else {
                                    (*(*p).code.offset((pc + 1i32) as isize)
                                         >> 0i32 + 6i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32 + 8i32) << 0i32) as
                                        libc::c_int
                                };
                            if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32 ==
                                   4i32 {
                                if 0 !=
                                       ::std::mem::size_of::<lu_byte>() as
                                           libc::c_ulong {
                                } else {
                                    __assert_fail(b"sizeof((((((((((((&p->k[b]))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (((((((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((&p->k[b])->value_).gc))))->ts))))))->extra)\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32
                                       == 4i32 {
                                } else {
                                    __assert_fail(b"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*(*p).k.offset(b_0 as
                                                         isize)).value_.gc).tt
                                       as libc::c_int & 15i32 == 4i32 {
                                } else {
                                    __assert_fail(b"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                *name =
                                    (&mut (*((*(*p).k.offset(b_0 as
                                                                 isize)).value_.gc
                                                 as *mut GCUnion)).ts as
                                         *mut TString_0 as
                                         *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                                       as
                                                                       libc::c_ulong
                                                                       as
                                                                       isize);
                                return b"constant\x00" as *const u8 as
                                           *const libc::c_char
                            }
                        }
                        _ => {
                            let mut k_0: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            kname(p, pc, k_0, name);
                            return b"method\x00" as *const u8 as
                                       *const libc::c_char
                        }
                    }
                }
                6 | 7 => {
                    current_block = 820271813250567934;
                    match current_block {
                        10680521327981672866 => {
                            let mut b: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            if b <
                                   (i >> 0i32 + 6i32 &
                                        !((!(0i32 as Instruction)) << 8i32) <<
                                            0i32) as libc::c_int {
                                return getobjname(p, pc, b, name)
                            }
                        }
                        820271813250567934 => {
                            let mut k: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut t: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut vn: *const libc::c_char =
                                if op as libc::c_uint ==
                                       OP_GETTABLE as libc::c_int as
                                           libc::c_uint {
                                    luaF_getlocalname(p, t + 1i32, pc)
                                } else { upvalname(p, t) };
                            kname(p, pc, k, name);
                            return if !vn.is_null() &&
                                          strcmp(vn,
                                                 b"_ENV\x00" as *const u8 as
                                                     *const libc::c_char) ==
                                              0i32 {
                                       b"global\x00" as *const u8 as
                                           *const libc::c_char
                                   } else {
                                       b"field\x00" as *const u8 as
                                           *const libc::c_char
                                   }
                        }
                        6483416627284290920 => {
                            *name =
                                upvalname(p,
                                          (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                               !((!(0i32 as Instruction)) <<
                                                     9i32) << 0i32) as
                                              libc::c_int);
                            return b"upvalue\x00" as *const u8 as
                                       *const libc::c_char
                        }
                        11006700562992250127 => {
                            let mut b_0: libc::c_int =
                                if op as libc::c_uint ==
                                       OP_LOADK as libc::c_int as libc::c_uint
                                   {
                                    (i >> 0i32 + 6i32 + 8i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32) << 0i32) as
                                        libc::c_int
                                } else {
                                    (*(*p).code.offset((pc + 1i32) as isize)
                                         >> 0i32 + 6i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32 + 8i32) << 0i32) as
                                        libc::c_int
                                };
                            if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32 ==
                                   4i32 {
                                if 0 !=
                                       ::std::mem::size_of::<lu_byte>() as
                                           libc::c_ulong {
                                } else {
                                    __assert_fail(b"sizeof((((((((((((&p->k[b]))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (((((((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((&p->k[b])->value_).gc))))->ts))))))->extra)\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32
                                       == 4i32 {
                                } else {
                                    __assert_fail(b"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*(*p).k.offset(b_0 as
                                                         isize)).value_.gc).tt
                                       as libc::c_int & 15i32 == 4i32 {
                                } else {
                                    __assert_fail(b"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                *name =
                                    (&mut (*((*(*p).k.offset(b_0 as
                                                                 isize)).value_.gc
                                                 as *mut GCUnion)).ts as
                                         *mut TString_0 as
                                         *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                                       as
                                                                       libc::c_ulong
                                                                       as
                                                                       isize);
                                return b"constant\x00" as *const u8 as
                                           *const libc::c_char
                            }
                        }
                        _ => {
                            let mut k_0: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            kname(p, pc, k_0, name);
                            return b"method\x00" as *const u8 as
                                       *const libc::c_char
                        }
                    }
                }
                5 => {
                    current_block = 6483416627284290920;
                    match current_block {
                        10680521327981672866 => {
                            let mut b: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            if b <
                                   (i >> 0i32 + 6i32 &
                                        !((!(0i32 as Instruction)) << 8i32) <<
                                            0i32) as libc::c_int {
                                return getobjname(p, pc, b, name)
                            }
                        }
                        820271813250567934 => {
                            let mut k: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut t: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut vn: *const libc::c_char =
                                if op as libc::c_uint ==
                                       OP_GETTABLE as libc::c_int as
                                           libc::c_uint {
                                    luaF_getlocalname(p, t + 1i32, pc)
                                } else { upvalname(p, t) };
                            kname(p, pc, k, name);
                            return if !vn.is_null() &&
                                          strcmp(vn,
                                                 b"_ENV\x00" as *const u8 as
                                                     *const libc::c_char) ==
                                              0i32 {
                                       b"global\x00" as *const u8 as
                                           *const libc::c_char
                                   } else {
                                       b"field\x00" as *const u8 as
                                           *const libc::c_char
                                   }
                        }
                        6483416627284290920 => {
                            *name =
                                upvalname(p,
                                          (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                               !((!(0i32 as Instruction)) <<
                                                     9i32) << 0i32) as
                                              libc::c_int);
                            return b"upvalue\x00" as *const u8 as
                                       *const libc::c_char
                        }
                        11006700562992250127 => {
                            let mut b_0: libc::c_int =
                                if op as libc::c_uint ==
                                       OP_LOADK as libc::c_int as libc::c_uint
                                   {
                                    (i >> 0i32 + 6i32 + 8i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32) << 0i32) as
                                        libc::c_int
                                } else {
                                    (*(*p).code.offset((pc + 1i32) as isize)
                                         >> 0i32 + 6i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32 + 8i32) << 0i32) as
                                        libc::c_int
                                };
                            if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32 ==
                                   4i32 {
                                if 0 !=
                                       ::std::mem::size_of::<lu_byte>() as
                                           libc::c_ulong {
                                } else {
                                    __assert_fail(b"sizeof((((((((((((&p->k[b]))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (((((((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((&p->k[b])->value_).gc))))->ts))))))->extra)\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32
                                       == 4i32 {
                                } else {
                                    __assert_fail(b"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*(*p).k.offset(b_0 as
                                                         isize)).value_.gc).tt
                                       as libc::c_int & 15i32 == 4i32 {
                                } else {
                                    __assert_fail(b"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                *name =
                                    (&mut (*((*(*p).k.offset(b_0 as
                                                                 isize)).value_.gc
                                                 as *mut GCUnion)).ts as
                                         *mut TString_0 as
                                         *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                                       as
                                                                       libc::c_ulong
                                                                       as
                                                                       isize);
                                return b"constant\x00" as *const u8 as
                                           *const libc::c_char
                            }
                        }
                        _ => {
                            let mut k_0: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            kname(p, pc, k_0, name);
                            return b"method\x00" as *const u8 as
                                       *const libc::c_char
                        }
                    }
                }
                1 | 2 => {
                    current_block = 11006700562992250127;
                    match current_block {
                        10680521327981672866 => {
                            let mut b: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            if b <
                                   (i >> 0i32 + 6i32 &
                                        !((!(0i32 as Instruction)) << 8i32) <<
                                            0i32) as libc::c_int {
                                return getobjname(p, pc, b, name)
                            }
                        }
                        820271813250567934 => {
                            let mut k: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut t: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut vn: *const libc::c_char =
                                if op as libc::c_uint ==
                                       OP_GETTABLE as libc::c_int as
                                           libc::c_uint {
                                    luaF_getlocalname(p, t + 1i32, pc)
                                } else { upvalname(p, t) };
                            kname(p, pc, k, name);
                            return if !vn.is_null() &&
                                          strcmp(vn,
                                                 b"_ENV\x00" as *const u8 as
                                                     *const libc::c_char) ==
                                              0i32 {
                                       b"global\x00" as *const u8 as
                                           *const libc::c_char
                                   } else {
                                       b"field\x00" as *const u8 as
                                           *const libc::c_char
                                   }
                        }
                        6483416627284290920 => {
                            *name =
                                upvalname(p,
                                          (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                               !((!(0i32 as Instruction)) <<
                                                     9i32) << 0i32) as
                                              libc::c_int);
                            return b"upvalue\x00" as *const u8 as
                                       *const libc::c_char
                        }
                        11006700562992250127 => {
                            let mut b_0: libc::c_int =
                                if op as libc::c_uint ==
                                       OP_LOADK as libc::c_int as libc::c_uint
                                   {
                                    (i >> 0i32 + 6i32 + 8i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32) << 0i32) as
                                        libc::c_int
                                } else {
                                    (*(*p).code.offset((pc + 1i32) as isize)
                                         >> 0i32 + 6i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32 + 8i32) << 0i32) as
                                        libc::c_int
                                };
                            if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32 ==
                                   4i32 {
                                if 0 !=
                                       ::std::mem::size_of::<lu_byte>() as
                                           libc::c_ulong {
                                } else {
                                    __assert_fail(b"sizeof((((((((((((&p->k[b]))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (((((((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((&p->k[b])->value_).gc))))->ts))))))->extra)\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32
                                       == 4i32 {
                                } else {
                                    __assert_fail(b"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*(*p).k.offset(b_0 as
                                                         isize)).value_.gc).tt
                                       as libc::c_int & 15i32 == 4i32 {
                                } else {
                                    __assert_fail(b"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                *name =
                                    (&mut (*((*(*p).k.offset(b_0 as
                                                                 isize)).value_.gc
                                                 as *mut GCUnion)).ts as
                                         *mut TString_0 as
                                         *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                                       as
                                                                       libc::c_ulong
                                                                       as
                                                                       isize);
                                return b"constant\x00" as *const u8 as
                                           *const libc::c_char
                            }
                        }
                        _ => {
                            let mut k_0: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            kname(p, pc, k_0, name);
                            return b"method\x00" as *const u8 as
                                       *const libc::c_char
                        }
                    }
                }
                12 => {
                    current_block = 3276175668257526147;
                    match current_block {
                        10680521327981672866 => {
                            let mut b: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            if b <
                                   (i >> 0i32 + 6i32 &
                                        !((!(0i32 as Instruction)) << 8i32) <<
                                            0i32) as libc::c_int {
                                return getobjname(p, pc, b, name)
                            }
                        }
                        820271813250567934 => {
                            let mut k: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut t: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            let mut vn: *const libc::c_char =
                                if op as libc::c_uint ==
                                       OP_GETTABLE as libc::c_int as
                                           libc::c_uint {
                                    luaF_getlocalname(p, t + 1i32, pc)
                                } else { upvalname(p, t) };
                            kname(p, pc, k, name);
                            return if !vn.is_null() &&
                                          strcmp(vn,
                                                 b"_ENV\x00" as *const u8 as
                                                     *const libc::c_char) ==
                                              0i32 {
                                       b"global\x00" as *const u8 as
                                           *const libc::c_char
                                   } else {
                                       b"field\x00" as *const u8 as
                                           *const libc::c_char
                                   }
                        }
                        6483416627284290920 => {
                            *name =
                                upvalname(p,
                                          (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                                               !((!(0i32 as Instruction)) <<
                                                     9i32) << 0i32) as
                                              libc::c_int);
                            return b"upvalue\x00" as *const u8 as
                                       *const libc::c_char
                        }
                        11006700562992250127 => {
                            let mut b_0: libc::c_int =
                                if op as libc::c_uint ==
                                       OP_LOADK as libc::c_int as libc::c_uint
                                   {
                                    (i >> 0i32 + 6i32 + 8i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32) << 0i32) as
                                        libc::c_int
                                } else {
                                    (*(*p).code.offset((pc + 1i32) as isize)
                                         >> 0i32 + 6i32 &
                                         !((!(0i32 as Instruction)) <<
                                               9i32 + 9i32 + 8i32) << 0i32) as
                                        libc::c_int
                                };
                            if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32 ==
                                   4i32 {
                                if 0 !=
                                       ::std::mem::size_of::<lu_byte>() as
                                           libc::c_ulong {
                                } else {
                                    __assert_fail(b"sizeof((((((((((((&p->k[b]))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (((((((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 469, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((&p->k[b])->value_).gc))))->ts))))))->extra)\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*p).k.offset(b_0 as isize)).tt_ & 15i32
                                       == 4i32 {
                                } else {
                                    __assert_fail(b"(((((((&p->k[b]))->tt_)) & 0x0F)) == (4))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                if (*(*(*p).k.offset(b_0 as
                                                         isize)).value_.gc).tt
                                       as libc::c_int & 15i32 == 4i32 {
                                } else {
                                    __assert_fail(b"(((((&p->k[b])->value_).gc)->tt) & 0x0F) == 4\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  469i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 57],
                                                                            &[libc::c_char; 57]>(b"const char *getobjname(Proto *, int, int, const char **)\x00")).as_ptr());
                                };
                                *name =
                                    (&mut (*((*(*p).k.offset(b_0 as
                                                                 isize)).value_.gc
                                                 as *mut GCUnion)).ts as
                                         *mut TString_0 as
                                         *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                                       as
                                                                       libc::c_ulong
                                                                       as
                                                                       isize);
                                return b"constant\x00" as *const u8 as
                                           *const libc::c_char
                            }
                        }
                        _ => {
                            let mut k_0: libc::c_int =
                                (i >> 0i32 + 6i32 + 8i32 &
                                     !((!(0i32 as Instruction)) << 9i32) <<
                                         0i32) as libc::c_int;
                            kname(p, pc, k_0, name);
                            return b"method\x00" as *const u8 as
                                       *const libc::c_char
                        }
                    }
                }
                _ => { }
            }
        }
        return 0 as *const libc::c_char
    };
}
unsafe extern "C" fn kname(mut p: *mut Proto_0, mut pc: libc::c_int,
                           mut c: libc::c_int,
                           mut name: *mut *const libc::c_char) -> () {
    if 0 != c & 1i32 << 9i32 - 1i32 {
        let mut kvalue: *mut TValue =
            &mut *(*p).k.offset((c & !(1i32 << 9i32 - 1i32)) as isize) as
                *mut TValue;
        if (*kvalue).tt_ & 15i32 == 4i32 {
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(b"sizeof((((((((((((kvalue))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((kvalue))->tt_)) & 0x0F)) == (4))\", \"ldebug.c\", 360, __extension__ __PRETTY_FUNCTION__)), (((((((((kvalue)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((kvalue)->value_).gc)->tt) & 0x0F) == 4\", \"ldebug.c\", 360, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((kvalue)->value_).gc))))->ts))))))->extra)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 360i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 45],
                                                        &[libc::c_char; 45]>(b"void kname(Proto *, int, int, const char **)\x00")).as_ptr());
            };
            if (*kvalue).tt_ & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((((kvalue))->tt_)) & 0x0F)) == (4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 360i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 45],
                                                        &[libc::c_char; 45]>(b"void kname(Proto *, int, int, const char **)\x00")).as_ptr());
            };
            if (*(*kvalue).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((kvalue)->value_).gc)->tt) & 0x0F) == 4\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 360i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 45],
                                                        &[libc::c_char; 45]>(b"void kname(Proto *, int, int, const char **)\x00")).as_ptr());
            };
            *name =
                (&mut (*((*kvalue).value_.gc as *mut GCUnion)).ts as
                     *mut TString_0 as
                     *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                   as libc::c_ulong as isize);
            return
        }
    } else {
        let mut what: *const libc::c_char = getobjname(p, pc, c, name);
        if !what.is_null() && *what as libc::c_int == 'c' as i32 { return }
    }
    *name = b"?\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn upvalname(mut p: *mut Proto_0, mut uv: libc::c_int)
 -> *const libc::c_char {
    if uv < (*p).sizeupvalues {
    } else {
        __assert_fail(b"uv < p->sizeupvalues\x00" as *const u8 as
                          *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      129i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"const char *upvalname(Proto *, int)\x00")).as_ptr());
    };
    let mut s: *mut TString = (*(*p).upvalues.offset(uv as isize)).name;
    if s.is_null() {
        return b"?\x00" as *const u8 as *const libc::c_char
    } else {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((s)->extra)\x00" as *const u8 as
                              *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          131i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 36],
                                                    &[libc::c_char; 36]>(b"const char *upvalname(Proto *, int)\x00")).as_ptr());
        };
        return (s as
                    *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                  as libc::c_ulong as isize)
    };
}
unsafe extern "C" fn findsetreg(mut p: *mut Proto_0, mut lastpc: libc::c_int,
                                mut reg: libc::c_int) -> libc::c_int {
    let mut pc: libc::c_int = 0;
    let mut setreg: libc::c_int = -1i32;
    let mut jmptarget: libc::c_int = 0i32;
    pc = 0i32;
    while pc < lastpc {
        let mut i: Instruction = *(*p).code.offset(pc as isize);
        let mut op: OpCode =
            (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as
                OpCode;
        let mut a: libc::c_int =
            (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                as libc::c_int;
        match op as libc::c_uint {
            4 => {
                let mut b: libc::c_int =
                    (i >> 0i32 + 6i32 + 8i32 + 9i32 &
                         !((!(0i32 as Instruction)) << 9i32) << 0i32) as
                        libc::c_int;
                if a <= reg && reg <= a + b {
                    setreg = filterpc(pc, jmptarget)
                }
            }
            41 => { if reg >= a + 2i32 { setreg = filterpc(pc, jmptarget) } }
            36 | 37 => { if reg >= a { setreg = filterpc(pc, jmptarget) } }
            30 => {
                let mut b_0: libc::c_int =
                    (i >> 0i32 + 6i32 + 8i32 &
                         !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                        as libc::c_int -
                        ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32);
                let mut dest: libc::c_int = pc + 1i32 + b_0;
                if pc < dest && dest <= lastpc {
                    if dest > jmptarget { jmptarget = dest }
                }
            }
            _ => {
                if 0 !=
                       luaP_opmodes[op as usize] as libc::c_int & 1i32 << 6i32
                       && reg == a {
                    setreg = filterpc(pc, jmptarget)
                }
            }
        }
        pc += 1
    }
    return setreg;
}
unsafe extern "C" fn filterpc(mut pc: libc::c_int, mut jmptarget: libc::c_int)
 -> libc::c_int {
    if pc < jmptarget { return -1i32 } else { return pc };
}
#[no_mangle]
pub unsafe extern "C" fn currentline(mut ci: *mut CallInfo_0) -> libc::c_int {
    if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      53i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"int currentline(CallInfo *)\x00")).as_ptr());
    };
    if (*(*(*ci).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      53i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"int currentline(CallInfo *)\x00")).as_ptr());
    };
    return if !(*(*(&mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l as
                        *mut LClosure)).p).lineinfo.is_null() {
               if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"ldebug.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 53i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 28],
                                                           &[libc::c_char; 28]>(b"int currentline(CallInfo *)\x00")).as_ptr());
               };
               if (*(*(*ci).func).value_.gc).tt as libc::c_int ==
                      6i32 | 0i32 << 4i32 {
               } else {
                   __assert_fail(b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"ldebug.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 53i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 28],
                                                           &[libc::c_char; 28]>(b"int currentline(CallInfo *)\x00")).as_ptr());
               };
               *(*(*(&mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l as
                         *mut LClosure)).p).lineinfo.offset(currentpc(ci) as
                                                                isize)
           } else { -1i32 };
}
unsafe extern "C" fn funcinfo(mut ar: *mut lua_Debug, mut cl: *mut Closure)
 -> () {
    let mut p: *mut Proto_0 = 0 as *mut Proto_0;
    if cl.is_null() || (*cl).c.tt as libc::c_int == 6i32 | 2i32 << 4i32 {
        (*ar).source = b"=[C]\x00" as *const u8 as *const libc::c_char;
        (*ar).linedefined = -1i32;
        (*ar).lastlinedefined = -1i32;
        (*ar).what = b"C\x00" as *const u8 as *const libc::c_char
    } else {
        p = (*cl).l.p;
        (*ar).source =
            if !(*p).source.is_null() {
                if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
                } else {
                    __assert_fail(b"sizeof((p->source)->extra)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"ldebug.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  221i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 38],
                                                            &[libc::c_char; 38]>(b"void funcinfo(lua_Debug *, Closure *)\x00")).as_ptr());
                };
                ((*p).source as
                     *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                   as libc::c_ulong as isize)
            } else { b"=?\x00" as *const u8 as *const libc::c_char };
        (*ar).linedefined = (*p).linedefined;
        (*ar).lastlinedefined = (*p).lastlinedefined;
        (*ar).what =
            if (*ar).linedefined == 0i32 {
                b"main\x00" as *const u8 as *const libc::c_char
            } else { b"Lua\x00" as *const u8 as *const libc::c_char }
    }
    luaO_chunkid((*ar).short_src.as_mut_ptr(), (*ar).source, 60i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn lua_getlocal(mut L: *mut lua_State_0,
                                      mut ar: *const lua_Debug,
                                      mut n: libc::c_int)
 -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let ref mut fresh6 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
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
        if !((*(*L).top.offset(-1isize)).tt_ ==
                 6i32 | 0i32 << 4i32 | 1i32 << 6i32) {
            name = 0 as *const libc::c_char
        } else {
            if (*(*L).top.offset(-1isize)).tt_ ==
                   6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((L->top - 1))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 180i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
            };
            if (*(*(*L).top.offset(-1isize)).value_.gc).tt as libc::c_int ==
                   6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"(((L->top - 1)->value_).gc)->tt == (6 | (0 << 4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 180i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
            };
            name =
                luaF_getlocalname((*&mut (*((*(*L).top.offset(-1isize)).value_.gc
                                                as *mut GCUnion)).cl.l).p, n,
                                  0i32)
        }
    } else {
        let mut pos: StkId = 0 as StkId;
        name = findlocal(L, (*ar).i_ci, n, &mut pos);
        if !name.is_null() {
            let mut io1: *mut TValue = (*L).top;
            *io1 = *pos;
            if 0 == (*io1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ldebug.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         186i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 62],
                                                                   &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
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
                                                      b"ldebug.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      186i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 62],
                                                                                &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
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
                                  b"ldebug.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  186i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 62],
                                                            &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
                };
            };
            (*L).top = (*L).top.offset(1isize);
            if (*L).top <= (*(*L).ci).top &&
                   !(b"stack overflow\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            } else {
                __assert_fail(b"(L->top <= L->ci->top) && \"stack overflow\"\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 187i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
            };
        }
    }
    swapextra(L);
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
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      191i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"const char *lua_getlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
    };
    return name;
}
unsafe extern "C" fn findlocal(mut L: *mut lua_State_0,
                               mut ci: *mut CallInfo_0, mut n: libc::c_int,
                               mut pos: *mut StkId) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut base: StkId = 0 as *mut TValue;
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
        if n < 0i32 {
            return findvararg(ci, -n, pos)
        } else {
            base = (*ci).u.l.base;
            if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 155i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 61],
                                                        &[libc::c_char; 61]>(b"const char *findlocal(lua_State *, CallInfo *, int, StkId *)\x00")).as_ptr());
            };
            if (*(*(*ci).func).value_.gc).tt as libc::c_int ==
                   6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 155i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 61],
                                                        &[libc::c_char; 61]>(b"const char *findlocal(lua_State *, CallInfo *, int, StkId *)\x00")).as_ptr());
            };
            name =
                luaF_getlocalname((*&mut (*((*(*ci).func).value_.gc as
                                                *mut GCUnion)).cl.l).p, n,
                                  currentpc(ci))
        }
    } else { base = (*ci).func.offset(1isize) }
    if name.is_null() {
        let mut limit: StkId =
            if ci == (*L).ci { (*L).top } else { (*(*ci).next).func };
        if base.offset_to(limit).expect("bad offset_to") as libc::c_long >=
               n as libc::c_long && n > 0i32 {
            name = b"(*temporary)\x00" as *const u8 as *const libc::c_char
        } else { return 0 as *const libc::c_char }
    }
    *pos = base.offset((n - 1i32) as isize);
    return name;
}
unsafe extern "C" fn findvararg(mut ci: *mut CallInfo_0, mut n: libc::c_int,
                                mut pos: *mut StkId) -> *const libc::c_char {
    if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(b"((((ci->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      136i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"const char *findvararg(CallInfo *, int, StkId *)\x00")).as_ptr());
    };
    if (*(*(*ci).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(b"(((ci->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      136i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"const char *findvararg(CallInfo *, int, StkId *)\x00")).as_ptr());
    };
    let mut nparams: libc::c_int =
        (*(*(&mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l as
                 *mut LClosure)).p).numparams as libc::c_int;
    if n >=
           (*ci).func.offset_to((*ci).u.l.base).expect("bad offset_to") as
               libc::c_long as libc::c_int - nparams {
        return 0 as *const libc::c_char
    } else {
        *pos = (*ci).func.offset(nparams as isize).offset(n as isize);
        return b"(*vararg)\x00" as *const u8 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setlocal(mut L: *mut lua_State_0,
                                      mut ar: *const lua_Debug,
                                      mut n: libc::c_int)
 -> *const libc::c_char {
    let mut pos: StkId = 0 as StkId;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
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
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      199i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
    };
    swapextra(L);
    name = findlocal(L, (*ar).i_ci, n, &mut pos);
    if !name.is_null() {
        let mut io1: *mut TValue = pos;
        *io1 = *(*L).top.offset(-1isize);
        if 0 == (*io1).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io1).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ldebug.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     203i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 62],
                                                               &[libc::c_char; 62]>(b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
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
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  203i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 62],
                                                                            &[libc::c_char; 62]>(b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
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
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 203i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"const char *lua_setlocal(lua_State *, const lua_Debug *, int)\x00")).as_ptr());
            };
        };
        (*L).top = (*L).top.offset(-1isize)
    }
    swapextra(L);
    let ref mut fresh11 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
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
pub unsafe extern "C" fn lua_sethook(mut L: *mut lua_State_0,
                                     mut func: lua_Hook,
                                     mut mask: libc::c_int,
                                     mut count: libc::c_int) -> () {
    if func.is_none() || mask == 0i32 { mask = 0i32; func = None }
    if 0 != (*(*L).ci).callstatus as libc::c_int & 1i32 << 1i32 {
        (*L).oldpc = (*(*L).ci).u.l.savedpc
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
pub unsafe extern "C" fn lua_gethookmask(mut L: *mut lua_State_0)
 -> libc::c_int {
    return (*L).hookmask;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gethookcount(mut L: *mut lua_State_0)
 -> libc::c_int {
    return (*L).basehookcount;
}
#[no_mangle]
pub unsafe extern "C" fn luaG_typeerror(mut L: *mut lua_State_0,
                                        mut o: *const TValue,
                                        mut op: *const libc::c_char) -> ! {
    let mut t: *const libc::c_char = luaT_objtypename(L, o);
    luaG_runerror(L,
                  b"attempt to %s a %s value%s\x00" as *const u8 as
                      *const libc::c_char, op, t, varinfo(L, o));
}
unsafe extern "C" fn varinfo(mut L: *mut lua_State_0, mut o: *const TValue)
 -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let mut kind: *const libc::c_char = 0 as *const libc::c_char;
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
        kind = getupvalname(ci, o, &mut name);
        if kind.is_null() && 0 != isinstack(ci, o) {
            if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 579i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 49],
                                                        &[libc::c_char; 49]>(b"const char *varinfo(lua_State *, const TValue *)\x00")).as_ptr());
            };
            if (*(*(*ci).func).value_.gc).tt as libc::c_int ==
                   6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 579i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 49],
                                                        &[libc::c_char; 49]>(b"const char *varinfo(lua_State *, const TValue *)\x00")).as_ptr());
            };
            kind =
                getobjname((*&mut (*((*(*ci).func).value_.gc as
                                         *mut GCUnion)).cl.l).p,
                           currentpc(ci),
                           (*ci).u.l.base.offset_to(o).expect("bad offset_to")
                               as libc::c_long as libc::c_int, &mut name)
        }
    }
    return if !kind.is_null() {
               luaO_pushfstring(L,
                                b" (%s \'%s\')\x00" as *const u8 as
                                    *const libc::c_char, kind, name)
           } else { b"\x00" as *const u8 as *const libc::c_char };
}
unsafe extern "C" fn isinstack(mut ci: *mut CallInfo_0, mut o: *const TValue)
 -> libc::c_int {
    let mut i: ptrdiff_t =
        (*ci).u.l.base.offset_to(o).expect("bad offset_to") as libc::c_long;
    return (0i32 as libc::c_long <= i &&
                i <
                    (*ci).u.l.base.offset_to((*ci).top).expect("bad offset_to")
                        as libc::c_long &&
                (*ci).u.l.base.offset(i as isize) == o as StkId) as
               libc::c_int;
}
unsafe extern "C" fn getupvalname(mut ci: *mut CallInfo_0,
                                  mut o: *const TValue,
                                  mut name: *mut *const libc::c_char)
 -> *const libc::c_char {
    if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      560i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"const char *getupvalname(CallInfo *, const TValue *, const char **)\x00")).as_ptr());
    };
    if (*(*(*ci).func).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                      560i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"const char *getupvalname(CallInfo *, const TValue *, const char **)\x00")).as_ptr());
    };
    let mut c: *mut LClosure =
        &mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l as
            *mut LClosure;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*c).nupvalues as libc::c_int {
        if (*(*c).upvals[i as usize]).v == o as *mut TValue {
            *name = upvalname((*c).p, i);
            return b"upvalue\x00" as *const u8 as *const libc::c_char
        } else { i += 1 }
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn luaG_concaterror(mut L: *mut lua_State_0,
                                          mut p1: *const TValue,
                                          mut p2: *const TValue) -> ! {
    if (*p1).tt_ & 15i32 == 4i32 || (*p1).tt_ & 15i32 == 3i32 { p1 = p2 }
    luaG_typeerror(L, p1,
                   b"concatenate\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_opinterror(mut L: *mut lua_State_0,
                                         mut p1: *const TValue,
                                         mut p2: *const TValue,
                                         mut msg: *const libc::c_char) -> ! {
    let mut temp: lua_Number = 0.;
    if 0 ==
           if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
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
           } else { luaV_tonumber_(p1, &mut temp) } {
        p2 = p1
    }
    luaG_typeerror(L, p2, msg);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_tointerror(mut L: *mut lua_State_0,
                                         mut p1: *const TValue,
                                         mut p2: *const TValue) -> ! {
    let mut temp: lua_Integer = 0;
    if 0 ==
           if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
               if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
               } else {
                   __assert_fail(b"((((p1))->tt_) == ((3 | (1 << 4))))\x00" as
                                     *const u8 as *const libc::c_char,
                                 b"ldebug.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 612i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 66],
                                                           &[libc::c_char; 66]>(b"void luaG_tointerror(lua_State *, const TValue *, const TValue *)\x00")).as_ptr());
               };
               temp = (*p1).value_.i;
               1i32
           } else { luaV_tointeger(p1, &mut temp, 0i32) } {
        p2 = p1
    }
    luaG_runerror(L,
                  b"number%s has no integer representation\x00" as *const u8
                      as *const libc::c_char, varinfo(L, p2));
}
#[no_mangle]
pub unsafe extern "C" fn luaG_ordererror(mut L: *mut lua_State_0,
                                         mut p1: *const TValue,
                                         mut p2: *const TValue) -> ! {
    let mut t1: *const libc::c_char = luaT_objtypename(L, p1);
    let mut t2: *const libc::c_char = luaT_objtypename(L, p2);
    if strcmp(t1, t2) == 0i32 {
        luaG_runerror(L,
                      b"attempt to compare two %s values\x00" as *const u8 as
                          *const libc::c_char, t1);
    } else {
        luaG_runerror(L,
                      b"attempt to compare %s with %s\x00" as *const u8 as
                          *const libc::c_char, t1, t2);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaG_addinfo(mut L: *mut lua_State_0,
                                      mut msg: *const libc::c_char,
                                      mut src: *mut TString,
                                      mut line: libc::c_int)
 -> *const libc::c_char {
    let mut buff: [libc::c_char; 60] = [0; 60];
    if !src.is_null() {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((src)->extra)\x00" as *const u8 as
                              *const libc::c_char,
                          b"ldebug.c\x00" as *const u8 as *const libc::c_char,
                          633i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 68],
                                                    &[libc::c_char; 68]>(b"const char *luaG_addinfo(lua_State *, const char *, TString *, int)\x00")).as_ptr());
        };
        luaO_chunkid(buff.as_mut_ptr(),
                     (src as
                          *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                        as libc::c_ulong as
                                                        isize),
                     60i32 as size_t);
    } else {
        buff[0usize] = '?' as i32 as libc::c_char;
        buff[1usize] = '\u{0}' as i32 as libc::c_char
    }
    return luaO_pushfstring(L,
                            b"%s:%d: %s\x00" as *const u8 as
                                *const libc::c_char, buff.as_mut_ptr(), line,
                            msg);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_errormsg(mut L: *mut lua_State_0) -> ! {
    if (*L).errfunc != 0i32 as libc::c_long {
        let mut errfunc: StkId =
            ((*L).stack as *mut libc::c_char).offset((*L).errfunc as isize) as
                *mut TValue;
        let mut io1: *mut TValue = (*L).top;
        *io1 = *(*L).top.offset(-1isize);
        if 0 == (*io1).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io1).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ldebug.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     644i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 32],
                                                               &[libc::c_char; 32]>(b"void luaG_errormsg(lua_State *)\x00")).as_ptr());
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
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  644i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 32],
                                                                            &[libc::c_char; 32]>(b"void luaG_errormsg(lua_State *)\x00")).as_ptr());
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
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 644i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 32],
                                                        &[libc::c_char; 32]>(b"void luaG_errormsg(lua_State *)\x00")).as_ptr());
            };
        };
        let mut io1_0: *mut TValue = (*L).top.offset(-1isize);
        *io1_0 = *errfunc;
        if 0 == (*io1_0).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ldebug.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     645i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 32],
                                                               &[libc::c_char; 32]>(b"void luaG_errormsg(lua_State *)\x00")).as_ptr());
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
                                                  b"ldebug.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  645i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 32],
                                                                            &[libc::c_char; 32]>(b"void luaG_errormsg(lua_State *)\x00")).as_ptr());
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
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 645i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 32],
                                                        &[libc::c_char; 32]>(b"void luaG_errormsg(lua_State *)\x00")).as_ptr());
            };
        };
        (*L).top = (*L).top.offset(1isize);
        luaD_callnoyield(L, (*L).top.offset(-2isize), 1i32);
    }
    luaD_throw(L, 2i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaG_traceexec(mut L: *mut lua_State_0) -> () {
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let mut mask: lu_byte = (*L).hookmask as lu_byte;
    (*L).hookcount -= 1;
    let mut counthook: libc::c_int =
        ((*L).hookcount == 0i32 && 0 != mask as libc::c_int & 1i32 << 3i32) as
            libc::c_int;
    if 0 != counthook {
        (*L).hookcount = (*L).basehookcount
    } else if 0 == mask as libc::c_int & 1i32 << 2i32 { return }
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 6i32 {
        (*ci).callstatus =
            ((*ci).callstatus as libc::c_int & !(1i32 << 6i32)) as
                libc::c_ushort;
        return
    } else {
        if 0 != counthook { luaD_hook(L, 3i32, -1i32); }
        if 0 != mask as libc::c_int & 1i32 << 2i32 {
            if (*(*ci).func).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((((ci)->func))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 668i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 33],
                                                        &[libc::c_char; 33]>(b"void luaG_traceexec(lua_State *)\x00")).as_ptr());
            };
            if (*(*(*ci).func).value_.gc).tt as libc::c_int ==
                   6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"((((ci)->func)->value_).gc)->tt == (6 | (0 << 4))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ldebug.c\x00" as *const u8 as
                                  *const libc::c_char, 668i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 33],
                                                        &[libc::c_char; 33]>(b"void luaG_traceexec(lua_State *)\x00")).as_ptr());
            };
            let mut p: *mut Proto_0 =
                (*(&mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l as
                       *mut LClosure)).p;
            let mut npc: libc::c_int =
                (*p).code.offset_to((*ci).u.l.savedpc).expect("bad offset_to")
                    as libc::c_long as libc::c_int - 1i32;
            let mut newline: libc::c_int =
                if !(*p).lineinfo.is_null() {
                    *(*p).lineinfo.offset(npc as isize)
                } else { -1i32 };
            if npc == 0i32 || (*ci).u.l.savedpc <= (*L).oldpc ||
                   newline !=
                       if !(*p).lineinfo.is_null() {
                           *(*p).lineinfo.offset(((*p).code.offset_to((*L).oldpc).expect("bad offset_to")
                                                      as libc::c_long as
                                                      libc::c_int - 1i32) as
                                                     isize)
                       } else { -1i32 } {
                luaD_hook(L, 2i32, newline);
            }
        }
        (*L).oldpc = (*ci).u.l.savedpc;
        if (*L).status as libc::c_int == 1i32 {
            if 0 != counthook { (*L).hookcount = 1i32 }
            (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(-1isize);
            (*ci).callstatus =
                ((*ci).callstatus as libc::c_int | 1i32 << 6i32) as
                    libc::c_ushort;
            (*ci).func = (*L).top.offset(-1isize);
            luaD_throw(L, 1i32);
        } else { return; }
    };
}
