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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn luaM_toobig(L: *mut lua_State_0) -> !;
    #[no_mangle]
    fn luaM_realloc_(L: *mut lua_State_0, block: *mut libc::c_void,
                     oldsize: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn luaZ_read(z: *mut ZIO, b: *mut libc::c_void, n: size_t) -> size_t;
    #[no_mangle]
    fn luaD_inctop(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State_0, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaF_newproto(L: *mut lua_State_0) -> *mut Proto;
    #[no_mangle]
    fn luaF_newLclosure(L: *mut lua_State_0, nelems: libc::c_int)
     -> *mut LClosure_0;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State_0, str: *const libc::c_char, l: size_t)
     -> *mut TString;
    #[no_mangle]
    fn luaS_createlngstrobj(L: *mut lua_State_0, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type ZIO = Zio;
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
pub type lua_Reader =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void,
                                _: *mut size_t) -> *const libc::c_char>;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata,
    cl: Closure,
    h: Table,
    p: Proto_0,
    th: lua_State,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure_0,
    l: LClosure_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto_0,
    pub upvals: [*mut UpVal; 1],
}
pub type LClosure_0 = LClosure;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LoadState {
    pub L: *mut lua_State_0,
    pub Z: *mut ZIO,
    pub name: *const libc::c_char,
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
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
pub type Proto = Proto_0;
pub type CClosure_0 = CClosure;
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
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type LocVar_0 = LocVar;
#[no_mangle]
pub unsafe extern "C" fn luaU_undump(mut L: *mut lua_State_0, mut Z: *mut ZIO,
                                     mut name: *const libc::c_char)
 -> *mut LClosure_0 {
    let mut S: LoadState =
        LoadState{L: 0 as *mut lua_State_0,
                  Z: 0 as *mut ZIO,
                  name: 0 as *const libc::c_char,};
    let mut cl: *mut LClosure_0 = 0 as *mut LClosure_0;
    if *name as libc::c_int == '@' as i32 ||
           *name as libc::c_int == '=' as i32 {
        S.name = name.offset(1isize)
    } else if *name as libc::c_int ==
                  (*::std::mem::transmute::<&[u8; 5],
                                            &[libc::c_char; 5]>(b"\x1bLua\x00"))[0usize]
                      as libc::c_int {
        S.name = b"binary string\x00" as *const u8 as *const libc::c_char
    } else { S.name = name }
    S.L = L;
    S.Z = Z;
    checkHeader(&mut S);
    cl = luaF_newLclosure(L, LoadByte(&mut S) as libc::c_int);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut LClosure_0 = cl;
    if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lundump.c\x00" as *const u8 as *const libc::c_char,
                      272i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"LClosure *luaU_undump(lua_State *, ZIO *, const char *)\x00")).as_ptr());
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
    (*io).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lundump.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 272i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 56],
                                                           &[libc::c_char; 56]>(b"LClosure *luaU_undump(lua_State *, ZIO *, const char *)\x00")).as_ptr());
               };
               (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int &&
                   (L.is_null() ||
                        {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lundump.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              272i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 56],
                                                                        &[libc::c_char; 56]>(b"LClosure *luaU_undump(lua_State *, ZIO *, const char *)\x00")).as_ptr());
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
                          b"lundump.c\x00" as *const u8 as
                              *const libc::c_char, 272i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 56],
                                                    &[libc::c_char; 56]>(b"LClosure *luaU_undump(lua_State *, ZIO *, const char *)\x00")).as_ptr());
        };
    };
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    LoadFunction(&mut S, (*cl).p, 0 as *mut TString);
    if (*cl).nupvalues as libc::c_int == (*(*cl).p).sizeupvalues {
    } else {
        __assert_fail(b"cl->nupvalues == cl->p->sizeupvalues\x00" as *const u8
                          as *const libc::c_char,
                      b"lundump.c\x00" as *const u8 as *const libc::c_char,
                      276i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 56],
                                                &[libc::c_char; 56]>(b"LClosure *luaU_undump(lua_State *, ZIO *, const char *)\x00")).as_ptr());
    };
    return cl;
}
unsafe extern "C" fn LoadFunction(mut S: *mut LoadState, mut f: *mut Proto,
                                  mut psource: *mut TString) -> () {
    (*f).source = LoadString(S);
    if (*f).source.is_null() { (*f).source = psource }
    (*f).linedefined = LoadInt(S);
    (*f).lastlinedefined = LoadInt(S);
    (*f).numparams = LoadByte(S);
    (*f).is_vararg = LoadByte(S);
    (*f).maxstacksize = LoadByte(S);
    LoadCode(S, f);
    LoadConstants(S, f);
    LoadUpvalues(S, f);
    LoadProtos(S, f);
    LoadDebug(S, f);
}
unsafe extern "C" fn LoadDebug(mut S: *mut LoadState, mut f: *mut Proto)
 -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (n as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong) {
        luaM_toobig((*S).L);
    } else { };
    (*f).lineinfo =
        luaM_realloc_((*S).L, 0 as *mut libc::c_void,
                      (0i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong),
                      (n as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong))
            as *mut libc::c_int;
    (*f).sizelineinfo = n;
    LoadBlock(S, (*f).lineinfo as *mut libc::c_void,
              (n as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                   as libc::c_ulong));
    n = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (n as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<LocVar_0>()
                                                as libc::c_ulong) {
        luaM_toobig((*S).L);
    } else { };
    (*f).locvars =
        luaM_realloc_((*S).L, 0 as *mut libc::c_void,
                      (0i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<LocVar_0>()
                                                           as libc::c_ulong),
                      (n as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<LocVar_0>()
                                                           as libc::c_ulong))
            as *mut LocVar_0;
    (*f).sizelocvars = n;
    i = 0i32;
    while i < n {
        let ref mut fresh0 = (*(*f).locvars.offset(i as isize)).varname;
        *fresh0 = 0 as *mut TString;
        i += 1
    }
    i = 0i32;
    while i < n {
        let ref mut fresh1 = (*(*f).locvars.offset(i as isize)).varname;
        *fresh1 = LoadString(S);
        (*(*f).locvars.offset(i as isize)).startpc = LoadInt(S);
        (*(*f).locvars.offset(i as isize)).endpc = LoadInt(S);
        i += 1
    }
    n = LoadInt(S);
    i = 0i32;
    while i < n {
        let ref mut fresh2 = (*(*f).upvalues.offset(i as isize)).name;
        *fresh2 = LoadString(S);
        i += 1
    };
}
unsafe extern "C" fn LoadString(mut S: *mut LoadState) -> *mut TString {
    let mut size: size_t = LoadByte(S) as size_t;
    if size == 255i32 as libc::c_ulong {
        LoadBlock(S, &mut size as *mut size_t as *mut libc::c_void,
                  (1i32 as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                       as libc::c_ulong));
    }
    if size == 0i32 as libc::c_ulong {
        return 0 as *mut TString
    } else {
        size = size.wrapping_sub(1);
        if size <= 40i32 as libc::c_ulong {
            let mut buff: [libc::c_char; 40] = [0; 40];
            LoadBlock(S, buff.as_mut_ptr() as *mut libc::c_void,
                      size.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong));
            return luaS_newlstr((*S).L, buff.as_mut_ptr(), size)
        } else {
            let mut ts: *mut TString = luaS_createlngstrobj((*S).L, size);
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(b"sizeof((ts)->extra)\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lundump.c\x00" as *const u8 as
                                  *const libc::c_char, 102i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 33],
                                                        &[libc::c_char; 33]>(b"TString *LoadString(LoadState *)\x00")).as_ptr());
            };
            LoadBlock(S,
                      (ts as
                           *mut libc::c_char).offset(::std::mem::size_of::<UTString_0>()
                                                         as libc::c_ulong as
                                                         isize) as
                          *mut libc::c_void,
                      size.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong));
            return ts
        }
    };
}
unsafe extern "C" fn LoadByte(mut S: *mut LoadState) -> lu_byte {
    let mut x: lu_byte = 0;
    LoadBlock(S, &mut x as *mut lu_byte as *mut libc::c_void,
              (1i32 as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<lu_byte>()
                                                   as libc::c_ulong));
    return x;
}
unsafe extern "C" fn LoadBlock(mut S: *mut LoadState,
                               mut b: *mut libc::c_void, mut size: size_t)
 -> () {
    if luaZ_read((*S).Z, b, size) != 0i32 as libc::c_ulong {
        error(S, b"truncated\x00" as *const u8 as *const libc::c_char);
    } else { return; };
}
unsafe extern "C" fn error(mut S: *mut LoadState,
                           mut why: *const libc::c_char) -> ! {
    luaO_pushfstring((*S).L,
                     b"%s: %s precompiled chunk\x00" as *const u8 as
                         *const libc::c_char, (*S).name, why);
    luaD_throw((*S).L, 3i32);
}
unsafe extern "C" fn LoadInt(mut S: *mut LoadState) -> libc::c_int {
    let mut x: libc::c_int = 0;
    LoadBlock(S, &mut x as *mut libc::c_int as *mut libc::c_void,
              (1i32 as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                   as libc::c_ulong));
    return x;
}
unsafe extern "C" fn LoadProtos(mut S: *mut LoadState, mut f: *mut Proto)
 -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (n as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<*mut Proto>()
                                                as libc::c_ulong) {
        luaM_toobig((*S).L);
    } else { };
    (*f).p =
        luaM_realloc_((*S).L, 0 as *mut libc::c_void,
                      (0i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>()
                                                           as libc::c_ulong),
                      (n as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>()
                                                           as libc::c_ulong))
            as *mut *mut Proto;
    (*f).sizep = n;
    i = 0i32;
    while i < n {
        let ref mut fresh3 = *(*f).p.offset(i as isize);
        *fresh3 = 0 as *mut Proto_0;
        i += 1
    }
    i = 0i32;
    while i < n {
        let ref mut fresh4 = *(*f).p.offset(i as isize);
        *fresh4 = luaF_newproto((*S).L);
        LoadFunction(S, *(*f).p.offset(i as isize), (*f).source);
        i += 1
    };
}
unsafe extern "C" fn LoadUpvalues(mut S: *mut LoadState, mut f: *mut Proto)
 -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (n as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<Upvaldesc_0>()
                                                as libc::c_ulong) {
        luaM_toobig((*S).L);
    } else { };
    (*f).upvalues =
        luaM_realloc_((*S).L, 0 as *mut libc::c_void,
                      (0i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<Upvaldesc_0>()
                                                           as libc::c_ulong),
                      (n as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<Upvaldesc_0>()
                                                           as libc::c_ulong))
            as *mut Upvaldesc_0;
    (*f).sizeupvalues = n;
    i = 0i32;
    while i < n {
        let ref mut fresh5 = (*(*f).upvalues.offset(i as isize)).name;
        *fresh5 = 0 as *mut TString;
        i += 1
    }
    i = 0i32;
    while i < n {
        (*(*f).upvalues.offset(i as isize)).instack = LoadByte(S);
        (*(*f).upvalues.offset(i as isize)).idx = LoadByte(S);
        i += 1
    };
}
unsafe extern "C" fn LoadConstants(mut S: *mut LoadState, mut f: *mut Proto)
 -> () {
    let mut io_0: *mut TValue = 0 as *mut TValue;
    let mut io_2: *mut TValue = 0 as *mut TValue;
    let mut io_1: *mut TValue = 0 as *mut TValue;
    let mut io: *mut TValue = 0 as *mut TValue;
    let mut x_: *mut TString = 0 as *mut TString;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (n as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<TValue>() as
                                                libc::c_ulong) {
        luaM_toobig((*S).L);
    } else { };
    (*f).k =
        luaM_realloc_((*S).L, 0 as *mut libc::c_void,
                      (0i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong),
                      (n as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong))
            as *mut TValue;
    (*f).sizek = n;
    i = 0i32;
    while i < n { (*(*f).k.offset(i as isize)).tt_ = 0i32; i += 1 }
    i = 0i32;
    while i < n {
        let mut o: *mut TValue =
            &mut *(*f).k.offset(i as isize) as *mut TValue;
        let mut t: libc::c_int = LoadByte(S) as libc::c_int;
        match t {
            0 => { (*o).tt_ = 0i32 }
            1 => {
                io = o;
                (*io).value_.b = LoadByte(S) as libc::c_int;
                (*io).tt_ = 1i32
            }
            3 => {
                io_0 = o;
                (*io_0).value_.n = LoadNumber(S);
                (*io_0).tt_ = 3i32 | 0i32 << 4i32
            }
            19 => {
                io_1 = o;
                (*io_1).value_.i = LoadInteger(S);
                (*io_1).tt_ = 3i32 | 1i32 << 4i32
            }
            4 | 20 => {
                io_2 = o;
                x_ = LoadString(S);
                if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
                } else {
                    __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"lundump.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  144i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 41],
                                                            &[libc::c_char; 41]>(b"void LoadConstants(LoadState *, Proto *)\x00")).as_ptr());
                };
                (*io_2).value_.gc =
                    &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
                (*io_2).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
                if 0 == (*io_2).tt_ & 1i32 << 6i32 ||
                       {
                           if 0 != (*io_2).tt_ & 1i32 << 6i32 {
                           } else {
                               __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             b"lundump.c\x00" as *const u8 as
                                                 *const libc::c_char,
                                             144i32 as libc::c_uint,
                                             (*::std::mem::transmute::<&[u8; 41],
                                                                       &[libc::c_char; 41]>(b"void LoadConstants(LoadState *, Proto *)\x00")).as_ptr());
                           };
                           (*io_2).tt_ & 63i32 ==
                               (*(*io_2).value_.gc).tt as libc::c_int &&
                               ((*S).L.is_null() ||
                                    {
                                        if 0 != (*io_2).tt_ & 1i32 << 6i32 {
                                        } else {
                                            __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          b"lundump.c\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          144i32 as
                                                              libc::c_uint,
                                                          (*::std::mem::transmute::<&[u8; 41],
                                                                                    &[libc::c_char; 41]>(b"void LoadConstants(LoadState *, Proto *)\x00")).as_ptr());
                                        };
                                        0 !=
                                            ((*(*io_2).value_.gc).marked as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32)) &
                                                ((*(*(*S).L).l_G).currentwhite
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
                                      b"lundump.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      144i32 as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 41],
                                                                &[libc::c_char; 41]>(b"void LoadConstants(LoadState *, Proto *)\x00")).as_ptr());
                    };
                };
            }
            _ => {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lundump.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  147i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 41],
                                                            &[libc::c_char; 41]>(b"void LoadConstants(LoadState *, Proto *)\x00")).as_ptr());
                };
            }
        }
        i += 1
    };
}
unsafe extern "C" fn LoadInteger(mut S: *mut LoadState) -> lua_Integer {
    let mut x: lua_Integer = 0;
    LoadBlock(S, &mut x as *mut lua_Integer as *mut libc::c_void,
              (1i32 as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<lua_Integer>()
                                                   as libc::c_ulong));
    return x;
}
unsafe extern "C" fn LoadNumber(mut S: *mut LoadState) -> lua_Number {
    let mut x: lua_Number = 0.;
    LoadBlock(S, &mut x as *mut lua_Number as *mut libc::c_void,
              (1i32 as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<lua_Number>()
                                                   as libc::c_ulong));
    return x;
}
unsafe extern "C" fn LoadCode(mut S: *mut LoadState, mut f: *mut Proto)
 -> () {
    let mut n: libc::c_int = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (n as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<Instruction>()
                                                as libc::c_ulong) {
        luaM_toobig((*S).L);
    } else { };
    (*f).code =
        luaM_realloc_((*S).L, 0 as *mut libc::c_void,
                      (0i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>()
                                                           as libc::c_ulong),
                      (n as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>()
                                                           as libc::c_ulong))
            as *mut Instruction;
    (*f).sizecode = n;
    LoadBlock(S, (*f).code as *mut libc::c_void,
              (n as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>()
                                                   as libc::c_ulong));
}
unsafe extern "C" fn checkHeader(mut S: *mut LoadState) -> () {
    checkliteral(S,
                 (b"\x1bLua\x00" as *const u8 as
                      *const libc::c_char).offset(1isize),
                 b"not a\x00" as *const u8 as *const libc::c_char);
    if LoadByte(S) as libc::c_int !=
           ((*::std::mem::transmute::<&[u8; 2],
                                      &[libc::c_char; 2]>(b"5\x00"))[0usize]
                as libc::c_int - '0' as i32) * 16i32 +
               ((*::std::mem::transmute::<&[u8; 2],
                                          &[libc::c_char; 2]>(b"3\x00"))[0usize]
                    as libc::c_int - '0' as i32) {
        error(S,
              b"version mismatch in\x00" as *const u8 as *const libc::c_char);
    } else if LoadByte(S) as libc::c_int != 0i32 {
        error(S,
              b"format mismatch in\x00" as *const u8 as *const libc::c_char);
    } else {
        checkliteral(S,
                     b"\x19\x93\r\n\x1a\n\x00" as *const u8 as
                         *const libc::c_char,
                     b"corrupted\x00" as *const u8 as *const libc::c_char);
        fchecksize(S, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                   b"int\x00" as *const u8 as *const libc::c_char);
        fchecksize(S, ::std::mem::size_of::<size_t>() as libc::c_ulong,
                   b"size_t\x00" as *const u8 as *const libc::c_char);
        fchecksize(S, ::std::mem::size_of::<Instruction>() as libc::c_ulong,
                   b"Instruction\x00" as *const u8 as *const libc::c_char);
        fchecksize(S, ::std::mem::size_of::<lua_Integer>() as libc::c_ulong,
                   b"lua_Integer\x00" as *const u8 as *const libc::c_char);
        fchecksize(S, ::std::mem::size_of::<lua_Number>() as libc::c_ulong,
                   b"lua_Number\x00" as *const u8 as *const libc::c_char);
        if LoadInteger(S) != 22136i32 as libc::c_longlong {
            error(S,
                  b"endianness mismatch in\x00" as *const u8 as
                      *const libc::c_char);
        } else if LoadNumber(S) != 370.5f64 {
            error(S,
                  b"float format mismatch in\x00" as *const u8 as
                      *const libc::c_char);
        } else { return; }
    };
}
unsafe extern "C" fn fchecksize(mut S: *mut LoadState, mut size: size_t,
                                mut tname: *const libc::c_char) -> () {
    if LoadByte(S) as libc::c_ulong != size {
        error(S,
              luaO_pushfstring((*S).L,
                               b"%s size mismatch in\x00" as *const u8 as
                                   *const libc::c_char, tname));
    } else { return; };
}
unsafe extern "C" fn checkliteral(mut S: *mut LoadState,
                                  mut s: *const libc::c_char,
                                  mut msg: *const libc::c_char) -> () {
    let mut buff: [libc::c_char; 12] = [0; 12];
    let mut len: size_t = strlen(s);
    LoadBlock(S, buff.as_mut_ptr() as *mut libc::c_void,
              len.wrapping_mul(::std::mem::size_of::<libc::c_char>() as
                                   libc::c_ulong));
    if memcmp(s as *const libc::c_void,
              buff.as_mut_ptr() as *const libc::c_void, len) != 0i32 {
        error(S, msg);
    } else { return; };
}
