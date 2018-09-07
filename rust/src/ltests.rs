use lfunc::UpVal;
use lobject::{
    lua_TValue, CClosure, Closure, GCObject, LClosure, Node, Proto, TString, TValue, Table, Udata,
    Value,
};
use lstate::{global_State, lua_State, stringtable, CallInfo, GCUnion};

extern crate libc;
extern "C" {
    pub type _IO_FILE_plus;
    /*
     ** $Id: lstate.h,v 2.132 2016/10/19 12:31:42 roberto Exp roberto $
     ** Global State
     ** See Copyright Notice in lua.h
     */
    /*

** Some notes about garbage-collected objects: All objects in Lua must
** be kept somehow accessible until being freed, so all objects always
** belong to one (and only one) of these lists, using field 'next' of
** the 'CommonHeader' for the link:
**
** 'allgc': all objects not marked for finalization;
** 'finobj': all objects marked for finalization;
** 'tobefnz': all objects ready to be finalized;
** 'fixedgc': all objects that are not to be collected (currently
** only small strings, such as reserved words).

*/
    /* defined in ldo.c */
    pub type lua_longjmp;
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State_0, arg: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_setupvalue(
        L: *mut lua_State_0,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State_0, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_getupvalue(
        L: *mut lua_State_0,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State_0, arg: libc::c_int, t: libc::c_int) -> ();
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaL_unref(L: *mut lua_State_0, t: libc::c_int, ref_0: libc::c_int) -> ();
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State_0, idx: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State_0, n: lua_Integer) -> ();
    #[no_mangle]
    fn luaL_checklstring(
        L: *mut lua_State_0,
        arg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(
        L: *mut lua_State_0,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State_0, name: *const libc::c_char) -> ();
    #[no_mangle]
    static luai_ctype_: [lu_byte; 257];
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_tointegerx(
        L: *mut lua_State_0,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
    #[no_mangle]
    fn lua_yieldk(
        L: *mut lua_State_0,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_tothread(L: *mut lua_State_0, idx: libc::c_int) -> *mut lua_State_0;
    #[no_mangle]
    fn lua_xmove(from: *mut lua_State_0, to: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_typename(L: *mut lua_State_0, tp: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn lua_topointer(L: *mut lua_State_0, idx: libc::c_int) -> *const libc::c_void;
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State_0, n: lua_Number) -> ();
    #[no_mangle]
    fn lua_tonumberx(L: *mut lua_State_0, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Number;
    #[no_mangle]
    fn lua_tocfunction(L: *mut lua_State_0, idx: libc::c_int) -> lua_CFunction;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State_0, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_error(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaL_testudata(
        L: *mut lua_State_0,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_settable(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State_0, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_gettable(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State_0, p: *mut libc::c_void) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_sethook(
        L: *mut lua_State_0,
        func: lua_Hook,
        mask: libc::c_int,
        count: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_resume(L: *mut lua_State_0, from: *mut lua_State_0, narg: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_copy(L: *mut lua_State_0, fromidx: libc::c_int, toidx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rawsetp(L: *mut lua_State_0, idx: libc::c_int, p: *const libc::c_void) -> ();
    #[no_mangle]
    fn lua_rawgetp(L: *mut lua_State_0, idx: libc::c_int, p: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State_0,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_isuserdata(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_tolstring(
        L: *mut lua_State_0,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pcallk(
        L: *mut lua_State_0,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_rawlen(L: *mut lua_State_0, idx: libc::c_int) -> size_t;
    #[no_mangle]
    fn lua_next(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State_0, sz: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_newthread(L: *mut lua_State_0) -> *mut lua_State_0;
    #[no_mangle]
    fn luaL_newmetatable(L: *mut lua_State_0, tname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_loadstring(L: *mut lua_State_0, s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_loadfilex(
        L: *mut lua_State_0,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_len(L: *mut lua_State_0, idx: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_len(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_iscfunction(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_gsub(
        L: *mut lua_State_0,
        s: *const libc::c_char,
        p: *const libc::c_char,
        r: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State_0, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_getglobal(L: *mut lua_State_0, name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_concat(L: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_compare(
        L: *mut lua_State_0,
        idx1: libc::c_int,
        idx2: libc::c_int,
        op: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State_0, sz: libc::c_int, msg: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_callk(
        L: *mut lua_State_0,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ();
    #[no_mangle]
    fn lua_arith(L: *mut lua_State_0, op: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_absindex(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State_0, arg: libc::c_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_ref(L: *mut lua_State_0, t: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State_0, arg: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getallocf(L: *mut lua_State_0, ud: *mut *mut libc::c_void) -> lua_Alloc;
    #[no_mangle]
    fn lua_newstate(f: lua_Alloc, ud: *mut libc::c_void) -> *mut lua_State_0;
    #[no_mangle]
    fn lua_atpanic(L: *mut lua_State_0, panicf: lua_CFunction) -> lua_CFunction;
    #[no_mangle]
    fn lua_close(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State_0, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_optlstring(
        L: *mut lua_State_0,
        arg: libc::c_int,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaopen_table(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_string(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_math(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_os(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_io(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_debug(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_coroutine(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaopen_base(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaL_getsubtable(
        L: *mut lua_State_0,
        idx: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_requiref(
        L: *mut lua_State_0,
        modname: *const libc::c_char,
        openf: lua_CFunction,
        glb: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn luaopen_package(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn luaF_getlocalname(
        func: *const Proto_0,
        local_number: libc::c_int,
        pc: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    #[no_mangle]
    fn luaO_ceillog2(x: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn luaO_int2fb(x: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn luaO_fb2int(x: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaH_mainposition(t: *const Table_0, key: *const TValue) -> *mut Node;
    #[no_mangle]
    fn luaL_checkoption(
        L: *mut lua_State_0,
        arg: libc::c_int,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaC_runtilstate(L: *mut lua_State_0, statesmask: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_loadbufferx(
        L: *mut lua_State_0,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_checknumber(L: *mut lua_State_0, arg: libc::c_int) -> lua_Number;
    #[no_mangle]
    fn lua_pushlstring(
        L: *mut lua_State_0,
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State_0, l: *const luaL_Reg_0, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn lua_setallocf(L: *mut lua_State_0, f: lua_Alloc, ud: *mut libc::c_void) -> ();
    /*
     ** generic extra include file
     */
    /*
     ** RCS ident string
     */
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
    #[no_mangle]
    static luaO_nilobject_: TValue;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type ptrdiff_t = libc::c_long;
pub type intptr_t = libc::c_long;
/* 16-bit ints */
/* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = libc::c_uchar;
pub type sig_atomic_t = __sig_atomic_t;
/* Functions to be called by the debugger in specific events */
pub type lua_Hook = Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut lua_Debug) -> ()>;
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
pub type lua_Debug = lua_Debug_0;
/* activation record */
#[derive(Copy, Clone)]
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
/* type for continuation-function contexts */
pub type lua_KContext = intptr_t;
/*
** Type for continuation functions
*/
pub type lua_KFunction = Option<
    unsafe extern "C" fn(_: *mut lua_State_0, _: libc::c_int, _: lua_KContext) -> libc::c_int,
>;
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
/* internal assertions for in-house debugging */
/* to avoid problems with conditions too long */
/*
** assertion for checking API calls
*/
/* macro to avoid warnings about unused variables */
/* type casts (a macro highlights casts in the code) */
/* cast a signed lua_Integer to lua_Unsigned */
/*
** cast a lua_Unsigned to a signed lua_Integer; this cast is
** not strict ISO C, but two-complement architectures should
** work fine.
*/
/*
** non-return type
*/
/*
** maximum depth for nested C calls and syntactical nested non-terminals
** in a program. (Value must fit in an unsigned short int.)
*/
/*
** type for virtual-machine instructions;
** must be an unsigned with (at least) 4 bytes (see details in lopcodes.h)
*/
pub type Instruction = libc::c_uint;
/* macro defining a nil value */
/* raw type tag of a TValue */
/* tag with no variants (bits 0-3) */
/* type tag of a TValue (bits 0-3 for tags + variant bits 4-5) */
/* type tag of a TValue with no variants (bits 0-3) */
/* Macros to test type */
/* Macros to access values */
/* a dead value may get the 'gc' field, but cannot access its contents */
/* Macros for internal tests */
/* Macros to set values */
/*
** different types of assignments, according to destination
*/
/* from stack to (same) stack */
/* to stack (not from same stack) */
/* from table to same table */
/* to new object */
/* to table (define it as an expression to be used in macros) */
/*
** {======================================================
** types and prototypes
** =======================================================
*/
/* index to stack elements */
pub type StkId = *mut TValue;
/*
** Common Header for all collectable objects (in macro form, to be
** included in other objects)
*/
/*
** Common type has only the common header
*/
/*
** Tagged Values. This is the basic representation of values in Lua,
** an actual value plus a tag with its type.
*/
/*
** Union of all Lua values
*/
pub type Value_0 = Value;
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State_0) -> libc::c_int>;
/*
** $Id: lobject.h,v 2.116 2015/11/03 18:33:10 roberto Exp roberto $
** Type definitions for Lua objects
** See Copyright Notice in lua.h
*/
/*
** Extra tags for non-values
*/
/* function prototypes */
/* removed keys in tables */
/*
** number of all possible tags (including LUA_TNONE but excluding DEADKEY)
*/
/*
** tags for Tagged Values have the following use of bits:
** bits 0-3: actual tag (a LUA_T* value)
** bits 4-5: variant bits
** bit 6: whether value is collectable
*/
/*
** LUA_TFUNCTION variants:
** 0 - Lua function
** 1 - light C function
** 2 - regular C function (closure)
*/
/* Variant tags for functions */
/* Lua closure */
/* light C function */
/* C closure */
/* Variant tags for strings */
/* short strings */
/* long strings */
/* Variant tags for numbers */
/* float numbers */
/* integer numbers */
/* Bit mark for collectable types */
/* mark a tag as collectable */
/*
** Common type for all collectable objects
*/
pub type GCObject_0 = GCObject;
/*
** Information about a call.
** When a thread yields, 'func' is adjusted to pretend that the
** top function has only the yielded values in its stack; in that
** case, the actual 'func' value is saved in field 'extra'.
** When a function calls another with a continuation, 'extra' keeps
** the function index so that, in case of errors, the continuation
** function can be called with the correct top.
*/
pub type CallInfo_0 = CallInfo;
/*
** Lua Upvalues
*/
pub type UpVal_0 = UpVal;
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    open: unnamed_3,
    value: TValue,
}
/* (when open) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
    pub next: *mut UpVal,
    pub touched: libc::c_int,
}
/*
** $Id: llimits.h,v 1.140 2015/10/21 18:40:47 roberto Exp roberto $
** Limits, basic types, and some other 'installation-dependent' definitions
** See Copyright Notice in lua.h
*/
/*
** 'lu_mem' and 'l_mem' are unsigned/signed integers big enough to count
** the total memory used by Lua (in bytes). Usually, 'size_t' and
** 'ptrdiff_t' should work, but we use 'long' for 16-bit machines.
*/
/* { external definitions? */
/* }{ */
pub type lu_mem = size_t;
/*
** Bits in CallInfo status
*/
/* original value of 'allowhook' */
/* call is running a Lua function */
/* call is running a debug hook */
/* call is running on a fresh invocation
                                   of luaV_execute */
/* call is a yieldable protected call */
/* call was tail called */
/* last hook called yielded */
/* using __lt for __le */
/* call is running a finalizer */
/* assume that CIST_OAH has offset 0 and that 'v' is strictly 0/1 */
/*
** 'global state', shared by all threads of this state
*/
pub type global_State_0 = global_State;
/*
** Header for string value; string bytes follow the end of this structure
** (aligned according to 'UTString'; see next).
*/
pub type TString_0 = TString;

#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_4 {
    lnglen: size_t,
    hnext: *mut TString_0,
}
/* copy a value into a key without messing up field 'next' */
pub type Node_0 = Node;
/*
** Tables
*/
pub type TKey = TKey_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union TKey_0 {
    nk: unnamed_5,
    tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_5 {
    pub value_: Value,
    pub tt_: libc::c_int,
    pub next: libc::c_int,
}
/*
** Atomic type (relative to signals) to better ensure that 'lua_sethook'
** is thread safe
*/
/* extra stack space to handle TM calls and some other extras */
/* kinds of Garbage Collection */
/* gc was forced by an allocation failure */
pub type stringtable_0 = stringtable;
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void, _: size_t, _: size_t)
        -> *mut libc::c_void,
>;
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
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
pub type Upvaldesc = Upvaldesc_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc_0 {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
pub type LocVar = LocVar_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar_0 {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type LClosure_0 = LClosure;
/*
** Closures
*/
pub type CClosure_0 = CClosure;
/*
** Function Prototypes
*/
pub type Proto_0 = Proto;
pub type Table_0 = Table;
/*
** Get the actual string (array of bytes) from a 'TString'.
** (Access to 'extra' ensures that value is really a 'TString'.)
*/
/* get the actual string (array of bytes) from a Lua value */
/* get string length from 'TString *s' */
/* get string length from 'TValue *o' */
/*
** Header for userdata; memory area follows the end of this structure
** (aligned according to 'UUdata'; see next).
*/
pub type Udata_0 = Udata;
/* test for lock/unlock */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct L_EXTRA {
    pub lock: libc::c_int,
    pub plock: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aux {
    pub jb: jmp_buf,
    pub paniccode: *const libc::c_char,
    pub L: *mut lua_State_0,
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
pub type OpCode = libc::c_uint;
/*	Ax	extra (larger) argument for previous opcode	*/
pub const OP_EXTRAARG: OpCode = 46;
/*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/
pub const OP_VARARG: OpCode = 45;
/*	A Bx	R(A) := closure(KPROTO[Bx])			*/
pub const OP_CLOSURE: OpCode = 44;
/*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/
pub const OP_SETLIST: OpCode = 43;
/*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/
pub const OP_TFORLOOP: OpCode = 42;
/*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
pub const OP_TFORCALL: OpCode = 41;
/*	A sBx	R(A)-=R(A+2); pc+=sBx				*/
pub const OP_FORPREP: OpCode = 40;
/*	A sBx	R(A)+=R(A+2);
			if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
pub const OP_FORLOOP: OpCode = 39;
/*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/
pub const OP_RETURN: OpCode = 38;
/*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
pub const OP_TAILCALL: OpCode = 37;
/*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_CALL: OpCode = 36;
/*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/
pub const OP_TESTSET: OpCode = 35;
/*	A C	if not (R(A) <=> C) then pc++			*/
pub const OP_TEST: OpCode = 34;
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/*	A B C	R(A) := R(B).. ... ..R(C)			*/
pub const OP_CONCAT: OpCode = 29;
/*	A B	R(A) := length of R(B)				*/
pub const OP_LEN: OpCode = 28;
/*	A B	R(A) := not R(B)				*/
pub const OP_NOT: OpCode = 27;
/*	A B	R(A) := ~R(B)					*/
pub const OP_BNOT: OpCode = 26;
/*	A B	R(A) := -R(B)					*/
pub const OP_UNM: OpCode = 25;
/*	A B C	R(A) := RK(B) >> RK(C)				*/
pub const OP_SHR: OpCode = 24;
/*	A B C	R(A) := RK(B) << RK(C)				*/
pub const OP_SHL: OpCode = 23;
/*	A B C	R(A) := RK(B) ~ RK(C)				*/
pub const OP_BXOR: OpCode = 22;
/*	A B C	R(A) := RK(B) | RK(C)				*/
pub const OP_BOR: OpCode = 21;
/*	A B C	R(A) := RK(B) & RK(C)				*/
pub const OP_BAND: OpCode = 20;
/*	A B C	R(A) := RK(B) // RK(C)				*/
pub const OP_IDIV: OpCode = 19;
/*	A B C	R(A) := RK(B) / RK(C)				*/
pub const OP_DIV: OpCode = 18;
/*	A B C	R(A) := RK(B) ^ RK(C)				*/
pub const OP_POW: OpCode = 17;
/*	A B C	R(A) := RK(B) % RK(C)				*/
pub const OP_MOD: OpCode = 16;
/*	A B C	R(A) := RK(B) * RK(C)				*/
pub const OP_MUL: OpCode = 15;
/*	A B C	R(A) := RK(B) - RK(C)				*/
pub const OP_SUB: OpCode = 14;
/*	A B C	R(A) := RK(B) + RK(C)				*/
pub const OP_ADD: OpCode = 13;
/*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/
pub const OP_SELF: OpCode = 12;
/*	A B C	R(A) := {} (size = B,C)				*/
pub const OP_NEWTABLE: OpCode = 11;
/*	A B C	R(A)[RK(B)] := RK(C)				*/
pub const OP_SETTABLE: OpCode = 10;
/*	A B	UpValue[B] := R(A)				*/
pub const OP_SETUPVAL: OpCode = 9;
/*	A B C	UpValue[A][RK(B)] := RK(C)			*/
pub const OP_SETTABUP: OpCode = 8;
/*	A B C	R(A) := R(B)[RK(C)]				*/
pub const OP_GETTABLE: OpCode = 7;
/*	A B C	R(A) := UpValue[B][RK(C)]			*/
pub const OP_GETTABUP: OpCode = 6;
/*	A B	R(A) := UpValue[B]				*/
pub const OP_GETUPVAL: OpCode = 5;
/*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
pub const OP_LOADNIL: OpCode = 4;
/*	A B C	R(A) := (Bool)B; if (C) pc++			*/
pub const OP_LOADBOOL: OpCode = 3;
/*	A 	R(A) := Kst(extra arg)				*/
pub const OP_LOADKX: OpCode = 2;
/*	A Bx	R(A) := Kst(Bx)					*/
pub const OP_LOADK: OpCode = 1;
/*----------------------------------------------------------------------
name		args	description
------------------------------------------------------------------------*/
/*	A B	R(A) := R(B)					*/
pub const OP_MOVE: OpCode = 0;
/* basic instruction format */
pub const iAx: OpMode = 3;
pub const iAsBx: OpMode = 2;
pub const iABx: OpMode = 1;
pub const iABC: OpMode = 0;
/*
** $Id: lopcodes.h,v 1.148 2014/10/25 11:50:46 roberto Exp roberto $
** Opcodes for Lua virtual machine
** See Copyright Notice in lua.h
*/
/*===========================================================================
  We assume that instructions are unsigned numbers.
  All instructions have an opcode in the first 6 bits.
  Instructions can have the following fields:
	'A' : 8 bits
	'B' : 9 bits
	'C' : 9 bits
	'Ax' : 26 bits ('A', 'B', and 'C' together)
	'Bx' : 18 bits ('B' and 'C' together)
	'sBx' : signed Bx

  A signed argument is represented in excess K; that is, the number
  value is the unsigned value minus K. K is exactly the maximum value
  for that argument (so that -max is represented by 0, and +max is
  represented by 2*max), which is half the maximum for the corresponding
  unsigned argument.
===========================================================================*/
pub type OpMode = libc::c_uint;
/*
** {======================================================================
** Controlled version for realloc.
** =======================================================================
*/
/* 01010101 (a nice pattern) */
pub type Header = Header_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Header_0 {
    a: L_Umaxalign,
    d: unnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_6 {
    pub size: size_t,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
#[no_mangle]
pub static mut l_memcontrol: Memcontrol_0 = unsafe {
    Memcontrol {
        numblocks: 0i64 as libc::c_ulong,
        total: 0i64 as libc::c_ulong,
        maxmem: 0i64 as libc::c_ulong,
        memlimit: 0i64 as libc::c_ulong,
        objcount: [
            0i64 as libc::c_ulong,
            0i64 as libc::c_ulong,
            0i64 as libc::c_ulong,
            0i64 as libc::c_ulong,
            0i64 as libc::c_ulong,
            0i64 as libc::c_ulong,
            0i64 as libc::c_ulong,
            0i64 as libc::c_ulong,
            0i64 as libc::c_ulong,
        ],
    }
};
/*
** generic variable for debug tricks
*/
#[no_mangle]
pub static mut l_Trick: *mut libc::c_void =
    unsafe { 0 as *const libc::c_void as *mut libc::c_void };
/*
** Function to traverse and check all memory used by Lua
*/
#[no_mangle]
pub unsafe extern "C" fn lua_checkmemory(mut L: *mut lua_State_0) -> libc::c_int {
    let mut g: *mut global_State = (*L).l_G;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    let mut maybedead: libc::c_int = 0;
    if (*g).gcstate as libc::c_int <= 1i32 {
        if 0 == (*(*g).mainthread).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
        } else {
            __assert_fail(
                b"!(((g->mainthread)->marked) & (((1<<(0)) | (1<<(1)))))\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                418i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_checkmemory(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                419i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_checkmemory(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        if 0 == (*(*g).l_registry.value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
        } else {
            __assert_fail(b"!(((((((((&g->l_registry)->tt_) & (1 << 6))) ? (void) (0) : __assert_fail (\"(((&g->l_registry)->tt_) & (1 << 6))\", \"ltests.c\", 419, __extension__ __PRETTY_FUNCTION__)), (((&g->l_registry)->value_).gc)))->marked) & (((1<<(0)) | (1<<(1)))))\x00"
                              as *const u8 as *const libc::c_char,
                          b"ltests.c\x00" as *const u8 as *const libc::c_char,
                          419i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"int lua_checkmemory(lua_State *)\x00")).as_ptr());
        };
    }
    if 0 != (*g).l_registry.tt_ & 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((&g->l_registry)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            421i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"int lua_checkmemory(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    if 0 != ((*(*g).l_registry.value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
    {
    } else {
        __assert_fail(b"!(!((((((((((&g->l_registry)->tt_) & (1 << 6))) ? (void) (0) : __assert_fail (\"(((&g->l_registry)->tt_) & (1 << 6))\", \"ltests.c\", 421, __extension__ __PRETTY_FUNCTION__)), (((&g->l_registry)->value_).gc)))->marked) ^ ((1<<(0)) | (1<<(1)))) & (((g)->currentwhite ^ ((1<<(0)) | (1<<(1)))))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ltests.c\x00" as *const u8 as *const libc::c_char,
                      421i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int lua_checkmemory(lua_State *)\x00")).as_ptr());
    };
    checkstack(g, (*g).mainthread);
    (*(*g).mainthread).marked = ((*(*g).mainthread).marked as libc::c_int
        & !(1i32 << 7i32) as lu_byte as libc::c_int) as lu_byte;
    if (*g).sweepgc.is_null()
        || 2i32 <= (*g).gcstate as libc::c_int && (*g).gcstate as libc::c_int <= 5i32
    {
    } else {
        __assert_fail(
            b"g->sweepgc == ((void*)0) || (2 <= (g)->gcstate && (g)->gcstate <= 5)\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            424i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"int lua_checkmemory(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    markgrays(g);
    /* check 'fixedgc' list */
    o = (*g).fixedgc;
    while !o.is_null() {
        if (*o).tt as libc::c_int == 4i32 | 0i32 << 4i32
            && 0 == (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32)
        {
        } else {
            __assert_fail(b"o->tt == (4 | (0 << 4)) && (!(((o)->marked) & (((1<<(0)) | (1<<(1))) | (1<<(2)))))\x00"
                              as *const u8 as *const libc::c_char,
                          b"ltests.c\x00" as *const u8 as *const libc::c_char,
                          428i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"int lua_checkmemory(lua_State *)\x00")).as_ptr());
        };
        o = (*o).next
    }
    /* check 'allgc' list */
    checkgray(g, (*g).allgc);
    maybedead =
        (1i32 < (*g).gcstate as libc::c_int && (*g).gcstate as libc::c_int <= 2i32) as libc::c_int;
    o = (*g).allgc;
    while !o.is_null() {
        checkobject(g, o, maybedead);
        if 0 == (*o).marked as libc::c_int & 1i32 << 3i32 {
        } else {
            __assert_fail(
                b"!(((o)->marked) & ((1<<(3))))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                435i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_checkmemory(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        o = (*o).next
    }
    /* check 'finobj' list */
    checkgray(g, (*g).finobj);
    o = (*g).finobj;
    while !o.is_null() {
        checkobject(g, o, 0i32);
        if 0 != (*o).marked as libc::c_int & 1i32 << 3i32 {
        } else {
            __assert_fail(
                b"(((o)->marked) & ((1<<(3))))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                441i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_checkmemory(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        if (*o).tt as libc::c_int == 7i32 || (*o).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"o->tt == 7 || o->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                442i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_checkmemory(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        o = (*o).next
    }
    /* check 'tobefnz' list */
    checkgray(g, (*g).tobefnz);
    o = (*g).tobefnz;
    while !o.is_null() {
        checkobject(g, o, 0i32);
        if 0 != (*o).marked as libc::c_int & 1i32 << 3i32 {
        } else {
            __assert_fail(
                b"(((o)->marked) & ((1<<(3))))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                448i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_checkmemory(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        if (*o).tt as libc::c_int == 7i32 || (*o).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"o->tt == 7 || o->tt == 5\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                449i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"int lua_checkmemory(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        o = (*o).next
    }
    return 0i32;
}
/* entire stack must have valid values */
unsafe extern "C" fn checkobject(
    mut g: *mut global_State,
    mut o: *mut GCObject,
    mut maybedead: libc::c_int,
) -> () {
    if 0 == ((*o).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
    {
        if 0 != maybedead {
        } else {
            __assert_fail(
                b"maybedead\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                325i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"void checkobject(global_State *, GCObject *, int)\x00",
                )).as_ptr(),
            );
        };
    } else {
        if (*g).gcstate as libc::c_int != 7i32
            || 0 != (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
        {
        } else {
            __assert_fail(
                b"g->gcstate != 7 || (((o)->marked) & (((1<<(0)) | (1<<(1)))))\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                327i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"void checkobject(global_State *, GCObject *, int)\x00",
                )).as_ptr(),
            );
        };
        match (*o).tt as libc::c_int {
            7 => {
                let mut uservalue: TValue = lua_TValue {
                    value_: Value_0 {
                        gc: 0 as *mut GCObject,
                    },
                    tt_: 0,
                };
                if (*o).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        331i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
                let mut mt: *mut Table_0 = (*&mut (*(o as *mut GCUnion)).u).metatable;
                if !mt.is_null() {
                    if (*mt).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
                    } else {
                        __assert_fail(
                            b"(((mt)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                            b"ltests.c\x00" as *const u8 as *const libc::c_char,
                            332i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                b"void checkobject(global_State *, GCObject *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    if 0 != testobjref(g, o, &mut (*(mt as *mut GCUnion)).gc) {
                    } else {
                        if 0 != 0i32 {
                        } else {
                            __assert_fail(
                                b"0\x00" as *const u8 as *const libc::c_char,
                                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                                332i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                    b"void checkobject(global_State *, GCObject *, int)\x00",
                                )).as_ptr(),
                            );
                        };
                    };
                }
                let mut io: *mut TValue = &mut uservalue;
                if (*o).tt as libc::c_int == 7i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 7\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        333i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
                let mut iu: *const Udata_0 = &mut (*(o as *mut GCUnion)).u;
                (*io).value_ = (*iu).user_;
                (*io).tt_ = (*iu).ttuv_ as libc::c_int;
                if 0 == (*io).tt_ & 1i32 << 6i32 || {
                    if 0 != (*io).tt_ & 1i32 << 6i32 {
                    } else {
                        __assert_fail(
                            b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                            b"ltests.c\x00" as *const u8 as *const libc::c_char,
                            333i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                b"void checkobject(global_State *, GCObject *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                    (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
                        && ((*g).mainthread.is_null() || {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(
                                    b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                        as *const libc::c_char,
                                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                                    333i32 as libc::c_uint,
                                    (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                        b"void checkobject(global_State *, GCObject *, int)\x00",
                                    )).as_ptr(),
                                );
                            };
                            0 != ((*(*io).value_.gc).marked as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                                & ((*(*(*g).mainthread).l_G).currentwhite as libc::c_int
                                    ^ (1i32 << 0i32 | 1i32 << 1i32))
                        })
                } {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(
                            b"0\x00" as *const u8 as *const libc::c_char,
                            b"ltests.c\x00" as *const u8 as *const libc::c_char,
                            333i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                                b"void checkobject(global_State *, GCObject *, int)\x00",
                            )).as_ptr(),
                        );
                    };
                };
                checkvalref(g, o, &mut uservalue);
            }
            5 => {
                if (*o).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        338i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
                checktable(g, &mut (*(o as *mut GCUnion)).h);
            }
            8 => {
                if (*o).tt as libc::c_int == 8i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 8\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        342i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
                checkstack(g, &mut (*(o as *mut GCUnion)).th);
            }
            6 => {
                if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        346i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
                checkLclosure(g, &mut (*(o as *mut GCUnion)).cl.l);
            }
            38 => {
                if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        350i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
                checkCclosure(g, &mut (*(o as *mut GCUnion)).cl.c);
            }
            9 => {
                if (*o).tt as libc::c_int == 9i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        354i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
                checkproto(g, &mut (*(o as *mut GCUnion)).p);
            }
            4 | 20 => {
                if 0 != (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32) {
                } else {
                    __assert_fail(
                        b"!(!(((o)->marked) & (((1<<(0)) | (1<<(1))) | (1<<(2)))))\x00" as *const u8
                            as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        359i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
            }
            _ => {
                /* strings are never gray */
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        362i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                            b"void checkobject(global_State *, GCObject *, int)\x00",
                        )).as_ptr(),
                    );
                };
            }
        }
    };
}
/*
** All marks are conditional because a GC may happen while the
** prototype is still being created
*/
unsafe extern "C" fn checkproto(mut g: *mut global_State, mut f: *mut Proto_0) -> () {
    let mut i: libc::c_int = 0;
    if (*f).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((f)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            250i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"void checkproto(global_State *, Proto *)\x00",
            )).as_ptr(),
        );
    };
    let mut fgc: *mut GCObject = &mut (*(f as *mut GCUnion)).gc;
    if !(*f).cache.is_null() {
        if (*(*f).cache).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((f->cache)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                251i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void checkproto(global_State *, Proto *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != testobjref(g, fgc, &mut (*((*f).cache as *mut GCUnion)).gc) {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    251i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checkproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
        };
    }
    if !(*f).source.is_null() {
        if (*(*f).source).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((f->source)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                252i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void checkproto(global_State *, Proto *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != testobjref(g, fgc, &mut (*((*f).source as *mut GCUnion)).gc) {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    252i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checkproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
        };
    }
    i = 0i32;
    while i < (*f).sizek {
        if (*(*f).k.offset(i as isize)).tt_ & 0xfi32 == 4i32 {
            if (*(*f).k.offset(i as isize)).tt_ & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((((f->k + i))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                        as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    255i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checkproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            if (*(*(*f).k.offset(i as isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
            } else {
                __assert_fail(
                    b"(((((f->k + i)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                        as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    255i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checkproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            if !(&mut (*((*(*f).k.offset(i as isize)).value_.gc as *mut GCUnion)).ts
                as *mut TString_0)
                .is_null()
            {
                if (*(*f).k.offset(i as isize)).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((f->k + i))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        255i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                            b"void checkproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*(*f).k.offset(i as isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((f->k + i)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        255i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                            b"void checkproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(&mut (*((*(*f).k.offset(i as isize)).value_.gc as *mut GCUnion)).ts
                    as *mut TString_0))
                    .tt as libc::c_int
                    & 0xfi32
                    < 9i32 + 1i32
                {
                } else {
                    __assert_fail(b"(((((((((((((f->k + i))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((f->k + i))->tt_)) & 0x0F)) == (4))\", \"ltests.c\", 255, __extension__ __PRETTY_FUNCTION__)), (((((((((f->k + i)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((f->k + i)->value_).gc)->tt) & 0x0F) == 4\", \"ltests.c\", 255, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((f->k + i)->value_).gc))))->ts))))))->tt) & 0x0F) < (9+1)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"ltests.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  255i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 41],
                                                            &[libc::c_char; 41]>(b"void checkproto(global_State *, Proto *)\x00")).as_ptr());
                };
                if (*(*f).k.offset(i as isize)).tt_ & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((((f->k + i))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                            as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        255i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                            b"void checkproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
                if (*(*(*f).k.offset(i as isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
                } else {
                    __assert_fail(
                        b"(((((f->k + i)->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                            as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        255i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                            b"void checkproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
                if 0 != testobjref(
                    g,
                    fgc,
                    &mut (*(&mut (*((*(*f).k.offset(i as isize)).value_.gc as *mut GCUnion)).ts
                        as *mut TString_0 as *mut GCUnion))
                        .gc,
                ) {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(
                            b"0\x00" as *const u8 as *const libc::c_char,
                            b"ltests.c\x00" as *const u8 as *const libc::c_char,
                            255i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                                b"void checkproto(global_State *, Proto *)\x00",
                            )).as_ptr(),
                        );
                    };
                };
            }
        }
        i += 1
    }
    i = 0i32;
    while i < (*f).sizeupvalues {
        if !(*(*f).upvalues.offset(i as isize)).name.is_null() {
            if (*(*(*f).upvalues.offset(i as isize)).name).tt as libc::c_int & 0xfi32 < 9i32 + 1i32
            {
            } else {
                __assert_fail(
                    b"(((f->upvalues[i].name)->tt) & 0x0F) < (9+1)\x00" as *const u8
                        as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    258i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checkproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            if 0 != testobjref(
                g,
                fgc,
                &mut (*((*(*f).upvalues.offset(i as isize)).name as *mut GCUnion)).gc,
            ) {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        258i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                            b"void checkproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
            };
        }
        i += 1
    }
    i = 0i32;
    while i < (*f).sizep {
        if !(*(*f).p.offset(i as isize)).is_null() {
            if (**(*f).p.offset(i as isize)).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((f->p[i])->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    260i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checkproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            if 0 != testobjref(
                g,
                fgc,
                &mut (*(*(*f).p.offset(i as isize) as *mut GCUnion)).gc,
            ) {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        260i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                            b"void checkproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
            };
        }
        i += 1
    }
    i = 0i32;
    while i < (*f).sizelocvars {
        if !(*(*f).locvars.offset(i as isize)).varname.is_null() {
            if (*(*(*f).locvars.offset(i as isize)).varname).tt as libc::c_int & 0xfi32
                < 9i32 + 1i32
            {
            } else {
                __assert_fail(
                    b"(((f->locvars[i].varname)->tt) & 0x0F) < (9+1)\x00" as *const u8
                        as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    262i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checkproto(global_State *, Proto *)\x00",
                    )).as_ptr(),
                );
            };
            if 0 != testobjref(
                g,
                fgc,
                &mut (*((*(*f).locvars.offset(i as isize)).varname as *mut GCUnion)).gc,
            ) {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        262i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                            b"void checkproto(global_State *, Proto *)\x00",
                        )).as_ptr(),
                    );
                };
            };
        }
        i += 1
    }
}
unsafe extern "C" fn testobjref(
    mut g: *mut global_State,
    mut f: *mut GCObject,
    mut t: *mut GCObject,
) -> libc::c_int {
    let mut r1: libc::c_int = testobjref1(g, f, t);
    if 0 == r1 {
        printf(
            b"%d(%02X) - \x00" as *const u8 as *const libc::c_char,
            (*g).gcstate as libc::c_int,
            (*g).currentwhite as libc::c_int,
        );
        printobj(g, f);
        printf(b"  ->  \x00" as *const u8 as *const libc::c_char);
        printobj(g, t);
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    return r1;
}
/* }====================================================================== */
/*
** {======================================================
** Functions to check memory consistency
** =======================================================
*/
unsafe extern "C" fn testobjref1(
    mut g: *mut global_State,
    mut f: *mut GCObject,
    mut t: *mut GCObject,
) -> libc::c_int {
    if 0 == ((*t).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
    {
        return 0i32;
    } else if !(2i32 <= (*g).gcstate as libc::c_int && (*g).gcstate as libc::c_int <= 5i32) {
        return !(0 != (*f).marked as libc::c_int & 1i32 << 2i32
            && 0 != (*t).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32))
            as libc::c_int;
    } else {
        return 1i32;
    };
}
unsafe extern "C" fn printobj(mut g: *mut global_State, mut o: *mut GCObject) -> () {
    printf(
        b"||%s(%p)-%c(%02X)||\x00" as *const u8 as *const libc::c_char,
        luaT_typenames_[(((*o).tt as libc::c_int & 0xfi32) + 1i32) as usize],
        o as *mut libc::c_void,
        if 0 == ((*o).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        {
            'd' as i32
        } else if 0 != (*o).marked as libc::c_int & 1i32 << 2i32 {
            'b' as i32
        } else if 0 != (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
            'w' as i32
        } else {
            'g' as i32
        },
        (*o).marked as libc::c_int,
    );
}
unsafe extern "C" fn checkCclosure(mut g: *mut global_State, mut cl: *mut CClosure) -> () {
    if (*cl).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((cl)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            268i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"void checkCclosure(global_State *, CClosure *)\x00",
            )).as_ptr(),
        );
    };
    let mut clgc: *mut GCObject = &mut (*(cl as *mut GCUnion)).gc;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        checkvalref(g, clgc, &mut *(*cl).upvalue.as_mut_ptr().offset(i as isize));
        i += 1
    }
}
unsafe extern "C" fn checkvalref(
    mut g: *mut global_State,
    mut f: *mut GCObject,
    mut t: *const TValue,
) -> () {
    if 0 == (*t).tt_ & 1i32 << 6i32 || {
        if 0 != (*t).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((t)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                223i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                    b"void checkvalref(global_State *, GCObject *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        (*t).tt_ & 0x3fi32 == (*(*t).value_.gc).tt as libc::c_int && {
            if 0 != (*t).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(
                    b"(((t)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    223i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                        b"void checkvalref(global_State *, GCObject *, const TValue *)\x00",
                    )).as_ptr(),
                );
            };
            0 != testobjref(g, f, (*t).value_.gc)
        }
    } {
    } else {
        __assert_fail(b"!(((t)->tt_) & (1 << 6)) || (((((t)->tt_) & 0x3F) == ((((((t)->tt_) & (1 << 6))) ? (void) (0) : __assert_fail (\"(((t)->tt_) & (1 << 6))\", \"ltests.c\", 223, __extension__ __PRETTY_FUNCTION__)), (((t)->value_).gc))->tt) && testobjref(g, f, ((((((t)->tt_) & (1 << 6))) ? (void) (0) : __assert_fail (\"(((t)->tt_) & (1 << 6))\", \"ltests.c\", 223, __extension__ __PRETTY_FUNCTION__)), (((t)->value_).gc))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ltests.c\x00" as *const u8 as *const libc::c_char,
                      223i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 61],
                                                &[libc::c_char; 61]>(b"void checkvalref(global_State *, GCObject *, const TValue *)\x00")).as_ptr());
    };
}
unsafe extern "C" fn checkLclosure(mut g: *mut global_State, mut cl: *mut LClosure_0) -> () {
    if (*cl).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((cl)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            276i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"void checkLclosure(global_State *, LClosure *)\x00",
            )).as_ptr(),
        );
    };
    let mut clgc: *mut GCObject = &mut (*(cl as *mut GCUnion)).gc;
    let mut i: libc::c_int = 0;
    if !(*cl).p.is_null() {
        if (*(*cl).p).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((cl->p)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                278i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"void checkLclosure(global_State *, LClosure *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != testobjref(g, clgc, &mut (*((*cl).p as *mut GCUnion)).gc) {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    278i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"void checkLclosure(global_State *, LClosure *)\x00",
                    )).as_ptr(),
                );
            };
        };
    }
    i = 0i32;
    while i < (*cl).nupvalues as libc::c_int {
        let mut uv: *mut UpVal = *(*cl).upvals.as_mut_ptr().offset(i as isize);
        if !uv.is_null() {
            /* only closed upvalues matter to invariant */
            if !((*uv).v != &mut (*uv).u.value as *mut TValue) {
                checkvalref(g, clgc, (*uv).v);
            }
            if (*uv).refcount > 0i32 as libc::c_ulong {
            } else {
                __assert_fail(
                    b"uv->refcount > 0\x00" as *const u8 as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    284i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"void checkLclosure(global_State *, LClosure *)\x00",
                    )).as_ptr(),
                );
            };
        }
        i += 1
    }
}
unsafe extern "C" fn checkstack(mut g: *mut global_State, mut L1: *mut lua_State_0) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let mut ci: *mut CallInfo_0 = 0 as *mut CallInfo_0;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    if 0 != ((*L1).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
        & ((*g).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
    {
    } else {
        __assert_fail(b"!(!((((L1)->marked) ^ ((1<<(0)) | (1<<(1)))) & (((g)->currentwhite ^ ((1<<(0)) | (1<<(1)))))))\x00"
                          as *const u8 as *const libc::c_char,
                      b"ltests.c\x00" as *const u8 as *const libc::c_char,
                      308i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void checkstack(global_State *, lua_State *)\x00")).as_ptr());
    };
    uv = (*L1).openupval;
    while !uv.is_null() {
        if (*uv).v != &mut (*uv).u.value as *mut TValue {
        } else {
            __assert_fail(
                b"((uv)->v != &(uv)->u.value)\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                310i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void checkstack(global_State *, lua_State *)\x00",
                )).as_ptr(),
            );
        };
        uv = (*uv).u.open.next
    }
    /* must be open */
    ci = (*L1).ci;
    while !ci.is_null() {
        if (*ci).top <= (*L1).stack_last {
        } else {
            __assert_fail(
                b"ci->top <= L1->stack_last\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                312i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void checkstack(global_State *, lua_State *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != lua_checkpc(L1, ci) {
        } else {
            __assert_fail(
                b"lua_checkpc(L1, ci)\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                313i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void checkstack(global_State *, lua_State *)\x00",
                )).as_ptr(),
            );
        };
        ci = (*ci).previous
    }
    if !(*L1).stack.is_null() {
        /* complete thread? */
        o = (*L1).stack;
        while o < (*L1).stack_last.offset(5isize) {
            if 0 == (*o).tt_ & 1i32 << 6i32 || {
                if 0 != (*o).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        317i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void checkstack(global_State *, lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                (*o).tt_ & 0x3fi32 == (*(*o).value_.gc).tt as libc::c_int
                    && (L1.is_null() || {
                        if 0 != (*o).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                                317i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                                    b"void checkstack(global_State *, lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        0 != ((*(*o).value_.gc).marked as libc::c_int
                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                            & ((*(*L1).l_G).currentwhite as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                    })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        317i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void checkstack(global_State *, lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            o = o.offset(1isize)
        }
    } else {
        if (*L1).stacksize == 0i32 {
        } else {
            __assert_fail(
                b"L1->stacksize == 0\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                319i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void checkstack(global_State *, lua_State *)\x00",
                )).as_ptr(),
            );
        };
    };
}
unsafe extern "C" fn lua_checkpc(mut L: *mut lua_State_0, mut ci: *mut CallInfo_0) -> libc::c_int {
    if 0 == (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
        return 1i32;
    } else {
        /* if function yielded (inside a hook), real 'func' is in 'extra' field */
        let mut f: StkId = if (*L).status as libc::c_int != 1i32 || ci != (*L).ci {
            (*ci).func
        } else {
            ((*L).stack as *mut libc::c_char).offset((*ci).extra as isize) as *mut TValue
        };
        if (*f).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((f))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                297i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"int lua_checkpc(lua_State *, CallInfo *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*f).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(
                b"(((f)->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                297i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"int lua_checkpc(lua_State *, CallInfo *)\x00",
                )).as_ptr(),
            );
        };
        let mut p: *mut Proto_0 = (*&mut (*((*f).value_.gc as *mut GCUnion)).cl.l).p;
        return ((*p).code <= (*ci).u.l.savedpc as *mut Instruction
            && (*ci).u.l.savedpc <= (*p).code.offset((*p).sizecode as isize))
            as libc::c_int;
    };
}
unsafe extern "C" fn checktable(mut g: *mut global_State, mut h: *mut Table_0) -> () {
    let mut i: libc::c_uint = 0;
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *(*h)
        .node
        .offset((1i32 << (*h).lsizenode as libc::c_int) as isize)
        as *mut Node;
    if (*h).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
    } else {
        __assert_fail(
            b"(((h)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            230i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"void checktable(global_State *, Table *)\x00",
            )).as_ptr(),
        );
    };
    let mut hgc: *mut GCObject = &mut (*(h as *mut GCUnion)).gc;
    if !(*h).metatable.is_null() {
        if (*(*h).metatable).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
        } else {
            __assert_fail(
                b"(((h->metatable)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                231i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void checktable(global_State *, Table *)\x00",
                )).as_ptr(),
            );
        };
        if 0 != testobjref(g, hgc, &mut (*((*h).metatable as *mut GCUnion)).gc) {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(
                    b"0\x00" as *const u8 as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    231i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checktable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
        };
    }
    i = 0i32 as libc::c_uint;
    while i < (*h).sizearray {
        checkvalref(g, hgc, &mut *(*h).array.offset(i as isize));
        i = i.wrapping_add(1)
    }
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        if !((*n).i_val.tt_ == 0i32) {
            if !((*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 0i32) {
            } else {
                __assert_fail(
                    b"!((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == (0))\x00" as *const u8
                        as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    236i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"void checktable(global_State *, Table *)\x00",
                    )).as_ptr(),
                );
            };
            checkvalref(g, hgc, &mut (*n).i_key.tvk as *mut TValue as *const TValue);
            checkvalref(g, hgc, &mut (*n).i_val);
        }
        n = n.offset(1isize)
    }
}
unsafe extern "C" fn checkgray(mut g: *mut global_State, mut o: *mut GCObject) -> () {
    while !o.is_null() {
        if 0 == (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32) {
            if !((*g).gcstate as libc::c_int <= 1i32)
                || 0 != (*o).marked as libc::c_int & 1i32 << 7i32
            {
            } else {
                __assert_fail(
                    b"!((g)->gcstate <= 1) || ((o->marked) & ((1<<(7))))\x00" as *const u8
                        as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    405i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"void checkgray(global_State *, GCObject *)\x00",
                    )).as_ptr(),
                );
            };
            (*o).marked =
                ((*o).marked as libc::c_int & !(1i32 << 7i32) as lu_byte as libc::c_int) as lu_byte
        }
        if 0 == (*o).marked as libc::c_int & 1i32 << 7i32 {
        } else {
            __assert_fail(
                b"!((o->marked) & ((1<<(7))))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                408i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                    b"void checkgray(global_State *, GCObject *)\x00",
                )).as_ptr(),
            );
        };
        o = (*o).next
    }
}
/*
** mark all objects in gray lists with the TESTGRAYBIT, so that
** 'checkmemory' can check that all gray objects are in a gray list
*/
unsafe extern "C" fn markgrays(mut g: *mut global_State) -> () {
    if !((*g).gcstate as libc::c_int <= 1i32) {
        return;
    } else {
        checkgraylist(g, (*g).gray);
        checkgraylist(g, (*g).grayagain);
        checkgraylist(g, (*g).weak);
        checkgraylist(g, (*g).ephemeron);
        checkgraylist(g, (*g).allweak);
        return;
    };
}
unsafe extern "C" fn checkgraylist(mut g: *mut global_State, mut o: *mut GCObject) -> () {
    /* better to keep it available if we need to print an object */
    while !o.is_null() {
        if 0 == (*o).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32) {
        } else {
            __assert_fail(
                b"(!(((o)->marked) & (((1<<(0)) | (1<<(1))) | (1<<(2)))))\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                373i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"void checkgraylist(global_State *, GCObject *)\x00",
                )).as_ptr(),
            );
        };
        if 0 == (*o).marked as libc::c_int & 1i32 << 7i32 {
        } else {
            __assert_fail(
                b"!((o->marked) & ((1<<(7))))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                374i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                    b"void checkgraylist(global_State *, GCObject *)\x00",
                )).as_ptr(),
            );
        };
        (*o).marked = ((*o).marked as libc::c_int | 1i32 << 7i32) as lu_byte;
        match (*o).tt as libc::c_int {
            5 => {
                if (*o).tt as libc::c_int == 5i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 5\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        377i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                            b"void checkgraylist(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                o = (*&mut (*(o as *mut GCUnion)).h).gclist
            }
            6 => {
                if (*o).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (0 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        378i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                            b"void checkgraylist(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                o = (*&mut (*(o as *mut GCUnion)).cl.l).gclist
            }
            38 => {
                if (*o).tt as libc::c_int == 6i32 | 2i32 << 4i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == (6 | (2 << 4))\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        379i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                            b"void checkgraylist(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                o = (*&mut (*(o as *mut GCUnion)).cl.c).gclist
            }
            8 => {
                if (*o).tt as libc::c_int == 8i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 8\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        380i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                            b"void checkgraylist(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                o = (*&mut (*(o as *mut GCUnion)).th).gclist
            }
            9 => {
                if (*o).tt as libc::c_int == 9i32 {
                } else {
                    __assert_fail(
                        b"(o)->tt == 9\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        381i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                            b"void checkgraylist(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
                o = (*&mut (*(o as *mut GCUnion)).p).gclist
            }
            _ => {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        382i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                            b"void checkgraylist(global_State *, GCObject *)\x00",
                        )).as_ptr(),
                    );
                };
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaB_opentests(mut L: *mut lua_State_0) -> libc::c_int {
    let mut ud: *mut libc::c_void = 0 as *mut libc::c_void;
    lua_atpanic(L, Some(tpanic));
    atexit(Some(checkfinalmem));
    if lua_getallocf(L, &mut ud) == Some(debug_realloc) {
    } else {
        __assert_fail(
            b"lua_getallocf(L, &ud) == debug_realloc\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            1563i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"int luaB_opentests(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    if ud == &mut l_memcontrol as *mut Memcontrol_0 as *mut libc::c_void {
    } else {
        __assert_fail(
            b"ud == ((void *)(&l_memcontrol))\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            1564i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"int luaB_opentests(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    lua_setallocf(L, lua_getallocf(L, 0 as *mut *mut libc::c_void), ud);
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
        (::std::mem::size_of::<[luaL_Reg; 36]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, tests_funcs.as_ptr(), 0i32);
    return 1i32;
}
/* }====================================================== */
static mut tests_funcs: [luaL_Reg; 36] = unsafe {
    [
        luaL_Reg {
            name: b"checkmemory\x00" as *const u8 as *const libc::c_char,
            func: Some(lua_checkmemory),
        },
        luaL_Reg {
            name: b"closestate\x00" as *const u8 as *const libc::c_char,
            func: Some(closestate),
        },
        luaL_Reg {
            name: b"d2s\x00" as *const u8 as *const libc::c_char,
            func: Some(d2s),
        },
        luaL_Reg {
            name: b"doonnewstack\x00" as *const u8 as *const libc::c_char,
            func: Some(doonnewstack),
        },
        luaL_Reg {
            name: b"doremote\x00" as *const u8 as *const libc::c_char,
            func: Some(doremote),
        },
        luaL_Reg {
            name: b"gccolor\x00" as *const u8 as *const libc::c_char,
            func: Some(gc_color),
        },
        luaL_Reg {
            name: b"gcstate\x00" as *const u8 as *const libc::c_char,
            func: Some(gc_state),
        },
        luaL_Reg {
            name: b"getref\x00" as *const u8 as *const libc::c_char,
            func: Some(getref),
        },
        luaL_Reg {
            name: b"hash\x00" as *const u8 as *const libc::c_char,
            func: Some(hash_query),
        },
        luaL_Reg {
            name: b"int2fb\x00" as *const u8 as *const libc::c_char,
            func: Some(int2fb_aux),
        },
        luaL_Reg {
            name: b"log2\x00" as *const u8 as *const libc::c_char,
            func: Some(log2_aux),
        },
        luaL_Reg {
            name: b"limits\x00" as *const u8 as *const libc::c_char,
            func: Some(get_limits),
        },
        luaL_Reg {
            name: b"listcode\x00" as *const u8 as *const libc::c_char,
            func: Some(listcode),
        },
        luaL_Reg {
            name: b"listk\x00" as *const u8 as *const libc::c_char,
            func: Some(listk),
        },
        luaL_Reg {
            name: b"listlocals\x00" as *const u8 as *const libc::c_char,
            func: Some(listlocals),
        },
        luaL_Reg {
            name: b"loadlib\x00" as *const u8 as *const libc::c_char,
            func: Some(loadlib),
        },
        luaL_Reg {
            name: b"checkpanic\x00" as *const u8 as *const libc::c_char,
            func: Some(checkpanic),
        },
        luaL_Reg {
            name: b"newstate\x00" as *const u8 as *const libc::c_char,
            func: Some(newstate),
        },
        luaL_Reg {
            name: b"newuserdata\x00" as *const u8 as *const libc::c_char,
            func: Some(newuserdata),
        },
        luaL_Reg {
            name: b"num2int\x00" as *const u8 as *const libc::c_char,
            func: Some(num2int),
        },
        luaL_Reg {
            name: b"pushuserdata\x00" as *const u8 as *const libc::c_char,
            func: Some(pushuserdata),
        },
        luaL_Reg {
            name: b"querystr\x00" as *const u8 as *const libc::c_char,
            func: Some(string_query),
        },
        luaL_Reg {
            name: b"querytab\x00" as *const u8 as *const libc::c_char,
            func: Some(table_query),
        },
        luaL_Reg {
            name: b"ref\x00" as *const u8 as *const libc::c_char,
            func: Some(tref),
        },
        luaL_Reg {
            name: b"resume\x00" as *const u8 as *const libc::c_char,
            func: Some(coresume),
        },
        luaL_Reg {
            name: b"s2d\x00" as *const u8 as *const libc::c_char,
            func: Some(s2d),
        },
        luaL_Reg {
            name: b"sethook\x00" as *const u8 as *const libc::c_char,
            func: Some(sethook),
        },
        luaL_Reg {
            name: b"stacklevel\x00" as *const u8 as *const libc::c_char,
            func: Some(stacklevel),
        },
        luaL_Reg {
            name: b"testC\x00" as *const u8 as *const libc::c_char,
            func: Some(testC),
        },
        luaL_Reg {
            name: b"makeCfunc\x00" as *const u8 as *const libc::c_char,
            func: Some(makeCfunc),
        },
        luaL_Reg {
            name: b"totalmem\x00" as *const u8 as *const libc::c_char,
            func: Some(mem_query),
        },
        luaL_Reg {
            name: b"trick\x00" as *const u8 as *const libc::c_char,
            func: Some(settrick),
        },
        luaL_Reg {
            name: b"udataval\x00" as *const u8 as *const libc::c_char,
            func: Some(udataval),
        },
        luaL_Reg {
            name: b"unref\x00" as *const u8 as *const libc::c_char,
            func: Some(unref),
        },
        luaL_Reg {
            name: b"upvalue\x00" as *const u8 as *const libc::c_char,
            func: Some(upvalue),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn upvalue(mut L: *mut lua_State_0) -> libc::c_int {
    let mut n: libc::c_int = luaL_checkinteger(L, 2i32) as libc::c_int;
    luaL_checktype(L, 1i32, 6i32);
    if lua_type(L, 3i32) == -1i32 {
        let mut name: *const libc::c_char = lua_getupvalue(L, 1i32, n);
        if name.is_null() {
            return 0i32;
        } else {
            lua_pushstring(L, name);
            return 2i32;
        }
    } else {
        let mut name_0: *const libc::c_char = lua_setupvalue(L, 1i32, n);
        lua_pushstring(L, name_0);
        return 1i32;
    };
}
unsafe extern "C" fn unref(mut L: *mut lua_State_0) -> libc::c_int {
    let mut level: libc::c_int = lua_gettop(L);
    luaL_unref(
        L,
        -50000i32 - 1000i32,
        luaL_checkinteger(L, 1i32) as libc::c_int,
    );
    if lua_gettop(L) == level {
    } else {
        __assert_fail(
            b"lua_gettop(L) == level\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            758i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int unref(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    return 0i32;
}
unsafe extern "C" fn udataval(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushinteger(L, lua_touserdata(L, 1i32) as libc::c_long as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn settrick(mut L: *mut lua_State_0) -> libc::c_int {
    if (*(*(*L).ci).func.offset(1isize)).tt_ == 0i32 {
        l_Trick = 0 as *mut libc::c_void
    } else {
        if 0 != (*(*(*L).ci).func.offset(1isize)).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"((((L->ci->func + (1)))->tt_) & (1 << 6))\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                614i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"int settrick(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        l_Trick = (*(*(*L).ci).func.offset(1isize)).value_.gc as *mut libc::c_void
    }
    return 0i32;
}
unsafe extern "C" fn mem_query(mut L: *mut lua_State_0) -> libc::c_int {
    if lua_type(L, 1i32) == -1i32 {
        lua_pushinteger(L, l_memcontrol.total as lua_Integer);
        lua_pushinteger(L, l_memcontrol.numblocks as lua_Integer);
        lua_pushinteger(L, l_memcontrol.maxmem as lua_Integer);
        return 3i32;
    } else if 0 != lua_isnumber(L, 1i32) {
        let mut limit: libc::c_ulong = luaL_checkinteger(L, 1i32) as libc::c_ulong;
        if limit == 0i32 as libc::c_ulong {
            limit = (9223372036854775807i64 as libc::c_ulong)
                .wrapping_mul(2u64)
                .wrapping_add(1u64)
        }
        l_memcontrol.memlimit = limit;
        return 0i32;
    } else {
        let mut t: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
        let mut i: libc::c_int = 0;
        i = 9i32 - 1i32;
        while i >= 0i32 {
            if strcmp(t, luaT_typenames_[(i + 1i32) as usize]) == 0i32 {
                lua_pushinteger(L, l_memcontrol.objcount[i as usize] as lua_Integer);
                return 1i32;
            } else {
                i -= 1
            }
        }
        return luaL_error(
            L,
            b"unkown type \'%s\'\x00" as *const u8 as *const libc::c_char,
            t,
        );
    };
}
unsafe extern "C" fn makeCfunc(mut L: *mut lua_State_0) -> libc::c_int {
    luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_pushcclosure(L, Some(Cfunc), lua_gettop(L));
    return 1i32;
}
unsafe extern "C" fn Cfunc(mut L: *mut lua_State_0) -> libc::c_int {
    return runC(
        L,
        L,
        lua_tolstring(L, -50000i32 - 1000i32 - 1i32, 0 as *mut size_t),
    );
}
unsafe extern "C" fn runC(
    mut L: *mut lua_State_0,
    mut L1: *mut lua_State_0,
    mut pc: *const libc::c_char,
) -> libc::c_int {
    let mut opt: *const libc::c_char = 0 as *const libc::c_char;
    let mut t_3: libc::c_int = 0;
    let mut a_0: libc::c_int = 0;
    let mut narg_0: libc::c_int = 0;
    let mut nres_2: libc::c_int = 0;
    let mut t_2: libc::c_int = 0;
    let mut s_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut narg_2: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut t_1: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut t_0: libc::c_int = 0;
    let mut i_5: libc::c_int = 0;
    let mut nres_0: libc::c_int = 0;
    let mut t_4: libc::c_int = 0;
    let mut nres_1: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut i_1: libc::c_int = 0;
    let mut nres: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut op: libc::c_int = 0;
    let mut op_0: libc::c_int = 0;
    let mut narg: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut i_4: libc::c_int = 0;
    let mut i_2: libc::c_int = 0;
    let mut i_0: libc::c_int = 0;
    let mut narg_1: libc::c_int = 0;
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: libc::c_int = 0;
    let mut b_0: libc::c_int = 0;
    let mut func: lua_CFunction = None;
    let mut buff: [libc::c_char; 300] = [0; 300];
    let mut status: libc::c_int = 0i32;
    if pc.is_null() {
        return luaL_error(
            L,
            b"attempt to runC null script\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        loop {
            let mut inst: *const libc::c_char = getstring_aux(L, buff.as_mut_ptr(), &mut pc);
            if strcmp(b"\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                return 0i32;
            } else if strcmp(b"absindex\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushnumber(
                    L1,
                    lua_absindex(L1, getindex_aux(L, L1, &mut pc)) as lua_Number,
                );
            } else if strcmp(b"append\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                t = getindex_aux(L, L1, &mut pc);
                i = lua_rawlen(L1, t) as libc::c_int;
                lua_rawseti(L1, t, (i + 1i32) as lua_Integer);
            } else if strcmp(b"arith\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                op = 0;
                skip(&mut pc);
                let fresh0 = pc;
                pc = pc.offset(1);
                op = strchr(ops.as_ptr(), *fresh0 as libc::c_int).wrapping_offset_from(ops.as_ptr())
                    as libc::c_long as libc::c_int;
                lua_arith(L1, op);
            } else if strcmp(b"call\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                narg = getnum_aux(L, L1, &mut pc);
                nres = getnum_aux(L, L1, &mut pc);
                lua_callk(L1, narg, nres, 0i32 as lua_KContext, None);
            } else if strcmp(b"callk\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                narg_0 = getnum_aux(L, L1, &mut pc);
                nres_0 = getnum_aux(L, L1, &mut pc);
                i_0 = getindex_aux(L, L1, &mut pc);
                lua_callk(L1, narg_0, nres_0, i_0 as lua_KContext, Some(Cfunck));
            } else if strcmp(b"checkstack\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                let mut sz: libc::c_int = getnum_aux(L, L1, &mut pc);
                let mut msg: *const libc::c_char = getstring_aux(L, buff.as_mut_ptr(), &mut pc);
                if *msg as libc::c_int == '\u{0}' as i32 {
                    /* to test 'luaL_checkstack' with no message */
                    msg = 0 as *const libc::c_char
                }
                luaL_checkstack(L1, sz, msg);
            } else if strcmp(b"compare\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                /* EQ, LT, or LE */
                opt = getstring_aux(L, buff.as_mut_ptr(), &mut pc);
                op_0 = if *opt.offset(0isize) as libc::c_int == 'E' as i32 {
                    0i32
                } else if *opt.offset(1isize) as libc::c_int == 'T' as i32 {
                    1i32
                } else {
                    2i32
                };
                a = getindex_aux(L, L1, &mut pc);
                b = getindex_aux(L, L1, &mut pc);
                lua_pushboolean(L1, lua_compare(L1, a, b, op_0));
            } else if strcmp(b"concat\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_concat(L1, getnum_aux(L, L1, &mut pc));
            } else if strcmp(b"copy\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                f = getindex_aux(L, L1, &mut pc);
                lua_copy(L1, f, getindex_aux(L, L1, &mut pc));
            } else if strcmp(b"func2num\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                func = lua_tocfunction(L1, getindex_aux(L, L1, &mut pc));
                lua_pushnumber(
                    L1,
                    ::std::mem::transmute::<lua_CFunction, size_t>(func) as lua_Number,
                );
            } else if strcmp(b"getfield\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                t_0 = getindex_aux(L, L1, &mut pc);
                lua_getfield(L1, t_0, getstring_aux(L, buff.as_mut_ptr(), &mut pc));
            } else if strcmp(b"getglobal\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_getglobal(L1, getstring_aux(L, buff.as_mut_ptr(), &mut pc));
            } else if strcmp(
                b"getmetatable\x00" as *const u8 as *const libc::c_char,
                inst,
            ) == 0i32
            {
                if !(lua_getmetatable(L1, getindex_aux(L, L1, &mut pc)) == 0i32) {
                    continue;
                }
                lua_pushnil(L1);
            } else if strcmp(b"gettable\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_gettable(L1, getindex_aux(L, L1, &mut pc));
            } else if strcmp(b"gettop\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushinteger(L1, lua_gettop(L1) as lua_Integer);
            } else if strcmp(b"gsub\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                a_0 = getnum_aux(L, L1, &mut pc);
                b_0 = getnum_aux(L, L1, &mut pc);
                c = getnum_aux(L, L1, &mut pc);
                luaL_gsub(
                    L1,
                    lua_tolstring(L1, a_0, 0 as *mut size_t),
                    lua_tolstring(L1, b_0, 0 as *mut size_t),
                    lua_tolstring(L1, c, 0 as *mut size_t),
                );
            } else if strcmp(b"insert\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_rotate(L1, getnum_aux(L, L1, &mut pc), 1i32);
            } else if strcmp(b"iscfunction\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(L1, lua_iscfunction(L1, getindex_aux(L, L1, &mut pc)));
            } else if strcmp(b"isfunction\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(
                    L1,
                    (lua_type(L1, getindex_aux(L, L1, &mut pc)) == 6i32) as libc::c_int,
                );
            } else if strcmp(b"isnil\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(
                    L1,
                    (lua_type(L1, getindex_aux(L, L1, &mut pc)) == 0i32) as libc::c_int,
                );
            } else if strcmp(b"isnull\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(
                    L1,
                    (lua_type(L1, getindex_aux(L, L1, &mut pc)) == -1i32) as libc::c_int,
                );
            } else if strcmp(b"isnumber\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(L1, lua_isnumber(L1, getindex_aux(L, L1, &mut pc)));
            } else if strcmp(b"isstring\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(L1, lua_isstring(L1, getindex_aux(L, L1, &mut pc)));
            } else if strcmp(b"istable\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(
                    L1,
                    (lua_type(L1, getindex_aux(L, L1, &mut pc)) == 5i32) as libc::c_int,
                );
            } else if strcmp(b"isudataval\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(
                    L1,
                    (lua_type(L1, getindex_aux(L, L1, &mut pc)) == 2i32) as libc::c_int,
                );
            } else if strcmp(b"isuserdata\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(L1, lua_isuserdata(L1, getindex_aux(L, L1, &mut pc)));
            } else if strcmp(b"len\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_len(L1, getindex_aux(L, L1, &mut pc));
            } else if strcmp(b"Llen\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushinteger(L1, luaL_len(L1, getindex_aux(L, L1, &mut pc)));
            } else if strcmp(b"loadfile\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                luaL_loadfilex(
                    L1,
                    luaL_checklstring(L1, getnum_aux(L, L1, &mut pc), 0 as *mut size_t),
                    0 as *const libc::c_char,
                );
            } else if strcmp(b"loadstring\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                s = luaL_checklstring(L1, getnum_aux(L, L1, &mut pc), 0 as *mut size_t);
                luaL_loadstring(L1, s);
            } else if strcmp(
                b"newmetatable\x00" as *const u8 as *const libc::c_char,
                inst,
            ) == 0i32
            {
                lua_pushboolean(
                    L1,
                    luaL_newmetatable(L1, getstring_aux(L, buff.as_mut_ptr(), &mut pc)),
                );
            } else if strcmp(b"newtable\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_createtable(L1, 0i32, 0i32);
            } else if strcmp(b"newthread\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_newthread(L1);
            } else if strcmp(b"newuserdata\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_newuserdata(L1, getnum_aux(L, L1, &mut pc) as size_t);
            } else if strcmp(b"next\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_next(L1, -2i32);
            } else if strcmp(b"objsize\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushinteger(
                    L1,
                    lua_rawlen(L1, getindex_aux(L, L1, &mut pc)) as lua_Integer,
                );
            } else if strcmp(b"pcall\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                narg_1 = getnum_aux(L, L1, &mut pc);
                nres_1 = getnum_aux(L, L1, &mut pc);
                status = lua_pcallk(
                    L1,
                    narg_1,
                    nres_1,
                    getnum_aux(L, L1, &mut pc),
                    0i32 as lua_KContext,
                    None,
                )
            } else if strcmp(b"pcallk\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                narg_2 = getnum_aux(L, L1, &mut pc);
                nres_2 = getnum_aux(L, L1, &mut pc);
                i_1 = getindex_aux(L, L1, &mut pc);
                status = lua_pcallk(L1, narg_2, nres_2, 0i32, i_1 as lua_KContext, Some(Cfunck))
            } else if strcmp(b"pop\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_settop(L1, -getnum_aux(L, L1, &mut pc) - 1i32);
            } else if strcmp(b"print\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                let mut n: libc::c_int = getnum_aux(L, L1, &mut pc);
                if n != 0i32 {
                    printf(
                        b"%s\n\x00" as *const u8 as *const libc::c_char,
                        luaL_tolstring(L1, n, 0 as *mut size_t),
                    );
                    lua_settop(L1, -1i32 - 1i32);
                } else {
                    printstack(L1);
                }
            } else if strcmp(b"pushbool\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(L1, getnum_aux(L, L1, &mut pc));
            } else if strcmp(
                b"pushcclosure\x00" as *const u8 as *const libc::c_char,
                inst,
            ) == 0i32
            {
                lua_pushcclosure(L1, Some(testC), getnum_aux(L, L1, &mut pc));
            } else if strcmp(b"pushint\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushinteger(L1, getnum_aux(L, L1, &mut pc) as lua_Integer);
            } else if strcmp(b"pushnil\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushnil(L1);
            } else if strcmp(b"pushnum\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushnumber(L1, getnum_aux(L, L1, &mut pc) as lua_Number);
            } else if strcmp(b"pushstatus\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                pushcode(L1, status);
            } else if strcmp(b"pushstring\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushstring(L1, getstring_aux(L, buff.as_mut_ptr(), &mut pc));
            } else if strcmp(
                b"pushupvalueindex\x00" as *const u8 as *const libc::c_char,
                inst,
            ) == 0i32
            {
                lua_pushinteger(
                    L1,
                    (-50000i32 - 1000i32 - getnum_aux(L, L1, &mut pc)) as lua_Integer,
                );
            } else if strcmp(b"pushvalue\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushvalue(L1, getindex_aux(L, L1, &mut pc));
            } else if strcmp(b"rawgeti\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                t_1 = getindex_aux(L, L1, &mut pc);
                lua_rawgeti(L1, t_1, getnum_aux(L, L1, &mut pc) as lua_Integer);
            } else if strcmp(b"rawgetp\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                t_2 = getindex_aux(L, L1, &mut pc);
                lua_rawgetp(
                    L1,
                    t_2,
                    getnum_aux(L, L1, &mut pc) as size_t as *mut libc::c_void,
                );
            } else if strcmp(b"rawsetp\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                t_3 = getindex_aux(L, L1, &mut pc);
                lua_rawsetp(
                    L1,
                    t_3,
                    getnum_aux(L, L1, &mut pc) as size_t as *mut libc::c_void,
                );
            } else if strcmp(b"remove\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_rotate(L1, getnum_aux(L, L1, &mut pc), -1i32);
                lua_settop(L1, -1i32 - 1i32);
            } else if strcmp(b"replace\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_copy(L1, -1i32, getindex_aux(L, L1, &mut pc));
                lua_settop(L1, -1i32 - 1i32);
            } else if strcmp(b"resume\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                i_2 = getindex_aux(L, L1, &mut pc);
                status = lua_resume(lua_tothread(L1, i_2), L, getnum_aux(L, L1, &mut pc))
            } else if strcmp(b"return\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                let mut n_0: libc::c_int = getnum_aux(L, L1, &mut pc);
                if L1 != L {
                    let mut i_3: libc::c_int = 0;
                    i_3 = 0i32;
                    while i_3 < n_0 {
                        lua_pushstring(L, lua_tolstring(L1, -(n_0 - i_3), 0 as *mut size_t));
                        i_3 += 1
                    }
                }
                return n_0;
            } else if strcmp(b"rotate\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                i_4 = getindex_aux(L, L1, &mut pc);
                lua_rotate(L1, i_4, getnum_aux(L, L1, &mut pc));
            } else if strcmp(b"setfield\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                t_4 = getindex_aux(L, L1, &mut pc);
                lua_setfield(L1, t_4, getstring_aux(L, buff.as_mut_ptr(), &mut pc));
            } else if strcmp(b"setglobal\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_setglobal(L1, getstring_aux(L, buff.as_mut_ptr(), &mut pc));
            } else if strcmp(b"sethook\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                mask = getnum_aux(L, L1, &mut pc);
                count = getnum_aux(L, L1, &mut pc);
                sethookaux(
                    L1,
                    mask,
                    count,
                    getstring_aux(L, buff.as_mut_ptr(), &mut pc),
                );
            } else if strcmp(
                b"setmetatable\x00" as *const u8 as *const libc::c_char,
                inst,
            ) == 0i32
            {
                lua_setmetatable(L1, getindex_aux(L, L1, &mut pc));
            } else if strcmp(b"settable\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_settable(L1, getindex_aux(L, L1, &mut pc));
            } else if strcmp(b"settop\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_settop(L1, getnum_aux(L, L1, &mut pc));
            } else if strcmp(b"testudata\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                i_5 = getindex_aux(L, L1, &mut pc);
                lua_pushboolean(
                    L1,
                    (luaL_testudata(L1, i_5, getstring_aux(L, buff.as_mut_ptr(), &mut pc))
                        != 0 as *mut libc::c_void) as libc::c_int,
                );
            } else if strcmp(b"error\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_error(L1);
            } else if strcmp(b"throw\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                luaL_error(L1, b"C++\x00" as *const u8 as *const libc::c_char);
                break;
            } else if strcmp(b"tobool\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushboolean(L1, lua_toboolean(L1, getindex_aux(L, L1, &mut pc)));
            } else if strcmp(b"tocfunction\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushcclosure(L1, lua_tocfunction(L1, getindex_aux(L, L1, &mut pc)), 0i32);
            } else if strcmp(b"tointeger\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushinteger(
                    L1,
                    lua_tointegerx(L1, getindex_aux(L, L1, &mut pc), 0 as *mut libc::c_int),
                );
            } else if strcmp(b"tonumber\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushnumber(
                    L1,
                    lua_tonumberx(L1, getindex_aux(L, L1, &mut pc), 0 as *mut libc::c_int),
                );
            } else if strcmp(b"topointer\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushnumber(
                    L1,
                    lua_topointer(L1, getindex_aux(L, L1, &mut pc)) as size_t as lua_Number,
                );
            } else if strcmp(b"tostring\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                s_0 = lua_tolstring(L1, getindex_aux(L, L1, &mut pc), 0 as *mut size_t);
                s1 = lua_pushstring(L1, s_0);
                if s_0.is_null() && s1.is_null() || strcmp(s_0, s1) == 0i32 {
                } else {
                    if 0 != 0i32 {
                    } else {
                        __assert_fail(
                            b"0\x00" as *const u8 as *const libc::c_char,
                            b"ltests.c\x00" as *const u8 as *const libc::c_char,
                            1359i32 as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 49], &[libc::c_char; 49]>(
                                b"int runC(lua_State *, lua_State *, const char *)\x00",
                            )).as_ptr(),
                        );
                    };
                };
            } else if strcmp(b"type\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                lua_pushstring(
                    L1,
                    lua_typename(L1, lua_type(L1, getnum_aux(L, L1, &mut pc))),
                );
            } else if strcmp(b"xmove\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                let mut f_0: libc::c_int = getindex_aux(L, L1, &mut pc);
                let mut t_5: libc::c_int = getindex_aux(L, L1, &mut pc);
                let mut fs: *mut lua_State_0 = if f_0 == 0i32 {
                    L1
                } else {
                    lua_tothread(L1, f_0)
                };
                let mut ts: *mut lua_State_0 = if t_5 == 0i32 {
                    L1
                } else {
                    lua_tothread(L1, t_5)
                };
                let mut n_1: libc::c_int = getnum_aux(L, L1, &mut pc);
                if n_1 == 0i32 {
                    n_1 = lua_gettop(fs)
                }
                lua_xmove(fs, ts, n_1);
            } else if strcmp(b"yield\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                return lua_yieldk(L1, getnum_aux(L, L1, &mut pc), 0i32 as lua_KContext, None);
            } else if strcmp(b"yieldk\x00" as *const u8 as *const libc::c_char, inst) == 0i32 {
                let mut nres_3: libc::c_int = getnum_aux(L, L1, &mut pc);
                let mut i_6: libc::c_int = getindex_aux(L, L1, &mut pc);
                return lua_yieldk(L1, nres_3, i_6 as lua_KContext, Some(Cfunck));
            } else {
                luaL_error(
                    L,
                    b"unknown instruction %s\x00" as *const u8 as *const libc::c_char,
                    buff.as_mut_ptr(),
                );
            }
        }
        return 0i32;
    };
}
unsafe extern "C" fn Cfunck(
    mut L: *mut lua_State_0,
    mut status: libc::c_int,
    mut ctx: lua_KContext,
) -> libc::c_int {
    pushcode(L, status);
    lua_setglobal(L, b"status\x00" as *const u8 as *const libc::c_char);
    lua_pushinteger(L, ctx as lua_Integer);
    lua_setglobal(L, b"ctx\x00" as *const u8 as *const libc::c_char);
    return runC(L, L, lua_tolstring(L, ctx as libc::c_int, 0 as *mut size_t));
}
unsafe extern "C" fn pushcode(mut L: *mut lua_State_0, mut code: libc::c_int) -> () {
    static mut codes: [*const libc::c_char; 7] = unsafe {
        [
            b"OK\x00" as *const u8 as *const libc::c_char,
            b"YIELD\x00" as *const u8 as *const libc::c_char,
            b"ERRRUN\x00" as *const u8 as *const libc::c_char,
            b"ERRSYNTAX\x00" as *const u8 as *const libc::c_char,
            b"ERRMEM\x00" as *const u8 as *const libc::c_char,
            b"ERRGCMM\x00" as *const u8 as *const libc::c_char,
            b"ERRERR\x00" as *const u8 as *const libc::c_char,
        ]
    };
    lua_pushstring(L, codes[code as usize]);
}
unsafe extern "C" fn getindex_aux(
    mut L: *mut lua_State_0,
    mut L1: *mut lua_State_0,
    mut pc: *mut *const libc::c_char,
) -> libc::c_int {
    skip(pc);
    let fresh1 = *pc;
    *pc = (*pc).offset(1);
    match *fresh1 as libc::c_int {
        82 => return -50000i32 - 1000i32,
        71 => {
            return luaL_error(
                L,
                b"deprecated index \'G\'\x00" as *const u8 as *const libc::c_char,
            )
        }
        85 => return -50000i32 - 1000i32 - getnum_aux(L, L1, pc),
        _ => {
            *pc = (*pc).offset(-1isize);
            return getnum_aux(L, L1, pc);
        }
    };
}
unsafe extern "C" fn getnum_aux(
    mut L: *mut lua_State_0,
    mut L1: *mut lua_State_0,
    mut pc: *mut *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0i32;
    let mut sig: libc::c_int = 1i32;
    skip(pc);
    if **pc as libc::c_int == '.' as i32 {
        res = lua_tointegerx(L1, -1i32, 0 as *mut libc::c_int) as libc::c_int;
        lua_settop(L1, -1i32 - 1i32);
        *pc = (*pc).offset(1isize);
        return res;
    } else if **pc as libc::c_int == '*' as i32 {
        res = lua_gettop(L1);
        *pc = (*pc).offset(1isize);
        return res;
    } else {
        if **pc as libc::c_int == '-' as i32 {
            sig = -1i32;
            *pc = (*pc).offset(1isize)
        }
        if 0 == luai_ctype_[(**pc as libc::c_uchar as libc::c_int + 1i32) as usize] as libc::c_int
            & 1i32 << 1i32
        {
            luaL_error(
                L,
                b"number expected (%s)\x00" as *const u8 as *const libc::c_char,
                *pc,
            );
        }
        while 0
            != luai_ctype_[(**pc as libc::c_uchar as libc::c_int + 1i32) as usize] as libc::c_int
                & 1i32 << 1i32
        {
            let fresh2 = *pc;
            *pc = (*pc).offset(1);
            res = res * 10i32 + *fresh2 as libc::c_int - '0' as i32
        }
        return sig * res;
    };
}
unsafe extern "C" fn skip(mut pc: *mut *const libc::c_char) -> () {
    loop {
        if **pc as libc::c_int != '\u{0}' as i32 && !strchr(delimits, **pc as libc::c_int).is_null()
        {
            *pc = (*pc).offset(1isize)
        } else {
            if !(**pc as libc::c_int == '#' as i32) {
                break;
            }
            while **pc as libc::c_int != '\n' as i32 && **pc as libc::c_int != '\u{0}' as i32 {
                *pc = (*pc).offset(1isize)
            }
        }
    }
}
static mut delimits: *const libc::c_char =
    unsafe { b" \t\n,;\x00" as *const u8 as *const libc::c_char };
unsafe extern "C" fn getstring_aux(
    mut L: *mut lua_State_0,
    mut buff: *mut libc::c_char,
    mut pc: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0i32;
    skip(pc);
    if **pc as libc::c_int == '\"' as i32 || **pc as libc::c_int == '\'' as i32 {
        /* quoted string? */
        let fresh3 = *pc;
        *pc = (*pc).offset(1);
        let mut quote: libc::c_int = *fresh3 as libc::c_int;
        while **pc as libc::c_int != quote {
            if **pc as libc::c_int == '\u{0}' as i32 {
                luaL_error(
                    L,
                    b"unfinished string in C script\x00" as *const u8 as *const libc::c_char,
                );
            }
            let fresh5 = i;
            i = i + 1;
            let fresh4 = *pc;
            *pc = (*pc).offset(1);
            *buff.offset(fresh5 as isize) = *fresh4
        }
        *pc = (*pc).offset(1isize)
    } else {
        while **pc as libc::c_int != '\u{0}' as i32
            && strchr(delimits, **pc as libc::c_int).is_null()
        {
            let fresh7 = i;
            i = i + 1;
            let fresh6 = *pc;
            *pc = (*pc).offset(1);
            *buff.offset(fresh7 as isize) = *fresh6
        }
    }
    *buff.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    return buff;
}
/*
** {====================================================================
** function to test the API with C. It interprets a kind of assembler
** language with calls to the API, so the test can be driven by Lua code
** =====================================================================
*/
unsafe extern "C" fn sethookaux(
    mut L: *mut lua_State_0,
    mut mask: libc::c_int,
    mut count: libc::c_int,
    mut scpt: *const libc::c_char,
) -> () {
    if *scpt as libc::c_int == '\u{0}' as i32 {
        /* no script? */
        /* turn off hooks */
        lua_sethook(L, None, 0i32, 0i32);
        return;
    } else {
        /* get C_HOOK table */
        lua_getfield(
            L,
            -50000i32 - 1000i32,
            b"C_HOOK\x00" as *const u8 as *const libc::c_char,
        );
        if !(lua_type(L, -1i32) == 5i32) {
            /* no hook table? */
            lua_settop(L, -1i32 - 1i32);
            lua_createtable(L, 0i32, 0i32);
            /* remove previous value */
            /* create new C_HOOK table */
            lua_pushvalue(L, -1i32);
            /* register it */
            lua_setfield(
                L,
                -50000i32 - 1000i32,
                b"C_HOOK\x00" as *const u8 as *const libc::c_char,
            );
        }
        lua_pushlightuserdata(L, L as *mut libc::c_void);
        lua_pushstring(L, scpt);
        /* C_HOOK[L] = script */
        lua_settable(L, -3i32);
        lua_sethook(L, Some(Chook), mask, count);
        return;
    };
}
/* }====================================================== */
/*
** {======================================================
** tests for C hooks
** =======================================================
*/
/*
** C hook that runs the C script stored in registry.C_HOOK[L]
*/
unsafe extern "C" fn Chook(mut L: *mut lua_State_0, mut ar: *mut lua_Debug) -> () {
    let mut scpt: *const libc::c_char = 0 as *const libc::c_char;
    let events: [*const libc::c_char; 5] = [
        b"call\x00" as *const u8 as *const libc::c_char,
        b"ret\x00" as *const u8 as *const libc::c_char,
        b"line\x00" as *const u8 as *const libc::c_char,
        b"count\x00" as *const u8 as *const libc::c_char,
        b"tailcall\x00" as *const u8 as *const libc::c_char,
    ];
    lua_getfield(
        L,
        -50000i32 - 1000i32,
        b"C_HOOK\x00" as *const u8 as *const libc::c_char,
    );
    lua_pushlightuserdata(L, L as *mut libc::c_void);
    /* get C_HOOK[L] (script saved by sethookaux) */
    lua_gettable(L, -2i32);
    /* not very religious (string will be popped) */
    scpt = lua_tolstring(L, -1i32, 0 as *mut size_t);
    lua_settop(L, -2i32 - 1i32);
    /* remove C_HOOK and script */
    /* may be used by script */
    lua_pushstring(L, events[(*ar).event as usize]);
    /* may be used by script */
    lua_pushinteger(L, (*ar).currentline as lua_Integer);
    /* run script from C_HOOK[L] */
    runC(L, L, scpt);
}
unsafe extern "C" fn testC(mut L: *mut lua_State_0) -> libc::c_int {
    let mut L1: *mut lua_State_0 = 0 as *mut lua_State_0;
    let mut pc: *const libc::c_char = 0 as *const libc::c_char;
    if 0 != lua_isuserdata(L, 1i32) {
        L1 = getstate(L);
        pc = luaL_checklstring(L, 2i32, 0 as *mut size_t)
    } else if lua_type(L, 1i32) == 8i32 {
        L1 = lua_tothread(L, 1i32);
        pc = luaL_checklstring(L, 2i32, 0 as *mut size_t)
    } else {
        L1 = L;
        pc = luaL_checklstring(L, 1i32, 0 as *mut size_t)
    }
    return runC(L, L1, pc);
}
unsafe extern "C" fn getstate(mut L: *mut lua_State_0) -> *mut lua_State_0 {
    let mut L1: *mut lua_State_0 = lua_touserdata(L, 1i32) as *mut lua_State_0;
    (!L1.is_null() || 0 != luaL_argerror(
        L,
        1i32,
        b"state expected\x00" as *const u8 as *const libc::c_char,
    )) as libc::c_int;
    return L1;
}
/* }====================================================== */
unsafe extern "C" fn printstack(mut L: *mut lua_State_0) -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = lua_gettop(L);
    i = 1i32;
    while i <= n {
        printf(
            b"%3d: %s\n\x00" as *const u8 as *const libc::c_char,
            i,
            luaL_tolstring(L, i, 0 as *mut size_t),
        );
        lua_settop(L, -1i32 - 1i32);
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
** arithmetic operation encoding for 'arith' instruction
** LUA_OPIDIV  -> \
** LUA_OPSHL   -> <
** LUA_OPSHR   -> >
** LUA_OPUNM   -> _
** LUA_OPBNOT  -> !
*/
static mut ops: [libc::c_char; 15] =
    unsafe { [43, 45, 42, 37, 94, 47, 92, 38, 124, 126, 60, 62, 95, 33, 0] };
unsafe extern "C" fn stacklevel(mut L: *mut lua_State_0) -> libc::c_int {
    let mut a: libc::c_ulong = 0i32 as libc::c_ulong;
    lua_pushinteger(
        L,
        (*L).top.wrapping_offset_from((*L).stack) as libc::c_long as lua_Integer,
    );
    lua_pushinteger(
        L,
        (*L).stack_last.wrapping_offset_from((*L).stack) as libc::c_long as lua_Integer,
    );
    lua_pushinteger(
        L,
        &mut a as *mut libc::c_ulong as libc::c_ulong as lua_Integer,
    );
    return 3i32;
}
unsafe extern "C" fn sethook(mut L: *mut lua_State_0) -> libc::c_int {
    if lua_type(L, 1i32) <= 0i32 {
        /* turn off hooks */
        lua_sethook(L, None, 0i32, 0i32);
    } else {
        let mut scpt: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
        let mut smask: *const libc::c_char = luaL_checklstring(L, 2i32, 0 as *mut size_t);
        let mut count: libc::c_int = luaL_optinteger(L, 3i32, 0i32 as lua_Integer) as libc::c_int;
        let mut mask: libc::c_int = 0i32;
        if !strchr(smask, 'c' as i32).is_null() {
            mask |= 1i32 << 0i32
        }
        if !strchr(smask, 'r' as i32).is_null() {
            mask |= 1i32 << 1i32
        }
        if !strchr(smask, 'l' as i32).is_null() {
            mask |= 1i32 << 2i32
        }
        if count > 0i32 {
            mask |= 1i32 << 3i32
        }
        sethookaux(L, mask, count, scpt);
    }
    return 0i32;
}
unsafe extern "C" fn s2d(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushnumber(
        L,
        *(luaL_checklstring(L, 1i32, 0 as *mut size_t) as *const libc::c_double),
    );
    return 1i32;
}
unsafe extern "C" fn coresume(mut L: *mut lua_State_0) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut co: *mut lua_State_0 = lua_tothread(L, 1i32);
    (!co.is_null() || 0 != luaL_argerror(
        L,
        1i32,
        b"coroutine expected\x00" as *const u8 as *const libc::c_char,
    )) as libc::c_int;
    status = lua_resume(co, L, 0i32);
    if status != 0i32 && status != 1i32 {
        lua_pushboolean(L, 0i32);
        lua_rotate(L, -2i32, 1i32);
        /* return false + error message */
        return 2i32;
    } else {
        lua_pushboolean(L, 1i32);
        return 1i32;
    };
}
unsafe extern "C" fn tref(mut L: *mut lua_State_0) -> libc::c_int {
    let mut level: libc::c_int = lua_gettop(L);
    luaL_checkany(L, 1i32);
    lua_pushvalue(L, 1i32);
    lua_pushinteger(L, luaL_ref(L, -50000i32 - 1000i32) as lua_Integer);
    if lua_gettop(L) == level + 1i32 {
    } else {
        __assert_fail(
            b"lua_gettop(L) == level+1\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            744i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"int tref(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    /* +1 for result */
    return 1i32;
}
unsafe extern "C" fn table_query(mut L: *mut lua_State_0) -> libc::c_int {
    let mut t: *const Table_0 = 0 as *const Table_0;
    let mut i: libc::c_int = luaL_optinteger(L, 2i32, -1i32 as lua_Integer) as libc::c_int;
    luaL_checktype(L, 1i32, 5i32);
    if (*(*(*L).ci).func.offset(1isize)).tt_ == 5i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((((L->ci->func + (1))))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            688i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"int table_query(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(*(*L).ci).func.offset(1isize)).value_.gc).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(
            b"((((L->ci->func + (1)))->value_).gc)->tt == 5\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            688i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"int table_query(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    t = &mut (*((*(*(*L).ci).func.offset(1isize)).value_.gc as *mut GCUnion)).h;
    if i == -1i32 {
        lua_pushinteger(L, (*t).sizearray as lua_Integer);
        lua_pushinteger(
            L,
            (if (*t).lastfree.is_null() {
                0i32
            } else {
                1i32 << (*t).lsizenode as libc::c_int
            }) as lua_Integer,
        );
        lua_pushinteger(
            L,
            (if (*t).lastfree.is_null() {
                0i32 as libc::c_long
            } else {
                (*t).lastfree.wrapping_offset_from((*t).node) as libc::c_long
            }) as lua_Integer,
        );
    } else if (i as libc::c_uint) < (*t).sizearray {
        lua_pushinteger(L, i as lua_Integer);
        pushobject(L, &mut *(*t).array.offset(i as isize));
        lua_pushnil(L);
    } else {
        i = (i as libc::c_uint).wrapping_sub((*t).sizearray) as libc::c_int as libc::c_int;
        if i < 1i32 << (*t).lsizenode as libc::c_int {
            if !((*(*t).node.offset(i as isize)).i_val.tt_ == 0i32)
                || (*(&mut (*(*t).node.offset(i as isize)).i_key.tvk as *mut TValue
                    as *const TValue))
                    .tt_
                    == 0i32
                || (*(&mut (*(*t).node.offset(i as isize)).i_key.tvk as *mut TValue
                    as *const TValue))
                    .tt_
                    & 0xfi32
                    == 3i32
            {
                pushobject(
                    L,
                    &mut (*(*t).node.offset(i as isize)).i_key.tvk as *mut TValue as *const TValue,
                );
            } else {
                lua_pushstring(L, b"<undef>\x00" as *const u8 as *const libc::c_char);
            }
            pushobject(L, &mut (*(*t).node.offset(i as isize)).i_val);
            if (*(*t).node.offset(i as isize)).i_key.nk.next != 0i32 {
                lua_pushinteger(
                    L,
                    (*(*t).node.offset(i as isize)).i_key.nk.next as lua_Integer,
                );
            } else {
                lua_pushnil(L);
            }
        }
    }
    return 3i32;
}
unsafe extern "C" fn pushobject(mut L: *mut lua_State_0, mut o: *const TValue) -> () {
    let mut io1: *mut TValue = (*L).top;
    *io1 = *o;
    if 0 == (*io1).tt_ & 1i32 << 6i32 || {
        if 0 != (*io1).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                64i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void pushobject(lua_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
        (*io1).tt_ & 0x3fi32 == (*(*io1).value_.gc).tt as libc::c_int
            && (L.is_null() || {
                if 0 != (*io1).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io1)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        64i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                            b"void pushobject(lua_State *, const TValue *)\x00",
                        )).as_ptr(),
                    );
                };
                0 != ((*(*io1).value_.gc).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                    & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            })
    } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(
                b"0\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                64i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"void pushobject(lua_State *, const TValue *)\x00",
                )).as_ptr(),
            );
        };
    };
    (*L).top = (*L).top.offset(1isize);
    if (*L).top <= (*(*L).ci).top
        && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            65i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"void pushobject(lua_State *, const TValue *)\x00",
            )).as_ptr(),
        );
    };
}
unsafe extern "C" fn string_query(mut L: *mut lua_State_0) -> libc::c_int {
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    let mut s: libc::c_int = luaL_optinteger(L, 1i32, 0i32 as lua_Integer) as libc::c_int - 1i32;
    if s == -1i32 {
        lua_pushinteger(L, (*tb).size as lua_Integer);
        lua_pushinteger(L, (*tb).nuse as lua_Integer);
        return 2i32;
    } else if s < (*tb).size {
        let mut ts: *mut TString = 0 as *mut TString;
        let mut n: libc::c_int = 0i32;
        ts = *(*tb).hash.offset(s as isize);
        while !ts.is_null() {
            let mut io: *mut TValue = (*L).top;
            let mut x_: *mut TString = ts;
            if (*x_).tt as libc::c_int & 0xfi32 < 9i32 + 1i32 {
            } else {
                __assert_fail(
                    b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    729i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                        b"int string_query(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
            if 0 == (*io).tt_ & 1i32 << 6i32 || {
                if 0 != (*io).tt_ & 1i32 << 6i32 {
                } else {
                    __assert_fail(
                        b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        729i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                            b"int string_query(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
                (*io).tt_ & 0x3fi32 == (*(*io).value_.gc).tt as libc::c_int
                    && (L.is_null() || {
                        if 0 != (*io).tt_ & 1i32 << 6i32 {
                        } else {
                            __assert_fail(
                                b"(((io)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                                729i32 as libc::c_uint,
                                (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                                    b"int string_query(lua_State *)\x00",
                                )).as_ptr(),
                            );
                        };
                        0 != ((*(*io).value_.gc).marked as libc::c_int
                            ^ (1i32 << 0i32 | 1i32 << 1i32))
                            & ((*(*L).l_G).currentwhite as libc::c_int
                                ^ (1i32 << 0i32 | 1i32 << 1i32))
                    })
            } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(
                        b"0\x00" as *const u8 as *const libc::c_char,
                        b"ltests.c\x00" as *const u8 as *const libc::c_char,
                        729i32 as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                            b"int string_query(lua_State *)\x00",
                        )).as_ptr(),
                    );
                };
            };
            (*L).top = (*L).top.offset(1isize);
            if (*L).top <= (*(*L).ci).top
                && !(b"stack overflow\x00" as *const u8 as *const libc::c_char).is_null()
            {
            } else {
                __assert_fail(
                    b"(L->top <= L->ci->top) && \"stack overflow\"\x00" as *const u8
                        as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    730i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                        b"int string_query(lua_State *)\x00",
                    )).as_ptr(),
                );
            };
            n += 1;
            ts = (*ts).u.hnext
        }
        return n;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn pushuserdata(mut L: *mut lua_State_0) -> libc::c_int {
    let mut u: lua_Integer = luaL_checkinteger(L, 1i32);
    lua_pushlightuserdata(L, u as size_t as *mut libc::c_void);
    return 1i32;
}
unsafe extern "C" fn num2int(mut L: *mut lua_State_0) -> libc::c_int {
    lua_pushinteger(L, lua_tointegerx(L, 1i32, 0 as *mut libc::c_int));
    return 1i32;
}
unsafe extern "C" fn newuserdata(mut L: *mut lua_State_0) -> libc::c_int {
    let mut size: size_t = luaL_checkinteger(L, 1i32) as size_t;
    let mut p: *mut libc::c_char = lua_newuserdata(L, size) as *mut libc::c_char;
    loop {
        let fresh8 = size;
        size = size.wrapping_sub(1);
        if !(0 != fresh8) {
            break;
        }
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = '\u{0}' as i32 as libc::c_char
    }
    return 1i32;
}
unsafe extern "C" fn newstate(mut L: *mut lua_State_0) -> libc::c_int {
    let mut ud: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut f: lua_Alloc = lua_getallocf(L, &mut ud);
    let mut L1: *mut lua_State_0 = lua_newstate(f, ud);
    if !L1.is_null() {
        lua_atpanic(L1, Some(tpanic));
        lua_pushlightuserdata(L, L1 as *mut libc::c_void);
    } else {
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn tpanic(mut L: *mut lua_State_0) -> libc::c_int {
    fprintf(
        stderr,
        b"PANIC: unprotected error in call to Lua API (%s)\n\x00" as *const u8
            as *const libc::c_char,
        lua_tolstring(L, -1i32, 0 as *mut size_t),
    );
    /* do not return to Lua */
    exit(1i32);
    return 0i32;
}
unsafe extern "C" fn checkpanic(mut L: *mut lua_State_0) -> libc::c_int {
    let mut b: Aux = Aux {
        jb: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        paniccode: 0 as *const libc::c_char,
        L: 0 as *mut lua_State_0,
    };
    let mut ud: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut L1: *mut lua_State_0 = 0 as *mut lua_State_0;
    let mut code: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut f: lua_Alloc = lua_getallocf(L, &mut ud);
    b.paniccode = luaL_optlstring(
        L,
        2i32,
        b"\x00" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    b.L = L;
    /* create new state */
    L1 = lua_newstate(f, ud);
    if L1.is_null() {
        /* error? */
        lua_pushnil(L);
        return 1i32;
    } else {
        /* set its panic function */
        lua_atpanic(L1, Some(panicback));
        lua_pushlightuserdata(L1, &mut b as *mut Aux as *mut libc::c_void);
        /* store 'Aux' struct */
        lua_setfield(
            L1,
            -50000i32 - 1000i32,
            b"_jmpbuf\x00" as *const u8 as *const libc::c_char,
        );
        if _setjmp(b.jb.as_mut_ptr()) == 0i32 {
            /* set jump buffer */
            /* run code unprotected */
            runC(L, L1, code);
            lua_pushstring(L, b"no errors\x00" as *const u8 as *const libc::c_char);
        } else {
            /* error handling */
            /* move error message to original state */
            lua_pushstring(L, lua_tolstring(L1, -1i32, 0 as *mut size_t));
        }
        lua_close(L1);
        return 1i32;
    };
}
/*
** does a long-jump back to "main program".
*/
unsafe extern "C" fn panicback(mut L: *mut lua_State_0) -> libc::c_int {
    let mut b: *mut Aux = 0 as *mut Aux;
    /* open space for 'Aux' struct */
    lua_checkstack(L, 1i32);
    /* get 'Aux' struct */
    lua_getfield(
        L,
        -50000i32 - 1000i32,
        b"_jmpbuf\x00" as *const u8 as *const libc::c_char,
    );
    b = lua_touserdata(L, -1i32) as *mut Aux;
    lua_settop(L, -1i32 - 1i32);
    /* remove 'Aux' struct */
    /* run optional panic code */
    runC((*b).L, L, (*b).paniccode);
    longjmp((*b).jb.as_mut_ptr(), 1i32);
}
unsafe extern "C" fn loadlib(mut L: *mut lua_State_0) -> libc::c_int {
    static mut libs: [luaL_Reg_0; 9] = unsafe {
        [
            luaL_Reg {
                name: b"_G\x00" as *const u8 as *const libc::c_char,
                func: Some(luaopen_base),
            },
            luaL_Reg {
                name: b"coroutine\x00" as *const u8 as *const libc::c_char,
                func: Some(luaopen_coroutine),
            },
            luaL_Reg {
                name: b"debug\x00" as *const u8 as *const libc::c_char,
                func: Some(luaopen_debug),
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
                name: b"math\x00" as *const u8 as *const libc::c_char,
                func: Some(luaopen_math),
            },
            luaL_Reg {
                name: b"string\x00" as *const u8 as *const libc::c_char,
                func: Some(luaopen_string),
            },
            luaL_Reg {
                name: b"table\x00" as *const u8 as *const libc::c_char,
                func: Some(luaopen_table),
            },
            luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            },
        ]
    };
    let mut L1: *mut lua_State_0 = getstate(L);
    let mut i: libc::c_int = 0;
    luaL_requiref(
        L1,
        b"package\x00" as *const u8 as *const libc::c_char,
        Some(luaopen_package),
        0i32,
    );
    if lua_type(L1, -1i32) == 5i32 {
    } else {
        __assert_fail(
            b"lua_type(L1, -1) == 5\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            868i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"int loadlib(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    /* 'requiref' should not reload module already loaded... */
    /* seg. fault if it reloads */
    luaL_requiref(
        L1,
        b"package\x00" as *const u8 as *const libc::c_char,
        None,
        1i32,
    );
    if 0 != lua_compare(L1, -1i32, -2i32, 0i32) {
    } else {
        __assert_fail(
            b"lua_compare(L1, -1, -2, 0)\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            872i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"int loadlib(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    /* ...but should return the same module */
    luaL_getsubtable(
        L1,
        -50000i32 - 1000i32,
        b"_PRELOAD\x00" as *const u8 as *const libc::c_char,
    );
    i = 0i32;
    while !libs[i as usize].name.is_null() {
        lua_pushcclosure(L1, libs[i as usize].func, 0i32);
        lua_setfield(L1, -2i32, libs[i as usize].name);
        i += 1
    }
    return 0i32;
}
unsafe extern "C" fn listlocals(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut Proto_0 = 0 as *mut Proto_0;
    let mut pc: libc::c_int = luaL_checkinteger(L, 2i32) as libc::c_int - 1i32;
    let mut i: libc::c_int = 0i32;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    (lua_type(L, 1i32) == 6i32 && 0 == lua_iscfunction(L, 1i32) || 0 != luaL_argerror(
        L,
        1i32,
        b"Lua function expected\x00" as *const u8 as *const libc::c_char,
    )) as libc::c_int;
    if (*(*(*L).ci).func.offset(1isize)).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((((L->ci->func + (1))))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            549i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"int listlocals(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(*(*L).ci).func.offset(1isize)).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"((((L->ci->func + (1)))->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            549i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"int listlocals(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    p = (*&mut (*((*(*(*L).ci).func.offset(1isize)).value_.gc as *mut GCUnion))
        .cl
        .l)
        .p;
    loop {
        i += 1;
        name = luaF_getlocalname(p, i, pc);
        if name.is_null() {
            break;
        }
        lua_pushstring(L, name);
    }
    return i - 1i32;
}
unsafe extern "C" fn listk(mut L: *mut lua_State_0) -> libc::c_int {
    let mut p: *mut Proto_0 = 0 as *mut Proto_0;
    let mut i: libc::c_int = 0;
    (lua_type(L, 1i32) == 6i32 && 0 == lua_iscfunction(L, 1i32) || 0 != luaL_argerror(
        L,
        1i32,
        b"Lua function expected\x00" as *const u8 as *const libc::c_char,
    )) as libc::c_int;
    if (*(*(*L).ci).func.offset(1isize)).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((((L->ci->func + (1))))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            532i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int listk(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(*(*L).ci).func.offset(1isize)).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"((((L->ci->func + (1)))->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            532i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int listk(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    p = (*&mut (*((*(*(*L).ci).func.offset(1isize)).value_.gc as *mut GCUnion))
        .cl
        .l)
        .p;
    lua_createtable(L, (*p).sizek, 0i32);
    i = 0i32;
    while i < (*p).sizek {
        pushobject(L, (*p).k.offset(i as isize));
        lua_rawseti(L, -2i32, (i + 1i32) as lua_Integer);
        i += 1
    }
    return 1i32;
}
unsafe extern "C" fn listcode(mut L: *mut lua_State_0) -> libc::c_int {
    let mut pc: libc::c_int = 0;
    let mut p: *mut Proto_0 = 0 as *mut Proto_0;
    (lua_type(L, 1i32) == 6i32 && 0 == lua_iscfunction(L, 1i32) || 0 != luaL_argerror(
        L,
        1i32,
        b"Lua function expected\x00" as *const u8 as *const libc::c_char,
    )) as libc::c_int;
    if (*(*(*L).ci).func.offset(1isize)).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
    } else {
        __assert_fail(
            b"(((((L->ci->func + (1))))->tt_) == ((((6 | (0 << 4))) | (1 << 6))))\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            513i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"int listcode(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    if (*(*(*(*L).ci).func.offset(1isize)).value_.gc).tt as libc::c_int == 6i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(
            b"((((L->ci->func + (1)))->value_).gc)->tt == (6 | (0 << 4))\x00" as *const u8
                as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            513i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"int listcode(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    p = (*&mut (*((*(*(*L).ci).func.offset(1isize)).value_.gc as *mut GCUnion))
        .cl
        .l)
        .p;
    lua_createtable(L, 0i32, 0i32);
    setnameval(
        L,
        b"maxstack\x00" as *const u8 as *const libc::c_char,
        (*p).maxstacksize as libc::c_int,
    );
    setnameval(
        L,
        b"numparams\x00" as *const u8 as *const libc::c_char,
        (*p).numparams as libc::c_int,
    );
    pc = 0i32;
    while pc < (*p).sizecode {
        let mut buff: [libc::c_char; 100] = [0; 100];
        lua_pushinteger(L, (pc + 1i32) as lua_Integer);
        lua_pushstring(L, buildop(p, pc, buff.as_mut_ptr()));
        lua_settable(L, -3i32);
        pc += 1
    }
    return 1i32;
}
/* }====================================================== */
/*
** {======================================================
** Disassembler
** =======================================================
*/
unsafe extern "C" fn buildop(
    mut p: *mut Proto_0,
    mut pc: libc::c_int,
    mut buff: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: Instruction = *(*p).code.offset(pc as isize);
    let mut o: OpCode = (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
    let mut name: *const libc::c_char = luaP_opnames[o as usize];
    let mut line: libc::c_int = if !(*p).lineinfo.is_null() {
        *(*p).lineinfo.offset(pc as isize)
    } else {
        -1i32
    };
    sprintf(
        buff,
        b"(%4d) %4d - \x00" as *const u8 as *const libc::c_char,
        line,
        pc,
    );
    match (luaP_opmodes[o as usize] as libc::c_int & 3i32) as OpMode as libc::c_uint {
        0 => {
            sprintf(
                buff.offset(strlen(buff) as isize),
                b"%-12s%4d %4d %4d\x00" as *const u8 as *const libc::c_char,
                name,
                (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int,
                (i >> 0i32 + 6i32 + 8i32 + 9i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                    as libc::c_int,
                (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                    as libc::c_int,
            );
        }
        1 => {
            sprintf(
                buff.offset(strlen(buff) as isize),
                b"%-12s%4d %4d\x00" as *const u8 as *const libc::c_char,
                name,
                (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int,
                (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                    as libc::c_int,
            );
        }
        2 => {
            sprintf(
                buff.offset(strlen(buff) as isize),
                b"%-12s%4d %4d\x00" as *const u8 as *const libc::c_char,
                name,
                (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int,
                (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                    as libc::c_int
                    - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32),
            );
        }
        3 => {
            sprintf(
                buff.offset(strlen(buff) as isize),
                b"%-12s%4d\x00" as *const u8 as *const libc::c_char,
                name,
                (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
                    as libc::c_int,
            );
        }
        _ => {}
    }
    return buff;
}
unsafe extern "C" fn setnameval(
    mut L: *mut lua_State_0,
    mut name: *const libc::c_char,
    mut val: libc::c_int,
) -> () {
    lua_pushstring(L, name);
    lua_pushinteger(L, val as lua_Integer);
    lua_settable(L, -3i32);
}
unsafe extern "C" fn get_limits(mut L: *mut lua_State_0) -> libc::c_int {
    lua_createtable(L, 0i32, 5i32);
    setnameval(
        L,
        b"BITS_INT\x00" as *const u8 as *const libc::c_char,
        32i32,
    );
    setnameval(
        L,
        b"MAXARG_Ax\x00" as *const u8 as *const libc::c_char,
        (1i32 << 9i32 + 9i32 + 8i32) - 1i32,
    );
    setnameval(
        L,
        b"MAXARG_Bx\x00" as *const u8 as *const libc::c_char,
        (1i32 << 9i32 + 9i32) - 1i32,
    );
    setnameval(
        L,
        b"MAXARG_sBx\x00" as *const u8 as *const libc::c_char,
        (1i32 << 9i32 + 9i32) - 1i32 >> 1i32,
    );
    setnameval(
        L,
        b"BITS_INT\x00" as *const u8 as *const libc::c_char,
        32i32,
    );
    setnameval(L, b"LFPF\x00" as *const u8 as *const libc::c_char, 50i32);
    setnameval(
        L,
        b"NUM_OPCODES\x00" as *const u8 as *const libc::c_char,
        OP_EXTRAARG as libc::c_int + 1i32,
    );
    return 1i32;
}
unsafe extern "C" fn log2_aux(mut L: *mut lua_State_0) -> libc::c_int {
    let mut x: libc::c_uint = luaL_checkinteger(L, 1i32) as libc::c_uint;
    lua_pushinteger(L, luaO_ceillog2(x) as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn int2fb_aux(mut L: *mut lua_State_0) -> libc::c_int {
    let mut b: libc::c_int = luaO_int2fb(luaL_checkinteger(L, 1i32) as libc::c_uint);
    lua_pushinteger(L, b as lua_Integer);
    lua_pushinteger(L, luaO_fb2int(b) as libc::c_uint as lua_Integer);
    return 2i32;
}
unsafe extern "C" fn hash_query(mut L: *mut lua_State_0) -> libc::c_int {
    let mut o: *mut TValue = 0 as *mut TValue;
    let mut t: *mut Table_0 = 0 as *mut Table_0;
    if lua_type(L, 2i32) == -1i32 {
        (lua_type(L, 1i32) == 4i32 || 0 != luaL_argerror(
            L,
            1i32,
            b"string expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
        if (*(*(*L).ci).func.offset(1isize)).tt_ & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"((((((((L->ci->func + (1))))->tt_)) & 0x0F)) == (4))\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                662i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"int hash_query(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*(*(*L).ci).func.offset(1isize)).value_.gc).tt as libc::c_int & 0xfi32 == 4i32 {
        } else {
            __assert_fail(
                b"((((((L->ci->func + (1)))->value_).gc)->tt) & 0x0F) == 4\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                662i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"int hash_query(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        lua_pushinteger(
            L,
            (*&mut (*((*(*(*L).ci).func.offset(1isize)).value_.gc as *mut GCUnion)).ts).hash
                as lua_Integer,
        );
    } else {
        o = (*(*L).ci).func.offset(1isize);
        t = 0 as *mut Table_0;
        luaL_checktype(L, 2i32, 5i32);
        if (*(*(*L).ci).func.offset(2isize)).tt_ == 5i32 | 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((((L->ci->func + (2))))->tt_) == (((5) | (1 << 6))))\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                668i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"int hash_query(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        if (*(*(*(*L).ci).func.offset(2isize)).value_.gc).tt as libc::c_int == 5i32 {
        } else {
            __assert_fail(
                b"((((L->ci->func + (2)))->value_).gc)->tt == 5\x00" as *const u8
                    as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                668i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"int hash_query(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        t = &mut (*((*(*(*L).ci).func.offset(2isize)).value_.gc as *mut GCUnion)).h;
        lua_pushinteger(
            L,
            luaH_mainposition(t, o).wrapping_offset_from((*t).node) as libc::c_long as lua_Integer,
        );
    }
    return 1i32;
}
unsafe extern "C" fn getref(mut L: *mut lua_State_0) -> libc::c_int {
    let mut level: libc::c_int = lua_gettop(L);
    lua_rawgeti(L, -50000i32 - 1000i32, luaL_checkinteger(L, 1i32));
    if lua_gettop(L) == level + 1i32 {
    } else {
        __assert_fail(
            b"lua_gettop(L) == level+1\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            751i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"int getref(lua_State *)\x00",
            )).as_ptr(),
        );
    };
    return 1i32;
}
unsafe extern "C" fn gc_state(mut L: *mut lua_State_0) -> libc::c_int {
    static mut statenames: [*const libc::c_char; 8] = unsafe {
        [
            b"propagate\x00" as *const u8 as *const libc::c_char,
            b"atomic\x00" as *const u8 as *const libc::c_char,
            b"sweepallgc\x00" as *const u8 as *const libc::c_char,
            b"sweepfinobj\x00" as *const u8 as *const libc::c_char,
            b"sweeptobefnz\x00" as *const u8 as *const libc::c_char,
            b"sweepend\x00" as *const u8 as *const libc::c_char,
            b"pause\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        ]
    };
    static mut states: [libc::c_int; 8] =
        unsafe { [0i32, 1i32, 2i32, 3i32, 4i32, 5i32, 7i32, -1i32] };
    let mut option: libc::c_int = states[luaL_checkoption(
        L,
        1i32,
        b"\x00" as *const u8 as *const libc::c_char,
        statenames.as_mut_ptr() as *const *const libc::c_char,
    ) as usize];
    if option == -1i32 {
        lua_pushstring(L, statenames[(*(*L).l_G).gcstate as usize]);
        return 1i32;
    } else {
        let mut g: *mut global_State = (*L).l_G;
        let ref mut fresh10 = *(*((L as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void as *mut L_EXTRA))
            .plock;
        let fresh11 = *fresh10;
        *fresh10 = *fresh10 + 1;
        if fresh11 == 0i32 {
        } else {
            __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ltests.c\x00" as *const u8 as *const libc::c_char,
                          647i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 26],
                                                    &[libc::c_char; 26]>(b"int gc_state(lua_State *)\x00")).as_ptr());
        };
        if option < (*g).gcstate as libc::c_int {
            /* must cross 'pause'? */
            /* run until pause */
            luaC_runtilstate(L, 1i32 << 7i32);
        }
        luaC_runtilstate(L, 1i32 << option);
        if (*(*L).l_G).gcstate as libc::c_int == option {
        } else {
            __assert_fail(
                b"(L->l_G)->gcstate == option\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                652i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"int gc_state(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        let ref mut fresh12 = *(*((L as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<L_EXTRA>() as libc::c_ulong as isize))
            as *mut libc::c_void as *mut L_EXTRA))
            .plock;
        *fresh12 -= 1;
        if *fresh12 == 0i32 {
        } else {
            __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ltests.c\x00" as *const u8 as *const libc::c_char,
                          653i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 26],
                                                    &[libc::c_char; 26]>(b"int gc_state(lua_State *)\x00")).as_ptr());
        };
        return 0i32;
    };
}
unsafe extern "C" fn gc_color(mut L: *mut lua_State_0) -> libc::c_int {
    let mut obj: *mut GCObject = 0 as *mut GCObject;
    let mut o: *mut TValue = 0 as *mut TValue;
    luaL_checkany(L, 1i32);
    o = (*(*L).ci).func.offset(1isize);
    if 0 == (*o).tt_ & 1i32 << 6i32 {
        lua_pushstring(L, b"no collectable\x00" as *const u8 as *const libc::c_char);
    } else {
        if 0 != (*o).tt_ & 1i32 << 6i32 {
        } else {
            __assert_fail(
                b"(((o)->tt_) & (1 << 6))\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                626i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"int gc_color(lua_State *)\x00",
                )).as_ptr(),
            );
        };
        obj = (*o).value_.gc;
        lua_pushstring(
            L,
            if 0 == ((*obj).marked as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*(*L).l_G).currentwhite as libc::c_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            {
                b"dead\x00" as *const u8 as *const libc::c_char
            } else if 0 != (*obj).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32) {
                b"white\x00" as *const u8 as *const libc::c_char
            } else if 0 != (*obj).marked as libc::c_int & 1i32 << 2i32 {
                b"black\x00" as *const u8 as *const libc::c_char
            } else {
                b"grey\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    return 1i32;
}
unsafe extern "C" fn doremote(mut L: *mut lua_State_0) -> libc::c_int {
    let mut L1: *mut lua_State_0 = getstate(L);
    let mut lcode: size_t = 0;
    let mut code: *const libc::c_char = luaL_checklstring(L, 2i32, &mut lcode);
    let mut status: libc::c_int = 0;
    lua_settop(L1, 0i32);
    status = luaL_loadbufferx(L1, code, lcode, code, 0 as *const libc::c_char);
    if status == 0i32 {
        status = lua_pcallk(L1, 0i32, -1i32, 0i32, 0i32 as lua_KContext, None)
    }
    if status != 0i32 {
        lua_pushnil(L);
        lua_pushstring(L, lua_tolstring(L1, -1i32, 0 as *mut size_t));
        lua_pushinteger(L, status as lua_Integer);
        return 3i32;
    } else {
        let mut i: libc::c_int = 0i32;
        loop {
            i += 1;
            if lua_type(L1, i) == -1i32 {
                break;
            }
            lua_pushstring(L, lua_tolstring(L1, i, 0 as *mut size_t));
        }
        lua_settop(L1, -(i - 1i32) - 1i32);
        return i - 1i32;
    };
}
unsafe extern "C" fn doonnewstack(mut L: *mut lua_State_0) -> libc::c_int {
    let mut L1: *mut lua_State_0 = lua_newthread(L);
    let mut l: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut l);
    let mut status: libc::c_int = luaL_loadbufferx(L1, s, l, s, 0 as *const libc::c_char);
    if status == 0i32 {
        status = lua_pcallk(L1, 0i32, 0i32, 0i32, 0i32 as lua_KContext, None)
    }
    lua_pushinteger(L, status as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn d2s(mut L: *mut lua_State_0) -> libc::c_int {
    let mut d: libc::c_double = luaL_checknumber(L, 1i32);
    lua_pushlstring(
        L,
        &mut d as *mut libc::c_double as *mut libc::c_char,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    return 1i32;
}
unsafe extern "C" fn closestate(mut L: *mut lua_State_0) -> libc::c_int {
    let mut L1: *mut lua_State_0 = getstate(L);
    lua_close(L1);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn debug_realloc(
    mut ud: *mut libc::c_void,
    mut b: *mut libc::c_void,
    mut oldsize: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut limit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mc: *mut Memcontrol_0 = ud as *mut Memcontrol_0;
    let mut block: *mut Header = b as *mut Header;
    let mut type_0: libc::c_int = 0;
    if (*mc).memlimit == 0i32 as libc::c_ulong {
        /* first time? */
        /* initialize memory limit */
        limit = getenv(b"MEMLIMIT\x00" as *const u8 as *const libc::c_char);
        (*mc).memlimit = if !limit.is_null() {
            strtoul(limit, 0 as *mut *mut libc::c_char, 10i32)
        } else {
            (9223372036854775807i64 as libc::c_ulong)
                .wrapping_mul(2u64)
                .wrapping_add(1u64)
        }
    }
    if block.is_null() {
        type_0 = (if oldsize < 9i32 as libc::c_ulong {
            oldsize
        } else {
            0i32 as libc::c_ulong
        }) as libc::c_int;
        oldsize = 0i32 as size_t
    } else {
        /* go to real header */
        block = block.offset(-1isize);
        type_0 = (*block).d.type_0;
        if oldsize == (*block).d.size {
        } else {
            __assert_fail(
                b"oldsize == block->d.size\x00" as *const u8 as *const libc::c_char,
                b"ltests.c\x00" as *const u8 as *const libc::c_char,
                142i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                    b"void *debug_realloc(void *, void *, size_t, size_t)\x00",
                )).as_ptr(),
            );
        };
    }
    if size == 0i32 as libc::c_ulong {
        freeblock(mc, block);
        return 0 as *mut libc::c_void;
    } else if size > oldsize
        && (*mc).total.wrapping_add(size).wrapping_sub(oldsize) > (*mc).memlimit
    {
        /* fake a memory allocation error */
        return 0 as *mut libc::c_void;
    } else {
        let mut newblock: *mut Header = 0 as *mut Header;
        let mut i: libc::c_int = 0;
        let mut commonsize: size_t = if oldsize < size { oldsize } else { size };
        let mut realsize: size_t = (::std::mem::size_of::<Header>() as libc::c_ulong)
            .wrapping_add(size)
            .wrapping_add(16i32 as libc::c_ulong);
        if realsize < size {
            /* arithmetic overflow! */
            return 0 as *mut libc::c_void;
        } else {
            /* alloc a new block */
            newblock = malloc(realsize) as *mut Header;
            if newblock.is_null() {
                /* really out of memory? */
                return 0 as *mut libc::c_void;
            } else {
                if !block.is_null() {
                    /* copy old contents */
                    memcpy(
                        newblock.offset(1isize) as *mut libc::c_void,
                        block.offset(1isize) as *const libc::c_void,
                        commonsize,
                    );
                    /* erase (and check) old copy */
                    freeblock(mc, block);
                }
                memset(
                    (newblock.offset(1isize) as *mut libc::c_char).offset(commonsize as isize)
                        as *mut libc::c_void,
                    -0x55i32,
                    size.wrapping_sub(commonsize),
                );
                /* initialize new part of the block with something weird */
                /* initialize marks after block */
                i = 0i32;
                while i < 16i32 {
                    *(newblock.offset(1isize) as *mut libc::c_char)
                        .offset(size as isize)
                        .offset(i as isize) = 0x55i32 as libc::c_char;
                    i += 1
                }
                (*newblock).d.size = size;
                (*newblock).d.type_0 = type_0;
                (*mc).total = (*mc).total.wrapping_add(size);
                if (*mc).total > (*mc).maxmem {
                    (*mc).maxmem = (*mc).total
                }
                (*mc).numblocks = (*mc).numblocks.wrapping_add(1);
                (*mc).objcount[type_0 as usize] = (*mc).objcount[type_0 as usize].wrapping_add(1);
                return newblock.offset(1isize) as *mut libc::c_void;
            }
        }
    };
}
/* full memory check */
/* size of marks after each block */
unsafe extern "C" fn freeblock(mut mc: *mut Memcontrol_0, mut block: *mut Header) -> () {
    if !block.is_null() {
        let mut size: size_t = (*block).d.size;
        let mut i: libc::c_int = 0;
        /* check marks after block */
        i = 0i32;
        while i < 16i32 {
            if *(block.offset(1isize) as *mut libc::c_char)
                .offset(size as isize)
                .offset(i as isize) as libc::c_int
                == 0x55i32
            {
            } else {
                __assert_fail(
                    b"*(((char *)(block + 1)) + size + i) == 0x55\x00" as *const u8
                        as *const libc::c_char,
                    b"ltests.c\x00" as *const u8 as *const libc::c_char,
                    117i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"void freeblock(Memcontrol *, Header *)\x00",
                    )).as_ptr(),
                );
            };
            i += 1
        }
        (*mc).objcount[(*block).d.type_0 as usize] =
            (*mc).objcount[(*block).d.type_0 as usize].wrapping_sub(1);
        memset(
            block as *mut libc::c_void,
            -0x55i32,
            (::std::mem::size_of::<Header>() as libc::c_ulong)
                .wrapping_add(size)
                .wrapping_add(16i32 as libc::c_ulong),
        );
        /* erase block */
        /* actually free block */
        free(block as *mut libc::c_void);
        /* update counts */
        (*mc).numblocks = (*mc).numblocks.wrapping_sub(1);
        (*mc).total = (*mc).total.wrapping_sub(size)
    };
}
unsafe extern "C" fn checkfinalmem() -> () {
    if l_memcontrol.numblocks == 0i32 as libc::c_ulong {
    } else {
        __assert_fail(
            b"l_memcontrol.numblocks == 0\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            1554i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"void checkfinalmem(void)\x00",
            )).as_ptr(),
        );
    };
    if l_memcontrol.total == 0i32 as libc::c_ulong {
    } else {
        __assert_fail(
            b"l_memcontrol.total == 0\x00" as *const u8 as *const libc::c_char,
            b"ltests.c\x00" as *const u8 as *const libc::c_char,
            1555i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"void checkfinalmem(void)\x00",
            )).as_ptr(),
        );
    };
}
/*
** $Id: ltests.c,v 2.210 2016/11/07 12:38:35 roberto Exp roberto $
** Internal Module for Debugging of the Lua Implementation
** See Copyright Notice in lua.h
*/
/*
** The whole module only makes sense with LUA_DEBUG on
*/
#[no_mangle]
pub static mut islocked: libc::c_int = unsafe { 0i32 };
