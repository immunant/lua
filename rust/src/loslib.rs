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
    pub type UpVal_0;
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn system(__command: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
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
    #[no_mangle]
    static mut l_memcontrol: Memcontrol_0;
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
    #[no_mangle]
    fn lua_close(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State_0, idx: libc::c_int,
                      isnum: *mut libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
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
    fn lua_getfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> libc::c_int;
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
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t)
     -> ();
    #[no_mangle]
    fn luaL_argerror(L: *mut lua_State_0, arg: libc::c_int,
                     extramsg: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_checklstring(L: *mut lua_State_0, arg: libc::c_int,
                         l: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_optlstring(L: *mut lua_State_0, arg: libc::c_int,
                       def: *const libc::c_char, l: *mut size_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State_0, arg: libc::c_int)
     -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State_0, arg: libc::c_int,
                       def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State_0, arg: libc::c_int, t: libc::c_int)
     -> ();
    #[no_mangle]
    fn luaL_checkoption(L: *mut lua_State_0, arg: libc::c_int,
                        def: *const libc::c_char,
                        lst: *const *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_fileresult(L: *mut lua_State_0, stat: libc::c_int,
                       fname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_execresult(L: *mut lua_State_0, stat: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0,
                     nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State_0, B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_prepbuffsize(B: *mut luaL_Buffer_0, sz: size_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
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
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
pub type luaL_Reg_0 = luaL_Reg;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State_0,
    pub initb: [libc::c_char; 23],
}
pub type luaL_Buffer_0 = luaL_Buffer;
#[no_mangle]
pub unsafe extern "C" fn luaopen_os(mut L: *mut lua_State_0) -> libc::c_int {
    luaL_checkversion_(L, 503i32 as lua_Number,
                       (::std::mem::size_of::<lua_Integer>() as
                            libc::c_ulong).wrapping_mul(16i32 as
                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<lua_Number>()
                                                                                            as
                                                                                            libc::c_ulong));
    lua_createtable(L, 0i32,
                    (::std::mem::size_of::<[luaL_Reg_0; 12]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_Reg_0>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int);
    luaL_setfuncs(L, syslib.as_ptr(), 0i32);
    return 1i32;
}
static mut syslib: [luaL_Reg_0; 12] =
    unsafe {
        [luaL_Reg{name: b"clock\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_clock),},
         luaL_Reg{name: b"date\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_date),},
         luaL_Reg{name: b"difftime\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_difftime),},
         luaL_Reg{name: b"execute\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_execute),},
         luaL_Reg{name: b"exit\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_exit),},
         luaL_Reg{name: b"getenv\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_getenv),},
         luaL_Reg{name: b"remove\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_remove),},
         luaL_Reg{name: b"rename\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_rename),},
         luaL_Reg{name: b"setlocale\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_setlocale),},
         luaL_Reg{name: b"time\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_time),},
         luaL_Reg{name: b"tmpname\x00" as *const u8 as *const libc::c_char,
                  func: Some(os_tmpname),},
         luaL_Reg{name: 0 as *const libc::c_char, func: None,}]
    };
unsafe extern "C" fn os_tmpname(mut L: *mut lua_State_0) -> libc::c_int {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut err: libc::c_int = 0;
    strcpy(buff.as_mut_ptr(),
           b"/tmp/lua_XXXXXX\x00" as *const u8 as *const libc::c_char);
    err = mkstemp(buff.as_mut_ptr());
    if err != -1i32 { close(err); }
    err = (err == -1i32) as libc::c_int;
    if 0 != err {
        return luaL_error(L,
                          b"unable to generate a unique filename\x00" as
                              *const u8 as *const libc::c_char)
    } else { lua_pushstring(L, buff.as_mut_ptr()); return 1i32 };
}
unsafe extern "C" fn os_time(mut L: *mut lua_State_0) -> libc::c_int {
    let mut ts: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           __tm_gmtoff: 0,
           __tm_zone: 0 as *const libc::c_char,};
    let mut t: time_t = 0;
    if lua_type(L, 1i32) <= 0i32 {
        t = time(0 as *mut time_t)
    } else {
        ts =
            tm{tm_sec: 0,
               tm_min: 0,
               tm_hour: 0,
               tm_mday: 0,
               tm_mon: 0,
               tm_year: 0,
               tm_wday: 0,
               tm_yday: 0,
               tm_isdst: 0,
               __tm_gmtoff: 0,
               __tm_zone: 0 as *const libc::c_char,};
        luaL_checktype(L, 1i32, 5i32);
        lua_settop(L, 1i32);
        ts.tm_sec =
            getfield(L, b"sec\x00" as *const u8 as *const libc::c_char, 0i32,
                     0i32);
        ts.tm_min =
            getfield(L, b"min\x00" as *const u8 as *const libc::c_char, 0i32,
                     0i32);
        ts.tm_hour =
            getfield(L, b"hour\x00" as *const u8 as *const libc::c_char,
                     12i32, 0i32);
        ts.tm_mday =
            getfield(L, b"day\x00" as *const u8 as *const libc::c_char, -1i32,
                     0i32);
        ts.tm_mon =
            getfield(L, b"month\x00" as *const u8 as *const libc::c_char,
                     -1i32, 1i32);
        ts.tm_year =
            getfield(L, b"year\x00" as *const u8 as *const libc::c_char,
                     -1i32, 1900i32);
        ts.tm_isdst =
            getboolfield(L, b"isdst\x00" as *const u8 as *const libc::c_char);
        t = mktime(&mut ts);
        setallfields(L, &mut ts);
    }
    if t != t as lua_Integer as time_t || t == -1i32 as time_t {
        luaL_error(L,
                   b"time result cannot be represented in this installation\x00"
                       as *const u8 as *const libc::c_char);
    }
    lua_pushinteger(L, t as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn setallfields(mut L: *mut lua_State_0, mut stm: *mut tm)
 -> () {
    setfield(L, b"sec\x00" as *const u8 as *const libc::c_char,
             (*stm).tm_sec);
    setfield(L, b"min\x00" as *const u8 as *const libc::c_char,
             (*stm).tm_min);
    setfield(L, b"hour\x00" as *const u8 as *const libc::c_char,
             (*stm).tm_hour);
    setfield(L, b"day\x00" as *const u8 as *const libc::c_char,
             (*stm).tm_mday);
    setfield(L, b"month\x00" as *const u8 as *const libc::c_char,
             (*stm).tm_mon + 1i32);
    setfield(L, b"year\x00" as *const u8 as *const libc::c_char,
             (*stm).tm_year + 1900i32);
    setfield(L, b"wday\x00" as *const u8 as *const libc::c_char,
             (*stm).tm_wday + 1i32);
    setfield(L, b"yday\x00" as *const u8 as *const libc::c_char,
             (*stm).tm_yday + 1i32);
    setboolfield(L, b"isdst\x00" as *const u8 as *const libc::c_char,
                 (*stm).tm_isdst);
}
unsafe extern "C" fn setboolfield(mut L: *mut lua_State_0,
                                  mut key: *const libc::c_char,
                                  mut value: libc::c_int) -> () {
    if value < 0i32 {
        return
    } else {
        lua_pushboolean(L, value);
        lua_setfield(L, -2i32, key);
        return;
    };
}
unsafe extern "C" fn setfield(mut L: *mut lua_State_0,
                              mut key: *const libc::c_char,
                              mut value: libc::c_int) -> () {
    lua_pushinteger(L, value as lua_Integer);
    lua_setfield(L, -2i32, key);
}
unsafe extern "C" fn getboolfield(mut L: *mut lua_State_0,
                                  mut key: *const libc::c_char)
 -> libc::c_int {
    let mut res: libc::c_int = 0;
    res =
        if lua_getfield(L, -1i32, key) == 0i32 {
            -1i32
        } else { lua_toboolean(L, -1i32) };
    lua_settop(L, -1i32 - 1i32);
    return res;
}
unsafe extern "C" fn getfield(mut L: *mut lua_State_0,
                              mut key: *const libc::c_char,
                              mut d: libc::c_int, mut delta: libc::c_int)
 -> libc::c_int {
    let mut isnum: libc::c_int = 0;
    let mut t: libc::c_int = lua_getfield(L, -1i32, key);
    let mut res: lua_Integer = lua_tointegerx(L, -1i32, &mut isnum);
    if 0 == isnum {
        if t != 0i32 {
            return luaL_error(L,
                              b"field \'%s\' is not an integer\x00" as
                                  *const u8 as *const libc::c_char, key)
        } else if d < 0i32 {
            return luaL_error(L,
                              b"field \'%s\' missing in date table\x00" as
                                  *const u8 as *const libc::c_char, key)
        } else { res = d as lua_Integer }
    } else if !(-(2147483647i32 / 2i32) as libc::c_longlong <= res &&
                    res <= (2147483647i32 / 2i32) as libc::c_longlong) {
        return luaL_error(L,
                          b"field \'%s\' is out-of-bound\x00" as *const u8 as
                              *const libc::c_char, key)
    } else { res -= delta as libc::c_longlong }
    lua_settop(L, -1i32 - 1i32);
    return res as libc::c_int;
}
unsafe extern "C" fn os_setlocale(mut L: *mut lua_State_0) -> libc::c_int {
    static mut cat: [libc::c_int; 6] =
        unsafe { [6i32, 3i32, 0i32, 4i32, 1i32, 2i32] };
    static mut catnames: [*const libc::c_char; 7] =
        unsafe {
            [b"all\x00" as *const u8 as *const libc::c_char,
             b"collate\x00" as *const u8 as *const libc::c_char,
             b"ctype\x00" as *const u8 as *const libc::c_char,
             b"monetary\x00" as *const u8 as *const libc::c_char,
             b"numeric\x00" as *const u8 as *const libc::c_char,
             b"time\x00" as *const u8 as *const libc::c_char,
             0 as *const libc::c_char]
        };
    let mut l: *const libc::c_char =
        luaL_optlstring(L, 1i32, 0 as *const libc::c_char, 0 as *mut size_t);
    let mut op: libc::c_int =
        luaL_checkoption(L, 2i32,
                         b"all\x00" as *const u8 as *const libc::c_char,
                         catnames.as_ptr());
    lua_pushstring(L, setlocale(cat[op as usize], l));
    return 1i32;
}
unsafe extern "C" fn os_rename(mut L: *mut lua_State_0) -> libc::c_int {
    let mut fromname: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut toname: *const libc::c_char =
        luaL_checklstring(L, 2i32, 0 as *mut size_t);
    return luaL_fileresult(L,
                           (rename(fromname, toname) == 0i32) as libc::c_int,
                           0 as *const libc::c_char);
}
unsafe extern "C" fn os_remove(mut L: *mut lua_State_0) -> libc::c_int {
    let mut filename: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    return luaL_fileresult(L, (remove(filename) == 0i32) as libc::c_int,
                           filename);
}
unsafe extern "C" fn os_getenv(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushstring(L, getenv(luaL_checklstring(L, 1i32, 0 as *mut size_t)));
    return 1i32;
}
unsafe extern "C" fn os_exit(mut L: *mut lua_State_0) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if lua_type(L, 1i32) == 1i32 {
        status = if 0 != lua_toboolean(L, 1i32) { 0i32 } else { 1i32 }
    } else {
        status = luaL_optinteger(L, 1i32, 0i32 as lua_Integer) as libc::c_int
    }
    if 0 != lua_toboolean(L, 2i32) { lua_close(L); }
    if !L.is_null() { exit(status); } else { return 0i32 };
}
unsafe extern "C" fn os_execute(mut L: *mut lua_State_0) -> libc::c_int {
    let mut cmd: *const libc::c_char =
        luaL_optlstring(L, 1i32, 0 as *const libc::c_char, 0 as *mut size_t);
    let mut stat: libc::c_int = system(cmd);
    if !cmd.is_null() {
        return luaL_execresult(L, stat)
    } else { lua_pushboolean(L, stat); return 1i32 };
}
unsafe extern "C" fn os_difftime(mut L: *mut lua_State_0) -> libc::c_int {
    let mut t1: time_t = l_checktime(L, 1i32);
    let mut t2: time_t = l_checktime(L, 2i32);
    lua_pushnumber(L, difftime(t1, t2));
    return 1i32;
}
unsafe extern "C" fn l_checktime(mut L: *mut lua_State_0,
                                 mut arg: libc::c_int) -> time_t {
    let mut t: lua_Integer = luaL_checkinteger(L, arg);
    return t as time_t;
}
unsafe extern "C" fn os_date(mut L: *mut lua_State_0) -> libc::c_int {
    let mut slen: size_t = 0;
    let mut s: *const libc::c_char =
        luaL_optlstring(L, 1i32,
                        b"%c\x00" as *const u8 as *const libc::c_char,
                        &mut slen);
    let mut t: time_t =
        if lua_type(L, 2i32) <= 0i32 {
            time(0 as *mut time_t)
        } else { l_checktime(L, 2i32) };
    let mut se: *const libc::c_char = s.offset(slen as isize);
    let mut tmr: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           __tm_gmtoff: 0,
           __tm_zone: 0 as *const libc::c_char,};
    let mut stm: *mut tm = 0 as *mut tm;
    if *s as libc::c_int == '!' as i32 {
        stm = gmtime_r(&mut t, &mut tmr);
        s = s.offset(1isize)
    } else { stm = localtime_r(&mut t, &mut tmr) }
    if stm.is_null() {
        luaL_error(L,
                   b"time result cannot be represented in this installation\x00"
                       as *const u8 as *const libc::c_char);
    }
    if strcmp(s, b"*t\x00" as *const u8 as *const libc::c_char) == 0i32 {
        lua_createtable(L, 0i32, 9i32);
        setallfields(L, stm);
    } else {
        let mut cc: [libc::c_char; 4] = [0; 4];
        let mut b: luaL_Buffer_0 =
            luaL_Buffer{b: 0 as *mut libc::c_char,
                        size: 0,
                        n: 0,
                        L: 0 as *mut lua_State_0,
                        initb: [0; 23],};
        cc[0usize] = '%' as i32 as libc::c_char;
        luaL_buffinit(L, &mut b);
        while s < se {
            if *s as libc::c_int != '%' as i32 {
                let fresh1 = b.n;
                b.n = b.n.wrapping_add(1);
                let fresh0 = s;
                s = s.offset(1);
                *b.b.offset(fresh1 as isize) = *fresh0
            } else {
                let mut reslen: size_t = 0;
                let mut buff: *mut libc::c_char =
                    luaL_prepbuffsize(&mut b, 250i32 as size_t);
                s = s.offset(1isize);
                s =
                    checkoption(L, s,
                                s.offset_to(se).expect("bad offset_to") as
                                    libc::c_long,
                                cc.as_mut_ptr().offset(1isize));
                reslen =
                    strftime(buff, 250i32 as size_t, cc.as_mut_ptr(), stm);
                b.n =
                    (b.n as libc::c_ulong).wrapping_add(reslen) as size_t as
                        size_t
            }
        }
        luaL_pushresult(&mut b);
    }
    return 1i32;
}
unsafe extern "C" fn checkoption(mut L: *mut lua_State_0,
                                 mut conv: *const libc::c_char,
                                 mut convlen: ptrdiff_t,
                                 mut buff: *mut libc::c_char)
 -> *const libc::c_char {
    let mut option: *const libc::c_char =
        b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\x00"
            as *const u8 as *const libc::c_char;
    let mut oplen: libc::c_int = 1i32;
    while *option as libc::c_int != '\u{0}' as i32 &&
              oplen as libc::c_long <= convlen {
        if *option as libc::c_int == '|' as i32 {
            oplen += 1
        } else if memcmp(conv as *const libc::c_void,
                         option as *const libc::c_void,
                         oplen as libc::c_ulong) == 0i32 {
            memcpy(buff as *mut libc::c_void, conv as *const libc::c_void,
                   oplen as libc::c_ulong);
            *buff.offset(oplen as isize) = '\u{0}' as i32 as libc::c_char;
            return conv.offset(oplen as isize)
        }
        option = option.offset(oplen as isize)
    }
    luaL_argerror(L, 1i32,
                  lua_pushfstring(L,
                                  b"invalid conversion specifier \'%%%s\'\x00"
                                      as *const u8 as *const libc::c_char,
                                  conv));
    return conv;
}
unsafe extern "C" fn os_clock(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(L,
                   clock() as lua_Number /
                       1000000i32 as __clock_t as lua_Number);
    return 1i32;
}
