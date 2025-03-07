use anyhow::{anyhow, Result};
use clap::Parser;
use rp_fancontrol::{FanHardwareConfig, PWM};

use rppal::pwm::{Channel, Pwm};
/// TODO:
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pwm: f64,

    #[arg(short, long, default_value_t = 2)]
    pwm_channel: u8,
}

// fn temp2duty(temp: Celsius) -> f64 {}

fn main() -> Result<()> {
    let args = Args::parse();

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

    // NOTE: see: https://www.noctua.at/pub/media/wysiwyg/Noctua_PWM_specifications_white_paper.pdf
    let mut pwm = Pwm::with_frequency(fan.pwm_channel, fan.freq, 0.0, fan.polarity, true)
        .expect("failed to set pwm");
    pwm.set_reset_on_drop(false);

    let pwm_value = PWM::new(args.pwm)?;

    pwm.set_frequency(fan.freq, pwm_value.0)?;
    Ok(())
}
