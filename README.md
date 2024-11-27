# gf1200-cli (WIP)
Uma interface de linha de comando pro roteador
[GF-1200](https://www.intelbras.com/pt-br/roteador-wi-fi-5-ac-1200-com-porta-internet-giga-e-lan-fast-wi-force-gf-1200) da Intelbras,
porque ele só ter uma interface web me irritou o suficiente. Feito pra Linux x86_64 e ARM64.

## Instalação
Testado no Linux x86_64 (NixOS).
Esse projeto usa [Nix](https://nixos.org/), e tem dois modos de instação.

### Sem Nix
Os requisitos de build são [Rust](http://rust-lang.org/), pkg-config e OpenSSL.
A versão do Rust é definida no próprio projeto (`rust-toolchain.toml`), sem necessidade de instalar com `rustup`.

1. Clone o projeto com `git clone https://github.com/lyrewind/gf1200-cli.git`.
2. Na pasta do projeto, `cargo install --path .` pra instalar no perfil atual.
3. Et voilà.

### Com Nix
Esse projeto é um flake, podendo ser consumido de várias formas (rodado sem instalar com `nix run`,
instalado impuramente com `nix install`, etc.).
O pacote é exposto como `packages.${system}.default`.

Por exemplo, pra instalar declarativamente num NixOS (x86_64 ou ARM64):
```nix
# flake.nix
{
    inputs.gf1200-cli.url = "github:lyrewind/gf1200-cli";
    # ...
}
```

```nix
# configuration.nix (ou outro arquivo)
{ inputs, system, ... }: {
    environment.systemPackages = [
        inputs.gf1200-cli.packages.${system}.default
    ];
    # ...
}
```

## Uso
`gf1200-cli` depois de instalado pra rodar. 
A interface funciona como um [REPL](https://pt.wikipedia.org/wiki/REPL) de comandos. O comando `help`
lista todos os comandos disponível.

Pra sair, `exit` ou Ctrl-C (ou Ctrl-D).

## Progresso
- [x] Funcionalidades básicas (status, restart)
- [ ] Configuração (de LAN, WAN e sistema)
- [ ] Auto-completion
- [ ] Chamada única (pra uso com pipes `|`)

## Licença
Unlicense (i.e. domínio público).
