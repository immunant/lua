#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    pub type lua_longjmp;
    pub type UpVal_0;
    #[no_mangle]
    fn random() -> libc::c_long;
    #[no_mangle]
    fn srandom(__seed: libc::c_uint) -> ();
    #[no_mangle]
    fn acos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn asin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn tan(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn exp(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log10(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn log2(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut signgam: libc::c_int;
    #[no_mangle]
    static mut l_memcontrol: Memcontrol_0;
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State_0, idx: libc::c_int,
                      isnum: *mut libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_compare(L: *mut lua_State_0, idx1: libc::c_int, idx2: libc::c_int,
                   op: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State_0, n: lua_Number) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int,
                       nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> ();
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t)
     -> ();
    #[no_mangle]
    fn luaL_argerror(L: *mut lua_State_0, arg: libc::c_int,
                     extramsg: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_checknumber(L: *mut lua_State_0, arg: libc::c_int) -> lua_Number;
    #[no_mangle]
    fn luaL_optnumber(L: *mut lua_State_0, arg: libc::c_int, def: lua_Number)
     -> lua_Number;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State_0, arg: libc::c_int)
     -> lua_Integer;
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State_0, arg: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0,
                     nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type size_t = libc::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type Memcontrol_0 = Memcontrol;
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type stringtable = stringtable_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
pub type luaL_Reg_0 = luaL_Reg;
pub type l_mem = ptrdiff_t;
pub type lua_Alloc =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void,
                                _: size_t, _: size_t) -> *mut libc::c_void>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct stringtable_0 {
    pub hash: *mut *mut TString,
    pub nuse: libc::c_int,
    pub size: libc::c_int,
}
pub type lua_Unsigned = libc::c_ulonglong;
pub type lu_mem = size_t;
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
#[no_mangle]
pub unsafe extern "C" fn luaopen_math(mut L: *mut lua_State_0)
 -> libc::c_int {
    luaL_checkversion_(L, 503i32 as lua_Number,
                       (::std::mem::size_of::<lua_Integer>() as
                            libc::c_ulong).wrapping_mul(16i32 as
                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<lua_Number>()
                                                                                            as
                                                                                            libc::c_ulong));
    lua_createtable(L, 0i32,
                    (::std::mem::size_of::<[luaL_Reg_0; 28]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_Reg_0>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int);
    luaL_setfuncs(L, mathlib.as_ptr(), 0i32);
    lua_pushnumber(L, 3.141592653589793f64);
    lua_setfield(L, -2i32, b"pi\x00" as *const u8 as *const libc::c_char);
    lua_pushnumber(L, ::std::f64::INFINITY);
    lua_setfield(L, -2i32, b"huge\x00" as *const u8 as *const libc::c_char);
    lua_pushinteger(L, 9223372036854775807i64);
    lua_setfield(L, -2i32,
                 b"maxinteger\x00" as *const u8 as *const libc::c_char);
    lua_pushinteger(L, -9223372036854775807i64 - 1i64);
    lua_setfield(L, -2i32,
                 b"mininteger\x00" as *const u8 as *const libc::c_char);
    return 1i32;
}
static mut mathlib: [luaL_Reg_0; 28] =
    unsafe {
        [luaL_Reg{name: b"abs\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_abs),},
         luaL_Reg{name: b"acos\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_acos),},
         luaL_Reg{name: b"asin\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_asin),},
         luaL_Reg{name: b"atan\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_atan),},
         luaL_Reg{name: b"ceil\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_ceil),},
         luaL_Reg{name: b"cos\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_cos),},
         luaL_Reg{name: b"deg\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_deg),},
         luaL_Reg{name: b"exp\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_exp),},
         luaL_Reg{name: b"tointeger\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_toint),},
         luaL_Reg{name: b"floor\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_floor),},
         luaL_Reg{name: b"fmod\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_fmod),},
         luaL_Reg{name: b"ult\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_ult),},
         luaL_Reg{name: b"log\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_log),},
         luaL_Reg{name: b"max\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_max),},
         luaL_Reg{name: b"min\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_min),},
         luaL_Reg{name: b"modf\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_modf),},
         luaL_Reg{name: b"rad\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_rad),},
         luaL_Reg{name: b"random\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_random),},
         luaL_Reg{name: b"randomseed\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_randomseed),},
         luaL_Reg{name: b"sin\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_sin),},
         luaL_Reg{name: b"sqrt\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_sqrt),},
         luaL_Reg{name: b"tan\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_tan),},
         luaL_Reg{name: b"type\x00" as *const u8 as *const libc::c_char,
                  func: Some(math_type),},
         luaL_Reg{name: b"pi\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: b"huge\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: b"maxinteger\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: b"mininteger\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: 0 as *const libc::c_char, func: None,}]
    };
unsafe extern "C" fn math_type(mut L: *mut lua_State_0) -> libc::c_int {
    if lua_type(L, 1i32) == 3i32 {
        if 0 != lua_isinteger(L, 1i32) {
            lua_pushstring(L,
                           b"integer\x00" as *const u8 as
                               *const libc::c_char);
        } else {
            lua_pushstring(L,
                           b"float\x00" as *const u8 as *const libc::c_char);
        }
    } else { luaL_checkany(L, 1i32); lua_pushnil(L); }
    return 1i32;
}
unsafe extern "C" fn math_tan(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, tan(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_sqrt(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, sqrt(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_sin(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, sin(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_randomseed(mut L: *mut lua_State_0) -> libc::c_int {
    srandom(luaL_checknumber(L, 1i32) as lua_Integer as libc::c_uint);
    random();
    return 0i32;
}
unsafe extern "C" fn math_random(mut L: *mut lua_State_0) -> libc::c_int {
    let mut low: lua_Integer = 0;
    let mut up: lua_Integer = 0;
    let mut r: libc::c_double =
        random() as libc::c_double *
            (1.0f64 / (2147483647i32 as libc::c_double + 1.0f64));
    match lua_gettop(L) {
        0 => { lua_pushnumber(L, r); return 1i32 }
        1 => { low = 1i32 as lua_Integer; up = luaL_checkinteger(L, 1i32) }
        2 => {
            low = luaL_checkinteger(L, 1i32);
            up = luaL_checkinteger(L, 2i32)
        }
        _ => {
            return luaL_error(L,
                              b"wrong number of arguments\x00" as *const u8 as
                                  *const libc::c_char)
        }
    }
    r *= (up - low) as libc::c_double + 1.0f64;
    lua_pushinteger(L, r as lua_Integer + low);
    return 1i32;
}
unsafe extern "C" fn math_rad(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L,
                   luaL_checknumber(L, 1i32) *
                       (3.141592653589793f64 / 180.0f64));
    return 1i32;
}
unsafe extern "C" fn math_modf(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: lua_Number = 0.;
    let mut ip: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        lua_settop(L, 1i32);
        lua_pushnumber(L, 0i32 as lua_Number);
    } else {
        n = luaL_checknumber(L, 1i32);
        ip = if n < 0i32 as libc::c_double { ceil(n) } else { floor(n) };
        pushnumint(L, ip);
        lua_pushnumber(L, if n == ip { 0.0f64 } else { n - ip });
    }
    return 2i32;
}
unsafe extern "C" fn pushnumint(mut L: *mut lua_State_0, mut d: lua_Number)
 -> () {
    let mut n: lua_Integer = 0;
    if d >= (-9223372036854775807i64 - 1i64) as libc::c_double &&
           d < -((-9223372036854775807i64 - 1i64) as libc::c_double) &&
           { n = d as libc::c_longlong; 0 != 1i32 } {
        lua_pushinteger(L, n);
    } else { lua_pushnumber(L, d); };
}
unsafe extern "C" fn math_min(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut imin: libc::c_int = 1i32;
    let mut i: libc::c_int = 0;
    i = 2i32;
    while i <= n { if 0 != lua_compare(L, i, imin, 1i32) { imin = i } i += 1 }
    lua_pushvalue(L, imin);
    return 1i32;
}
unsafe extern "C" fn math_max(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut imax: libc::c_int = 1i32;
    let mut i: libc::c_int = 0;
    i = 2i32;
    while i <= n { if 0 != lua_compare(L, imax, i, 1i32) { imax = i } i += 1 }
    lua_pushvalue(L, imax);
    return 1i32;
}
unsafe extern "C" fn math_log(mut L: *mut lua_State_0) -> libc::c_int {
    let mut x: lua_Number = luaL_checknumber(L, 1i32);
    let mut res: lua_Number = 0.;
    if lua_type(L, 2i32) <= 0i32 {
        res = log(x)
    } else {
        let mut base: lua_Number = luaL_checknumber(L, 2i32);
        if base == 2.0f64 {
            res = log2(x)
        } else if base == 10.0f64 {
            res = log10(x)
        } else { res = log(x) / log(base) }
    }
    lua_pushnumber(L, res);
    return 1i32;
}
unsafe extern "C" fn math_ult(mut L: *mut lua_State_0) -> libc::c_int {
    let mut a: lua_Integer = luaL_checkinteger(L, 1i32);
    let mut b: lua_Integer = luaL_checkinteger(L, 2i32);
    lua_pushboolean(L,
                    ((a as lua_Unsigned) < b as lua_Unsigned) as libc::c_int);
    return 1i32;
}
unsafe extern "C" fn math_fmod(mut L: *mut lua_State_0) -> libc::c_int {
    if 0 != lua_isinteger(L, 1i32) && 0 != lua_isinteger(L, 2i32) {
        let mut d: lua_Integer =
            lua_tointegerx(L, 2i32, 0 as *mut libc::c_int);
        if (d as lua_Unsigned).wrapping_add(1u32 as libc::c_ulonglong) <=
               1u32 as libc::c_ulonglong {
            lua_pushinteger(L, 0i32 as lua_Integer);
        } else {
            lua_pushinteger(L,
                            lua_tointegerx(L, 1i32, 0 as *mut libc::c_int) %
                                d);
        }
    } else {
        lua_pushnumber(L,
                       fmod(luaL_checknumber(L, 1i32),
                            luaL_checknumber(L, 2i32)));
    }
    return 1i32;
}
unsafe extern "C" fn math_floor(mut L: *mut lua_State_0) -> libc::c_int {
    let mut d: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        lua_settop(L, 1i32);
    } else { d = floor(luaL_checknumber(L, 1i32)); pushnumint(L, d); }
    return 1i32;
}
unsafe extern "C" fn math_toint(mut L: *mut lua_State_0) -> libc::c_int {
    let mut valid: libc::c_int = 0;
    let mut n: lua_Integer = lua_tointegerx(L, 1i32, &mut valid);
    if 0 != valid {
        lua_pushinteger(L, n);
    } else { luaL_checkany(L, 1i32); lua_pushnil(L); }
    return 1i32;
}
unsafe extern "C" fn math_exp(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, exp(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_deg(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L,
                   luaL_checknumber(L, 1i32) *
                       (180.0f64 / 3.141592653589793f64));
    return 1i32;
}
unsafe extern "C" fn math_cos(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, cos(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_ceil(mut L: *mut lua_State_0) -> libc::c_int {
    let mut d: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        lua_settop(L, 1i32);
    } else { d = ceil(luaL_checknumber(L, 1i32)); pushnumint(L, d); }
    return 1i32;
}
unsafe extern "C" fn math_atan(mut L: *mut lua_State_0) -> libc::c_int {
    let mut y: lua_Number = luaL_checknumber(L, 1i32);
    let mut x: lua_Number = luaL_optnumber(L, 2i32, 1i32 as lua_Number);
    lua_pushnumber(L, atan2(y, x));
    return 1i32;
}
unsafe extern "C" fn math_asin(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, asin(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_acos(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L, acos(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_abs(mut L: *mut lua_State_0) -> libc::c_int {
    if 0 != lua_isinteger(L, 1i32) {
        let mut n: lua_Integer =
            lua_tointegerx(L, 1i32, 0 as *mut libc::c_int);
        if n < 0i32 as libc::c_longlong {
            n =
                (0u32 as libc::c_ulonglong).wrapping_sub(n as lua_Unsigned) as
                    lua_Integer
        }
        lua_pushinteger(L, n);
    } else { lua_pushnumber(L, fabs(luaL_checknumber(L, 1i32))); }
    return 1i32;
}
