//! examples/async-channel-no-sender.rs

#![no_main]
#![no_std]
//#![deny(warnings)]
//#![deny(unsafe_code)]
#![deny(missing_docs)]
#![feature(type_alias_impl_trait)]

use panic_rtt_target as _;

#[rtic::app(device = esp32c3, dispatchers = [FROM_CPU_INTR1])]
mod app {
    use rtt_target::{rtt_init_print, rprintln};
    use rtic_sync::{channel::*, make_channel};
    use esp32c3_hal::{riscv};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    const CAPACITY: usize = 1;
    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        rtt_init_print!();
        let (_s, r) = make_channel!(u32, CAPACITY);
       // unsafe{riscv::asm::nop()};

        //the pointer is pc, so it's as if the instruction read is unaligned causing an exception, and when the pointer is dereferenced in exception handler 
        //it panics

        //but why is instruction read unaligned
        //and this makes sense since 0x42000xxxx is program memory

        //could it be that compressed instructions are causing this? eg. pc is pointing at compressed instruction, which is 2 bytes, causing the misaligned access.

        //why is the nop needed?? must look into it
        //exception is thrown otherwise with some misaligned access, so it doesn't even get to exception just panic
        //this worked fine before...
        receiver::spawn(r).unwrap();

        (Shared {}, Local {})
    }

    #[task]
    async fn receiver(_c: receiver::Context, mut receiver: Receiver<'static, u32, CAPACITY>) {
        rprintln!("Receiver got: {:?}", receiver.recv().await);

        rprintln!("Finished");
    }
}
