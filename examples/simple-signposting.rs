use c_str_macro::c_str;
// use oslog::to_cstr;
use oslog::OSSignpostID;
use oslog::OsLog;

fn main() {
    let log = OsLog::new("com.signposter", "the-category");

    let signpost_id = OSSignpostID::generate(&log);
    log.signpost_event(
        &signpost_id,
        c_str!("the-signpost-name"),
        c_str!("%{public}s"),
        c_str!("the-signpost-message"),
    );
    log.signpost_event(
        &signpost_id,
        c_str!("the-signpost-name2"),
        c_str!("%{public}s"),
        c_str!("the-signpost-message2"),
    );
}
