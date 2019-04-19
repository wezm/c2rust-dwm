fn main() {
    println!("cargo:rustc-flags=-l Xinerama");
    println!("cargo:rustc-flags=-l fontconfig");
    println!("cargo:rustc-flags=-l Xft");
    println!("cargo:rustc-flags=-l X11");
}
