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
