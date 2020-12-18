use anyhow::Result;
use argh::FromArgs;
use ddc_hi::{Ddc, Display};
use mccs_db::ValueType;
use std::collections::HashMap;

const INPUT_SELECT: u8 = 0x60;

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

struct Monitor {
    name: String,
    inputs: Vec<String>,
}

fn switch(monitor: String, input: String, codes: HashMap<&str, u16>) -> Result<()> {
    for mut display in Display::enumerate() {
        let _ = display.update_capabilities();

        if let Some(name) = display.info.model_name {
            if name == monitor {
                let code = codes[input.as_str()];
                display
                    .handle
                    .set_vcp_feature(INPUT_SELECT, code)
                    .expect("switched!");
            }
        }
    }
    Ok(())
}

fn get_monitors() -> Result<Vec<Monitor>> {
    let mut monitors = vec![];

    for mut display in Display::enumerate() {
        let mut monitor: Monitor = Monitor {
            name: "".to_string(),
            inputs: vec![],
        };

        let _ = display.update_capabilities();
        if let Some(name) = display.info.model_name {
            monitor.name = name;
        }

        if let Some(d) = display.info.mccs_database.get(INPUT_SELECT) {
            if let ValueType::NonContinuous {
                interpretation: _,
                values,
            } = &d.ty
            {
                let mut inputs: Vec<String> = vec![];
                for input_types in values.values() {
                    match input_types {
                        Some(input) => inputs.push(input.to_string()),
                        None => (),
                    };
                }
                monitor.inputs = inputs;
            }
            monitors.push(monitor);
        }
    }
    Ok(monitors)
}

fn list(monitors: Vec<Monitor>) -> Result<()> {
    for monitor in monitors.iter() {
        println!(
            "Found the following inputs available on {:?}:",
            monitor.name
        );

        for input in monitor.inputs.iter() {
            println!("{:?}", input);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut inputs_codes: HashMap<&str, u16> = HashMap::new();
    inputs_codes.insert("DVI 1", 0x03);
    inputs_codes.insert("DVI 2", 0x04);
    inputs_codes.insert("DisplayPort 1", 0x0f);
    inputs_codes.insert("DisplayPort 2", 0x10);
    inputs_codes.insert("HDMI 1", 0x11);
    inputs_codes.insert("HDMI 2", 0x12);

    let args: Args = argh::from_env();

    let monitors = get_monitors()?;

    match args.commands {
        Commands::Switch(x) => switch(x.monitor, x.input, inputs_codes)?,
        Commands::List(_) => list(monitors)?,
    }

    Ok(())
}
