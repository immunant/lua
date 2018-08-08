#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, offset_to)]
extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    pub type lua_longjmp;
    pub type UpVal_0;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn localeconv() -> *mut lconv;
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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int)
     -> ();
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State_0, tp: libc::c_int)
     -> *const libc::c_char;
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
    fn lua_touserdata(L: *mut lua_State_0, idx: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State_0, n: lua_Number) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushlstring(L: *mut lua_State_0, s: *const libc::c_char,
                       len: size_t) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State_0, fn_0: lua_CFunction,
                        n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_gettable(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int,
                       nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State_0, sz: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State_0, objindex: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lua_callk(L: *mut lua_State_0, nargs: libc::c_int,
                 nresults: libc::c_int, ctx: lua_KContext, k: lua_KFunction)
     -> ();
    #[no_mangle]
    fn lua_dump(L: *mut lua_State_0, writer_0: lua_Writer,
                data: *mut libc::c_void, strip: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t)
     -> ();
    #[no_mangle]
    fn luaL_tolstring(L: *mut lua_State_0, idx: libc::c_int, len: *mut size_t)
     -> *const libc::c_char;
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
    fn luaL_checknumber(L: *mut lua_State_0, arg: libc::c_int) -> lua_Number;
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
    fn luaL_checktype(L: *mut lua_State_0, arg: libc::c_int, t: libc::c_int)
     -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0,
                     nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State_0, B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_prepbuffsize(B: *mut luaL_Buffer_0, sz: size_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn luaL_addlstring(B: *mut luaL_Buffer_0, s: *const libc::c_char,
                       l: size_t) -> ();
    #[no_mangle]
    fn luaL_addstring(B: *mut luaL_Buffer_0, s: *const libc::c_char) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_pushresultsize(B: *mut luaL_Buffer_0, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_buffinitsize(L: *mut lua_State_0, B: *mut luaL_Buffer_0,
                         sz: size_t) -> *mut libc::c_char;
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
pub type ptrdiff_t = libc::c_long;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct GMatchState {
    pub src: *const libc::c_char,
    pub p: *const libc::c_char,
    pub lastmatch: *const libc::c_char,
    pub ms: MatchState,
}
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
pub type lua_Unsigned = libc::c_ulonglong;
pub type lua_Writer =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *const libc::c_void,
                                _: size_t, _: *mut libc::c_void)
               -> libc::c_int>;
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
pub type GMatchState_0 = GMatchState;
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
pub const Knop: KOption_0 = 8;
pub const Kpadding: KOption_0 = 6;
pub const Kpaddalign: KOption_0 = 7;
pub const Kzstr: KOption_0 = 5;
pub type Header = Header_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Header_0 {
    pub L: *mut lua_State_0,
    pub islittle: libc::c_int,
    pub maxalign: libc::c_int,
}
pub const Kstring: KOption_0 = 4;
pub const Kchar: KOption_0 = 3;
pub type Ftypes = Ftypes_0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Ftypes_0 {
    f: libc::c_float,
    d: libc::c_double,
    n: lua_Number,
    buff: [libc::c_char; 40],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_5 {
    dummy: libc::c_int,
    little: libc::c_char,
}
pub const Kfloat: KOption_0 = 2;
pub const Kint: KOption_0 = 0;
pub type KOption = KOption_0;
pub type KOption_0 = libc::c_uint;
pub const Kuint: KOption_0 = 1;
pub type MatchState = MatchState_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct MatchState_0 {
    pub src_init: *const libc::c_char,
    pub src_end: *const libc::c_char,
    pub p_end: *const libc::c_char,
    pub L: *mut lua_State_0,
    pub matchdepth: libc::c_int,
    pub level: libc::c_uchar,
    pub capture: [unnamed_6; 32],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_6 {
    pub init: *const libc::c_char,
    pub len: ptrdiff_t,
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_string(mut L: *mut lua_State_0)
 -> libc::c_int {
    luaL_checkversion_(L, 503i32 as lua_Number,
                       (::std::mem::size_of::<lua_Integer>() as
                            libc::c_ulong).wrapping_mul(16i32 as
                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<lua_Number>()
                                                                                            as
                                                                                            libc::c_ulong));
    lua_createtable(L, 0i32,
                    (::std::mem::size_of::<[luaL_Reg_0; 18]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_Reg_0>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int);
    luaL_setfuncs(L, strlib.as_ptr(), 0i32);
    createmetatable(L);
    return 1i32;
}
unsafe extern "C" fn createmetatable(mut L: *mut lua_State_0) -> () {
    lua_createtable(L, 0i32, 1i32);
    lua_pushstring(L, b"\x00" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, -2i32);
    lua_setmetatable(L, -2i32);
    lua_settop(L, -1i32 - 1i32);
    lua_pushvalue(L, -2i32);
    lua_setfield(L, -2i32,
                 b"__index\x00" as *const u8 as *const libc::c_char);
    lua_settop(L, -1i32 - 1i32);
}
static mut strlib: [luaL_Reg_0; 18] =
    unsafe {
        [luaL_Reg{name: b"byte\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_byte),},
         luaL_Reg{name: b"char\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_char),},
         luaL_Reg{name: b"dump\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_dump),},
         luaL_Reg{name: b"find\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_find),},
         luaL_Reg{name: b"format\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_format),},
         luaL_Reg{name: b"gmatch\x00" as *const u8 as *const libc::c_char,
                  func: Some(gmatch),},
         luaL_Reg{name: b"gsub\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_gsub),},
         luaL_Reg{name: b"len\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_len),},
         luaL_Reg{name: b"lower\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_lower),},
         luaL_Reg{name: b"match\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_match),},
         luaL_Reg{name: b"rep\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_rep),},
         luaL_Reg{name: b"reverse\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_reverse),},
         luaL_Reg{name: b"sub\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_sub),},
         luaL_Reg{name: b"upper\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_upper),},
         luaL_Reg{name: b"pack\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_pack),},
         luaL_Reg{name: b"packsize\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_packsize),},
         luaL_Reg{name: b"unpack\x00" as *const u8 as *const libc::c_char,
                  func: Some(str_unpack),},
         luaL_Reg{name: 0 as *const libc::c_char, func: None,}]
    };
unsafe extern "C" fn str_unpack(mut L: *mut lua_State_0) -> libc::c_int {
    let mut h: Header =
        Header_0{L: 0 as *mut lua_State_0, islittle: 0, maxalign: 0,};
    let mut fmt: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut ld: size_t = 0;
    let mut data: *const libc::c_char = luaL_checklstring(L, 2i32, &mut ld);
    let mut pos: size_t =
        (posrelat(luaL_optinteger(L, 3i32, 1i32 as lua_Integer), ld) as
             size_t).wrapping_sub(1i32 as libc::c_ulong);
    let mut n: libc::c_int = 0i32;
    initheader(L, &mut h);
    while *fmt as libc::c_int != '\u{0}' as i32 {
        let mut size: libc::c_int = 0;
        let mut ntoalign: libc::c_int = 0;
        let mut opt: KOption =
            getdetails(&mut h, pos, &mut fmt, &mut size, &mut ntoalign);
        if (ntoalign as size_t).wrapping_add(size as libc::c_ulong) > !pos ||
               pos.wrapping_add(ntoalign as
                                    libc::c_ulong).wrapping_add(size as
                                                                    libc::c_ulong)
                   > ld {
            luaL_argerror(L, 2i32,
                          b"data string too short\x00" as *const u8 as
                              *const libc::c_char);
        }
        pos =
            (pos as libc::c_ulong).wrapping_add(ntoalign as libc::c_ulong) as
                size_t as size_t;
        luaL_checkstack(L, 2i32,
                        b"too many results\x00" as *const u8 as
                            *const libc::c_char);
        n += 1;
        match opt as libc::c_uint {
            0 | 1 => {
                let mut res: lua_Integer =
                    unpackint(L, data.offset(pos as isize), h.islittle, size,
                              (opt as libc::c_uint ==
                                   Kint as libc::c_int as libc::c_uint) as
                                  libc::c_int);
                lua_pushinteger(L, res);
            }
            2 => {
                let mut u: Ftypes = Ftypes_0{f: 0.,};
                let mut num: lua_Number = 0.;
                copywithendian(::std::ptr::read_volatile::<Ftypes>(&u as
                                                                       *const Ftypes).buff.as_mut_ptr(),
                               data.offset(pos as isize), size, h.islittle);
                if size as libc::c_ulong ==
                       ::std::mem::size_of::<libc::c_float>() as libc::c_ulong
                   {
                    num =
                        ::std::ptr::read_volatile::<Ftypes>(&u as
                                                                *const Ftypes).f
                            as lua_Number
                } else if size as libc::c_ulong ==
                              ::std::mem::size_of::<libc::c_double>() as
                                  libc::c_ulong {
                    num =
                        ::std::ptr::read_volatile::<Ftypes>(&u as
                                                                *const Ftypes).d
                } else {
                    num =
                        ::std::ptr::read_volatile::<Ftypes>(&u as
                                                                *const Ftypes).n
                }
                lua_pushnumber(L, num);
            }
            3 => {
                lua_pushlstring(L, data.offset(pos as isize), size as size_t);
            }
            4 => {
                let mut len: size_t =
                    unpackint(L, data.offset(pos as isize), h.islittle, size,
                              0i32) as size_t;
                lua_pushlstring(L,
                                data.offset(pos as
                                                isize).offset(size as isize),
                                len);
                pos =
                    (pos as libc::c_ulong).wrapping_add(len) as size_t as
                        size_t
            }
            5 => {
                let mut len_0: size_t =
                    strlen(data.offset(pos as isize)) as libc::c_int as
                        size_t;
                lua_pushlstring(L, data.offset(pos as isize), len_0);
                pos =
                    (pos as
                         libc::c_ulong).wrapping_add(len_0.wrapping_add(1i32
                                                                            as
                                                                            libc::c_ulong))
                        as size_t as size_t
            }
            7 | 6 | 8 => { n -= 1 }
            _ => { }
        }
        pos =
            (pos as libc::c_ulong).wrapping_add(size as libc::c_ulong) as
                size_t as size_t
    }
    lua_pushinteger(L,
                    pos.wrapping_add(1i32 as libc::c_ulong) as lua_Integer);
    return n + 1i32;
}
unsafe extern "C" fn posrelat(mut pos: lua_Integer, mut len: size_t)
 -> lua_Integer {
    if pos >= 0i32 as libc::c_longlong {
        return pos
    } else if (0u32 as libc::c_ulong).wrapping_sub(pos as size_t) > len {
        return 0i32 as lua_Integer
    } else { return len as lua_Integer + pos + 1i32 as libc::c_longlong };
}
unsafe extern "C" fn unpackint(mut L: *mut lua_State_0,
                               mut str: *const libc::c_char,
                               mut islittle: libc::c_int,
                               mut size: libc::c_int,
                               mut issigned: libc::c_int) -> lua_Integer {
    let mut res: lua_Unsigned = 0i32 as lua_Unsigned;
    let mut i: libc::c_int = 0;
    let mut limit: libc::c_int =
        if size <=
               ::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
                   libc::c_int {
            size
        } else {
            ::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
                libc::c_int
        };
    i = limit - 1i32;
    while i >= 0i32 {
        res <<= 8i32;
        res |=
            *str.offset((if 0 != islittle { i } else { size - 1i32 - i }) as
                            isize) as libc::c_uchar as lua_Unsigned;
        i -= 1
    }
    if size <
           ::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
               libc::c_int {
        if 0 != issigned {
            let mut mask: lua_Unsigned =
                (1i32 as lua_Unsigned) << size * 8i32 - 1i32;
            res = (res ^ mask).wrapping_sub(mask)
        }
    } else if size >
                  ::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
                      libc::c_int {
        let mut mask_0: libc::c_int =
            if 0 == issigned || res as lua_Integer >= 0i32 as libc::c_longlong
               {
                0i32
            } else { (1i32 << 8i32) - 1i32 };
        i = limit;
        while i < size {
            if *str.offset((if 0 != islittle { i } else { size - 1i32 - i })
                               as isize) as libc::c_uchar as libc::c_int !=
                   mask_0 {
                luaL_error(L,
                           b"%d-byte integer does not fit into Lua Integer\x00"
                               as *const u8 as *const libc::c_char, size);
            }
            i += 1
        }
    }
    return res as lua_Integer;
}
unsafe extern "C" fn copywithendian(mut dest: *mut libc::c_char,
                                    mut src: *const libc::c_char,
                                    mut size: libc::c_int,
                                    mut islittle: libc::c_int) -> () {
    if islittle == nativeendian.little as libc::c_int {
        loop  {
            let fresh0 = size;
            size = size - 1;
            if !(fresh0 != 0i32) { break ; }
            let fresh2 = dest;
            dest = dest.offset(1);
            let fresh1 = src;
            src = src.offset(1);
            ::std::ptr::write_volatile(fresh2,
                                       ::std::ptr::read_volatile::<libc::c_char>(fresh1))
        }
    } else {
        dest = dest.offset((size - 1i32) as isize);
        loop  {
            let fresh3 = size;
            size = size - 1;
            if !(fresh3 != 0i32) { break ; }
            let fresh5 = dest;
            dest = dest.offset(-1);
            let fresh4 = src;
            src = src.offset(1);
            ::std::ptr::write_volatile(fresh5,
                                       ::std::ptr::read_volatile::<libc::c_char>(fresh4))
        }
    };
}
static mut nativeendian: unnamed_5 = unsafe { unnamed_5{dummy: 1i32,} };
unsafe extern "C" fn getdetails(mut h: *mut Header, mut totalsize: size_t,
                                mut fmt: *mut *const libc::c_char,
                                mut psize: *mut libc::c_int,
                                mut ntoalign: *mut libc::c_int) -> KOption {
    let mut opt: KOption = getoption(h, fmt, psize);
    let mut align: libc::c_int = *psize;
    if opt as libc::c_uint == Kpaddalign as libc::c_int as libc::c_uint {
        if **fmt as libc::c_int == '\u{0}' as i32 ||
               getoption(h, fmt, &mut align) as libc::c_uint ==
                   Kchar as libc::c_int as libc::c_uint || align == 0i32 {
            luaL_argerror((*h).L, 1i32,
                          b"invalid next option for option \'X\'\x00" as
                              *const u8 as *const libc::c_char);
        }
    }
    if align <= 1i32 ||
           opt as libc::c_uint == Kchar as libc::c_int as libc::c_uint {
        *ntoalign = 0i32
    } else {
        if align > (*h).maxalign { align = (*h).maxalign }
        if align & align - 1i32 != 0i32 {
            luaL_argerror((*h).L, 1i32,
                          b"format asks for alignment not power of 2\x00" as
                              *const u8 as *const libc::c_char);
        }
        *ntoalign =
            align -
                (totalsize & (align - 1i32) as libc::c_ulong) as libc::c_int &
                align - 1i32
    }
    return opt;
}
unsafe extern "C" fn getoption(mut h: *mut Header,
                               mut fmt: *mut *const libc::c_char,
                               mut size: *mut libc::c_int) -> KOption {
    let fresh6 = *fmt;
    *fmt = (*fmt).offset(1);
    let mut opt: libc::c_int = *fresh6 as libc::c_int;
    *size = 0i32;
    match opt {
        98 => {
            *size =
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as
                    libc::c_int;
            return Kint
        }
        66 => {
            *size =
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as
                    libc::c_int;
            return Kuint
        }
        104 => {
            *size =
                ::std::mem::size_of::<libc::c_short>() as libc::c_ulong as
                    libc::c_int;
            return Kint
        }
        72 => {
            *size =
                ::std::mem::size_of::<libc::c_short>() as libc::c_ulong as
                    libc::c_int;
            return Kuint
        }
        108 => {
            *size =
                ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as
                    libc::c_int;
            return Kint
        }
        76 => {
            *size =
                ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as
                    libc::c_int;
            return Kuint
        }
        106 => {
            *size =
                ::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
                    libc::c_int;
            return Kint
        }
        74 => {
            *size =
                ::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
                    libc::c_int;
            return Kuint
        }
        84 => {
            *size =
                ::std::mem::size_of::<size_t>() as libc::c_ulong as
                    libc::c_int;
            return Kuint
        }
        102 => {
            *size =
                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong as
                    libc::c_int;
            return Kfloat
        }
        100 => {
            *size =
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as
                    libc::c_int;
            return Kfloat
        }
        110 => {
            *size =
                ::std::mem::size_of::<lua_Number>() as libc::c_ulong as
                    libc::c_int;
            return Kfloat
        }
        105 => {
            *size =
                getnumlimit(h, fmt,
                            ::std::mem::size_of::<libc::c_int>() as
                                libc::c_ulong as libc::c_int);
            return Kint
        }
        73 => {
            *size =
                getnumlimit(h, fmt,
                            ::std::mem::size_of::<libc::c_int>() as
                                libc::c_ulong as libc::c_int);
            return Kuint
        }
        115 => {
            *size =
                getnumlimit(h, fmt,
                            ::std::mem::size_of::<size_t>() as libc::c_ulong
                                as libc::c_int);
            return Kstring
        }
        99 => {
            *size = getnum(fmt, -1i32);
            if *size == -1i32 {
                luaL_error((*h).L,
                           b"missing size for format option \'c\'\x00" as
                               *const u8 as *const libc::c_char);
            }
            return Kchar
        }
        122 => { return Kzstr }
        120 => { *size = 1i32; return Kpadding }
        88 => { return Kpaddalign }
        32 => { }
        60 => { (*h).islittle = 1i32 }
        62 => { (*h).islittle = 0i32 }
        61 => { (*h).islittle = nativeendian.little as libc::c_int }
        33 => { (*h).maxalign = getnumlimit(h, fmt, 8u64 as libc::c_int) }
        _ => {
            luaL_error((*h).L,
                       b"invalid format option \'%c\'\x00" as *const u8 as
                           *const libc::c_char, opt);
        }
    }
    return Knop;
}
unsafe extern "C" fn getnumlimit(mut h: *mut Header,
                                 mut fmt: *mut *const libc::c_char,
                                 mut df: libc::c_int) -> libc::c_int {
    let mut sz: libc::c_int = getnum(fmt, df);
    if sz > 16i32 || sz <= 0i32 {
        luaL_error((*h).L,
                   b"integral size (%d) out of limits [1,%d]\x00" as *const u8
                       as *const libc::c_char, sz, 16i32);
    }
    return sz;
}
unsafe extern "C" fn getnum(mut fmt: *mut *const libc::c_char,
                            mut df: libc::c_int) -> libc::c_int {
    if 0 == digit(**fmt as libc::c_int) {
        return df
    } else {
        let mut a: libc::c_int = 0i32;
        loop  {
            let fresh7 = *fmt;
            *fmt = (*fmt).offset(1);
            a = a * 10i32 + (*fresh7 as libc::c_int - '0' as i32);
            if !(0 != digit(**fmt as libc::c_int) &&
                     a <=
                         ((if (::std::mem::size_of::<size_t>() as
                                   libc::c_ulong) <
                                  ::std::mem::size_of::<libc::c_int>() as
                                      libc::c_ulong {
                               !(0i32 as size_t)
                           } else { 2147483647i32 as size_t }) as libc::c_int
                              - 9i32) / 10i32) {
                break ;
            }
        }
        return a
    };
}
unsafe extern "C" fn digit(mut c: libc::c_int) -> libc::c_int {
    return ('0' as i32 <= c && c <= '9' as i32) as libc::c_int;
}
unsafe extern "C" fn initheader(mut L: *mut lua_State_0, mut h: *mut Header)
 -> () {
    (*h).L = L;
    (*h).islittle = nativeendian.little as libc::c_int;
    (*h).maxalign = 1i32;
}
unsafe extern "C" fn str_packsize(mut L: *mut lua_State_0) -> libc::c_int {
    let mut h: Header =
        Header_0{L: 0 as *mut lua_State_0, islittle: 0, maxalign: 0,};
    let mut fmt: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut totalsize: size_t = 0i32 as size_t;
    initheader(L, &mut h);
    while *fmt as libc::c_int != '\u{0}' as i32 {
        let mut size: libc::c_int = 0;
        let mut ntoalign: libc::c_int = 0;
        let mut opt: KOption =
            getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
        size += ntoalign;
        totalsize =
            (totalsize as libc::c_ulong).wrapping_add(size as libc::c_ulong)
                as size_t as size_t;
        match opt as libc::c_uint { 4 | 5 => { } _ => { continue ; } }
        luaL_argerror(L, 1i32,
                      b"variable-length format\x00" as *const u8 as
                          *const libc::c_char);
    }
    lua_pushinteger(L, totalsize as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn str_pack(mut L: *mut lua_State_0) -> libc::c_int {
    let mut lim: lua_Integer = 0;
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    let mut h: Header =
        Header_0{L: 0 as *mut lua_State_0, islittle: 0, maxalign: 0,};
    let mut fmt: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut arg: libc::c_int = 1i32;
    let mut totalsize: size_t = 0i32 as size_t;
    initheader(L, &mut h);
    lua_pushnil(L);
    luaL_buffinit(L, &mut b);
    's_11:
        while *fmt as libc::c_int != '\u{0}' as i32 {
            let mut size: libc::c_int = 0;
            let mut ntoalign: libc::c_int = 0;
            let mut opt: KOption =
                getdetails(&mut h, totalsize, &mut fmt, &mut size,
                           &mut ntoalign);
            totalsize =
                (totalsize as
                     libc::c_ulong).wrapping_add((ntoalign + size) as
                                                     libc::c_ulong) as size_t
                    as size_t;
            loop  {
                let fresh8 = ntoalign;
                ntoalign = ntoalign - 1;
                if !(fresh8 > 0i32) { break ; }
                let fresh9 = b.n;
                b.n = b.n.wrapping_add(1);
                *b.b.offset(fresh9 as isize) = 0i32 as libc::c_char
            }
            arg += 1;
            match opt as libc::c_uint {
                0 => {
                    let mut n: lua_Integer = luaL_checkinteger(L, arg);
                    if size <
                           ::std::mem::size_of::<lua_Integer>() as
                               libc::c_ulong as libc::c_int {
                        lim = (1i32 as lua_Integer) << size * 8i32 - 1i32;
                    }
                    packint(&mut b, n as lua_Unsigned, h.islittle, size,
                            (n < 0i32 as libc::c_longlong) as libc::c_int);
                    continue ;
                }
                1 => {
                    let mut n_0: lua_Integer = luaL_checkinteger(L, arg);
                    size <
                        ::std::mem::size_of::<lua_Integer>() as libc::c_ulong
                            as libc::c_int;
                    packint(&mut b, n_0 as lua_Unsigned, h.islittle, size,
                            0i32);
                    continue ;
                }
                2 => {
                    let mut u: Ftypes = Ftypes_0{f: 0.,};
                    let mut buff: *mut libc::c_char =
                        luaL_prepbuffsize(&mut b, size as size_t);
                    let mut n_1: lua_Number = luaL_checknumber(L, arg);
                    if size as libc::c_ulong ==
                           ::std::mem::size_of::<libc::c_float>() as
                               libc::c_ulong {
                        ::std::ptr::write_volatile(&mut u.f as
                                                       *mut libc::c_float,
                                                   n_1 as libc::c_float)
                    } else if size as libc::c_ulong ==
                                  ::std::mem::size_of::<libc::c_double>() as
                                      libc::c_ulong {
                        ::std::ptr::write_volatile(&mut u.d as
                                                       *mut libc::c_double,
                                                   n_1)
                    } else {
                        ::std::ptr::write_volatile(&mut u.n as
                                                       *mut lua_Number, n_1)
                    }
                    copywithendian(buff as *mut libc::c_char,
                                   ::std::ptr::read_volatile::<Ftypes>(&u as
                                                                           *const Ftypes).buff.as_mut_ptr(),
                                   size, h.islittle);
                    b.n =
                        (b.n as
                             libc::c_ulong).wrapping_add(size as
                                                             libc::c_ulong) as
                            size_t as size_t;
                    continue ;
                }
                3 => {
                    let mut len: size_t = 0;
                    let mut s: *const libc::c_char =
                        luaL_checklstring(L, arg, &mut len);
                    luaL_addlstring(&mut b, s, len);
                    loop  {
                        let fresh10 = len;
                        len = len.wrapping_add(1);
                        if !(fresh10 < size as size_t) { continue 's_11 ; }
                        let fresh11 = b.n;
                        b.n = b.n.wrapping_add(1);
                        *b.b.offset(fresh11 as isize) = 0i32 as libc::c_char
                    }
                }
                4 => {
                    let mut len_0: size_t = 0;
                    let mut s_0: *const libc::c_char =
                        luaL_checklstring(L, arg, &mut len_0);
                    packint(&mut b, len_0 as lua_Unsigned, h.islittle, size,
                            0i32);
                    luaL_addlstring(&mut b, s_0, len_0);
                    totalsize =
                        (totalsize as libc::c_ulong).wrapping_add(len_0) as
                            size_t as size_t;
                    continue ;
                }
                5 => {
                    let mut len_1: size_t = 0;
                    let mut s_1: *const libc::c_char =
                        luaL_checklstring(L, arg, &mut len_1);
                    luaL_addlstring(&mut b, s_1, len_1);
                    let fresh12 = b.n;
                    b.n = b.n.wrapping_add(1);
                    *b.b.offset(fresh12 as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    totalsize =
                        (totalsize as
                             libc::c_ulong).wrapping_add(len_1.wrapping_add(1i32
                                                                                as
                                                                                libc::c_ulong))
                            as size_t as size_t;
                    continue ;
                }
                6 => {
                    let fresh13 = b.n;
                    b.n = b.n.wrapping_add(1);
                    *b.b.offset(fresh13 as isize) = 0i32 as libc::c_char
                }
                7 | 8 => { }
                _ => { continue ; }
            }
            arg -= 1
        }
    luaL_pushresult(&mut b);
    return 1i32;
}
unsafe extern "C" fn packint(mut b: *mut luaL_Buffer_0, mut n: lua_Unsigned,
                             mut islittle: libc::c_int, mut size: libc::c_int,
                             mut neg: libc::c_int) -> () {
    let mut buff: *mut libc::c_char = luaL_prepbuffsize(b, size as size_t);
    let mut i: libc::c_int = 0;
    *buff.offset((if 0 != islittle { 0i32 } else { size - 1i32 }) as isize) =
        (n & ((1i32 << 8i32) - 1i32) as libc::c_ulonglong) as libc::c_char;
    i = 1i32;
    while i < size {
        n >>= 8i32;
        *buff.offset((if 0 != islittle { i } else { size - 1i32 - i }) as
                         isize) =
            (n & ((1i32 << 8i32) - 1i32) as libc::c_ulonglong) as
                libc::c_char;
        i += 1
    }
    if 0 != neg &&
           size >
               ::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
                   libc::c_int {
        i =
            ::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
                libc::c_int;
        while i < size {
            *buff.offset((if 0 != islittle { i } else { size - 1i32 - i }) as
                             isize) = ((1i32 << 8i32) - 1i32) as libc::c_char;
            i += 1
        }
    }
    (*b).n =
        ((*b).n as libc::c_ulong).wrapping_add(size as libc::c_ulong) as
            size_t as size_t;
}
unsafe extern "C" fn str_upper(mut L: *mut lua_State_0) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut l);
    let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, l);
    i = 0i32 as size_t;
    while i < l {
        *p.offset(i as isize) =
            toupper(*s.offset(i as isize) as libc::c_uchar as libc::c_int) as
                libc::c_char;
        i = i.wrapping_add(1)
    }
    luaL_pushresultsize(&mut b, l);
    return 1i32;
}
unsafe extern "C" fn str_sub(mut L: *mut lua_State_0) -> libc::c_int {
    let mut l: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut l);
    let mut start: lua_Integer = posrelat(luaL_checkinteger(L, 2i32), l);
    let mut end: lua_Integer =
        posrelat(luaL_optinteger(L, 3i32, -1i32 as lua_Integer), l);
    if start < 1i32 as libc::c_longlong { start = 1i32 as lua_Integer }
    if end > l as lua_Integer { end = l as lua_Integer }
    if start <= end {
        lua_pushlstring(L, s.offset(start as isize).offset(-1isize),
                        ((end - start) as
                             size_t).wrapping_add(1i32 as libc::c_ulong));
    } else { lua_pushstring(L, b"\x00" as *const u8 as *const libc::c_char); }
    return 1i32;
}
unsafe extern "C" fn str_reverse(mut L: *mut lua_State_0) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut l);
    let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, l);
    i = 0i32 as size_t;
    while i < l {
        *p.offset(i as isize) =
            *s.offset(l.wrapping_sub(i).wrapping_sub(1i32 as libc::c_ulong) as
                          isize);
        i = i.wrapping_add(1)
    }
    luaL_pushresultsize(&mut b, l);
    return 1i32;
}
unsafe extern "C" fn str_rep(mut L: *mut lua_State_0) -> libc::c_int {
    let mut l: size_t = 0;
    let mut lsep: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut l);
    let mut n: lua_Integer = luaL_checkinteger(L, 2i32);
    let mut sep: *const libc::c_char =
        luaL_optlstring(L, 3i32, b"\x00" as *const u8 as *const libc::c_char,
                        &mut lsep);
    if n <= 0i32 as libc::c_longlong {
        lua_pushstring(L, b"\x00" as *const u8 as *const libc::c_char);
    } else if l.wrapping_add(lsep) < l ||
                  l.wrapping_add(lsep) as libc::c_ulonglong >
                      ((if (::std::mem::size_of::<size_t>() as libc::c_ulong)
                               <
                               ::std::mem::size_of::<libc::c_int>() as
                                   libc::c_ulong {
                            !(0i32 as size_t)
                        } else { 2147483647i32 as size_t }) as
                           libc::c_ulonglong).wrapping_div(n as
                                                               libc::c_ulonglong)
     {
        return luaL_error(L,
                          b"resulting string too large\x00" as *const u8 as
                              *const libc::c_char)
    } else {
        let mut totallen: size_t =
            (n as
                 size_t).wrapping_mul(l).wrapping_add(((n -
                                                            1i32 as
                                                                libc::c_longlong)
                                                           as
                                                           size_t).wrapping_mul(lsep));
        let mut b: luaL_Buffer_0 =
            luaL_Buffer{b: 0 as *mut libc::c_char,
                        size: 0,
                        n: 0,
                        L: 0 as *mut lua_State_0,
                        initb: [0; 23],};
        let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, totallen);
        loop  {
            let fresh14 = n;
            n = n - 1;
            if !(fresh14 > 1i32 as libc::c_longlong) { break ; }
            memcpy(p as *mut libc::c_void, s as *const libc::c_void,
                   l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as
                                      libc::c_ulong));
            p = p.offset(l as isize);
            if !(lsep > 0i32 as libc::c_ulong) { continue ; }
            memcpy(p as *mut libc::c_void, sep as *const libc::c_void,
                   lsep.wrapping_mul(::std::mem::size_of::<libc::c_char>() as
                                         libc::c_ulong));
            p = p.offset(lsep as isize)
        }
        memcpy(p as *mut libc::c_void, s as *const libc::c_void,
               l.wrapping_mul(::std::mem::size_of::<libc::c_char>() as
                                  libc::c_ulong));
        luaL_pushresultsize(&mut b, totallen);
    }
    return 1i32;
}
unsafe extern "C" fn str_match(mut L: *mut lua_State_0) -> libc::c_int {
    return str_find_aux(L, 0i32);
}
unsafe extern "C" fn str_find_aux(mut L: *mut lua_State_0,
                                  mut find: libc::c_int) -> libc::c_int {
    let mut ls: size_t = 0;
    let mut lp: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut ls);
    let mut p: *const libc::c_char = luaL_checklstring(L, 2i32, &mut lp);
    let mut init: lua_Integer =
        posrelat(luaL_optinteger(L, 3i32, 1i32 as lua_Integer), ls);
    if init < 1i32 as libc::c_longlong {
        init = 1i32 as lua_Integer
    } else if init > ls as lua_Integer + 1i32 as libc::c_longlong {
        lua_pushnil(L);
        return 1i32
    }
    if 0 != find && (0 != lua_toboolean(L, 4i32) || 0 != nospecials(p, lp)) {
        let mut s2: *const libc::c_char =
            lmemfind(s.offset(init as isize).offset(-1isize),
                     ls.wrapping_sub(init as
                                         size_t).wrapping_add(1i32 as
                                                                  libc::c_ulong),
                     p, lp);
        if !s2.is_null() {
            lua_pushinteger(L,
                            (s.offset_to(s2).expect("bad offset_to") as
                                 libc::c_long + 1i32 as libc::c_long) as
                                lua_Integer);
            lua_pushinteger(L,
                            (s.offset_to(s2).expect("bad offset_to") as
                                 libc::c_long as
                                 libc::c_ulong).wrapping_add(lp) as
                                lua_Integer);
            return 2i32
        }
    } else {
        let mut ms: MatchState =
            MatchState_0{src_init: 0 as *const libc::c_char,
                         src_end: 0 as *const libc::c_char,
                         p_end: 0 as *const libc::c_char,
                         L: 0 as *mut lua_State_0,
                         matchdepth: 0,
                         level: 0,
                         capture:
                             [unnamed_6{init: 0 as *const libc::c_char,
                                        len: 0,}; 32],};
        let mut s1: *const libc::c_char =
            s.offset(init as isize).offset(-1isize);
        let mut anchor: libc::c_int =
            (*p as libc::c_int == '^' as i32) as libc::c_int;
        if 0 != anchor { p = p.offset(1isize); lp = lp.wrapping_sub(1) }
        prepstate(&mut ms, L, s, ls, p, lp);
        loop  {
            let mut res: *const libc::c_char = 0 as *const libc::c_char;
            reprepstate(&mut ms);
            res = match_0(&mut ms, s1, p);
            if !res.is_null() {
                if 0 != find {
                    lua_pushinteger(L,
                                    (s.offset_to(s1).expect("bad offset_to")
                                         as libc::c_long +
                                         1i32 as libc::c_long) as
                                        lua_Integer);
                    lua_pushinteger(L,
                                    s.offset_to(res).expect("bad offset_to")
                                        as libc::c_long as lua_Integer);
                    return push_captures(&mut ms, 0 as *const libc::c_char,
                                         0 as *const libc::c_char) + 2i32
                } else { return push_captures(&mut ms, s1, res) }
            } else {
                let fresh15 = s1;
                s1 = s1.offset(1);
                if !(fresh15 < ms.src_end && 0 == anchor) { break ; }
            }
        }
    }
    lua_pushnil(L);
    return 1i32;
}
unsafe extern "C" fn push_captures(mut ms: *mut MatchState,
                                   mut s: *const libc::c_char,
                                   mut e: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut nlevels: libc::c_int =
        if (*ms).level as libc::c_int == 0i32 && !s.is_null() {
            1i32
        } else { (*ms).level as libc::c_int };
    luaL_checkstack((*ms).L, nlevels,
                    b"too many captures\x00" as *const u8 as
                        *const libc::c_char);
    i = 0i32;
    while i < nlevels { push_onecapture(ms, i, s, e); i += 1 }
    return nlevels;
}
unsafe extern "C" fn push_onecapture(mut ms: *mut MatchState,
                                     mut i: libc::c_int,
                                     mut s: *const libc::c_char,
                                     mut e: *const libc::c_char) -> () {
    if i >= (*ms).level as libc::c_int {
        if i == 0i32 {
            lua_pushlstring((*ms).L, s,
                            s.offset_to(e).expect("bad offset_to") as
                                libc::c_long as size_t);
        } else {
            luaL_error((*ms).L,
                       b"invalid capture index %%%d\x00" as *const u8 as
                           *const libc::c_char, i + 1i32);
        }
    } else {
        let mut l: ptrdiff_t = (*ms).capture[i as usize].len;
        if l == -1i32 as libc::c_long {
            luaL_error((*ms).L,
                       b"unfinished capture\x00" as *const u8 as
                           *const libc::c_char);
        }
        if l == -2i32 as libc::c_long {
            lua_pushinteger((*ms).L,
                            ((*ms).src_init.offset_to((*ms).capture[i as
                                                                        usize].init).expect("bad offset_to")
                                 as libc::c_long + 1i32 as libc::c_long) as
                                lua_Integer);
        } else {
            lua_pushlstring((*ms).L, (*ms).capture[i as usize].init,
                            l as size_t);
        }
    };
}
unsafe extern "C" fn match_0(mut ms: *mut MatchState,
                             mut s: *const libc::c_char,
                             mut p: *const libc::c_char)
 -> *const libc::c_char {
    let mut ep_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let fresh16 = (*ms).matchdepth;
    (*ms).matchdepth = (*ms).matchdepth - 1;
    if fresh16 == 0i32 {
        luaL_error((*ms).L,
                   b"pattern too complex\x00" as *const u8 as
                       *const libc::c_char);
    }
    loop  {
        if !(p != (*ms).p_end) {
            current_block = 11174649648027449784;
            break ;
        }
        match *p as libc::c_int {
            40 => {
                if *p.offset(1isize) as libc::c_int == ')' as i32 {
                    current_block = 12675440807659640239;
                    break ;
                } else { current_block = 16658872821858055392; break ; }
            }
            41 => {
                s = end_capture(ms, s, p.offset(1isize));
                current_block = 11174649648027449784;
                break ;
            }
            36 => {
                if !(p.offset(1isize) != (*ms).p_end) {
                    s =
                        if s == (*ms).src_end {
                            s
                        } else { 0 as *const libc::c_char };
                    current_block = 11174649648027449784;
                    break ;
                }
            }
            37 => {
                match *p.offset(1isize) as libc::c_int {
                    98 => {
                        current_block = 7351195479953500246;
                        match current_block {
                            7351195479953500246 => {
                                s = matchbalance(ms, s, p.offset(2isize));
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                                p = p.offset(4isize);
                                continue ;
                            }
                            13183875560443969876 => {
                                let mut ep: *const libc::c_char =
                                    0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2isize);
                                if *p as libc::c_int != '[' as i32 {
                                    luaL_error((*ms).L,
                                               b"missing \'[\' after \'%%f\' in pattern\x00"
                                                   as *const u8 as
                                                   *const libc::c_char);
                                }
                                ep = classend(ms, p);
                                previous =
                                    (if s == (*ms).src_init {
                                         '\u{0}' as i32
                                     } else {
                                         *s.offset(-1isize) as libc::c_int
                                     }) as libc::c_char;
                                if 0 ==
                                       matchbracketclass(previous as
                                                             libc::c_uchar as
                                                             libc::c_int, p,
                                                         ep.offset(-1isize))
                                       &&
                                       0 !=
                                           matchbracketclass(*s as
                                                                 libc::c_uchar
                                                                 as
                                                                 libc::c_int,
                                                             p,
                                                             ep.offset(-1isize))
                                   {
                                    p = ep;
                                    continue ;
                                } else {
                                    s = 0 as *const libc::c_char;
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                            }
                            _ => {
                                s =
                                    match_capture(ms, s,
                                                  *p.offset(1isize) as
                                                      libc::c_uchar as
                                                      libc::c_int);
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                                p = p.offset(2isize);
                                continue ;
                            }
                        }
                    }
                    102 => {
                        current_block = 13183875560443969876;
                        match current_block {
                            7351195479953500246 => {
                                s = matchbalance(ms, s, p.offset(2isize));
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                                p = p.offset(4isize);
                                continue ;
                            }
                            13183875560443969876 => {
                                let mut ep: *const libc::c_char =
                                    0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2isize);
                                if *p as libc::c_int != '[' as i32 {
                                    luaL_error((*ms).L,
                                               b"missing \'[\' after \'%%f\' in pattern\x00"
                                                   as *const u8 as
                                                   *const libc::c_char);
                                }
                                ep = classend(ms, p);
                                previous =
                                    (if s == (*ms).src_init {
                                         '\u{0}' as i32
                                     } else {
                                         *s.offset(-1isize) as libc::c_int
                                     }) as libc::c_char;
                                if 0 ==
                                       matchbracketclass(previous as
                                                             libc::c_uchar as
                                                             libc::c_int, p,
                                                         ep.offset(-1isize))
                                       &&
                                       0 !=
                                           matchbracketclass(*s as
                                                                 libc::c_uchar
                                                                 as
                                                                 libc::c_int,
                                                             p,
                                                             ep.offset(-1isize))
                                   {
                                    p = ep;
                                    continue ;
                                } else {
                                    s = 0 as *const libc::c_char;
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                            }
                            _ => {
                                s =
                                    match_capture(ms, s,
                                                  *p.offset(1isize) as
                                                      libc::c_uchar as
                                                      libc::c_int);
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                                p = p.offset(2isize);
                                continue ;
                            }
                        }
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 11050875288958768710;
                        match current_block {
                            7351195479953500246 => {
                                s = matchbalance(ms, s, p.offset(2isize));
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                                p = p.offset(4isize);
                                continue ;
                            }
                            13183875560443969876 => {
                                let mut ep: *const libc::c_char =
                                    0 as *const libc::c_char;
                                let mut previous: libc::c_char = 0;
                                p = p.offset(2isize);
                                if *p as libc::c_int != '[' as i32 {
                                    luaL_error((*ms).L,
                                               b"missing \'[\' after \'%%f\' in pattern\x00"
                                                   as *const u8 as
                                                   *const libc::c_char);
                                }
                                ep = classend(ms, p);
                                previous =
                                    (if s == (*ms).src_init {
                                         '\u{0}' as i32
                                     } else {
                                         *s.offset(-1isize) as libc::c_int
                                     }) as libc::c_char;
                                if 0 ==
                                       matchbracketclass(previous as
                                                             libc::c_uchar as
                                                             libc::c_int, p,
                                                         ep.offset(-1isize))
                                       &&
                                       0 !=
                                           matchbracketclass(*s as
                                                                 libc::c_uchar
                                                                 as
                                                                 libc::c_int,
                                                             p,
                                                             ep.offset(-1isize))
                                   {
                                    p = ep;
                                    continue ;
                                } else {
                                    s = 0 as *const libc::c_char;
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                            }
                            _ => {
                                s =
                                    match_capture(ms, s,
                                                  *p.offset(1isize) as
                                                      libc::c_uchar as
                                                      libc::c_int);
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break ;
                                }
                                p = p.offset(2isize);
                                continue ;
                            }
                        }
                    }
                    _ => { }
                }
            }
            _ => { }
        }
        ep_0 = classend(ms, p);
        if 0 == singlematch(ms, s, p, ep_0) {
            if *ep_0 as libc::c_int == '*' as i32 ||
                   *ep_0 as libc::c_int == '?' as i32 ||
                   *ep_0 as libc::c_int == '-' as i32 {
                p = ep_0.offset(1isize)
            } else {
                s = 0 as *const libc::c_char;
                current_block = 11174649648027449784;
                break ;
            }
        } else {
            match *ep_0 as libc::c_int {
                63 => {
                    let mut res: *const libc::c_char =
                        0 as *const libc::c_char;
                    res = match_0(ms, s.offset(1isize), ep_0.offset(1isize));
                    if !res.is_null() {
                        s = res;
                        current_block = 11174649648027449784;
                        break ;
                    } else { p = ep_0.offset(1isize) }
                }
                43 => {
                    s = s.offset(1isize);
                    current_block = 5298694902333424248;
                    break ;
                }
                42 => { current_block = 5298694902333424248; break ; }
                45 => {
                    s = min_expand(ms, s, p, ep_0);
                    current_block = 11174649648027449784;
                    break ;
                }
                _ => { s = s.offset(1isize); p = ep_0 }
            }
        }
    }
    match current_block {
        16658872821858055392 => {
            s = start_capture(ms, s, p.offset(1isize), -1i32)
        }
        5298694902333424248 => { s = max_expand(ms, s, p, ep_0) }
        12675440807659640239 => {
            s = start_capture(ms, s, p.offset(2isize), -2i32)
        }
        _ => { }
    }
    (*ms).matchdepth += 1;
    return s;
}
unsafe extern "C" fn classend(mut ms: *mut MatchState,
                              mut p: *const libc::c_char)
 -> *const libc::c_char {
    let fresh17 = p;
    p = p.offset(1);
    match *fresh17 as libc::c_int {
        37 => {
            if p == (*ms).p_end {
                luaL_error((*ms).L,
                           b"malformed pattern (ends with \'%%\')\x00" as
                               *const u8 as *const libc::c_char);
            }
            return p.offset(1isize)
        }
        91 => {
            if *p as libc::c_int == '^' as i32 { p = p.offset(1isize) }
            loop  {
                if p == (*ms).p_end {
                    luaL_error((*ms).L,
                               b"malformed pattern (missing \']\')\x00" as
                                   *const u8 as *const libc::c_char);
                }
                let fresh18 = p;
                p = p.offset(1);
                if *fresh18 as libc::c_int == '%' as i32 && p < (*ms).p_end {
                    p = p.offset(1isize)
                }
                if !(*p as libc::c_int != ']' as i32) { break ; }
            }
            return p.offset(1isize)
        }
        _ => { return p }
    };
}
unsafe extern "C" fn min_expand(mut ms: *mut MatchState,
                                mut s: *const libc::c_char,
                                mut p: *const libc::c_char,
                                mut ep: *const libc::c_char)
 -> *const libc::c_char {
    loop  {
        let mut res: *const libc::c_char = match_0(ms, s, ep.offset(1isize));
        if !res.is_null() {
            return res
        } else if 0 != singlematch(ms, s, p, ep) {
            s = s.offset(1isize)
        } else { return 0 as *const libc::c_char }
    };
}
unsafe extern "C" fn singlematch(mut ms: *mut MatchState,
                                 mut s: *const libc::c_char,
                                 mut p: *const libc::c_char,
                                 mut ep: *const libc::c_char) -> libc::c_int {
    if s >= (*ms).src_end {
        return 0i32
    } else {
        let mut c: libc::c_int = *s as libc::c_uchar as libc::c_int;
        match *p as libc::c_int {
            46 => { return 1i32 }
            37 => {
                return match_class(c,
                                   *p.offset(1isize) as libc::c_uchar as
                                       libc::c_int)
            }
            91 => { return matchbracketclass(c, p, ep.offset(-1isize)) }
            _ => {
                return (*p as libc::c_uchar as libc::c_int == c) as
                           libc::c_int
            }
        }
    };
}
unsafe extern "C" fn matchbracketclass(mut c: libc::c_int,
                                       mut p: *const libc::c_char,
                                       mut ec: *const libc::c_char)
 -> libc::c_int {
    let mut sig: libc::c_int = 1i32;
    if *p.offset(1isize) as libc::c_int == '^' as i32 {
        sig = 0i32;
        p = p.offset(1isize)
    }
    loop  {
        p = p.offset(1isize);
        if !(p < ec) { break ; }
        if *p as libc::c_int == '%' as i32 {
            p = p.offset(1isize);
            if !(0 != match_class(c, *p as libc::c_uchar as libc::c_int)) {
                continue ;
            }
            return sig
        } else if *p.offset(1isize) as libc::c_int == '-' as i32 &&
                      p.offset(2isize) < ec {
            p = p.offset(2isize);
            if !(*p.offset(-2isize) as libc::c_uchar as libc::c_int <= c &&
                     c <= *p as libc::c_uchar as libc::c_int) {
                continue ;
            }
            return sig
        } else {
            if !(*p as libc::c_uchar as libc::c_int == c) { continue ; }
            return sig
        }
    }
    return (0 == sig) as libc::c_int;
}
unsafe extern "C" fn match_class(mut c: libc::c_int, mut cl: libc::c_int)
 -> libc::c_int {
    let mut res: libc::c_int = 0;
    match tolower(cl) {
        97 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
        }
        99 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
        }
        100 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        }
        103 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISgraph as libc::c_int as libc::c_ushort as libc::c_int
        }
        108 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISlower as libc::c_int as libc::c_ushort as libc::c_int
        }
        112 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISpunct as libc::c_int as libc::c_ushort as libc::c_int
        }
        115 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISspace as libc::c_int as libc::c_ushort as libc::c_int
        }
        117 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISupper as libc::c_int as libc::c_ushort as libc::c_int
        }
        119 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
        }
        120 => {
            res =
                *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                    _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
        }
        122 => { res = (c == 0i32) as libc::c_int }
        _ => { return (cl == c) as libc::c_int }
    }
    return if 0 !=
                  *(*__ctype_b_loc()).offset(cl as isize) as libc::c_int &
                      _ISlower as libc::c_int as libc::c_ushort as libc::c_int
              {
               res
           } else { (0 == res) as libc::c_int };
}
unsafe extern "C" fn max_expand(mut ms: *mut MatchState,
                                mut s: *const libc::c_char,
                                mut p: *const libc::c_char,
                                mut ep: *const libc::c_char)
 -> *const libc::c_char {
    let mut i: ptrdiff_t = 0i32 as ptrdiff_t;
    while 0 != singlematch(ms, s.offset(i as isize), p, ep) { i += 1 }
    while i >= 0i32 as libc::c_long {
        let mut res: *const libc::c_char =
            match_0(ms, s.offset(i as isize), ep.offset(1isize));
        if !res.is_null() { return res } else { i -= 1 }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn match_capture(mut ms: *mut MatchState,
                                   mut s: *const libc::c_char,
                                   mut l: libc::c_int)
 -> *const libc::c_char {
    let mut len: size_t = 0;
    l = check_capture(ms, l);
    len = (*ms).capture[l as usize].len as size_t;
    if s.offset_to((*ms).src_end).expect("bad offset_to") as libc::c_long as
           size_t >= len &&
           memcmp((*ms).capture[l as usize].init as *const libc::c_void,
                  s as *const libc::c_void, len) == 0i32 {
        return s.offset(len as isize)
    } else { return 0 as *const libc::c_char };
}
unsafe extern "C" fn check_capture(mut ms: *mut MatchState,
                                   mut l: libc::c_int) -> libc::c_int {
    l -= '1' as i32;
    if l < 0i32 || l >= (*ms).level as libc::c_int ||
           (*ms).capture[l as usize].len == -1i32 as libc::c_long {
        return luaL_error((*ms).L,
                          b"invalid capture index %%%d\x00" as *const u8 as
                              *const libc::c_char, l + 1i32)
    } else { return l };
}
unsafe extern "C" fn matchbalance(mut ms: *mut MatchState,
                                  mut s: *const libc::c_char,
                                  mut p: *const libc::c_char)
 -> *const libc::c_char {
    if p >= (*ms).p_end.offset(-1isize) {
        luaL_error((*ms).L,
                   b"malformed pattern (missing arguments to \'%%b\')\x00" as
                       *const u8 as *const libc::c_char);
    }
    if *s as libc::c_int != *p as libc::c_int {
        return 0 as *const libc::c_char
    } else {
        let mut b: libc::c_int = *p as libc::c_int;
        let mut e: libc::c_int = *p.offset(1isize) as libc::c_int;
        let mut cont: libc::c_int = 1i32;
        loop  {
            s = s.offset(1isize);
            if !(s < (*ms).src_end) { break ; }
            if *s as libc::c_int == e {
                cont -= 1;
                if !(cont == 0i32) { continue ; }
                return s.offset(1isize)
            } else { if !(*s as libc::c_int == b) { continue ; } cont += 1 }
        }
        return 0 as *const libc::c_char
    };
}
unsafe extern "C" fn end_capture(mut ms: *mut MatchState,
                                 mut s: *const libc::c_char,
                                 mut p: *const libc::c_char)
 -> *const libc::c_char {
    let mut l: libc::c_int = capture_to_close(ms);
    let mut res: *const libc::c_char = 0 as *const libc::c_char;
    (*ms).capture[l as usize].len =
        (*ms).capture[l as usize].init.offset_to(s).expect("bad offset_to") as
            libc::c_long;
    res = match_0(ms, s, p);
    if res.is_null() { (*ms).capture[l as usize].len = -1i32 as ptrdiff_t }
    return res;
}
unsafe extern "C" fn capture_to_close(mut ms: *mut MatchState)
 -> libc::c_int {
    let mut level: libc::c_int = (*ms).level as libc::c_int;
    level -= 1;
    while level >= 0i32 {
        if (*ms).capture[level as usize].len == -1i32 as libc::c_long {
            return level
        } else { level -= 1 }
    }
    return luaL_error((*ms).L,
                      b"invalid pattern capture\x00" as *const u8 as
                          *const libc::c_char);
}
unsafe extern "C" fn start_capture(mut ms: *mut MatchState,
                                   mut s: *const libc::c_char,
                                   mut p: *const libc::c_char,
                                   mut what: libc::c_int)
 -> *const libc::c_char {
    let mut res: *const libc::c_char = 0 as *const libc::c_char;
    let mut level: libc::c_int = (*ms).level as libc::c_int;
    if level >= 32i32 {
        luaL_error((*ms).L,
                   b"too many captures\x00" as *const u8 as
                       *const libc::c_char);
    }
    (*ms).capture[level as usize].init = s;
    (*ms).capture[level as usize].len = what as ptrdiff_t;
    (*ms).level = (level + 1i32) as libc::c_uchar;
    res = match_0(ms, s, p);
    if res.is_null() { (*ms).level = (*ms).level.wrapping_sub(1) }
    return res;
}
unsafe extern "C" fn reprepstate(mut ms: *mut MatchState) -> () {
    (*ms).level = 0i32 as libc::c_uchar;
    if (*ms).matchdepth == 200i32 {
    } else {
        __assert_fail(b"ms->matchdepth == 200\x00" as *const u8 as
                          *const libc::c_char,
                      b"lstrlib.c\x00" as *const u8 as *const libc::c_char,
                      604i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void reprepstate(MatchState *)\x00")).as_ptr());
    };
}
unsafe extern "C" fn prepstate(mut ms: *mut MatchState,
                               mut L: *mut lua_State_0,
                               mut s: *const libc::c_char, mut ls: size_t,
                               mut p: *const libc::c_char, mut lp: size_t)
 -> () {
    (*ms).L = L;
    (*ms).matchdepth = 200i32;
    (*ms).src_init = s;
    (*ms).src_end = s.offset(ls as isize);
    (*ms).p_end = p.offset(lp as isize);
}
unsafe extern "C" fn lmemfind(mut s1: *const libc::c_char, mut l1: size_t,
                              mut s2: *const libc::c_char, mut l2: size_t)
 -> *const libc::c_char {
    if l2 == 0i32 as libc::c_ulong {
        return s1
    } else if l2 > l1 {
        return 0 as *const libc::c_char
    } else {
        let mut init: *const libc::c_char = 0 as *const libc::c_char;
        l2 = l2.wrapping_sub(1);
        l1 = l1.wrapping_sub(l2);
        while l1 > 0i32 as libc::c_ulong &&
                  {
                      init =
                          memchr(s1 as *const libc::c_void,
                                 *s2 as libc::c_int, l1) as
                              *const libc::c_char;
                      !init.is_null()
                  } {
            init = init.offset(1isize);
            if memcmp(init as *const libc::c_void,
                      s2.offset(1isize) as *const libc::c_void, l2) == 0i32 {
                return init.offset(-1isize)
            } else {
                l1 =
                    (l1 as
                         libc::c_ulong).wrapping_sub(s1.offset_to(init).expect("bad offset_to")
                                                         as libc::c_long as
                                                         libc::c_ulong) as
                        size_t as size_t;
                s1 = init
            }
        }
        return 0 as *const libc::c_char
    };
}
unsafe extern "C" fn nospecials(mut p: *const libc::c_char, mut l: size_t)
 -> libc::c_int {
    let mut upto: size_t = 0i32 as size_t;
    loop  {
        if !strpbrk(p.offset(upto as isize),
                    b"^$*+?.([%-\x00" as *const u8 as
                        *const libc::c_char).is_null() {
            return 0i32
        } else {
            upto =
                (upto as
                     libc::c_ulong).wrapping_add(strlen(p.offset(upto as
                                                                     isize)).wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_ulong))
                    as size_t as size_t;
            if !(upto <= l) { break ; }
        }
    }
    return 1i32;
}
unsafe extern "C" fn str_lower(mut L: *mut lua_State_0) -> libc::c_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut l);
    let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, l);
    i = 0i32 as size_t;
    while i < l {
        *p.offset(i as isize) =
            tolower(*s.offset(i as isize) as libc::c_uchar as libc::c_int) as
                libc::c_char;
        i = i.wrapping_add(1)
    }
    luaL_pushresultsize(&mut b, l);
    return 1i32;
}
unsafe extern "C" fn str_len(mut L: *mut lua_State_0) -> libc::c_int {
    let mut l: size_t = 0;
    luaL_checklstring(L, 1i32, &mut l);
    lua_pushinteger(L, l as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn str_gsub(mut L: *mut lua_State_0) -> libc::c_int {
    let mut srcl: size_t = 0;
    let mut lp: size_t = 0;
    let mut src: *const libc::c_char = luaL_checklstring(L, 1i32, &mut srcl);
    let mut p: *const libc::c_char = luaL_checklstring(L, 2i32, &mut lp);
    let mut lastmatch: *const libc::c_char = 0 as *const libc::c_char;
    let mut tr: libc::c_int = lua_type(L, 3i32);
    let mut max_s: lua_Integer =
        luaL_optinteger(L, 4i32,
                        srcl.wrapping_add(1i32 as libc::c_ulong) as
                            lua_Integer);
    let mut anchor: libc::c_int =
        (*p as libc::c_int == '^' as i32) as libc::c_int;
    let mut n: lua_Integer = 0i32 as lua_Integer;
    let mut ms: MatchState =
        MatchState_0{src_init: 0 as *const libc::c_char,
                     src_end: 0 as *const libc::c_char,
                     p_end: 0 as *const libc::c_char,
                     L: 0 as *mut lua_State_0,
                     matchdepth: 0,
                     level: 0,
                     capture:
                         [unnamed_6{init: 0 as *const libc::c_char, len: 0,};
                             32],};
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    luaL_buffinit(L, &mut b);
    if 0 != anchor { p = p.offset(1isize); lp = lp.wrapping_sub(1) }
    prepstate(&mut ms, L, src, srcl, p, lp);
    while n < max_s {
        let mut e: *const libc::c_char = 0 as *const libc::c_char;
        reprepstate(&mut ms);
        e = match_0(&mut ms, src, p);
        if !e.is_null() && e != lastmatch {
            n += 1;
            add_value(&mut ms, &mut b, src, e, tr);
            lastmatch = e;
            src = lastmatch
        } else {
            if !(src < ms.src_end) { break ; }
            let fresh20 = b.n;
            b.n = b.n.wrapping_add(1);
            let fresh19 = src;
            src = src.offset(1);
            *b.b.offset(fresh20 as isize) = *fresh19
        }
        if 0 != anchor { break ; }
    }
    luaL_addlstring(&mut b, src,
                    src.offset_to(ms.src_end).expect("bad offset_to") as
                        libc::c_long as size_t);
    luaL_pushresult(&mut b);
    lua_pushinteger(L, n);
    return 2i32;
}
unsafe extern "C" fn add_value(mut ms: *mut MatchState,
                               mut b: *mut luaL_Buffer_0,
                               mut s: *const libc::c_char,
                               mut e: *const libc::c_char,
                               mut tr: libc::c_int) -> () {
    let mut L: *mut lua_State_0 = (*ms).L;
    match tr {
        6 => {
            let mut n: libc::c_int = 0;
            lua_pushvalue(L, 3i32);
            n = push_captures(ms, s, e);
            lua_callk(L, n, 1i32, 0i32 as lua_KContext, None);
        }
        5 => { push_onecapture(ms, 0i32, s, e); lua_gettable(L, 3i32); }
        _ => { add_s(ms, b, s, e); return }
    }
    if 0 == lua_toboolean(L, -1i32) {
        lua_settop(L, -1i32 - 1i32);
        lua_pushlstring(L, s,
                        s.offset_to(e).expect("bad offset_to") as libc::c_long
                            as size_t);
    } else if 0 == lua_isstring(L, -1i32) {
        luaL_error(L,
                   b"invalid replacement value (a %s)\x00" as *const u8 as
                       *const libc::c_char,
                   lua_typename(L, lua_type(L, -1i32)));
    }
    luaL_addvalue(b);
}
unsafe extern "C" fn add_s(mut ms: *mut MatchState, mut b: *mut luaL_Buffer_0,
                           mut s: *const libc::c_char,
                           mut e: *const libc::c_char) -> () {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut L: *mut lua_State_0 = (*ms).L;
    let mut news: *const libc::c_char = lua_tolstring(L, 3i32, &mut l);
    i = 0i32 as size_t;
    while i < l {
        if *news.offset(i as isize) as libc::c_int != '%' as i32 {
            let fresh21 = (*b).n;
            (*b).n = (*b).n.wrapping_add(1);
            *(*b).b.offset(fresh21 as isize) = *news.offset(i as isize)
        } else {
            i = i.wrapping_add(1);
            if 0 ==
                   *(*__ctype_b_loc()).offset(*news.offset(i as isize) as
                                                  libc::c_uchar as libc::c_int
                                                  as isize) as libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int {
                if *news.offset(i as isize) as libc::c_int != '%' as i32 {
                    luaL_error(L,
                               b"invalid use of \'%c\' in replacement string\x00"
                                   as *const u8 as *const libc::c_char,
                               '%' as i32);
                }
                let fresh22 = (*b).n;
                (*b).n = (*b).n.wrapping_add(1);
                *(*b).b.offset(fresh22 as isize) = *news.offset(i as isize)
            } else if *news.offset(i as isize) as libc::c_int == '0' as i32 {
                luaL_addlstring(b, s,
                                s.offset_to(e).expect("bad offset_to") as
                                    libc::c_long as size_t);
            } else {
                push_onecapture(ms,
                                *news.offset(i as isize) as libc::c_int -
                                    '1' as i32, s, e);
                luaL_tolstring(L, -1i32, 0 as *mut size_t);
                lua_rotate(L, -2i32, -1i32);
                lua_settop(L, -1i32 - 1i32);
                luaL_addvalue(b);
            }
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn gmatch(mut L: *mut lua_State_0) -> libc::c_int {
    let mut ls: size_t = 0;
    let mut lp: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut ls);
    let mut p: *const libc::c_char = luaL_checklstring(L, 2i32, &mut lp);
    let mut gm: *mut GMatchState_0 = 0 as *mut GMatchState_0;
    lua_settop(L, 2i32);
    gm =
        lua_newuserdata(L,
                        ::std::mem::size_of::<GMatchState_0>() as
                            libc::c_ulong) as *mut GMatchState_0;
    prepstate(&mut (*gm).ms, L, s, ls, p, lp);
    (*gm).src = s;
    (*gm).p = p;
    (*gm).lastmatch = 0 as *const libc::c_char;
    lua_pushcclosure(L, Some(gmatch_aux), 3i32);
    return 1i32;
}
unsafe extern "C" fn gmatch_aux(mut L: *mut lua_State_0) -> libc::c_int {
    let mut gm: *mut GMatchState_0 =
        lua_touserdata(L, -50000i32 - 1000i32 - 3i32) as *mut GMatchState_0;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    (*gm).ms.L = L;
    src = (*gm).src;
    while src <= (*gm).ms.src_end {
        let mut e: *const libc::c_char = 0 as *const libc::c_char;
        reprepstate(&mut (*gm).ms);
        e = match_0(&mut (*gm).ms, src, (*gm).p);
        if !e.is_null() && e != (*gm).lastmatch {
            (*gm).lastmatch = e;
            (*gm).src = (*gm).lastmatch;
            return push_captures(&mut (*gm).ms, src, e)
        } else { src = src.offset(1isize) }
    }
    return 0i32;
}
unsafe extern "C" fn str_format(mut L: *mut lua_State_0) -> libc::c_int {
    let mut top: libc::c_int = lua_gettop(L);
    let mut arg: libc::c_int = 1i32;
    let mut sfl: size_t = 0;
    let mut strfrmt: *const libc::c_char =
        luaL_checklstring(L, arg, &mut sfl);
    let mut strfrmt_end: *const libc::c_char = strfrmt.offset(sfl as isize);
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    luaL_buffinit(L, &mut b);
    while strfrmt < strfrmt_end {
        if *strfrmt as libc::c_int != '%' as i32 {
            let fresh24 = b.n;
            b.n = b.n.wrapping_add(1);
            let fresh23 = strfrmt;
            strfrmt = strfrmt.offset(1);
            *b.b.offset(fresh24 as isize) = *fresh23
        } else {
            strfrmt = strfrmt.offset(1isize);
            if *strfrmt as libc::c_int == '%' as i32 {
                let fresh26 = b.n;
                b.n = b.n.wrapping_add(1);
                let fresh25 = strfrmt;
                strfrmt = strfrmt.offset(1);
                *b.b.offset(fresh26 as isize) = *fresh25
            } else {
                let mut form: [libc::c_char; 32] = [0; 32];
                let mut buff: *mut libc::c_char =
                    luaL_prepbuffsize(&mut b, (120i32 + 308i32) as size_t);
                let mut nb: libc::c_int = 0i32;
                arg += 1;
                if arg > top {
                    luaL_argerror(L, arg,
                                  b"no value\x00" as *const u8 as
                                      *const libc::c_char);
                }
                strfrmt = scanformat(L, strfrmt, form.as_mut_ptr());
                let fresh27 = strfrmt;
                strfrmt = strfrmt.offset(1);
                match *fresh27 as libc::c_int {
                    99 => {
                        memset(buff as *mut libc::c_void, 171i32,
                               (120i32 + 308i32) as libc::c_ulong);
                        nb =
                            snprintf(buff, (120i32 + 308i32) as libc::c_ulong,
                                     form.as_mut_ptr(),
                                     luaL_checkinteger(L, arg) as libc::c_int)
                    }
                    100 | 105 | 111 | 117 | 120 | 88 => {
                        let mut n: lua_Integer = luaL_checkinteger(L, arg);
                        addlenmod(form.as_mut_ptr(),
                                  b"ll\x00" as *const u8 as
                                      *const libc::c_char);
                        memset(buff as *mut libc::c_void, 171i32,
                               (120i32 + 308i32) as libc::c_ulong);
                        nb =
                            snprintf(buff, (120i32 + 308i32) as libc::c_ulong,
                                     form.as_mut_ptr(), n)
                    }
                    97 | 65 => {
                        addlenmod(form.as_mut_ptr(),
                                  b"\x00" as *const u8 as
                                      *const libc::c_char);
                        memset(buff as *mut libc::c_void, 171i32,
                               (120i32 + 308i32) as libc::c_ulong);
                        nb =
                            snprintf(buff, (120i32 + 308i32) as libc::c_ulong,
                                     form.as_mut_ptr(),
                                     luaL_checknumber(L, arg))
                    }
                    101 | 69 | 102 | 103 | 71 => {
                        let mut n_0: lua_Number = luaL_checknumber(L, arg);
                        addlenmod(form.as_mut_ptr(),
                                  b"\x00" as *const u8 as
                                      *const libc::c_char);
                        memset(buff as *mut libc::c_void, 171i32,
                               (120i32 + 308i32) as libc::c_ulong);
                        nb =
                            snprintf(buff, (120i32 + 308i32) as libc::c_ulong,
                                     form.as_mut_ptr(), n_0)
                    }
                    113 => { addliteral(L, &mut b, arg); }
                    115 => {
                        let mut l: size_t = 0;
                        let mut s: *const libc::c_char =
                            luaL_tolstring(L, arg, &mut l);
                        if form[2usize] as libc::c_int == '\u{0}' as i32 {
                            luaL_addvalue(&mut b);
                        } else if strchr(form.as_mut_ptr(),
                                         '.' as i32).is_null() &&
                                      l >= 100i32 as libc::c_ulong {
                            luaL_addvalue(&mut b);
                        } else {
                            memset(buff as *mut libc::c_void, 171i32,
                                   (120i32 + 308i32) as libc::c_ulong);
                            nb =
                                snprintf(buff,
                                         (120i32 + 308i32) as libc::c_ulong,
                                         form.as_mut_ptr(), s);
                            lua_settop(L, -1i32 - 1i32);
                        }
                    }
                    _ => {
                        return luaL_error(L,
                                          b"invalid option \'%%%c\' to \'format\'\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          *strfrmt.offset(-1isize) as
                                              libc::c_int)
                    }
                }
                if nb < 120i32 + 308i32 {
                } else {
                    __assert_fail(b"nb < (120 + (308))\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lstrlib.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1088i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 28],
                                                            &[libc::c_char; 28]>(b"int str_format(lua_State *)\x00")).as_ptr());
                };
                b.n =
                    (b.n as libc::c_ulong).wrapping_add(nb as libc::c_ulong)
                        as size_t as size_t
            }
        }
    }
    luaL_pushresult(&mut b);
    return 1i32;
}
unsafe extern "C" fn addliteral(mut L: *mut lua_State_0,
                                mut b: *mut luaL_Buffer_0,
                                mut arg: libc::c_int) -> () {
    let mut format: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: lua_Number = 0.;
    let mut n_0: lua_Integer = 0;
    match lua_type(L, arg) {
        4 => {
            let mut len: size_t = 0;
            let mut s: *const libc::c_char = lua_tolstring(L, arg, &mut len);
            addquoted(b, s, len);
        }
        3 => {
            let mut buff: *mut libc::c_char =
                luaL_prepbuffsize(b, (120i32 + 308i32) as size_t);
            let mut nb: libc::c_int = 0;
            if 0 == lua_isinteger(L, arg) {
                n = lua_tonumberx(L, arg, 0 as *mut libc::c_int);
                memset(buff as *mut libc::c_void, 171i32,
                       (120i32 + 308i32) as libc::c_ulong);
                nb =
                    snprintf(buff, (120i32 + 308i32) as libc::c_ulong,
                             b"%a\x00" as *const u8 as *const libc::c_char,
                             n);
                checkdp(buff, nb);
            } else {
                n_0 = lua_tointegerx(L, arg, 0 as *mut libc::c_int);
                format =
                    if n_0 == -9223372036854775807i64 - 1i64 {
                        b"0x%llx\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"%lld\x00" as *const u8 as *const libc::c_char
                    };
                memset(buff as *mut libc::c_void, 171i32,
                       (120i32 + 308i32) as libc::c_ulong);
                nb =
                    snprintf(buff, (120i32 + 308i32) as libc::c_ulong, format,
                             n_0)
            }
            (*b).n =
                ((*b).n as libc::c_ulong).wrapping_add(nb as libc::c_ulong) as
                    size_t as size_t
        }
        0 | 1 => {
            luaL_tolstring(L, arg, 0 as *mut size_t);
            luaL_addvalue(b);
        }
        _ => {
            luaL_argerror(L, arg,
                          b"value has no literal form\x00" as *const u8 as
                              *const libc::c_char);
        }
    };
}
unsafe extern "C" fn checkdp(mut buff: *mut libc::c_char, mut nb: libc::c_int)
 -> () {
    if memchr(buff as *const libc::c_void, '.' as i32,
              nb as libc::c_ulong).is_null() {
        let mut point: libc::c_char =
            *(*localeconv()).decimal_point.offset(0isize);
        let mut ppoint: *mut libc::c_char =
            memchr(buff as *const libc::c_void, point as libc::c_int,
                   nb as libc::c_ulong) as *mut libc::c_char;
        if !ppoint.is_null() { *ppoint = '.' as i32 as libc::c_char }
    };
}
unsafe extern "C" fn addquoted(mut b: *mut luaL_Buffer_0,
                               mut s: *const libc::c_char, mut len: size_t)
 -> () {
    let fresh28 = (*b).n;
    (*b).n = (*b).n.wrapping_add(1);
    *(*b).b.offset(fresh28 as isize) = '\"' as i32 as libc::c_char;
    loop  {
        let fresh29 = len;
        len = len.wrapping_sub(1);
        if !(0 != fresh29) { break ; }
        if *s as libc::c_int == '\"' as i32 ||
               *s as libc::c_int == '\\' as i32 ||
               *s as libc::c_int == '\n' as i32 {
            let fresh30 = (*b).n;
            (*b).n = (*b).n.wrapping_add(1);
            *(*b).b.offset(fresh30 as isize) = '\\' as i32 as libc::c_char;
            let fresh31 = (*b).n;
            (*b).n = (*b).n.wrapping_add(1);
            *(*b).b.offset(fresh31 as isize) = *s
        } else if 0 !=
                      *(*__ctype_b_loc()).offset(*s as libc::c_uchar as
                                                     libc::c_int as isize) as
                          libc::c_int &
                          _IScntrl as libc::c_int as libc::c_ushort as
                              libc::c_int {
            let mut buff: [libc::c_char; 10] = [0; 10];
            if 0 ==
                   *(*__ctype_b_loc()).offset(*s.offset(1isize) as
                                                  libc::c_uchar as libc::c_int
                                                  as isize) as libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int {
                memset(buff.as_mut_ptr() as *mut libc::c_void, 171i32,
                       ::std::mem::size_of::<[libc::c_char; 10]>() as
                           libc::c_ulong);
                snprintf(buff.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 10]>() as
                             libc::c_ulong,
                         b"\\%d\x00" as *const u8 as *const libc::c_char,
                         *s as libc::c_uchar as libc::c_int);
            } else {
                memset(buff.as_mut_ptr() as *mut libc::c_void, 171i32,
                       ::std::mem::size_of::<[libc::c_char; 10]>() as
                           libc::c_ulong);
                snprintf(buff.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 10]>() as
                             libc::c_ulong,
                         b"\\%03d\x00" as *const u8 as *const libc::c_char,
                         *s as libc::c_uchar as libc::c_int);
            }
            luaL_addstring(b, buff.as_mut_ptr());
        } else {
            let fresh32 = (*b).n;
            (*b).n = (*b).n.wrapping_add(1);
            *(*b).b.offset(fresh32 as isize) = *s
        }
        s = s.offset(1isize)
    }
    let fresh33 = (*b).n;
    (*b).n = (*b).n.wrapping_add(1);
    *(*b).b.offset(fresh33 as isize) = '\"' as i32 as libc::c_char;
}
unsafe extern "C" fn addlenmod(mut form: *mut libc::c_char,
                               mut lenmod: *const libc::c_char) -> () {
    let mut l: size_t = strlen(form);
    let mut lm: size_t = strlen(lenmod);
    let mut spec: libc::c_char =
        *form.offset(l.wrapping_sub(1i32 as libc::c_ulong) as isize);
    strcpy(form.offset(l as isize).offset(-1isize), lenmod);
    *form.offset(l.wrapping_add(lm).wrapping_sub(1i32 as libc::c_ulong) as
                     isize) = spec;
    *form.offset(l.wrapping_add(lm) as isize) =
        '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn scanformat(mut L: *mut lua_State_0,
                                mut strfrmt: *const libc::c_char,
                                mut form: *mut libc::c_char)
 -> *const libc::c_char {
    let mut p: *const libc::c_char = strfrmt;
    while *p as libc::c_int != '\u{0}' as i32 &&
              !strchr(b"-+ #0\x00" as *const u8 as *const libc::c_char,
                      *p as libc::c_int).is_null() {
        p = p.offset(1isize)
    }
    if strfrmt.offset_to(p).expect("bad offset_to") as libc::c_long as size_t
           >=
           (::std::mem::size_of::<[libc::c_char; 6]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong) {
        luaL_error(L,
                   b"invalid format (repeated flags)\x00" as *const u8 as
                       *const libc::c_char);
    }
    if 0 !=
           *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int {
        p = p.offset(1isize)
    }
    if 0 !=
           *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int {
        p = p.offset(1isize)
    }
    if *p as libc::c_int == '.' as i32 {
        p = p.offset(1isize);
        if 0 !=
               *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int
                                              as isize) as libc::c_int &
                   _ISdigit as libc::c_int as libc::c_ushort as libc::c_int {
            p = p.offset(1isize)
        }
        if 0 !=
               *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int
                                              as isize) as libc::c_int &
                   _ISdigit as libc::c_int as libc::c_ushort as libc::c_int {
            p = p.offset(1isize)
        }
    }
    if 0 !=
           *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int {
        luaL_error(L,
                   b"invalid format (width or precision too long)\x00" as
                       *const u8 as *const libc::c_char);
    }
    let fresh34 = form;
    form = form.offset(1);
    *fresh34 = '%' as i32 as libc::c_char;
    memcpy(form as *mut libc::c_void, strfrmt as *const libc::c_void,
           ((strfrmt.offset_to(p).expect("bad offset_to") as libc::c_long +
                 1i32 as libc::c_long) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    form =
        form.offset((strfrmt.offset_to(p).expect("bad offset_to") as
                         libc::c_long + 1i32 as libc::c_long) as isize);
    *form = '\u{0}' as i32 as libc::c_char;
    return p;
}
unsafe extern "C" fn str_find(mut L: *mut lua_State_0) -> libc::c_int {
    return str_find_aux(L, 1i32);
}
unsafe extern "C" fn str_dump(mut L: *mut lua_State_0) -> libc::c_int {
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    let mut strip: libc::c_int = lua_toboolean(L, 2i32);
    luaL_checktype(L, 1i32, 6i32);
    lua_settop(L, 1i32);
    luaL_buffinit(L, &mut b);
    if lua_dump(L, Some(writer),
                &mut b as *mut luaL_Buffer_0 as *mut libc::c_void, strip) !=
           0i32 {
        return luaL_error(L,
                          b"unable to dump given function\x00" as *const u8 as
                              *const libc::c_char)
    } else { luaL_pushresult(&mut b); return 1i32 };
}
unsafe extern "C" fn writer(mut L: *mut lua_State_0,
                            mut b: *const libc::c_void, mut size: size_t,
                            mut B: *mut libc::c_void) -> libc::c_int {
    luaL_addlstring(B as *mut luaL_Buffer_0, b as *const libc::c_char, size);
    return 0i32;
}
unsafe extern "C" fn str_char(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut i: libc::c_int = 0;
    let mut b: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    let mut p: *mut libc::c_char = luaL_buffinitsize(L, &mut b, n as size_t);
    i = 1i32;
    while i <= n {
        let mut c: lua_Integer = luaL_checkinteger(L, i);
        *p.offset((i - 1i32) as isize) = c as libc::c_uchar as libc::c_char;
        i += 1
    }
    luaL_pushresultsize(&mut b, n as size_t);
    return 1i32;
}
unsafe extern "C" fn str_byte(mut L: *mut lua_State_0) -> libc::c_int {
    let mut l: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut l);
    let mut posi: lua_Integer =
        posrelat(luaL_optinteger(L, 2i32, 1i32 as lua_Integer), l);
    let mut pose: lua_Integer = posrelat(luaL_optinteger(L, 3i32, posi), l);
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if posi < 1i32 as libc::c_longlong { posi = 1i32 as lua_Integer }
    if pose > l as lua_Integer { pose = l as lua_Integer }
    if posi > pose {
        return 0i32
    } else if pose - posi >= 2147483647i32 as libc::c_longlong {
        return luaL_error(L,
                          b"string slice too long\x00" as *const u8 as
                              *const libc::c_char)
    } else {
        n = (pose - posi) as libc::c_int + 1i32;
        luaL_checkstack(L, n,
                        b"string slice too long\x00" as *const u8 as
                            *const libc::c_char);
        i = 0i32;
        while i < n {
            lua_pushinteger(L,
                            *s.offset((posi + i as libc::c_longlong -
                                           1i32 as libc::c_longlong) as isize)
                                as libc::c_uchar as lua_Integer);
            i += 1
        }
        return n
    };
}
