#![no_main]
#![no_std]

use rtic_learning as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0])]
mod app {
    // use stm32f4xx_hal::pac::Interrupt;
    use rtic_learning::mono::MonoTimer;
    use fugit::ExtU32;
    use stm32f4xx_hal::{pac, prelude::*};

    const FREQ: u32 = 48_000_000;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[monotonic(binds = TIM2, default = true)]
    type MyMono = MonoTimer<pac::TIM2, 1_000_000>;

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::println!("Hello, world!");

        let _core: cortex_m::Peripherals = ctx.core;
        let device: stm32f4xx_hal::pac::Peripherals = ctx.device;

        // The following block allows defmt to work even when going to sleep (WFI)
        // See https://github.com/probe-rs/probe-rs/issues/350#issuecomment-740550519
        device.DBGMCU.cr.modify(|_, w| {
            w.dbg_sleep().set_bit();
            w.dbg_standby().set_bit();
            w.dbg_stop().set_bit()
        });
        device.RCC.ahb1enr.modify(|_, w| w.dma1en().enabled());

        // Initialization required for MyMono...
        device.RCC.apb1enr.modify(|_, w| w.tim2en().set_bit());
        device.RCC.apb1rstr.modify(|_, w| w.tim2rst().set_bit());
        device.RCC.apb1rstr.modify(|_, w| w.tim2rst().clear_bit());

        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(FREQ.hz()).freeze();
        let mono = MyMono::new(device.TIM2, &clocks);

        // core.SCB.set_sleepdeep();

        let gpioa = device.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        led.set_high();

        foo::spawn().unwrap();
        bar::spawn().unwrap();

        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        // defmt::println!("in idle");

        loop {
            // Use this to just spin useless cycles
            cortex_m::asm::nop();

            // Use this for going to sleep during idle
            // rtic::export::wfi();
        }
    }

    #[task]
    fn foo(_: foo::Context) {
        defmt::println!("in foo");

        foo::spawn_after(1.secs()).ok();
    }

    #[task]
    fn bar(_: bar::Context) {
        defmt::println!("in bar");

        bar::spawn_after(10.secs()).ok();
    }
}
