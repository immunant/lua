#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type lua_longjmp;
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
    fn luaC_newobj(L: *mut lua_State_0, tt: libc::c_int, sz: size_t)
     -> *mut GCObject;
    #[no_mangle]
    fn luaM_realloc_(L: *mut lua_State_0, block: *mut libc::c_void,
                     oldsize: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn luaC_upvalbarrier_(L: *mut lua_State_0, uv: *mut UpVal) -> ();
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
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
pub type CClosure = CClosure_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type LocVar_0 = LocVar;
pub type LClosure = LClosure_0;
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
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure,
    l: LClosure,
}
pub type Proto_0 = Proto;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
pub type Upvaldesc_0 = Upvaldesc;
#[no_mangle]
pub unsafe extern "C" fn luaF_newproto(mut L: *mut lua_State_0)
 -> *mut Proto_0 {
    let mut o: *mut GCObject =
        luaC_newobj(L, 9i32,
                    ::std::mem::size_of::<Proto_0>() as libc::c_ulong);
    if (*o).tt as libc::c_int == 9i32 {
    } else {
        __assert_fail(b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
                      b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                      101i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"Proto *luaF_newproto(lua_State *)\x00")).as_ptr());
    };
    let mut f: *mut Proto_0 = &mut (*(o as *mut GCUnion)).p as *mut Proto;
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
    (*f).locvars = 0 as *mut LocVar_0;
    (*f).sizelocvars = 0i32;
    (*f).linedefined = 0i32;
    (*f).lastlinedefined = 0i32;
    (*f).source = 0 as *mut TString;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newCclosure(mut L: *mut lua_State_0,
                                          mut n: libc::c_int)
 -> *mut CClosure {
    let mut o: *mut GCObject =
        luaC_newobj(L, 6i32 | 2i32 << 4i32,
                    (::std::mem::size_of::<CClosure>() as libc::c_ulong as
                         libc::c_int +
                         (::std::mem::size_of::<TValue>() as
                              libc::c_ulong).wrapping_mul((n - 1i32) as
                                                              libc::c_ulong)
                             as libc::c_int) as size_t);
    if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
    } else {
        __assert_fail(b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as
                          *const libc::c_char,
                      b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                      27i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"CClosure *luaF_newCclosure(lua_State *, int)\x00")).as_ptr());
    };
    let mut c: *mut CClosure =
        &mut (*(o as *mut GCUnion)).cl.c as *mut CClosure;
    (*c).nupvalues = n as lu_byte;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newLclosure(mut L: *mut lua_State_0,
                                          mut n: libc::c_int)
 -> *mut LClosure {
    let mut o: *mut GCObject =
        luaC_newobj(L, 6i32 | 0i32 << 4i32,
                    (::std::mem::size_of::<LClosure>() as libc::c_ulong as
                         libc::c_int +
                         (::std::mem::size_of::<*mut TValue>() as
                              libc::c_ulong).wrapping_mul((n - 1i32) as
                                                              libc::c_ulong)
                             as libc::c_int) as size_t);
    if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as
                          *const libc::c_char,
                      b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                      35i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"LClosure *luaF_newLclosure(lua_State *, int)\x00")).as_ptr());
    };
    let mut c: *mut LClosure =
        &mut (*(o as *mut GCUnion)).cl.l as *mut LClosure;
    (*c).p = 0 as *mut Proto;
    (*c).nupvalues = n as lu_byte;
    loop  {
        let fresh0 = n;
        n = n - 1;
        if !(0 != fresh0) { break ; }
        (*c).upvals[n as usize] = 0 as *mut UpVal
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_initupvals(mut L: *mut lua_State_0,
                                         mut cl: *mut LClosure) -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv: *mut UpVal =
            luaM_realloc_(L, 0 as *mut libc::c_void, 0i32 as size_t,
                          ::std::mem::size_of::<UpVal>() as libc::c_ulong) as
                *mut UpVal;
        (*uv).refcount = 1i32 as lu_mem;
        (*uv).v = &mut (*uv).u.value as *mut TValue;
        (*(*uv).v).tt_ = 0i32;
        (*cl).upvals[i as usize] = uv;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaF_findupval(mut L: *mut lua_State_0,
                                        mut level: StkId) -> *mut UpVal {
    let mut pp: *mut *mut UpVal = &mut (*L).openupval as *mut *mut UpVal;
    let mut p: *mut UpVal = 0 as *mut UpVal;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    if (*L).twups != L || (*L).openupval.is_null() {
    } else {
        __assert_fail(b"(L->twups != L) || L->openupval == ((void*)0)\x00" as
                          *const u8 as *const libc::c_char,
                      b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                      61i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"UpVal *luaF_findupval(lua_State *, StkId)\x00")).as_ptr());
    };
    while !(*pp).is_null() && { p = *pp; (*p).v >= level } {
        if (*p).v != &mut (*p).u.value as *mut TValue {
        } else {
            __assert_fail(b"((p)->v != &(p)->u.value)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                          63i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 42],
                                                    &[libc::c_char; 42]>(b"UpVal *luaF_findupval(lua_State *, StkId)\x00")).as_ptr());
        };
        if (*p).v == level {
            return p
        } else { pp = &mut (*p).u.open.next as *mut *mut UpVal }
    }
    uv =
        luaM_realloc_(L, 0 as *mut libc::c_void, 0i32 as size_t,
                      ::std::mem::size_of::<UpVal>() as libc::c_ulong) as
            *mut UpVal;
    (*uv).refcount = 0i32 as lu_mem;
    (*uv).u.open.next = *pp;
    (*uv).u.open.touched = 1i32;
    *pp = uv;
    (*uv).v = level;
    if !((*L).twups != L) {
        (*L).twups = (*(*L).l_G).twups;
        (*(*L).l_G).twups = L
    }
    return uv;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_close(mut L: *mut lua_State_0, mut level: StkId)
 -> () {
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    while !(*L).openupval.is_null() &&
              { uv = (*L).openupval; (*uv).v >= level } {
        if (*uv).v != &mut (*uv).u.value as *mut TValue {
        } else {
            __assert_fail(b"((uv)->v != &(uv)->u.value)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lfunc.c\x00" as *const u8 as *const libc::c_char,
                          86i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 36],
                                                    &[libc::c_char; 36]>(b"void luaF_close(lua_State *, StkId)\x00")).as_ptr());
        };
        (*L).openupval = (*uv).u.open.next;
        if (*uv).refcount == 0i32 as libc::c_ulong {
            luaM_realloc_(L, uv as *mut libc::c_void,
                          ::std::mem::size_of::<UpVal>() as libc::c_ulong,
                          0i32 as size_t);
        } else {
            let mut io1: *mut TValue = &mut (*uv).u.value as *mut TValue;
            *io1 = *(*uv).v;
            if 0 == (*io1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"lfunc.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         91i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 36],
                                                                   &[libc::c_char; 36]>(b"void luaF_close(lua_State *, StkId)\x00")).as_ptr());
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
                                                      b"lfunc.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      91i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 36],
                                                                                &[libc::c_char; 36]>(b"void luaF_close(lua_State *, StkId)\x00")).as_ptr());
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
                                  b"lfunc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  91i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 36],
                                                            &[libc::c_char; 36]>(b"void luaF_close(lua_State *, StkId)\x00")).as_ptr());
                };
            };
            (*uv).v = &mut (*uv).u.value as *mut TValue;
            if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 &&
                   !((*uv).v != &mut (*uv).u.value as *mut TValue) {
                luaC_upvalbarrier_(L, uv);
            } else { };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaF_freeproto(mut L: *mut lua_State_0,
                                        mut f: *mut Proto_0) -> () {
    luaM_realloc_(L, (*f).code as *mut libc::c_void,
                  ((*f).sizecode as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, (*f).p as *mut libc::c_void,
                  ((*f).sizep as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, (*f).k as *mut libc::c_void,
                  ((*f).sizek as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, (*f).lineinfo as *mut libc::c_void,
                  ((*f).sizelineinfo as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, (*f).locvars as *mut libc::c_void,
                  ((*f).sizelocvars as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<LocVar_0>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, (*f).upvalues as *mut libc::c_void,
                  ((*f).sizeupvalues as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<Upvaldesc_0>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, f as *mut libc::c_void,
                  ::std::mem::size_of::<Proto_0>() as libc::c_ulong,
                  0i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn luaF_getlocalname(mut f: *const Proto_0,
                                           mut local_number: libc::c_int,
                                           mut pc: libc::c_int)
 -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*f).sizelocvars &&
              (*(*f).locvars.offset(i as isize)).startpc <= pc {
        if pc < (*(*f).locvars.offset(i as isize)).endpc {
            local_number -= 1;
            if local_number == 0i32 {
                if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
                } else {
                    __assert_fail(b"sizeof((f->locvars[i].varname)->extra)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lfunc.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  146i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 55],
                                                            &[libc::c_char; 55]>(b"const char *luaF_getlocalname(const Proto *, int, int)\x00")).as_ptr());
                };
                return ((*(*f).locvars.offset(i as isize)).varname as
                            *mut libc::c_char).offset(::std::mem::size_of::<UTString_0>()
                                                          as libc::c_ulong as
                                                          isize)
            }
        }
        i += 1
    }
    return 0 as *const libc::c_char;
}
