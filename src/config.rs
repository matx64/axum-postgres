use std::{env, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| Config::load_from_env())
}

pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn load_from_env() -> Config {
        dotenv::dotenv().ok();
        Config {
            db_url: get_env("DB_URL"),
        }
    }
}

fn get_env(name: &'static str) -> String {
    env::var(name).expect(&format!("❌ Couldn't load {} environment variable", name))
}
