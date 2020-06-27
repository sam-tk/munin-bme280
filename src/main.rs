extern crate simple_munin_plugin;

mod muninbme280 {
  use simple_munin_plugin::MuninNodePlugin;
  use linux_embedded_hal::{Delay, I2cdev};
  use bme280::BME280;
  // use average::{MeanWithError, Estimate};
  use std::time::Duration;
  use std::thread;

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
graph_vlabel C
temp.label Temperature
temp.type GAUGE
temp.max  50
temp.min -30

multigraph bme280_humidity
graph_title BME280 Environment Humidity
graph_category environment
graph_label Humidity
graph_vlabel (%)
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
graph_vlabel hPa
graph_args -X 0
graph_scale no
pressure.label pressure
pressure.type GAUGE
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
      /*
      let mut temp = MeanWithError::new();
      let mut hum = MeanWithError::new();
      let mut pressure = MeanWithError::new();
      

      //let duration = Duration::from_millis(1500);
      let duration = Duration::from_secs(2);
      for _i in 0..5 {
        let measurements = bme280.measure().unwrap();
        temp.add(measurements.temperature.into());
        hum.add(measurements.humidity.into());
        pressure.add(measurements.pressure.into());
        thread::sleep(duration);
      } 
      */

      let mut measuments = bme280.measure().unwrap();
      let humidity_base = 80.0_f32 ; 
      let temperature_base = 20.0_f32 ;
      for _i in 0..5 {
        if (measuments.humidity - humidity_base).abs() < 0.5 &&
           (measuments.temperature - temperature_base).abs() < 0.1 {
             let duration = Duration::from_secs(4);
             thread::sleep(duration);
             measuments = bme280.measure().unwrap();
           } 
        else { break; }
      }
       

      println!("multigraph bme280_temp");
      println!("temp.value {}", measuments.temperature);
      println!("multigraph bme280_humidity");
      println!("hum.value {}", measuments.humidity);
      println!("multigraph bme280_pressure");
      println!("pressure.value {}", measuments.pressure / 100.0);

    }
  }
}

use simple_munin_plugin::MuninNodePlugin;

fn main() {
  let plugin = muninbme280::Bme280Plugin::new();
  std::process::exit(plugin.start());
}
