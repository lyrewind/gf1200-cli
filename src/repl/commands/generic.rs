use super::{Arg, ArgType, Command};

pub fn devices() -> Command {
    Command {
        name: "devices",
        description: "Lists connected devices.",
        args: None,
        run: |state, _| match state.api.connected_devices() {
            Some(devices) => {
                if devices.is_empty() {
                    println!("[#] found no device connected.");
                    return;
                }

                println!(
                    "[#] found {} {} connected:",
                    devices.len(),
                    if devices.len() > 1 {
                        "devices"
                    } else {
                        "device"
                    }
                );
                for dev in devices {
                    println!("{dev}")
                }
            }
            None => {
                println!("failed to fetch connected devices.");
            }
        },
    }
}

pub fn device() -> Command {
    Command {
        name: "device",
        description: "Lists device information.",
        args: Some(Vec::from([Arg {
            name: "mac",
            typing: ArgType::String,
        }])),
        run: |state, args| {
            let Some(mac) = args.get(0) else {
                unreachable!()
            };

            match state.api.device(mac) {
                Some(dev) => println!("{dev}"),
                None => {
                    println!("no device with mac address '{mac}' found.")
                }
            }
        },
    }
}
