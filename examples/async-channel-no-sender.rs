//! examples/async-channel-no-sender.rs

#![no_main]
#![no_std]
//#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![feature(type_alias_impl_trait)]

use panic_rtt_target as _;

#[rtic::app(device = esp32c3, dispatchers = [FROM_CPU_INTR1])]
mod app {
    use rtt_target::{rtt_init_print, rprintln};
    use rtic_sync::{channel::*, make_channel};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    const CAPACITY: usize = 1;
    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        rtt_init_print!();
        let (_s, r) = make_channel!(u32, CAPACITY);

        receiver::spawn(r).unwrap();

        (Shared {}, Local {})
    }

    #[task]
    async fn receiver(_c: receiver::Context, mut receiver: Receiver<'static, u32, CAPACITY>) {
        rprintln!("Receiver got: {:?}", receiver.recv().await);

        rprintln!("Finished");
    }
}
