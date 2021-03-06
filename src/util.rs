use crate::config;

macro_rules! exit {
    ( $description:expr ) => {{
        eprintln!("{}", $description);
        std::process::exit(1)
    }};
    ( $description:expr, $error:expr ) => {{
        eprintln!("{}: {}", $description, $error);
        std::process::exit(1)
    }};
}

pub fn get_bridge() -> huelib::Bridge {
    match config::get() {
        Ok(v) => huelib::Bridge::new(v.bridge_ip, &v.bridge_username),
        Err(e) => exit!("Failed to get configuration environment variables", e),
    }
}
