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
    pub type _IO_FILE_plus;
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
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int)
     -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State_0, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State_0, tp: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_compare(L: *mut lua_State_0, idx1: libc::c_int, idx2: libc::c_int,
                   op: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_geti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer)
     -> libc::c_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int,
                       nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State_0, objindex: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_seti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_callk(L: *mut lua_State_0, nargs: libc::c_int,
                 nresults: libc::c_int, ctx: lua_KContext, k: lua_KFunction)
     -> ();
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
    fn luaL_len(L: *mut lua_State_0, idx: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg,
                     nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State_0, B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const libc::c_char, l: size_t)
     -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static luaO_nilobject_: TValue;
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
pub type luaL_Buffer = luaL_Buffer_0;
pub type luaL_Reg = luaL_Reg_0;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
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
pub type lua_Unsigned = libc::c_ulonglong;
pub type IdxT = libc::c_uint;
pub type clock_t = __clock_t;
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
pub struct luaL_Reg_0 {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct luaL_Buffer_0 {
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State_0,
    pub initb: [libc::c_char; 23],
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_table(mut L: *mut lua_State_0)
 -> libc::c_int {
    luaL_checkversion_(L, 503i32 as lua_Number,
                       (::std::mem::size_of::<lua_Integer>() as
                            libc::c_ulong).wrapping_mul(16i32 as
                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<lua_Number>()
                                                                                            as
                                                                                            libc::c_ulong));
    lua_createtable(L, 0i32,
                    (::std::mem::size_of::<[luaL_Reg; 8]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<luaL_Reg>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int);
    luaL_setfuncs(L, tab_funcs.as_ptr(), 0i32);
    return 1i32;
}
static mut tab_funcs: [luaL_Reg; 8] =
    unsafe {
        [luaL_Reg_0{name: b"concat\x00" as *const u8 as *const libc::c_char,
                    func: Some(tconcat),},
         luaL_Reg_0{name: b"insert\x00" as *const u8 as *const libc::c_char,
                    func: Some(tinsert),},
         luaL_Reg_0{name: b"pack\x00" as *const u8 as *const libc::c_char,
                    func: Some(pack),},
         luaL_Reg_0{name: b"unpack\x00" as *const u8 as *const libc::c_char,
                    func: Some(unpack),},
         luaL_Reg_0{name: b"remove\x00" as *const u8 as *const libc::c_char,
                    func: Some(tremove),},
         luaL_Reg_0{name: b"move\x00" as *const u8 as *const libc::c_char,
                    func: Some(tmove),},
         luaL_Reg_0{name: b"sort\x00" as *const u8 as *const libc::c_char,
                    func: Some(sort),},
         luaL_Reg_0{name: 0 as *const libc::c_char, func: None,}]
    };
unsafe extern "C" fn sort(mut L: *mut lua_State_0) -> libc::c_int {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut n: lua_Integer = luaL_len(L, 1i32);
    if n > 1i32 as libc::c_longlong {
        if !(lua_type(L, 2i32) <= 0i32) { luaL_checktype(L, 2i32, 6i32); }
        lua_settop(L, 2i32);
        auxsort(L, 1i32 as IdxT, n as IdxT, 0i32 as libc::c_uint);
    }
    return 0i32;
}
unsafe extern "C" fn checktab(mut L: *mut lua_State_0, mut arg: libc::c_int,
                              mut what: libc::c_int) -> () {
    if lua_type(L, arg) != 5i32 {
        let mut n: libc::c_int = 1i32;
        if 0 != lua_getmetatable(L, arg) &&
               (0 == what & 1i32 ||
                    {
                        n += 1;
                        0 !=
                            checkfield(L,
                                       b"__index\x00" as *const u8 as
                                           *const libc::c_char, n)
                    }) &&
               (0 == what & 2i32 ||
                    {
                        n += 1;
                        0 !=
                            checkfield(L,
                                       b"__newindex\x00" as *const u8 as
                                           *const libc::c_char, n)
                    }) &&
               (0 == what & 4i32 ||
                    {
                        n += 1;
                        0 !=
                            checkfield(L,
                                       b"__len\x00" as *const u8 as
                                           *const libc::c_char, n)
                    }) {
            lua_settop(L, -n - 1i32);
        } else { luaL_checktype(L, arg, 5i32); }
    };
}
unsafe extern "C" fn checkfield(mut L: *mut lua_State_0,
                                mut key: *const libc::c_char,
                                mut n: libc::c_int) -> libc::c_int {
    lua_pushstring(L, key);
    return (lua_rawget(L, -n) != 0i32) as libc::c_int;
}
unsafe extern "C" fn auxsort(mut L: *mut lua_State_0, mut lo: IdxT,
                             mut up: IdxT, mut rnd: libc::c_uint) -> () {
    while lo < up {
        let mut p: IdxT = 0;
        let mut n: IdxT = 0;
        lua_geti(L, 1i32, lo as lua_Integer);
        lua_geti(L, 1i32, up as lua_Integer);
        if 0 != sort_comp(L, -1i32, -2i32) {
            set2(L, lo, up);
        } else { lua_settop(L, -2i32 - 1i32); }
        if up.wrapping_sub(lo) == 1i32 as libc::c_uint {
            return
        } else {
            if up.wrapping_sub(lo) < 100u32 || rnd == 0i32 as libc::c_uint {
                p = lo.wrapping_add(up).wrapping_div(2i32 as libc::c_uint)
            } else { p = choosePivot(lo, up, rnd) }
            lua_geti(L, 1i32, p as lua_Integer);
            lua_geti(L, 1i32, lo as lua_Integer);
            if 0 != sort_comp(L, -2i32, -1i32) {
                set2(L, p, lo);
            } else {
                lua_settop(L, -1i32 - 1i32);
                lua_geti(L, 1i32, up as lua_Integer);
                if 0 != sort_comp(L, -1i32, -2i32) {
                    set2(L, p, up);
                } else { lua_settop(L, -2i32 - 1i32); }
            }
            if up.wrapping_sub(lo) == 2i32 as libc::c_uint {
                return
            } else {
                lua_geti(L, 1i32, p as lua_Integer);
                lua_pushvalue(L, -1i32);
                lua_geti(L, 1i32,
                         up.wrapping_sub(1i32 as libc::c_uint) as
                             lua_Integer);
                set2(L, p, up.wrapping_sub(1i32 as libc::c_uint));
                p = partition(L, lo, up);
                if p.wrapping_sub(lo) < up.wrapping_sub(p) {
                    auxsort(L, lo, p.wrapping_sub(1i32 as libc::c_uint), rnd);
                    n = p.wrapping_sub(lo);
                    lo = p.wrapping_add(1i32 as libc::c_uint)
                } else {
                    auxsort(L, p.wrapping_add(1i32 as libc::c_uint), up, rnd);
                    n = up.wrapping_sub(p);
                    up = p.wrapping_sub(1i32 as libc::c_uint)
                }
                if !(up.wrapping_sub(lo).wrapping_div(128i32 as libc::c_uint)
                         > n) {
                    continue ;
                }
                rnd = l_randomizePivot()
            }
        }
    };
}
unsafe extern "C" fn l_randomizePivot() -> libc::c_uint {
    let mut c: clock_t = clock();
    let mut t: time_t = time(0 as *mut time_t);
    let mut buff: [libc::c_uint; 4] = [0; 4];
    let mut i: libc::c_uint = 0;
    let mut rnd: libc::c_uint = 0i32 as libc::c_uint;
    memcpy(buff.as_mut_ptr() as *mut libc::c_void,
           &mut c as *mut clock_t as *const libc::c_void,
           (::std::mem::size_of::<clock_t>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>()
                                                as
                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                as
                                                                                libc::c_ulong));
    memcpy(buff.as_mut_ptr().offset((::std::mem::size_of::<clock_t>() as
                                         libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong)
                                        as isize) as *mut libc::c_void,
           &mut t as *mut time_t as *const libc::c_void,
           (::std::mem::size_of::<time_t>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>()
                                                as
                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_uint>()
                                                                                as
                                                                                libc::c_ulong));
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[libc::c_uint; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>()
                                                   as libc::c_ulong) {
        rnd = rnd.wrapping_add(buff[i as usize]);
        i = i.wrapping_add(1)
    }
    return rnd;
}
unsafe extern "C" fn partition(mut L: *mut lua_State_0, mut lo: IdxT,
                               mut up: IdxT) -> IdxT {
    let mut i: IdxT = lo;
    let mut j: IdxT = up.wrapping_sub(1i32 as libc::c_uint);
    loop  {
        i = i.wrapping_add(1);
        lua_geti(L, 1i32, i as lua_Integer);
        if 0 != sort_comp(L, -1i32, -2i32) {
            if i == up.wrapping_sub(1i32 as libc::c_uint) {
                luaL_error(L,
                           b"invalid order function for sorting\x00" as
                               *const u8 as *const libc::c_char);
            }
            lua_settop(L, -1i32 - 1i32);
        } else {
            loop  {
                j = j.wrapping_sub(1);
                lua_geti(L, 1i32, j as lua_Integer);
                if !(0 != sort_comp(L, -3i32, -1i32)) { break ; }
                if j < i {
                    luaL_error(L,
                               b"invalid order function for sorting\x00" as
                                   *const u8 as *const libc::c_char);
                }
                lua_settop(L, -1i32 - 1i32);
            }
            if j < i {
                lua_settop(L, -1i32 - 1i32);
                set2(L, up.wrapping_sub(1i32 as libc::c_uint), i);
                return i
            } else { set2(L, i, j); }
        }
    };
}
unsafe extern "C" fn set2(mut L: *mut lua_State_0, mut i: IdxT, mut j: IdxT)
 -> () {
    lua_seti(L, 1i32, i as lua_Integer);
    lua_seti(L, 1i32, j as lua_Integer);
}
unsafe extern "C" fn sort_comp(mut L: *mut lua_State_0, mut a: libc::c_int,
                               mut b: libc::c_int) -> libc::c_int {
    if lua_type(L, 2i32) == 0i32 {
        return lua_compare(L, a, b, 1i32)
    } else {
        let mut res: libc::c_int = 0;
        lua_pushvalue(L, 2i32);
        lua_pushvalue(L, a - 1i32);
        lua_pushvalue(L, b - 2i32);
        lua_callk(L, 2i32, 1i32, 0i32 as lua_KContext, None);
        res = lua_toboolean(L, -1i32);
        lua_settop(L, -1i32 - 1i32);
        return res
    };
}
unsafe extern "C" fn choosePivot(mut lo: IdxT, mut up: IdxT,
                                 mut rnd: libc::c_uint) -> IdxT {
    let mut r4: IdxT = up.wrapping_sub(lo).wrapping_div(4i32 as libc::c_uint);
    let mut p: IdxT =
        rnd.wrapping_rem(r4.wrapping_mul(2i32 as
                                             libc::c_uint)).wrapping_add(lo.wrapping_add(r4));
    if lo.wrapping_add(r4) <= p && p <= up.wrapping_sub(r4) {
    } else {
        __assert_fail(b"lo + r4 <= p && p <= up - r4\x00" as *const u8 as
                          *const libc::c_char,
                      b"ltablib.c\x00" as *const u8 as *const libc::c_char,
                      350i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"IdxT choosePivot(IdxT, IdxT, unsigned int)\x00")).as_ptr());
    };
    return p;
}
unsafe extern "C" fn tmove(mut L: *mut lua_State_0) -> libc::c_int {
    let mut f: lua_Integer = luaL_checkinteger(L, 2i32);
    let mut e: lua_Integer = luaL_checkinteger(L, 3i32);
    let mut t: lua_Integer = luaL_checkinteger(L, 4i32);
    let mut tt: libc::c_int =
        if !(lua_type(L, 5i32) <= 0i32) { 5i32 } else { 1i32 };
    checktab(L, 1i32, 1i32);
    checktab(L, tt, 2i32);
    if e >= f {
        let mut n: lua_Integer = 0;
        let mut i: lua_Integer = 0;
        n = e - f + 1i32 as libc::c_longlong;
        if t > e || t <= f ||
               tt != 1i32 && 0 == lua_compare(L, 1i32, tt, 0i32) {
            i = 0i32 as lua_Integer;
            while i < n {
                lua_geti(L, 1i32, f + i);
                lua_seti(L, tt, t + i);
                i += 1
            }
        } else {
            i = n - 1i32 as libc::c_longlong;
            while i >= 0i32 as libc::c_longlong {
                lua_geti(L, 1i32, f + i);
                lua_seti(L, tt, t + i);
                i -= 1
            }
        }
    }
    lua_pushvalue(L, tt);
    return 1i32;
}
unsafe extern "C" fn tremove(mut L: *mut lua_State_0) -> libc::c_int {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut size: lua_Integer = luaL_len(L, 1i32);
    let mut pos: lua_Integer = luaL_optinteger(L, 2i32, size);
    pos != size;
    lua_geti(L, 1i32, pos);
    while pos < size {
        lua_geti(L, 1i32, pos + 1i32 as libc::c_longlong);
        lua_seti(L, 1i32, pos);
        pos += 1
    }
    lua_pushnil(L);
    lua_seti(L, 1i32, pos);
    return 1i32;
}
unsafe extern "C" fn unpack(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: lua_Unsigned = 0;
    let mut i: lua_Integer = luaL_optinteger(L, 2i32, 1i32 as lua_Integer);
    let mut e: lua_Integer =
        if lua_type(L, 3i32) <= 0i32 {
            luaL_len(L, 1i32)
        } else { luaL_checkinteger(L, 3i32) };
    if i > e {
        return 0i32
    } else {
        n = (e as lua_Unsigned).wrapping_sub(i as libc::c_ulonglong);
        if n >= 2147483647i32 as libc::c_uint as libc::c_ulonglong ||
               {
                   n = n.wrapping_add(1);
                   0 == lua_checkstack(L, n as libc::c_int)
               } {
            return luaL_error(L,
                              b"too many results to unpack\x00" as *const u8
                                  as *const libc::c_char)
        } else {
            while i < e { lua_geti(L, 1i32, i); i += 1 }
            lua_geti(L, 1i32, e);
            return n as libc::c_int
        }
    };
}
unsafe extern "C" fn pack(mut L: *mut lua_State_0) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = lua_gettop(L);
    lua_createtable(L, n, 1i32);
    lua_rotate(L, 1i32, 1i32);
    i = n;
    while i >= 1i32 { lua_seti(L, 1i32, i as lua_Integer); i -= 1 }
    lua_pushinteger(L, n as lua_Integer);
    lua_setfield(L, 1i32, b"n\x00" as *const u8 as *const libc::c_char);
    return 1i32;
}
unsafe extern "C" fn tinsert(mut L: *mut lua_State_0) -> libc::c_int {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut e: lua_Integer = luaL_len(L, 1i32) + 1i32 as libc::c_longlong;
    let mut pos: lua_Integer = 0;
    match lua_gettop(L) {
        2 => { pos = e }
        3 => {
            let mut i: lua_Integer = 0;
            pos = luaL_checkinteger(L, 2i32);
            i = e;
            while i > pos {
                lua_geti(L, 1i32, i - 1i32 as libc::c_longlong);
                lua_seti(L, 1i32, i);
                i -= 1
            }
        }
        _ => {
            return luaL_error(L,
                              b"wrong number of arguments to \'insert\'\x00"
                                  as *const u8 as *const libc::c_char)
        }
    }
    lua_seti(L, 1i32, pos);
    return 0i32;
}
unsafe extern "C" fn tconcat(mut L: *mut lua_State_0) -> libc::c_int {
    let mut b: luaL_Buffer =
        luaL_Buffer_0{b: 0 as *mut libc::c_char,
                      size: 0,
                      n: 0,
                      L: 0 as *mut lua_State_0,
                      initb: [0; 23],};
    checktab(L, 1i32, 1i32 | 4i32);
    let mut last: lua_Integer = luaL_len(L, 1i32);
    let mut lsep: size_t = 0;
    let mut sep: *const libc::c_char =
        luaL_optlstring(L, 2i32, b"\x00" as *const u8 as *const libc::c_char,
                        &mut lsep);
    let mut i: lua_Integer = luaL_optinteger(L, 3i32, 1i32 as lua_Integer);
    last = luaL_optinteger(L, 4i32, last);
    luaL_buffinit(L, &mut b);
    while i < last {
        addfield(L, &mut b, i);
        luaL_addlstring(&mut b, sep, lsep);
        i += 1
    }
    if i == last { addfield(L, &mut b, i); }
    luaL_pushresult(&mut b);
    return 1i32;
}
unsafe extern "C" fn addfield(mut L: *mut lua_State_0,
                              mut b: *mut luaL_Buffer, mut i: lua_Integer)
 -> () {
    lua_geti(L, 1i32, i);
    if 0 == lua_isstring(L, -1i32) {
        luaL_error(L,
                   b"invalid value (%s) at index %d in table for \'concat\'\x00"
                       as *const u8 as *const libc::c_char,
                   lua_typename(L, lua_type(L, -1i32)), i);
    }
    luaL_addvalue(b);
}
