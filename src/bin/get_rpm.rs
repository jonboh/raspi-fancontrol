use std::time::Instant;

use clap::Parser;

use anyhow::Result;
use rp_fancontrol::Tacho;
use rp_fancontrol::TachoHardwareConfig;
use rppal::gpio::Gpio;
use rppal::gpio::Trigger;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Delay in milliseconds to evaluate rpm
    #[arg(short, long, default_value_t = 1000)]
    delay: u64,

    /// GPIO pin to which the tachometer is connected
    #[arg(short, long, default_value_t = 23)]
    gpio_tacho: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let delay = std::time::Duration::from_millis(args.delay);

    let tacho_config = TachoHardwareConfig::noctua_tacho(args.gpio_tacho);

    let mut tacho_pin = Gpio::new()?.get(args.gpio_tacho)?.into_input_pullup();
    let mut tacho_counter = Tacho::new(tacho_config);
    tacho_pin.set_interrupt(Trigger::FallingEdge, None)?;

    let start = Instant::now();
    while start.elapsed() < delay {
        if (tacho_pin.poll_interrupt(false, Some(std::time::Duration::from_millis(10)))?).is_some()
        {
            tacho_counter.handle_interrupt();
        };
    }
    let rpm = tacho_counter.get_rpm()?;
    println!("{rpm}");
    Ok(())
}
