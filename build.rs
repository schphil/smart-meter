fn main() {
    if cfg!(feature = "esp_std") {
        println!("cargo:rustc-cfg=esp_std");
        println!("cargo:rustc-env=ESP_IDF_VERSION=release/v4.4");
        embuild::build::CfgArgs::output_propagated("ESP_IDF").unwrap();
        embuild::build::LinkArgs::output_propagated("ESP_IDF").unwrap();
    } else if cfg!(feature = "esp") {
        println!("cargo:rustc-cfg=esp");
        // println!("cargo:rustc-flags=-C force-frame-pointers");
        println!("cargo:rustc-link-arg-examples=-Tlinkall.x");
        // println!("cargo:rustc-link-arg-examples=-Tesp32c3_rom_functions.x");
    } else {
        println!("cargo:rustc-cfg=default");
    }
}
