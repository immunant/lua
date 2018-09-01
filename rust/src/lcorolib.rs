use lstate::{CallInfo, lua_State};

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
    fn lua_newthread(L: *mut lua_State_0) -> *mut lua_State_0;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State_0, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_xmove(from: *mut lua_State_0, to: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tothread(L: *mut lua_State_0, idx: libc::c_int) -> *mut lua_State_0;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State_0, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushthread(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_yieldk(
        L: *mut lua_State_0,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_resume(L: *mut lua_State_0, from: *mut lua_State_0, narg: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_status(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_isyieldable(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_error(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_concat(L: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State_0, level: libc::c_int, ar: *mut lua_Debug_0) -> libc::c_int;
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
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State_0,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State_0, arg: libc::c_int, t: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_where(L: *mut lua_State_0, lvl: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0, nup: libc::c_int) -> ();
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
/* activation record */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
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
/*
** {==============================================================
** some useful macros
** ===============================================================
*/
/* }============================================================== */
/*
** {==============================================================
** compatibility macros for unsigned conversions
** ===============================================================
*/
/* }============================================================== */
/*
** {======================================================================
** Debug API
** =======================================================================
*/
/*
** Event codes
*/
/*
** Event masks
*/
pub type lua_Debug_0 = lua_Debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type intptr_t = libc::c_long;
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
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
/* type for continuation-function contexts */
pub type lua_KContext = intptr_t;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State_0) -> libc::c_int>;
/*
** Type for continuation functions
*/
pub type lua_KFunction = Option<
    unsafe extern "C" fn(_: *mut lua_State_0, _: libc::c_int, _: lua_KContext) -> libc::c_int,
>;
pub type _IO_lock_t = ();
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
/*
** $Id: lauxlib.h,v 1.130 2016/12/04 20:17:24 roberto Exp roberto $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/
/* extra error code for 'luaL_loadfilex' */
/* key, in the registry, for table of loaded modules */
/* key, in the registry, for table of preloaded loaders */
pub type luaL_Reg_0 = luaL_Reg;
#[no_mangle]
pub unsafe extern "C" fn luaopen_coroutine(mut L: *mut lua_State_0) -> libc::c_int {
    luaL_checkversion_(
        L,
        503i32 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::std::mem::size_of::<[luaL_Reg_0; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg_0>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, co_funcs.as_ptr(), 0i32);
    return 1i32;
}
static mut co_funcs: [luaL_Reg_0; 8] = unsafe {
    [
        luaL_Reg {
            name: b"create\x00" as *const u8 as *const libc::c_char,
            func: Some(luaB_cocreate),
        },
        luaL_Reg {
            name: b"resume\x00" as *const u8 as *const libc::c_char,
            func: Some(luaB_coresume),
        },
        luaL_Reg {
            name: b"running\x00" as *const u8 as *const libc::c_char,
            func: Some(luaB_corunning),
        },
        luaL_Reg {
            name: b"status\x00" as *const u8 as *const libc::c_char,
            func: Some(luaB_costatus),
        },
        luaL_Reg {
            name: b"wrap\x00" as *const u8 as *const libc::c_char,
            func: Some(luaB_cowrap),
        },
        luaL_Reg {
            name: b"yield\x00" as *const u8 as *const libc::c_char,
            func: Some(luaB_yield),
        },
        luaL_Reg {
            name: b"isyieldable\x00" as *const u8 as *const libc::c_char,
            func: Some(luaB_yieldable),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn luaB_yieldable(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushboolean(L, lua_isyieldable(L));
    return 1i32;
}
unsafe extern "C" fn luaB_yield(mut L: *mut lua_State_0) -> libc::c_int {
    return lua_yieldk(L, lua_gettop(L), 0i32 as lua_KContext, None);
}
unsafe extern "C" fn luaB_cowrap(mut L: *mut lua_State_0) -> libc::c_int {
    luaB_cocreate(L);
    lua_pushcclosure(L, Some(luaB_auxwrap), 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_auxwrap(mut L: *mut lua_State_0) -> libc::c_int {
    let mut co: *mut lua_State_0 = lua_tothread(L, -50000i32 - 1000i32 - 1i32);
    let mut r: libc::c_int = auxresume(L, co, lua_gettop(L));
    if r < 0i32 {
        if lua_type(L, -1i32) == 4i32 {
            /* error object is a string? */
            /* add extra info */
            luaL_where(L, 1i32);
            lua_rotate(L, -2i32, 1i32);
            lua_concat(L, 2i32);
        }
        /* propagate error */
        return lua_error(L);
    } else {
        return r;
    };
}
unsafe extern "C" fn auxresume(
    mut L: *mut lua_State_0,
    mut co: *mut lua_State_0,
    mut narg: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if 0 == lua_checkstack(co, narg) {
        lua_pushstring(
            L,
            b"too many arguments to resume\x00" as *const u8 as *const libc::c_char,
        );
        /* error flag */
        return -1i32;
    } else if lua_status(co) == 0i32 && lua_gettop(co) == 0i32 {
        lua_pushstring(
            L,
            b"cannot resume dead coroutine\x00" as *const u8 as *const libc::c_char,
        );
        /* error flag */
        return -1i32;
    } else {
        lua_xmove(L, co, narg);
        status = lua_resume(co, L, narg);
        if status == 0i32 || status == 1i32 {
            let mut nres: libc::c_int = lua_gettop(co);
            if 0 == lua_checkstack(L, nres + 1i32) {
                lua_settop(co, -nres - 1i32);
                lua_pushstring(
                    L,
                    b"too many results to resume\x00" as *const u8 as *const libc::c_char,
                );
                /* remove results anyway */
                /* error flag */
                return -1i32;
            } else {
                /* move yielded values */
                lua_xmove(co, L, nres);
                return nres;
            }
        } else {
            /* move error message */
            lua_xmove(co, L, 1i32);
            /* error flag */
            return -1i32;
        }
    };
}
unsafe extern "C" fn luaB_cocreate(mut L: *mut lua_State_0) -> libc::c_int {
    let mut NL: *mut lua_State_0 = 0 as *mut lua_State_0;
    luaL_checktype(L, 1i32, 6i32);
    NL = lua_newthread(L);
    /* move function to top */
    lua_pushvalue(L, 1i32);
    /* move function from L to NL */
    lua_xmove(L, NL, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_costatus(mut L: *mut lua_State_0) -> libc::c_int {
    let mut co: *mut lua_State_0 = getco(L);
    if L == co {
        lua_pushstring(L, b"running\x00" as *const u8 as *const libc::c_char);
    } else {
        match lua_status(co) {
            1 => {
                lua_pushstring(L, b"suspended\x00" as *const u8 as *const libc::c_char);
            }
            0 => {
                let mut ar: lua_Debug_0 = lua_Debug {
                    event: 0,
                    name: 0 as *const libc::c_char,
                    namewhat: 0 as *const libc::c_char,
                    what: 0 as *const libc::c_char,
                    source: 0 as *const libc::c_char,
                    currentline: 0,
                    linedefined: 0,
                    lastlinedefined: 0,
                    nups: 0,
                    nparams: 0,
                    isvararg: 0,
                    istailcall: 0,
                    short_src: [0; 60],
                    i_ci: 0 as *mut CallInfo,
                };
                /* does it have frames? */
                if lua_getstack(co, 0i32, &mut ar) > 0i32 {
                    lua_pushstring(L, b"normal\x00" as *const u8 as *const libc::c_char);
                } else if lua_gettop(co) == 0i32 {
                    lua_pushstring(L, b"dead\x00" as *const u8 as *const libc::c_char);
                } else {
                    lua_pushstring(L, b"suspended\x00" as *const u8 as *const libc::c_char);
                }
            }
            _ => {
                /* initial state */
                lua_pushstring(L, b"dead\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    return 1i32;
}
/*
** $Id: lcorolib.c,v 1.9 2014/11/02 19:19:04 roberto Exp roberto $
** Coroutine Library
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn getco(mut L: *mut lua_State_0) -> *mut lua_State_0 {
    let mut co: *mut lua_State_0 = lua_tothread(L, 1i32);
    (!co.is_null()
        || 0 != luaL_argerror(
            L,
            1i32,
            b"thread expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    return co;
}
unsafe extern "C" fn luaB_corunning(mut L: *mut lua_State_0) -> libc::c_int {
    let mut ismain: libc::c_int = lua_pushthread(L);
    lua_pushboolean(L, ismain);
    return 2i32;
}
unsafe extern "C" fn luaB_coresume(mut L: *mut lua_State_0) -> libc::c_int {
    let mut co: *mut lua_State_0 = getco(L);
    let mut r: libc::c_int = 0;
    r = auxresume(L, co, lua_gettop(L) - 1i32);
    if r < 0i32 {
        lua_pushboolean(L, 0i32);
        lua_rotate(L, -2i32, 1i32);
        /* return false + error message */
        return 2i32;
    } else {
        lua_pushboolean(L, 1i32);
        lua_rotate(L, -(r + 1i32), 1i32);
        /* return true + 'resume' returns */
        return r + 1i32;
    };
}
