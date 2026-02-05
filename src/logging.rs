#[macro_export]
macro_rules! log {
    ($fmt:expr) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[97m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                $fmt
            );
        }
    };

    ($fmt:expr, $($arg:tt)*) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[97m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                format_args!($fmt, $($arg)*)
            );
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[32mDEBUG\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[32m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                $fmt
            );
        }
    };

    ($fmt:expr, $($arg:tt)*) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[32mDEBUG\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[32m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                format_args!($fmt, $($arg)*)
            );
        }
    };
}

#[macro_export]
macro_rules! info {
    ($fmt:expr) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[34mINFO\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[34m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                $fmt
            );
        }
    };

    ($fmt:expr, $($arg:tt)*) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[34mINFO\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[34m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                format_args!($fmt, $($arg)*)
            );
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[33mWARN\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[33m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                $fmt
            );
        }
    };

    ($fmt:expr, $($arg:tt)*) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            println!(
                "[\x1b[97m{}\x1b[0m] [\x1b[33mWARN\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[33m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                format_args!($fmt, $($arg)*)
            );
        }
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:expr) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            eprintln!(
                "[\x1b[97m{}\x1b[0m] [\x1b[31mERROR\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[31m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                $fmt
            );
        }
    };

    ($fmt:expr, $($arg:tt)*) => {
        {
            let ts = ::chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            eprintln!(
                "[\x1b[97m{}\x1b[0m] [\x1b[31mERROR\x1b[0m] [\x1b[1;32m{}\x1b[0m:\x1b[35m{}\x1b[0m] [\x1b[36m{:?}\x1b[0m] \x1b[31m{}\x1b[0m",
                ts,
                file!(),
                line!(),
                ::std::thread::current().id(),
                format_args!($fmt, $($arg)*)
            );
        }
    };
}
