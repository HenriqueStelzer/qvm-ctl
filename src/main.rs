mod cli;
mod commands;
mod config;
mod display;
mod error;
mod ovmf;
mod port;
mod process;
mod qemu;
mod tpm;
mod vm;

use clap::Parser;
use cli::{Cli, Commands};
use config::EnvDefaults;
use display::{die, sep};

fn print_custom_help() {
    sep();
    if display::is_color_enabled() {
        println!("\x1b[1mqvm\x1b[0m 1.2.0 — QEMU/KVM VM manager");
    } else {
        println!("qvm 1.2.0 — QEMU/KVM VM manager");
    }
    sep();
    println!("  qvm create  <name> <windows.iso> [virtio.iso]");
    println!("                              create a new VM");
    println!("  qvm launch  <name> [--no-iso] [--headless]");
    println!("                              boot VM (optionally without ISO or SPICE viewer)");
    println!("  qvm app     <name> <app_path> [args...]");
    println!("                              launch application via FreeRDP RemoteApp");
    println!("  qvm stop    <name>      gracefully stop VM");
    println!("  qvm list               show VMs and status");
    println!("  qvm disable <name>      stop and delete VM");
    println!("  qvm version             print version");
    sep();
    println!("  Environment overrides:");
    println!("    QVM_DIR       VM storage directory      (default: ~/vms)");
    println!("    QVM_RAM       RAM in MB                 (default: 8192)");
    println!("    QVM_CPUS      vCPU count                (default: 4)");
    println!("    QVM_DISK      Disk size                 (default: 40G)");
    println!("    QVM_RDP_USER  RemoteApp RDP username    (default: $USER or vm.conf)");
    println!("    QVM_RDP_PASS  RemoteApp RDP password    (default: prompt or vm.conf)");
    sep();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "help" {
        print_custom_help();
        return;
    }

    let cli = Cli::parse();
    let env_defaults = EnvDefaults::load();
    let base = &env_defaults.vm_dir;

    let result = match cli.command {
        Some(Commands::Create {
            name,
            iso,
            virtio_iso,
        }) => commands::create::run(base, name, iso, virtio_iso),
        Some(Commands::Launch {
            name,
            no_iso,
            headless,
        }) => commands::launch::run(base, name, no_iso, headless),
        Some(Commands::App {
            name,
            app_path,
            args,
        }) => commands::app::run(base, name, app_path, args),
        Some(Commands::Stop { name }) => commands::stop::run(base, name),
        Some(Commands::List) => commands::list::run(base),
        Some(Commands::Disable { name }) => commands::disable::run(base, name),
        Some(Commands::Version) => {
            println!("qvm 1.2.0");
            Ok(())
        }
        None => {
            print_custom_help();
            Ok(())
        }
    };

    if let Err(e) = result {
        die(e);
    }
}
