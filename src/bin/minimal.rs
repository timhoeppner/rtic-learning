#![no_main]
#![no_std]

use rtic_learning as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0])]
mod app {
    // use stm32f4xx_hal::pac::Interrupt;
    use rtic_learning::{
        debug
    };
    use fugit::ExtU32;
    use stm32f4xx_hal::{pac, prelude::*, timer::MonoTimerUs, gpio::{Output, PushPull, PA5}};

    const CRYSTAL_FREQ: u32 = 8_000_000;
    const MAIN_FREQ: u32 = 84_000_000;
    const PCLK1_FREQ: u32 = 42_000_000;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PA5<Output<PushPull>>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::println!("Hello, world!");

        let core: cortex_m::Peripherals = ctx.core;
        let device: stm32f4xx_hal::pac::Peripherals = ctx.device;

        debug::enable_debug_during_sleep(&device);

        let rcc = device.RCC.constrain();
        // let clocks = rcc.cfgr.sysclk(FREQ.hz()).freeze();
        let clocks = rcc.cfgr
            .use_hse(CRYSTAL_FREQ.Hz())
            .sysclk(MAIN_FREQ.Hz())
            .pclk1(PCLK1_FREQ.Hz())
            .pclk2(MAIN_FREQ.Hz())
            .freeze();
        let mono = device.TIM2.monotonic_us(&clocks);

        // let mut timer = device.TIM1.counter_us(&clocks);

        let gpioa = device.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        led.set_high();

        foo::spawn().unwrap();
        bar::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
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

    #[task(local = [led])]
    fn foo(ctx: foo::Context) {
        defmt::println!("in foo");

        ctx.local.led.toggle();

        foo::spawn_after(1.secs()).ok();
    }

    #[task]
    fn bar(_: bar::Context) {
        defmt::println!("in bar");

        bar::spawn_after(10.secs()).ok();
    }
}
