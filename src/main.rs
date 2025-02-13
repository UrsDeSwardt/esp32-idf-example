mod temperature;

fn main() {
    esp_idf_sys::link_patches(); // Necessary for linking ESP-IDF correctly

    let sensor = temperature::TemperatureSensor::new().unwrap();

    loop {
        match sensor.read_temperature() {
            Ok(temp) => {
                println!("Temperature: {:.2}°C", temp);
            }
            Err(e) => {
                println!("Error reading temperature: {:?}", e);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
