use esp_idf_sys::{
    esp_err_t, temperature_sensor_config_t, temperature_sensor_disable, temperature_sensor_enable,
    temperature_sensor_get_celsius, temperature_sensor_handle_t, temperature_sensor_install,
    temperature_sensor_uninstall, ESP_OK,
};

pub struct TemperatureSensor {
    handle: temperature_sensor_handle_t,
}

impl TemperatureSensor {
    pub fn new() -> Result<Self, esp_err_t> {
        let mut handle: temperature_sensor_handle_t = std::ptr::null_mut();
        let config = temperature_sensor_config_t {
            range_min: -10,
            range_max: 80,
            clk_src: esp_idf_sys::soc_periph_temperature_sensor_clk_src_t_TEMPERATURE_SENSOR_CLK_SRC_DEFAULT,
        };

        let ret = unsafe { temperature_sensor_install(&config, &mut handle) };
        if ret != ESP_OK {
            return Err(ret);
        }

        let ret = unsafe { temperature_sensor_enable(handle) };
        if ret != ESP_OK {
            return Err(ret);
        }

        Ok(Self { handle })
    }

    pub fn read_temperature(&self) -> Result<f32, esp_err_t> {
        let mut temp_celsius: f32 = 0.0;
        let ret = unsafe { temperature_sensor_get_celsius(self.handle, &mut temp_celsius) };
        if ret == ESP_OK {
            Ok(temp_celsius)
        } else {
            Err(ret)
        }
    }
}

impl Drop for TemperatureSensor {
    fn drop(&mut self) {
        unsafe {
            temperature_sensor_disable(self.handle);
            temperature_sensor_uninstall(self.handle);
        }
    }
}
