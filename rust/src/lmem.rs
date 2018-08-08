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
    fn luaG_runerror(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State_0, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaC_fullgc(L: *mut lua_State_0, isemergency: libc::c_int) -> ();
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
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
pub type Memcontrol_0 = Memcontrol;
#[no_mangle]
pub unsafe extern "C" fn luaM_toobig(mut L: *mut lua_State_0) -> ! {
    luaG_runerror(L,
                  b"memory allocation error: block too big\x00" as *const u8
                      as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn luaM_realloc_(mut L: *mut lua_State_0,
                                       mut block: *mut libc::c_void,
                                       mut osize: size_t, mut nsize: size_t)
 -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut g: *mut global_State = (*L).l_G;
    let mut realosize: size_t =
        if !block.is_null() { osize } else { 0i32 as libc::c_ulong };
    if (realosize == 0i32 as libc::c_ulong) as libc::c_int ==
           (block == 0 as *mut libc::c_void) as libc::c_int {
    } else {
        __assert_fail(b"(realosize == 0) == (block == ((void*)0))\x00" as
                          *const u8 as *const libc::c_char,
                      b"lmem.c\x00" as *const u8 as *const libc::c_char,
                      82i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"void *luaM_realloc_(lua_State *, void *, size_t, size_t)\x00")).as_ptr());
    };
    newblock =
        (*g).frealloc.expect("non-null function pointer")((*g).ud, block,
                                                          osize, nsize);
    if newblock.is_null() && nsize > 0i32 as libc::c_ulong {
        if nsize > realosize {
        } else {
            __assert_fail(b"nsize > realosize\x00" as *const u8 as
                              *const libc::c_char,
                          b"lmem.c\x00" as *const u8 as *const libc::c_char,
                          89i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 57],
                                                    &[libc::c_char; 57]>(b"void *luaM_realloc_(lua_State *, void *, size_t, size_t)\x00")).as_ptr());
        };
        if !(*g).version.is_null() {
            luaC_fullgc(L, 1i32);
            newblock =
                (*g).frealloc.expect("non-null function pointer")((*g).ud,
                                                                  block,
                                                                  osize,
                                                                  nsize)
        }
        if newblock.is_null() { luaD_throw(L, 4i32); }
    }
    if (nsize == 0i32 as libc::c_ulong) as libc::c_int ==
           (newblock == 0 as *mut libc::c_void) as libc::c_int {
    } else {
        __assert_fail(b"(nsize == 0) == (newblock == ((void*)0))\x00" as
                          *const u8 as *const libc::c_char,
                      b"lmem.c\x00" as *const u8 as *const libc::c_char,
                      97i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"void *luaM_realloc_(lua_State *, void *, size_t, size_t)\x00")).as_ptr());
    };
    (*g).GCdebt =
        ((*g).GCdebt as
             libc::c_ulong).wrapping_add(nsize).wrapping_sub(realosize) as
            l_mem;
    return newblock;
}
#[no_mangle]
pub unsafe extern "C" fn luaM_growaux_(mut L: *mut lua_State_0,
                                       mut block: *mut libc::c_void,
                                       mut size: *mut libc::c_int,
                                       mut size_elems: size_t,
                                       mut limit: libc::c_int,
                                       mut what: *const libc::c_char)
 -> *mut libc::c_void {
    let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut newsize: libc::c_int = 0;
    if *size >= limit / 2i32 {
        if *size >= limit {
            luaG_runerror(L,
                          b"too many %s (limit is %d)\x00" as *const u8 as
                              *const libc::c_char, what, limit);
        } else { newsize = limit }
    } else { newsize = *size * 2i32; if newsize < 4i32 { newsize = 4i32 } }
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (newsize as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as size_t)).wrapping_div(size_elems) {
        luaM_toobig(L);
    } else { };
    newblock =
        luaM_realloc_(L, block,
                      (*size as libc::c_ulong).wrapping_mul(size_elems),
                      (newsize as libc::c_ulong).wrapping_mul(size_elems));
    *size = newsize;
    return newblock;
}
