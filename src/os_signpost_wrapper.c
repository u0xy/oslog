#include "mod_os_signpost.h"

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
