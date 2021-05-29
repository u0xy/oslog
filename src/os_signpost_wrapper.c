#include "mod_os_signpost.h"

void va_os_signpost_event_emit_with_type(os_log_t log, os_signpost_type_t spty, os_signpost_id_t spid, const char* name, ...) {
    va_list ap;
    va_start(ap, name);
    mod_os_signpost_emit_with_type(log, spty, spid, name, "%s", "hello");
    va_end(ap);
}

// void va2_os_signpost_event_emit_with_type(os_log_t log, os_signpost_type_t spty, os_signpost_id_t spid, const char* name, const char* format, const char* message) {

//     va_list ap;
//     va_start(ap, name);
//     uint8_t _Alignas(16) OS_LOG_UNINITIALIZED _os_fmt_buf[__builtin_os_log_format_buffer_size(ap)];
//     _os_signpost_emit_with_name_impl(log, spty, spid, name);
//     va_end(ap);
// }

void wrapped_os_signpost_event_emit(os_log_t log, os_signpost_id_t spid, const char* name, const char* format, const char* message) {
    printf("%s", format);
    mod_os_signpost_emit_with_type(log, OS_SIGNPOST_EVENT, spid, name, "%s", message);
}

void wrapped_os_signpost_interval_begin(os_log_t log, os_signpost_id_t spid, const char* name, const char* format, const char* message) {
    printf("%s", format);
    mod_os_signpost_emit_with_type(log, OS_SIGNPOST_INTERVAL_BEGIN, spid, name, "%s", message);
}

void wrapped_os_signpost_interval_end(os_log_t log, os_signpost_id_t spid, const char* name, const char* format, const char* message) {
    printf("%s", format);
    mod_os_signpost_emit_with_type(log, OS_SIGNPOST_INTERVAL_END, spid, name, "%s", message);
}
