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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    fn luaM_realloc_(L: *mut lua_State_0, block: *mut libc::c_void,
                     oldsize: size_t, size: size_t) -> *mut libc::c_void;
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
    fn luaH_setint(L: *mut lua_State_0, t: *mut Table_0, key: lua_Integer,
                   value: *mut TValue) -> ();
    #[no_mangle]
    fn luaH_resize(L: *mut lua_State_0, t: *mut Table_0, nasize: libc::c_uint,
                   nhsize: libc::c_uint) -> ();
    #[no_mangle]
    fn luaM_toobig(L: *mut lua_State_0) -> !;
    #[no_mangle]
    fn luaD_rawrunprotected(L: *mut lua_State_0, f: Pfunc,
                            ud: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn luaS_hash(str: *const libc::c_char, l: size_t, seed: libc::c_uint)
     -> libc::c_uint;
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
pub type time_t = __time_t;
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
pub type LG = LG_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LG_0 {
    pub l: LX,
    pub g: global_State,
}
pub type LX = LX_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LX_0 {
    pub extra_: [lu_byte; 16],
    pub l: lua_State_0,
}
pub type Table_0 = Table;
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
#[no_mangle]
pub unsafe extern "C" fn lua_newstate(mut f: lua_Alloc,
                                      mut ud: *mut libc::c_void)
 -> *mut lua_State_0 {
    let mut i: libc::c_int = 0;
    let mut L: *mut lua_State_0 = 0 as *mut lua_State_0;
    let mut g: *mut global_State = 0 as *mut global_State;
    let mut l: *mut LG =
        f.expect("non-null function pointer")(ud, 0 as *mut libc::c_void,
                                              8i32 as size_t,
                                              ::std::mem::size_of::<LG>() as
                                                  libc::c_ulong) as *mut LG;
    if l.is_null() {
        return 0 as *mut lua_State_0
    } else {
        L = &mut (*l).l.l as *mut lua_State_0;
        g = &mut (*l).g as *mut global_State;
        (*L).next = 0 as *mut GCObject;
        (*L).tt = 8i32 as lu_byte;
        (*g).currentwhite = (1i32 << 0i32) as lu_byte;
        (*L).marked =
            ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32))
                as lu_byte;
        preinit_thread(L, g);
        (*g).frealloc = f;
        (*g).ud = ud;
        (*g).mainthread = L;
        (*g).seed = makeseed(L);
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
        (*g).totalbytes =
            ::std::mem::size_of::<LG>() as libc::c_ulong as l_mem;
        (*g).GCdebt = 0i32 as l_mem;
        (*g).gcfinnum = 0i32 as libc::c_uint;
        (*g).gcpause = 200i32;
        (*g).gcstepmul = 200i32;
        i = 0i32;
        while i < 9i32 { (*g).mt[i as usize] = 0 as *mut Table; i += 1 }
        if luaD_rawrunprotected(L, Some(f_luaopen), 0 as *mut libc::c_void) !=
               0i32 {
            close_state(L);
            L = 0 as *mut lua_State_0
        }
        return L
    };
}
unsafe extern "C" fn close_state(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    luaF_close(L, (*L).stack);
    luaC_freeallobjects(L);
    if !(*g).version.is_null() {
        if (*((L as
                   *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                   as libc::c_ulong as isize))
                  as *mut libc::c_void as *mut L_EXTRA)).lock == 1i32 &&
               (*((L as
                       *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                       as libc::c_ulong as
                                                       isize)) as
                      *mut libc::c_void as *mut L_EXTRA)).plock ==
                   &mut (*((L as
                                *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                                as
                                                                libc::c_ulong
                                                                as isize)) as
                               *mut libc::c_void as *mut L_EXTRA)).lock as
                       *mut libc::c_int {
        } else {
            __assert_fail(b"((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->lock == 1 && ((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock == &(((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->lock)\x00"
                              as *const u8 as *const libc::c_char,
                          b"lstate.c\x00" as *const u8 as *const libc::c_char,
                          247i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 30],
                                                    &[libc::c_char; 30]>(b"void close_state(lua_State *)\x00")).as_ptr());
        };
    }
    luaM_realloc_(L, (*(*L).l_G).strt.hash as *mut libc::c_void,
                  ((*(*L).l_G).strt.size as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    freestack(L);
    if ((*g).totalbytes + (*g).GCdebt) as lu_mem ==
           ::std::mem::size_of::<LG>() as libc::c_ulong {
    } else {
        __assert_fail(b"((lu_mem)((g)->totalbytes + (g)->GCdebt)) == sizeof(LG)\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      250i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"void close_state(lua_State *)\x00")).as_ptr());
    };
    (*g).frealloc.expect("non-null function pointer")((*g).ud,
                                                      (L as
                                                           *mut lu_byte).offset(-16isize)
                                                          as *mut LX as
                                                          *mut libc::c_void,
                                                      ::std::mem::size_of::<LG>()
                                                          as libc::c_ulong,
                                                      0i32 as size_t);
}
unsafe extern "C" fn freestack(mut L: *mut lua_State_0) -> () {
    if (*L).stack.is_null() {
        return
    } else {
        (*L).ci = &mut (*L).base_ci as *mut CallInfo_0;
        luaE_freeCI(L);
        if (*L).nci as libc::c_int == 0i32 {
        } else {
            __assert_fail(b"L->nci == 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"lstate.c\x00" as *const u8 as *const libc::c_char,
                          176i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 28],
                                                    &[libc::c_char; 28]>(b"void freestack(lua_State *)\x00")).as_ptr());
        };
        luaM_realloc_(L, (*L).stack as *mut libc::c_void,
                      ((*L).stacksize as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong),
                      0i32 as size_t);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaE_freeCI(mut L: *mut lua_State_0) -> () {
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let mut next: *mut CallInfo_0 = (*ci).next;
    (*ci).next = 0 as *mut CallInfo;
    loop  {
        ci = next;
        if ci.is_null() { break ; }
        next = (*ci).next;
        luaM_realloc_(L, ci as *mut libc::c_void,
                      ::std::mem::size_of::<CallInfo_0>() as libc::c_ulong,
                      0i32 as size_t);
        (*L).nci = (*L).nci.wrapping_sub(1)
    };
}
unsafe extern "C" fn f_luaopen(mut L: *mut lua_State_0,
                               mut ud: *mut libc::c_void) -> () {
    let mut g: *mut global_State = (*L).l_G;
    ud = 0 as *mut libc::c_void;
    stack_init(L, L);
    init_registry(L, g);
    luaS_init(L);
    luaT_init(L);
    luaX_init(L);
    (*g).gcrunning = 1i32 as lu_byte;
    (*g).version = lua_version(0 as *mut lua_State_0);
    (*((L as
            *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>() as
                                            libc::c_ulong as isize)) as
           *mut libc::c_void as *mut L_EXTRA)).lock = 0i32;
    let ref mut fresh0 =
        (*((L as
                *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                as libc::c_ulong as isize)) as
               *mut libc::c_void as *mut L_EXTRA)).plock;
    *fresh0 =
        &mut (*((L as
                     *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                     as libc::c_ulong as
                                                     isize)) as
                    *mut libc::c_void as *mut L_EXTRA)).lock as
            *mut libc::c_int;
}
unsafe extern "C" fn init_registry(mut L: *mut lua_State_0,
                                   mut g: *mut global_State) -> () {
    let mut temp: TValue =
        lua_TValue{value_: Value_0{gc: 0 as *mut GCObject,}, tt_: 0,};
    let mut registry: *mut Table_0 = luaH_new(L);
    let mut io: *mut TValue = &mut (*g).l_registry as *mut TValue;
    let mut x_: *mut Table_0 = registry;
    if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      188i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
    (*io).tt_ = 5i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lstate.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 188i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 48],
                                                           &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
               };
               (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int &&
                   (L.is_null() ||
                        {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lstate.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              188i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 48],
                                                                        &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
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
                          b"lstate.c\x00" as *const u8 as *const libc::c_char,
                          188i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
        };
    };
    luaH_resize(L, registry, 2i32 as libc::c_uint, 0i32 as libc::c_uint);
    let mut io_0: *mut TValue = &mut temp;
    let mut x__0: *mut lua_State_0 = L;
    if (*x__0).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      191i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
    };
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc as *mut GCObject;
    (*io_0).tt_ = 8i32 | 1i32 << 6i32;
    if 0 == (*io_0).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io_0).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lstate.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 191i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 48],
                                                           &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
               };
               (*io_0).tt_ & 63i32 == (*(*io_0).value_.gc).tt as libc::c_int
                   &&
                   (L.is_null() ||
                        {
                            if 0 != (*io_0).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lstate.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              191i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 48],
                                                                        &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
                            };
                            0 !=
                                ((*(*io_0).value_.gc).marked as libc::c_int ^
                                     (1i32 << 0i32 | 1i32 << 1i32)) &
                                    ((*(*L).l_G).currentwhite as libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32))
                        })
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lstate.c\x00" as *const u8 as *const libc::c_char,
                          191i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
        };
    };
    luaH_setint(L, registry, 1i32 as lua_Integer, &mut temp);
    let mut io_1: *mut TValue = &mut temp;
    let mut x__1: *mut Table_0 = luaH_new(L);
    if (*x__1).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      194i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
    };
    (*io_1).value_.gc = &mut (*(x__1 as *mut GCUnion)).gc as *mut GCObject;
    (*io_1).tt_ = 5i32 | 1i32 << 6i32;
    if 0 == (*io_1).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io_1).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lstate.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 194i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 48],
                                                           &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
               };
               (*io_1).tt_ & 63i32 == (*(*io_1).value_.gc).tt as libc::c_int
                   &&
                   (L.is_null() ||
                        {
                            if 0 != (*io_1).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lstate.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              194i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 48],
                                                                        &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
                            };
                            0 !=
                                ((*(*io_1).value_.gc).marked as libc::c_int ^
                                     (1i32 << 0i32 | 1i32 << 1i32)) &
                                    ((*(*L).l_G).currentwhite as libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32))
                        })
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"lstate.c\x00" as *const u8 as *const libc::c_char,
                          194i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"void init_registry(lua_State *, global_State *)\x00")).as_ptr());
        };
    };
    luaH_setint(L, registry, 2i32 as lua_Integer, &mut temp);
}
unsafe extern "C" fn stack_init(mut L1: *mut lua_State_0,
                                mut L: *mut lua_State_0) -> () {
    let mut i: libc::c_int = 0;
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           ((2i32 * 20i32) as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<TValue>() as
                                                libc::c_ulong) {
        luaM_toobig(L);
    } else { };
    (*L1).stack =
        luaM_realloc_(L, 0 as *mut libc::c_void,
                      (0i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong),
                      ((2i32 * 20i32) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong))
            as *mut TValue;
    (*L1).stacksize = 2i32 * 20i32;
    i = 0i32;
    while i < 2i32 * 20i32 {
        (*(*L1).stack.offset(i as isize)).tt_ = 0i32;
        i += 1
    }
    (*L1).top = (*L1).stack;
    (*L1).stack_last =
        (*L1).stack.offset((*L1).stacksize as isize).offset(-5isize);
    ci = &mut (*L1).base_ci as *mut CallInfo_0;
    (*ci).previous = 0 as *mut CallInfo;
    (*ci).next = (*ci).previous;
    (*ci).callstatus = 0i32 as libc::c_ushort;
    (*ci).func = (*L1).top;
    let fresh1 = (*L1).top;
    (*L1).top = (*L1).top.offset(1);
    (*fresh1).tt_ = 0i32;
    (*ci).top = (*L1).top.offset(20isize);
    (*L1).ci = ci;
}
unsafe extern "C" fn makeseed(mut L: *mut lua_State_0) -> libc::c_uint {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut h: libc::c_uint = time(0 as *mut time_t) as libc::c_uint;
    let mut p: libc::c_int = 0i32;
    let mut t: size_t = L as size_t;
    memcpy(buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
           &mut t as *mut size_t as *const libc::c_void,
           ::std::mem::size_of::<size_t>() as libc::c_ulong);
    p =
        (p as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as
                                             libc::c_ulong) as libc::c_int as
            libc::c_int;
    let mut t_0: size_t = h as size_t;
    memcpy(buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
           &mut t_0 as *mut size_t as *const libc::c_void,
           ::std::mem::size_of::<size_t>() as libc::c_ulong);
    p =
        (p as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as
                                             libc::c_ulong) as libc::c_int as
            libc::c_int;
    let mut t_1: size_t = (&luaO_nilobject_ as *const ::lstate::lua_TValue) as *mut size_t as size_t;
    memcpy(buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
           &mut t_1 as *mut size_t as *const libc::c_void,
           ::std::mem::size_of::<size_t>() as libc::c_ulong);
    p =
        (p as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as
                                             libc::c_ulong) as libc::c_int as
            libc::c_int;
    let mut t_2: size_t = lua_newstate as size_t;
    memcpy(buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
           &mut t_2 as *mut size_t as *const libc::c_void,
           ::std::mem::size_of::<size_t>() as libc::c_ulong);
    p =
        (p as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as
                                             libc::c_ulong) as libc::c_int as
            libc::c_int;
    if p as libc::c_ulong ==
           ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong {
    } else {
        __assert_fail(b"p == sizeof(buff)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      89i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 35],
                                                &[libc::c_char; 35]>(b"unsigned int makeseed(lua_State *)\x00")).as_ptr());
    };
    return luaS_hash(buff.as_mut_ptr(), p as size_t, h);
}
unsafe extern "C" fn preinit_thread(mut L: *mut lua_State_0,
                                    mut g: *mut global_State) -> () {
    (*L).l_G = g;
    (*L).stack = 0 as StkId;
    (*L).ci = 0 as *mut CallInfo_0;
    (*L).nci = 0i32 as libc::c_ushort;
    (*L).stacksize = 0i32;
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
    L = (*(*L).l_G).mainthread;
    let ref mut fresh2 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
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
pub unsafe extern "C" fn lua_newthread(mut L: *mut lua_State_0)
 -> *mut lua_State_0 {
    let mut g: *mut global_State = (*L).l_G;
    let mut L1: *mut lua_State_0 = 0 as *mut lua_State_0;
    let ref mut fresh4 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
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
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long { luaC_step(L); }
    L1 =
        &mut (*(luaM_realloc_(L, 0 as *mut libc::c_void, 8i32 as size_t,
                              ::std::mem::size_of::<LX>() as libc::c_ulong) as
                    *mut LX)).l as *mut lua_State_0;
    (*L1).marked =
        ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as
            lu_byte;
    (*L1).tt = 8i32 as lu_byte;
    (*L1).next = (*g).allgc;
    if (*L1).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((L1)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      266i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
    };
    (*g).allgc = &mut (*(L1 as *mut GCUnion)).gc as *mut GCObject;
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut lua_State_0 = L1;
    if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      268i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
    (*io).tt_ = 8i32 | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"lstate.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 268i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 38],
                                                           &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
               };
               (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int &&
                   (L.is_null() ||
                        {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"lstate.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              268i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 38],
                                                                        &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
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
                          b"lstate.c\x00" as *const u8 as *const libc::c_char,
                          268i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 38],
                                                    &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top &&
           !(b"stack overflow\x00" as *const u8 as
                 *const libc::c_char).is_null() {
    } else {
        __assert_fail(b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as
                          *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      269i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
    };
    preinit_thread(L1, g);
    (*L1).hookmask = (*L).hookmask;
    (*L1).basehookcount = (*L).basehookcount;
    ::std::ptr::write_volatile(&mut (*L1).hook as *mut lua_Hook, (*L).hook);
    (*L1).hookcount = (*L1).basehookcount;
    memcpy((L1 as
                *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                as libc::c_ulong as isize)) as
               *mut libc::c_void,
           ((*g).mainthread as
                *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                as libc::c_ulong as isize)) as
               *mut libc::c_void,
           ::std::mem::size_of::<L_EXTRA>() as libc::c_ulong);
    if (*((L1 as
               *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>() as
                                               libc::c_ulong as isize)) as
              *mut libc::c_void as *mut L_EXTRA)).plock ==
           (*((L as
                   *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                   as libc::c_ulong as isize))
                  as *mut libc::c_void as *mut L_EXTRA)).plock {
    } else {
        __assert_fail(b"((struct L_EXTRA*)(((void *)((char *)(L1) - sizeof(struct L_EXTRA)))))->plock == ((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      278i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"lua_State *lua_newthread(lua_State *)\x00")).as_ptr());
    };
    stack_init(L1, L);
    let ref mut fresh6 =
        *(*((L as
                 *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                 as libc::c_ulong as isize))
                as *mut libc::c_void as *mut L_EXTRA)).plock;
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
pub unsafe extern "C" fn luaE_setdebt(mut g: *mut global_State,
                                      mut debt: l_mem) -> () {
    let mut tb: l_mem = ((*g).totalbytes + (*g).GCdebt) as lu_mem as l_mem;
    if tb > 0i32 as libc::c_long {
    } else {
        __assert_fail(b"tb > 0\x00" as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      100i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void luaE_setdebt(global_State *, l_mem)\x00")).as_ptr());
    };
    if debt < tb - (!(0i32 as lu_mem) >> 1i32) as l_mem {
        debt = tb - (!(0i32 as lu_mem) >> 1i32) as l_mem
    }
    (*g).totalbytes = tb - debt;
    (*g).GCdebt = debt;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_freethread(mut L: *mut lua_State_0,
                                         mut L1: *mut lua_State_0) -> () {
    let mut l: *mut LX = (L1 as *mut lu_byte).offset(-16isize) as *mut LX;
    luaF_close(L1, (*L1).stack);
    if (*L1).openupval.is_null() {
    } else {
        __assert_fail(b"L1->openupval == ((void*)0)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      288i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void luaE_freethread(lua_State *, lua_State *)\x00")).as_ptr());
    };
    if (*((L as
               *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>() as
                                               libc::c_ulong as isize)) as
              *mut libc::c_void as *mut L_EXTRA)).plock ==
           (*((L1 as
                   *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                   as libc::c_ulong as isize))
                  as *mut libc::c_void as *mut L_EXTRA)).plock {
    } else {
        __assert_fail(b"((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock == ((struct L_EXTRA*)(((void *)((char *)(L1) - sizeof(struct L_EXTRA)))))->plock\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      289i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void luaE_freethread(lua_State *, lua_State *)\x00")).as_ptr());
    };
    freestack(L1);
    luaM_realloc_(L, l as *mut libc::c_void,
                  ::std::mem::size_of::<LX>() as libc::c_ulong,
                  0i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn luaE_extendCI(mut L: *mut lua_State_0)
 -> *mut CallInfo_0 {
    let mut ci: *mut CallInfo_0 =
        luaM_realloc_(L, 0 as *mut libc::c_void, 0i32 as size_t,
                      ::std::mem::size_of::<CallInfo_0>() as libc::c_ulong) as
            *mut CallInfo_0;
    if (*(*L).ci).next.is_null() {
    } else {
        __assert_fail(b"L->ci->next == ((void*)0)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstate.c\x00" as *const u8 as *const libc::c_char,
                      110i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 37],
                                                &[libc::c_char; 37]>(b"CallInfo *luaE_extendCI(lua_State *)\x00")).as_ptr());
    };
    (*(*L).ci).next = ci;
    (*ci).previous = (*L).ci;
    (*ci).next = 0 as *mut CallInfo;
    (*L).nci = (*L).nci.wrapping_add(1);
    return ci;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_shrinkCI(mut L: *mut lua_State_0) -> () {
    let mut ci: *mut CallInfo_0 = (*L).ci;
    let mut next2: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    while !(*ci).next.is_null() &&
              { next2 = (*(*ci).next).next; !next2.is_null() } {
        luaM_realloc_(L, (*ci).next as *mut libc::c_void,
                      ::std::mem::size_of::<CallInfo>() as libc::c_ulong,
                      0i32 as size_t);
        (*L).nci = (*L).nci.wrapping_sub(1);
        (*ci).next = next2;
        (*next2).previous = ci;
        ci = next2
    };
}
