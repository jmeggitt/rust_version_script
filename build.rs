pub fn main() {
    println!("cargo:rerun-if-changed=*");

    // We need to use lld since the default uses an anonymous version script and prevents any
    // additional version scripts from being used
    println!("cargo:rustc-cdylib-link-arg=-fuse-ld=lld");

    // Add the version script to the linker args
    println!("cargo:rustc-cdylib-link-arg=-Wl,--version-script=version_script");

    // Create new symbols for exported functions so our version script can be applied to them
    // without being overwritten by the one provided by Rust.
    for symbol in &["foo"] {
        println!("cargo:rustc-cdylib-link-arg=-Wl,--defsym={}={}_impl", symbol, symbol);
    }
}