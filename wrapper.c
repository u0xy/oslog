#include <os/log.h>
#include <os/signpost.h>

#define to_literal(lit) #lit
#define wrap(log, spid, name, message) \
    void wrapped(os_log_t log, os_signpost_id_t spid, const char* message) { \
        os_signpost_event_emit(log, spid, name, "%{public}s", message); \
    }

os_log_t wrapped_get_default_log() {
    return OS_LOG_DEFAULT;
}

void wrapped_os_log_with_type(os_log_t log, os_log_type_t type, const char* message) {
    os_log_with_type(log, type, "%{public}s", message);
}

void wrapped_os_log_debug(os_log_t log, const char* message) {
    os_log_debug(log, "%{public}s", message);
}

void wrapped_os_log_info(os_log_t log, const char* message) {
    os_log_info(log, "%{public}s", message);
}

void wrapped_os_log_default(os_log_t log, const char* message) {
    os_log(log, "%{public}s", message);
}

void wrapped_os_log_error(os_log_t log, const char* message) {
    os_log_error(log, "%{public}s", message);
}

void wrapped_os_log_fault(os_log_t log, const char* message) {
    os_log_fault(log, "%{public}s", message);
}

// -- source: os/trace_base.h: lines 94-101
//
// modification: avoid using OS_LOG_STRING which forces static const strings
// for performance.
//
// initial inspiration: https://github.com/flutter/flutter/issues/47771
// and https://dart-review.googlesource.com/c/sdk/+/131360/10/runtime/vm/timeline_macos.cc#37

#define MOD_OS_LOG_CALL_WITH_FORMAT_NAME(fun, fun_args, name, fmt, ...) __extension__({ \
        OS_LOG_PRAGMA_PUSH \
        uint8_t _Alignas(16) OS_LOG_UNINITIALIZED _os_fmt_buf[__builtin_os_log_format_buffer_size(fmt, ##__VA_ARGS__)]; \
        fun(OS_LOG_REMOVE_PARENS fun_args, name, fmt, \
                (uint8_t *)__builtin_os_log_format(_os_fmt_buf, fmt, ##__VA_ARGS__), \
                (uint32_t)sizeof(_os_fmt_buf)) OS_LOG_PRAGMA_POP; \
})

// --


// -- source: os/signpost.h: line 383
//
// modification: follow the chain of macros, replacing most original calls with
// mod_* in order to end up calling MOD_OS_LOG_CALL_WITH_FORMAT_NAME.

#define _mod_os_signpost_emit_with_type(emitfn, log, type, spid, name, ...) \
    __extension__({ \
        os_log_t _log_tmp = (log); \
        os_signpost_type_t _type_tmp = (type); \
        os_signpost_id_t _spid_tmp = (spid); \
        if (_spid_tmp != OS_SIGNPOST_ID_NULL && \
                _spid_tmp != OS_SIGNPOST_ID_INVALID && \
                os_signpost_enabled(_log_tmp)) { \
            MOD_OS_LOG_CALL_WITH_FORMAT_NAME((emitfn), \
                    (&__dso_handle, _log_tmp, _type_tmp, _spid_tmp), \
                    name, "" __VA_ARGS__); \
        } \
    })

#if OS_LOG_TARGET_HAS_10_14_FEATURES
#define mod_os_signpost_emit_with_type(log, type, spid, name, ...) \
        _mod_os_signpost_emit_with_type(_os_signpost_emit_with_name_impl, log, \
                type, spid, name, ##__VA_ARGS__)
#else
#define mod_os_signpost_emit_with_type(log, type, spid, name, ...) \
    __extension__({ \
        if (_os_signpost_emit_with_name_impl != NULL) { \
            _mod_os_signpost_emit_with_type(_os_signpost_emit_with_name_impl, log, \
                    type, spid, name, ##__VA_ARGS__); \
        } \
    })
#endif

// --


void wrapped_os_signpost_event_emit(os_log_t log, os_signpost_id_t spid, const char* name, const char* format, const char* message) {
    printf("%s", format);
    mod_os_signpost_emit_with_type(log, OS_SIGNPOST_EVENT, spid, name, "%s", message);
}
    // _os_signpost_emit_with_name_impl(&__dso_handle, log, OS_SIGNPOST_EVENT, spid, name, format, message, strlen(message));
