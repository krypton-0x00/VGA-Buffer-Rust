
#![no_std] 
#![no_main]

#[allow(unused_imports)]
use core::panic::PanicInfo;


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

#[no_mangle]
pub extern "C" fn _start()-> !{ 
    loop{}
}
