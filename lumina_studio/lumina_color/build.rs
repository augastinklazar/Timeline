fn main() {
    cxx_build::bridge("src/ocio.rs")
        .flag_if_supported("-std=c++14")
        .compile("lumina_ocio");

    // Link against the system OCIO installed via apt
    println!("cargo:rustc-link-lib=OpenColorIO");
}
