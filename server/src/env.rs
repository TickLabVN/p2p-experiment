use dotenv::dotenv;

pub struct AppEnv {
    pub port: u16,
}

impl AppEnv {
    pub fn load() -> Self {
        dotenv().ok();
        let port = dotenv::var("PORT")
            .unwrap_or_else(|_| "8888".to_string())
            .parse()
            .unwrap_or(8888);
        
        AppEnv { port }
    }
}