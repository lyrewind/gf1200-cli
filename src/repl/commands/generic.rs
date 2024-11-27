use super::{Arg, ArgType, Command};

pub fn devices() -> Command {
    Command {
        name: "devices",
        description: "lista dispositivos conectados.",
        args: None,
        run: |state, _| match state.api.connected_devices() {
            Some(devices) => {
                if devices.is_empty() {
                    println!("[#] nenhum dispositivo conectado.");
                    return;
                }

                println!(
                    "[#] {} {} encontrados:",
                    devices.len(),
                    if devices.len() > 1 {
                        "dispositivos"
                    } else {
                        "dispositivo"
                    }
                );
                for dev in devices {
                    println!("{dev}")
                }
            }
            None => {
                println!("falha ao ler dispositivos.");
            }
        },
    }
}

pub fn device() -> Command {
    Command {
        name: "device",
        description: "lista informações de um dispositivo",
        args: Some(Vec::from([Arg {
            name: "mac",
            typing: ArgType::String,
        }])),
        run: |state, args| {
            let Some(mac) = args.get(0) else {
                unreachable!()
            };

            match state.api.connected_device(mac) {
                Some(dev) => println!("{dev}"),
                None => {
                    println!("nenhum dispositivo com endereço '{mac}' encontrado.")
                }
            }
        },
    }
}

pub fn restart() -> Command {
    Command {
        name: "restart",
        description: "reinicia o dispositivo.",
        args: None,
        run: |state, _| match state.api.restart() {
            Ok(_) => {
                println!("reiniciando...");
                std::process::exit(0)
            }
            Err(_) => {
                // TODO: token refreshing.
                println!("falha ao reiniciar dispositivo.");
            }
        },
    }
}
