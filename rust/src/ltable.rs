#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(extern_types, libc, offset_to)]
extern crate libc;
extern "C" {
    pub type lua_longjmp;
    pub type UpVal_0;
    #[no_mangle]
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut signgam: libc::c_int;
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
    fn luaO_ceillog2(x: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaM_toobig(L: *mut lua_State_0) -> !;
    #[no_mangle]
    fn luaM_realloc_(L: *mut lua_State_0, block: *mut libc::c_void,
                     oldsize: size_t, size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn luaC_newobj(L: *mut lua_State_0, tt: libc::c_int, sz: size_t)
     -> *mut GCObject;
    #[no_mangle]
    fn luaC_barrierback_(L: *mut lua_State_0, o: *mut Table) -> ();
    #[no_mangle]
    fn luaS_hashlongstr(ts: *mut TString) -> libc::c_uint;
    #[no_mangle]
    fn luaV_equalobj(L: *mut lua_State_0, t1: *const TValue,
                     t2: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer,
                      mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaG_runerror(L: *mut lua_State_0, fmt: *const libc::c_char, ...) -> !;
}
pub type __sig_atomic_t = libc::c_int;
pub type LocVar = LocVar_0;
pub type Upvaldesc = Upvaldesc_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Memcontrol {
    pub numblocks: libc::c_ulong,
    pub total: libc::c_ulong,
    pub maxmem: libc::c_ulong,
    pub memlimit: libc::c_ulong,
    pub objcount: [libc::c_ulong; 9],
}
pub type CClosure = CClosure_0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Closure {
    c: CClosure,
    l: LClosure,
}
pub type LClosure = LClosure_0;
pub type Memcontrol_0 = Memcontrol;
pub type Table = Table_0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table_0,
    pub len: size_t,
    pub user_: Value_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LocVar_0 {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
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
pub struct Upvaldesc_0 {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union GCUnion {
    gc: GCObject,
    ts: TString_0,
    u: Udata,
    cl: Closure,
    h: Table_0,
    p: Proto,
    th: lua_State,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct LClosure_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
    pub mt: [*mut Table_0; 9],
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
pub struct Table_0 {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,
    pub lsizenode: lu_byte,
    pub sizearray: libc::c_uint,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table_0,
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
    pub locvars: *mut LocVar,
    pub upvalues: *mut Upvaldesc,
    pub cache: *mut LClosure_0,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getint(mut t: *mut Table, mut key: lua_Integer)
 -> *const TValue {
    if (key as lua_Unsigned).wrapping_sub(1i32 as libc::c_ulonglong) <
           (*t).sizearray as libc::c_ulonglong {
        return &mut *(*t).array.offset((key - 1i32 as libc::c_longlong) as
                                           isize) as *mut TValue
    } else {
        if 1i32 << (*t).lsizenode as libc::c_int &
               (1i32 << (*t).lsizenode as libc::c_int) - 1i32 == 0i32 {
        } else {
            __assert_fail(b"(((1<<((t)->lsizenode)))&(((1<<((t)->lsizenode)))-1))==0\x00"
                              as *const u8 as *const libc::c_char,
                          b"ltable.c\x00" as *const u8 as *const libc::c_char,
                          503i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"const TValue *luaH_getint(Table *, lua_Integer)\x00")).as_ptr());
        };
        let mut n: *mut Node =
            &mut *(*t).node.offset((key &
                                        ((1i32 <<
                                              (*t).lsizenode as libc::c_int) -
                                             1i32) as libc::c_longlong) as
                                       libc::c_int as isize) as *mut Node;
        loop  {
            if (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ ==
                   3i32 | 1i32 << 4i32 &&
                   {
                       if (*(&mut (*n).i_key.tvk as *mut TValue as
                                 *const TValue)).tt_ == 3i32 | 1i32 << 4i32 {
                       } else {
                           __assert_fail(b"((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == ((3 | (1 << 4))))\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         b"ltable.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         505i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 48],
                                                                   &[libc::c_char; 48]>(b"const TValue *luaH_getint(Table *, lua_Integer)\x00")).as_ptr());
                       };
                       (*(&mut (*n).i_key.tvk as *mut TValue as
                              *const TValue)).value_.i == key
                   } {
                return &mut (*n).i_val as *mut TValue
            } else {
                let mut nx: libc::c_int = (*n).i_key.nk.next;
                if nx == 0i32 { break ; }
                n = n.offset(nx as isize)
            }
        }
        return &luaO_nilobject_
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_setint(mut L: *mut lua_State_0,
                                     mut t: *mut Table, mut key: lua_Integer,
                                     mut value: *mut TValue) -> () {
    let mut p: *const TValue = luaH_getint(t, key);
    let mut cell: *mut TValue = 0 as *mut TValue;
    if p != &luaO_nilobject_ {
        cell = p as *mut TValue
    } else {
        let mut k: TValue =
            lua_TValue{value_:
                           Value_0{gc:
                                       0 as *const GCObject as
                                           *mut GCObject,},
                       tt_: 0,};
        let mut io: *mut TValue = &mut k;
        (*io).value_.i = key;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        cell = luaH_newkey(L, t, &mut k)
    }
    *cell = *value;
    if 0 == (*cell).tt_ & 1i32 << 6i32 ||
           {
               if 0 != (*cell).tt_ & 1i32 << 6i32 {
               } else {
                   __assert_fail(b"((((cell))->tt_) & (1 << 6))\x00" as
                                     *const u8 as *const libc::c_char,
                                 b"ltable.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 610i32 as libc::c_uint,
                                 (*::std::mem::transmute::<&[u8; 62],
                                                           &[libc::c_char; 62]>(b"void luaH_setint(lua_State *, Table *, lua_Integer, TValue *)\x00")).as_ptr());
               };
               (*cell).tt_ & 63i32 == (*(*cell).value_.gc).tt as libc::c_int
                   &&
                   (L.is_null() ||
                        {
                            if 0 != (*cell).tt_ & 1i32 << 6i32 {
                            } else {
                                __assert_fail(b"((((cell))->tt_) & (1 << 6))\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"ltable.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              610i32 as libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 62],
                                                                        &[libc::c_char; 62]>(b"void luaH_setint(lua_State *, Table *, lua_Integer, TValue *)\x00")).as_ptr());
                            };
                            0 !=
                                ((*(*cell).value_.gc).marked as libc::c_int ^
                                     (1i32 << 0i32 | 1i32 << 1i32)) &
                                    ((*(*L).l_G).currentwhite as libc::c_int ^
                                         (1i32 << 0i32 | 1i32 << 1i32))
                        })
           } {
    } else {
        if 0 != 0i32 {
        } else {
            __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                          b"ltable.c\x00" as *const u8 as *const libc::c_char,
                          610i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 62],
                                                    &[libc::c_char; 62]>(b"void luaH_setint(lua_State *, Table *, lua_Integer, TValue *)\x00")).as_ptr());
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_newkey(mut L: *mut lua_State_0,
                                     mut t: *mut Table,
                                     mut key: *const TValue) -> *mut TValue {
    let mut mp: *mut Node = 0 as *mut Node;
    let mut aux: TValue =
        lua_TValue{value_:
                       Value_0{gc: 0 as *const GCObject as *mut GCObject,},
                   tt_: 0,};
    if (*key).tt_ == 0i32 {
        luaG_runerror(L,
                      b"table index is nil\x00" as *const u8 as
                          *const libc::c_char);
    } else {
        if (*key).tt_ == 3i32 | 0i32 << 4i32 {
            let mut k: lua_Integer = 0;
            if 0 != luaV_tointeger(key, &mut k, 0i32) {
                let mut io: *mut TValue = &mut aux;
                (*io).value_.i = k;
                (*io).tt_ = 3i32 | 1i32 << 4i32;
                key = &mut aux
            } else {
                if (*key).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"((((key))->tt_) == ((3 | (0 << 4))))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"ltable.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  453i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 58],
                                                            &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
                };
                if (*key).tt_ == 3i32 | 0i32 << 4i32 {
                } else {
                    __assert_fail(b"((((key))->tt_) == ((3 | (0 << 4))))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"ltable.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  453i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 58],
                                                            &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
                };
                if !((*key).value_.n == (*key).value_.n) {
                    luaG_runerror(L,
                                  b"table index is NaN\x00" as *const u8 as
                                      *const libc::c_char);
                }
            }
        }
        mp = mainposition(t, key);
        if !((*mp).i_val.tt_ == 0i32) || (*t).lastfree.is_null() {
            let mut othern: *mut Node = 0 as *mut Node;
            let mut f: *mut Node = getfreepos(t);
            if f.is_null() {
                rehash(L, t, key);
                return luaH_set(L, t, key)
            } else {
                if !(*t).lastfree.is_null() {
                } else {
                    __assert_fail(b"!((t)->lastfree == ((void*)0))\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"ltable.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  465i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 58],
                                                            &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
                };
                othern =
                    mainposition(t,
                                 &mut (*mp).i_key.tvk as *mut TValue as
                                     *const TValue);
                if othern != mp {
                    while othern.offset((*othern).i_key.nk.next as isize) !=
                              mp {
                        othern =
                            othern.offset((*othern).i_key.nk.next as isize)
                    }
                    (*othern).i_key.nk.next =
                        othern.offset_to(f).expect("bad offset_to") as
                            libc::c_long as libc::c_int;
                    *f = *mp;
                    if (*mp).i_key.nk.next != 0i32 {
                        (*f).i_key.nk.next +=
                            f.offset_to(mp).expect("bad offset_to") as
                                libc::c_long as libc::c_int;
                        (*mp).i_key.nk.next = 0i32
                    }
                    (*mp).i_val.tt_ = 0i32
                } else {
                    if (*mp).i_key.nk.next != 0i32 {
                        (*f).i_key.nk.next =
                            f.offset_to(mp.offset((*mp).i_key.nk.next as
                                                      isize)).expect("bad offset_to")
                                as libc::c_long as libc::c_int
                    } else {
                        if (*f).i_key.nk.next == 0i32 {
                        } else {
                            __assert_fail(b"((f)->i_key.nk.next) == 0\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"ltable.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          483i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 58],
                                                                    &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
                        };
                    }
                    (*mp).i_key.nk.next =
                        mp.offset_to(f).expect("bad offset_to") as
                            libc::c_long as libc::c_int;
                    mp = f
                }
            }
        }
        let mut k_: *mut TKey = &mut (*mp).i_key as *mut TKey;
        let mut io_: *const TValue = key;
        (*k_).nk.value_ = (*io_).value_;
        (*k_).nk.tt_ = (*io_).tt_;
        if 0 == (*io_).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io_).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io_)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ltable.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     488i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 58],
                                                               &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
                   };
                   (*io_).tt_ & 63i32 == (*(*io_).value_.gc).tt as libc::c_int
                       &&
                       (L.is_null() ||
                            {
                                if 0 != (*io_).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"(((io_)->tt_) & (1 << 6))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ltable.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  488i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 58],
                                                                            &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
                                };
                                0 !=
                                    ((*(*io_).value_.gc).marked as libc::c_int
                                         ^ (1i32 << 0i32 | 1i32 << 1i32)) &
                                        ((*(*L).l_G).currentwhite as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32))
                            })
               } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 488i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 58],
                                                        &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
            };
        };
        if 0 != (*key).tt_ & 1i32 << 6i32 &&
               0 != (*t).marked as libc::c_int & 1i32 << 2i32 &&
               {
                   if 0 != (*key).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((key)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ltable.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     489i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 58],
                                                               &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
                   };
                   0 !=
                       (*(*key).value_.gc).marked as libc::c_int &
                           (1i32 << 0i32 | 1i32 << 1i32)
               } {
            luaC_barrierback_(L, t);
        } else { };
        if (*mp).i_val.tt_ == 0i32 {
        } else {
            __assert_fail(b"(((((&(mp)->i_val)))->tt_) == (0))\x00" as
                              *const u8 as *const libc::c_char,
                          b"ltable.c\x00" as *const u8 as *const libc::c_char,
                          490i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"TValue *luaH_newkey(lua_State *, Table *, const TValue *)\x00")).as_ptr());
        };
        return &mut (*mp).i_val as *mut TValue
    };
}
unsafe extern "C" fn getfreepos(mut t: *mut Table) -> *mut Node {
    if !(*t).lastfree.is_null() {
        while (*t).lastfree > (*t).node {
            (*t).lastfree = (*t).lastfree.offset(-1isize);
            if !((*(&mut (*(*t).lastfree).i_key.tvk as *mut TValue as
                        *const TValue)).tt_ == 0i32) {
                continue ;
            }
            return (*t).lastfree
        }
    }
    return 0 as *mut Node;
}
unsafe extern "C" fn mainposition(mut t: *const Table, mut key: *const TValue)
 -> *mut Node {
    match (*key).tt_ & 63i32 {
        19 => {
            if 1i32 << (*t).lsizenode as libc::c_int &
                   (1i32 << (*t).lsizenode as libc::c_int) - 1i32 == 0i32 {
            } else {
                __assert_fail(b"(((1<<((t)->lsizenode)))&(((1<<((t)->lsizenode)))-1))==0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 121i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            if (*key).tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(b"((((key))->tt_) == ((3 | (1 << 4))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 121i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            return &mut *(*t).node.offset(((*key).value_.i &
                                               ((1i32 <<
                                                     (*t).lsizenode as
                                                         libc::c_int) - 1i32)
                                                   as libc::c_longlong) as
                                              libc::c_int as isize) as
                       *mut Node
        }
        3 => {
            if (*key).tt_ == 3i32 | 0i32 << 4i32 {
            } else {
                __assert_fail(b"((((key))->tt_) == ((3 | (0 << 4))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 123i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            return &mut *(*t).node.offset((l_hashfloat((*key).value_.n) %
                                               ((1i32 <<
                                                     (*t).lsizenode as
                                                         libc::c_int) - 1i32 |
                                                    1i32)) as isize) as
                       *mut Node
        }
        4 => {
            if 1i32 << (*t).lsizenode as libc::c_int &
                   (1i32 << (*t).lsizenode as libc::c_int) - 1i32 == 0i32 {
            } else {
                __assert_fail(b"(((1<<((t)->lsizenode)))&(((1<<((t)->lsizenode)))-1))==0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 125i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            if (*key).tt_ & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((((key))->tt_)) & 0x0F)) == (4))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 125i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            if (*(*key).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((key)->value_).gc)->tt) & 0x0F) == 4\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 125i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            return &mut *(*t).node.offset(((*(&mut (*((*key).value_.gc as
                                                          *mut GCUnion)).ts as
                                                  *mut TString_0)).hash &
                                               ((1i32 <<
                                                     (*t).lsizenode as
                                                         libc::c_int) - 1i32)
                                                   as libc::c_uint) as
                                              libc::c_int as isize) as
                       *mut Node
        }
        20 => {
            if 1i32 << (*t).lsizenode as libc::c_int &
                   (1i32 << (*t).lsizenode as libc::c_int) - 1i32 == 0i32 {
            } else {
                __assert_fail(b"(((1<<((t)->lsizenode)))&(((1<<((t)->lsizenode)))-1))==0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 127i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            if (*key).tt_ & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((((key))->tt_)) & 0x0F)) == (4))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 127i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            if (*(*key).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((key)->value_).gc)->tt) & 0x0F) == 4\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 127i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            return &mut *(*t).node.offset((luaS_hashlongstr(&mut (*((*key).value_.gc
                                                                        as
                                                                        *mut GCUnion)).ts)
                                               &
                                               ((1i32 <<
                                                     (*t).lsizenode as
                                                         libc::c_int) - 1i32)
                                                   as libc::c_uint) as
                                              libc::c_int as isize) as
                       *mut Node
        }
        1 => {
            if 1i32 << (*t).lsizenode as libc::c_int &
                   (1i32 << (*t).lsizenode as libc::c_int) - 1i32 == 0i32 {
            } else {
                __assert_fail(b"(((1<<((t)->lsizenode)))&(((1<<((t)->lsizenode)))-1))==0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 129i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            if (*key).tt_ == 1i32 {
            } else {
                __assert_fail(b"((((key))->tt_) == (1))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 129i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            return &mut *(*t).node.offset(((*key).value_.b &
                                               (1i32 <<
                                                    (*t).lsizenode as
                                                        libc::c_int) - 1i32)
                                              as isize) as *mut Node
        }
        2 => {
            if (*key).tt_ == 2i32 {
            } else {
                __assert_fail(b"((((key))->tt_) == (2))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 131i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            return &mut *(*t).node.offset((((*key).value_.p as size_t &
                                                (2147483647i32 as
                                                     libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                    as libc::c_ulong) as
                                               libc::c_uint).wrapping_rem(((1i32
                                                                                <<
                                                                                (*t).lsizenode
                                                                                    as
                                                                                    libc::c_int)
                                                                               -
                                                                               1i32
                                                                               |
                                                                               1i32)
                                                                              as
                                                                              libc::c_uint)
                                              as isize) as *mut Node
        }
        22 => {
            if (*key).tt_ == 6i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(b"((((key))->tt_) == ((6 | (1 << 4))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 133i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            return &mut *(*t).node.offset((((*key).value_.f.unwrap() as size_t &
                                                (2147483647i32 as
                                                     libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                    as libc::c_ulong) as
                                               libc::c_uint).wrapping_rem(((1i32
                                                                                <<
                                                                                (*t).lsizenode
                                                                                    as
                                                                                    libc::c_int)
                                                                               -
                                                                               1i32
                                                                               |
                                                                               1i32)
                                                                              as
                                                                              libc::c_uint)
                                              as isize) as *mut Node
        }
        _ => {
            if !((*key).tt_ == 9i32 + 1i32) {
            } else {
                __assert_fail(b"!((((key))->tt_) == ((9+1)))\x00" as *const u8
                                  as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 135i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            if 0 != (*key).tt_ & 1i32 << 6i32 {
            } else {
                __assert_fail(b"(((key)->tt_) & (1 << 6))\x00" as *const u8 as
                                  *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 136i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 50],
                                                        &[libc::c_char; 50]>(b"Node *mainposition(const Table *, const TValue *)\x00")).as_ptr());
            };
            return &mut *(*t).node.offset((((*key).value_.gc as size_t &
                                                (2147483647i32 as
                                                     libc::c_uint).wrapping_mul(2u32).wrapping_add(1u32)
                                                    as libc::c_ulong) as
                                               libc::c_uint).wrapping_rem(((1i32
                                                                                <<
                                                                                (*t).lsizenode
                                                                                    as
                                                                                    libc::c_int)
                                                                               -
                                                                               1i32
                                                                               |
                                                                               1i32)
                                                                              as
                                                                              libc::c_uint)
                                              as isize) as *mut Node
        }
    };
}
unsafe extern "C" fn l_hashfloat(mut n: lua_Number) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ni: lua_Integer = 0;
    n = frexp(n, &mut i) * -((-2147483647i32 - 1i32) as lua_Number);
    if !(n >= (-9223372036854775807i64 - 1i64) as libc::c_double &&
             n < -((-9223372036854775807i64 - 1i64) as libc::c_double) &&
             { ni = n as libc::c_longlong; 0 != 1i32 }) {
        if !(n == n) || fabs(n) == ::std::f64::INFINITY {
        } else {
            __assert_fail(b"(!(((n))==((n)))) || fabs(n) == ((lua_Number)(((__builtin_huge_val ()))))\x00"
                              as *const u8 as *const libc::c_char,
                          b"ltable.c\x00" as *const u8 as *const libc::c_char,
                          103i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 28],
                                                    &[libc::c_char; 28]>(b"int l_hashfloat(lua_Number)\x00")).as_ptr());
        };
        return 0i32
    } else {
        let mut u: libc::c_uint =
            (i as libc::c_uint).wrapping_add(ni as libc::c_uint);
        return (if u <= 2147483647i32 as libc::c_uint { u } else { !u }) as
                   libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_set(mut L: *mut lua_State_0, mut t: *mut Table,
                                  mut key: *const TValue) -> *mut TValue {
    let mut p: *const TValue = luaH_get(t, key);
    if p != &luaO_nilobject_ {
        return p as *mut TValue
    } else { return luaH_newkey(L, t, key) };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_get(mut t: *mut Table, mut key: *const TValue)
 -> *const TValue {
    match (*key).tt_ & 63i32 {
        4 => {
            if (*key).tt_ & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((((key))->tt_)) & 0x0F)) == (4))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 573i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 48],
                                                        &[libc::c_char; 48]>(b"const TValue *luaH_get(Table *, const TValue *)\x00")).as_ptr());
            };
            if (*(*key).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
            } else {
                __assert_fail(b"(((((key)->value_).gc)->tt) & 0x0F) == 4\x00"
                                  as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 573i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 48],
                                                        &[libc::c_char; 48]>(b"const TValue *luaH_get(Table *, const TValue *)\x00")).as_ptr());
            };
            return luaH_getshortstr(t,
                                    &mut (*((*key).value_.gc as
                                                *mut GCUnion)).ts)
        }
        19 => {
            if (*key).tt_ == 3i32 | 1i32 << 4i32 {
            } else {
                __assert_fail(b"((((key))->tt_) == ((3 | (1 << 4))))\x00" as
                                  *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 574i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 48],
                                                        &[libc::c_char; 48]>(b"const TValue *luaH_get(Table *, const TValue *)\x00")).as_ptr());
            };
            return luaH_getint(t, (*key).value_.i)
        }
        0 => { return &luaO_nilobject_ }
        3 => {
            let mut k: lua_Integer = 0;
            if 0 != luaV_tointeger(key, &mut k, 0i32) {
                return luaH_getint(t, k)
            }
        }
        _ => { }
    }
    return getgeneric(t, key);
}
unsafe extern "C" fn getgeneric(mut t: *mut Table, mut key: *const TValue)
 -> *const TValue {
    let mut n: *mut Node = mainposition(t, key);
    loop  {
        if 0 !=
               luaV_equalobj(0 as *mut lua_State_0,
                             &mut (*n).i_key.tvk as *mut TValue as
                                 *const TValue, key) {
            return &mut (*n).i_val as *mut TValue
        } else {
            let mut nx: libc::c_int = (*n).i_key.nk.next;
            if nx == 0i32 {
                return &luaO_nilobject_
            } else { n = n.offset(nx as isize) }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getshortstr(mut t: *mut Table,
                                          mut key: *mut TString)
 -> *const TValue {
    if 1i32 << (*t).lsizenode as libc::c_int &
           (1i32 << (*t).lsizenode as libc::c_int) - 1i32 == 0i32 {
    } else {
        __assert_fail(b"(((1<<((t)->lsizenode)))&(((1<<((t)->lsizenode)))-1))==0\x00"
                          as *const u8 as *const libc::c_char,
                      b"ltable.c\x00" as *const u8 as *const libc::c_char,
                      522i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"const TValue *luaH_getshortstr(Table *, TString *)\x00")).as_ptr());
    };
    let mut n: *mut Node =
        &mut *(*t).node.offset(((*key).hash &
                                    ((1i32 << (*t).lsizenode as libc::c_int) -
                                         1i32) as libc::c_uint) as libc::c_int
                                   as isize) as *mut Node;
    if (*key).tt as libc::c_int == 4i32 | 0i32 << 4i32 {
    } else {
        __assert_fail(b"key->tt == (4 | (0 << 4))\x00" as *const u8 as
                          *const libc::c_char,
                      b"ltable.c\x00" as *const u8 as *const libc::c_char,
                      523i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 51],
                                                &[libc::c_char; 51]>(b"const TValue *luaH_getshortstr(Table *, TString *)\x00")).as_ptr());
    };
    loop  {
        let mut k: *const TValue =
            &mut (*n).i_key.tvk as *mut TValue as *const TValue;
        if (*k).tt_ == 4i32 | 0i32 << 4i32 | 1i32 << 6i32 &&
               {
                   if (*k).tt_ & 15i32 == 4i32 {
                   } else {
                       __assert_fail(b"(((((((k))->tt_)) & 0x0F)) == (4))\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"ltable.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     526i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 51],
                                                               &[libc::c_char; 51]>(b"const TValue *luaH_getshortstr(Table *, TString *)\x00")).as_ptr());
                   };
                   if (*(*k).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
                   } else {
                       __assert_fail(b"(((((k)->value_).gc)->tt) & 0x0F) == 4\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"ltable.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     526i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 51],
                                                               &[libc::c_char; 51]>(b"const TValue *luaH_getshortstr(Table *, TString *)\x00")).as_ptr());
                   };
                   if (*(&mut (*((*k).value_.gc as *mut GCUnion)).ts as
                             *mut TString_0)).tt as libc::c_int ==
                          4i32 | 0i32 << 4i32 {
                   } else {
                       __assert_fail(b"(((((((((((k))->tt_)) & 0x0F)) == (4))) ? (void) (0) : __assert_fail (\"(((((((k))->tt_)) & 0x0F)) == (4))\", \"ltable.c\", 526, __extension__ __PRETTY_FUNCTION__)), (((((((((k)->value_).gc)->tt) & 0x0F) == 4) ? (void) (0) : __assert_fail (\"(((((k)->value_).gc)->tt) & 0x0F) == 4\", \"ltable.c\", 526, __extension__ __PRETTY_FUNCTION__)), (&((((union GCUnion *)((((k)->value_).gc))))->ts))))))->tt == (4 | (0 << 4))\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"ltable.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     526i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 51],
                                                               &[libc::c_char; 51]>(b"const TValue *luaH_getshortstr(Table *, TString *)\x00")).as_ptr());
                   };
                   if (*k).tt_ & 15i32 == 4i32 {
                   } else {
                       __assert_fail(b"(((((((k))->tt_)) & 0x0F)) == (4))\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"ltable.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     526i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 51],
                                                               &[libc::c_char; 51]>(b"const TValue *luaH_getshortstr(Table *, TString *)\x00")).as_ptr());
                   };
                   if (*(*k).value_.gc).tt as libc::c_int & 15i32 == 4i32 {
                   } else {
                       __assert_fail(b"(((((k)->value_).gc)->tt) & 0x0F) == 4\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"ltable.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     526i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 51],
                                                               &[libc::c_char; 51]>(b"const TValue *luaH_getshortstr(Table *, TString *)\x00")).as_ptr());
                   };
                   &mut (*((*k).value_.gc as *mut GCUnion)).ts as
                       *mut TString_0 == key
               } {
            return &mut (*n).i_val as *mut TValue
        } else {
            let mut nx: libc::c_int = (*n).i_key.nk.next;
            if nx == 0i32 {
                return &luaO_nilobject_
            } else { n = n.offset(nx as isize) }
        }
    };
}
unsafe extern "C" fn rehash(mut L: *mut lua_State_0, mut t: *mut Table,
                            mut ek: *const TValue) -> () {
    let mut asize: libc::c_uint = 0;
    let mut na: libc::c_uint = 0;
    let mut nums: [libc::c_uint; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut totaluse: libc::c_int = 0;
    i = 0i32;
    while i <=
              (::std::mem::size_of::<libc::c_int>() as
                   libc::c_ulong).wrapping_mul(8i32 as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)
                  as libc::c_int {
        nums[i as usize] = 0i32 as libc::c_uint;
        i += 1
    }
    na = numusearray(t, nums.as_mut_ptr());
    totaluse = na as libc::c_int;
    totaluse += numusehash(t, nums.as_mut_ptr(), &mut na);
    na = na.wrapping_add(countint(ek, nums.as_mut_ptr()) as libc::c_uint);
    totaluse += 1;
    asize = computesizes(nums.as_mut_ptr(), &mut na);
    luaH_resize(L, t, asize, (totaluse as libc::c_uint).wrapping_sub(na));
}
#[no_mangle]
pub unsafe extern "C" fn luaH_resize(mut L: *mut lua_State_0,
                                     mut t: *mut Table,
                                     mut nasize: libc::c_uint,
                                     mut nhsize: libc::c_uint) -> () {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_int = 0;
    let mut oldasize: libc::c_uint = (*t).sizearray;
    let mut oldhsize: libc::c_int =
        if (*t).lastfree.is_null() {
            0i32
        } else { 1i32 << (*t).lsizenode as libc::c_int };
    let mut nold: *mut Node = (*t).node;
    if nasize > oldasize { setarrayvector(L, t, nasize); }
    setnodevector(L, t, nhsize);
    if nasize < oldasize {
        (*t).sizearray = nasize;
        i = nasize;
        while i < oldasize {
            if !((*(*t).array.offset(i as isize)).tt_ == 0i32) {
                luaH_setint(L, t,
                            i.wrapping_add(1i32 as libc::c_uint) as
                                lua_Integer,
                            &mut *(*t).array.offset(i as isize));
            }
            i = i.wrapping_add(1)
        }
        if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong >=
               ::std::mem::size_of::<size_t>() as libc::c_ulong &&
               (nasize as size_t).wrapping_add(1i32 as libc::c_ulong) >
                   (!(0i32 as
                          size_t)).wrapping_div(::std::mem::size_of::<TValue>()
                                                    as libc::c_ulong) {
            luaM_toobig(L);
        } else { };
        (*t).array =
            luaM_realloc_(L, (*t).array as *mut libc::c_void,
                          (oldasize as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                               as
                                                               libc::c_ulong),
                          (nasize as
                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                               as
                                                               libc::c_ulong))
                as *mut TValue
    }
    j = oldhsize - 1i32;
    while j >= 0i32 {
        let mut old: *mut Node = nold.offset(j as isize);
        if !((*old).i_val.tt_ == 0i32) {
            let mut io1: *mut TValue =
                luaH_set(L, t,
                         &mut (*old).i_key.tvk as *mut TValue as
                             *const TValue);
            *io1 = (*old).i_val;
            if 0 == (*io1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ltable.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         361i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 67],
                                                                   &[libc::c_char; 67]>(b"void luaH_resize(lua_State *, Table *, unsigned int, unsigned int)\x00")).as_ptr());
                       };
                       (*io1).tt_ & 63i32 ==
                           (*(*io1).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ltable.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      361i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 67],
                                                                                &[libc::c_char; 67]>(b"void luaH_resize(lua_State *, Table *, unsigned int, unsigned int)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io1).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ltable.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  361i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 67],
                                                            &[libc::c_char; 67]>(b"void luaH_resize(lua_State *, Table *, unsigned int, unsigned int)\x00")).as_ptr());
                };
            };
        }
        j -= 1
    }
    if oldhsize > 0i32 {
        luaM_realloc_(L, nold as *mut libc::c_void,
                      (oldhsize as
                           size_t).wrapping_mul(::std::mem::size_of::<Node>()
                                                    as libc::c_ulong),
                      0i32 as size_t);
    };
}
unsafe extern "C" fn setnodevector(mut L: *mut lua_State_0, mut t: *mut Table,
                                   mut size: libc::c_uint) -> () {
    if size == 0i32 as libc::c_uint {
        (*t).node = &dummynode_ as *const Node as *mut Node;
        (*t).lsizenode = 0i32 as lu_byte;
        (*t).lastfree = 0 as *mut Node
    } else {
        let mut i: libc::c_int = 0;
        let mut lsize: libc::c_int = luaO_ceillog2(size);
        if lsize >
               (::std::mem::size_of::<libc::c_int>() as
                    libc::c_ulong).wrapping_mul(8i32 as
                                                    libc::c_ulong).wrapping_sub(1i32
                                                                                    as
                                                                                    libc::c_ulong)
                   as libc::c_int - 1i32 {
            luaG_runerror(L,
                          b"table overflow\x00" as *const u8 as
                              *const libc::c_char);
        } else {
            size = (1i32 << lsize) as libc::c_uint;
            if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong >=
                   ::std::mem::size_of::<size_t>() as libc::c_ulong &&
                   (size as size_t).wrapping_add(1i32 as libc::c_ulong) >
                       (!(0i32 as
                              size_t)).wrapping_div(::std::mem::size_of::<Node>()
                                                        as libc::c_ulong) {
                luaM_toobig(L);
            } else { };
            (*t).node =
                luaM_realloc_(L, 0 as *mut libc::c_void,
                              (0i32 as
                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<Node>()
                                                                   as
                                                                   libc::c_ulong),
                              (size as
                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<Node>()
                                                                   as
                                                                   libc::c_ulong))
                    as *mut Node;
            i = 0i32;
            while i < size as libc::c_int {
                let mut n: *mut Node =
                    &mut *(*t).node.offset(i as isize) as *mut Node;
                (*n).i_key.nk.next = 0i32;
                (*n).i_key.nk.tt_ = 0i32;
                (*n).i_val.tt_ = 0i32;
                i += 1
            }
            (*t).lsizenode = lsize as lu_byte;
            (*t).lastfree = &mut *(*t).node.offset(size as isize) as *mut Node
        }
    };
}
static mut dummynode_: Node =
    unsafe {
        Node_0{i_val:
                   lua_TValue{value_:
                                  Value_0{gc:
                                              0 as *const GCObject as
                                                  *mut GCObject,},
                              tt_: 0i32,},
               i_key:
                   TKey_0{nk:
                              unnamed_3{value_:
                                            Value_0{gc:
                                                        0 as *const GCObject
                                                            as
                                                            *mut GCObject,},
                                        tt_: 0i32,
                                        next: 0i32,},},}
    };
