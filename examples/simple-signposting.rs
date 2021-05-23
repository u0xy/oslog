use oslog::{cstr, OSLog, OSSignpostID};

fn main() {
    let log = OSLog::new("com.signposter", "the-category");

    let signpost_id = OSSignpostID::default();
    log.signpost_event(
        &signpost_id,
        cstr!("the-default-signpost-name"),
        cstr!("%{public}s"),
        cstr!("the-default-signpost-message"),
    );
    log.signpost_event(
        &signpost_id,
        cstr!("the-default-signpost-name2"),
        cstr!("%{public}s"),
        cstr!("the-default-signpost-message2"),
    );

    let signpost_id = OSSignpostID::generate(&log);
    log.signpost_event(
        &signpost_id,
        cstr!("the-signpost-name"),
        cstr!("%{public}s"),
        cstr!("the-signpost-message"),
    );
    log.signpost_event(
        &signpost_id,
        cstr!("the-signpost-name2"),
        cstr!("%{public}s"),
        cstr!("the-signpost-message2"),
    );

    let ref_object = String::from("reference-object");
    let signpost_id = OSSignpostID::generate_with_pointer(&log, &ref_object);
    log.signpost_event(
        &signpost_id,
        cstr!("the-ref-signpost-name"),
        cstr!("%{public}s"),
        cstr!("the-ref-signpost-message"),
    );
    log.signpost_event(
        &signpost_id,
        cstr!("the-ref-signpost-name2"),
        cstr!("%{public}s"),
        cstr!("the-ref-signpost-message2"),
    );
}
