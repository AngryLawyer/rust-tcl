#[cfg(feature="pkg-config")]
extern crate pkg_config;

fn main() {
    if !build_pkgconfig() {
      println!("cargo:rustc-flags=-l tcl8.5");
    }
}


#[cfg(not(feature="pkg-config"))]
fn build_pkgconfig() -> bool {
    false
}

#[cfg(feature="pkg-config")]
fn build_pkgconfig() -> bool {
    let opts = pkg_config::default_options("tcl8.5");
    if pkg_config::find_library_opts("tcl8.5", &opts).is_err() {
        panic!("Could not find Tcl via pkgconfig");
    }
    true
}