unsafe extern "C" fn setarrayvector(mut L: *mut lua_State_0,
                                    mut t: *mut Table, mut size: libc::c_uint)
 -> () {
    let mut i: libc::c_uint = 0;
    if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong >=
           ::std::mem::size_of::<size_t>() as libc::c_ulong &&
           (size as size_t).wrapping_add(1i32 as libc::c_ulong) >
               (!(0i32 as
                      size_t)).wrapping_div(::std::mem::size_of::<TValue>() as
                                                libc::c_ulong) {
        luaM_toobig(L);
    } else { };
    (*t).array =
        luaM_realloc_(L, (*t).array as *mut libc::c_void,
                      ((*t).sizearray as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong),
                      (size as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                           as libc::c_ulong))
            as *mut TValue;
    i = (*t).sizearray;
    while i < size {
        (*(*t).array.offset(i as isize)).tt_ = 0i32;
        i = i.wrapping_add(1)
    }
    (*t).sizearray = size;
}
unsafe extern "C" fn computesizes(mut nums: *mut libc::c_uint,
                                  mut pna: *mut libc::c_uint)
 -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut twotoi: libc::c_uint = 0;
    let mut a: libc::c_uint = 0i32 as libc::c_uint;
    let mut na: libc::c_uint = 0i32 as libc::c_uint;
    let mut optimal: libc::c_uint = 0i32 as libc::c_uint;
    i = 0i32;
    twotoi = 1i32 as libc::c_uint;
    while *pna > twotoi.wrapping_div(2i32 as libc::c_uint) {
        if *nums.offset(i as isize) > 0i32 as libc::c_uint {
            a = a.wrapping_add(*nums.offset(i as isize));
            if a > twotoi.wrapping_div(2i32 as libc::c_uint) {
                optimal = twotoi;
                na = a
            }
        }
        i += 1;
        twotoi = twotoi.wrapping_mul(2i32 as libc::c_uint)
    }
    if (optimal == 0i32 as libc::c_uint ||
            optimal.wrapping_div(2i32 as libc::c_uint) < na) && na <= optimal
       {
    } else {
        __assert_fail(b"(optimal == 0 || optimal / 2 < na) && na <= optimal\x00"
                          as *const u8 as *const libc::c_char,
                      b"ltable.c\x00" as *const u8 as *const libc::c_char,
                      236i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"unsigned int computesizes(unsigned int *, unsigned int *)\x00")).as_ptr());
    };
    *pna = na;
    return optimal;
}
unsafe extern "C" fn countint(mut key: *const TValue,
                              mut nums: *mut libc::c_uint) -> libc::c_int {
    let mut k: libc::c_uint = arrayindex(key);
    if k != 0i32 as libc::c_uint {
        let ref mut fresh0 = *nums.offset(luaO_ceillog2(k) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        return 1i32
    } else { return 0i32 };
}
unsafe extern "C" fn arrayindex(mut key: *const TValue) -> libc::c_uint {
    if (*key).tt_ == 3i32 | 1i32 << 4i32 {
        if (*key).tt_ == 3i32 | 1i32 << 4i32 {
        } else {
            __assert_fail(b"((((key))->tt_) == ((3 | (1 << 4))))\x00" as
                              *const u8 as *const libc::c_char,
                          b"ltable.c\x00" as *const u8 as *const libc::c_char,
                          147i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_char; 40]>(b"unsigned int arrayindex(const TValue *)\x00")).as_ptr());
        };
        let mut k: lua_Integer = (*key).value_.i;
        if (0i32 as libc::c_longlong) < k &&
               k as lua_Unsigned <=
                   (1u32 <<
                        (::std::mem::size_of::<libc::c_int>() as
                             libc::c_ulong).wrapping_mul(8i32 as
                                                             libc::c_ulong).wrapping_sub(1i32
                                                                                             as
                                                                                             libc::c_ulong)
                            as libc::c_int) as libc::c_ulonglong {
            return k as libc::c_uint
        }
    }
    return 0i32 as libc::c_uint;
}
unsafe extern "C" fn numusehash(mut t: *const Table,
                                mut nums: *mut libc::c_uint,
                                mut pna: *mut libc::c_uint) -> libc::c_int {
    let mut totaluse: libc::c_int = 0i32;
    let mut ause: libc::c_int = 0i32;
    let mut i: libc::c_int = 1i32 << (*t).lsizenode as libc::c_int;
    loop  {
        let fresh1 = i;
        i = i - 1;
        if !(0 != fresh1) { break ; }
        let mut n: *mut Node =
            &mut *(*t).node.offset(i as isize) as *mut Node;
        if (*n).i_val.tt_ == 0i32 { continue ; }
        ause +=
            countint(&mut (*n).i_key.tvk as *mut TValue as *const TValue,
                     nums);
        totaluse += 1
    }
    *pna = (*pna).wrapping_add(ause as libc::c_uint);
    return totaluse;
}
unsafe extern "C" fn numusearray(mut t: *const Table,
                                 mut nums: *mut libc::c_uint)
 -> libc::c_uint {
    let mut lg: libc::c_int = 0;
    let mut ttlg: libc::c_uint = 0;
    let mut ause: libc::c_uint = 0i32 as libc::c_uint;
    let mut i: libc::c_uint = 1i32 as libc::c_uint;
    lg = 0i32;
    ttlg = 1i32 as libc::c_uint;
    while lg <=
              (::std::mem::size_of::<libc::c_int>() as
                   libc::c_ulong).wrapping_mul(8i32 as
                                                   libc::c_ulong).wrapping_sub(1i32
                                                                                   as
                                                                                   libc::c_ulong)
                  as libc::c_int {
        let mut lc: libc::c_uint = 0i32 as libc::c_uint;
        let mut lim: libc::c_uint = ttlg;
        if lim > (*t).sizearray {
            lim = (*t).sizearray;
            if i > lim { break ; }
        }
        while i <= lim {
            if !((*(*t).array.offset(i.wrapping_sub(1i32 as libc::c_uint) as
                                         isize)).tt_ == 0i32) {
                lc = lc.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        let ref mut fresh2 = *nums.offset(lg as isize);
        *fresh2 = (*fresh2).wrapping_add(lc);
        ause = ause.wrapping_add(lc);
        lg += 1;
        ttlg = ttlg.wrapping_mul(2i32 as libc::c_uint)
    }
    return ause;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getstr(mut t: *mut Table, mut key: *mut TString)
 -> *const TValue {
    if (*key).tt as libc::c_int == 4i32 | 0i32 << 4i32 {
        return luaH_getshortstr(t, key)
    } else {
        let mut ko: TValue =
            lua_TValue{value_:
                           Value_0{gc:
                                       0 as *const GCObject as
                                           *mut GCObject,},
                       tt_: 0,};
        let mut io: *mut TValue = &mut ko;
        let mut x_: *mut TString = key;
        if (*x_).tt as libc::c_int & 15i32 < 9i32 + 1i32 {
        } else {
            __assert_fail(b"(((x_)->tt) & 0x0F) < (9+1)\x00" as *const u8 as
                              *const libc::c_char,
                          b"ltable.c\x00" as *const u8 as *const libc::c_char,
                          562i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"const TValue *luaH_getstr(Table *, TString *)\x00")).as_ptr());
        };
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc as *mut GCObject;
        (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
        if 0 == (*io).tt_ & 1i32 << 6i32 ||
               {
                   if 0 != (*io).tt_ & 1i32 << 6i32 {
                   } else {
                       __assert_fail(b"(((io)->tt_) & (1 << 6))\x00" as
                                         *const u8 as *const libc::c_char,
                                     b"ltable.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     562i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 46],
                                                               &[libc::c_char; 46]>(b"const TValue *luaH_getstr(Table *, TString *)\x00")).as_ptr());
                   };
                   (*io).tt_ & 63i32 == (*(*io).value_.gc).tt as libc::c_int
                       &&
                       ((0 as *mut libc::c_void as *mut lua_State_0).is_null()
                            ||
                            {
                                if 0 != (*io).tt_ & 1i32 << 6i32 {
                                } else {
                                    __assert_fail(b"(((io)->tt_) & (1 << 6))\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"ltable.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  562i32 as libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 46],
                                                                            &[libc::c_char; 46]>(b"const TValue *luaH_getstr(Table *, TString *)\x00")).as_ptr());
                                };
                                0 !=
                                    ((*(*io).value_.gc).marked as libc::c_int
                                         ^ (1i32 << 0i32 | 1i32 << 1i32)) &
                                        ((*(*(0 as *mut libc::c_void as
                                                  *mut lua_State_0)).l_G).currentwhite
                                             as libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32))
                            })
               } {
        } else {
            if 0 != 0i32 {
            } else {
                __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                              b"ltable.c\x00" as *const u8 as
                                  *const libc::c_char, 562i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 46],
                                                        &[libc::c_char; 46]>(b"const TValue *luaH_getstr(Table *, TString *)\x00")).as_ptr());
            };
        };
        return getgeneric(t, &mut ko)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_new(mut L: *mut lua_State_0) -> *mut Table {
    let mut o: *mut GCObject =
        luaC_newobj(L, 5i32, ::std::mem::size_of::<Table>() as libc::c_ulong);
    if (*o).tt as libc::c_int == 5i32 {
    } else {
        __assert_fail(b"(o)->tt == 5\x00" as *const u8 as *const libc::c_char,
                      b"ltable.c\x00" as *const u8 as *const libc::c_char,
                      405i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"Table *luaH_new(lua_State *)\x00")).as_ptr());
    };
    let mut t: *mut Table = &mut (*(o as *mut GCUnion)).h as *mut Table_0;
    (*t).metatable = 0 as *mut Table_0;
    (*t).flags = !0i32 as lu_byte;
    (*t).array = 0 as *mut TValue;
    (*t).sizearray = 0i32 as libc::c_uint;
    setnodevector(L, t, 0i32 as libc::c_uint);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_resizearray(mut L: *mut lua_State_0,
                                          mut t: *mut Table,
                                          mut nasize: libc::c_uint) -> () {
    let mut nsize: libc::c_int =
        if (*t).lastfree.is_null() {
            0i32
        } else { 1i32 << (*t).lsizenode as libc::c_int };
    luaH_resize(L, t, nasize, nsize as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_free(mut L: *mut lua_State_0, mut t: *mut Table)
 -> () {
    if !(*t).lastfree.is_null() {
        luaM_realloc_(L, (*t).node as *mut libc::c_void,
                      ((1i32 << (*t).lsizenode as libc::c_int) as
                           size_t).wrapping_mul(::std::mem::size_of::<Node>()
                                                    as libc::c_ulong),
                      0i32 as size_t);
    }
    luaM_realloc_(L, (*t).array as *mut libc::c_void,
                  ((*t).sizearray as
                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>()
                                                       as libc::c_ulong),
                  0i32 as size_t);
    luaM_realloc_(L, t as *mut libc::c_void,
                  ::std::mem::size_of::<Table>() as libc::c_ulong,
                  0i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_next(mut L: *mut lua_State_0, mut t: *mut Table,
                                   mut key: StkId) -> libc::c_int {
    let mut i: libc::c_uint = findindex(L, t, key);
    while i < (*t).sizearray {
        if !((*(*t).array.offset(i as isize)).tt_ == 0i32) {
            let mut io: *mut TValue = key;
            (*io).value_.i =
                i.wrapping_add(1i32 as libc::c_uint) as lua_Integer;
            (*io).tt_ = 3i32 | 1i32 << 4i32;
            let mut io1: *mut TValue = key.offset(1isize);
            *io1 = *(*t).array.offset(i as isize);
            if 0 == (*io1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ltable.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         192i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 43],
                                                                   &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                       };
                       (*io1).tt_ & 63i32 ==
                           (*(*io1).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io1).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ltable.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      192i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 43],
                                                                                &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io1).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ltable.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  192i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 43],
                                                            &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                };
            };
            return 1i32
        } else { i = i.wrapping_add(1) }
    }
    i = i.wrapping_sub((*t).sizearray);
    while (i as libc::c_int) < 1i32 << (*t).lsizenode as libc::c_int {
        if !((*(*t).node.offset(i as isize)).i_val.tt_ == 0i32) {
            let mut io1_0: *mut TValue = key;
            *io1_0 =
                *(&mut (*(*t).node.offset(i as isize)).i_key.tvk as
                      *mut TValue as *const TValue);
            if 0 == (*io1_0).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ltable.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         198i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 43],
                                                                   &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                       };
                       (*io1_0).tt_ & 63i32 ==
                           (*(*io1_0).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io1_0).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ltable.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      198i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 43],
                                                                                &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io1_0).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ltable.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  198i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 43],
                                                            &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                };
            };
            let mut io1_1: *mut TValue = key.offset(1isize);
            *io1_1 = (*(*t).node.offset(i as isize)).i_val;
            if 0 == (*io1_1).tt_ & 1i32 << 6i32 ||
                   {
                       if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
                       } else {
                           __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"ltable.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         199i32 as libc::c_uint,
                                         (*::std::mem::transmute::<&[u8; 43],
                                                                   &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                       };
                       (*io1_1).tt_ & 63i32 ==
                           (*(*io1_1).value_.gc).tt as libc::c_int &&
                           (L.is_null() ||
                                {
                                    if 0 != (*io1_1).tt_ & 1i32 << 6i32 {
                                    } else {
                                        __assert_fail(b"(((io1)->tt_) & (1 << 6))\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      b"ltable.c\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      199i32 as libc::c_uint,
                                                      (*::std::mem::transmute::<&[u8; 43],
                                                                                &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                                    };
                                    0 !=
                                        ((*(*io1_1).value_.gc).marked as
                                             libc::c_int ^
                                             (1i32 << 0i32 | 1i32 << 1i32)) &
                                            ((*(*L).l_G).currentwhite as
                                                 libc::c_int ^
                                                 (1i32 << 0i32 |
                                                      1i32 << 1i32))
                                })
                   } {
            } else {
                if 0 != 0i32 {
                } else {
                    __assert_fail(b"0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"ltable.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  199i32 as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 43],
                                                            &[libc::c_char; 43]>(b"int luaH_next(lua_State *, Table *, StkId)\x00")).as_ptr());
                };
            };
            return 1i32
        } else { i = i.wrapping_add(1) }
    }
    return 0i32;
}
unsafe extern "C" fn findindex(mut L: *mut lua_State_0, mut t: *mut Table,
                               mut key: StkId) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    if (*key).tt_ == 0i32 {
        return 0i32 as libc::c_uint
    } else {
        i = arrayindex(key as *const TValue);
        if i != 0i32 as libc::c_uint && i <= (*t).sizearray {
            return i
        } else {
            let mut nx: libc::c_int = 0;
            let mut n: *mut Node = mainposition(t, key as *const TValue);
            loop  {
                if 0 !=
                       luaV_equalobj(0 as *mut lua_State_0,
                                     &mut (*n).i_key.tvk as *mut TValue as
                                         *const TValue, key as *const TValue)
                       ||
                       (*(&mut (*n).i_key.tvk as *mut TValue as
                              *const TValue)).tt_ == 9i32 + 1i32 &&
                           0 != (*key).tt_ & 1i32 << 6i32 &&
                           {
                               if (*(&mut (*n).i_key.tvk as *mut TValue as
                                         *const TValue)).tt_ == 9i32 + 1i32 {
                               } else {
                                   __assert_fail(b"((((((const TValue*)((&(n)->i_key.tvk)))))->tt_) == ((9+1)))\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"ltable.c\x00" as *const u8
                                                     as *const libc::c_char,
                                                 173i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 52],
                                                                           &[libc::c_char; 52]>(b"unsigned int findindex(lua_State *, Table *, StkId)\x00")).as_ptr());
                               };
                               if 0 != (*key).tt_ & 1i32 << 6i32 {
                               } else {
                                   __assert_fail(b"(((key)->tt_) & (1 << 6))\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 b"ltable.c\x00" as *const u8
                                                     as *const libc::c_char,
                                                 173i32 as libc::c_uint,
                                                 (*::std::mem::transmute::<&[u8; 52],
                                                                           &[libc::c_char; 52]>(b"unsigned int findindex(lua_State *, Table *, StkId)\x00")).as_ptr());
                               };
                               (*(&mut (*n).i_key.tvk as *mut TValue as
                                      *const TValue)).value_.gc as
                                   *mut libc::c_void ==
                                   (*key).value_.gc as *mut libc::c_void
                           } {
                    i =
                        (&mut *(*t).node.offset(0isize) as
                             *mut Node).offset_to(n).expect("bad offset_to")
                            as libc::c_long as libc::c_int as libc::c_uint;
                    return i.wrapping_add(1i32 as
                                              libc::c_uint).wrapping_add((*t).sizearray)
                } else {
                    nx = (*n).i_key.nk.next;
                    if nx == 0i32 {
                        luaG_runerror(L,
                                      b"invalid key to \'next\'\x00" as
                                          *const u8 as *const libc::c_char);
                    } else { n = n.offset(nx as isize) }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getn(mut t: *mut Table) -> libc::c_int {
    let mut j: libc::c_uint = (*t).sizearray;
    if j > 0i32 as libc::c_uint &&
           (*(*t).array.offset(j.wrapping_sub(1i32 as libc::c_uint) as
                                   isize)).tt_ == 0i32 {
        let mut i: libc::c_uint = 0i32 as libc::c_uint;
        while j.wrapping_sub(i) > 1i32 as libc::c_uint {
            let mut m: libc::c_uint =
                i.wrapping_add(j).wrapping_div(2i32 as libc::c_uint);
            if (*(*t).array.offset(m.wrapping_sub(1i32 as libc::c_uint) as
                                       isize)).tt_ == 0i32 {
                j = m
            } else { i = m }
        }
        return i as libc::c_int
    } else if (*t).lastfree.is_null() {
        return j as libc::c_int
    } else { return unbound_search(t, j) };
}
unsafe extern "C" fn unbound_search(mut t: *mut Table, mut j: libc::c_uint)
 -> libc::c_int {
    let mut i: libc::c_uint = j;
    j = j.wrapping_add(1);
    while !((*luaH_getint(t, j as lua_Integer)).tt_ == 0i32) {
        i = j;
        if j >
               (2147483647i32 as
                    libc::c_uint).wrapping_div(2i32 as libc::c_uint) {
            i = 1i32 as libc::c_uint;
            while !((*luaH_getint(t, i as lua_Integer)).tt_ == 0i32) {
                i = i.wrapping_add(1)
            }
            return i.wrapping_sub(1i32 as libc::c_uint) as libc::c_int
        } else { j = j.wrapping_mul(2i32 as libc::c_uint) }
    }
    while j.wrapping_sub(i) > 1i32 as libc::c_uint {
        let mut m: libc::c_uint =
            i.wrapping_add(j).wrapping_div(2i32 as libc::c_uint);
        if (*luaH_getint(t, m as lua_Integer)).tt_ == 0i32 {
            j = m
        } else { i = m }
    }
    return i as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_mainposition(mut t: *const Table,
                                           mut key: *const TValue)
 -> *mut Node {
    return mainposition(t, key);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_isdummy(mut t: *const Table) -> libc::c_int {
    return ((*t).lastfree == 0 as *mut libc::c_void as *mut Node) as
               libc::c_int;
}
