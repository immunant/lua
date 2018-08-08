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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn localeconv() -> *mut lconv;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE_0) -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn tmpfile() -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn setvbuf(__stream: *mut FILE, __buf: *mut libc::c_char,
               __modes: libc::c_int, __n: size_t) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t,
             __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftello(__stream: *mut FILE) -> __off64_t;
    #[no_mangle]
    fn clearerr(__stream: *mut FILE) -> ();
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn flockfile(__stream: *mut FILE) -> ();
    #[no_mangle]
    fn funlockfile(__stream: *mut FILE) -> ();
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int)
     -> ();
    #[no_mangle]
    fn lua_copy(L: *mut lua_State_0, fromidx: libc::c_int, toidx: libc::c_int)
     -> ();
    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumberx(L: *mut lua_State_0, idx: libc::c_int,
                     isnum: *mut libc::c_int) -> lua_Number;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State_0, idx: libc::c_int,
                      isnum: *mut libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State_0, idx: libc::c_int, len: *mut size_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_rawlen(L: *mut lua_State_0, idx: libc::c_int) -> size_t;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State_0, idx: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State_0, fn_0: lua_CFunction,
                        n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int,
                       nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State_0, sz: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_stringtonumber(L: *mut lua_State_0, s: *const libc::c_char)
     -> size_t;
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
    fn luaL_checkstack(L: *mut lua_State_0, sz: libc::c_int,
                       msg: *const libc::c_char) -> ();
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State_0, arg: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_newmetatable(L: *mut lua_State_0, tname: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn luaL_setmetatable(L: *mut lua_State_0, tname: *const libc::c_char)
     -> ();
    #[no_mangle]
    fn luaL_testudata(L: *mut lua_State_0, ud: libc::c_int,
                      tname: *const libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn luaL_checkudata(L: *mut lua_State_0, ud: libc::c_int,
                       tname: *const libc::c_char) -> *mut libc::c_void;
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
    fn lua_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type unnamed = libc::c_uint;
pub const _ISalnum: unnamed = 8;
pub const _ISpunct: unnamed = 4;
pub const _IScntrl: unnamed = 2;
pub const _ISblank: unnamed = 1;
pub const _ISgraph: unnamed = 32768;
pub const _ISprint: unnamed = 16384;
pub const _ISspace: unnamed = 8192;
pub const _ISxdigit: unnamed = 4096;
pub const _ISdigit: unnamed = 2048;
pub const _ISalpha: unnamed = 1024;
pub const _ISlower: unnamed = 512;
pub const _ISupper: unnamed = 256;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub type size_t = libc::c_ulong;
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
pub type FILE = _IO_FILE;
pub type _IO_FILE_0 = _IO_FILE;
pub type off_t = __off64_t;
pub type luaL_Stream = luaL_Stream_0;
pub type LStream = luaL_Stream;
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
    pub u: unnamed_0,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    l: unnamed_2,
    c: unnamed_1,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_1 {
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
pub struct unnamed_2 {
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
    pub u: unnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_3 {
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
    nk: unnamed_4,
    tvk: TValue,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_4 {
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
pub struct RN {
    pub f: *mut FILE,
    pub c: libc::c_int,
    pub n: libc::c_int,
    pub buff: [libc::c_char; 201],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct luaL_Stream_0 {
    pub f: *mut FILE,
    pub closef: lua_CFunction,
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
pub unsafe extern "C" fn luaopen_io(mut L: *mut lua_State_0) -> libc::c_int {
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
    luaL_setfuncs(L, iolib.as_ptr(), 0i32);
    createmeta(L);
    createstdfile(L, stdin,
                  b"_IO_input\x00" as *const u8 as *const libc::c_char,
                  b"stdin\x00" as *const u8 as *const libc::c_char);
    createstdfile(L, stdout,
                  b"_IO_output\x00" as *const u8 as *const libc::c_char,
                  b"stdout\x00" as *const u8 as *const libc::c_char);
    createstdfile(L, stderr, 0 as *const libc::c_char,
                  b"stderr\x00" as *const u8 as *const libc::c_char);
    return 1i32;
}
unsafe extern "C" fn createstdfile(mut L: *mut lua_State_0, mut f: *mut FILE,
                                   mut k: *const libc::c_char,
                                   mut fname: *const libc::c_char) -> () {
    let mut p: *mut LStream = newprefile(L);
    (*p).f = f;
    (*p).closef = Some(io_noclose);
    if !k.is_null() {
        lua_pushvalue(L, -1i32);
        lua_setfield(L, -50000i32 - 1000i32, k);
    }
    lua_setfield(L, -2i32, fname);
}
unsafe extern "C" fn io_noclose(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32,
                        b"FILE*\x00" as *const u8 as *const libc::c_char) as
            *mut LStream;
    (*p).closef = Some(io_noclose);
    lua_pushnil(L);
    lua_pushstring(L,
                   b"cannot close standard file\x00" as *const u8 as
                       *const libc::c_char);
    return 2i32;
}
unsafe extern "C" fn newprefile(mut L: *mut lua_State_0) -> *mut LStream {
    let mut p: *mut LStream =
        lua_newuserdata(L, ::std::mem::size_of::<LStream>() as libc::c_ulong)
            as *mut LStream;
    (*p).closef = None;
    luaL_setmetatable(L, b"FILE*\x00" as *const u8 as *const libc::c_char);
    return p;
}
unsafe extern "C" fn createmeta(mut L: *mut lua_State_0) -> () {
    luaL_newmetatable(L, b"FILE*\x00" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, -1i32);
    lua_setfield(L, -2i32,
                 b"__index\x00" as *const u8 as *const libc::c_char);
    luaL_setfuncs(L, flib.as_ptr(), 0i32);
    lua_settop(L, -1i32 - 1i32);
}
static mut flib: [luaL_Reg_0; 10] =
    unsafe {
        [luaL_Reg{name: b"close\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_close),},
         luaL_Reg{name: b"flush\x00" as *const u8 as *const libc::c_char,
                  func: Some(f_flush),},
         luaL_Reg{name: b"lines\x00" as *const u8 as *const libc::c_char,
                  func: Some(f_lines),},
         luaL_Reg{name: b"read\x00" as *const u8 as *const libc::c_char,
                  func: Some(f_read),},
         luaL_Reg{name: b"seek\x00" as *const u8 as *const libc::c_char,
                  func: Some(f_seek),},
         luaL_Reg{name: b"setvbuf\x00" as *const u8 as *const libc::c_char,
                  func: Some(f_setvbuf),},
         luaL_Reg{name: b"write\x00" as *const u8 as *const libc::c_char,
                  func: Some(f_write),},
         luaL_Reg{name: b"__gc\x00" as *const u8 as *const libc::c_char,
                  func: Some(f_gc),},
         luaL_Reg{name: b"__tostring\x00" as *const u8 as *const libc::c_char,
                  func: Some(f_tostring),},
         luaL_Reg{name: 0 as *const libc::c_char, func: None,}]
    };
unsafe extern "C" fn f_tostring(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32,
                        b"FILE*\x00" as *const u8 as *const libc::c_char) as
            *mut LStream;
    if (*p).closef.is_none() {
        lua_pushstring(L,
                       b"file (closed)\x00" as *const u8 as
                           *const libc::c_char);
    } else {
        lua_pushfstring(L,
                        b"file (%p)\x00" as *const u8 as *const libc::c_char,
                        (*p).f);
    }
    return 1i32;
}
unsafe extern "C" fn f_gc(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32,
                        b"FILE*\x00" as *const u8 as *const libc::c_char) as
            *mut LStream;
    if (*p).closef.is_some() && !(*p).f.is_null() { aux_close(L); }
    return 0i32;
}
unsafe extern "C" fn aux_close(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32,
                        b"FILE*\x00" as *const u8 as *const libc::c_char) as
            *mut LStream;
    let mut cf: lua_CFunction = (*p).closef;
    (*p).closef = None;
    return ::std::ptr::read_volatile::<lua_CFunction>(&cf as
                                                          *const lua_CFunction).expect("non-null function pointer")(L);
}
unsafe extern "C" fn f_write(mut L: *mut lua_State_0) -> libc::c_int {
    let mut f: *mut FILE = tofile(L);
    lua_pushvalue(L, 1i32);
    return g_write(L, f, 2i32);
}
unsafe extern "C" fn tofile(mut L: *mut lua_State_0) -> *mut FILE {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32,
                        b"FILE*\x00" as *const u8 as *const libc::c_char) as
            *mut LStream;
    if (*p).closef.is_none() {
        luaL_error(L,
                   b"attempt to use a closed file\x00" as *const u8 as
                       *const libc::c_char);
    }
    if !(*p).f.is_null() {
    } else {
        __assert_fail(b"p->f\x00" as *const u8 as *const libc::c_char,
                      b"liolib.c\x00" as *const u8 as *const libc::c_char,
                      179i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"FILE *tofile(lua_State *)\x00")).as_ptr());
    };
    return (*p).f;
}
unsafe extern "C" fn g_write(mut L: *mut lua_State_0, mut f: *mut FILE,
                             mut arg: libc::c_int) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut nargs: libc::c_int = lua_gettop(L) - arg;
    let mut status: libc::c_int = 1i32;
    loop  {
        let fresh0 = nargs;
        nargs = nargs - 1;
        if !(0 != fresh0) { break ; }
        if lua_type(L, arg) == 3i32 {
            len =
                if 0 != lua_isinteger(L, arg) {
                    fprintf(f,
                            b"%lld\x00" as *const u8 as *const libc::c_char,
                            lua_tointegerx(L, arg, 0 as *mut libc::c_int))
                } else {
                    fprintf(f,
                            b"%.14g\x00" as *const u8 as *const libc::c_char,
                            lua_tonumberx(L, arg, 0 as *mut libc::c_int))
                };
            status = (0 != status && len > 0i32) as libc::c_int
        } else {
            let mut l: size_t = 0;
            let mut s: *const libc::c_char =
                luaL_checklstring(L, arg, &mut l);
            status =
                (0 != status &&
                     fwrite(s as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong, l, f) == l) as libc::c_int
        }
        arg += 1
    }
    if 0 != status {
        return 1i32
    } else { return luaL_fileresult(L, status, 0 as *const libc::c_char) };
}
unsafe extern "C" fn f_setvbuf(mut L: *mut lua_State_0) -> libc::c_int {
    static mut mode: [libc::c_int; 3] = unsafe { [2i32, 0i32, 1i32] };
    static mut modenames: [*const libc::c_char; 4] =
        unsafe {
            [b"no\x00" as *const u8 as *const libc::c_char,
             b"full\x00" as *const u8 as *const libc::c_char,
             b"line\x00" as *const u8 as *const libc::c_char,
             0 as *const libc::c_char]
        };
    let mut f: *mut FILE = tofile(L);
    let mut op: libc::c_int =
        luaL_checkoption(L, 2i32, 0 as *const libc::c_char,
                         modenames.as_ptr());
    let mut sz: lua_Integer = luaL_optinteger(L, 3i32, 23i32 as lua_Integer);
    let mut res: libc::c_int =
        setvbuf(f, 0 as *mut libc::c_char, mode[op as usize], sz as size_t);
    return luaL_fileresult(L, (res == 0i32) as libc::c_int,
                           0 as *const libc::c_char);
}
unsafe extern "C" fn f_seek(mut L: *mut lua_State_0) -> libc::c_int {
    static mut mode: [libc::c_int; 3] = unsafe { [0i32, 1i32, 2i32] };
    static mut modenames: [*const libc::c_char; 4] =
        unsafe {
            [b"set\x00" as *const u8 as *const libc::c_char,
             b"cur\x00" as *const u8 as *const libc::c_char,
             b"end\x00" as *const u8 as *const libc::c_char,
             0 as *const libc::c_char]
        };
    let mut f: *mut FILE = tofile(L);
    let mut op: libc::c_int =
        luaL_checkoption(L, 2i32,
                         b"cur\x00" as *const u8 as *const libc::c_char,
                         modenames.as_ptr());
    let mut p3: lua_Integer = luaL_optinteger(L, 3i32, 0i32 as lua_Integer);
    let mut offset: off_t = p3 as off_t;
    op = fseeko(f, offset, mode[op as usize]);
    if 0 != op {
        return luaL_fileresult(L, 0i32, 0 as *const libc::c_char)
    } else { lua_pushinteger(L, ftello(f) as lua_Integer); return 1i32 };
}
unsafe extern "C" fn f_read(mut L: *mut lua_State_0) -> libc::c_int {
    return g_read(L, tofile(L), 2i32);
}
unsafe extern "C" fn g_read(mut L: *mut lua_State_0, mut f: *mut FILE,
                            mut first: libc::c_int) -> libc::c_int {
    let mut l: size_t = 0;
    let mut nargs: libc::c_int = lua_gettop(L) - 1i32;
    let mut success: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    clearerr(f);
    if nargs == 0i32 {
        success = read_line(L, f, 1i32);
        n = first + 1i32
    } else {
        luaL_checkstack(L, nargs + 20i32,
                        b"too many arguments\x00" as *const u8 as
                            *const libc::c_char);
        success = 1i32;
        n = first;
        loop  {
            let fresh1 = nargs;
            nargs = nargs - 1;
            if !(0 != fresh1 && 0 != success) { break ; }
            if lua_type(L, n) == 3i32 {
                l = luaL_checkinteger(L, n) as size_t;
                success =
                    if l == 0i32 as libc::c_ulong {
                        test_eof(L, f)
                    } else { read_chars(L, f, l) }
            } else {
                let mut p: *const libc::c_char =
                    luaL_checklstring(L, n, 0 as *mut size_t);
                if *p as libc::c_int == '*' as i32 { p = p.offset(1isize) }
                match *p as libc::c_int {
                    110 => { success = read_number(L, f) }
                    108 => { success = read_line(L, f, 1i32) }
                    76 => { success = read_line(L, f, 0i32) }
                    97 => { read_all(L, f); success = 1i32 }
                    _ => {
                        return luaL_argerror(L, n,
                                             b"invalid format\x00" as
                                                 *const u8 as
                                                 *const libc::c_char)
                    }
                }
            }
            n += 1
        }
    }
    if 0 != ferror(f) {
        return luaL_fileresult(L, 0i32, 0 as *const libc::c_char)
    } else {
        if 0 == success { lua_settop(L, -1i32 - 1i32); lua_pushnil(L); }
        return n - first
    };
}
unsafe extern "C" fn read_all(mut L: *mut lua_State_0, mut f: *mut FILE)
 -> () {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nr: size_t = 0;
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    luaL_buffinit(L, &mut b);
    loop  {
        p = luaL_prepbuffsize(&mut b, 23i32 as size_t);
        nr =
            fread(p as *mut libc::c_void,
                  ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                  23i32 as size_t, f);
        b.n = (b.n as libc::c_ulong).wrapping_add(nr) as size_t as size_t;
        if !(nr == 23i32 as libc::c_ulong) { break ; }
    }
    luaL_pushresult(&mut b);
}
unsafe extern "C" fn read_line(mut L: *mut lua_State_0, mut f: *mut FILE,
                               mut chop: libc::c_int) -> libc::c_int {
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    let mut c: libc::c_int = '\u{0}' as i32;
    luaL_buffinit(L, &mut b);
    while c != -1i32 && c != '\n' as i32 {
        let mut buff: *mut libc::c_char =
            luaL_prepbuffsize(&mut b, 23i32 as size_t);
        let mut i: libc::c_int = 0i32;
        flockfile(f);
        while i < 23i32 && { c = getc_unlocked(f); c != -1i32 } &&
                  c != '\n' as i32 {
            let fresh2 = i;
            i = i + 1;
            *buff.offset(fresh2 as isize) = c as libc::c_char
        }
        funlockfile(f);
        b.n =
            (b.n as libc::c_ulong).wrapping_add(i as libc::c_ulong) as size_t
                as size_t
    }
    if 0 == chop && c == '\n' as i32 {
        let fresh3 = b.n;
        b.n = b.n.wrapping_add(1);
        *b.b.offset(fresh3 as isize) = c as libc::c_char
    }
    luaL_pushresult(&mut b);
    return (c == '\n' as i32 || lua_rawlen(L, -1i32) > 0i32 as libc::c_ulong)
               as libc::c_int;
}
unsafe extern "C" fn read_number(mut L: *mut lua_State_0, mut f: *mut FILE)
 -> libc::c_int {
    let mut rn: RN = RN{f: 0 as *mut FILE, c: 0, n: 0, buff: [0; 201],};
    let mut count: libc::c_int = 0i32;
    let mut hex: libc::c_int = 0i32;
    let mut decp: [libc::c_char; 2] = [0; 2];
    rn.f = f;
    rn.n = 0i32;
    decp[0usize] = *(*localeconv()).decimal_point.offset(0isize);
    decp[1usize] = '.' as i32 as libc::c_char;
    flockfile(rn.f);
    loop  {
        rn.c = getc_unlocked(rn.f);
        if !(0 !=
                 *(*__ctype_b_loc()).offset(rn.c as isize) as libc::c_int &
                     _ISspace as libc::c_int as libc::c_ushort as libc::c_int)
           {
            break ;
        }
    }
    test2(&mut rn, b"-+\x00" as *const u8 as *const libc::c_char);
    if 0 != test2(&mut rn, b"00\x00" as *const u8 as *const libc::c_char) {
        if 0 != test2(&mut rn, b"xX\x00" as *const u8 as *const libc::c_char)
           {
            hex = 1i32
        } else { count = 1i32 }
    }
    count += readdigits(&mut rn, hex);
    if 0 != test2(&mut rn, decp.as_mut_ptr()) {
        count += readdigits(&mut rn, hex)
    }
    if count > 0i32 &&
           0 !=
               test2(&mut rn,
                     if 0 != hex {
                         b"pP\x00" as *const u8 as *const libc::c_char
                     } else { b"eE\x00" as *const u8 as *const libc::c_char })
       {
        test2(&mut rn, b"-+\x00" as *const u8 as *const libc::c_char);
        readdigits(&mut rn, 0i32);
    }
    ungetc(rn.c, rn.f);
    funlockfile(rn.f);
    rn.buff[rn.n as usize] = '\u{0}' as i32 as libc::c_char;
    if 0 != lua_stringtonumber(L, rn.buff.as_mut_ptr()) {
        return 1i32
    } else { lua_pushnil(L); return 0i32 };
}
unsafe extern "C" fn readdigits(mut rn: *mut RN, mut hex: libc::c_int)
 -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    while 0 !=
              if 0 != hex {
                  *(*__ctype_b_loc()).offset((*rn).c as isize) as libc::c_int
                      &
                      _ISxdigit as libc::c_int as libc::c_ushort as
                          libc::c_int
              } else {
                  *(*__ctype_b_loc()).offset((*rn).c as isize) as libc::c_int
                      &
                      _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
              } && 0 != nextc(rn) {
        count += 1
    }
    return count;
}
unsafe extern "C" fn nextc(mut rn: *mut RN) -> libc::c_int {
    if (*rn).n >= 200i32 {
        (*rn).buff[0usize] = '\u{0}' as i32 as libc::c_char;
        return 0i32
    } else {
        let fresh4 = (*rn).n;
        (*rn).n = (*rn).n + 1;
        (*rn).buff[fresh4 as usize] = (*rn).c as libc::c_char;
        (*rn).c = getc_unlocked((*rn).f);
        return 1i32
    };
}
unsafe extern "C" fn test2(mut rn: *mut RN, mut set: *const libc::c_char)
 -> libc::c_int {
    if (*rn).c == *set.offset(0isize) as libc::c_int ||
           (*rn).c == *set.offset(1isize) as libc::c_int {
        return nextc(rn)
    } else { return 0i32 };
}
unsafe extern "C" fn read_chars(mut L: *mut lua_State_0, mut f: *mut FILE,
                                mut n: size_t) -> libc::c_int {
    let mut nr: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    luaL_buffinit(L, &mut b);
    p = luaL_prepbuffsize(&mut b, n);
    nr =
        fread(p as *mut libc::c_void,
              ::std::mem::size_of::<libc::c_char>() as libc::c_ulong, n, f);
    b.n = (b.n as libc::c_ulong).wrapping_add(nr) as size_t as size_t;
    luaL_pushresult(&mut b);
    return (nr > 0i32 as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn test_eof(mut L: *mut lua_State_0, mut f: *mut FILE)
 -> libc::c_int {
    let mut c: libc::c_int = _IO_getc(f);
    ungetc(c, f);
    lua_pushstring(L, b"\x00" as *const u8 as *const libc::c_char);
    return (c != -1i32) as libc::c_int;
}
unsafe extern "C" fn f_lines(mut L: *mut lua_State_0) -> libc::c_int {
    tofile(L);
    aux_lines(L, 0i32);
    return 1i32;
}
unsafe extern "C" fn aux_lines(mut L: *mut lua_State_0,
                               mut toclose: libc::c_int) -> () {
    let mut n: libc::c_int = lua_gettop(L) - 1i32;
    lua_pushinteger(L, n as lua_Integer);
    lua_pushboolean(L, toclose);
    lua_rotate(L, 2i32, 2i32);
    lua_pushcclosure(L, Some(io_readline), 3i32 + n);
}
unsafe extern "C" fn io_readline(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream =
        lua_touserdata(L, -50000i32 - 1000i32 - 1i32) as *mut LStream;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int =
        lua_tointegerx(L, -50000i32 - 1000i32 - 2i32, 0 as *mut libc::c_int)
            as libc::c_int;
    if (*p).closef.is_none() {
        return luaL_error(L,
                          b"file is already closed\x00" as *const u8 as
                              *const libc::c_char)
    } else {
        lua_settop(L, 1i32);
        luaL_checkstack(L, n,
                        b"too many arguments\x00" as *const u8 as
                            *const libc::c_char);
        i = 1i32;
        while i <= n {
            lua_pushvalue(L, -50000i32 - 1000i32 - (3i32 + i));
            i += 1
        }
        n = g_read(L, (*p).f, 2i32);
        if n > 0i32 {
        } else {
            __assert_fail(b"n > 0\x00" as *const u8 as *const libc::c_char,
                          b"liolib.c\x00" as *const u8 as *const libc::c_char,
                          596i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 29],
                                                    &[libc::c_char; 29]>(b"int io_readline(lua_State *)\x00")).as_ptr());
        };
        if 0 != lua_toboolean(L, -n) {
            return n
        } else if n > 1i32 {
            return luaL_error(L,
                              b"%s\x00" as *const u8 as *const libc::c_char,
                              lua_tolstring(L, -n + 1i32, 0 as *mut size_t))
        } else {
            if 0 != lua_toboolean(L, -50000i32 - 1000i32 - 3i32) {
                lua_settop(L, 0i32);
                lua_pushvalue(L, -50000i32 - 1000i32 - 1i32);
                aux_close(L);
            }
            return 0i32
        }
    };
}
unsafe extern "C" fn f_flush(mut L: *mut lua_State_0) -> libc::c_int {
    return luaL_fileresult(L, (fflush(tofile(L)) == 0i32) as libc::c_int,
                           0 as *const libc::c_char);
}
unsafe extern "C" fn io_close(mut L: *mut lua_State_0) -> libc::c_int {
    if lua_type(L, 1i32) == -1i32 {
        lua_getfield(L, -50000i32 - 1000i32,
                     b"_IO_output\x00" as *const u8 as *const libc::c_char);
    }
    tofile(L);
    return aux_close(L);
}
static mut iolib: [luaL_Reg_0; 12] =
    unsafe {
        [luaL_Reg{name: b"close\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_close),},
         luaL_Reg{name: b"flush\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_flush),},
         luaL_Reg{name: b"input\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_input),},
         luaL_Reg{name: b"lines\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_lines),},
         luaL_Reg{name: b"open\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_open),},
         luaL_Reg{name: b"output\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_output),},
         luaL_Reg{name: b"popen\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_popen),},
         luaL_Reg{name: b"read\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_read),},
         luaL_Reg{name: b"tmpfile\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_tmpfile),},
         luaL_Reg{name: b"type\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_type),},
         luaL_Reg{name: b"write\x00" as *const u8 as *const libc::c_char,
                  func: Some(io_write),},
         luaL_Reg{name: 0 as *const libc::c_char, func: None,}]
    };
unsafe extern "C" fn io_write(mut L: *mut lua_State_0) -> libc::c_int {
    return g_write(L,
                   getiofile(L,
                             b"_IO_output\x00" as *const u8 as
                                 *const libc::c_char), 1i32);
}
unsafe extern "C" fn getiofile(mut L: *mut lua_State_0,
                               mut findex: *const libc::c_char) -> *mut FILE {
    let mut p: *mut LStream = 0 as *mut LStream;
    lua_getfield(L, -50000i32 - 1000i32, findex);
    p = lua_touserdata(L, -1i32) as *mut LStream;
    if (*p).closef.is_none() {
        luaL_error(L,
                   b"standard %s file is closed\x00" as *const u8 as
                       *const libc::c_char,
                   findex.offset((::std::mem::size_of::<[libc::c_char; 5]>()
                                      as
                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                                      as
                                                                      libc::c_ulong).wrapping_sub(1i32
                                                                                                      as
                                                                                                      libc::c_ulong)
                                     as isize));
    }
    return (*p).f;
}
unsafe extern "C" fn io_type(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream = 0 as *mut LStream;
    luaL_checkany(L, 1i32);
    p =
        luaL_testudata(L, 1i32,
                       b"FILE*\x00" as *const u8 as *const libc::c_char) as
            *mut LStream;
    if p.is_null() {
        lua_pushnil(L);
    } else if (*p).closef.is_none() {
        lua_pushstring(L,
                       b"closed file\x00" as *const u8 as
                           *const libc::c_char);
    } else {
        lua_pushstring(L, b"file\x00" as *const u8 as *const libc::c_char);
    }
    return 1i32;
}
unsafe extern "C" fn io_tmpfile(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream = newfile(L);
    (*p).f = tmpfile();
    return if (*p).f.is_null() {
               luaL_fileresult(L, 0i32, 0 as *const libc::c_char)
           } else { 1i32 };
}
unsafe extern "C" fn newfile(mut L: *mut lua_State_0) -> *mut LStream {
    let mut p: *mut LStream = newprefile(L);
    (*p).f = 0 as *mut FILE;
    (*p).closef = Some(io_fclose);
    return p;
}
unsafe extern "C" fn io_fclose(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32,
                        b"FILE*\x00" as *const u8 as *const libc::c_char) as
            *mut LStream;
    let mut res: libc::c_int = fclose((*p).f);
    return luaL_fileresult(L, (res == 0i32) as libc::c_int,
                           0 as *const libc::c_char);
}
unsafe extern "C" fn io_read(mut L: *mut lua_State_0) -> libc::c_int {
    return g_read(L,
                  getiofile(L,
                            b"_IO_input\x00" as *const u8 as
                                *const libc::c_char), 1i32);
}
unsafe extern "C" fn io_popen(mut L: *mut lua_State_0) -> libc::c_int {
    let mut filename: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut mode: *const libc::c_char =
        luaL_optlstring(L, 2i32, b"r\x00" as *const u8 as *const libc::c_char,
                        0 as *mut size_t);
    let mut p: *mut LStream = newprefile(L);
    fflush(0 as *mut FILE);
    (*p).f = popen(filename, mode);
    (*p).closef = Some(io_pclose);
    return if (*p).f.is_null() {
               luaL_fileresult(L, 0i32, filename)
           } else { 1i32 };
}
unsafe extern "C" fn io_pclose(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32,
                        b"FILE*\x00" as *const u8 as *const libc::c_char) as
            *mut LStream;
    return luaL_execresult(L, pclose((*p).f));
}
unsafe extern "C" fn io_output(mut L: *mut lua_State_0) -> libc::c_int {
    return g_iofile(L, b"_IO_output\x00" as *const u8 as *const libc::c_char,
                    b"w\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn g_iofile(mut L: *mut lua_State_0,
                              mut f: *const libc::c_char,
                              mut mode: *const libc::c_char) -> libc::c_int {
    if !(lua_type(L, 1i32) <= 0i32) {
        let mut filename: *const libc::c_char =
            lua_tolstring(L, 1i32, 0 as *mut size_t);
        if !filename.is_null() {
            opencheck(L, filename, mode);
        } else { tofile(L); lua_pushvalue(L, 1i32); }
        lua_setfield(L, -50000i32 - 1000i32, f);
    }
    lua_getfield(L, -50000i32 - 1000i32, f);
    return 1i32;
}
unsafe extern "C" fn opencheck(mut L: *mut lua_State_0,
                               mut fname: *const libc::c_char,
                               mut mode: *const libc::c_char) -> () {
    let mut p: *mut LStream = newfile(L);
    (*p).f = fopen(fname, mode);
    if (*p).f.is_null() {
        luaL_error(L,
                   b"cannot open file \'%s\' (%s)\x00" as *const u8 as
                       *const libc::c_char, fname,
                   strerror(*__errno_location()));
    };
}
unsafe extern "C" fn io_open(mut L: *mut lua_State_0) -> libc::c_int {
    let mut filename: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut mode: *const libc::c_char =
        luaL_optlstring(L, 2i32, b"r\x00" as *const u8 as *const libc::c_char,
                        0 as *mut size_t);
    let mut p: *mut LStream = newfile(L);
    let mut md: *const libc::c_char = mode;
    (*p).f = fopen(filename, mode);
    return if (*p).f.is_null() {
               luaL_fileresult(L, 0i32, filename)
           } else { 1i32 };
}
unsafe extern "C" fn l_checkmode(mut mode: *const libc::c_char)
 -> libc::c_int {
    return (*mode as libc::c_int != '\u{0}' as i32 &&
                {
                    let fresh5 = mode;
                    mode = mode.offset(1);
                    !strchr(b"rwa\x00" as *const u8 as *const libc::c_char,
                            *fresh5 as libc::c_int).is_null()
                } &&
                (*mode as libc::c_int != '+' as i32 ||
                     { mode = mode.offset(1isize); 0 != 1i32 }) &&
                strspn(mode, b"b\x00" as *const u8 as *const libc::c_char) ==
                    strlen(mode)) as libc::c_int;
}
unsafe extern "C" fn io_lines(mut L: *mut lua_State_0) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut toclose: libc::c_int = 0;
    if lua_type(L, 1i32) == -1i32 { lua_pushnil(L); }
    if lua_type(L, 1i32) == 0i32 {
        lua_getfield(L, -50000i32 - 1000i32,
                     b"_IO_input\x00" as *const u8 as *const libc::c_char);
        lua_copy(L, -1i32, 1i32);
        lua_settop(L, -1i32 - 1i32);
        tofile(L);
        toclose = 0i32
    } else {
        filename = luaL_checklstring(L, 1i32, 0 as *mut size_t);
        opencheck(L, filename, b"r\x00" as *const u8 as *const libc::c_char);
        lua_copy(L, -1i32, 1i32);
        lua_settop(L, -1i32 - 1i32);
        toclose = 1i32
    }
    aux_lines(L, toclose);
    return 1i32;
}
unsafe extern "C" fn io_input(mut L: *mut lua_State_0) -> libc::c_int {
    return g_iofile(L, b"_IO_input\x00" as *const u8 as *const libc::c_char,
                    b"r\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn io_flush(mut L: *mut lua_State_0) -> libc::c_int {
    return luaL_fileresult(L,
                           (fflush(getiofile(L,
                                             b"_IO_output\x00" as *const u8 as
                                                 *const libc::c_char)) ==
                                0i32) as libc::c_int,
                           0 as *const libc::c_char);
}
