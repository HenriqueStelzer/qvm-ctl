use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "qvm", version = "1.2.0", about = "QEMU/KVM VM manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new VM
    Create {
        /// VM name
        name: String,
        /// Windows / installer ISO path
        iso: String,
        /// Optional VirtIO driver ISO path
        virtio_iso: Option<String>,
    },
    /// Boot VM (optionally without ISO or SPICE viewer)
    Launch {
        /// VM name
        name: String,
        /// Boot without installer ISO attached
        #[arg(long)]
        no_iso: bool,
        /// Run headless without opening SPICE viewer
        #[arg(long)]
        headless: bool,
    },
    /// Launch application via FreeRDP RemoteApp
    App {
        /// VM name
        name: String,
        /// Application path in guest
        app_path: String,
        /// Arguments for the application
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Gracefully stop VM
    Stop {
        /// VM name
        name: String,
    },
    /// Show VMs and status
    List,
    /// Stop and delete VM
    Disable {
        /// VM name
        name: String,
    },
    /// Print version
    Version,
}
