#![no_std]

#[link(name = "usbsys")]
extern "C" {
    /// Initialize the USB PLL and clocks.
    ///
    /// This must be invoked before `usb_init()`.
    pub fn usb_pll_start();
    /// Initialize the USB module.
    pub fn usb_init();
    /// Runs the interrupt service routine.
    pub fn isr();

    fn usb_serial_write(buffer: *const u8, size: u32) -> i32;

    /// Flush the serial buffer
    pub fn usb_serial_flush_output();
}

pub fn serial_write<B: AsRef<[u8]>>(buffer: &B) {
    unsafe {
        let buffer = buffer.as_ref();
        usb_serial_write(buffer.as_ptr(), buffer.len() as u32);
    }
}
