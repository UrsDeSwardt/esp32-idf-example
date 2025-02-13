use esp_idf_sys::{esp_deep_sleep, esp_light_sleep_start, esp_sleep_enable_timer_wakeup};
use std::time::Duration;

pub fn light_sleep(duration: Duration) {
    unsafe {
        esp_sleep_enable_timer_wakeup(duration.subsec_micros() as u64);
        println!("Entering light sleep for {}ms...", duration.as_millis());
        esp_light_sleep_start();
    }
}

pub fn deep_sleep(duration: Duration) {
    unsafe { esp_deep_sleep(duration.as_micros() as u64) }
}
