

trait Config {
    fn get_value(&self, key: &str) -> Option<&str>;
}

struct AppConfig {
    settings: std::collections::HashMap<String, String>,
}

impl Config for AppConfig {
    fn get_value(&self, key: &str) -> Option<&str> {
        self.settings.get(key).map(|v| v.as_str())
    }
}

fn use_config<T: Config>(config: &T) {
    if let Some(value) = config.get_value("key") {
        println!("Value for key : {}", value);
    } else {
        println!("Key not found");
    }
}

fn main() {
    println!("Hello, config design explanation !");
    let app_config = AppConfig {
        settings: {
            let mut map = std::collections::HashMap::new();
            map.insert("key".to_string(), "apple".to_string());
            map
        },
    };

    use_config(&app_config);
}
