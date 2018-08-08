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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    static mut l_memcontrol: Memcontrol_0;
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
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
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
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
    fn lua_pushlstring(L: *mut lua_State_0, s: *const libc::c_char,
                       len: size_t) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State_0, fn_0: lua_CFunction,
                        n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State_0, p: *mut libc::c_void) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer)
     -> libc::c_int;
    #[no_mangle]
    fn lua_rawgetp(L: *mut lua_State_0, idx: libc::c_int,
                   p: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int,
                       nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer)
     -> ();
    #[no_mangle]
    fn lua_rawsetp(L: *mut lua_State_0, idx: libc::c_int,
                   p: *const libc::c_void) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State_0, objindex: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lua_callk(L: *mut lua_State_0, nargs: libc::c_int,
                 nresults: libc::c_int, ctx: lua_KContext, k: lua_KFunction)
     -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t)
     -> ();
    #[no_mangle]
    fn luaL_checklstring(L: *mut lua_State_0, arg: libc::c_int,
                         l: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_optlstring(L: *mut lua_State_0, arg: libc::c_int,
                       def: *const libc::c_char, l: *mut size_t)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaL_loadfilex(L: *mut lua_State_0, filename: *const libc::c_char,
                      mode: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_len(L: *mut lua_State_0, idx: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_gsub(L: *mut lua_State_0, s: *const libc::c_char,
                 p: *const libc::c_char, r: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0,
                     nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_getsubtable(L: *mut lua_State_0, idx: libc::c_int,
                        fname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State_0, B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer_0) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
    #[no_mangle]
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlerror() -> *mut libc::c_char;
    #[no_mangle]
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State_0,
    pub initb: [libc::c_char; 23],
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
pub type luaL_Buffer_0 = luaL_Buffer;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
pub type luaL_Reg_0 = luaL_Reg;
#[no_mangle]
pub unsafe extern "C" fn luaopen_package(mut L: *mut lua_State_0)
 -> libc::c_int {
    createclibstable(L);
    luaL_checkversion_(L, 503i32 as lua_Number,
                       (::std::mem::size_of::<lua_Integer>() as
                            libc::c_ulong).wrapping_mul(16i32 as
                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<lua_Number>()
                                                                                            as
                                                                                            libc::c_ulong));
    lua_createtable(L, 0i32,
                    (::std::mem::size_of::<[luaL_Reg_0; 8]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_Reg_0>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int);
    luaL_setfuncs(L, pk_funcs.as_ptr(), 0i32);
    createsearcherstable(L);
    setpath(L, b"path\x00" as *const u8 as *const libc::c_char,
            b"LUA_PATH\x00" as *const u8 as *const libc::c_char,
            b"/usr/local/share/lua/5.3/?.lua;/usr/local/share/lua/5.3/?/init.lua;/usr/local/lib/lua/5.3/?.lua;/usr/local/lib/lua/5.3/?/init.lua;./?.lua;./?/init.lua\x00"
                as *const u8 as *const libc::c_char);
    setpath(L, b"cpath\x00" as *const u8 as *const libc::c_char,
            b"LUA_CPATH\x00" as *const u8 as *const libc::c_char,
            b"/usr/local/lib/lua/5.3/?.so;/usr/local/lib/lua/5.3/loadall.so;./?.so\x00"
                as *const u8 as *const libc::c_char);
    lua_pushstring(L,
                   b"/\n;\n?\n!\n-\n\x00" as *const u8 as
                       *const libc::c_char);
    lua_setfield(L, -2i32, b"config\x00" as *const u8 as *const libc::c_char);
    luaL_getsubtable(L, -50000i32 - 1000i32,
                     b"_LOADED\x00" as *const u8 as *const libc::c_char);
    lua_setfield(L, -2i32, b"loaded\x00" as *const u8 as *const libc::c_char);
    luaL_getsubtable(L, -50000i32 - 1000i32,
                     b"_PRELOAD\x00" as *const u8 as *const libc::c_char);
    lua_setfield(L, -2i32,
                 b"preload\x00" as *const u8 as *const libc::c_char);
    lua_rawgeti(L, -50000i32 - 1000i32, 2i32 as lua_Integer);
    lua_pushvalue(L, -2i32);
    luaL_setfuncs(L, ll_funcs.as_ptr(), 1i32);
    lua_settop(L, -1i32 - 1i32);
    return 1i32;
}
static mut ll_funcs: [luaL_Reg_0; 2] =
    unsafe {
        [luaL_Reg{name: b"require\x00" as *const u8 as *const libc::c_char,
                  func: Some(ll_require),},
         luaL_Reg{name: 0 as *const libc::c_char, func: None,}]
    };
unsafe extern "C" fn ll_require(mut L: *mut lua_State_0) -> libc::c_int {
    let mut name: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_settop(L, 1i32);
    lua_getfield(L, -50000i32 - 1000i32,
                 b"_LOADED\x00" as *const u8 as *const libc::c_char);
    lua_getfield(L, 2i32, name);
    if 0 != lua_toboolean(L, -1i32) {
        return 1i32
    } else {
        lua_settop(L, -1i32 - 1i32);
        findloader(L, name);
        lua_pushstring(L, name);
        lua_rotate(L, -2i32, 1i32);
        lua_callk(L, 2i32, 1i32, 0i32 as lua_KContext, None);
        if !(lua_type(L, -1i32) == 0i32) { lua_setfield(L, 2i32, name); }
        if lua_getfield(L, 2i32, name) == 0i32 {
            lua_pushboolean(L, 1i32);
            lua_pushvalue(L, -1i32);
            lua_setfield(L, 2i32, name);
        }
        return 1i32
    };
}
unsafe extern "C" fn findloader(mut L: *mut lua_State_0,
                                mut name: *const libc::c_char) -> () {
    let mut i: libc::c_int = 0;
    let mut msg: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    luaL_buffinit(L, &mut msg);
    if lua_getfield(L, -50000i32 - 1000i32 - 1i32,
                    b"searchers\x00" as *const u8 as *const libc::c_char) !=
           5i32 {
        luaL_error(L,
                   b"\'package.searchers\' must be a table\x00" as *const u8
                       as *const libc::c_char);
    }
    i = 1i32;
    loop  {
        if lua_rawgeti(L, 3i32, i as lua_Integer) == 0i32 {
            lua_settop(L, -1i32 - 1i32);
            luaL_pushresult(&mut msg);
            luaL_error(L,
                       b"module \'%s\' not found:%s\x00" as *const u8 as
                           *const libc::c_char, name,
                       lua_tolstring(L, -1i32, 0 as *mut size_t));
        }
        lua_pushstring(L, name);
        lua_callk(L, 1i32, 2i32, 0i32 as lua_KContext, None);
        if lua_type(L, -2i32) == 6i32 {
            return
        } else {
            if 0 != lua_isstring(L, -2i32) {
                lua_settop(L, -1i32 - 1i32);
                luaL_addvalue(&mut msg);
            } else { lua_settop(L, -2i32 - 1i32); }
            i += 1
        }
    };
}
unsafe extern "C" fn setpath(mut L: *mut lua_State_0,
                             mut fieldname: *const libc::c_char,
                             mut envname: *const libc::c_char,
                             mut dft: *const libc::c_char) -> () {
    let mut nver: *const libc::c_char =
        lua_pushfstring(L, b"%s%s\x00" as *const u8 as *const libc::c_char,
                        envname,
                        b"_5_3\x00" as *const u8 as *const libc::c_char);
    let mut path: *const libc::c_char = getenv(nver);
    if path.is_null() { path = getenv(envname) }
    if path.is_null() || 0 != noenv(L) {
        lua_pushstring(L, dft);
    } else {
        path =
            luaL_gsub(L, path, b";;\x00" as *const u8 as *const libc::c_char,
                      b";\x01;\x00" as *const u8 as *const libc::c_char);
        luaL_gsub(L, path, b"\x01\x00" as *const u8 as *const libc::c_char,
                  dft);
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    }
    lua_setfield(L, -3i32, fieldname);
    lua_settop(L, -1i32 - 1i32);
}
unsafe extern "C" fn noenv(mut L: *mut lua_State_0) -> libc::c_int {
    let mut b: libc::c_int = 0;
    lua_getfield(L, -50000i32 - 1000i32,
                 b"LUA_NOENV\x00" as *const u8 as *const libc::c_char);
    b = lua_toboolean(L, -1i32);
    lua_settop(L, -1i32 - 1i32);
    return b;
}
unsafe extern "C" fn createsearcherstable(mut L: *mut lua_State_0) -> () {
    static mut searchers: [lua_CFunction; 5] =
        unsafe {
            [Some(searcher_preload), Some(searcher_Lua), Some(searcher_C),
             Some(searcher_Croot), None]
        };
    let mut i: libc::c_int = 0;
    lua_createtable(L,
                    (::std::mem::size_of::<[lua_CFunction; 5]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<lua_CFunction>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int, 0i32);
    i = 0i32;
    while searchers[i as usize].is_some() {
        lua_pushvalue(L, -2i32);
        lua_pushcclosure(L, searchers[i as usize], 1i32);
        lua_rawseti(L, -2i32, (i + 1i32) as lua_Integer);
        i += 1
    }
    lua_setfield(L, -2i32,
                 b"searchers\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn searcher_Croot(mut L: *mut lua_State_0) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut p: *const libc::c_char = strchr(name, '.' as i32);
    let mut stat: libc::c_int = 0;
    if p.is_null() {
        return 0i32
    } else {
        lua_pushlstring(L, name,
                        name.offset_to(p).expect("bad offset_to") as
                            libc::c_long as size_t);
        filename =
            findfile(L, lua_tolstring(L, -1i32, 0 as *mut size_t),
                     b"cpath\x00" as *const u8 as *const libc::c_char,
                     b"/\x00" as *const u8 as *const libc::c_char);
        if filename.is_null() {
            return 1i32
        } else {
            stat = loadfunc(L, filename, name);
            if stat != 0i32 {
                if stat != 2i32 {
                    return checkload(L, 0i32, filename)
                } else {
                    lua_pushfstring(L,
                                    b"\n\tno module \'%s\' in file \'%s\'\x00"
                                        as *const u8 as *const libc::c_char,
                                    name, filename);
                    return 1i32
                }
            } else { lua_pushstring(L, filename); return 2i32 }
        }
    };
}
unsafe extern "C" fn checkload(mut L: *mut lua_State_0, mut stat: libc::c_int,
                               mut filename: *const libc::c_char)
 -> libc::c_int {
    if 0 != stat {
        lua_pushstring(L, filename);
        return 2i32
    } else {
        return luaL_error(L,
                          b"error loading module \'%s\' from file \'%s\':\n\t%s\x00"
                              as *const u8 as *const libc::c_char,
                          lua_tolstring(L, 1i32, 0 as *mut size_t), filename,
                          lua_tolstring(L, -1i32, 0 as *mut size_t))
    };
}
unsafe extern "C" fn loadfunc(mut L: *mut lua_State_0,
                              mut filename: *const libc::c_char,
                              mut modname: *const libc::c_char)
 -> libc::c_int {
    let mut openfunc: *const libc::c_char = 0 as *const libc::c_char;
    let mut mark: *const libc::c_char = 0 as *const libc::c_char;
    modname =
        luaL_gsub(L, modname, b".\x00" as *const u8 as *const libc::c_char,
                  b"_\x00" as *const u8 as *const libc::c_char);
    mark =
        strchr(modname,
               *(b"-\x00" as *const u8 as *const libc::c_char) as
                   libc::c_int);
    if !mark.is_null() {
        let mut stat: libc::c_int = 0;
        openfunc =
            lua_pushlstring(L, modname,
                            modname.offset_to(mark).expect("bad offset_to") as
                                libc::c_long as size_t);
        openfunc =
            lua_pushfstring(L,
                            b"luaopen_%s\x00" as *const u8 as
                                *const libc::c_char, openfunc);
        stat = lookforfunc(L, filename, openfunc);
        if stat != 2i32 { return stat } else { modname = mark.offset(1isize) }
    }
    openfunc =
        lua_pushfstring(L,
                        b"luaopen_%s\x00" as *const u8 as *const libc::c_char,
                        modname);
    return lookforfunc(L, filename, openfunc);
}
unsafe extern "C" fn lookforfunc(mut L: *mut lua_State_0,
                                 mut path: *const libc::c_char,
                                 mut sym: *const libc::c_char)
 -> libc::c_int {
    let mut reg: *mut libc::c_void = checkclib(L, path);
    if reg.is_null() {
        reg =
            lsys_load(L, path,
                      (*sym as libc::c_int == '*' as i32) as libc::c_int);
        if reg.is_null() { return 1i32 } else { addtoclib(L, path, reg); }
    }
    if *sym as libc::c_int == '*' as i32 {
        lua_pushboolean(L, 1i32);
        return 0i32
    } else {
        let mut f: lua_CFunction = lsys_sym(L, reg, sym);
        if f.is_none() {
            return 2i32
        } else { lua_pushcclosure(L, f, 0i32); return 0i32 }
    };
}
unsafe extern "C" fn checkclib(mut L: *mut lua_State_0,
                               mut path: *const libc::c_char)
 -> *mut libc::c_void {
    let mut plib: *mut libc::c_void = 0 as *mut libc::c_void;
    lua_rawgetp(L, -50000i32 - 1000i32,
                &CLIBS as *const libc::c_int as *const libc::c_void);
    lua_getfield(L, -1i32, path);
    plib = lua_touserdata(L, -1i32);
    lua_settop(L, -2i32 - 1i32);
    return plib;
}
static mut CLIBS: libc::c_int = unsafe { 0i32 };
unsafe extern "C" fn lsys_sym(mut L: *mut lua_State_0,
                              mut lib: *mut libc::c_void,
                              mut sym: *const libc::c_char) -> lua_CFunction {
    let mut f: lua_CFunction =
        ::std::mem::transmute::<*mut libc::c_void,
                                lua_CFunction>(dlsym(lib, sym));
    if f.is_none() { lua_pushstring(L, dlerror()); }
    return f;
}
unsafe extern "C" fn addtoclib(mut L: *mut lua_State_0,
                               mut path: *const libc::c_char,
                               mut plib: *mut libc::c_void) -> () {
    lua_rawgetp(L, -50000i32 - 1000i32,
                &CLIBS as *const libc::c_int as *const libc::c_void);
    lua_pushlightuserdata(L, plib);
    lua_pushvalue(L, -1i32);
    lua_setfield(L, -3i32, path);
    lua_rawseti(L, -2i32, luaL_len(L, -2i32) + 1i32 as libc::c_longlong);
    lua_settop(L, -1i32 - 1i32);
}
unsafe extern "C" fn lsys_load(mut L: *mut lua_State_0,
                               mut path: *const libc::c_char,
                               mut seeglb: libc::c_int) -> *mut libc::c_void {
    let mut lib: *mut libc::c_void =
        dlopen(path, 2i32 | if 0 != seeglb { 256i32 } else { 0i32 });
    if lib.is_null() { lua_pushstring(L, dlerror()); }
    return lib;
}
unsafe extern "C" fn findfile(mut L: *mut lua_State_0,
                              mut name: *const libc::c_char,
                              mut pname: *const libc::c_char,
                              mut dirsep: *const libc::c_char)
 -> *const libc::c_char {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    lua_getfield(L, -50000i32 - 1000i32 - 1i32, pname);
    path = lua_tolstring(L, -1i32, 0 as *mut size_t);
    if path.is_null() {
        luaL_error(L,
                   b"\'package.%s\' must be a string\x00" as *const u8 as
                       *const libc::c_char, pname);
    }
    return searchpath(L, name, path,
                      b".\x00" as *const u8 as *const libc::c_char, dirsep);
}
unsafe extern "C" fn searchpath(mut L: *mut lua_State_0,
                                mut name: *const libc::c_char,
                                mut path: *const libc::c_char,
                                mut sep: *const libc::c_char,
                                mut dirsep: *const libc::c_char)
 -> *const libc::c_char {
    let mut msg: luaL_Buffer_0 =
        luaL_Buffer{b: 0 as *mut libc::c_char,
                    size: 0,
                    n: 0,
                    L: 0 as *mut lua_State_0,
                    initb: [0; 23],};
    luaL_buffinit(L, &mut msg);
    if *sep as libc::c_int != '\u{0}' as i32 {
        name = luaL_gsub(L, name, sep, dirsep)
    }
    loop  {
        path = pushnexttemplate(L, path);
        if path.is_null() { break ; }
        let mut filename: *const libc::c_char =
            luaL_gsub(L, lua_tolstring(L, -1i32, 0 as *mut size_t),
                      b"?\x00" as *const u8 as *const libc::c_char, name);
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        if 0 != readable(filename) {
            return filename
        } else {
            lua_pushfstring(L,
                            b"\n\tno file \'%s\'\x00" as *const u8 as
                                *const libc::c_char, filename);
            lua_rotate(L, -2i32, -1i32);
            lua_settop(L, -1i32 - 1i32);
            luaL_addvalue(&mut msg);
        }
    }
    luaL_pushresult(&mut msg);
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn readable(mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut f: *mut FILE =
        fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() { return 0i32 } else { fclose(f); return 1i32 };
}
unsafe extern "C" fn pushnexttemplate(mut L: *mut lua_State_0,
                                      mut path: *const libc::c_char)
 -> *const libc::c_char {
    let mut l: *const libc::c_char = 0 as *const libc::c_char;
    while *path as libc::c_int ==
              *(b";\x00" as *const u8 as *const libc::c_char) as libc::c_int {
        path = path.offset(1isize)
    }
    if *path as libc::c_int == '\u{0}' as i32 {
        return 0 as *const libc::c_char
    } else {
        l =
            strchr(path,
                   *(b";\x00" as *const u8 as *const libc::c_char) as
                       libc::c_int);
        if l.is_null() { l = path.offset(strlen(path) as isize) }
        lua_pushlstring(L, path,
                        path.offset_to(l).expect("bad offset_to") as
                            libc::c_long as size_t);
        return l
    };
}
unsafe extern "C" fn searcher_C(mut L: *mut lua_State_0) -> libc::c_int {
    let mut name: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut filename: *const libc::c_char =
        findfile(L, name, b"cpath\x00" as *const u8 as *const libc::c_char,
                 b"/\x00" as *const u8 as *const libc::c_char);
    if filename.is_null() {
        return 1i32
    } else {
        return checkload(L,
                         (loadfunc(L, filename, name) == 0i32) as libc::c_int,
                         filename)
    };
}
unsafe extern "C" fn searcher_Lua(mut L: *mut lua_State_0) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    filename =
        findfile(L, name, b"path\x00" as *const u8 as *const libc::c_char,
                 b"/\x00" as *const u8 as *const libc::c_char);
    if filename.is_null() {
        return 1i32
    } else {
        return checkload(L,
                         (luaL_loadfilex(L, filename,
                                         0 as *const libc::c_char) == 0i32) as
                             libc::c_int, filename)
    };
}
unsafe extern "C" fn searcher_preload(mut L: *mut lua_State_0)
 -> libc::c_int {
    let mut name: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_getfield(L, -50000i32 - 1000i32,
                 b"_PRELOAD\x00" as *const u8 as *const libc::c_char);
    if lua_getfield(L, -1i32, name) == 0i32 {
        lua_pushfstring(L,
                        b"\n\tno field package.preload[\'%s\']\x00" as
                            *const u8 as *const libc::c_char, name);
    }
    return 1i32;
}
static mut pk_funcs: [luaL_Reg_0; 8] =
    unsafe {
        [luaL_Reg{name: b"loadlib\x00" as *const u8 as *const libc::c_char,
                  func: Some(ll_loadlib),},
         luaL_Reg{name: b"searchpath\x00" as *const u8 as *const libc::c_char,
                  func: Some(ll_searchpath),},
         luaL_Reg{name: b"preload\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: b"cpath\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: b"path\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: b"searchers\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: b"loaded\x00" as *const u8 as *const libc::c_char,
                  func: None,},
         luaL_Reg{name: 0 as *const libc::c_char, func: None,}]
    };
unsafe extern "C" fn ll_searchpath(mut L: *mut lua_State_0) -> libc::c_int {
    let mut f: *const libc::c_char =
        searchpath(L, luaL_checklstring(L, 1i32, 0 as *mut size_t),
                   luaL_checklstring(L, 2i32, 0 as *mut size_t),
                   luaL_optlstring(L, 3i32,
                                   b".\x00" as *const u8 as
                                       *const libc::c_char, 0 as *mut size_t),
                   luaL_optlstring(L, 4i32,
                                   b"/\x00" as *const u8 as
                                       *const libc::c_char,
                                   0 as *mut size_t));
    if !f.is_null() {
        return 1i32
    } else { lua_pushnil(L); lua_rotate(L, -2i32, 1i32); return 2i32 };
}
unsafe extern "C" fn ll_loadlib(mut L: *mut lua_State_0) -> libc::c_int {
    let mut path: *const libc::c_char =
        luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut init: *const libc::c_char =
        luaL_checklstring(L, 2i32, 0 as *mut size_t);
    let mut stat: libc::c_int = lookforfunc(L, path, init);
    if stat == 0i32 {
        return 1i32
    } else {
        lua_pushnil(L);
        lua_rotate(L, -2i32, 1i32);
        lua_pushstring(L,
                       if stat == 1i32 {
                           b"open\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"init\x00" as *const u8 as *const libc::c_char
                       });
        return 3i32
    };
}
unsafe extern "C" fn createclibstable(mut L: *mut lua_State_0) -> () {
    lua_createtable(L, 0i32, 0i32);
    lua_createtable(L, 0i32, 1i32);
    lua_pushcclosure(L, Some(gctm), 0i32);
    lua_setfield(L, -2i32, b"__gc\x00" as *const u8 as *const libc::c_char);
    lua_setmetatable(L, -2i32);
    lua_rawsetp(L, -50000i32 - 1000i32,
                &CLIBS as *const libc::c_int as *const libc::c_void);
}
unsafe extern "C" fn gctm(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: lua_Integer = luaL_len(L, 1i32);
    while n >= 1i32 as libc::c_longlong {
        lua_rawgeti(L, 1i32, n);
        lsys_unloadlib(lua_touserdata(L, -1i32));
        lua_settop(L, -1i32 - 1i32);
        n -= 1
    }
    return 0i32;
}
unsafe extern "C" fn lsys_unloadlib(mut lib: *mut libc::c_void) -> () {
    dlclose(lib);
}
