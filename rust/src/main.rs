#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, const_slice_as_ptr, offset_to)]
extern crate libc;
pub mod lapi;
pub mod lauxlib;
pub mod lbaselib;
pub mod lbitlib;
pub mod lcode;
pub mod lcorolib;
pub mod lctype;
pub mod ldblib;
pub mod ldebug;
pub mod ldo;
pub mod ldump;
pub mod lfunc;
pub mod lgc;
pub mod linit;
pub mod liolib;
pub mod llex;
pub mod lmathlib;
pub mod lmem;
pub mod loadlib;
pub mod lobject;
pub mod lopcodes;
pub mod loslib;
pub mod lparser;
pub mod lstate;
pub mod lstring;
pub mod lstrlib;
pub mod ltable;
pub mod ltablib;
pub mod ltests;
pub mod ltm;
pub mod lundump;
pub mod lutf8lib;
pub mod lvm;
pub mod lzio;
extern "C" {
    pub type _IO_FILE_plus;
    pub type lua_longjmp;
    pub type UpVal_0;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
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
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t,
              __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn luaB_opentests(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn debug_realloc(ud: *mut libc::c_void, block: *mut libc::c_void,
                     osize: size_t, nsize: size_t) -> *mut libc::c_void;
    #[no_mangle]
    static lua_ident: [libc::c_char; 0];
    #[no_mangle]
    fn lua_newstate(f: lua_Alloc, ud: *mut libc::c_void) -> *mut lua_State_0;
    #[no_mangle]
    fn lua_close(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State_0) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State_0, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State_0, idx: libc::c_int, n: libc::c_int)
     -> ();
    #[no_mangle]
    fn lua_type(L: *mut lua_State_0, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State_0, tp: libc::c_int)
     -> *const libc::c_char;
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
    fn lua_pushboolean(L: *mut lua_State_0, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State_0, p: *mut libc::c_void) -> ();
    #[no_mangle]
    fn lua_getglobal(L: *mut lua_State_0, name: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer)
     -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State_0, narr: libc::c_int,
                       nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State_0, name: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State_0, idx: libc::c_int,
                    k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State_0, idx: libc::c_int, n: lua_Integer)
     -> ();
    #[no_mangle]
    fn lua_pcallk(L: *mut lua_State_0, nargs: libc::c_int,
                  nresults: libc::c_int, errfunc: libc::c_int,
                  ctx: lua_KContext, k: lua_KFunction) -> libc::c_int;
    #[no_mangle]
    fn lua_concat(L: *mut lua_State_0, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_sethook(L: *mut lua_State_0, func: lua_Hook, mask: libc::c_int,
                   count: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State_0, ver: lua_Number, sz: size_t)
     -> ();
    #[no_mangle]
    fn luaL_callmeta(L: *mut lua_State_0, obj: libc::c_int,
                     e: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State_0, sz: libc::c_int,
                       msg: *const libc::c_char) -> ();
    #[no_mangle]
    fn luaL_loadfilex(L: *mut lua_State_0, filename: *const libc::c_char,
                      mode: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_loadbufferx(L: *mut lua_State_0, buff: *const libc::c_char,
                        sz: size_t, name: *const libc::c_char,
                        mode: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_len(L: *mut lua_State_0, idx: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_traceback(L: *mut lua_State_0, L1: *mut lua_State_0,
                      msg: *const libc::c_char, level: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_requiref(L: *mut lua_State_0, modname: *const libc::c_char,
                     openf: lua_CFunction, glb: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_openlibs(L: *mut lua_State_0) -> ();
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> libc::c_int;
    #[no_mangle]
    static mut __environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    static mut emacs_standard_keymap: KEYMAP_ENTRY_ARRAY;
    #[no_mangle]
    static mut emacs_meta_keymap: KEYMAP_ENTRY_ARRAY;
    #[no_mangle]
    static mut emacs_ctlx_keymap: KEYMAP_ENTRY_ARRAY;
    #[no_mangle]
    static mut vi_insertion_keymap: KEYMAP_ENTRY_ARRAY;
    #[no_mangle]
    static mut vi_movement_keymap: KEYMAP_ENTRY_ARRAY;
    #[no_mangle]
    static mut tilde_expansion_preexpansion_hook:
           Option<unsafe extern "C" fn(_: *mut libc::c_char)
                      -> *mut libc::c_char>;
    #[no_mangle]
    static mut tilde_expansion_failure_hook:
           Option<unsafe extern "C" fn(_: *mut libc::c_char)
                      -> *mut libc::c_char>;
    #[no_mangle]
    static mut tilde_additional_prefixes: *mut *mut libc::c_char;
    #[no_mangle]
    static mut tilde_additional_suffixes: *mut *mut libc::c_char;
    #[no_mangle]
    static mut rl_undo_list: *mut UNDO_LIST;
    #[no_mangle]
    static mut funmap: *mut *mut FUNMAP;
    #[no_mangle]
    fn readline(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut rl_library_version: *const libc::c_char;
    #[no_mangle]
    static mut rl_readline_version: libc::c_int;
    #[no_mangle]
    static mut rl_gnu_readline_p: libc::c_int;
    #[no_mangle]
    static mut rl_readline_state: libc::c_ulong;
    #[no_mangle]
    static mut rl_editing_mode: libc::c_int;
    #[no_mangle]
    static mut rl_insert_mode: libc::c_int;
    #[no_mangle]
    static mut rl_readline_name: *const libc::c_char;
    #[no_mangle]
    static mut rl_prompt: *mut libc::c_char;
    #[no_mangle]
    static mut rl_display_prompt: *mut libc::c_char;
    #[no_mangle]
    static mut rl_line_buffer: *mut libc::c_char;
    #[no_mangle]
    static mut rl_point: libc::c_int;
    #[no_mangle]
    static mut rl_end: libc::c_int;
    #[no_mangle]
    static mut rl_mark: libc::c_int;
    #[no_mangle]
    static mut rl_done: libc::c_int;
    #[no_mangle]
    static mut rl_pending_input: libc::c_int;
    #[no_mangle]
    static mut rl_dispatching: libc::c_int;
    #[no_mangle]
    static mut rl_explicit_arg: libc::c_int;
    #[no_mangle]
    static mut rl_numeric_arg: libc::c_int;
    #[no_mangle]
    static mut rl_last_func:
           Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                      -> libc::c_int>;
    #[no_mangle]
    static mut rl_terminal_name: *const libc::c_char;
    #[no_mangle]
    static mut rl_instream: *mut FILE;
    #[no_mangle]
    static mut rl_outstream: *mut FILE;
    #[no_mangle]
    static mut rl_prefer_env_winsize: libc::c_int;
    #[no_mangle]
    static mut rl_startup_hook: Option<unsafe extern "C" fn() -> libc::c_int>;
    #[no_mangle]
    static mut rl_pre_input_hook:
           Option<unsafe extern "C" fn() -> libc::c_int>;
    #[no_mangle]
    static mut rl_event_hook: Option<unsafe extern "C" fn() -> libc::c_int>;
    #[no_mangle]
    static mut rl_signal_event_hook:
           Option<unsafe extern "C" fn() -> libc::c_int>;
    #[no_mangle]
    static mut rl_input_available_hook:
           Option<unsafe extern "C" fn() -> libc::c_int>;
    #[no_mangle]
    static mut rl_getc_function:
           Option<unsafe extern "C" fn(_: *mut FILE) -> libc::c_int>;
    #[no_mangle]
    static mut rl_redisplay_function: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut rl_prep_term_function:
           Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    #[no_mangle]
    static mut rl_deprep_term_function: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut rl_executing_keymap: Keymap;
    #[no_mangle]
    static mut rl_binding_keymap: Keymap;
    #[no_mangle]
    static mut rl_executing_key: libc::c_int;
    #[no_mangle]
    static mut rl_executing_keyseq: *mut libc::c_char;
    #[no_mangle]
    static mut rl_key_sequence_length: libc::c_int;
    #[no_mangle]
    static mut rl_erase_empty_line: libc::c_int;
    #[no_mangle]
    static mut rl_already_prompted: libc::c_int;
    #[no_mangle]
    static mut rl_num_chars_to_read: libc::c_int;
    #[no_mangle]
    static mut rl_executing_macro: *mut libc::c_char;
    #[no_mangle]
    static mut rl_catch_signals: libc::c_int;
    #[no_mangle]
    static mut rl_catch_sigwinch: libc::c_int;
    #[no_mangle]
    static mut rl_change_environment: libc::c_int;
    #[no_mangle]
    static mut rl_completion_entry_function:
           Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int)
                      -> *mut libc::c_char>;
    #[no_mangle]
    static mut rl_menu_completion_entry_function:
           Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int)
                      -> *mut libc::c_char>;
    #[no_mangle]
    static mut rl_ignore_some_completions_function:
           Option<unsafe extern "C" fn(_: *mut *mut libc::c_char)
                      -> libc::c_int>;
    #[no_mangle]
    static mut rl_attempted_completion_function:
           Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                                       _: libc::c_int)
                      -> *mut *mut libc::c_char>;
    #[no_mangle]
    static mut rl_basic_word_break_characters: *const libc::c_char;
    #[no_mangle]
    static mut rl_completer_word_break_characters: *mut libc::c_char;
    #[no_mangle]
    static mut rl_completion_word_break_hook:
           Option<unsafe extern "C" fn() -> *mut libc::c_char>;
    #[no_mangle]
    static mut rl_completer_quote_characters: *const libc::c_char;
    #[no_mangle]
    static mut rl_basic_quote_characters: *const libc::c_char;
    #[no_mangle]
    static mut rl_filename_quote_characters: *const libc::c_char;
    #[no_mangle]
    static mut rl_special_prefixes: *const libc::c_char;
    #[no_mangle]
    static mut rl_directory_completion_hook:
           Option<unsafe extern "C" fn(_: *mut *mut libc::c_char)
                      -> libc::c_int>;
    #[no_mangle]
    static mut rl_directory_rewrite_hook:
           Option<unsafe extern "C" fn(_: *mut *mut libc::c_char)
                      -> libc::c_int>;
    #[no_mangle]
    static mut rl_filename_stat_hook:
           Option<unsafe extern "C" fn(_: *mut *mut libc::c_char)
                      -> libc::c_int>;
    #[no_mangle]
    static mut rl_filename_rewrite_hook:
           Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                      -> *mut libc::c_char>;
    #[no_mangle]
    static mut rl_completion_display_matches_hook:
           Option<unsafe extern "C" fn(_: *mut *mut libc::c_char,
                                       _: libc::c_int, _: libc::c_int) -> ()>;
    #[no_mangle]
    static mut rl_filename_completion_desired: libc::c_int;
    #[no_mangle]
    static mut rl_filename_quoting_desired: libc::c_int;
    #[no_mangle]
    static mut rl_filename_quoting_function:
           Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int,
                                       _: *mut libc::c_char)
                      -> *mut libc::c_char>;
    #[no_mangle]
    static mut rl_filename_dequoting_function:
           Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                      -> *mut libc::c_char>;
    #[no_mangle]
    static mut rl_char_is_quoted_p:
           Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                      -> libc::c_int>;
    #[no_mangle]
    static mut rl_attempted_completion_over: libc::c_int;
    #[no_mangle]
    static mut rl_completion_type: libc::c_int;
    #[no_mangle]
    static mut rl_completion_invoking_key: libc::c_int;
    #[no_mangle]
    static mut rl_completion_query_items: libc::c_int;
    #[no_mangle]
    static mut rl_completion_append_character: libc::c_int;
    #[no_mangle]
    static mut rl_completion_suppress_append: libc::c_int;
    #[no_mangle]
    static mut rl_completion_quote_character: libc::c_int;
    #[no_mangle]
    static mut rl_completion_found_quote: libc::c_int;
    #[no_mangle]
    static mut rl_completion_suppress_quote: libc::c_int;
    #[no_mangle]
    static mut rl_sort_completion_matches: libc::c_int;
    #[no_mangle]
    static mut rl_completion_mark_symlink_dirs: libc::c_int;
    #[no_mangle]
    static mut rl_ignore_completion_duplicates: libc::c_int;
    #[no_mangle]
    static mut rl_inhibit_completion: libc::c_int;
    #[no_mangle]
    static mut rl_persistent_signal_handlers: libc::c_int;
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
    fn add_history(_: *const libc::c_char) -> ();
    #[no_mangle]
    static mut history_base: libc::c_int;
    #[no_mangle]
    static mut history_length: libc::c_int;
    #[no_mangle]
    static mut history_max_entries: libc::c_int;
    #[no_mangle]
    static mut history_offset: libc::c_int;
    #[no_mangle]
    static mut history_lines_read_from_file: libc::c_int;
    #[no_mangle]
    static mut history_lines_written_to_file: libc::c_int;
    #[no_mangle]
    static mut history_expansion_char: libc::c_char;
    #[no_mangle]
    static mut history_subst_char: libc::c_char;
    #[no_mangle]
    static mut history_word_delimiters: *mut libc::c_char;
    #[no_mangle]
    static mut history_comment_char: libc::c_char;
    #[no_mangle]
    static mut history_no_expand_chars: *mut libc::c_char;
    #[no_mangle]
    static mut history_search_delimiter_chars: *mut libc::c_char;
    #[no_mangle]
    static mut history_quotes_inhibit_expansion: libc::c_int;
    #[no_mangle]
    static mut history_write_timestamps: libc::c_int;
    #[no_mangle]
    static mut history_multiline_entries: libc::c_int;
    #[no_mangle]
    static mut history_file_version: libc::c_int;
    #[no_mangle]
    static mut max_input_history: libc::c_int;
    #[no_mangle]
    static mut history_inhibit_expansion_function:
           Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                      -> libc::c_int>;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub const UNDO_DELETE: undo_code = 0;
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
pub const UNDO_INSERT: undo_code = 1;
pub type rl_hook_func_t = unsafe extern "C" fn() -> libc::c_int;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct undo_list {
    pub next: *mut undo_list,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub text: *mut libc::c_char,
    pub what: undo_code,
}
pub type rl_icppfunc_t =
    unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int;
pub type rl_getc_func_t = unsafe extern "C" fn(_: *mut FILE) -> libc::c_int;
pub type rl_linebuf_func_t =
    unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _keymap_entry {
    pub type_0: libc::c_char,
    pub function: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                             -> libc::c_int>,
}
pub type rl_voidfunc_t = unsafe extern "C" fn() -> ();
pub type undo_code = libc::c_uint;
pub type rl_quote_func_t =
    unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int,
                         _: *mut libc::c_char) -> *mut libc::c_char;
pub type rl_compdisp_func_t =
    unsafe extern "C" fn(_: *mut *mut libc::c_char, _: libc::c_int,
                         _: libc::c_int) -> ();
pub type tilde_hook_func_t =
    unsafe extern "C" fn(_: *mut libc::c_char) -> *mut libc::c_char;
pub type KEYMAP_ENTRY_ARRAY = [KEYMAP_ENTRY; 257];
pub type rl_compentry_func_t =
    unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int)
        -> *mut libc::c_char;
pub type rl_dequote_func_t =
    unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
        -> *mut libc::c_char;
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
pub type rl_completion_func_t =
    unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                         _: libc::c_int) -> *mut *mut libc::c_char;
