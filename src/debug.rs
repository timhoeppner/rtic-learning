pub fn enable_debug_during_sleep(device: &stm32f4xx_hal::pac::Peripherals) {
    // See https://github.com/probe-rs/probe-rs/issues/350#issuecomment-740550519

    // The following block allows defmt to work even when going to sleep (WFI)
    device.DBGMCU.cr.modify(|_, w| {
        w.dbg_sleep().set_bit();
        w.dbg_standby().set_bit();
        w.dbg_stop().set_bit()
    });

    // This enables the DMA clock on AHB bus so something is always active
    device.RCC.ahb1enr.modify(|_, w| w.dma1en().enabled());
}