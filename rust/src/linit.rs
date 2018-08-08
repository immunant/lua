#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc)]
extern crate libc;
extern "C" {
    pub type lua_State;
    pub type _IO_FILE_plus;
    #[no_mangle]
    static mut l_memcontrol: Memcontrol_0;
    #[no_mangle]
    static mut l_Trick: *mut libc::c_void;
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
    fn luaL_requiref(L: *mut lua_State_0, modname: *const libc::c_char,
                     openf: lua_CFunction, glb: libc::c_int) -> ();
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
pub type lua_State_0 = lua_State;
pub type lua_CFunction =
    Option<unsafe extern "C" fn(_: *mut lua_State_0) -> libc::c_int>;
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
pub type luaL_Reg = luaL_Reg_0;
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
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
pub type Memcontrol_0 = Memcontrol;
pub type _IO_lock_t = ();
#[no_mangle]
pub unsafe extern "C" fn luaL_openlibs(mut L: *mut lua_State_0) -> () {
    let mut lib: *const luaL_Reg = 0 as *const luaL_Reg;
    lib = loadedlibs.as_ptr();
    while (*lib).func.is_some() {
        luaL_requiref(L, (*lib).name, (*lib).func, 1i32);
        lua_settop(L, -1i32 - 1i32);
        lib = lib.offset(1isize)
    };
}
static mut loadedlibs: [luaL_Reg; 11] =
    unsafe {
        [luaL_Reg_0{name: b"_G\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_base),},
         luaL_Reg_0{name: b"package\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_package),},
         luaL_Reg_0{name:
                        b"coroutine\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_coroutine),},
         luaL_Reg_0{name: b"table\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_table),},
         luaL_Reg_0{name: b"io\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_io),},
         luaL_Reg_0{name: b"os\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_os),},
         luaL_Reg_0{name: b"string\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_string),},
         luaL_Reg_0{name: b"math\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_math),},
         luaL_Reg_0{name: b"utf8\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_utf8),},
         luaL_Reg_0{name: b"debug\x00" as *const u8 as *const libc::c_char,
                    func: Some(luaopen_debug),},
         luaL_Reg_0{name: 0 as *const libc::c_char, func: None,}]
    };
