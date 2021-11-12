// #![feature(asm)]

// #[no_mangle]
// pub unsafe extern "system" fn foo_impl() {
// }
#[export_name="foo"]
pub unsafe extern "system" fn foo() {
}
