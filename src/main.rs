use anyhow::{anyhow, Result};
use clap::Parser;
use rp_fancontrol::Celsius;
use rp_fancontrol::FanHardwareConfig;
use rp_fancontrol::PWM;
use rppal::pwm::Channel;
use std::path::PathBuf;
use std::time::Duration;

use rppal::pwm::Pwm;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File from which to read the temperature of the cpu. Set to the default sysfs file used in
    /// raspberrys
    #[arg(long, default_value = "/sys/class/thermal/thermal_zone0/temp")]
    temp_file: PathBuf,

    /// Interval in miliseconds to reevaluate fan power under current temperature
    #[arg(short, long, default_value_t = 5000)]
    interval: u64,

    /// Temp value in the temp2pwm curve.
    /// There must be as many as --pwm values.
    /// Use it more than once to define several points in the curve
    #[arg(short, long)]
    temp: Vec<f64>,

    /// PWM value in the temp2pwm curve.
    /// There must be as many as --temp values.
    /// Use it more than once to define several points in the curve
    #[arg(short, long)]
    pwm: Vec<f64>,

    /// PWM Channel to which the fan is connected.
    /// By default set to PWM2==GPIO18.
    /// PWM0 = GPIO12
    /// PWM1 = GPIO13
    /// PWM2 = GPIO18
    /// PWM3 = GPIO19
    #[arg(long, default_value_t = 2)]
    pwm_channel: u8,
}

struct Config {
    temp2pwm: Vec<(Celsius, PWM)>,
}

impl Config {
    fn new(temps: Vec<Celsius>, pwms: Vec<PWM>) -> Result<Self> {
        if temps.len() != pwms.len() {
            return Err(anyhow!(
                "Temperature and PWM vectors must be the same length"
            ));
        }
        if temps.is_empty() {
            return Err(anyhow!("At least one Temp-PWM point must be provided"));
        }
        let curve: Vec<(Celsius, PWM)> = temps.into_iter().zip(pwms).collect();
        for values in curve.windows(2) {
            let (x1, y1) = &values[0];
            let (x2, y2) = &values[1];
            if x1 >= x2 {
                return Err(anyhow!(
                    "Temperature x points must be increasing {x1} !< {x2}"
                ));
            }
            if y1 > y2 {
                return Err(anyhow!("PWM y points must be non-decreasing {y1} !<= {y2}"));
            }
        }
        Ok(Self { temp2pwm: curve })
    }

    /// Get the new PWM value by interpolating `temp` into `temp2pwm`
    fn evaluate(&self, temp: Celsius) -> PWM {
        match self
            .temp2pwm
            .iter()
            .enumerate()
            .find(|(_, (t, _))| *t >= temp)
        {
            Some((i, _)) => {
                if i == self.temp2pwm.len() {
                    self.temp2pwm
                        .last()
                        .expect("temp2pwm is non-empty by construction")
                        .1
                } else {
                    let t0 = self.temp2pwm[i].0;
                    let t1 = self.temp2pwm[i + 1].0;
                    let p0 = self.temp2pwm[i].1;
                    let p1 = self.temp2pwm[i + 1].1;
                    PWM::new_saturate(p0.0 + (p1.0 - p0.0) * ((temp.0 - t0.0) / (t1.0 - t0.0)))
                }
            }
            None => {
                self.temp2pwm
                    .last()
                    .expect("temp2pwm is non-empty by construction")
                    .1
            }
        }
    }
}

fn read_temperature(file: &PathBuf) -> Result<Celsius> {
    Ok(Celsius(
        std::fs::read_to_string(file)?.trim().parse::<u64>()? as f64 / 1000f64,
    ))
}

fn main() -> Result<()> {
    let args = Args::parse();

    let temps: Vec<Celsius> = args.temp.into_iter().map(Celsius).collect();
    let pwms: Vec<PWM> = args
        .pwm
        .into_iter()
        .map(PWM::new)
        .collect::<Result<Vec<PWM>, _>>()?;
    let config = Config::new(temps, pwms)?;

    let pwm_channel = match args.pwm_channel {
        0 => Channel::Pwm0,
        1 => Channel::Pwm1,
        2 => Channel::Pwm2,
        3 => Channel::Pwm3,
        p => {
            return Err(anyhow!(
                "Invalid PWM channel {p}, must be one of [0, 1, 2, 3]"
            ))
        }
    };
    let fan = FanHardwareConfig::noctua_fan(pwm_channel);

    let pwm = Pwm::with_frequency(fan.pwm_channel, fan.freq, 0.0, fan.polarity, true)
        .expect("failed to set pwm");
    loop {
        std::thread::sleep(Duration::from_millis(args.interval));
        let temp = read_temperature(&args.temp_file)?;
        let new_pwm = config.evaluate(temp);
        pwm.set_frequency(fan.freq, new_pwm.0 as f64)?;
        println!("Temp={temp} setting PWM Duty to: {new_pwm}");
    }
}
