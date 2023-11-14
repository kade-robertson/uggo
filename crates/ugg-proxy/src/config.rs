use std::env::var;

pub struct Config {
    pub bind_address: String,
    pub bind_port: u16,
    pub log_level: String,
    pub cache_size: u64,
}

pub fn get_config() -> Config {
    Config {
        bind_address: var("BIND_ADDRESS").map_or_else(|_| "127.0.0.1".to_string(), |s| s),
        bind_port: var("BIND_PORT").map_or(3000, |s| s.parse::<u16>().map_or(3000, |p| p)),
        log_level: var("LOG_LEVEL").map_or_else(|_| "info".to_string(), |s| s),
        cache_size: var("CACHE_SIZE").map_or(1000, |s| s.parse::<u64>().map_or(1000, |p| p)),
    }
}
