use dotenv::dotenv;

pub struct AppEnv {
    pub ports: Vec<u16>,
}

impl AppEnv {
    pub fn load() -> Self {
        dotenv().ok();
        let ports: Vec<u16> = dotenv::var("PORTS")
            .unwrap_or_else(|_| "8888".to_string())
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        AppEnv { ports }
    }
}