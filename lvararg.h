#ifndef lvararg_h
#define lvararg_h

#include "lua.h"

#include "lstate.h"
/*
 * lua.h
 */
LUA_API const char *(lua_pushfstring) (lua_State *L, const char *fmt, ...);


/*
 * lauxlib.h
 */
LUALIB_API int (luaL_error) (lua_State *L, const char *fmt, ...);


/*
 * ldebug.h
 */
LUAI_FUNC l_noret luaG_runerror (lua_State *L, const char *fmt, ...);


/*
 * lobject.h
 */
LUAI_FUNC const char *luaO_pushfstring (lua_State *L, const char *fmt, ...);



#endif
