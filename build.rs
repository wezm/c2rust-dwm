use std::os::unix::ffi::OsStrExt;

use pkg_config;
use std::io::{stdout, Write};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();

    link_lib(&mut stdout, "fontconfig");
    link_lib(&mut stdout, "x11");
    link_lib(&mut stdout, "xinerama");
    link_lib(&mut stdout, "xft");
}

fn link_lib<W: Write>(write: &mut W, lib: &str) {
    let config = pkg_config::probe_library(lib).unwrap();

    for link_path in config.link_paths {
        write!(write, "cargo:rustc-link-search=native=").unwrap();
        write.write_all(link_path.as_os_str().as_bytes()).unwrap();
        write.write_all(b"\n").unwrap();
    }

    for lib in config.libs {
        writeln!(write, "cargo:rustc-link-lib={}", lib);
    }
}
