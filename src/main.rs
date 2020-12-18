use anyhow::{anyhow, Result};
use argh::FromArgs;
use ddc_hi::Display;
use mccs_db::ValueType;

const INPUT_SELECT: u8 = 0x60;

// symbolic_input_source! {
//     DisplayPort1: 0x0f
//     DisplayPort2: 0x10
//     Hdmi1: 0x11
//     Hdmi2: 0x12
// }

#[derive(FromArgs, Debug)]
#[argh(description = "monitor-switch: switch monitors via DDC/CI")]
struct Args {
    #[argh(subcommand)]
    commands: Commands,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Commands {
    List(List),
    Switch(Switch),
}

#[derive(FromArgs, Debug)]
#[argh(subcommand, description = "list available monitors", name = "list")]
pub struct List {}

#[derive(FromArgs, Debug)]
#[argh(subcommand, description = "switch input on monitor.", name = "switch")]
pub struct Switch {
    #[argh(option, short = 'i', description = "input source to change too.")]
    input: String,
    #[argh(option, short = 'm', description = "the monitor to change inputs on.")]
    monitor: String,
}

fn switch(monitor: String, input: String) -> Result<()> {
    // if display.info.model_name == Some(String::from("S2719DGF")) {
    //   display.update_capabilities().unwrap();
    //   let feature = display.info.mccs_database.get(INPUT_SELECT);
    //   // display.handle.set_vcp_feature(INPUT_SELECT, 0x12).expect("switched to HDMI2");
    // }
    Ok(())
}

fn list() -> Result<()> {
    println!("Listing displays...");
    for mut display in Display::enumerate() {
        println!("---Display---");
        if let Err(err) = display.update_capabilities() {
            println!("Error getting display capabilities: {}", err);
        }
        if let Some(d) = display.info.model_name {
            println!("{} has the following inputs:", d);
        }

        if let Some(d) = display.info.mccs_database.get(INPUT_SELECT) {
            if let ValueType::NonContinuous {
                interpretation: _,
                values,
            } = &d.ty
            {
                for input_types in values.values() {
                    match input_types {
                        Some(input) => println!("{:?}", input),
                        None => (),
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    match args.commands {
        Commands::Switch(x) => switch(x.monitor, x.input)?,
        Commands::List(_) => list()?,
    }

    Ok(())
}
