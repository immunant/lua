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
    pub type UpVal_0;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn luaC_fix(L: *mut lua_State_0, o: *mut GCObject) -> ();
    #[no_mangle]
    fn luaC_newobj(L: *mut lua_State_0, tt: libc::c_int, sz: size_t)
     -> *mut GCObject;
}
pub type size_t = libc::c_ulong;
pub type Upvaldesc = Upvaldesc_0;
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
    pub u: unnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_2 {
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
    nk: unnamed_3,
    tvk: TValue,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_3 {
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
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure_0,
    l: LClosure,
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
pub type LClosure = LClosure_0;
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
    pub locvars: *mut LocVar_0,
    pub upvalues: *mut Upvaldesc,
    pub cache: *mut LClosure_0,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
pub type CClosure_0 = CClosure;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type UTString = UTString_0;
pub type Udata_0 = Udata;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Upvaldesc_0 {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
pub type LocVar_0 = LocVar;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union UUdata {
    dummy: L_Umaxalign,
    uv: Udata_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union UTString_0 {
    dummy: L_Umaxalign,
    tsv: TString,
}
#[no_mangle]
pub unsafe extern "C" fn luaS_hash(mut str: *const libc::c_char,
                                   mut l: size_t, mut seed: libc::c_uint)
 -> libc::c_uint {
    let mut h: libc::c_uint = seed ^ l as libc::c_uint;
    let mut step: size_t = (l >> 5i32).wrapping_add(1i32 as libc::c_ulong);
    while l >= step {
        h ^=
            (h <<
                 5i32).wrapping_add(h >>
                                        2i32).wrapping_add(*str.offset(l.wrapping_sub(1i32
                                                                                          as
                                                                                          libc::c_ulong)
                                                                           as
                                                                           isize)
                                                               as lu_byte as
                                                               libc::c_uint);
        l = (l as libc::c_ulong).wrapping_sub(step) as size_t as size_t
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_hashlongstr(mut ts: *mut TString)
 -> libc::c_uint {
    if (*ts).tt as libc::c_int == 4i32 | 1i32 << 4i32 {
    } else {
        __assert_fail(b"ts->tt == (4 | (1 << 4))\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      59i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"unsigned int luaS_hashlongstr(TString *)\x00")).as_ptr());
    };
    if (*ts).extra as libc::c_int == 0i32 {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((ts)->extra)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lstring.c\x00" as *const u8 as
                              *const libc::c_char, 61i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 41],
                                                    &[libc::c_char; 41]>(b"unsigned int luaS_hashlongstr(TString *)\x00")).as_ptr());
        };
        (*ts).hash =
            luaS_hash((ts as
                           *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                         as libc::c_ulong as
                                                         isize),
                      (*ts).u.lnglen, (*ts).hash);
        (*ts).extra = 1i32 as lu_byte
    }
    return (*ts).hash;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_eqlngstr(mut a: *mut TString,
                                       mut b: *mut TString) -> libc::c_int {
    let mut len: size_t = (*a).u.lnglen;
    if (*a).tt as libc::c_int == 4i32 | 1i32 << 4i32 &&
           (*b).tt as libc::c_int == 4i32 | 1i32 << 4i32 {
    } else {
        __assert_fail(b"a->tt == (4 | (1 << 4)) && b->tt == (4 | (1 << 4))\x00"
                          as *const u8 as *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      42i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"int luaS_eqlngstr(TString *, TString *)\x00")).as_ptr());
    };
    return (a == b ||
                len == (*b).u.lnglen &&
                    {
                        if 0 !=
                               ::std::mem::size_of::<lu_byte>() as
                                   libc::c_ulong {
                        } else {
                            __assert_fail(b"sizeof((a)->extra)\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"lstring.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          45i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 40],
                                                                    &[libc::c_char; 40]>(b"int luaS_eqlngstr(TString *, TString *)\x00")).as_ptr());
                        };
                        if 0 !=
                               ::std::mem::size_of::<lu_byte>() as
                                   libc::c_ulong {
                        } else {
                            __assert_fail(b"sizeof((b)->extra)\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"lstring.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          45i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 40],
                                                                    &[libc::c_char; 40]>(b"int luaS_eqlngstr(TString *, TString *)\x00")).as_ptr());
                        };
                        memcmp((a as
                                    *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as isize) as
                                   *const libc::c_void,
                               (b as
                                    *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as isize) as
                                   *const libc::c_void, len) == 0i32
                    }) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_resize(mut L: *mut lua_State_0,
                                     mut newsize: libc::c_int) -> () {
    let mut i: libc::c_int = 0;
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt as *mut stringtable;
    if newsize > (*tb).size {
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
               ::std::mem::size_of::<size_t>() as libc::c_ulong &&
               (newsize as size_t).wrapping_add(1i32 as libc::c_ulong) >
                   (!(0i32 as
                          size_t)).wrapping_div(::std::mem::size_of::<*mut TString>()
                                                    as libc::c_ulong) {
            luaM_toobig(L);
        } else { };
        (*tb).hash =
            luaM_realloc_(L, (*tb).hash as *mut libc::c_void,
                          ((*tb).size as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                               as
                                                               libc::c_ulong),
                          (newsize as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                               as
                                                               libc::c_ulong))
                as *mut *mut TString;
        i = (*tb).size;
        while i < newsize {
            let ref mut fresh0 = *(*tb).hash.offset(i as isize);
            *fresh0 = 0 as *mut TString;
            i += 1
        }
    }
    i = 0i32;
    while i < (*tb).size {
        let mut p: *mut TString = *(*tb).hash.offset(i as isize);
        let ref mut fresh1 = *(*tb).hash.offset(i as isize);
        *fresh1 = 0 as *mut TString;
        while !p.is_null() {
            let mut hnext: *mut TString = (*p).u.hnext;
            if newsize & newsize - 1i32 == 0i32 {
            } else {
                __assert_fail(b"(newsize&(newsize-1))==0\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lstring.c\x00" as *const u8 as
                                  *const libc::c_char, 84i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 35],
                                                        &[libc::c_char; 35]>(b"void luaS_resize(lua_State *, int)\x00")).as_ptr());
            };
            let mut h: libc::c_uint =
                ((*p).hash & (newsize - 1i32) as libc::c_uint) as libc::c_int
                    as libc::c_uint;
            (*p).u.hnext = *(*tb).hash.offset(h as isize);
            let ref mut fresh2 = *(*tb).hash.offset(h as isize);
            *fresh2 = p;
            p = hnext
        }
        i += 1
    }
    if newsize < (*tb).size {
        if (*(*tb).hash.offset(newsize as isize)).is_null() &&
               (*(*tb).hash.offset(((*tb).size - 1i32) as isize)).is_null() {
        } else {
            __assert_fail(b"tb->hash[newsize] == ((void*)0) && tb->hash[tb->size - 1] == ((void*)0)\x00"
                              as *const u8 as *const libc::c_char,
                          b"lstring.c\x00" as *const u8 as
                              *const libc::c_char, 92i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void luaS_resize(lua_State *, int)\x00")).as_ptr());
        };
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
               ::std::mem::size_of::<size_t>() as libc::c_ulong &&
               (newsize as size_t).wrapping_add(1i32 as libc::c_ulong) >
                   (!(0i32 as
                          size_t)).wrapping_div(::std::mem::size_of::<*mut TString>()
                                                    as libc::c_ulong) {
            luaM_toobig(L);
        } else { };
        (*tb).hash =
            luaM_realloc_(L, (*tb).hash as *mut libc::c_void,
                          ((*tb).size as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                               as
                                                               libc::c_ulong),
                          (newsize as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                               as
                                                               libc::c_ulong))
                as *mut *mut TString
    }
    (*tb).size = newsize;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_clearcache(mut g: *mut global_State) -> () {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0i32;
    while i < 23i32 {
        j = 0i32;
        while j < 5i32 {
            if 0 !=
                   (*(*g).strcache[i as usize][j as usize]).marked as
                       libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
                (*g).strcache[i as usize][j as usize] = (*g).memerrmsg
            }
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_init(mut L: *mut lua_State_0) -> () {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    luaS_resize(L, 2i32);
    (*g).memerrmsg =
        luaS_newlstr(L,
                     b"not enough memory\x00" as *const u8 as
                         *const libc::c_char,
                     (::std::mem::size_of::<[libc::c_char; 18]>() as
                          libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                          as
                                                          libc::c_ulong).wrapping_sub(1i32
                                                                                          as
                                                                                          libc::c_ulong));
    if (*(*g).memerrmsg).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((g->memerrmsg)->tt) & 0x0F) < (9+1)\x00" as
                          *const u8 as *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      122i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"void luaS_init(lua_State *)\x00")).as_ptr());
    };
    luaC_fix(L, &mut (*((*g).memerrmsg as *mut GCUnion)).gc);
    i = 0i32;
    while i < 23i32 {
        j = 0i32;
        while j < 5i32 {
            (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newlstr(mut L: *mut lua_State_0,
                                      mut str: *const libc::c_char,
                                      mut l: size_t) -> *mut TString {
    if l <= 40i32 as libc::c_ulong {
        return internshrstr(L, str, l)
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        if l >=
               if (::std::mem::size_of::<size_t>() as libc::c_ulong) <
                      ::std::mem::size_of::<lua_Integer>() as libc::c_ulong {
                   !(0i32 as size_t)
               } else {
                   9223372036854775807i64 as size_t
               }.wrapping_sub(::std::mem::size_of::<TString>() as
                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                                  as
                                                                  libc::c_ulong)
           {
            luaM_toobig(L);
        } else {
            ts = luaS_createlngstrobj(L, l);
            if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
            } else {
                __assert_fail(b"sizeof((ts)->extra)\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lstring.c\x00" as *const u8 as
                                  *const libc::c_char, 207i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 57],
                                                        &[libc::c_char; 57]>(b"TString *luaS_newlstr(lua_State *, const char *, size_t)\x00")).as_ptr());
            };
            memcpy((ts as
                        *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                      as libc::c_ulong as
                                                      isize) as
                       *mut libc::c_void, str as *const libc::c_void,
                   l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as
                                      libc::c_ulong));
            return ts
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_createlngstrobj(mut L: *mut lua_State_0,
                                              mut l: size_t) -> *mut TString {
    let mut ts: *mut TString =
        createstrobj(L, l, 4i32 | 1i32 << 4i32, (*(*L).l_G).seed);
    (*ts).u.lnglen = l;
    return ts;
}
unsafe extern "C" fn createstrobj(mut L: *mut lua_State_0, mut l: size_t,
                                  mut tag: libc::c_int, mut h: libc::c_uint)
 -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    let mut totalsize: size_t = 0;
    totalsize =
        (::std::mem::size_of::<UTString_0>() as
             libc::c_ulong).wrapping_add(l.wrapping_add(1i32 as
                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                            as
                                                                                            libc::c_ulong));
    o = luaC_newobj(L, tag, totalsize);
    if (*o).tt as libc::c_int & 15i32 == 4i32 {
    } else {
        __assert_fail(b"(((o)->tt) & 0x0F) == 4\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      139i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"TString *createstrobj(lua_State *, size_t, int, unsigned int)\x00")).as_ptr());
    };
    ts = &mut (*(o as *mut GCUnion)).ts as *mut TString_0;
    (*ts).hash = h;
    (*ts).extra = 0i32 as lu_byte;
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof((ts)->extra)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      142i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"TString *createstrobj(lua_State *, size_t, int, unsigned int)\x00")).as_ptr());
    };
    *(ts as
          *mut libc::c_char).offset(::std::mem::size_of::<UTString>() as
                                        libc::c_ulong as
                                        isize).offset(l as isize) =
        '\u{0}' as i32 as libc::c_char;
    return ts;
}
unsafe extern "C" fn internshrstr(mut L: *mut lua_State_0,
                                  mut str: *const libc::c_char, mut l: size_t)
 -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut g: *mut global_State = (*L).l_G;
    let mut h: libc::c_uint = luaS_hash(str, l, (*g).seed);
    if (*g).strt.size & (*g).strt.size - 1i32 == 0i32 {
    } else {
        __assert_fail(b"(g->strt.size&(g->strt.size-1))==0\x00" as *const u8
                          as *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      171i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"TString *internshrstr(lua_State *, const char *, size_t)\x00")).as_ptr());
    };
    let mut list: *mut *mut TString =
        &mut *(*g).strt.hash.offset((h &
                                         ((*g).strt.size - 1i32) as
                                             libc::c_uint) as libc::c_int as
                                        isize) as *mut *mut TString;
    if !str.is_null() {
    } else {
        __assert_fail(b"str != ((void*)0)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      172i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"TString *internshrstr(lua_State *, const char *, size_t)\x00")).as_ptr());
    };
    ts = *list;
    while !ts.is_null() {
        if l == (*ts).shrlen as libc::c_ulong &&
               {
                   if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
                   } else {
                       __assert_fail(b"sizeof((ts)->extra)\x00" as *const u8
                                         as *const libc::c_char,
                                     b"lstring.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     175i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 57],
                                                               &[libc::c_char; 57]>(b"TString *internshrstr(lua_State *, const char *, size_t)\x00")).as_ptr());
                   };
                   memcmp(str as *const libc::c_void,
                          (ts as
                               *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                             as libc::c_ulong
                                                             as isize) as
                              *const libc::c_void,
                          l.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                             as libc::c_ulong)) == 0i32
               } {
            if 0 ==
                   ((*ts).marked as libc::c_int ^
                        (1i32 << 0i32 | 1i32 << 1i32)) &
                       ((*g).currentwhite as libc::c_int ^
                            (1i32 << 0i32 | 1i32 << 1i32)) {
                (*ts).marked =
                    ((*ts).marked as libc::c_int ^
                         (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
            }
            return ts
        } else { ts = (*ts).u.hnext }
    }
    if (*g).strt.nuse >= (*g).strt.size &&
           (*g).strt.size <= 2147483647i32 / 2i32 {
        luaS_resize(L, (*g).strt.size * 2i32);
        if (*g).strt.size & (*g).strt.size - 1i32 == 0i32 {
        } else {
            __assert_fail(b"(g->strt.size&(g->strt.size-1))==0\x00" as
                              *const u8 as *const libc::c_char,
                          b"lstring.c\x00" as *const u8 as
                              *const libc::c_char, 184i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"TString *internshrstr(lua_State *, const char *, size_t)\x00")).as_ptr());
        };
        list =
            &mut *(*g).strt.hash.offset((h &
                                             ((*g).strt.size - 1i32) as
                                                 libc::c_uint) as libc::c_int
                                            as isize) as *mut *mut TString
    }
    ts = createstrobj(L, l, 4i32 | 0i32 << 4i32, h);
    if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof((ts)->extra)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      187i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"TString *internshrstr(lua_State *, const char *, size_t)\x00")).as_ptr());
    };
    memcpy((ts as
                *mut libc::c_char).offset(::std::mem::size_of::<UTString>() as
                                              libc::c_ulong as isize) as
               *mut libc::c_void, str as *const libc::c_void,
           l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as
                              libc::c_ulong));
    (*ts).shrlen = l as lu_byte;
    (*ts).u.hnext = *list;
    *list = ts;
    (*g).strt.nuse += 1;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_remove(mut L: *mut lua_State_0,
                                     mut ts: *mut TString) -> () {
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt as *mut stringtable;
    if (*tb).size & (*tb).size - 1i32 == 0i32 {
    } else {
        __assert_fail(b"(tb->size&(tb->size-1))==0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstring.c\x00" as *const u8 as *const libc::c_char,
                      156i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"void luaS_remove(lua_State *, TString *)\x00")).as_ptr());
    };
    let mut p: *mut *mut TString =
        &mut *(*tb).hash.offset(((*ts).hash &
                                     ((*tb).size - 1i32) as libc::c_uint) as
                                    libc::c_int as isize) as
            *mut *mut TString;
    while *p != ts { p = &mut (**p).u.hnext as *mut *mut TString_0 }
    *p = (**p).u.hnext;
    (*tb).nuse -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newudata(mut L: *mut lua_State_0, mut s: size_t)
 -> *mut Udata_0 {
    let mut u: *mut Udata_0 = 0 as *mut Udata_0;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    if s >
           if (::std::mem::size_of::<size_t>() as libc::c_ulong) <
                  ::std::mem::size_of::<lua_Integer>() as libc::c_ulong {
               !(0i32 as size_t)
           } else {
               9223372036854775807i64 as size_t
           }.wrapping_sub(::std::mem::size_of::<Udata_0>() as libc::c_ulong) {
        luaM_toobig(L);
    } else {
        o =
            luaC_newobj(L, 7i32,
                        (::std::mem::size_of::<UUdata>() as
                             libc::c_ulong).wrapping_add(s));
        if (*o).tt as libc::c_int == 7i32 {
        } else {
            __assert_fail(b"(o)->tt == 7\x00" as *const u8 as
                              *const libc::c_char,
                          b"lstring.c\x00" as *const u8 as
                              *const libc::c_char, 242i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 42],
                                                    &[libc::c_char; 42]>(b"Udata *luaS_newudata(lua_State *, size_t)\x00")).as_ptr());
        };
        u = &mut (*(o as *mut GCUnion)).u as *mut Udata;
        (*u).len = s;
        (*u).metatable = 0 as *mut Table;
        let mut io: *const TValue = &luaO_nilobject_;
        let mut iu: *mut Udata_0 = u;
        (*iu).user_ = (*io).value_;
        (*iu).ttuv_ = (*io).tt_ as lu_byte;
        if 0 == (*io).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"lstring.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     245i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 42],
                                                               &[libc::c_char; 42]>(b"Udata *luaS_newudata(lua_State *, size_t)\x00")).as_ptr());
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
                                                  b"lstring.c\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  245i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 42],
                                                                            &[libc::c_char; 42]>(b"Udata *luaS_newudata(lua_State *, size_t)\x00")).as_ptr());
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
                              b"lstring.c\x00" as *const u8 as
                                  *const libc::c_char, 245i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 42],
                                                        &[libc::c_char; 42]>(b"Udata *luaS_newudata(lua_State *, size_t)\x00")).as_ptr());
            };
        };
        return u
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_new(mut L: *mut lua_State_0,
                                  mut str: *const libc::c_char)
 -> *mut TString {
    let mut i: libc::c_uint =
        ((str as size_t &
              (2147483647i32 as
                   libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32) as
                  libc::c_ulong) as
             libc::c_uint).wrapping_rem(23i32 as libc::c_uint);
    let mut j: libc::c_int = 0;
    let mut p: *mut *mut TString =
        (*(*L).l_G).strcache[i as usize].as_mut_ptr();
    j = 0i32;
    while j < 5i32 {
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((p[j])->extra)\x00" as *const u8 as
                              *const libc::c_char,
                          b"lstring.c\x00" as *const u8 as
                              *const libc::c_char, 224i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 45],
                                                    &[libc::c_char; 45]>(b"TString *luaS_new(lua_State *, const char *)\x00")).as_ptr());
        };
        if strcmp(str,
                  (*p.offset(j as isize) as
                       *mut libc::c_char).offset(::std::mem::size_of::<UTString>()
                                                     as libc::c_ulong as
                                                     isize)) == 0i32 {
            return *p.offset(j as isize)
        } else { j += 1 }
    }
    j = 5i32 - 1i32;
    while j > 0i32 {
        let ref mut fresh3 = *p.offset(j as isize);
        *fresh3 = *p.offset((j - 1i32) as isize);
        j -= 1
    }
    let ref mut fresh4 = *p.offset(0isize);
    *fresh4 = luaS_newlstr(L, str, strlen(str));
    return *p.offset(0isize);
}
