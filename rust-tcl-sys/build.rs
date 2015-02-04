#![feature(os)]

extern crate "pkg-config" as pkg_config;

fn main() {
    if std::os::getenv("CARGO_FEATURE_USE_PKGCONFIG").is_some() {
      if build_pkgconfig() { return; }
      panic!("Could not find SDL2 via pkgconfig");
    } else {
      println!("cargo:rustc-flags=-l tcl");
    }
}

fn build_pkgconfig() -> bool {
    let opts = pkg_config::default_options("tcl");
    pkg_config::find_library_opts("tcl", &opts).is_ok()
}
