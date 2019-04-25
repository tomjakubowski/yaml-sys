// Whitelisting most of the functions listed in "Parser API Synopsis" and "Emitter API Synopsis"
// `FILE *` API not supported.
// https://pyyaml.org/wiki/LibYAML

use bindgen::Builder as BindgenBuilder;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=yaml");

    let bindings = BindgenBuilder::default()
        .header("wrapper.h")
        // Header says: "All members are internal" for these types, and the members pull in tons of
        // garbage to the bindings.
        .opaque_type("yaml_parser_s")
        .opaque_type("yaml_emitter_s")
        // For some reason this is necessary
        .blacklist_type("yaml_parser_s__bindgen.*")
        .blacklist_type("yaml_emitter_s__bindgen.*")
        // Parser API
        .whitelist_type("yaml_parser_t")
        .whitelist_type("yaml_event_t")
        .whitelist_function("yaml_parser_initialize")
        .whitelist_function("yaml_parser_set_input_string")
        .whitelist_function("yaml_parser_set_input")
        .whitelist_function("yaml_parser_parse")
        .whitelist_function("yaml_event_delete")
        .whitelist_function("yaml_parser_delete")
        // Emitter API
        .whitelist_type("yaml_emitter_t")
        .whitelist_function("yaml_emitter_initialize")
        .whitelist_function("yaml_emitter_set_output")
        .whitelist_function("yaml_stream_start_event_initialize")
        .whitelist_function("yaml_emitter_emit")
        .whitelist_function("yaml_stream_end_event_initialize")
        .whitelist_function("yaml_emitter_delete")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
