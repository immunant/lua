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
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaT_gettm(events: *mut Table_0, event: TMS, ename: *mut TString)
     -> *const TValue;
    #[no_mangle]
    fn luaT_gettmbyobj(L: *mut lua_State_0, o: *const TValue, event: TMS)
     -> *const TValue;
    #[no_mangle]
    fn luaM_realloc_(L: *mut lua_State_0, block: *mut libc::c_void,
                     oldsize: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn luaE_setdebt(g: *mut global_State, debt: l_mem) -> ();
    #[no_mangle]
    fn luaE_freethread(L: *mut lua_State_0, L1: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State_0, func: StkId,
                        nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_pcall(L: *mut lua_State_0, func: Pfunc, u: *mut libc::c_void,
                  oldtop: ptrdiff_t, ef: ptrdiff_t) -> libc::c_int;
    #[no_mangle]
    fn luaD_shrinkstack(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State_0, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaF_freeproto(L: *mut lua_State_0, f: *mut Proto_0) -> ();
    #[no_mangle]
    fn luaS_remove(L: *mut lua_State_0, ts: *mut TString) -> ();
    #[no_mangle]
    fn luaH_free(L: *mut lua_State_0, t: *mut Table_0) -> ();
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaS_resize(L: *mut lua_State_0, newsize: libc::c_int) -> ();
    #[no_mangle]
    fn luaS_clearcache(g: *mut global_State) -> ();
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
pub type Pfunc =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void)
               -> ()>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union UTString {
    dummy: L_Umaxalign,
    tsv: TString,
}
pub type UTString_0 = UTString;
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
pub type Udata_0 = Udata;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union UUdata {
    dummy: L_Umaxalign,
    uv: Udata_0,
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
pub type Proto_0 = Proto;
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
pub type CClosure_0 = CClosure;
pub type LClosure_0 = LClosure;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure_0,
    l: LClosure_0,
}
pub type Table_0 = Table;
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
#[no_mangle]
pub unsafe extern "C" fn luaC_fix(mut L: *mut lua_State_0,
                                  mut o: *mut GCObject) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if (*g).allgc == o {
    } else {
        __assert_fail(b"g->allgc == o\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      197i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 39],
                                                &[libc::c_char; 39]>(b"void luaC_fix(lua_State *, GCObject *)\x00")).as_ptr());
    };
    (*o).marked =
        ((*o).marked as libc::c_int &
             !(1i32 << 0i32 | 1i32 << 1i32) as lu_byte as libc::c_int) as
            lu_byte;
    (*g).allgc = (*o).next;
    (*o).next = (*g).fixedgc;
    (*g).fixedgc = o;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_freeallobjects(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    separatetobefnz(g, 1i32);
    if (*g).finobj.is_null() {
    } else {
        __assert_fail(b"g->finobj == ((void*)0)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      970i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"void luaC_freeallobjects(lua_State *)\x00")).as_ptr());
    };
    callallpendingfinalizers(L);
    if (*g).tobefnz.is_null() {
    } else {
        __assert_fail(b"g->tobefnz == ((void*)0)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      972i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"void luaC_freeallobjects(lua_State *)\x00")).as_ptr());
    };
    (*g).currentwhite = (1i32 << 0i32 | 1i32 << 1i32) as lu_byte;
    (*g).gckind = 0i32 as lu_byte;
    sweeplist(L, &mut (*g).finobj, !(0i32 as lu_mem));
    sweeplist(L, &mut (*g).allgc, !(0i32 as lu_mem));
    sweeplist(L, &mut (*g).fixedgc, !(0i32 as lu_mem));
    if (*g).strt.nuse == 0i32 {
    } else {
        __assert_fail(b"g->strt.nuse == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      978i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"void luaC_freeallobjects(lua_State *)\x00")).as_ptr());
    };
}
unsafe extern "C" fn sweeplist(mut L: *mut lua_State_0,
                               mut p: *mut *mut GCObject, mut count: lu_mem)
 -> *mut *mut GCObject {
    let mut g: *mut global_State = (*L).l_G;
    let mut ow: libc::c_int =
        (*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32);
    let mut white: libc::c_int =
        ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as
            lu_byte as libc::c_int;
    while !(*p).is_null() &&
              {
                  let fresh0 = count;
                  count = count.wrapping_sub(1);
                  fresh0 > 0i32 as libc::c_ulong
              } {
        let mut curr: *mut GCObject = *p;
        let mut marked: libc::c_int = (*curr).marked as libc::c_int;
        if 0 == (marked ^ (1i32 << 0i32 | 1i32 << 1i32)) & ow {
            *p = (*curr).next;
            freeobj(L, curr);
        } else {
            (*curr).marked =
                (marked & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32)) |
                     white) as lu_byte;
            p = &mut (*curr).next as *mut *mut GCObject
        }
    }
    return if (*p).is_null() { 0 as *mut *mut GCObject } else { p };
}
unsafe extern "C" fn freeobj(mut L: *mut lua_State_0, mut o: *mut GCObject)
 -> () {
    match (*o).tt as libc::c_int {
        9 => {
            if (*o).tt as libc::c_int == 9i32 {
            } else {
                __assert_fail(b"(o)->tt == 9\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 699i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            luaF_freeproto(L, &mut (*(o as *mut GCUnion)).p);
        }
        6 => {
            if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 701i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            freeLclosure(L, &mut (*(o as *mut GCUnion)).cl.l);
        }
        38 => {
            if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 705i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            luaM_realloc_(L, o as *mut libc::c_void,
                          (::std::mem::size_of::<CClosure_0>() as
                               libc::c_ulong as libc::c_int +
                               (::std::mem::size_of::<TValue>() as
                                    libc::c_ulong).wrapping_mul(((*&mut (*(o
                                                                               as
                                                                               *mut GCUnion)).cl.c).nupvalues
                                                                     as
                                                                     libc::c_int
                                                                     - 1i32)
                                                                    as
                                                                    libc::c_ulong)
                                   as libc::c_int) as size_t, 0i32 as size_t);
        }
        5 => {
            if (*o).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(b"(o)->tt == 5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 708i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            luaH_free(L, &mut (*(o as *mut GCUnion)).h);
        }
        8 => {
            if (*o).tt as libc::c_int == 8i32 {
            } else {
                __assert_fail(b"(o)->tt == 8\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 709i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            luaE_freethread(L, &mut (*(o as *mut GCUnion)).th);
        }
        7 => {
            if (*o).tt as libc::c_int == 7i32 {
            } else {
                __assert_fail(b"(o)->tt == 7\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 710i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            luaM_realloc_(L, o as *mut libc::c_void,
                          (::std::mem::size_of::<UUdata>() as
                               libc::c_ulong).wrapping_add((*&mut (*(o as
                                                                         *mut GCUnion)).u).len),
                          0i32 as size_t);
        }
        4 => {
            if (*o).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 712i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            luaS_remove(L, &mut (*(o as *mut GCUnion)).ts);
            if (*o).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 713i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            luaM_realloc_(L, o as *mut libc::c_void,
                          (::std::mem::size_of::<UTString>() as
                               libc::c_ulong).wrapping_add((((*&mut (*(o as
                                                                           *mut GCUnion)).ts).shrlen
                                                                 as
                                                                 libc::c_int +
                                                                 1i32) as
                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                as
                                                                                                libc::c_ulong)),
                          0i32 as size_t);
        }
        20 => {
            if (*o).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 716i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
            luaM_realloc_(L, o as *mut libc::c_void,
                          (::std::mem::size_of::<UTString>() as
                               libc::c_ulong).wrapping_add((*&mut (*(o as
                                                                         *mut GCUnion)).ts).u.lnglen.wrapping_add(1i32
                                                                                                                      as
                                                                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong)),
                          0i32 as size_t);
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 719i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 38],
                                                        &[libc::c_char; 38]>(b"void freeobj(lua_State *, GCObject *)\x00")).as_ptr());
            };
        }
    };
}
unsafe extern "C" fn freeLclosure(mut L: *mut lua_State_0,
                                  mut cl: *mut LClosure_0) -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv: *mut UpVal = (*cl).upvals[i as usize];
        if !uv.is_null() { luaC_upvdeccount(L, uv); }
        i += 1
    }
    luaM_realloc_(L, cl as *mut libc::c_void,
                  (::std::mem::size_of::<LClosure_0>() as libc::c_ulong as
                       libc::c_int +
                       (::std::mem::size_of::<*mut TValue>() as
                            libc::c_ulong).wrapping_mul(((*cl).nupvalues as
                                                             libc::c_int -
                                                             1i32) as
                                                            libc::c_ulong) as
                           libc::c_int) as size_t, 0i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_upvdeccount(mut L: *mut lua_State_0,
                                          mut uv: *mut UpVal) -> () {
    if (*uv).refcount > 0i32 as libc::c_ulong {
    } else {
        __assert_fail(b"uv->refcount > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      679i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"void luaC_upvdeccount(lua_State *, UpVal *)\x00")).as_ptr());
    };
    (*uv).refcount = (*uv).refcount.wrapping_sub(1);
    if (*uv).refcount == 0i32 as libc::c_ulong &&
           !((*uv).v != &mut (*uv).u.value as *mut TValue) {
        luaM_realloc_(L, uv as *mut libc::c_void,
                      ::std::mem::size_of::<UpVal>() as libc::c_ulong,
                      0i32 as size_t);
    };
}
unsafe extern "C" fn callallpendingfinalizers(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    while !(*g).tobefnz.is_null() { GCTM(L, 0i32); };
}
unsafe extern "C" fn GCTM(mut L: *mut lua_State_0,
                          mut propagateerrors: libc::c_int) -> () {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut g: *mut global_State = (*L).l_G;
    let mut tm: *const TValue = 0 as *const TValue;
    let mut v: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut io: *mut TValue = &mut v;
    let mut i_g: *mut GCObject = udata2finalize(g);
    (*io).value_.gc = i_g;
    (*io).tt_ = (*i_g).tt as libc::c_int | 1i32 << 6i32;
    tm = luaT_gettmbyobj(L, &mut v, TM_GC);
    if !tm.is_null() && (*tm).tt_ & 15i32 == 6i32 {
        let mut status: libc::c_int = 0;
        let mut oldah: lu_byte = (*L).allowhook;
        let mut running: libc::c_int = (*g).gcrunning as libc::c_int;
        (*L).allowhook = 0i32 as lu_byte;
        (*g).gcrunning = 0i32 as lu_byte;
        let mut io1: *mut TValue = (*L).top;
        *io1 = *tm;
        if 0 == (*io1).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io1).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     819i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 28],
                                                               &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
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
                                                  b"lgc.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  819i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 28],
                                                                            &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
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
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 819i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 28],
                                                        &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
            };
        };
        let mut io1_0: *mut TValue = (*L).top.offset(1isize);
        *io1_0 = v;
        if 0 == (*io1_0).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     820i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 28],
                                                               &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
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
                                                  b"lgc.c\x00" as *const u8 as
                                                      *const libc::c_char,
                                                  820i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 28],
                                                                            &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
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
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 820i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 28],
                                                        &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
            };
        };
        (*L).top = (*L).top.offset(2isize);
        (*(*L).ci).callstatus =
            ((*(*L).ci).callstatus as libc::c_int | 1i32 << 8i32) as
                libc::c_ushort;
        status =
            luaD_pcall(L, Some(dothecall), 0 as *mut libc::c_void,
                       ((*L).stack as
                            *mut libc::c_char).offset_to((*L).top.offset(-2isize)
                                                             as
                                                             *mut libc::c_char).expect("bad offset_to")
                           as libc::c_long, 0i32 as ptrdiff_t);
        (*(*L).ci).callstatus =
            ((*(*L).ci).callstatus as libc::c_int & !(1i32 << 8i32)) as
                libc::c_ushort;
        (*L).allowhook = oldah;
        (*g).gcrunning = running as lu_byte;
        if status != 0i32 && 0 != propagateerrors {
            if status == 2i32 {
                msg =
                    if (*(*L).top.offset(-1isize)).tt_ & 15i32 == 4i32 {
                        if 0 !=
                               ::std::mem::size_of::<lu_byte>() as
                                   libc::c_ulong {
                        } else {
                            __assert_fail(b"sizeof((((((((((((L->top - 1))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((L->top - 1))->tt_)) & 0x0F)) == (4))\", \"lgc.c\", 830, __extension__ __PRETTY_FUNCTION__)), (((((((((L->top - 1)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((L->top - 1)->value_).gc)->tt) & 0x0F) == 4\", \"lgc.c\", 830, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((L->top - 1)->value_).gc))))->ts))))))->extra)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          830i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 28],
                                                                    &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
                        };
                        if (*(*L).top.offset(-1isize)).tt_ & 15i32 == 4i32 {
                        } else {
                            __assert_fail(b"(((((((L->top - 1))->tt_)) & 0x0F)) == (4))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          830i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 28],
                                                                    &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
                        };
                        if (*(*(*L).top.offset(-1isize)).value_.gc).tt as
                               libc::c_int & 15i32 == 4i32 {
                        } else {
                            __assert_fail(b"(((((L->top - 1)->value_).gc)->tt) & 0x0F) == 4\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          830i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 28],
                                                                    &[libc::c_char; 28]>(b"void GCTM(lua_State *, int)\x00")).as_ptr());
                        };
                        (&mut (*((*(*L).top.offset(-1isize)).value_.gc as
                                     *mut GCUnion)).ts as *mut TString_0 as
                             *mut libc::c_char).offset(::std::mem::size_of::<UTString_0>()
                                                           as libc::c_ulong as
                                                           isize)
                    } else {
                        b"no message\x00" as *const u8 as *const libc::c_char
                    };
                luaO_pushfstring(L,
                                 b"error in __gc metamethod (%s)\x00" as
                                     *const u8 as *const libc::c_char, msg);
                status = 5i32
            }
            luaD_throw(L, status);
        }
    };
}
unsafe extern "C" fn dothecall(mut L: *mut lua_State_0,
                               mut ud: *mut libc::c_void) -> () {
    ud = 0 as *mut libc::c_void;
    luaD_callnoyield(L, (*L).top.offset(-2isize), 0i32);
}
unsafe extern "C" fn udata2finalize(mut g: *mut global_State)
 -> *mut GCObject {
    let mut o: *mut GCObject = (*g).tobefnz;
    if 0 != (*o).marked as libc::c_int & 1i32 << 3i32 {
    } else {
        __assert_fail(b"(((o)->marked) & ((1<<(3))))\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      790i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"GCObject *udata2finalize(global_State *)\x00")).as_ptr());
    };
    (*g).tobefnz = (*o).next;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    (*o).marked =
        ((*o).marked as libc::c_int &
             !(1i32 << 3i32) as lu_byte as libc::c_int) as lu_byte;
    if 2i32 <= (*g).gcstate as libc::c_int &&
           (*g).gcstate as libc::c_int <= 5i32 {
        (*o).marked =
            ((*o).marked as libc::c_int &
                 !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32)) |
                 ((*g).currentwhite as libc::c_int &
                      (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte as
                     libc::c_int) as lu_byte
    }
    return o;
}
unsafe extern "C" fn separatetobefnz(mut g: *mut global_State,
                                     mut all: libc::c_int) -> () {
    let mut curr: *mut GCObject = 0 as *mut GCObject;
    let mut p: *mut *mut GCObject = &mut (*g).finobj as *mut *mut GCObject;
    let mut lastnext: *mut *mut GCObject = findlast(&mut (*g).tobefnz);
    loop  {
        curr = *p;
        if curr.is_null() { break ; }
        if 0 != (*curr).marked as libc::c_int & 1i32 << 3i32 {
        } else {
            __assert_fail(b"(((curr)->marked) & ((1<<(3))))\x00" as *const u8
                              as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          885i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 42],
                                                    &[libc::c_char; 42]>(b"void separatetobefnz(global_State *, int)\x00")).as_ptr());
        };
        if !(0 !=
                 (*curr).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                 || 0 != all) {
            p = &mut (*curr).next as *mut *mut GCObject
        } else {
            *p = (*curr).next;
            (*curr).next = *lastnext;
            *lastnext = curr;
            lastnext = &mut (*curr).next as *mut *mut GCObject
        }
    };
}
unsafe extern "C" fn findlast(mut p: *mut *mut GCObject)
 -> *mut *mut GCObject {
    while !(*p).is_null() { p = &mut (**p).next as *mut *mut GCObject }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_step(mut L: *mut lua_State_0) -> () {
    let mut work: lu_mem = 0;
    let mut g: *mut global_State = (*L).l_G;
    let mut debt: l_mem = getdebt(g);
    if 0 == (*g).gcrunning {
        luaE_setdebt(g,
                     (-((100i32 as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<TString>()
                                                             as libc::c_ulong)
                            as libc::c_int) * 10i32) as l_mem);
        return
    } else {
        loop  {
            work = singlestep(L);
            debt =
                (debt as libc::c_ulong).wrapping_sub(work) as l_mem as l_mem;
            if !(debt >
                     -((100i32 as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<TString>()
                                                            as libc::c_ulong)
                           as libc::c_int) as libc::c_long &&
                     (*g).gcstate as libc::c_int != 7i32) {
                break ;
            }
        }
        if (*g).gcstate as libc::c_int == 7i32 {
            setpause(g);
        } else {
            debt =
                debt / (*g).gcstepmul as libc::c_long *
                    200i32 as libc::c_long;
            luaE_setdebt(g, debt);
            runafewfinalizers(L);
        }
        return;
    };
}
unsafe extern "C" fn runafewfinalizers(mut L: *mut lua_State_0)
 -> libc::c_int {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: libc::c_uint = 0;
    if (*g).tobefnz.is_null() || (*g).gcfinnum > 0i32 as libc::c_uint {
    } else {
        __assert_fail(b"!g->tobefnz || g->gcfinnum > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      847i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"int runafewfinalizers(lua_State *)\x00")).as_ptr());
    };
    i = 0i32 as libc::c_uint;
    while !(*g).tobefnz.is_null() && i < (*g).gcfinnum {
        GCTM(L, 1i32);
        i = i.wrapping_add(1)
    }
    (*g).gcfinnum =
        if (*g).tobefnz.is_null() {
            0i32 as libc::c_uint
        } else { (*g).gcfinnum.wrapping_mul(2i32 as libc::c_uint) };
    return i as libc::c_int;
}
unsafe extern "C" fn getdebt(mut g: *mut global_State) -> l_mem {
    let mut debt: l_mem = (*g).GCdebt;
    let mut stepmul: libc::c_int = (*g).gcstepmul;
    if debt <= 0i32 as libc::c_long {
        return 0i32 as l_mem
    } else {
        debt = debt / 200i32 as libc::c_long + 1i32 as libc::c_long;
        debt =
            if debt <
                   (!(0i32 as lu_mem) >> 1i32) as l_mem /
                       stepmul as libc::c_long {
                debt * stepmul as libc::c_long
            } else { (!(0i32 as lu_mem) >> 1i32) as l_mem };
        return debt
    };
}
unsafe extern "C" fn setpause(mut g: *mut global_State) -> () {
    let mut threshold: l_mem = 0;
    let mut debt: l_mem = 0;
    let mut estimate: l_mem =
        (*g).GCestimate.wrapping_div(100i32 as libc::c_ulong) as l_mem;
    if estimate > 0i32 as libc::c_long {
    } else {
        __assert_fail(b"estimate > 0\x00" as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      943i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void setpause(global_State *)\x00")).as_ptr());
    };
    threshold =
        if ((*g).gcpause as libc::c_long) <
               (!(0i32 as lu_mem) >> 1i32) as l_mem / estimate {
            estimate * (*g).gcpause as libc::c_long
        } else { (!(0i32 as lu_mem) >> 1i32) as l_mem };
    debt =
        (((*g).totalbytes + (*g).GCdebt) as
             lu_mem).wrapping_sub(threshold as libc::c_ulong) as l_mem;
    luaE_setdebt(g, debt);
}
unsafe extern "C" fn singlestep(mut L: *mut lua_State_0) -> lu_mem {
    let mut g: *mut global_State = (*L).l_G;
    match (*g).gcstate as libc::c_int {
        7 => {
            (*g).GCmemtrav =
                ((*g).strt.size as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut GCObject>()
                                                     as libc::c_ulong);
            restartcollection(g);
            (*g).gcstate = 0i32 as lu_byte;
            return (*g).GCmemtrav
        }
        0 => {
            (*g).GCmemtrav = 0i32 as lu_mem;
            if !(*g).gray.is_null() {
            } else {
                __assert_fail(b"g->gray\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1056i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 31],
                                                        &[libc::c_char; 31]>(b"lu_mem singlestep(lua_State *)\x00")).as_ptr());
            };
            propagatemark(g);
            if (*g).gray.is_null() { (*g).gcstate = 1i32 as lu_byte }
            return (*g).GCmemtrav
        }
        1 => {
            let mut work: lu_mem = 0;
            propagateall(g);
            work = atomic(L) as lu_mem;
            entersweep(L);
            (*g).GCestimate = ((*g).totalbytes + (*g).GCdebt) as lu_mem;
            return work
        }
        2 => { return sweepstep(L, g, 3i32, &mut (*g).finobj) }
        3 => { return sweepstep(L, g, 4i32, &mut (*g).tobefnz) }
        4 => { return sweepstep(L, g, 5i32, 0 as *mut *mut GCObject) }
        5 => {
            (*(*g).mainthread).marked =
                ((*(*g).mainthread).marked as libc::c_int &
                     !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32)) |
                     ((*g).currentwhite as libc::c_int &
                          (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte as
                         libc::c_int) as lu_byte;
            checkSizes(L, g);
            (*g).gcstate = 6i32 as lu_byte;
            return 0i32 as lu_mem
        }
        6 => {
            if !(*g).tobefnz.is_null() && (*g).gckind as libc::c_int != 1i32 {
                let mut n: libc::c_int = runafewfinalizers(L);
                return (n as
                            libc::c_ulong).wrapping_mul((::std::mem::size_of::<TString>()
                                                             as
                                                             libc::c_ulong).wrapping_add(4i32
                                                                                             as
                                                                                             libc::c_ulong).wrapping_div(4i32
                                                                                                                             as
                                                                                                                             libc::c_ulong))
            } else { (*g).gcstate = 7i32 as lu_byte; return 0i32 as lu_mem }
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1095i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 31],
                                                        &[libc::c_char; 31]>(b"lu_mem singlestep(lua_State *)\x00")).as_ptr());
            };
            return 0i32 as lu_mem
        }
    };
}
unsafe extern "C" fn checkSizes(mut L: *mut lua_State_0,
                                mut g: *mut global_State) -> () {
    if (*g).gckind as libc::c_int != 1i32 {
        let mut olddebt: l_mem = (*g).GCdebt;
        if (*g).strt.nuse < (*g).strt.size / 4i32 {
            luaS_resize(L, (*g).strt.size / 2i32);
        }
        (*g).GCestimate =
            ((*g).GCestimate as
                 libc::c_ulong).wrapping_add(((*g).GCdebt - olddebt) as
                                                 libc::c_ulong) as lu_mem as
                lu_mem
    };
}
unsafe extern "C" fn sweepstep(mut L: *mut lua_State_0,
                               mut g: *mut global_State,
                               mut nextstate: libc::c_int,
                               mut nextlist: *mut *mut GCObject) -> lu_mem {
    if !(*g).sweepgc.is_null() {
        let mut olddebt: l_mem = (*g).GCdebt;
        (*g).sweepgc =
            sweeplist(L, (*g).sweepgc,
                      ((100i32 as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<TString>()
                                                            as libc::c_ulong)
                           as libc::c_int as
                           libc::c_ulong).wrapping_div((::std::mem::size_of::<TString>()
                                                            as
                                                            libc::c_ulong).wrapping_add(4i32
                                                                                            as
                                                                                            libc::c_ulong).wrapping_div(4i32
                                                                                                                            as
                                                                                                                            libc::c_ulong)).wrapping_div(4i32
                                                                                                                                                             as
                                                                                                                                                             libc::c_ulong)
                          as libc::c_int as lu_mem);
        (*g).GCestimate =
            ((*g).GCestimate as
                 libc::c_ulong).wrapping_add(((*g).GCdebt - olddebt) as
                                                 libc::c_ulong) as lu_mem as
                lu_mem;
        if !(*g).sweepgc.is_null() {
            return (((100i32 as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<TString>()
                                                          as libc::c_ulong) as
                         libc::c_int as
                         libc::c_ulong).wrapping_div((::std::mem::size_of::<TString>()
                                                          as
                                                          libc::c_ulong).wrapping_add(4i32
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(4i32
                                                                                                                          as
                                                                                                                          libc::c_ulong)).wrapping_div(4i32
                                                                                                                                                           as
                                                                                                                                                           libc::c_ulong)
                        as libc::c_int as
                        libc::c_ulong).wrapping_mul((::std::mem::size_of::<TString>()
                                                         as
                                                         libc::c_ulong).wrapping_add(4i32
                                                                                         as
                                                                                         libc::c_ulong).wrapping_div(4i32
                                                                                                                         as
                                                                                                                         libc::c_ulong))
        }
    }
    (*g).gcstate = nextstate as lu_byte;
    (*g).sweepgc = nextlist;
    return 0i32 as lu_mem;
}
unsafe extern "C" fn entersweep(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    (*g).gcstate = 2i32 as lu_byte;
    if (*g).sweepgc.is_null() {
    } else {
        __assert_fail(b"g->sweepgc == ((void*)0)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      962i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"void entersweep(lua_State *)\x00")).as_ptr());
    };
    (*g).sweepgc = sweeplist(L, &mut (*g).allgc, 1i32 as lu_mem);
}
unsafe extern "C" fn atomic(mut L: *mut lua_State_0) -> l_mem {
    let mut g: *mut global_State = (*L).l_G;
    let mut work: l_mem = 0;
    let mut origweak: *mut GCObject = 0 as *mut GCObject;
    let mut origall: *mut GCObject = 0 as *mut GCObject;
    let mut grayagain: *mut GCObject = (*g).grayagain;
    if (*g).ephemeron.is_null() && (*g).weak.is_null() {
    } else {
        __assert_fail(b"g->ephemeron == ((void*)0) && g->weak == ((void*)0)\x00"
                          as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      987i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"l_mem atomic(lua_State *)\x00")).as_ptr());
    };
    if 0 ==
           (*(*g).mainthread).marked as libc::c_int &
               (1i32 << 0i32 | 1i32 << 1i32) {
    } else {
        __assert_fail(b"!(((g->mainthread)->marked) & (((1<<(0)) | (1<<(1)))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      988i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"l_mem atomic(lua_State *)\x00")).as_ptr());
    };
    (*g).gcstate = (7i32 + 1i32) as lu_byte;
    (*g).GCmemtrav = 0i32 as lu_mem;
    if 0 != (*L).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
        if (*L).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((L)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          991i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 26],
                                                    &[libc::c_char; 26]>(b"l_mem atomic(lua_State *)\x00")).as_ptr());
        };
        reallymarkobject(g, &mut (*(L as *mut GCUnion)).gc);
    }
    if 0 == (*g).l_registry.tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((&g->l_registry)->tt_) & (1 << 6))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 993i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 26],
                                                           &[libc::c_char; 26]>(b"l_mem atomic(lua_State *)\x00")).as_ptr());
               };
               (*g).l_registry.tt_ & 63i32 ==
                   (*(*g).l_registry.value_.gc).tt as libc::c_int
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          993i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 26],
                                                    &[libc::c_char; 26]>(b"l_mem atomic(lua_State *)\x00")).as_ptr());
        };
    };
    if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 &&
           {
               if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((&g->l_registry)->tt_) & (1 << 6))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 993i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 26],
                                                           &[libc::c_char; 26]>(b"l_mem atomic(lua_State *)\x00")).as_ptr());
               };
               0 !=
                   (*(*g).l_registry.value_.gc).marked as libc::c_int &
                       (1i32 << 0i32 | 1i32 << 1i32)
           } {
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as
                              *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          993i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 26],
                                                    &[libc::c_char; 26]>(b"l_mem atomic(lua_State *)\x00")).as_ptr());
        };
        reallymarkobject(g, (*g).l_registry.value_.gc);
    }
    markmt(g);
    remarkupvals(g);
    propagateall(g);
    work = (*g).GCmemtrav as l_mem;
    (*g).gray = grayagain;
    propagateall(g);
    (*g).GCmemtrav = 0i32 as lu_mem;
    convergeephemerons(g);
    clearvalues(g, (*g).weak, 0 as *mut GCObject);
    clearvalues(g, (*g).allweak, 0 as *mut GCObject);
    origweak = (*g).weak;
    origall = (*g).allweak;
    work =
        (work as libc::c_ulong).wrapping_add((*g).GCmemtrav) as l_mem as
            l_mem;
    separatetobefnz(g, 0i32);
    (*g).gcfinnum = 1i32 as libc::c_uint;
    markbeingfnz(g);
    propagateall(g);
    (*g).GCmemtrav = 0i32 as lu_mem;
    convergeephemerons(g);
    clearkeys(g, (*g).ephemeron, 0 as *mut GCObject);
    clearkeys(g, (*g).allweak, 0 as *mut GCObject);
    clearvalues(g, (*g).weak, origweak);
    clearvalues(g, (*g).allweak, origall);
    luaS_clearcache(g);
    (*g).currentwhite =
        ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32)) as
            lu_byte;
    work =
        (work as libc::c_ulong).wrapping_add((*g).GCmemtrav) as l_mem as
            l_mem;
    return work;
}
unsafe extern "C" fn clearvalues(mut g: *mut global_State,
                                 mut l: *mut GCObject, mut f: *mut GCObject)
 -> () {
    while l != f {
        if (*l).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(b"(l)->tt == 5\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          660i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void clearvalues(global_State *, GCObject *, GCObject *)\x00")).as_ptr());
        };
        let mut h: *mut Table_0 = &mut (*(l as *mut GCUnion)).h as *mut Table;
        let mut n: *mut Node = 0 as *mut Node;
        let mut limit: *mut Node =
            &mut *(*h).node.offset((1i32 << (*h).lsizenode as libc::c_int) as
                                       size_t as isize) as *mut Node;
        let mut i: libc::c_uint = 0;
        i = 0i32 as libc::c_uint;
        while i < (*h).sizearray {
            let mut o: *mut TValue =
                &mut *(*h).array.offset(i as isize) as *mut TValue;
            if 0 != iscleared(g, o) { (*o).tt_ = 0i32 }
            i = i.wrapping_add(1)
        }
        n = &mut *(*h).node.offset(0isize) as *mut Node;
        while n < limit {
            if !((*n).i_val.tt_ == 0i32) && 0 != iscleared(g, &mut (*n).i_val)
               {
                (*n).i_val.tt_ = 0i32;
                removeentry(n);
            }
            n = n.offset(1isize)
        }
        if (*l).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(b"(l)->tt == 5\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          659i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void clearvalues(global_State *, GCObject *, GCObject *)\x00")).as_ptr());
        };
        l = (*(&mut (*(l as *mut GCUnion)).h as *mut Table)).gclist
    };
}
unsafe extern "C" fn removeentry(mut n: *mut Node) -> () {
    if (*n).i_val.tt_ == 0i32 {
    } else {
        __assert_fail(b"(((((&(n)->i_val)))->tt_) == (0))\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      127i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void removeentry(Node *)\x00")).as_ptr());
    };
    if 0 !=
           (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ &
               1i32 << 6i32 &&
           {
               if 0 !=
                      (*(&mut (*n).i_key.tvk as *mut TValue as
                             *const TValue)).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 128i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 25],
                                                           &[libc::c_char; 25]>(b"void removeentry(Node *)\x00")).as_ptr());
               };
               0 !=
                   (*(*(&mut (*n).i_key.tvk as *mut TValue as
                            *const TValue)).value_.gc).marked as libc::c_int &
                       (1i32 << 0i32 | 1i32 << 1i32)
           } {
        (*n).i_key.nk.tt_ = 9i32 + 1i32
    };
}
unsafe extern "C" fn iscleared(mut g: *mut global_State, mut o: *const TValue)
 -> libc::c_int {
    if 0 == (*o).tt_ & 1i32 << 6i32 {
        return 0i32
    } else if (*o).tt_ & 15i32 == 4i32 {
        if (*o).tt_ & 15i32 == 4i32 {
        } else {
            __assert_fail(b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as
                              *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          143i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
        };
        if (*(*o).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
        } else {
            __assert_fail(b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as
                              *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          143i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
        };
        if 0 !=
               (*(&mut (*((*o).value_.gc as *mut GCUnion)).ts as
                      *mut TString_0)).marked as libc::c_int &
                   (1i32 << 0i32 | 1i32 << 1i32) {
            if (*o).tt_ & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 143i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
            };
            if (*(*o).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 143i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
            };
            if (*(&mut (*((*o).value_.gc as *mut GCUnion)).ts as
                      *mut TString_0)).tt as libc::c_int & 15i32 < 9i32 + 1i32
               {
            } else {
                __assert_fail(b"(((((((((((((o))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((o))->tt_)) & 0x0F)) == (4))\", \"lgc.c\", 143, __extension__ __PRETTY_FUNCTION__)), (((((((((o)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((o)->value_).gc)->tt) & 0x0F) == 4\", \"lgc.c\", 143, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((o)->value_).gc))))->ts))))))->tt) & 0x0F) < (9+1)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 143i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
            };
            if (*o).tt_ & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 143i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
            };
            if (*(*o).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 143i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
            };
            reallymarkobject(g,
                             &mut (*(&mut (*((*o).value_.gc as
                                                 *mut GCUnion)).ts as
                                         *mut TString_0 as *mut GCUnion)).gc);
        }
        return 0i32
    } else {
        if 0 != (*o).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          146i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"int iscleared(global_State *, const TValue *)\x00")).as_ptr());
        };
        return (*(*o).value_.gc).marked as libc::c_int &
                   (1i32 << 0i32 | 1i32 << 1i32)
    };
}
unsafe extern "C" fn reallymarkobject(mut g: *mut global_State,
                                      mut o: *mut GCObject) -> () {
    loop  {
        (*o).marked =
            ((*o).marked as libc::c_int &
                 !(1i32 << 0i32 | 1i32 << 1i32) as lu_byte as libc::c_int) as
                lu_byte;
        match (*o).tt as libc::c_int {
            4 => {
                (*o).marked =
                    ((*o).marked as libc::c_int | 1i32 << 2i32) as lu_byte;
                if (*o).tt as libc::c_int & 15i32 == 4i32 {
                } else {
                    __assert_fail(b"(((o)->tt) & 0x0F) == 4\x00" as *const u8
                                      as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  242i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                (*g).GCmemtrav =
                    ((*g).GCmemtrav as
                         libc::c_ulong).wrapping_add((::std::mem::size_of::<UTString>()
                                                          as
                                                          libc::c_ulong).wrapping_add((((*(&mut (*(o
                                                                                                       as
                                                                                                       *mut GCUnion)).ts
                                                                                               as
                                                                                               *mut TString_0)).shrlen
                                                                                            as
                                                                                            libc::c_int
                                                                                            +
                                                                                            1i32)
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                                           as
                                                                                                                           libc::c_ulong)))
                        as lu_mem as lu_mem;
                break ;
            }
            20 => {
                (*o).marked =
                    ((*o).marked as libc::c_int | 1i32 << 2i32) as lu_byte;
                if (*o).tt as libc::c_int & 15i32 == 4i32 {
                } else {
                    __assert_fail(b"(((o)->tt) & 0x0F) == 4\x00" as *const u8
                                      as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  247i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                (*g).GCmemtrav =
                    ((*g).GCmemtrav as
                         libc::c_ulong).wrapping_add((::std::mem::size_of::<UTString>()
                                                          as
                                                          libc::c_ulong).wrapping_add((*(&mut (*(o
                                                                                                     as
                                                                                                     *mut GCUnion)).ts
                                                                                             as
                                                                                             *mut TString_0)).u.lnglen.wrapping_add(1i32
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong)))
                        as lu_mem as lu_mem;
                break ;
            }
            7 => {
                let mut uvalue: TValue =
                    lua_TValue{value_:
                                   Value_0{gc:
                                               0 as *const GCObject as
                                                   *mut GCObject,},
                               tt_: 0,};
                if (*o).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(b"(o)->tt == 7\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  252i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if !(*(&mut (*(o as *mut GCUnion)).u as
                           *mut Udata)).metatable.is_null() {
                    if (*o).tt as libc::c_int == 7i32 {
                    } else {
                        __assert_fail(b"(o)->tt == 7\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"lgc.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      252i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 50],
                                                                &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                    };
                    if 0 !=
                           (*(*(&mut (*(o as *mut GCUnion)).u as
                                    *mut Udata)).metatable).marked as
                               libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
                        if (*o).tt as libc::c_int == 7i32 {
                        } else {
                            __assert_fail(b"(o)->tt == 7\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          252i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 50],
                                                                    &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                        };
                        if (*(*(&mut (*(o as *mut GCUnion)).u as
                                    *mut Udata)).metatable).tt as libc::c_int
                               & 15i32 < 9i32 + 1i32 {
                        } else {
                            __assert_fail(b"(((((((o)->tt == 7) ? (void) (0) : __assert_fail (\"(o)->tt == 7\", \"lgc.c\", 252, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->u)))->metatable)->tt) & 0x0F) < (9+1)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          252i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 50],
                                                                    &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                        };
                        if (*o).tt as libc::c_int == 7i32 {
                        } else {
                            __assert_fail(b"(o)->tt == 7\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          252i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 50],
                                                                    &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                        };
                        reallymarkobject(g,
                                         &mut (*((*(&mut (*(o as
                                                                *mut GCUnion)).u
                                                        as
                                                        *mut Udata)).metatable
                                                     as *mut GCUnion)).gc);
                    }
                }
                (*o).marked =
                    ((*o).marked as libc::c_int | 1i32 << 2i32) as lu_byte;
                if (*o).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(b"(o)->tt == 7\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  254i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                (*g).GCmemtrav =
                    ((*g).GCmemtrav as
                         libc::c_ulong).wrapping_add((::std::mem::size_of::<UUdata>()
                                                          as
                                                          libc::c_ulong).wrapping_add((*(&mut (*(o
                                                                                                     as
                                                                                                     *mut GCUnion)).u
                                                                                             as
                                                                                             *mut Udata)).len))
                        as lu_mem as lu_mem;
                let mut io: *mut TValue = &mut uvalue;
                if (*o).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(b"(o)->tt == 7\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  255i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                let mut iu: *const Udata_0 =
                    &mut (*(o as *mut GCUnion)).u as *mut Udata;
                (*io).value_ = (*iu).user_;
                (*io).tt_ = (*iu).ttuv_ as libc::c_int;
                if 0 == (*io).tt_ & 1i32 << 6i32 ||
                       {
                           if 0 != (*io).tt_ & 1i32 << 6i32 {
                           } else {
                               __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lgc.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             255i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 50],
                                                                       &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                           };
                           (*io).tt_ & 63i32 ==
                               (*(*io).value_.gc).tt as libc::c_int &&
                               ((*g).mainthread.is_null() ||
                                    {
                                        if 0 != (*io).tt_ & 1i32 << 6i32 {
                                        } else {
                                            __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          b"lgc.c\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          255i32 as
                                                              libc::c_uint,
                                                          (*::std::mem::transmute::<&[u8; 50],
                                                                                    &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                                        };
                                        0 !=
                                            ((*(*io).value_.gc).marked as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32)) &
                                                ((*(*(*g).mainthread).l_G).currentwhite
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
                                      b"lgc.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      255i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 50],
                                                                &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                    };
                };
                if !(0 != uvalue.tt_ & 1i32 << 6i32 &&
                         {
                             if 0 != uvalue.tt_ & 1i32 << 6i32 {
                             } else {
                                 __assert_fail(b"(((&uvalue)->tt_) & (1 << 6))\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               b"lgc.c\x00" as *const u8 as
                                                   *const libc::c_char,
                                               256i32 as libc::c_uint,
                                               (*::std::mem::transmute::<&[u8; 50],
                                                                         &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                             };
                             0 !=
                                 (*uvalue.value_.gc).marked as libc::c_int &
                                     (1i32 << 0i32 | 1i32 << 1i32)
                         }) {
                    break ;
                }
                if 0 != uvalue.tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(b"(((&uvalue)->tt_) & (1 << 6))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  257i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                o = uvalue.value_.gc
            }
            6 => {
                if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"(o)->tt == (6 | (0 << 4))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  263i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                let ref mut fresh1 =
                    (*(&mut (*(o as *mut GCUnion)).cl.l as
                           *mut LClosure_0)).gclist;
                *fresh1 = (*g).gray;
                if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"(o)->tt == (6 | (0 << 4))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  263i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*(&mut (*(o as *mut GCUnion)).cl.l as *mut LClosure_0)).tt
                       as libc::c_int & 15i32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((((((o)->tt == (6 | (0 << 4))) ? (void) (0) : __assert_fail (\"(o)->tt == (6 | (0 << 4))\", \"lgc.c\", 263, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->cl.l))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  263i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"(o)->tt == (6 | (0 << 4))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  263i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).cl.l as
                                *mut LClosure_0 as *mut GCUnion)).gc as
                        *mut GCObject;
                break ;
            }
            38 => {
                if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                } else {
                    __assert_fail(b"(o)->tt == (6 | (2 << 4))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  267i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                let ref mut fresh2 =
                    (*(&mut (*(o as *mut GCUnion)).cl.c as
                           *mut CClosure_0)).gclist;
                *fresh2 = (*g).gray;
                if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                } else {
                    __assert_fail(b"(o)->tt == (6 | (2 << 4))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  267i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*(&mut (*(o as *mut GCUnion)).cl.c as *mut CClosure_0)).tt
                       as libc::c_int & 15i32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((((((o)->tt == (6 | (2 << 4))) ? (void) (0) : __assert_fail (\"(o)->tt == (6 | (2 << 4))\", \"lgc.c\", 267, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->cl.c))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  267i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                } else {
                    __assert_fail(b"(o)->tt == (6 | (2 << 4))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  267i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).cl.c as
                                *mut CClosure_0 as *mut GCUnion)).gc as
                        *mut GCObject;
                break ;
            }
            5 => {
                if (*o).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(b"(o)->tt == 5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  271i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                let ref mut fresh3 =
                    (*(&mut (*(o as *mut GCUnion)).h as *mut Table)).gclist;
                *fresh3 = (*g).gray;
                if (*o).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(b"(o)->tt == 5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  271i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*(&mut (*(o as *mut GCUnion)).h as *mut Table)).tt as
                       libc::c_int & 15i32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((((((o)->tt == 5) ? (void) (0) : __assert_fail (\"(o)->tt == 5\", \"lgc.c\", 271, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->h))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  271i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(b"(o)->tt == 5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  271i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).h as *mut Table as
                                *mut GCUnion)).gc as *mut GCObject;
                break ;
            }
            8 => {
                if (*o).tt as libc::c_int == 8i32 {
                } else {
                    __assert_fail(b"(o)->tt == 8\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  275i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                let ref mut fresh4 =
                    (*(&mut (*(o as *mut GCUnion)).th as
                           *mut lua_State)).gclist;
                *fresh4 = (*g).gray;
                if (*o).tt as libc::c_int == 8i32 {
                } else {
                    __assert_fail(b"(o)->tt == 8\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  275i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*(&mut (*(o as *mut GCUnion)).th as *mut lua_State)).tt as
                       libc::c_int & 15i32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((((((o)->tt == 8) ? (void) (0) : __assert_fail (\"(o)->tt == 8\", \"lgc.c\", 275, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->th))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  275i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 8i32 {
                } else {
                    __assert_fail(b"(o)->tt == 8\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  275i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).th as *mut lua_State
                                as *mut GCUnion)).gc as *mut GCObject;
                break ;
            }
            9 => {
                if (*o).tt as libc::c_int == 9i32 {
                } else {
                    __assert_fail(b"(o)->tt == 9\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  279i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                let ref mut fresh5 =
                    (*(&mut (*(o as *mut GCUnion)).p as *mut Proto)).gclist;
                *fresh5 = (*g).gray;
                if (*o).tt as libc::c_int == 9i32 {
                } else {
                    __assert_fail(b"(o)->tt == 9\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  279i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*(&mut (*(o as *mut GCUnion)).p as *mut Proto)).tt as
                       libc::c_int & 15i32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((((((o)->tt == 9) ? (void) (0) : __assert_fail (\"(o)->tt == 9\", \"lgc.c\", 279, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((o))))->p))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  279i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                if (*o).tt as libc::c_int == 9i32 {
                } else {
                    __assert_fail(b"(o)->tt == 9\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  279i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).p as *mut Proto as
                                *mut GCUnion)).gc as *mut GCObject;
                break ;
            }
            _ => {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  282i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void reallymarkobject(global_State *, GCObject *)\x00")).as_ptr());
                };
                break ;
            }
        }
    };
}
unsafe extern "C" fn clearkeys(mut g: *mut global_State, mut l: *mut GCObject,
                               mut f: *mut GCObject) -> () {
    while l != f {
        if (*l).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(b"(l)->tt == 5\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          642i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 55],
                                                    &[libc::c_char; 55]>(b"void clearkeys(global_State *, GCObject *, GCObject *)\x00")).as_ptr());
        };
        let mut h: *mut Table_0 = &mut (*(l as *mut GCUnion)).h as *mut Table;
        let mut n: *mut Node = 0 as *mut Node;
        let mut limit: *mut Node =
            &mut *(*h).node.offset((1i32 << (*h).lsizenode as libc::c_int) as
                                       size_t as isize) as *mut Node;
        n = &mut *(*h).node.offset(0isize) as *mut Node;
        while n < limit {
            if !((*n).i_val.tt_ == 0i32) &&
                   0 !=
                       iscleared(g,
                                 &mut (*n).i_key.tvk as *mut TValue as
                                     *const TValue) {
                (*n).i_val.tt_ = 0i32;
                removeentry(n);
            }
            n = n.offset(1isize)
        }
        if (*l).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(b"(l)->tt == 5\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          641i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 55],
                                                    &[libc::c_char; 55]>(b"void clearkeys(global_State *, GCObject *, GCObject *)\x00")).as_ptr());
        };
        l = (*(&mut (*(l as *mut GCUnion)).h as *mut Table)).gclist
    };
}
unsafe extern "C" fn convergeephemerons(mut g: *mut global_State) -> () {
    let mut changed: libc::c_int = 0;
    loop  {
        let mut w: *mut GCObject = 0 as *mut GCObject;
        let mut next: *mut GCObject = (*g).ephemeron;
        (*g).ephemeron = 0 as *mut GCObject;
        changed = 0i32;
        loop  {
            w = next;
            if w.is_null() { break ; }
            if (*w).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(b"(w)->tt == 5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 617i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 40],
                                                        &[libc::c_char; 40]>(b"void convergeephemerons(global_State *)\x00")).as_ptr());
            };
            next = (*(&mut (*(w as *mut GCUnion)).h as *mut Table)).gclist;
            if (*w).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(b"(w)->tt == 5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 618i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 40],
                                                        &[libc::c_char; 40]>(b"void convergeephemerons(global_State *)\x00")).as_ptr());
            };
            if !(0 != traverseephemeron(g, &mut (*(w as *mut GCUnion)).h)) {
                continue ;
            }
            propagateall(g);
            changed = 1i32
        }
        if !(0 != changed) { break ; }
    };
}
unsafe extern "C" fn propagateall(mut g: *mut global_State) -> () {
    while !(*g).gray.is_null() { propagatemark(g); };
}
unsafe extern "C" fn propagatemark(mut g: *mut global_State) -> () {
    let mut size: lu_mem = 0;
    let mut o: *mut GCObject = (*g).gray;
    if 0 ==
           (*o).marked as libc::c_int &
               (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32) {
    } else {
        __assert_fail(b"(!(((o)->marked) & (((1<<(0)) | (1<<(1))) | (1<<(2)))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      563i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void propagatemark(global_State *)\x00")).as_ptr());
    };
    (*o).marked = ((*o).marked as libc::c_int | 1i32 << 2i32) as lu_byte;
    match (*o).tt as libc::c_int {
        5 => {
            if (*o).tt as libc::c_int == 5i32 {
            } else {
                __assert_fail(b"(o)->tt == 5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 567i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void propagatemark(global_State *)\x00")).as_ptr());
            };
            let mut h: *mut Table_0 =
                &mut (*(o as *mut GCUnion)).h as *mut Table;
            (*g).gray = (*h).gclist;
            size = traversetable(g, h)
        }
        6 => {
            if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 573i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void propagatemark(global_State *)\x00")).as_ptr());
            };
            let mut cl: *mut LClosure_0 =
                &mut (*(o as *mut GCUnion)).cl.l as *mut LClosure_0;
            (*g).gray = (*cl).gclist;
            size = traverseLclosure(g, cl)
        }
        38 => {
            if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
            } else {
                __assert_fail(b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 579i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void propagatemark(global_State *)\x00")).as_ptr());
            };
            let mut cl_0: *mut CClosure_0 =
                &mut (*(o as *mut GCUnion)).cl.c as *mut CClosure_0;
            (*g).gray = (*cl_0).gclist;
            size = traverseCclosure(g, cl_0)
        }
        8 => {
            if (*o).tt as libc::c_int == 8i32 {
            } else {
                __assert_fail(b"(o)->tt == 8\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 585i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void propagatemark(global_State *)\x00")).as_ptr());
            };
            let mut th: *mut lua_State_0 =
                &mut (*(o as *mut GCUnion)).th as *mut lua_State;
            (*g).gray = (*th).gclist;
            (*th).gclist = (*g).grayagain;
            if (*th).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((th)->tt) & 0x0F) < (9+1)\x00" as *const u8
                                  as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 587i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void propagatemark(global_State *)\x00")).as_ptr());
            };
            (*g).grayagain = &mut (*(th as *mut GCUnion)).gc as *mut GCObject;
            (*o).marked =
                ((*o).marked as libc::c_int &
                     !(1i32 << 2i32) as lu_byte as libc::c_int) as lu_byte;
            size = traversethread(g, th)
        }
        9 => {
            if (*o).tt as libc::c_int == 9i32 {
            } else {
                __assert_fail(b"(o)->tt == 9\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 593i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void propagatemark(global_State *)\x00")).as_ptr());
            };
            let mut p: *mut Proto_0 =
                &mut (*(o as *mut GCUnion)).p as *mut Proto;
            (*g).gray = (*p).gclist;
            size = traverseproto(g, p) as lu_mem
        }
        _ => {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 598i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void propagatemark(global_State *)\x00")).as_ptr());
            };
            return
        }
    }
    (*g).GCmemtrav =
        ((*g).GCmemtrav as libc::c_ulong).wrapping_add(size) as lu_mem as
            lu_mem;
}
unsafe extern "C" fn traverseproto(mut g: *mut global_State,
                                   mut f: *mut Proto_0) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if !(*f).cache.is_null() &&
           0 !=
               (*(*f).cache).marked as libc::c_int &
                   (1i32 << 0i32 | 1i32 << 1i32) {
        (*f).cache = 0 as *mut LClosure
    }
    if !(*f).source.is_null() {
        if 0 !=
               (*(*f).source).marked as libc::c_int &
                   (1i32 << 0i32 | 1i32 << 1i32) {
            if (*(*f).source).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((f->source)->tt) & 0x0F) < (9+1)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 484i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 43],
                                                        &[libc::c_char; 43]>(b"int traverseproto(global_State *, Proto *)\x00")).as_ptr());
            };
            reallymarkobject(g, &mut (*((*f).source as *mut GCUnion)).gc);
        }
    }
    i = 0i32;
    while i < (*f).sizek {
        if 0 == (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((&f->k[i])->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     486i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 43],
                                                               &[libc::c_char; 43]>(b"int traverseproto(global_State *, Proto *)\x00")).as_ptr());
                   };
                   (*(*f).k.offset(i as isize)).tt_ & 63i32 ==
                       (*(*(*f).k.offset(i as isize)).value_.gc).tt as
                           libc::c_int
               } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 486i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 43],
                                                        &[libc::c_char; 43]>(b"int traverseproto(global_State *, Proto *)\x00")).as_ptr());
            };
        };
        if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 &&
               {
                   if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((&f->k[i])->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     486i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 43],
                                                               &[libc::c_char; 43]>(b"int traverseproto(global_State *, Proto *)\x00")).as_ptr());
                   };
                   0 !=
                       (*(*(*f).k.offset(i as isize)).value_.gc).marked as
                           libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
               } {
            if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((&f->k[i])->tt_) & (1 << 6))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 486i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 43],
                                                        &[libc::c_char; 43]>(b"int traverseproto(global_State *, Proto *)\x00")).as_ptr());
            };
            reallymarkobject(g, (*(*f).k.offset(i as isize)).value_.gc);
        }
        i += 1
    }
    i = 0i32;
    while i < (*f).sizeupvalues {
        if !(*(*f).upvalues.offset(i as isize)).name.is_null() {
            if 0 !=
                   (*(*(*f).upvalues.offset(i as isize)).name).marked as
                       libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
                if (*(*(*f).upvalues.offset(i as isize)).name).tt as
                       libc::c_int & 15i32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((f->upvalues[i].name)->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  488i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 43],
                                                            &[libc::c_char; 43]>(b"int traverseproto(global_State *, Proto *)\x00")).as_ptr());
                };
                reallymarkobject(g,
                                 &mut (*((*(*f).upvalues.offset(i as
                                                                    isize)).name
                                             as *mut GCUnion)).gc);
            }
        }
        i += 1
    }
    i = 0i32;
    while i < (*f).sizep {
        if !(*(*f).p.offset(i as isize)).is_null() {
            if 0 !=
                   (**(*f).p.offset(i as isize)).marked as libc::c_int &
                       (1i32 << 0i32 | 1i32 << 1i32) {
                if (**(*f).p.offset(i as isize)).tt as libc::c_int & 15i32 <
                       9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((f->p[i])->tt) & 0x0F) < (9+1)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  490i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 43],
                                                            &[libc::c_char; 43]>(b"int traverseproto(global_State *, Proto *)\x00")).as_ptr());
                };
                reallymarkobject(g,
                                 &mut (*(*(*f).p.offset(i as isize) as
                                             *mut GCUnion)).gc);
            }
        }
        i += 1
    }
    i = 0i32;
    while i < (*f).sizelocvars {
        if !(*(*f).locvars.offset(i as isize)).varname.is_null() {
            if 0 !=
                   (*(*(*f).locvars.offset(i as isize)).varname).marked as
                       libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
                if (*(*(*f).locvars.offset(i as isize)).varname).tt as
                       libc::c_int & 15i32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((f->locvars[i].varname)->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  492i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 43],
                                                            &[libc::c_char; 43]>(b"int traverseproto(global_State *, Proto *)\x00")).as_ptr());
                };
                reallymarkobject(g,
                                 &mut (*((*(*f).locvars.offset(i as
                                                                   isize)).varname
                                             as *mut GCUnion)).gc);
            }
        }
        i += 1
    }
    return (::std::mem::size_of::<Proto_0>() as
                libc::c_ulong).wrapping_add((::std::mem::size_of::<Instruction>()
                                                 as
                                                 libc::c_ulong).wrapping_mul((*f).sizecode
                                                                                 as
                                                                                 libc::c_ulong)).wrapping_add((::std::mem::size_of::<*mut Proto_0>()
                                                                                                                   as
                                                                                                                   libc::c_ulong).wrapping_mul((*f).sizep
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong)).wrapping_add((::std::mem::size_of::<TValue>()
                                                                                                                                                                                     as
                                                                                                                                                                                     libc::c_ulong).wrapping_mul((*f).sizek
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_ulong)).wrapping_add((::std::mem::size_of::<libc::c_int>()
                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                       libc::c_ulong).wrapping_mul((*f).sizelineinfo
                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                       libc::c_ulong)).wrapping_add((::std::mem::size_of::<LocVar_0>()
                                                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                                                         libc::c_ulong).wrapping_mul((*f).sizelocvars
                                                                                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                                                                                         libc::c_ulong)).wrapping_add((::std::mem::size_of::<Upvaldesc_0>()
                                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                                           libc::c_ulong).wrapping_mul((*f).sizeupvalues
                                                                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                                                                           libc::c_ulong))
               as libc::c_int;
}
unsafe extern "C" fn traversethread(mut g: *mut global_State,
                                    mut th: *mut lua_State_0) -> lu_mem {
    let mut o: StkId = (*th).stack;
    if o.is_null() {
        return 1i32 as lu_mem
    } else {
        if (*g).gcstate as libc::c_int == 7i32 + 1i32 ||
               (*th).openupval.is_null() || (*th).twups != th {
        } else {
            __assert_fail(b"g->gcstate == (7 + 1) || th->openupval == ((void*)0) || (th->twups != th)\x00"
                              as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          536i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 51],
                                                    &[libc::c_char; 51]>(b"lu_mem traversethread(global_State *, lua_State *)\x00")).as_ptr());
        };
        while o < (*th).top {
            if 0 == (*o).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*o).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((o)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         538i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 51],
                                                                   &[libc::c_char; 51]>(b"lu_mem traversethread(global_State *, lua_State *)\x00")).as_ptr());
                       };
                       (*o).tt_ & 63i32 == (*(*o).value_.gc).tt as libc::c_int
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  538i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 51],
                                                            &[libc::c_char; 51]>(b"lu_mem traversethread(global_State *, lua_State *)\x00")).as_ptr());
                };
            };
            if 0 != (*o).tt_ & 1i32 << 6i32 &&
                   {
                       if 0 != (*o).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((o)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         538i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 51],
                                                                   &[libc::c_char; 51]>(b"lu_mem traversethread(global_State *, lua_State *)\x00")).as_ptr());
                       };
                       0 !=
                           (*(*o).value_.gc).marked as libc::c_int &
                               (1i32 << 0i32 | 1i32 << 1i32)
                   } {
                if 0 != (*o).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(b"(((o)->tt_) & (1 << 6))\x00" as *const u8
                                      as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  538i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 51],
                                                            &[libc::c_char; 51]>(b"lu_mem traversethread(global_State *, lua_State *)\x00")).as_ptr());
                };
                reallymarkobject(g, (*o).value_.gc);
            }
            o = o.offset(1isize)
        }
        if (*g).gcstate as libc::c_int == 7i32 + 1i32 {
            let mut lim: StkId = (*th).stack.offset((*th).stacksize as isize);
            while o < lim { (*o).tt_ = 0i32; o = o.offset(1isize) }
            if !((*th).twups != th) && !(*th).openupval.is_null() {
                (*th).twups = (*g).twups;
                (*g).twups = th
            }
        } else if (*g).gckind as libc::c_int != 1i32 { luaD_shrinkstack(th); }
        return (::std::mem::size_of::<lua_State_0>() as
                    libc::c_ulong).wrapping_add((::std::mem::size_of::<TValue>()
                                                     as
                                                     libc::c_ulong).wrapping_mul((*th).stacksize
                                                                                     as
                                                                                     libc::c_ulong)).wrapping_add((::std::mem::size_of::<CallInfo_0>()
                                                                                                                       as
                                                                                                                       libc::c_ulong).wrapping_mul((*th).nci
                                                                                                                                                       as
                                                                                                                                                       libc::c_ulong))
    };
}
unsafe extern "C" fn traverseCclosure(mut g: *mut global_State,
                                      mut cl: *mut CClosure_0) -> lu_mem {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        if 0 == (*cl).upvalue[i as usize].tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*cl).upvalue[i as usize].tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((&cl->upvalue[i])->tt_) & (1 << 6))\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     505i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 52],
                                                               &[libc::c_char; 52]>(b"lu_mem traverseCclosure(global_State *, CClosure *)\x00")).as_ptr());
                   };
                   (*cl).upvalue[i as usize].tt_ & 63i32 ==
                       (*(*cl).upvalue[i as usize].value_.gc).tt as
                           libc::c_int
               } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 505i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[libc::c_char; 52]>(b"lu_mem traverseCclosure(global_State *, CClosure *)\x00")).as_ptr());
            };
        };
        if 0 != (*cl).upvalue[i as usize].tt_ & 1i32 << 6i32 &&
               {
                   if 0 != (*cl).upvalue[i as usize].tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((&cl->upvalue[i])->tt_) & (1 << 6))\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     505i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 52],
                                                               &[libc::c_char; 52]>(b"lu_mem traverseCclosure(global_State *, CClosure *)\x00")).as_ptr());
                   };
                   0 !=
                       (*(*cl).upvalue[i as usize].value_.gc).marked as
                           libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
               } {
            if 0 != (*cl).upvalue[i as usize].tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((&cl->upvalue[i])->tt_) & (1 << 6))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 505i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[libc::c_char; 52]>(b"lu_mem traverseCclosure(global_State *, CClosure *)\x00")).as_ptr());
            };
            reallymarkobject(g, (*cl).upvalue[i as usize].value_.gc);
        }
        i += 1
    }
    return (::std::mem::size_of::<CClosure_0>() as libc::c_ulong as
                libc::c_int +
                (::std::mem::size_of::<TValue>() as
                     libc::c_ulong).wrapping_mul(((*cl).nupvalues as
                                                      libc::c_int - 1i32) as
                                                     libc::c_ulong) as
                    libc::c_int) as lu_mem;
}
unsafe extern "C" fn traverseLclosure(mut g: *mut global_State,
                                      mut cl: *mut LClosure_0) -> lu_mem {
    let mut i: libc::c_int = 0;
    if !(*cl).p.is_null() {
        if 0 !=
               (*(*cl).p).marked as libc::c_int &
                   (1i32 << 0i32 | 1i32 << 1i32) {
            if (*(*cl).p).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((cl->p)->tt) & 0x0F) < (9+1)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 517i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[libc::c_char; 52]>(b"lu_mem traverseLclosure(global_State *, LClosure *)\x00")).as_ptr());
            };
            reallymarkobject(g, &mut (*((*cl).p as *mut GCUnion)).gc);
        }
    }
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv: *mut UpVal = (*cl).upvals[i as usize];
        if !uv.is_null() {
            if (*uv).v != &mut (*uv).u.value as *mut TValue &&
                   (*g).gcstate as libc::c_int != 7i32 + 1i32 {
                (*uv).u.open.touched = 1i32
            } else {
                if 0 == (*(*uv).v).tt_ & 1i32 << 6i32 ||
                       {
                           if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                           } else {
                               __assert_fail(b"(((uv->v)->tt_) & (1 << 6))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lgc.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             524i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 52],
                                                                       &[libc::c_char; 52]>(b"lu_mem traverseLclosure(global_State *, LClosure *)\x00")).as_ptr());
                           };
                           (*(*uv).v).tt_ & 63i32 ==
                               (*(*(*uv).v).value_.gc).tt as libc::c_int
                       } {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(b"0\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"lgc.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      524i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 52],
                                                                &[libc::c_char; 52]>(b"lu_mem traverseLclosure(global_State *, LClosure *)\x00")).as_ptr());
                    };
                };
                if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 &&
                       {
                           if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                           } else {
                               __assert_fail(b"(((uv->v)->tt_) & (1 << 6))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lgc.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             524i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 52],
                                                                       &[libc::c_char; 52]>(b"lu_mem traverseLclosure(global_State *, LClosure *)\x00")).as_ptr());
                           };
                           0 !=
                               (*(*(*uv).v).value_.gc).marked as libc::c_int &
                                   (1i32 << 0i32 | 1i32 << 1i32)
                       } {
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(b"(((uv->v)->tt_) & (1 << 6))\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"lgc.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      524i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 52],
                                                                &[libc::c_char; 52]>(b"lu_mem traverseLclosure(global_State *, LClosure *)\x00")).as_ptr());
                    };
                    reallymarkobject(g, (*(*uv).v).value_.gc);
                }
            }
        }
        i += 1
    }
    return (::std::mem::size_of::<LClosure_0>() as libc::c_ulong as
                libc::c_int +
                (::std::mem::size_of::<*mut TValue>() as
                     libc::c_ulong).wrapping_mul(((*cl).nupvalues as
                                                      libc::c_int - 1i32) as
                                                     libc::c_ulong) as
                    libc::c_int) as lu_mem;
}
unsafe extern "C" fn traversetable(mut g: *mut global_State,
                                   mut h: *mut Table_0) -> lu_mem {
    let mut weakkey: *const libc::c_char = 0 as *const libc::c_char;
    let mut weakvalue: *const libc::c_char = 0 as *const libc::c_char;
    let mut mode: *const TValue =
        if (*h).metatable.is_null() {
            0 as *const TValue
        } else if 0 !=
                      (*(*h).metatable).flags as libc::c_uint &
                          1u32 << TM_MODE as libc::c_int {
            0 as *const TValue
        } else {
            luaT_gettm((*h).metatable, TM_MODE,
                       (*g).tmname[TM_MODE as libc::c_int as usize])
        };
    if !(*h).metatable.is_null() {
        if 0 !=
               (*(*h).metatable).marked as libc::c_int &
                   (1i32 << 0i32 | 1i32 << 1i32) {
            if (*(*h).metatable).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((h->metatable)->tt) & 0x0F) < (9+1)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 455i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
            };
            reallymarkobject(g, &mut (*((*h).metatable as *mut GCUnion)).gc);
        }
    }
    if !mode.is_null() && (*mode).tt_ & 15i32 == 4i32 &&
           {
               if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
               } else {
                   __assert_fail(b"sizeof((((((((((((mode))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((mode))->tt_)) & 0x0F)) == (4))\", \"lgc.c\", 457, __extension__ __PRETTY_FUNCTION__)), (((((((((mode)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((mode)->value_).gc)->tt) & 0x0F) == 4\", \"lgc.c\", 457, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((mode)->value_).gc))))->ts))))))->extra)\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 457i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 46],
                                                           &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
               };
               if (*mode).tt_ & 15i32 == 4i32 {
               } else {
                   __assert_fail(b"(((((((mode))->tt_)) & 0x0F)) == (4))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 457i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 46],
                                                           &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
               };
               if (*(*mode).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
               } else {
                   __assert_fail(b"(((((mode)->value_).gc)->tt) & 0x0F) == 4\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 457i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 46],
                                                           &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
               };
               weakkey =
                   strchr((&mut (*((*mode).value_.gc as *mut GCUnion)).ts as
                               *mut TString_0 as
                               *mut libc::c_char).offset(::std::mem::size_of::<UTString_0>()
                                                             as libc::c_ulong
                                                             as isize),
                          'k' as i32);
               if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
               } else {
                   __assert_fail(b"sizeof((((((((((((mode))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((mode))->tt_)) & 0x0F)) == (4))\", \"lgc.c\", 458, __extension__ __PRETTY_FUNCTION__)), (((((((((mode)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((mode)->value_).gc)->tt) & 0x0F) == 4\", \"lgc.c\", 458, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((mode)->value_).gc))))->ts))))))->extra)\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 458i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 46],
                                                           &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
               };
               if (*mode).tt_ & 15i32 == 4i32 {
               } else {
                   __assert_fail(b"(((((((mode))->tt_)) & 0x0F)) == (4))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 458i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 46],
                                                           &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
               };
               if (*(*mode).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
               } else {
                   __assert_fail(b"(((((mode)->value_).gc)->tt) & 0x0F) == 4\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 458i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 46],
                                                           &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
               };
               weakvalue =
                   strchr((&mut (*((*mode).value_.gc as *mut GCUnion)).ts as
                               *mut TString_0 as
                               *mut libc::c_char).offset(::std::mem::size_of::<UTString_0>()
                                                             as libc::c_ulong
                                                             as isize),
                          'v' as i32);
               !weakkey.is_null() || !weakvalue.is_null()
           } {
        (*h).marked =
            ((*h).marked as libc::c_int &
                 !(1i32 << 2i32) as lu_byte as libc::c_int) as lu_byte;
        if weakkey.is_null() {
            traverseweakvalue(g, h);
        } else if weakvalue.is_null() {
            traverseephemeron(g, h);
        } else {
            (*h).gclist = (*g).allweak;
            if (*h).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8
                                  as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 466i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"lu_mem traversetable(global_State *, Table *)\x00")).as_ptr());
            };
            (*g).allweak = &mut (*(h as *mut GCUnion)).gc as *mut GCObject
        }
    } else { traversestrongtable(g, h); }
    return (::std::mem::size_of::<Table_0>() as
                libc::c_ulong).wrapping_add((::std::mem::size_of::<TValue>()
                                                 as
                                                 libc::c_ulong).wrapping_mul((*h).sizearray
                                                                                 as
                                                                                 libc::c_ulong)).wrapping_add((::std::mem::size_of::<Node>()
                                                                                                                   as
                                                                                                                   libc::c_ulong).wrapping_mul((if (*h).lastfree.is_null()
                                                                                                                                                   {
                                                                                                                                                    0i32
                                                                                                                                                } else {
                                                                                                                                                    1i32
                                                                                                                                                        <<
                                                                                                                                                        (*h).lsizenode
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                })
                                                                                                                                                   as
                                                                                                                                                   size_t));
}
unsafe extern "C" fn traversestrongtable(mut g: *mut global_State,
                                         mut h: *mut Table_0) -> () {
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node =
        &mut *(*h).node.offset((1i32 << (*h).lsizenode as libc::c_int) as
                                   size_t as isize) as *mut Node;
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while i < (*h).sizearray {
        if 0 == (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32
                      {
                   } else {
                       __assert_fail(b"(((&h->array[i])->tt_) & (1 << 6))\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     438i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 50],
                                                               &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                   };
                   (*(*h).array.offset(i as isize)).tt_ & 63i32 ==
                       (*(*(*h).array.offset(i as isize)).value_.gc).tt as
                           libc::c_int
               } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 438i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
            };
        };
        if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 &&
               {
                   if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32
                      {
                   } else {
                       __assert_fail(b"(((&h->array[i])->tt_) & (1 << 6))\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     438i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 50],
                                                               &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                   };
                   0 !=
                       (*(*(*h).array.offset(i as isize)).value_.gc).marked as
                           libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
               } {
            if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((&h->array[i])->tt_) & (1 << 6))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 438i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
            };
            reallymarkobject(g, (*(*h).array.offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1)
    }
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ ==
                 9i32 + 1i32) || (*n).i_val.tt_ == 0i32 {
        } else {
            __assert_fail(b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == ((9+1))) || (((((&(n)->i_val)))->tt_) == (0))\x00"
                              as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          440i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 50],
                                                    &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
        };
        if (*n).i_val.tt_ == 0i32 {
            removeentry(n);
        } else {
            if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_
                     == 0i32) {
            } else {
                __assert_fail(b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == (0))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 444i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
            };
            if 0 ==
                   (*(&mut (*n).i_key.tvk as *mut TValue as
                          *const TValue)).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 !=
                              (*(&mut (*n).i_key.tvk as *mut TValue as
                                     *const TValue)).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         445i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 50],
                                                                   &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                       };
                       (*(&mut (*n).i_key.tvk as *mut TValue as
                              *const TValue)).tt_ & 63i32 ==
                           (*(*(&mut (*n).i_key.tvk as *mut TValue as
                                    *const TValue)).value_.gc).tt as
                               libc::c_int
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  445i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                };
            };
            if 0 !=
                   (*(&mut (*n).i_key.tvk as *mut TValue as
                          *const TValue)).tt_ & 1i32 << 6i32 &&
                   {
                       if 0 !=
                              (*(&mut (*n).i_key.tvk as *mut TValue as
                                     *const TValue)).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         445i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 50],
                                                                   &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                       };
                       0 !=
                           (*(*(&mut (*n).i_key.tvk as *mut TValue as
                                    *const TValue)).value_.gc).marked as
                               libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                   } {
                if 0 !=
                       (*(&mut (*n).i_key.tvk as *mut TValue as
                              *const TValue)).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  445i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                };
                reallymarkobject(g,
                                 (*(&mut (*n).i_key.tvk as *mut TValue as
                                        *const TValue)).value_.gc);
            }
            if 0 == (*n).i_val.tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"((((&(n)->i_val))->tt_) & (1 << 6))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         446i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 50],
                                                                   &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                       };
                       (*n).i_val.tt_ & 63i32 ==
                           (*(*n).i_val.value_.gc).tt as libc::c_int
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  446i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                };
            };
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32 &&
                   {
                       if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"((((&(n)->i_val))->tt_) & (1 << 6))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         446i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 50],
                                                                   &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                       };
                       0 !=
                           (*(*n).i_val.value_.gc).marked as libc::c_int &
                               (1i32 << 0i32 | 1i32 << 1i32)
                   } {
                if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(b"((((&(n)->i_val))->tt_) & (1 << 6))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  446i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 50],
                                                            &[libc::c_char; 50]>(b"void traversestrongtable(global_State *, Table *)\x00")).as_ptr());
                };
                reallymarkobject(g, (*n).i_val.value_.gc);
            }
        }
        n = n.offset(1isize)
    };
}
unsafe extern "C" fn traverseephemeron(mut g: *mut global_State,
                                       mut h: *mut Table_0) -> libc::c_int {
    let mut marked: libc::c_int = 0i32;
    let mut hasclears: libc::c_int = 0i32;
    let mut hasww: libc::c_int = 0i32;
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node =
        &mut *(*h).node.offset((1i32 << (*h).lsizenode as libc::c_int) as
                                   size_t as isize) as *mut Node;
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while i < (*h).sizearray {
        if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 &&
               {
                   if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32
                      {
                   } else {
                       __assert_fail(b"(((&h->array[i])->tt_) & (1 << 6))\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"lgc.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     403i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 47],
                                                               &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
                   };
                   0 !=
                       (*(*(*h).array.offset(i as isize)).value_.gc).marked as
                           libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
               } {
            marked = 1i32;
            if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((&h->array[i])->tt_) & (1 << 6))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 405i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 47],
                                                        &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
            };
            reallymarkobject(g, (*(*h).array.offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1)
    }
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ ==
                 9i32 + 1i32) || (*n).i_val.tt_ == 0i32 {
        } else {
            __assert_fail(b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == ((9+1))) || (((((&(n)->i_val)))->tt_) == (0))\x00"
                              as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          410i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
        };
        if (*n).i_val.tt_ == 0i32 {
            removeentry(n);
        } else if 0 !=
                      iscleared(g,
                                &mut (*n).i_key.tvk as *mut TValue as
                                    *const TValue) {
            hasclears = 1i32;
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32 &&
                   {
                       if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"((((&(n)->i_val))->tt_) & (1 << 6))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         415i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 47],
                                                                   &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
                       };
                       0 !=
                           (*(*n).i_val.value_.gc).marked as libc::c_int &
                               (1i32 << 0i32 | 1i32 << 1i32)
                   } {
                hasww = 1i32
            }
        } else if 0 != (*n).i_val.tt_ & 1i32 << 6i32 &&
                      {
                          if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
                          } else {
                              __assert_fail(b"((((&(n)->i_val))->tt_) & (1 << 6))\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            b"lgc.c\x00" as *const u8 as
                                                *const libc::c_char,
                                            418i32 as libc::c_uint,
                                            (*::std::mem::transmute::<&[u8; 47],
                                                                      &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
                          };
                          0 !=
                              (*(*n).i_val.value_.gc).marked as libc::c_int &
                                  (1i32 << 0i32 | 1i32 << 1i32)
                      } {
            marked = 1i32;
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"((((&(n)->i_val))->tt_) & (1 << 6))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 420i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 47],
                                                        &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
            };
            reallymarkobject(g, (*n).i_val.value_.gc);
        }
        n = n.offset(1isize)
    }
    if (*g).gcstate as libc::c_int == 0i32 {
        (*h).gclist = (*g).grayagain;
        if (*h).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          425i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
        };
        (*g).grayagain = &mut (*(h as *mut GCUnion)).gc as *mut GCObject
    } else if 0 != hasww {
        (*h).gclist = (*g).ephemeron;
        if (*h).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          427i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
        };
        (*g).ephemeron = &mut (*(h as *mut GCUnion)).gc as *mut GCObject
    } else if 0 != hasclears {
        (*h).gclist = (*g).allweak;
        if (*h).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          429i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 47],
                                                    &[libc::c_char; 47]>(b"int traverseephemeron(global_State *, Table *)\x00")).as_ptr());
        };
        (*g).allweak = &mut (*(h as *mut GCUnion)).gc as *mut GCObject
    }
    return marked;
}
unsafe extern "C" fn traverseweakvalue(mut g: *mut global_State,
                                       mut h: *mut Table_0) -> () {
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node =
        &mut *(*h).node.offset((1i32 << (*h).lsizenode as libc::c_int) as
                                   size_t as isize) as *mut Node;
    let mut hasclears: libc::c_int =
        ((*h).sizearray > 0i32 as libc::c_uint) as libc::c_int;
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ ==
                 9i32 + 1i32) || (*n).i_val.tt_ == 0i32 {
        } else {
            __assert_fail(b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == ((9+1))) || (((((&(n)->i_val)))->tt_) == (0))\x00"
                              as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          368i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
        };
        if (*n).i_val.tt_ == 0i32 {
            removeentry(n);
        } else {
            if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_
                     == 0i32) {
            } else {
                __assert_fail(b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == (0))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 372i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 48],
                                                        &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
            };
            if 0 ==
                   (*(&mut (*n).i_key.tvk as *mut TValue as
                          *const TValue)).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 !=
                              (*(&mut (*n).i_key.tvk as *mut TValue as
                                     *const TValue)).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         373i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 48],
                                                                   &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
                       };
                       (*(&mut (*n).i_key.tvk as *mut TValue as
                              *const TValue)).tt_ & 63i32 ==
                           (*(*(&mut (*n).i_key.tvk as *mut TValue as
                                    *const TValue)).value_.gc).tt as
                               libc::c_int
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  373i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 48],
                                                            &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
                };
            };
            if 0 !=
                   (*(&mut (*n).i_key.tvk as *mut TValue as
                          *const TValue)).tt_ & 1i32 << 6i32 &&
                   {
                       if 0 !=
                              (*(&mut (*n).i_key.tvk as *mut TValue as
                                     *const TValue)).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"lgc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         373i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 48],
                                                                   &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
                       };
                       0 !=
                           (*(*(&mut (*n).i_key.tvk as *mut TValue as
                                    *const TValue)).value_.gc).marked as
                               libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                   } {
                if 0 !=
                       (*(&mut (*n).i_key.tvk as *mut TValue as
                              *const TValue)).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(b"(((((const TValue*)((&(n)->i_key.tvk))))->tt_) & (1 << 6))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  373i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 48],
                                                            &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
                };
                reallymarkobject(g,
                                 (*(&mut (*n).i_key.tvk as *mut TValue as
                                        *const TValue)).value_.gc);
            }
            if 0 == hasclears && 0 != iscleared(g, &mut (*n).i_val) {
                hasclears = 1i32
            }
        }
        n = n.offset(1isize)
    }
    if (*g).gcstate as libc::c_int == 0i32 {
        (*h).gclist = (*g).grayagain;
        if (*h).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          379i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
        };
        (*g).grayagain = &mut (*(h as *mut GCUnion)).gc as *mut GCObject
    } else if 0 != hasclears {
        (*h).gclist = (*g).weak;
        if (*h).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          381i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"void traverseweakvalue(global_State *, Table *)\x00")).as_ptr());
        };
        (*g).weak = &mut (*(h as *mut GCUnion)).gc as *mut GCObject
    };
}
unsafe extern "C" fn markbeingfnz(mut g: *mut global_State) -> () {
    let mut o: *mut GCObject = 0 as *mut GCObject;
    o = (*g).tobefnz;
    while !o.is_null() {
        if 0 != (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
            if (*o).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((o)->tt) & 0x0F) < (9+1)\x00" as *const u8
                                  as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 303i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 34],
                                                        &[libc::c_char; 34]>(b"void markbeingfnz(global_State *)\x00")).as_ptr());
            };
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).gc);
        }
        o = (*o).next
    };
}
unsafe extern "C" fn remarkupvals(mut g: *mut global_State) -> () {
    let mut thread: *mut lua_State_0 = 0 as *mut lua_State_0;
    let mut p: *mut *mut lua_State_0 = &mut (*g).twups as *mut *mut lua_State;
    loop  {
        thread = *p;
        if thread.is_null() { break ; }
        if 0 == (*thread).marked as libc::c_int & 1i32 << 2i32 {
        } else {
            __assert_fail(b"!(((thread)->marked) & ((1<<(2))))\x00" as
                              *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          317i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 34],
                                                    &[libc::c_char; 34]>(b"void remarkupvals(global_State *)\x00")).as_ptr());
        };
        if 0 ==
               (*thread).marked as libc::c_int &
                   (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32) &&
               !(*thread).openupval.is_null() {
            p = &mut (*thread).twups as *mut *mut lua_State
        } else {
            let mut uv: *mut UpVal = 0 as *mut UpVal;
            *p = (*thread).twups;
            (*thread).twups = thread;
            uv = (*thread).openupval;
            while !uv.is_null() {
                if 0 != (*uv).u.open.touched {
                    if 0 == (*(*uv).v).tt_ & 1i32 << 6i32 ||
                           {
                               if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                               } else {
                                   __assert_fail(b"(((uv->v)->tt_) & (1 << 6))\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"lgc.c\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 326i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 34],
                                                                           &[libc::c_char; 34]>(b"void remarkupvals(global_State *)\x00")).as_ptr());
                               };
                               (*(*uv).v).tt_ & 63i32 ==
                                   (*(*(*uv).v).value_.gc).tt as libc::c_int
                           } {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(b"0\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          326i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 34],
                                                                    &[libc::c_char; 34]>(b"void remarkupvals(global_State *)\x00")).as_ptr());
                        };
                    };
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 &&
                           {
                               if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                               } else {
                                   __assert_fail(b"(((uv->v)->tt_) & (1 << 6))\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"lgc.c\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 326i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 34],
                                                                           &[libc::c_char; 34]>(b"void remarkupvals(global_State *)\x00")).as_ptr());
                               };
                               0 !=
                                   (*(*(*uv).v).value_.gc).marked as
                                       libc::c_int &
                                       (1i32 << 0i32 | 1i32 << 1i32)
                           } {
                        if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(b"(((uv->v)->tt_) & (1 << 6))\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lgc.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          326i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 34],
                                                                    &[libc::c_char; 34]>(b"void remarkupvals(global_State *)\x00")).as_ptr());
                        };
                        reallymarkobject(g, (*(*uv).v).value_.gc);
                    }
                    (*uv).u.open.touched = 0i32
                }
                uv = (*uv).u.open.next
            }
        }
    };
}
unsafe extern "C" fn markmt(mut g: *mut global_State) -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 9i32 {
        if !(*g).mt[i as usize].is_null() {
            if 0 !=
                   (*(*g).mt[i as usize]).marked as libc::c_int &
                       (1i32 << 0i32 | 1i32 << 1i32) {
                if (*(*g).mt[i as usize]).tt as libc::c_int & 15i32 <
                       9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((g->mt[i])->tt) & 0x0F) < (9+1)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lgc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  293i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 28],
                                                            &[libc::c_char; 28]>(b"void markmt(global_State *)\x00")).as_ptr());
                };
                reallymarkobject(g,
                                 &mut (*((*g).mt[i as usize] as
                                             *mut GCUnion)).gc);
            }
        }
        i += 1
    };
}
unsafe extern "C" fn restartcollection(mut g: *mut global_State) -> () {
    (*g).grayagain = 0 as *mut GCObject;
    (*g).gray = (*g).grayagain;
    (*g).ephemeron = 0 as *mut GCObject;
    (*g).allweak = (*g).ephemeron;
    (*g).weak = (*g).allweak;
    if 0 !=
           (*(*g).mainthread).marked as libc::c_int &
               (1i32 << 0i32 | 1i32 << 1i32) {
        if (*(*g).mainthread).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((g->mainthread)->tt) & 0x0F) < (9+1)\x00" as
                              *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          341i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 39],
                                                    &[libc::c_char; 39]>(b"void restartcollection(global_State *)\x00")).as_ptr());
        };
        reallymarkobject(g, &mut (*((*g).mainthread as *mut GCUnion)).gc);
    }
    if 0 == (*g).l_registry.tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((&g->l_registry)->tt_) & (1 << 6))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 342i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 39],
                                                           &[libc::c_char; 39]>(b"void restartcollection(global_State *)\x00")).as_ptr());
               };
               (*g).l_registry.tt_ & 63i32 ==
                   (*(*g).l_registry.value_.gc).tt as libc::c_int
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          342i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 39],
                                                    &[libc::c_char; 39]>(b"void restartcollection(global_State *)\x00")).as_ptr());
        };
    };
    if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 &&
           {
               if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((&g->l_registry)->tt_) & (1 << 6))\x00"
                                     as *const u8 as *const libc::c_char,
                                 b"lgc.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 342i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 39],
                                                           &[libc::c_char; 39]>(b"void restartcollection(global_State *)\x00")).as_ptr());
               };
               0 !=
                   (*(*g).l_registry.value_.gc).marked as libc::c_int &
                       (1i32 << 0i32 | 1i32 << 1i32)
           } {
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as
                              *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          342i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 39],
                                                    &[libc::c_char; 39]>(b"void restartcollection(global_State *)\x00")).as_ptr());
        };
        reallymarkobject(g, (*g).l_registry.value_.gc);
    }
    markmt(g);
    markbeingfnz(g);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_runtilstate(mut L: *mut lua_State_0,
                                          mut statesmask: libc::c_int) -> () {
    let mut g: *mut global_State = (*L).l_G;
    while 0 == statesmask & 1i32 << (*g).gcstate as libc::c_int {
        singlestep(L);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_fullgc(mut L: *mut lua_State_0,
                                     mut isemergency: libc::c_int) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if (*g).gckind as libc::c_int == 0i32 {
    } else {
        __assert_fail(b"g->gckind == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      1161i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void luaC_fullgc(lua_State *, int)\x00")).as_ptr());
    };
    if 0 != isemergency { (*g).gckind = 1i32 as lu_byte }
    if (*g).gcstate as libc::c_int <= 1i32 { entersweep(L); }
    luaC_runtilstate(L, 1i32 << 7i32);
    luaC_runtilstate(L, !(1i32 << 7i32));
    luaC_runtilstate(L, 1i32 << 6i32);
    if (*g).GCestimate == ((*g).totalbytes + (*g).GCdebt) as lu_mem {
    } else {
        __assert_fail(b"g->GCestimate == ((lu_mem)((g)->totalbytes + (g)->GCdebt))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      1171i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"void luaC_fullgc(lua_State *, int)\x00")).as_ptr());
    };
    luaC_runtilstate(L, 1i32 << 7i32);
    (*g).gckind = 0i32 as lu_byte;
    setpause(g);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_newobj(mut L: *mut lua_State_0,
                                     mut tt: libc::c_int, mut sz: size_t)
 -> *mut GCObject {
    let mut g: *mut global_State = (*L).l_G;
    let mut o: *mut GCObject =
        luaM_realloc_(L, 0 as *mut libc::c_void, (tt & 15i32) as size_t, sz)
            as *mut GCObject;
    (*o).marked =
        ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as
            lu_byte;
    (*o).tt = tt as lu_byte;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_barrier_(mut L: *mut lua_State_0,
                                       mut o: *mut GCObject,
                                       mut v: *mut GCObject) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if 0 != (*o).marked as libc::c_int & 1i32 << 2i32 &&
           0 != (*v).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) &&
           0 !=
               ((*v).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32)) &
                   ((*g).currentwhite as libc::c_int ^
                        (1i32 << 0i32 | 1i32 << 1i32)) &&
           0 !=
               ((*o).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32)) &
                   ((*g).currentwhite as libc::c_int ^
                        (1i32 << 0i32 | 1i32 << 1i32)) {
    } else {
        __assert_fail(b"(((o)->marked) & ((1<<(2)))) && (((v)->marked) & (((1<<(0)) | (1<<(1))))) && !(!((((v)->marked) ^ ((1<<(0)) | (1<<(1)))) & (((g)->currentwhite ^ ((1<<(0)) | (1<<(1))))))) && !(!((((o)->marked) ^ ((1<<(0)) | (1<<(1)))) & (((g)->currentwhite ^ ((1<<(0)) | (1<<(1)))))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      158i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"void luaC_barrier_(lua_State *, GCObject *, GCObject *)\x00")).as_ptr());
    };
    if (*g).gcstate as libc::c_int <= 1i32 {
        reallymarkobject(g, v);
    } else {
        if 2i32 <= (*g).gcstate as libc::c_int &&
               (*g).gcstate as libc::c_int <= 5i32 {
        } else {
            __assert_fail(b"(2 <= (g)->gcstate && (g)->gcstate <= 5)\x00" as
                              *const u8 as *const libc::c_char,
                          b"lgc.c\x00" as *const u8 as *const libc::c_char,
                          162i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 56],
                                                    &[libc::c_char; 56]>(b"void luaC_barrier_(lua_State *, GCObject *, GCObject *)\x00")).as_ptr());
        };
        (*o).marked =
            ((*o).marked as libc::c_int &
                 !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32)) |
                 ((*g).currentwhite as libc::c_int &
                      (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte as
                     libc::c_int) as lu_byte
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_barrierback_(mut L: *mut lua_State_0,
                                           mut t: *mut Table_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if 0 != (*t).marked as libc::c_int & 1i32 << 2i32 &&
           0 !=
               ((*t).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32)) &
                   ((*g).currentwhite as libc::c_int ^
                        (1i32 << 0i32 | 1i32 << 1i32)) {
    } else {
        __assert_fail(b"(((t)->marked) & ((1<<(2)))) && !(!((((t)->marked) ^ ((1<<(0)) | (1<<(1)))) & (((g)->currentwhite ^ ((1<<(0)) | (1<<(1)))))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      174i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void luaC_barrierback_(lua_State *, Table *)\x00")).as_ptr());
    };
    (*t).marked =
        ((*t).marked as libc::c_int &
             !(1i32 << 2i32) as lu_byte as libc::c_int) as lu_byte;
    (*t).gclist = (*g).grayagain;
    if (*t).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((t)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      176i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void luaC_barrierback_(lua_State *, Table *)\x00")).as_ptr());
    };
    (*g).grayagain = &mut (*(t as *mut GCUnion)).gc as *mut GCObject;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_upvalbarrier_(mut L: *mut lua_State_0,
                                            mut uv: *mut UpVal) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 {
    } else {
        __assert_fail(b"(((uv->v)->tt_) & (1 << 6))\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      188i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"void luaC_upvalbarrier_(lua_State *, UpVal *)\x00")).as_ptr());
    };
    let mut o: *mut GCObject = (*(*uv).v).value_.gc;
    if !((*uv).v != &mut (*uv).u.value as *mut TValue) {
    } else {
        __assert_fail(b"!((uv)->v != &(uv)->u.value)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lgc.c\x00" as *const u8 as *const libc::c_char,
                      189i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 46],
                                                &[libc::c_char; 46]>(b"void luaC_upvalbarrier_(lua_State *, UpVal *)\x00")).as_ptr());
    };
    if (*g).gcstate as libc::c_int <= 1i32 {
        if 0 != (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
            if (*o).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
            } else {
                __assert_fail(b"(((o)->tt) & 0x0F) < (9+1)\x00" as *const u8
                                  as *const libc::c_char,
                              b"lgc.c\x00" as *const u8 as
                                  *const libc::c_char, 191i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"void luaC_upvalbarrier_(lua_State *, UpVal *)\x00")).as_ptr());
            };
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).gc);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_checkfinalizer(mut L: *mut lua_State_0,
                                             mut o: *mut GCObject,
                                             mut mt: *mut Table_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if 0 != (*o).marked as libc::c_int & 1i32 << 3i32 ||
           if mt.is_null() {
               0 as *const TValue
           } else if 0 !=
                         (*mt).flags as libc::c_uint &
                             1u32 << TM_GC as libc::c_int {
               0 as *const TValue
           } else {
               luaT_gettm(mt, TM_GC,
                          (*g).tmname[TM_GC as libc::c_int as usize])
           }.is_null() {
        return
    } else {
        let mut p: *mut *mut GCObject = 0 as *mut *mut GCObject;
        if 2i32 <= (*g).gcstate as libc::c_int &&
               (*g).gcstate as libc::c_int <= 5i32 {
            (*o).marked =
                ((*o).marked as libc::c_int &
                     !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32)) |
                     ((*g).currentwhite as libc::c_int &
                          (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte as
                         libc::c_int) as lu_byte;
            if (*g).sweepgc == &mut (*o).next as *mut *mut GCObject {
                (*g).sweepgc = sweeptolive(L, (*g).sweepgc)
            }
        }
        p = &mut (*g).allgc as *mut *mut GCObject;
        while *p != o { p = &mut (**p).next as *mut *mut GCObject }
        *p = (*o).next;
        (*o).next = (*g).finobj;
        (*g).finobj = o;
        (*o).marked = ((*o).marked as libc::c_int | 1i32 << 3i32) as lu_byte;
        return;
    };
}
unsafe extern "C" fn sweeptolive(mut L: *mut lua_State_0,
                                 mut p: *mut *mut GCObject)
 -> *mut *mut GCObject {
    let mut old: *mut *mut GCObject = p;
    loop  { p = sweeplist(L, p, 1i32 as lu_mem); if !(p == old) { break ; } }
    return p;
}
