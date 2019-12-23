#[cfg(feature = "use-bindgen")]
use bindgen;
#[cfg(feature = "use-bindgen")]
use std::env;

#[cfg(feature = "use-bindgen")]
use std::path::PathBuf;

#[cfg(feature = "use-bindgen")]
fn main() {
    let libnds_header: PathBuf = [
        &env::var("DEVKITPRO").unwrap(),
        "libnds",
        "include",
        "nds.h",
    ]
    .iter()
    .collect();

    let libnds_include: PathBuf = [&env::var("DEVKITPRO").unwrap(), "libnds", "include"]
        .iter()
        .collect();

    let libnds_system_include: PathBuf =
        [&env::var("DEVKITARM").unwrap(), "arm-none-eabi", "include"]
            .iter()
            .collect();

    let libnds_stdio_header = libnds_system_include.join("stdio.h");

    let bindings = bindgen::Builder::default()
        .header(libnds_header.to_str().unwrap())
        .header(libnds_stdio_header.to_str().unwrap())
        .trust_clang_mangling(false)
        .use_core()
        .clang_arg("-DARM9")
        .clang_arg("-DNDEBUG")
        .clang_arg("-isystem".to_owned() + libnds_include.to_str().unwrap())
        .clang_arg("-isystem".to_owned() + libnds_system_include.to_str().unwrap())
        .prepend_enum_name(false)
        .blacklist_type("u(8|16|32|64)")
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "use-bindgen"))]
fn main() {}
