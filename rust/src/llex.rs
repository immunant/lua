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
    pub type BlockCnt;
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
    static luai_ctype_: [lu_byte; 257];
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaO_utf8esc(buff: *mut libc::c_char, x: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> size_t;
    #[no_mangle]
    fn luaO_hexavalue(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaM_realloc_(L: *mut lua_State_0, block: *mut libc::c_void,
                     oldsize: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn luaZ_fill(z: *mut ZIO) -> libc::c_int;
    #[no_mangle]
    fn luaG_addinfo(L: *mut lua_State_0, msg: *const libc::c_char,
                    src: *mut TString, line: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State_0, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaC_fix(L: *mut lua_State_0, o: *mut GCObject) -> ();
    #[no_mangle]
    fn luaC_step(L: *mut lua_State_0) -> ();
    #[no_mangle]
    fn luaS_new(L: *mut lua_State_0, str: *const libc::c_char)
     -> *mut TString;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State_0, str: *const libc::c_char, l: size_t)
     -> *mut TString;
    #[no_mangle]
    fn luaH_set(L: *mut lua_State_0, t: *mut Table_0, key: *const TValue)
     -> *mut TValue;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State_0, fmt: *const libc::c_char, ...)
     -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub const TK_END: RESERVED = 262;
pub const TK_SHR: RESERVED = 287;
pub const TK_REPEAT: RESERVED = 273;
pub const TK_DBCOLON: RESERVED = 288;
pub const TK_CONCAT: RESERVED = 280;
pub type ptrdiff_t = libc::c_long;
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
pub type lua_Reader =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *mut libc::c_void,
                                _: *mut size_t) -> *const libc::c_char>;
pub type Labellist = Labellist_0;
pub const TK_THEN: RESERVED = 275;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LexState {
    pub current: libc::c_int,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
    pub t: Token_0,
    pub lookahead: Token_0,
    pub fs: *mut FuncState,
    pub L: *mut lua_State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer_0,
    pub h: *mut Table_0,
    pub dyd: *mut Dyndata,
    pub source: *mut TString,
    pub envn: *mut TString,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata,
    cl: Closure,
    h: Table,
    p: Proto,
    th: lua_State,
}
pub const TK_LE: RESERVED = 284;
pub const TK_FOR: RESERVED = 264;
pub const TK_RETURN: RESERVED = 274;
pub type Labeldesc = Labeldesc_0;
pub const TK_NE: RESERVED = 285;
pub const TK_NIL: RESERVED = 270;
pub const TK_IF: RESERVED = 267;
pub const TK_ELSEIF: RESERVED = 261;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_4 {
    pub arr: *mut Vardesc_0,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
pub const TK_STRING: RESERVED = 293;
pub const TK_FALSE: RESERVED = 263;
pub const TK_TRUE: RESERVED = 276;
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
pub const TK_IN: RESERVED = 268;
pub const TK_AND: RESERVED = 257;
pub const TK_GOTO: RESERVED = 266;
pub type RESERVED = libc::c_uint;
pub const TK_BREAK: RESERVED = 258;
pub const TK_SHL: RESERVED = 286;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Token {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
pub const TK_NOT: RESERVED = 271;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Labeldesc_0 {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: lu_byte,
}
pub const TK_DO: RESERVED = 259;
pub const TK_IDIV: RESERVED = 279;
pub const TK_EQ: RESERVED = 282;
pub const TK_LOCAL: RESERVED = 269;
pub const TK_UNTIL: RESERVED = 277;
pub const TK_FUNCTION: RESERVED = 265;
pub const TK_FLT: RESERVED = 290;
pub const TK_ELSE: RESERVED = 260;
pub const TK_DOTS: RESERVED = 281;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Labellist_0 {
    pub arr: *mut Labeldesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
pub const TK_GE: RESERVED = 283;
pub type Token_0 = Token;
pub type LexState_0 = LexState;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto_0,
    pub prev: *mut FuncState,
    pub ls: *mut LexState,
    pub bl: *mut BlockCnt,
    pub pc: libc::c_int,
    pub lasttarget: libc::c_int,
    pub jpc: libc::c_int,
    pub nk: libc::c_int,
    pub np: libc::c_int,
    pub firstlocal: libc::c_int,
    pub nlocvars: libc::c_short,
    pub nactvar: lu_byte,
    pub nups: lu_byte,
    pub freereg: lu_byte,
}
pub const TK_OR: RESERVED = 272;
pub const TK_WHILE: RESERVED = 278;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Vardesc {
    pub idx: libc::c_short,
}
pub const TK_EOS: RESERVED = 289;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table,
    pub len: size_t,
    pub user_: Value_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
pub type Upvaldesc_0 = Upvaldesc;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type LocVar_0 = LocVar;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Proto {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub numparams: lu_byte,
    pub is_vararg: lu_byte,
    pub maxstacksize: lu_byte,
    pub sizeupvalues: libc::c_int,
    pub sizek: libc::c_int,
    pub sizecode: libc::c_int,
    pub sizelineinfo: libc::c_int,
    pub sizep: libc::c_int,
    pub sizelocvars: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub k: *mut TValue,
    pub code: *mut Instruction,
    pub p: *mut *mut Proto,
    pub lineinfo: *mut libc::c_int,
    pub locvars: *mut LocVar_0,
    pub upvalues: *mut Upvaldesc_0,
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
pub type Proto_0 = Proto;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
pub type CClosure_0 = CClosure;
pub type LClosure_0 = LClosure;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure_0,
    l: LClosure_0,
}
pub type Table_0 = Table;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Dyndata {
    pub actvar: unnamed_4,
    pub gt: Labellist,
    pub label: Labellist,
}
pub const TK_INT: RESERVED = 291;
pub const TK_NAME: RESERVED = 292;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union SemInfo {
    r: lua_Number,
    i: lua_Integer,
    ts: *mut TString,
}
pub type Vardesc_0 = Vardesc;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State_0,
}
pub type ZIO = Zio;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut libc::c_char,
    pub n: size_t,
    pub buffsize: size_t,
}
pub type Mbuffer_0 = Mbuffer;
#[no_mangle]
pub unsafe extern "C" fn luaX_init(mut L: *mut lua_State_0) -> () {
    let mut i: libc::c_int = 0;
    let mut e: *mut TString =
        luaS_newlstr(L, b"_ENV\x00" as *const u8 as *const libc::c_char,
                     (::std::mem::size_of::<[libc::c_char; 5]>() as
                          libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                          as
                                                          libc::c_ulong).wrapping_sub(1i32
                                                                                          as
                                                                                          libc::c_ulong));
    if (*e).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((e)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"llex.c\x00" as *const u8 as *const libc::c_char,
                      74i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"void luaX_init(lua_State *)\x00")).as_ptr());
    };
    luaC_fix(L, &mut (*(e as *mut GCUnion)).gc);
    i = 0i32;
    while i < TK_WHILE as libc::c_int - 257i32 + 1i32 {
        let mut ts: *mut TString = luaS_new(L, luaX_tokens[i as usize]);
        if (*ts).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((ts)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"llex.c\x00" as *const u8 as *const libc::c_char,
                          77i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 28],
                                                    &[libc::c_char; 28]>(b"void luaX_init(lua_State *)\x00")).as_ptr());
        };
        luaC_fix(L, &mut (*(ts as *mut GCUnion)).gc);
        (*ts).extra = (i + 1i32) as lu_byte;
        i += 1
    };
}
static mut luaX_tokens: [*const libc::c_char; 37] =
    unsafe {
        [b"and\x00" as *const u8 as *const libc::c_char,
         b"break\x00" as *const u8 as *const libc::c_char,
         b"do\x00" as *const u8 as *const libc::c_char,
         b"else\x00" as *const u8 as *const libc::c_char,
         b"elseif\x00" as *const u8 as *const libc::c_char,
         b"end\x00" as *const u8 as *const libc::c_char,
         b"false\x00" as *const u8 as *const libc::c_char,
         b"for\x00" as *const u8 as *const libc::c_char,
         b"function\x00" as *const u8 as *const libc::c_char,
         b"goto\x00" as *const u8 as *const libc::c_char,
         b"if\x00" as *const u8 as *const libc::c_char,
         b"in\x00" as *const u8 as *const libc::c_char,
         b"local\x00" as *const u8 as *const libc::c_char,
         b"nil\x00" as *const u8 as *const libc::c_char,
         b"not\x00" as *const u8 as *const libc::c_char,
         b"or\x00" as *const u8 as *const libc::c_char,
         b"repeat\x00" as *const u8 as *const libc::c_char,
         b"return\x00" as *const u8 as *const libc::c_char,
         b"then\x00" as *const u8 as *const libc::c_char,
         b"true\x00" as *const u8 as *const libc::c_char,
         b"until\x00" as *const u8 as *const libc::c_char,
         b"while\x00" as *const u8 as *const libc::c_char,
         b"//\x00" as *const u8 as *const libc::c_char,
         b"..\x00" as *const u8 as *const libc::c_char,
         b"...\x00" as *const u8 as *const libc::c_char,
         b"==\x00" as *const u8 as *const libc::c_char,
         b">=\x00" as *const u8 as *const libc::c_char,
         b"<=\x00" as *const u8 as *const libc::c_char,
         b"~=\x00" as *const u8 as *const libc::c_char,
         b"<<\x00" as *const u8 as *const libc::c_char,
         b">>\x00" as *const u8 as *const libc::c_char,
         b"::\x00" as *const u8 as *const libc::c_char,
         b"<eof>\x00" as *const u8 as *const libc::c_char,
         b"<number>\x00" as *const u8 as *const libc::c_char,
         b"<integer>\x00" as *const u8 as *const libc::c_char,
         b"<name>\x00" as *const u8 as *const libc::c_char,
         b"<string>\x00" as *const u8 as *const libc::c_char]
    };
