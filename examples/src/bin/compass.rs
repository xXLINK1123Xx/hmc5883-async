//! Connect SDA to P0.03, SCL to P0.04
//! $ DEFMT_LOG=info cargo rb simple

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// Formulas and compass logic found:
// https://github.com/kalkyl/qmc5883l-async/blob/main/examples/src/bin/compass.rs
const DECLINATION_RADS: f32 = 0.122173048;

use compass_example as _; // global logger + panicking-behavior

use defmt::info;
use embassy::executor::Spawner;
use embassy::time::{Delay, Duration, Timer};
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{interrupt, Peripherals};
use hmc5883_async::*;
use libm::atan2;
use core::f32::consts::PI;

/// This example demonstrate how the sensor
/// returns temperature and magnitude vector data
/// We then apply a formula found in Arduino
/// to get the North (towards zero or 360°).
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

        match hmc.get_mag_vector().await {
            Ok([x,y,z]) => {

                let mut heading = atan2(y as f64, x as f64) as f32 + DECLINATION_RADS;
                if heading < 0.0 {
                    heading += 2.0 * PI;
                } else if heading > 2.0 * PI {
                    heading -= 2.0 * PI;
                }
                let heading_degrees = heading * 180.0 / PI;
                info!(
                    "x={}, y={}, z={}: heading={} degrees",
                    x, y, z, heading_degrees
                );
            },
            Err(e) => info!("Error {}", e),
        }

        Timer::after(Duration::from_millis(500)).await;
    }
}
