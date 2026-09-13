use std::fmt;

pub fn is_color_enabled() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    supports_color::on(supports_color::Stream::Stdout).is_some()
}

pub fn info(msg: impl fmt::Display) {
    if is_color_enabled() {
        println!("\x1b[0;36m→\x1b[0m {}", msg);
    } else {
        println!("→ {}", msg);
    }
}

pub fn ok(msg: impl fmt::Display) {
    if is_color_enabled() {
        println!("\x1b[0;32m✓\x1b[0m {}", msg);
    } else {
        println!("✓ {}", msg);
    }
}

pub fn warn(msg: impl fmt::Display) {
    if is_color_enabled() {
        println!("\x1b[1;33m!\x1b[0m {}", msg);
    } else {
        println!("! {}", msg);
    }
}

pub fn die(msg: impl fmt::Display) -> ! {
    if std::env::var_os("NO_COLOR").is_none()
        && supports_color::on(supports_color::Stream::Stderr).is_some()
    {
        eprintln!("\x1b[0;31m✗\x1b[0m {}", msg);
    } else {
        eprintln!("✗ {}", msg);
    }
    std::process::exit(1);
}

pub fn sep() {
    if is_color_enabled() {
        println!("\x1b[1m────────────────────────────────────────\x1b[0m");
    } else {
        println!("────────────────────────────────────────");
    }
}
