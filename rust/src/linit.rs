use lstate::lua_State;

extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    static mut l_memcontrol: Memcontrol_0;
    /*
     ** generic variable for debug tricks
     */
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
    /*
     ** generic extra include file
     */
    /*
     ** RCS ident string
     */
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn luaopen_base(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_coroutine(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_table(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_io(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_os(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_string(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_utf8(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_math(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_debug(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_package(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaL_requiref(
        L: *mut lua_State_0,
        modname: *const libc::c_char,
        openf: lua_CFunction,
        glb: libc::c_int,
    ) -> ();
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
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
/*
** $Id: lua.h,v 1.331 2016/05/30 15:53:28 roberto Exp roberto $
** Lua - A Scripting Language
** Lua.org, PUC-Rio, Brazil (http://www.lua.org)
** See Copyright Notice at the end of this file
*/
/* mark for precompiled code ('<esc>Lua') */
/* option for multiple returns in 'lua_pcall' and 'lua_call' */
/*
** Pseudo-indices
** (-LUAI_MAXSTACK is the minimum valid index; we keep some free empty
** space after that to help overflow detection)
*/
/* thread status */
pub type lua_State_0 = lua_State;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State_0) -> libc::c_int>;
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
/*
** $Id: lauxlib.h,v 1.130 2016/12/04 20:17:24 roberto Exp roberto $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/
/* extra error code for 'luaL_loadfilex' */
/* key, in the registry, for table of loaded modules */
/* key, in the registry, for table of preloaded loaders */
pub type luaL_Reg_0 = luaL_Reg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
/*
** $Id: ltests.h,v 2.49 2015/09/22 14:18:24 roberto Exp roberto $
** Internal Header for Debugging of the Lua Implementation
** See Copyright Notice in lua.h
*/
/* test Lua with no compatibility code */
/* turn on assertions */
/* to avoid warnings, and to make sure value is really unused */
/* test for sizes in 'l_sprintf' (make sure whole buffer is available) */
/* memory-allocator control variables */
pub type Memcontrol_0 = Memcontrol;
#[derive(Copy, Clone)]
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
#[no_mangle]
pub unsafe extern "C" fn luaL_openlibs(mut L: *mut lua_State_0) -> () {
    let mut lib: *const luaL_Reg_0 = 0 as *const luaL_Reg_0;
    /* "require" functions from 'loadedlibs' and set results to global table */
    lib = loadedlibs.as_ptr();
    while (*lib).func.is_some() {
        luaL_requiref(L, (*lib).name, (*lib).func, 1i32);
        lua_settop(L, -1i32 - 1i32);
        lib = lib.offset(1isize)
    }
}
/*
** $Id: linit.c,v 1.38 2015/01/05 13:48:33 roberto Exp roberto $
** Initialization of libraries for lua.c and other clients
** See Copyright Notice in lua.h
*/
/*
** If you embed Lua in your program and need to open the standard
** libraries, call luaL_openlibs in your program. If you need a
** different set of libraries, copy this file to your project and edit
** it to suit your needs.
**
** You can also *preload* libraries, so that a later 'require' can
** open the library, which is already linked to the application.
** For that, do the following code:
**
**  luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
**  lua_pushcfunction(L, luaopen_modname);
**  lua_setfield(L, -2, modname);
**  lua_pop(L, 1);  // remove PRELOAD table
*/
/*
** these libs are loaded by lua.c and are readily available to any Lua
** program
*/
static mut loadedlibs: [luaL_Reg_0; 11] = unsafe {
    [
        luaL_Reg {
            name: b"_G\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_base),
        },
        luaL_Reg {
            name: b"package\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_package),
        },
        luaL_Reg {
            name: b"coroutine\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_coroutine),
        },
        luaL_Reg {
            name: b"table\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_table),
        },
        luaL_Reg {
            name: b"io\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_io),
        },
        luaL_Reg {
            name: b"os\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_os),
        },
        luaL_Reg {
            name: b"string\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_string),
        },
        luaL_Reg {
            name: b"math\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_math),
        },
        luaL_Reg {
            name: b"utf8\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_utf8),
        },
        luaL_Reg {
            name: b"debug\x00" as *const u8 as *const libc::c_char,
            func: Some(luaopen_debug),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
