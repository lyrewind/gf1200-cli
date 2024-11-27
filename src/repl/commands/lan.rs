use super::Command;

pub fn status() -> Command {
    Command {
        name: "lan-status",
        description: "lista o status da rede local (LAN).",
        args: None,
        run: |state, _| match state.api.lan_status() {
            Some(status) => {
                println!("<LAN>");
                println!("  [endereço IPv4] {}", status.ip4);
                println!("  [máscara de subrede] {}", status.netmask);
                println!("  [endereço MAC] {}", status.mac);
                for (i, addr) in status.ip6_list.iter().enumerate() {
                    println!("  [endereço IPv6 #{}] {addr}", i + 1);
                }
            }
            None => eprintln!("falha ao requisitar status da rede local."),
        },
    }
}
