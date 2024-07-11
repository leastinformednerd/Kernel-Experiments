#![no_std]
#![no_main]

#[used]
#[link_section = ".requests"]
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: limine::request::FramebufferRequest = limine::request::FramebufferRequest::new();

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());    

    loop {}
}
