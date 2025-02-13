mod temperature;
mod utils;

use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use std::thread;
use std::time::Duration;

fn main() {
    esp_idf_sys::link_patches(); // Necessary for linking ESP-IDF correctly

    thread::spawn(move || {
        let peripherals = Peripherals::take().expect("Failed to take peripherals");
        let mut led = PinDriver::output(peripherals.pins.gpio8).expect("Unable to get led"); // Set GPIO2 as output
        loop {
            led.toggle().expect("Unable to toggle led");
            println!("LED Toggled!");
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let temp_sensor =
            temperature::TemperatureSensor::new().expect("Unable to get temperature sensor");

        loop {
            match temp_sensor.read_temperature() {
                Ok(temp) => {
                    println!("Temperature: {:.2}°C", temp);
                }
                Err(e) => {
                    println!("Error reading temperature: {:?}", e);
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        thread::sleep(Duration::from_secs(10));
        println!("Heartbeat");
    }
}
