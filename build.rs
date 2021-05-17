fn main() {
    cc::Build::new()
        .file("src/os_log_wrapper.c")
        .file("src/os_signpost_wrapper.c")
        .include("src")
        .compile("wrapper");
}
