extern crate simple_munin_plugin;

mod muninbme280 {
  use simple_munin_plugin::*;
  use linux_embedded_hal::{Delay, I2cdev};
  use bme280::BME280;

  pub struct Bme280Plugin;

  impl Bme280Plugin {
    pub fn new() -> Bme280Plugin {
      Bme280Plugin
    }
  }

  impl MuninNodePlugin for Bme280Plugin {
    fn config(&self) {
      println!(r#"multigraph bme280_temp
graph_title BME280 Environment Temperture
graph_category environment
graph_label Temperature
temp.label Temperature
temp.type GAUGE
temp.max  50
temp.min -30

multigraph bme280_humidity
graph_title BME280 Environment Humidity
graph_category environment
graph_label Humidity
hum.label Humidity
hum.type GAUGE
hum.max 100
hum.min 0
hum.critical 95
hum.warning  90

multigraph bme280_pressure
graph_title BME280 Environment Atmospheric pressure
graph_category environment
graph_label pressure
pressure.label pressure
pressure.type GAUGE
pressure.max 1150
pressure.min  850
"#);

    }
    
    fn run(&self) {
      // using Linux I2C Bus #1 in this example
      let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
      // initialize the BME280 using the primary I2C address 0x76
      let mut bme280 = BME280::new_primary(i2c_bus, Delay);
      // initialize the sensor
      bme280.init().unwrap();
      // measure temperature, pressure, and humidity
      let measurements = bme280.measure().unwrap();
  
      println!("multigraph bme280_temp");
      println!("temp.value {}", measurements.temperature);
      println!("multigraph bme280_humidity");
      println!("hum.value {}", measurements.humidity);
      println!("multigraph bme280_pressure");
      println!("pressure.value {}", measurements.pressure / 100.0);

    }
  }
}

use simple_munin_plugin::*;

fn main() {
  let plugin = muninbme280::Bme280Plugin::new();
  std::process::exit(plugin.start());
}
