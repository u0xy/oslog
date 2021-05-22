use oslog::{c_str, OSSignpostID, OSLog};

fn main() {
    let log = OSLog::new("com.signposter", "the-category");

    let signpost_id = OSSignpostID::default();
    log.signpost_event(
        &signpost_id,
        c_str!("the-default-signpost-name"),
        c_str!("%{public}s"),
        c_str!("the-default-signpost-message"),
    );
    log.signpost_event(
        &signpost_id,
        c_str!("the-default-signpost-name2"),
        c_str!("%{public}s"),
        c_str!("the-default-signpost-message2"),
    );

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

    let ref_object = String::from("reference-object");
    let signpost_id = OSSignpostID::generate_with_pointer(&log, &ref_object);
    log.signpost_event(
        &signpost_id,
        c_str!("the-ref-signpost-name"),
        c_str!("%{public}s"),
        c_str!("the-ref-signpost-message"),
    );
    log.signpost_event(
        &signpost_id,
        c_str!("the-ref-signpost-name2"),
        c_str!("%{public}s"),
        c_str!("the-ref-signpost-message2"),
    );
}
