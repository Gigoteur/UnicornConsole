#include "duktape.h"

DUK_EXTERNAL duk_context *_duk_create_heap_default(void) {
    return duk_create_heap_default();
}

DUK_EXTERNAL void _duk_eval_string(duk_context *ctx, const char *src) {
    duk_eval_raw((ctx), (src), 0, 0 /*args*/ | DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN | DUK_COMPILE_NOFILENAME);
}