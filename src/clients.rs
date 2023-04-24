use ini::Ini;
use std::{fs, path::Path};

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub id: String,
    pub address: String,
    pub port: String,
}

impl ClientConfig {
    fn new(id: String, address: String, port: String) -> Self {
        Self { id, address, port }
    }
}

pub struct ClientesConfigManager {
    config: Vec<ClientConfig>,
}

impl ClientesConfigManager {
    pub fn new() -> Self {
        Self { config: Vec::new() }
    }

    pub fn get_clientes_by_ini(&mut self) -> &Vec<ClientConfig> {
        if self.config.is_empty() {
            // Get path
            let mut path = std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf();
            path.push("Clients");

            // Max 10 clients
            for i in 1..=10 {
                let client_id = if i == 10 {
                    format!("0{}-client.properties", i)
                } else {
                    format!("00{}-client.properties", i)
                };

                let file_path = path.join(format!("{}.txt", client_id));
                let file_path_str = match file_path.to_str() {
                    Some(s) => s.trim_start_matches("\\\\?\\"),
                    None => continue,
                };
                if Path::new(file_path_str).exists() {
                    let id = read_ini_value(file_path_str, "Properties", "id").unwrap();
                    let address = read_ini_value(file_path_str, "Properties", "address").unwrap();
                    let port = read_ini_value(file_path_str, "Properties", "port").unwrap();
                    let config = ClientConfig::new(id, address, port);
                    self.config.push(config);
                }
            }
        }

        &self.config
    }
}

fn read_ini_value(file_path: &str, section: &str, key: &str) -> Option<String> {
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return None,
    };
    let ini = match Ini::load_from_str(&contents) {
        Ok(ini) => ini,
        Err(e) => return Err(e.msg.as_str().to_owned()).ok(),
    };
    let section = ini.section(Some(section));
    section.and_then(|s| s.get(key).map(|v| v.to_owned()))
}
