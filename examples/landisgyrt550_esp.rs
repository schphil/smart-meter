#![no_std]
#![no_main]
#![feature(c_variadic)]
#![feature(const_mut_refs)]

use core::cell::RefCell;
use critical_section::Mutex;
use esp32c3_hal::{
    clock::{ClockControl, CpuClock},
    pac::Peripherals,
    prelude::*,
    serial::TxRxPins,
    timer::TimerGroup,
    Rtc,
    IO,
};
use esp_backtrace as _;
use esp_println::{logger::init_logger, println};
use smart_meter::{
    device::device::Device,
    smart_meter::{Interface, Protocol, SmartMeter},
};

#[riscv_rt::entry]
fn main() -> ! {
    println!("Init!");
    init_logger(log::LevelFilter::Info);

    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock160MHz).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio4.into_push_pull_output(),
        io.pins.gpio5.into_floating_input(),
    );
    let uart1 = peripherals.UART1;

    let pins_mutex = Mutex::new(RefCell::new(Some(pins)));
    let uart1_mutex = Mutex::new(RefCell::new(Some(uart1)));
    let clocks_mutex = Mutex::new(RefCell::new(clocks));

    let smart_meter = SmartMeter {
        device: Device::LandGyrT550,
        protocol: Protocol::EN6205621,
        interface: Interface::Serial(pins_mutex, uart1_mutex, clocks_mutex),
    };

    let data = smart_meter.request_data().unwrap();
    println!("Device data: {:?}", data);

    loop {}
}