pub const UNDO_END: undo_code = 3;
pub type rl_command_func_t =
    unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> libc::c_int;
pub const UNDO_BEGIN: undo_code = 2;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _funmap {
    pub name: *const libc::c_char,
    pub function: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                             -> libc::c_int>,
}
pub type rl_compignore_func_t =
    unsafe extern "C" fn(_: *mut *mut libc::c_char) -> libc::c_int;
pub type rl_cpvfunc_t = unsafe extern "C" fn() -> *mut libc::c_char;
pub type UNDO_LIST = undo_list;
pub type FUNMAP = _funmap;
pub type Keymap = *mut KEYMAP_ENTRY;
pub type rl_vintfunc_t = unsafe extern "C" fn(_: libc::c_int) -> ();
pub type KEYMAP_ENTRY = _keymap_entry;
static mut globalL: *mut lua_State_0 =
    unsafe { 0 as *const lua_State_0 as *mut lua_State_0 };
static mut progname: *const libc::c_char =
    unsafe { b"lua\x00" as *const u8 as *const libc::c_char };
unsafe extern "C" fn lstop(mut L: *mut lua_State_0, mut ar: *mut lua_Debug)
 -> () {
    lua_sethook(L, None, 0i32, 0i32);
    luaL_error(L, b"interrupted!\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn laction(mut i: libc::c_int) -> () {
    signal(i, None);
    lua_sethook(globalL, Some(lstop),
                1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 3i32, 1i32);
}
unsafe extern "C" fn print_usage(mut badoption: *const libc::c_char) -> () {
    fprintf(stderr, b"%s: \x00" as *const u8 as *const libc::c_char,
            progname);
    fflush(stderr);
    if *badoption.offset(1isize) as libc::c_int == 'e' as i32 ||
           *badoption.offset(1isize) as libc::c_int == 'l' as i32 {
        fprintf(stderr,
                b"\'%s\' needs argument\n\x00" as *const u8 as
                    *const libc::c_char, badoption);
        fflush(stderr);
    } else {
        fprintf(stderr,
                b"unrecognized option \'%s\'\n\x00" as *const u8 as
                    *const libc::c_char, badoption);
        fflush(stderr);
    }
    fprintf(stderr,
            b"usage: %s [options] [script [args]]\nAvailable options are:\n  -e stat  execute string \'stat\'\n  -i       enter interactive mode after executing \'script\'\n  -l name  require library \'name\'\n  -v       show version information\n  -E       ignore environment variables\n  --       stop handling options\n  -        stop handling options and execute stdin\n\x00"
                as *const u8 as *const libc::c_char, progname);
    fflush(stderr);
}
unsafe extern "C" fn l_message(mut pname: *const libc::c_char,
                               mut msg: *const libc::c_char) -> () {
    if !pname.is_null() {
        fprintf(stderr, b"%s: \x00" as *const u8 as *const libc::c_char,
                pname);
        fflush(stderr);
    }
    fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char, msg);
    fflush(stderr);
}
unsafe extern "C" fn report(mut L: *mut lua_State_0, mut status: libc::c_int)
 -> libc::c_int {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    if status != 0i32 {
        msg = lua_tolstring(L, -1i32, 0 as *mut size_t);
        l_message(progname, msg);
        lua_settop(L, -1i32 - 1i32);
    }
    return status;
}
unsafe extern "C" fn msghandler(mut L: *mut lua_State_0) -> libc::c_int {
    let mut msg: *const libc::c_char =
        lua_tolstring(L, 1i32, 0 as *mut size_t);
    if msg.is_null() {
        if 0 !=
               luaL_callmeta(L, 1i32,
                             b"__tostring\x00" as *const u8 as
                                 *const libc::c_char) &&
               lua_type(L, -1i32) == 4i32 {
            return 1i32
        } else {
            msg =
                lua_pushfstring(L,
                                b"(error object is a %s value)\x00" as
                                    *const u8 as *const libc::c_char,
                                lua_typename(L, lua_type(L, 1i32)))
        }
    }
    luaL_traceback(L, L, msg, 1i32);
    return 1i32;
}
unsafe extern "C" fn docall(mut L: *mut lua_State_0, mut narg: libc::c_int,
                            mut nres: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut base: libc::c_int = lua_gettop(L) - narg;
    lua_pushcclosure(L, Some(msghandler), 0i32);
    lua_rotate(L, base, 1i32);
    globalL = L;
    signal(2i32, Some(laction));
    status = lua_pcallk(L, narg, nres, base, 0i32 as lua_KContext, None);
    signal(2i32, None);
    lua_rotate(L, base, -1i32);
    lua_settop(L, -1i32 - 1i32);
    return status;
}
unsafe extern "C" fn print_version() -> () {
    fwrite(b"Lua 5.3.4  Copyright (C) 1994-2017 Lua.org, PUC-Rio\x00" as
               *const u8 as *const libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
           strlen(b"Lua 5.3.4  Copyright (C) 1994-2017 Lua.org, PUC-Rio\x00"
                      as *const u8 as *const libc::c_char), stdout);
    fwrite(b"\n\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
           1i32 as size_t, stdout);
    fflush(stdout);
}
unsafe extern "C" fn createargtable(mut L: *mut lua_State_0,
                                    mut argv: *mut *mut libc::c_char,
                                    mut argc: libc::c_int,
                                    mut script: libc::c_int) -> () {
    let mut i: libc::c_int = 0;
    let mut narg: libc::c_int = 0;
    if script == argc { script = 0i32 }
    narg = argc - (script + 1i32);
    lua_createtable(L, narg, script + 1i32);
    i = 0i32;
    while i < argc {
        lua_pushstring(L, *argv.offset(i as isize));
        lua_rawseti(L, -2i32, (i - script) as lua_Integer);
        i += 1
    }
    lua_setglobal(L, b"arg\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dochunk(mut L: *mut lua_State_0, mut status: libc::c_int)
 -> libc::c_int {
    if status == 0i32 { status = docall(L, 0i32, 0i32) }
    return report(L, status);
}
unsafe extern "C" fn dofile(mut L: *mut lua_State_0,
                            mut name: *const libc::c_char) -> libc::c_int {
    return dochunk(L, luaL_loadfilex(L, name, 0 as *const libc::c_char));
}
unsafe extern "C" fn dostring(mut L: *mut lua_State_0,
                              mut s: *const libc::c_char,
                              mut name: *const libc::c_char) -> libc::c_int {
    return dochunk(L,
                   luaL_loadbufferx(L, s, strlen(s), name,
                                    0 as *const libc::c_char));
}
unsafe extern "C" fn dolibrary(mut L: *mut lua_State_0,
                               mut name: *const libc::c_char) -> libc::c_int {
    let mut status: libc::c_int = 0;
    lua_getglobal(L, b"require\x00" as *const u8 as *const libc::c_char);
    lua_pushstring(L, name);
    status = docall(L, 1i32, 1i32);
    if status == 0i32 { lua_setglobal(L, name); }
    return report(L, status);
}
unsafe extern "C" fn get_prompt(mut L: *mut lua_State_0,
                                mut firstline: libc::c_int)
 -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    lua_getglobal(L,
                  if 0 != firstline {
                      b"_PROMPT\x00" as *const u8 as *const libc::c_char
                  } else {
                      b"_PROMPT2\x00" as *const u8 as *const libc::c_char
                  });
    p = lua_tolstring(L, -1i32, 0 as *mut size_t);
    if p.is_null() {
        p =
            if 0 != firstline {
                b"> \x00" as *const u8 as *const libc::c_char
            } else { b">> \x00" as *const u8 as *const libc::c_char }
    }
    return p;
}
unsafe extern "C" fn incomplete(mut L: *mut lua_State_0,
                                mut status: libc::c_int) -> libc::c_int {
    if status == 3i32 {
        let mut lmsg: size_t = 0;
        let mut msg: *const libc::c_char = lua_tolstring(L, -1i32, &mut lmsg);
        if lmsg >=
               (::std::mem::size_of::<[libc::c_char; 6]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong)
               &&
               strcmp(msg.offset(lmsg as
                                     isize).offset(-((::std::mem::size_of::<[libc::c_char; 6]>()
                                                          as
                                                          libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_sub(1i32
                                                                                                                          as
                                                                                                                          libc::c_ulong)
                                                         as isize)),
                      b"<eof>\x00" as *const u8 as *const libc::c_char) ==
                   0i32 {
            lua_settop(L, -1i32 - 1i32);
            return 1i32
        }
    }
    return 0i32;
}
unsafe extern "C" fn pushline(mut L: *mut lua_State_0,
                              mut firstline: libc::c_int) -> libc::c_int {
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut b: *mut libc::c_char = buffer.as_mut_ptr();
    let mut l: size_t = 0;
    let mut prmt: *const libc::c_char = get_prompt(L, firstline);
    b = readline(prmt);
    let mut readstatus: libc::c_int =
        (b != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    if readstatus == 0i32 {
        return 0i32
    } else {
        lua_settop(L, -1i32 - 1i32);
        l = strlen(b);
        if l > 0i32 as libc::c_ulong &&
               *b.offset(l.wrapping_sub(1i32 as libc::c_ulong) as isize) as
                   libc::c_int == '\n' as i32 {
            l = l.wrapping_sub(1);
            *b.offset(l as isize) = '\u{0}' as i32 as libc::c_char
        }
        if 0 != firstline && *b.offset(0isize) as libc::c_int == '=' as i32 {
            lua_pushfstring(L,
                            b"return %s\x00" as *const u8 as
                                *const libc::c_char, b.offset(1isize));
        } else { lua_pushlstring(L, b, l); }
        free(b as *mut libc::c_void);
        return 1i32
    };
}
unsafe extern "C" fn addreturn(mut L: *mut lua_State_0) -> libc::c_int {
    let mut line: *const libc::c_char =
        lua_tolstring(L, -1i32, 0 as *mut size_t);
    let mut retline: *const libc::c_char =
        lua_pushfstring(L,
                        b"return %s;\x00" as *const u8 as *const libc::c_char,
                        line);
    let mut status: libc::c_int =
        luaL_loadbufferx(L, retline, strlen(retline),
                         b"=stdin\x00" as *const u8 as *const libc::c_char,
                         0 as *const libc::c_char);
    if status == 0i32 {
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        if *line.offset(0isize) as libc::c_int != '\u{0}' as i32 {
            add_history(line);
        }
    } else { lua_settop(L, -2i32 - 1i32); }
    return status;
}
unsafe extern "C" fn multiline(mut L: *mut lua_State_0) -> libc::c_int {
    loop  {
        let mut len: size_t = 0;
        let mut line: *const libc::c_char = lua_tolstring(L, 1i32, &mut len);
        let mut status: libc::c_int =
            luaL_loadbufferx(L, line, len,
                             b"=stdin\x00" as *const u8 as
                                 *const libc::c_char,
                             0 as *const libc::c_char);
        if 0 == incomplete(L, status) || 0 == pushline(L, 0i32) {
            add_history(line);
            return status
        } else {
            lua_pushstring(L, b"\n\x00" as *const u8 as *const libc::c_char);
            lua_rotate(L, -2i32, 1i32);
            lua_concat(L, 3i32);
        }
    };
}
unsafe extern "C" fn loadline(mut L: *mut lua_State_0) -> libc::c_int {
    let mut status: libc::c_int = 0;
    lua_settop(L, 0i32);
    if 0 == pushline(L, 1i32) {
        return -1i32
    } else {
        status = addreturn(L);
        if status != 0i32 { status = multiline(L) }
        lua_rotate(L, 1i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        if lua_gettop(L) == 1i32 {
        } else {
            __assert_fail(b"lua_gettop(L) == 1\x00" as *const u8 as
                              *const libc::c_char,
                          b"lua.c\x00" as *const u8 as *const libc::c_char,
                          381i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 26],
                                                    &[libc::c_char; 26]>(b"int loadline(lua_State *)\x00")).as_ptr());
        };
        return status
    };
}
unsafe extern "C" fn l_print(mut L: *mut lua_State_0) -> () {
    let mut n: libc::c_int = lua_gettop(L);
    if n > 0i32 {
        luaL_checkstack(L, 20i32,
                        b"too many results to print\x00" as *const u8 as
                            *const libc::c_char);
        lua_getglobal(L, b"print\x00" as *const u8 as *const libc::c_char);
        lua_rotate(L, 1i32, 1i32);
        if lua_pcallk(L, n, 0i32, 0i32, 0i32 as lua_KContext, None) != 0i32 {
            l_message(progname,
                      lua_pushfstring(L,
                                      b"error calling \'print\' (%s)\x00" as
                                          *const u8 as *const libc::c_char,
                                      lua_tolstring(L, -1i32,
                                                    0 as *mut size_t)));
        }
    };
}
unsafe extern "C" fn doREPL(mut L: *mut lua_State_0) -> () {
    let mut status: libc::c_int = 0;
    let mut oldprogname: *const libc::c_char = progname;
    progname = 0 as *const libc::c_char;
    loop  {
        status = loadline(L);
        if !(status != -1i32) { break ; }
        if status == 0i32 { status = docall(L, 0i32, -1i32) }
        if status == 0i32 { l_print(L); } else { report(L, status); }
    }
    lua_settop(L, 0i32);
    fwrite(b"\n\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
           1i32 as size_t, stdout);
    fflush(stdout);
    progname = oldprogname;
}
unsafe extern "C" fn pushargs(mut L: *mut lua_State_0) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if lua_getglobal(L, b"arg\x00" as *const u8 as *const libc::c_char) !=
           5i32 {
        luaL_error(L,
                   b"\'arg\' is not a table\x00" as *const u8 as
                       *const libc::c_char);
    }
    n = luaL_len(L, -1i32) as libc::c_int;
    luaL_checkstack(L, n + 3i32,
                    b"too many arguments to script\x00" as *const u8 as
                        *const libc::c_char);
    i = 1i32;
    while i <= n { lua_rawgeti(L, -i, i as lua_Integer); i += 1 }
    lua_rotate(L, -i, -1i32);
    lua_settop(L, -1i32 - 1i32);
    return n;
}
unsafe extern "C" fn handle_script(mut L: *mut lua_State_0,
                                   mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut fname: *const libc::c_char = *argv.offset(0isize);
    if strcmp(fname, b"-\x00" as *const u8 as *const libc::c_char) == 0i32 &&
           strcmp(*argv.offset(-1i32 as isize),
                  b"--\x00" as *const u8 as *const libc::c_char) != 0i32 {
        fname = 0 as *const libc::c_char
    }
    status = luaL_loadfilex(L, fname, 0 as *const libc::c_char);
    if status == 0i32 { n = pushargs(L); status = docall(L, n, -1i32) }
    return report(L, status);
}
unsafe extern "C" fn collectargs(mut argv: *mut *mut libc::c_char,
                                 mut first: *mut libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut args: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    i = 1i32;
    while !(*argv.offset(i as isize)).is_null() {
        *first = i;
        if *(*argv.offset(i as isize)).offset(0isize) as libc::c_int !=
               '-' as i32 {
            return args
        } else {
            match *(*argv.offset(i as isize)).offset(1isize) as libc::c_int {
                45 => {
                    if *(*argv.offset(i as isize)).offset(2isize) as
                           libc::c_int != '\u{0}' as i32 {
                        return 1i32
                    } else { *first = i + 1i32; return args }
                }
                0 => { return args }
                69 => {
                    if *(*argv.offset(i as isize)).offset(2isize) as
                           libc::c_int != '\u{0}' as i32 {
                        return 1i32
                    } else { args |= 16i32 }
                    current_block = 16668937799742929182;
                }
                105 => { args |= 2i32; current_block = 15610786168616941354; }
                118 => { current_block = 15610786168616941354; }
                101 => { args |= 8i32; current_block = 16699941670520535354; }
                108 => { current_block = 16699941670520535354; }
                _ => { return 1i32 }
            }
            match current_block {
                16699941670520535354 => {
                    if *(*argv.offset(i as isize)).offset(2isize) as
                           libc::c_int == '\u{0}' as i32 {
                        i += 1;
                        if (*argv.offset(i as isize)).is_null() ||
                               *(*argv.offset(i as isize)).offset(0isize) as
                                   libc::c_int == '-' as i32 {
                            return 1i32
                        }
                    }
                }
                15610786168616941354 => {
                    if *(*argv.offset(i as isize)).offset(2isize) as
                           libc::c_int != '\u{0}' as i32 {
                        return 1i32
                    } else { args |= 4i32 }
                }
                _ => { }
            }
            i += 1
        }
    }
    *first = i;
    return args;
}
unsafe extern "C" fn runargs(mut L: *mut lua_State_0,
                             mut argv: *mut *mut libc::c_char,
                             mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < n {
        let mut option: libc::c_int =
            *(*argv.offset(i as isize)).offset(1isize) as libc::c_int;
        if *(*argv.offset(i as isize)).offset(0isize) as libc::c_int ==
               '-' as i32 {
        } else {
            __assert_fail(b"argv[i][0] == \'-\'\x00" as *const u8 as
                              *const libc::c_char,
                          b"lua.c\x00" as *const u8 as *const libc::c_char,
                          519i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 39],
                                                    &[libc::c_char; 39]>(b"int runargs(lua_State *, char **, int)\x00")).as_ptr());
        };
        if option == 'e' as i32 || option == 'l' as i32 {
            let mut status: libc::c_int = 0;
            let mut extra: *const libc::c_char =
                (*argv.offset(i as isize)).offset(2isize);
            if *extra as libc::c_int == '\u{0}' as i32 {
                i += 1;
                extra = *argv.offset(i as isize)
            }
            if !extra.is_null() {
            } else {
                __assert_fail(b"extra != ((void*)0)\x00" as *const u8 as
                                  *const libc::c_char,
                              b"lua.c\x00" as *const u8 as
                                  *const libc::c_char, 524i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 39],
                                                        &[libc::c_char; 39]>(b"int runargs(lua_State *, char **, int)\x00")).as_ptr());
            };
            status =
                if option == 'e' as i32 {
                    dostring(L, extra,
                             b"=(command line)\x00" as *const u8 as
                                 *const libc::c_char)
                } else { dolibrary(L, extra) };
            if status != 0i32 { return 0i32 }
        }
        i += 1
    }
    return 1i32;
}
unsafe extern "C" fn handle_luainit(mut L: *mut lua_State_0) -> libc::c_int {
    let mut name: *const libc::c_char =
        b"=LUA_INIT_5_3\x00" as *const u8 as *const libc::c_char;
    let mut init: *const libc::c_char = getenv(name.offset(1isize));
    if init.is_null() {
        name = b"=LUA_INIT\x00" as *const u8 as *const libc::c_char;
        init = getenv(name.offset(1isize))
    }
    if init.is_null() {
        return 0i32
    } else if *init.offset(0isize) as libc::c_int == '@' as i32 {
        return dofile(L, init.offset(1isize))
    } else { return dostring(L, init, name) };
}
unsafe extern "C" fn pmain(mut L: *mut lua_State_0) -> libc::c_int {
    let mut argc: libc::c_int =
        lua_tointegerx(L, 1i32, 0 as *mut libc::c_int) as libc::c_int;
    let mut argv: *mut *mut libc::c_char =
        lua_touserdata(L, 2i32) as *mut *mut libc::c_char;
    let mut script: libc::c_int = 0;
    let mut args: libc::c_int = collectargs(argv, &mut script);
    luaL_checkversion_(L, 503i32 as lua_Number,
                       (::std::mem::size_of::<lua_Integer>() as
                            libc::c_ulong).wrapping_mul(16i32 as
                                                            libc::c_ulong).wrapping_add(::std::mem::size_of::<lua_Number>()
                                                                                            as
                                                                                            libc::c_ulong));
    if !(*argv.offset(0isize)).is_null() &&
           0 != *(*argv.offset(0isize)).offset(0isize) as libc::c_int {
        progname = *argv.offset(0isize)
    }
    if args == 1i32 {
        print_usage(*argv.offset(script as isize));
        return 0i32
    } else {
        if 0 != args & 4i32 { print_version(); }
        if 0 != args & 16i32 {
            lua_pushboolean(L, 1i32);
            lua_setfield(L, -50000i32 - 1000i32,
                         b"LUA_NOENV\x00" as *const u8 as
                             *const libc::c_char);
        }
        luaL_openlibs(L);
        luaL_requiref(L, b"T\x00" as *const u8 as *const libc::c_char,
                      Some(luaB_opentests), 1i32);
        lua_settop(L, -1i32 - 1i32);
        createargtable(L, argv, argc, script);
        if 0 == args & 16i32 { if handle_luainit(L) != 0i32 { return 0i32 } }
        if 0 == runargs(L, argv, script) {
            return 0i32
        } else if script < argc &&
                      handle_script(L, argv.offset(script as isize)) != 0i32 {
            return 0i32
        } else {
            if 0 != args & 2i32 {
                doREPL(L);
            } else if script == argc && 0 == args & (8i32 | 4i32) {
                if 0 != isatty(0i32) {
                    print_version();
                    doREPL(L);
                } else { dofile(L, 0 as *const libc::c_char); }
            }
            lua_pushboolean(L, 1i32);
            return 1i32
        }
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut L: *mut lua_State_0 =
        lua_newstate(Some(debug_realloc),
                     &mut l_memcontrol as *mut Memcontrol_0 as
                         *mut libc::c_void);
    if L.is_null() {
        l_message(*argv.offset(0isize),
                  b"cannot create state: not enough memory\x00" as *const u8
                      as *const libc::c_char);
        return 1i32
    } else {
        lua_pushcclosure(L, Some(pmain), 0i32);
        lua_pushinteger(L, argc as lua_Integer);
        lua_pushlightuserdata(L, argv as *mut libc::c_void);
        status = lua_pcallk(L, 2i32, 1i32, 0i32, 0i32 as lua_KContext, None);
        result = lua_toboolean(L, -1i32);
        report(L, status);
        lua_close(L);
        return if 0 != result && status == 0i32 { 0i32 } else { 1i32 }
    };
}
pub fn main() -> () {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
