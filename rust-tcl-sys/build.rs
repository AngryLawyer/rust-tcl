#![feature(os)]

extern crate "pkg-config" as pkg_config;

fn main() {
    if std::os::getenv("CARGO_FEATURE_USE_PKGCONFIG").is_some() {
      if build_pkgconfig() { return; }
      panic!("Could not find Tcl via pkgconfig");
    } else {
      println!("cargo:rustc-flags=-l tcl8.5");
    }
}

fn build_pkgconfig() -> bool {
    let opts = pkg_config::default_options("tcl8.5");
    pkg_config::find_library_opts("tcl8.5", &opts).is_ok()
}