#[no_mangle]
pub unsafe extern "C" fn luaX_setinput(mut L: *mut lua_State_0,
                                       mut ls: *mut LexState_0,
                                       mut z: *mut ZIO,
                                       mut source: *mut TString,
                                       mut firstchar: libc::c_int) -> () {
    (*ls).t.token = 0i32;
    (*ls).L = L;
    (*ls).current = firstchar;
    (*ls).lookahead.token = TK_EOS as libc::c_int;
    (*ls).z = z;
    (*ls).fs = 0 as *mut FuncState;
    (*ls).linenumber = 1i32;
    (*ls).lastline = 1i32;
    (*ls).source = source;
    (*ls).envn =
        luaS_newlstr(L, b"_ENV\x00" as *const u8 as *const libc::c_char,
                     (::std::mem::size_of::<[libc::c_char; 5]>() as
                          libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                          as
                                                          libc::c_ulong).wrapping_sub(1i32
                                                                                          as
                                                                                          libc::c_ulong));
    (*(*ls).buff).buffer =
        luaM_realloc_((*ls).L, (*(*ls).buff).buffer as *mut libc::c_void,
                      (*(*ls).buff).buffsize.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                              as
                                                              libc::c_ulong),
                      (32i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong))
            as *mut libc::c_char;
    (*(*ls).buff).buffsize = 32i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_newstring(mut ls: *mut LexState_0,
                                        mut str: *const libc::c_char,
                                        mut l: size_t) -> *mut TString {
    let mut L: *mut lua_State_0 = (*ls).L;
    let mut o: *mut TValue = 0 as *mut TValue;
    let mut ts: *mut TString = luaS_newlstr(L, str, l);
    let fresh0 = (*L).top;
    (*L).top = (*L).top.offset(1);
    let mut io: *mut TValue = fresh0;
    let mut x_: *mut TString = ts;
    if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
    } else {
        __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                          *const libc::c_char,
                      b"llex.c\x00" as *const u8 as *const libc::c_char,
                      132i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"TString *luaX_newstring(LexState *, const char *, size_t)\x00")).as_ptr());
    };
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
    (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
    if 0 == (*io).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*io).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as *const u8
                                     as *const libc::c_char,
                                 b"llex.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 132i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 58],
                                                           &[libc::c_char; 58]>(b"TString *luaX_newstring(LexState *, const char *, size_t)\x00")).as_ptr());
               };
               (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int &&
                   (L.is_null() ||
                        {
                            if 0 != (*io).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"llex.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              132i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 58],
                                                                        &[libc::c_char; 58]>(b"TString *luaX_newstring(LexState *, const char *, size_t)\x00")).as_ptr());
                            };
                            0 !=
                                ((*(*io).value_.gc).marked as libc::c_int ^
                                     (1i32 << 0i32 | 1i32 << 1i32)) &
                                    ((*(*L).l_G).currentwhite as libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32))
                        })
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"llex.c\x00" as *const u8 as *const libc::c_char,
                          132i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"TString *luaX_newstring(LexState *, const char *, size_t)\x00")).as_ptr());
        };
    };
    o = luaH_set(L, (*ls).h, (*L).top.offset(-1isize) as *const TValue);
    if (*o).tt_ == 0i32 {
        let mut io_0: *mut TValue = o;
        (*io_0).value_.b = 1i32;
        (*io_0).tt_ = 1i32;
        if (*(*L).l_G).GCdebt > 0i32 as libc::c_long { luaC_step(L); }
    } else {
        if (*(&mut (*((o as *mut libc::c_char).offset(-0isize) as
                          *mut Node)).i_key.tvk as *mut TValue as
                  *const TValue)).tt_ & 15i32 == 4i32 {
        } else {
            __assert_fail(b"((((((((((const TValue*)((&(((Node *)(((char *)((o))) - __builtin_offsetof(Node, i_val))))->i_key.tvk))))))->tt_)) & 0x0F)) == (4))\x00"
                              as *const u8 as *const libc::c_char,
                          b"llex.c\x00" as *const u8 as *const libc::c_char,
                          141i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"TString *luaX_newstring(LexState *, const char *, size_t)\x00")).as_ptr());
        };
        if (*(*(&mut (*((o as *mut libc::c_char).offset(-0isize) as
                            *mut Node)).i_key.tvk as *mut TValue as
                    *const TValue)).value_.gc).tt as libc::c_int & 15i32 ==
               4i32 {
        } else {
            __assert_fail(b"((((((((const TValue*)((&(((Node *)(((char *)((o))) - __builtin_offsetof(Node, i_val))))->i_key.tvk)))))->value_).gc)->tt) & 0x0F) == 4\x00"
                              as *const u8 as *const libc::c_char,
                          b"llex.c\x00" as *const u8 as *const libc::c_char,
                          141i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"TString *luaX_newstring(LexState *, const char *, size_t)\x00")).as_ptr());
        };
        ts =
            &mut (*((*(&mut (*((o as *mut libc::c_char).offset(-0isize) as
                                   *mut Node)).i_key.tvk as *mut TValue as
                           *const TValue)).value_.gc as *mut GCUnion)).ts as
                *mut TString_0
    }
    (*L).top = (*L).top.offset(-1isize);
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_next(mut ls: *mut LexState_0) -> () {
    (*ls).lastline = (*ls).linenumber;
    if (*ls).lookahead.token != TK_EOS as libc::c_int {
        (*ls).t = (*ls).lookahead;
        (*ls).lookahead.token = TK_EOS as libc::c_int
    } else { (*ls).t.token = llex(ls, &mut (*ls).t.seminfo) };
}
unsafe extern "C" fn llex(mut ls: *mut LexState_0, mut seminfo: *mut SemInfo)
 -> libc::c_int {
    (*(*ls).buff).n = 0i32 as size_t;
    loop  {
        match (*ls).current {
            10 | 13 => { inclinenumber(ls); }
            32 | 12 | 9 | 11 => {
                let fresh1 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh1 > 0i32 as libc::c_ulong {
                        let fresh2 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh2 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) }
            }
            45 => {
                let fresh3 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh3 > 0i32 as libc::c_ulong {
                        let fresh4 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh4 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                if (*ls).current != '-' as i32 {
                    return '-' as i32
                } else {
                    let fresh5 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current =
                        if fresh5 > 0i32 as libc::c_ulong {
                            let fresh6 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh6 as libc::c_uchar as libc::c_int
                        } else { luaZ_fill((*ls).z) };
                    if (*ls).current == '[' as i32 {
                        let mut sep: libc::c_int = skip_sep(ls);
                        (*(*ls).buff).n = 0i32 as size_t;
                        if sep >= 0i32 {
                            read_long_string(ls, 0 as *mut SemInfo, sep);
                            (*(*ls).buff).n = 0i32 as size_t;
                            continue ;
                        }
                    }
                    while !((*ls).current == '\n' as i32 ||
                                (*ls).current == '\r' as i32) &&
                              (*ls).current != -1i32 {
                        let fresh7 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current =
                            if fresh7 > 0i32 as libc::c_ulong {
                                let fresh8 = (*(*ls).z).p;
                                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                                *fresh8 as libc::c_uchar as libc::c_int
                            } else { luaZ_fill((*ls).z) }
                    }
                }
            }
            91 => {
                let mut sep_0: libc::c_int = skip_sep(ls);
                if sep_0 >= 0i32 {
                    read_long_string(ls, seminfo, sep_0);
                    return TK_STRING as libc::c_int
                } else if sep_0 != -1i32 {
                    lexerror(ls,
                             b"invalid long string delimiter\x00" as *const u8
                                 as *const libc::c_char,
                             TK_STRING as libc::c_int);
                } else { return '[' as i32 }
            }
            61 => {
                let fresh9 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh9 > 0i32 as libc::c_ulong {
                        let fresh10 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh10 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_EQ as libc::c_int
                } else { return '=' as i32 }
            }
            60 => {
                let fresh11 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh11 > 0i32 as libc::c_ulong {
                        let fresh12 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh12 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_LE as libc::c_int
                } else if 0 != check_next1(ls, '<' as i32) {
                    return TK_SHL as libc::c_int
                } else { return '<' as i32 }
            }
            62 => {
                let fresh13 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh13 > 0i32 as libc::c_ulong {
                        let fresh14 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh14 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_GE as libc::c_int
                } else if 0 != check_next1(ls, '>' as i32) {
                    return TK_SHR as libc::c_int
                } else { return '>' as i32 }
            }
            47 => {
                let fresh15 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh15 > 0i32 as libc::c_ulong {
                        let fresh16 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh16 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                if 0 != check_next1(ls, '/' as i32) {
                    return TK_IDIV as libc::c_int
                } else { return '/' as i32 }
            }
            126 => {
                let fresh17 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh17 > 0i32 as libc::c_ulong {
                        let fresh18 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh18 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_NE as libc::c_int
                } else { return '~' as i32 }
            }
            58 => {
                let fresh19 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh19 > 0i32 as libc::c_ulong {
                        let fresh20 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh20 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                if 0 != check_next1(ls, ':' as i32) {
                    return TK_DBCOLON as libc::c_int
                } else { return ':' as i32 }
            }
            34 | 39 => {
                read_string(ls, (*ls).current, seminfo);
                return TK_STRING as libc::c_int
            }
            46 => {
                save(ls, (*ls).current);
                let fresh21 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh21 > 0i32 as libc::c_ulong {
                        let fresh22 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh22 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                if 0 != check_next1(ls, '.' as i32) {
                    if 0 != check_next1(ls, '.' as i32) {
                        return TK_DOTS as libc::c_int
                    } else { return TK_CONCAT as libc::c_int }
                } else if 0 ==
                              luai_ctype_[((*ls).current + 1i32) as usize] as
                                  libc::c_int & 1i32 << 1i32 {
                    return '.' as i32
                } else { return read_numeral(ls, seminfo) }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                return read_numeral(ls, seminfo)
            }
            -1 => { return TK_EOS as libc::c_int }
            _ => {
                if 0 !=
                       luai_ctype_[((*ls).current + 1i32) as usize] as
                           libc::c_int & 1i32 << 0i32 {
                    let mut ts: *mut TString = 0 as *mut TString;
                    loop  {
                        save(ls, (*ls).current);
                        let fresh23 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current =
                            if fresh23 > 0i32 as libc::c_ulong {
                                let fresh24 = (*(*ls).z).p;
                                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                                *fresh24 as libc::c_uchar as libc::c_int
                            } else { luaZ_fill((*ls).z) };
                        if !(0 !=
                                 luai_ctype_[((*ls).current + 1i32) as usize]
                                     as libc::c_int &
                                     (1i32 << 0i32 | 1i32 << 1i32)) {
                            break ;
                        }
                    }
                    ts =
                        luaX_newstring(ls, (*(*ls).buff).buffer,
                                       (*(*ls).buff).n);
                    (*seminfo).ts = ts;
                    if (*ts).tt as libc::c_int == 4i32 | 0i32 << 4i32 &&
                           (*ts).extra as libc::c_int > 0i32 {
                        return (*ts).extra as libc::c_int - 1i32 + 257i32
                    } else { return TK_NAME as libc::c_int }
                } else {
                    let mut c: libc::c_int = (*ls).current;
                    let fresh25 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current =
                        if fresh25 > 0i32 as libc::c_ulong {
                            let fresh26 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh26 as libc::c_uchar as libc::c_int
                        } else { luaZ_fill((*ls).z) };
                    return c
                }
            }
        }
    };
}
unsafe extern "C" fn save(mut ls: *mut LexState_0, mut c: libc::c_int) -> () {
    let mut b: *mut Mbuffer_0 = (*ls).buff;
    if (*b).n.wrapping_add(1i32 as libc::c_ulong) > (*b).buffsize {
        let mut newsize: size_t = 0;
        if (*b).buffsize >=
               if (::std::mem::size_of::<size_t>() as libc::c_ulong) <
                      ::std::mem::size_of::<lua_Integer>() as libc::c_ulong {
                   !(0i32 as size_t)
               } else {
                   9223372036854775807i64 as size_t
               }.wrapping_div(2i32 as libc::c_ulong) {
            lexerror(ls,
                     b"lexical element too long\x00" as *const u8 as
                         *const libc::c_char, 0i32);
        } else {
            newsize = (*b).buffsize.wrapping_mul(2i32 as libc::c_ulong);
            (*b).buffer =
                luaM_realloc_((*ls).L, (*b).buffer as *mut libc::c_void,
                              (*b).buffsize.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                             as
                                                             libc::c_ulong),
                              newsize.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                       as libc::c_ulong)) as
                    *mut libc::c_char;
            (*b).buffsize = newsize
        }
    }
    let fresh27 = (*b).n;
    (*b).n = (*b).n.wrapping_add(1);
    *(*b).buffer.offset(fresh27 as isize) = c as libc::c_char;
}
unsafe extern "C" fn lexerror(mut ls: *mut LexState_0,
                              mut msg: *const libc::c_char,
                              mut token: libc::c_int) -> ! {
    msg = luaG_addinfo((*ls).L, msg, (*ls).source, (*ls).linenumber);
    if 0 != token {
        luaO_pushfstring((*ls).L,
                         b"%s near %s\x00" as *const u8 as
                             *const libc::c_char, msg, txtToken(ls, token));
    }
    luaD_throw((*ls).L, 3i32);
}
unsafe extern "C" fn txtToken(mut ls: *mut LexState_0, mut token: libc::c_int)
 -> *const libc::c_char {
    match token {
        292 | 293 | 290 | 291 => {
            save(ls, '\u{0}' as i32);
            return luaO_pushfstring((*ls).L,
                                    b"\'%s\'\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*(*ls).buff).buffer)
        }
        _ => { return luaX_token2str(ls, token) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaX_token2str(mut ls: *mut LexState_0,
                                        mut token: libc::c_int)
 -> *const libc::c_char {
    if token < 257i32 {
        if token == token as libc::c_uchar as libc::c_int {
        } else {
            __assert_fail(b"token == ((unsigned char)((token)))\x00" as
                              *const u8 as *const libc::c_char,
                          b"llex.c\x00" as *const u8 as *const libc::c_char,
                          85i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 44],
                                                    &[libc::c_char; 44]>(b"const char *luaX_token2str(LexState *, int)\x00")).as_ptr());
        };
        return luaO_pushfstring((*ls).L,
                                b"\'%c\'\x00" as *const u8 as
                                    *const libc::c_char, token)
    } else {
        let mut s: *const libc::c_char =
            luaX_tokens[(token - 257i32) as usize];
        if token < TK_EOS as libc::c_int {
            return luaO_pushfstring((*ls).L,
                                    b"\'%s\'\x00" as *const u8 as
                                        *const libc::c_char, s)
        } else { return s }
    };
}
unsafe extern "C" fn read_numeral(mut ls: *mut LexState_0,
                                  mut seminfo: *mut SemInfo) -> libc::c_int {
    let mut obj: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    let mut expo: *const libc::c_char =
        b"Ee\x00" as *const u8 as *const libc::c_char;
    let mut first: libc::c_int = (*ls).current;
    if 0 !=
           luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int &
               1i32 << 1i32 {
    } else {
        __assert_fail(b"(luai_ctype_[(ls->current)+1] & ((1 << (1))))\x00" as
                          *const u8 as *const libc::c_char,
                      b"llex.c\x00" as *const u8 as *const libc::c_char,
                      219i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"int read_numeral(LexState *, SemInfo *)\x00")).as_ptr());
    };
    save(ls, (*ls).current);
    let fresh28 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh28 > 0i32 as libc::c_ulong {
            let fresh29 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh29 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    if first == '0' as i32 &&
           0 != check_next2(ls, b"xX\x00" as *const u8 as *const libc::c_char)
       {
        expo = b"Pp\x00" as *const u8 as *const libc::c_char
    }
    loop  {
        if 0 != check_next2(ls, expo) {
            check_next2(ls, b"-+\x00" as *const u8 as *const libc::c_char);
        }
        if 0 !=
               luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int &
                   1i32 << 4i32 {
            save(ls, (*ls).current);
            let fresh30 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current =
                if fresh30 > 0i32 as libc::c_ulong {
                    let fresh31 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh31 as libc::c_uchar as libc::c_int
                } else { luaZ_fill((*ls).z) }
        } else {
            if !((*ls).current == '.' as i32) { break ; }
            save(ls, (*ls).current);
            let fresh32 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current =
                if fresh32 > 0i32 as libc::c_ulong {
                    let fresh33 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh33 as libc::c_uchar as libc::c_int
                } else { luaZ_fill((*ls).z) }
        }
    }
    save(ls, '\u{0}' as i32);
    if luaO_str2num((*(*ls).buff).buffer, &mut obj) == 0i32 as libc::c_ulong {
        lexerror(ls,
                 b"malformed number\x00" as *const u8 as *const libc::c_char,
                 TK_FLT as libc::c_int);
    } else if obj.tt_ == 3i32 | 1i32 << 4i32 {
        if obj.tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(b"((((&obj))->tt_) == ((3 | (1 << 4))))\x00" as
                              *const u8 as *const libc::c_char,
                          b"llex.c\x00" as *const u8 as *const libc::c_char,
                          236i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_char; 40]>(b"int read_numeral(LexState *, SemInfo *)\x00")).as_ptr());
        };
        (*seminfo).i = obj.value_.i;
        return TK_INT as libc::c_int
    } else {
        if obj.tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(b"((((&obj))->tt_) == ((3 | (0 << 4))))\x00" as
                              *const u8 as *const libc::c_char,
                          b"llex.c\x00" as *const u8 as *const libc::c_char,
                          240i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_char; 40]>(b"int read_numeral(LexState *, SemInfo *)\x00")).as_ptr());
        };
        if obj.tt_ == 3i32 | 0i32 << 4i32 {
        } else {
            __assert_fail(b"((((&obj))->tt_) == ((3 | (0 << 4))))\x00" as
                              *const u8 as *const libc::c_char,
                          b"llex.c\x00" as *const u8 as *const libc::c_char,
                          241i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_char; 40]>(b"int read_numeral(LexState *, SemInfo *)\x00")).as_ptr());
        };
        (*seminfo).r = obj.value_.n;
        return TK_FLT as libc::c_int
    };
}
unsafe extern "C" fn check_next2(mut ls: *mut LexState_0,
                                 mut set: *const libc::c_char)
 -> libc::c_int {
    if *set.offset(2isize) as libc::c_int == '\u{0}' as i32 {
    } else {
        __assert_fail(b"set[2] == \'\\0\'\x00" as *const u8 as
                          *const libc::c_char,
                      b"llex.c\x00" as *const u8 as *const libc::c_char,
                      201i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"int check_next2(LexState *, const char *)\x00")).as_ptr());
    };
    if (*ls).current == *set.offset(0isize) as libc::c_int ||
           (*ls).current == *set.offset(1isize) as libc::c_int {
        save(ls, (*ls).current);
        let fresh34 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current =
            if fresh34 > 0i32 as libc::c_ulong {
                let fresh35 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh35 as libc::c_uchar as libc::c_int
            } else { luaZ_fill((*ls).z) };
        return 1i32
    } else { return 0i32 };
}
unsafe extern "C" fn check_next1(mut ls: *mut LexState_0, mut c: libc::c_int)
 -> libc::c_int {
    if (*ls).current == c {
        let fresh36 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current =
            if fresh36 > 0i32 as libc::c_ulong {
                let fresh37 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh37 as libc::c_uchar as libc::c_int
            } else { luaZ_fill((*ls).z) };
        return 1i32
    } else { return 0i32 };
}
unsafe extern "C" fn read_string(mut ls: *mut LexState_0,
                                 mut del: libc::c_int,
                                 mut seminfo: *mut SemInfo) -> () {
    let mut current_block: u64;
    save(ls, (*ls).current);
    let fresh38 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh38 > 0i32 as libc::c_ulong {
            let fresh39 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh39 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    's_4:
        while (*ls).current != del {
            match (*ls).current {
                -1 => {
                    lexerror(ls,
                             b"unfinished string\x00" as *const u8 as
                                 *const libc::c_char, TK_EOS as libc::c_int);
                }
                10 | 13 => {
                    lexerror(ls,
                             b"unfinished string\x00" as *const u8 as
                                 *const libc::c_char,
                             TK_STRING as libc::c_int);
                }
                92 => {
                    let mut c: libc::c_int = 0;
                    save(ls, (*ls).current);
                    let fresh40 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current =
                        if fresh40 > 0i32 as libc::c_ulong {
                            let fresh41 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh41 as libc::c_uchar as libc::c_int
                        } else { luaZ_fill((*ls).z) };
                    match (*ls).current {
                        97 => {
                            c = '\u{7}' as i32;
                            current_block = 979468731264905436;
                        }
                        98 => {
                            c = '\u{8}' as i32;
                            current_block = 979468731264905436;
                        }
                        102 => {
                            c = '\u{c}' as i32;
                            current_block = 979468731264905436;
                        }
                        110 => {
                            c = '\n' as i32;
                            current_block = 979468731264905436;
                        }
                        114 => {
                            c = '\r' as i32;
                            current_block = 979468731264905436;
                        }
                        116 => {
                            c = '\t' as i32;
                            current_block = 979468731264905436;
                        }
                        118 => {
                            c = '\u{b}' as i32;
                            current_block = 979468731264905436;
                        }
                        120 => {
                            c = readhexaesc(ls);
                            current_block = 979468731264905436;
                        }
                        117 => { utf8esc(ls); continue ; }
                        10 | 13 => {
                            inclinenumber(ls);
                            c = '\n' as i32;
                            current_block = 14478200684148302768;
                        }
                        92 | 34 | 39 => {
                            c = (*ls).current;
                            current_block = 979468731264905436;
                        }
                        -1 => { continue ; }
                        122 => {
                            (*(*ls).buff).n =
                                ((*(*ls).buff).n as
                                     libc::c_ulong).wrapping_sub(1i32 as
                                                                     libc::c_ulong)
                                    as size_t as size_t;
                            let fresh42 = (*(*ls).z).n;
                            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                            (*ls).current =
                                if fresh42 > 0i32 as libc::c_ulong {
                                    let fresh43 = (*(*ls).z).p;
                                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                                    *fresh43 as libc::c_uchar as libc::c_int
                                } else { luaZ_fill((*ls).z) };
                            loop  {
                                if !(0 !=
                                         luai_ctype_[((*ls).current + 1i32) as
                                                         usize] as libc::c_int
                                             & 1i32 << 3i32) {
                                    continue 's_4 ;
                                }
                                if (*ls).current == '\n' as i32 ||
                                       (*ls).current == '\r' as i32 {
                                    inclinenumber(ls);
                                } else {
                                    let fresh44 = (*(*ls).z).n;
                                    (*(*ls).z).n =
                                        (*(*ls).z).n.wrapping_sub(1);
                                    (*ls).current =
                                        if fresh44 > 0i32 as libc::c_ulong {
                                            let fresh45 = (*(*ls).z).p;
                                            (*(*ls).z).p =
                                                (*(*ls).z).p.offset(1);
                                            *fresh45 as libc::c_uchar as
                                                libc::c_int
                                        } else { luaZ_fill((*ls).z) }
                                }
                            }
                        }
                        _ => {
                            esccheck(ls,
                                     luai_ctype_[((*ls).current + 1i32) as
                                                     usize] as libc::c_int &
                                         1i32 << 1i32,
                                     b"invalid escape sequence\x00" as
                                         *const u8 as *const libc::c_char);
                            c = readdecesc(ls);
                            current_block = 14478200684148302768;
                        }
                    }
                    match current_block {
                        979468731264905436 => {
                            let fresh46 = (*(*ls).z).n;
                            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                            (*ls).current =
                                if fresh46 > 0i32 as libc::c_ulong {
                                    let fresh47 = (*(*ls).z).p;
                                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                                    *fresh47 as libc::c_uchar as libc::c_int
                                } else { luaZ_fill((*ls).z) }
                        }
                        _ => { }
                    }
                    (*(*ls).buff).n =
                        ((*(*ls).buff).n as
                             libc::c_ulong).wrapping_sub(1i32 as
                                                             libc::c_ulong) as
                            size_t as size_t;
                    save(ls, c);
                }
                _ => {
                    save(ls, (*ls).current);
                    let fresh48 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current =
                        if fresh48 > 0i32 as libc::c_ulong {
                            let fresh49 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh49 as libc::c_uchar as libc::c_int
                        } else { luaZ_fill((*ls).z) }
                }
            }
        }
    save(ls, (*ls).current);
    let fresh50 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh50 > 0i32 as libc::c_ulong {
            let fresh51 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh51 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    (*seminfo).ts =
        luaX_newstring(ls, (*(*ls).buff).buffer.offset(1isize),
                       (*(*ls).buff).n.wrapping_sub(2i32 as libc::c_ulong));
}
unsafe extern "C" fn readdecesc(mut ls: *mut LexState_0) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0i32;
    i = 0i32;
    while i < 3i32 &&
              0 !=
                  luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int
                      & 1i32 << 1i32 {
        r = 10i32 * r + (*ls).current - '0' as i32;
        save(ls, (*ls).current);
        let fresh52 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current =
            if fresh52 > 0i32 as libc::c_ulong {
                let fresh53 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh53 as libc::c_uchar as libc::c_int
            } else { luaZ_fill((*ls).z) };
        i += 1
    }
    esccheck(ls, (r <= 127i32 * 2i32 + 1i32) as libc::c_int,
             b"decimal escape too large\x00" as *const u8 as
                 *const libc::c_char);
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(i as libc::c_ulong) as
            size_t as size_t;
    return r;
}
unsafe extern "C" fn esccheck(mut ls: *mut LexState_0, mut c: libc::c_int,
                              mut msg: *const libc::c_char) -> () {
    if 0 == c {
        if (*ls).current != -1i32 {
            save(ls, (*ls).current);
            let fresh54 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current =
                if fresh54 > 0i32 as libc::c_ulong {
                    let fresh55 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh55 as libc::c_uchar as libc::c_int
                } else { luaZ_fill((*ls).z) }
        }
        lexerror(ls, msg, TK_STRING as libc::c_int);
    } else { return; };
}
unsafe extern "C" fn inclinenumber(mut ls: *mut LexState_0) -> () {
    let mut old: libc::c_int = (*ls).current;
    if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
    } else {
        __assert_fail(b"(ls->current == \'\\n\' || ls->current == \'\\r\')\x00"
                          as *const u8 as *const libc::c_char,
                      b"llex.c\x00" as *const u8 as *const libc::c_char,
                      154i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"void inclinenumber(LexState *)\x00")).as_ptr());
    };
    let fresh56 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh56 > 0i32 as libc::c_ulong {
            let fresh57 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh57 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    if ((*ls).current == '\n' as i32 || (*ls).current == '\r' as i32) &&
           (*ls).current != old {
        let fresh58 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current =
            if fresh58 > 0i32 as libc::c_ulong {
                let fresh59 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh59 as libc::c_uchar as libc::c_int
            } else { luaZ_fill((*ls).z) }
    }
    (*ls).linenumber += 1;
    if (*ls).linenumber >= 2147483647i32 {
        lexerror(ls,
                 b"chunk has too many lines\x00" as *const u8 as
                     *const libc::c_char, 0i32);
    } else { return; };
}
unsafe extern "C" fn utf8esc(mut ls: *mut LexState_0) -> () {
    let mut buff: [libc::c_char; 8] = [0; 8];
    let mut n: libc::c_int = luaO_utf8esc(buff.as_mut_ptr(), readutf8esc(ls));
    while n > 0i32 {
        save(ls, buff[(8i32 - n) as usize] as libc::c_int);
        n -= 1
    };
}
unsafe extern "C" fn readutf8esc(mut ls: *mut LexState_0) -> libc::c_ulong {
    let mut r: libc::c_ulong = 0;
    let mut i: libc::c_int = 4i32;
    save(ls, (*ls).current);
    let fresh60 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh60 > 0i32 as libc::c_ulong {
            let fresh61 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh61 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    esccheck(ls, ((*ls).current == '{' as i32) as libc::c_int,
             b"missing \'{\'\x00" as *const u8 as *const libc::c_char);
    r = gethexa(ls) as libc::c_ulong;
    loop  {
        save(ls, (*ls).current);
        let fresh62 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current =
            if fresh62 > 0i32 as libc::c_ulong {
                let fresh63 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh63 as libc::c_uchar as libc::c_int
            } else { luaZ_fill((*ls).z) };
        if !(0 !=
                 luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int &
                     1i32 << 4i32) {
            break ;
        }
        i += 1;
        r =
            (r <<
                 4i32).wrapping_add(luaO_hexavalue((*ls).current) as
                                        libc::c_ulong);
        esccheck(ls, (r <= 1114111i32 as libc::c_ulong) as libc::c_int,
                 b"UTF-8 value too large\x00" as *const u8 as
                     *const libc::c_char);
    }
    esccheck(ls, ((*ls).current == '}' as i32) as libc::c_int,
             b"missing \'}\'\x00" as *const u8 as *const libc::c_char);
    let fresh64 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh64 > 0i32 as libc::c_ulong {
            let fresh65 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh65 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(i as libc::c_ulong) as
            size_t as size_t;
    return r;
}
unsafe extern "C" fn gethexa(mut ls: *mut LexState_0) -> libc::c_int {
    save(ls, (*ls).current);
    let fresh66 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh66 > 0i32 as libc::c_ulong {
            let fresh67 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh67 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    esccheck(ls,
             luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int &
                 1i32 << 4i32,
             b"hexadecimal digit expected\x00" as *const u8 as
                 *const libc::c_char);
    return luaO_hexavalue((*ls).current);
}
unsafe extern "C" fn readhexaesc(mut ls: *mut LexState_0) -> libc::c_int {
    let mut r: libc::c_int = gethexa(ls);
    r = (r << 4i32) + gethexa(ls);
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(2i32 as libc::c_ulong)
            as size_t as size_t;
    return r;
}
unsafe extern "C" fn skip_sep(mut ls: *mut LexState_0) -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut s: libc::c_int = (*ls).current;
    if s == '[' as i32 || s == ']' as i32 {
    } else {
        __assert_fail(b"s == \'[\' || s == \']\'\x00" as *const u8 as
                          *const libc::c_char,
                      b"llex.c\x00" as *const u8 as *const libc::c_char,
                      255i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"int skip_sep(LexState *)\x00")).as_ptr());
    };
    save(ls, (*ls).current);
    let fresh68 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh68 > 0i32 as libc::c_ulong {
            let fresh69 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh69 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    while (*ls).current == '=' as i32 {
        save(ls, (*ls).current);
        let fresh70 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current =
            if fresh70 > 0i32 as libc::c_ulong {
                let fresh71 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh71 as libc::c_uchar as libc::c_int
            } else { luaZ_fill((*ls).z) };
        count += 1
    }
    return if (*ls).current == s { count } else { -count - 1i32 };
}
unsafe extern "C" fn read_long_string(mut ls: *mut LexState_0,
                                      mut seminfo: *mut SemInfo,
                                      mut sep: libc::c_int) -> () {
    let mut line: libc::c_int = (*ls).linenumber;
    save(ls, (*ls).current);
    let fresh72 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current =
        if fresh72 > 0i32 as libc::c_ulong {
            let fresh73 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh73 as libc::c_uchar as libc::c_int
        } else { luaZ_fill((*ls).z) };
    if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
        inclinenumber(ls);
    }
    loop  {
        match (*ls).current {
            -1 => {
                let mut what: *const libc::c_char =
                    if !seminfo.is_null() {
                        b"string\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"comment\x00" as *const u8 as *const libc::c_char
                    };
                let mut msg: *const libc::c_char =
                    luaO_pushfstring((*ls).L,
                                     b"unfinished long %s (starting at line %d)\x00"
                                         as *const u8 as *const libc::c_char,
                                     what, line);
                lexerror(ls, msg, TK_EOS as libc::c_int);
            }
            93 => {
                if !(skip_sep(ls) == sep) { continue ; }
                save(ls, (*ls).current);
                let fresh74 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current =
                    if fresh74 > 0i32 as libc::c_ulong {
                        let fresh75 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh75 as libc::c_uchar as libc::c_int
                    } else { luaZ_fill((*ls).z) };
                break ;
            }
            10 | 13 => {
                save(ls, '\n' as i32);
                inclinenumber(ls);
                if !seminfo.is_null() { continue ; }
                (*(*ls).buff).n = 0i32 as size_t
            }
            _ => {
                if !seminfo.is_null() {
                    save(ls, (*ls).current);
                    let fresh76 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current =
                        if fresh76 > 0i32 as libc::c_ulong {
                            let fresh77 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh77 as libc::c_uchar as libc::c_int
                        } else { luaZ_fill((*ls).z) }
                } else {
                    let fresh78 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current =
                        if fresh78 > 0i32 as libc::c_ulong {
                            let fresh79 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh79 as libc::c_uchar as libc::c_int
                        } else { luaZ_fill((*ls).z) }
                }
            }
        }
    }
    if !seminfo.is_null() {
        (*seminfo).ts =
            luaX_newstring(ls,
                           (*(*ls).buff).buffer.offset((2i32 + sep) as isize),
                           (*(*ls).buff).n.wrapping_sub((2i32 * (2i32 + sep))
                                                            as libc::c_ulong))
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaX_lookahead(mut ls: *mut LexState_0)
 -> libc::c_int {
    if (*ls).lookahead.token == TK_EOS as libc::c_int {
    } else {
        __assert_fail(b"ls->lookahead.token == TK_EOS\x00" as *const u8 as
                          *const libc::c_char,
                      b"llex.c\x00" as *const u8 as *const libc::c_char,
                      562i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 31],
                                                &[libc::c_char; 31]>(b"int luaX_lookahead(LexState *)\x00")).as_ptr());
    };
    (*ls).lookahead.token = llex(ls, &mut (*ls).lookahead.seminfo);
    return (*ls).lookahead.token;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_syntaxerror(mut ls: *mut LexState_0,
                                          mut msg: *const libc::c_char) -> ! {
    lexerror(ls, msg, (*ls).t.token);
}
