//! Connect SDA to P0.03, SCL to P0.04
//! $ DEFMT_LOG=info cargo rb simple

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use nrf_embassy as _; // global logger + panicking-behavior

use defmt::info;
use embassy::executor::Spawner;
use embassy::time::{Delay, Duration, Timer};
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{interrupt, Peripherals};
use hmc5883_async::*;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let i2c = Twim::new(p.TWISPI0, irq, p.P0_03, p.P0_04, config);

    let mut hmc = HMC5983::new(i2c);

    hmc.init(&mut Delay).await.expect("init failed");

    loop {
        if let Ok(temp) = hmc.get_temperature().await {
            info!("Temperature: {:?}", temp);
        }
        // Go and unquote if !Self::reading_in_range(&sample_i16) 
        // on l. 243 in the driver to get bad readings :)
        match hmc.get_mag_vector().await {
            Ok(mag) => info!("Magnitude vector: {:?}", mag),
            Err(e) => info!("Error {}", e),
        }

        Timer::after(Duration::from_secs(3)).await;
    }
}