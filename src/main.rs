use clap::{Parser, ValueEnum};
use rppal::gpio::Gpio;
use std::error::Error;

const SNAP_POWER_PIN: u8 = 20;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Power state of the SNAP
    #[clap(value_enum)]
    state: State,
}

#[derive(Debug, Clone, ValueEnum)]
enum State {
    On,
    Off,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Retrieve the GPIO pin and configure it as an output.
    let mut pin = Gpio::new()?.get(SNAP_POWER_PIN)?.into_output();

    match args.state {
        State::On => pin.set_high(),
        State::Off => pin.set_low(),
    }

    Ok(())
}
