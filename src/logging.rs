pub mod colours {
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const RED: &str = "\x1b[31m";
    pub const RESET: &str = "\x1b[0m";
}

#[macro_export]
macro_rules! green {
    ($fmt:literal) => {
        format!(
            "{}{}{}",
            $crate::logging::colours::GREEN,
            $fmt,
            $crate::logging::colours::RESET
        )
    };
}

#[macro_export]
macro_rules! log_base {
    ($level:expr, $fmt:expr) => {
        let ts = ::chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();
        let level = $level;
        let fmt = format!($fmt);
        let file = file!();
        let line = line!();
        let thread = std::thread::current().id();
        println!("[\x1b[97m{ts}\x1b[0m] [{level}\x1b[0m] [{file}:{line}] [{thread:?}] {fmt}");
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt:literal) => {{
        let fmt = format!($fmt);
        $crate::log_base!($crate::green!("DEBUG"), $crate::green!(fmt));
    }};
}

#[macro_export]
macro_rules! info {
    ($fmt:literal) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[34mINFO\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[34m{}\x1b[0m",
                ts,
                ::std::thread::current().id(),
                $fmt
            );
        }
    };

    ($fmt:expr, $($arg:tt)*) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[34mINFO\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[34m{}\x1b[0m",
                ts,
                ::std::thread::current().id(),
                format_args!($fmt, $($arg)*)
            );
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt:literal) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[33mWARN\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[33m{}\x1b[0m",
                ts,
                ::std::thread::current().id(),
                $fmt
            );
        }
    };

    ($fmt:expr, $($arg:tt)*) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[33mWARN\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[33m{}\x1b[0m",
                ts,
                ::std::thread::current().id(),
                format_args!($fmt, $($arg)*)
            );
        }
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:literal) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            eprintln!(
                "[\x1b[97m{}\x1b[0m] [\x1b[31mERROR\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[31m{}\x1b[0m",
                ts,
                ::std::thread::current().id(),
                $fmt
            );
        }
    };

    ($fmt:literal, $($arg:tt)*) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            eprintln!(
                "[\x1b[97m{}\x1b[0m] [\x1b[31mERROR\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[31m{}\x1b[0m",
                ts,
                ::std::thread::current().id(),
                format_args!($fmt, $($arg)*)
            );
        }
    };
}
