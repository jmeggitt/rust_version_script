

pub fn main() {
    println!("cargo:rerun-if-changed=*");

    println!("cargo:rustc-cdylib-link-arg=-fuse-ld=lld");
    println!("cargo:rustc-cdylib-link-arg=-Wl,--version-script=version_script");

    // for symbol in &["foo"] {
    //     println!("cargo:rustc-cdylib-link-arg=-Wl,--defsym={}={}_impl", symbol, symbol);
    // }

}