# smart-meter

[![CI](https://github.com/schphil/smart-meter/workflows/CI/badge.svg)](https://github.com/schphil/smart-meter/actions?query=workflow%3ACI)
[![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/schphil/smart-meter)

This is work-in-progress! 

This library intends to help read data from several smart meter devices. 

This should work with mac, linux and esp32 (riscv) with either use of esp-idf-hal or esp-hal from [esp-rs](https://github.com/esp-rs).

## supported smart meters

## example

# LandisGyr T550 

```rust
cargo build --example landisgyrt550
```

## usage 

```rust
use env_logger::Env;

use smart_meter::{
    device::device::Device,
    error::Error,
    smart_meter::{Interface, Protocol, SmartMeter},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = Env::default().filter_or("RUST_LOG", "smart_meter=info");
    env_logger::init_from_env(env);

    let serial_path = "/dev/**";

    let smart_meter = SmartMeter {
        device: Device::LandGyrT550,
        protocol: Protocol::EN6205621,
        interface: Interface::Serial(serial_path.to_string()),
    };

    let data = smart_meter.request_data().await.unwrap();
    log::info!("Device data: {:?}", data);

    Ok(())
}
```

# LandisGyr T550 esp std

```rust
cargo +nightly build -Z build-std=std,panic_abort --no-default-features --features=esp_std,esp-idf-sys --target riscv32imc-esp-espidf --example landisgyrt550_esp_std
```

# LandisGyr T550 esp 

```rust
cargo +nightly build -Z build-std=core --no-default-features --features=esp,esp_example --target riscv32imac-unknown-none-elf --example landisgyrt550_esp
```

## License

`smart-meter` is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.