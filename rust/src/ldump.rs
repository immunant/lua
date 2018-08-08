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
    static luaO_nilobject_: TValue;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type lua_Writer =
    Option<unsafe extern "C" fn(_: *mut lua_State_0, _: *const libc::c_void,
                                _: size_t, _: *mut libc::c_void)
               -> libc::c_int>;
pub type CClosure = CClosure_0;
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
pub struct L_EXTRA {
    pub lock: libc::c_int,
    pub plock: *mut libc::c_int,
}
pub type Proto = Proto_0;
pub type LClosure = LClosure_0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata,
    cl: Closure,
    h: Table,
    p: Proto_0,
    th: lua_State,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure,
    l: LClosure,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct CClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct DumpState {
    pub L: *mut lua_State_0,
    pub writer: lua_Writer,
    pub data: *mut libc::c_void,
    pub strip: libc::c_int,
    pub status: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union L_Umaxalign {
    b: [libc::c_char; 64],
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union UTString {
    dummy: L_Umaxalign,
    tsv: TString,
}
pub type UTString_0 = UTString;
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
pub struct Proto_0 {
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
    pub p: *mut *mut Proto_0,
    pub lineinfo: *mut libc::c_int,
    pub locvars: *mut LocVar_0,
    pub upvalues: *mut Upvaldesc_0,
    pub cache: *mut LClosure_0,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto_0,
    pub upvals: [*mut UpVal; 1],
}
#[no_mangle]
pub unsafe extern "C" fn luaU_dump(mut L: *mut lua_State_0,
                                   mut f: *const Proto, mut w: lua_Writer,
                                   mut data: *mut libc::c_void,
                                   mut strip: libc::c_int) -> libc::c_int {
    let mut D: DumpState =
        DumpState{L: 0 as *mut lua_State_0,
                  writer: None,
                  data: 0 as *mut libc::c_void,
                  strip: 0,
                  status: 0,};
    D.L = L;
    D.writer = w;
    D.data = data;
    D.strip = strip;
    D.status = 0i32;
    DumpHeader(&mut D);
    DumpByte((*f).sizeupvalues, &mut D);
    DumpFunction(f, 0 as *mut TString, &mut D);
    return D.status;
}
unsafe extern "C" fn DumpFunction(mut f: *const Proto,
                                  mut psource: *mut TString,
                                  mut D: *mut DumpState) -> () {
    if 0 != (*D).strip || (*f).source == psource {
        DumpString(0 as *const TString, D);
    } else { DumpString((*f).source, D); }
    DumpInt((*f).linedefined, D);
    DumpInt((*f).lastlinedefined, D);
    DumpByte((*f).numparams as libc::c_int, D);
    DumpByte((*f).is_vararg as libc::c_int, D);
    DumpByte((*f).maxstacksize as libc::c_int, D);
    DumpCode(f, D);
    DumpConstants(f, D);
    DumpUpvalues(f, D);
    DumpProtos(f, D);
    DumpDebug(f, D);
}
unsafe extern "C" fn DumpDebug(mut f: *const Proto, mut D: *mut DumpState)
 -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = if 0 != (*D).strip { 0i32 } else { (*f).sizelineinfo };
    DumpInt(n, D);
    DumpBlock((*f).lineinfo as *const libc::c_void,
              (n as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                   as libc::c_ulong), D);
    n = if 0 != (*D).strip { 0i32 } else { (*f).sizelocvars };
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        DumpString((*(*f).locvars.offset(i as isize)).varname, D);
        DumpInt((*(*f).locvars.offset(i as isize)).startpc, D);
        DumpInt((*(*f).locvars.offset(i as isize)).endpc, D);
        i += 1
    }
    n = if 0 != (*D).strip { 0i32 } else { (*f).sizeupvalues };
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        DumpString((*(*f).upvalues.offset(i as isize)).name, D);
        i += 1
    };
}
unsafe extern "C" fn DumpString(mut s: *const TString, mut D: *mut DumpState)
 -> () {
    if s.is_null() {
        DumpByte(0i32, D);
    } else {
        let mut size: size_t =
            if (*s).tt as libc::c_int == 4i32 | 0i32 << 4i32 {
                (*s).shrlen as libc::c_ulong
            } else { (*s).u.lnglen }.wrapping_add(1i32 as libc::c_ulong);
        if 0 != ::std::mem::size_of::<lu_byte>() as libc::c_ulong {
        } else {
            __assert_fail(b"sizeof((s)->extra)\x00" as *const u8 as
                              *const libc::c_char,
                          b"ldump.c\x00" as *const u8 as *const libc::c_char,
                          78i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"void DumpString(const TString *, DumpState *)\x00")).as_ptr());
        };
        let mut str: *const libc::c_char =
            (s as
                 *mut libc::c_char).offset(::std::mem::size_of::<UTString_0>()
                                               as libc::c_ulong as isize);
        if size < 255i32 as libc::c_ulong {
            DumpByte(size as libc::c_int, D);
        } else {
            DumpByte(255i32, D);
            DumpBlock(&mut size as *mut size_t as *const libc::c_void,
                      (1i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                           as libc::c_ulong),
                      D);
        }
        DumpBlock(str as *const libc::c_void,
                  size.wrapping_sub(1i32 as
                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                        as
                                                                        libc::c_ulong),
                  D);
    };
}
unsafe extern "C" fn DumpBlock(mut b: *const libc::c_void, mut size: size_t,
                               mut D: *mut DumpState) -> () {
    if (*D).status == 0i32 && size > 0i32 as libc::c_ulong {
        let ref mut fresh0 =
            *(*(((*D).L as
                     *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                     as libc::c_ulong as
                                                     isize)) as
                    *mut libc::c_void as *mut L_EXTRA)).plock;
        *fresh0 -= 1;
        if *fresh0 == 0i32 {
        } else {
            __assert_fail(b"--(*((struct L_EXTRA*)(((void *)((char *)(D->L) - sizeof(struct L_EXTRA)))))->plock) == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldump.c\x00" as *const u8 as *const libc::c_char,
                          42i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 50],
                                                    &[libc::c_char; 50]>(b"void DumpBlock(const void *, size_t, DumpState *)\x00")).as_ptr());
        };
        (*D).status =
            (*D).writer.expect("non-null function pointer")((*D).L, b, size,
                                                            (*D).data);
        let ref mut fresh1 =
            *(*(((*D).L as
                     *mut libc::c_char).offset(-(::std::mem::size_of::<L_EXTRA>()
                                                     as libc::c_ulong as
                                                     isize)) as
                    *mut libc::c_void as *mut L_EXTRA)).plock;
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        if fresh2 == 0i32 {
        } else {
            __assert_fail(b"(*((struct L_EXTRA*)(((void *)((char *)(D->L) - sizeof(struct L_EXTRA)))))->plock)++ == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ldump.c\x00" as *const u8 as *const libc::c_char,
                          44i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 50],
                                                    &[libc::c_char; 50]>(b"void DumpBlock(const void *, size_t, DumpState *)\x00")).as_ptr());
        };
    };
}
unsafe extern "C" fn DumpByte(mut y: libc::c_int, mut D: *mut DumpState)
 -> () {
    let mut x: lu_byte = y as lu_byte;
    DumpBlock(&mut x as *mut lu_byte as *const libc::c_void,
              (1i32 as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<lu_byte>()
                                                   as libc::c_ulong), D);
}
unsafe extern "C" fn DumpInt(mut x: libc::c_int, mut D: *mut DumpState)
 -> () {
    DumpBlock(&mut x as *mut libc::c_int as *const libc::c_void,
              (1i32 as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                   as libc::c_ulong), D);
}
unsafe extern "C" fn DumpProtos(mut f: *const Proto, mut D: *mut DumpState)
 -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*f).sizep;
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        DumpFunction(*(*f).p.offset(i as isize), (*f).source, D);
        i += 1
    };
}
unsafe extern "C" fn DumpUpvalues(mut f: *const Proto, mut D: *mut DumpState)
 -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*f).sizeupvalues;
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        DumpByte((*(*f).upvalues.offset(i as isize)).instack as libc::c_int,
                 D);
        DumpByte((*(*f).upvalues.offset(i as isize)).idx as libc::c_int, D);
        i += 1
    };
}
unsafe extern "C" fn DumpConstants(mut f: *const Proto, mut D: *mut DumpState)
 -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*f).sizek;
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        let mut o: *const TValue =
            &mut *(*f).k.offset(i as isize) as *mut TValue;
        DumpByte((*o).tt_ & 63i32, D);
        match (*o).tt_ & 63i32 {
            0 => { }
            1 => {
                if (*o).tt_ == 1i32 {
                } else {
                    __assert_fail(b"((((o))->tt_) == (1))\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldump.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  109i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"void DumpConstants(const Proto *, DumpState *)\x00")).as_ptr());
                };
                DumpByte((*o).value_.b, D);
            }
            3 => {
                if (*o).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"((((o))->tt_) == ((3 | (0 << 4))))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"ldump.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  112i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"void DumpConstants(const Proto *, DumpState *)\x00")).as_ptr());
                };
                DumpNumber((*o).value_.n, D);
            }
            19 => {
                if (*o).tt_ == 3i32 | 1i32 << 4i32 {
                } else {
                    __assert_fail(b"((((o))->tt_) == ((3 | (1 << 4))))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"ldump.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  115i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"void DumpConstants(const Proto *, DumpState *)\x00")).as_ptr());
                };
                DumpInteger((*o).value_.i, D);
            }
            4 | 20 => {
                if (*o).tt_ & 15i32 == 4i32 {
                } else {
                    __assert_fail(b"(((((((o))->tt_)) & 0x0F)) == (4))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"ldump.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  119i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"void DumpConstants(const Proto *, DumpState *)\x00")).as_ptr());
                };
                if (*(*o).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
                } else {
                    __assert_fail(b"(((((o)->value_).gc)->tt) & 0x0F) == 4\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"ldump.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  119i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"void DumpConstants(const Proto *, DumpState *)\x00")).as_ptr());
                };
                DumpString(&mut (*((*o).value_.gc as *mut GCUnion)).ts, D);
            }
            _ => {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ldump.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  122i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 47],
                                                            &[libc::c_char; 47]>(b"void DumpConstants(const Proto *, DumpState *)\x00")).as_ptr());
                };
            }
        }
        i += 1
    };
}
unsafe extern "C" fn DumpInteger(mut x: lua_Integer, mut D: *mut DumpState)
 -> () {
    DumpBlock(&mut x as *mut lua_Integer as *const libc::c_void,
              (1i32 as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<lua_Integer>()
                                                   as libc::c_ulong), D);
}
unsafe extern "C" fn DumpNumber(mut x: lua_Number, mut D: *mut DumpState)
 -> () {
    DumpBlock(&mut x as *mut lua_Number as *const libc::c_void,
              (1i32 as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<lua_Number>()
                                                   as libc::c_ulong), D);
}
unsafe extern "C" fn DumpCode(mut f: *const Proto, mut D: *mut DumpState)
 -> () {
    DumpInt((*f).sizecode, D);
    DumpBlock((*f).code as *const libc::c_void,
              ((*f).sizecode as
                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>()
                                                   as libc::c_ulong), D);
}
unsafe extern "C" fn DumpHeader(mut D: *mut DumpState) -> () {
    DumpBlock(b"\x1bLua\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void,
              (::std::mem::size_of::<[libc::c_char; 5]>() as
                   libc::c_ulong).wrapping_sub(::std::mem::size_of::<libc::c_char>()
                                                   as libc::c_ulong), D);
    DumpByte(((*::std::mem::transmute::<&[u8; 2],
                                        &[libc::c_char; 2]>(b"5\x00"))[0usize]
                  as libc::c_int - '0' as i32) * 16i32 +
                 ((*::std::mem::transmute::<&[u8; 2],
                                            &[libc::c_char; 2]>(b"3\x00"))[0usize]
                      as libc::c_int - '0' as i32), D);
    DumpByte(0i32, D);
    DumpBlock(b"\x19\x93\r\n\x1a\n\x00" as *const u8 as *const libc::c_char as
                  *const libc::c_void,
              (::std::mem::size_of::<[libc::c_char; 7]>() as
                   libc::c_ulong).wrapping_sub(::std::mem::size_of::<libc::c_char>()
                                                   as libc::c_ulong), D);
    DumpByte(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                 libc::c_int, D);
    DumpByte(::std::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int,
             D);
    DumpByte(::std::mem::size_of::<Instruction>() as libc::c_ulong as
                 libc::c_int, D);
    DumpByte(::std::mem::size_of::<lua_Integer>() as libc::c_ulong as
                 libc::c_int, D);
    DumpByte(::std::mem::size_of::<lua_Number>() as libc::c_ulong as
                 libc::c_int, D);
    DumpInteger(22136i32 as lua_Integer, D);
    DumpNumber(370.5f64, D);
}
