//! examples/async-channel-done.rs

#![no_main]
#![no_std]
//#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![feature(type_alias_impl_trait)]


#[rtic::app(device = esp32c3, dispatchers = [FROM_CPU_INTR1])]
mod app {
    use panic_rtt_target as _;
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
        let (s, r) = make_channel!(u32, CAPACITY);

        receiver::spawn(r).unwrap();
        sender1::spawn(s.clone()).unwrap();
        sender2::spawn(s.clone()).unwrap();
        sender3::spawn(s).unwrap();

        (Shared {}, Local {})
    }

    #[task]
    async fn receiver(_c: receiver::Context, mut receiver: Receiver<'static, u32, CAPACITY>) {
        while let Ok(val) = receiver.recv().await {
            rprintln!("Receiver got: {}", val);
            if val == 3 {
                rprintln!("Finished");
            }
        }
    }

    #[task]
    async fn sender1(_c: sender1::Context, mut sender: Sender<'static, u32, CAPACITY>) {
        rprintln!("Sender 1 sending: 1");
        sender.send(1).await.unwrap();
        rprintln!("Sender 1 done");
    }

    #[task]
    async fn sender2(_c: sender2::Context, mut sender: Sender<'static, u32, CAPACITY>) {
        rprintln!("Sender 2 sending: 2");
        sender.send(2).await.unwrap();
        rprintln!("Sender 2 done");
    }

    #[task]
    async fn sender3(_c: sender3::Context, mut sender: Sender<'static, u32, CAPACITY>) {
        rprintln!("Sender 3 sending: 3");
        sender.send(3).await.unwrap();
        rprintln!("Sender 3 done");
    }
}
