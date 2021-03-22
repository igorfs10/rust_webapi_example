use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

const SETTINGS_FILE: &str = "postgre_api.toml";

#[derive(Serialize, Deserialize)]
pub struct ConfigurationFile {
    pub connection: String,
    pub server_address: String,
}

impl ConfigurationFile {
    pub fn get_configuration_file() -> Self {
        let caminho_executavel_full = env::current_exe().unwrap();
        let caminho = caminho_executavel_full.parent().unwrap().to_str().unwrap();
        let caminho_configuracao = format!("{}/{}", caminho, SETTINGS_FILE);
        let arquivo = fs::read_to_string(&caminho_configuracao)
            .expect("Não foi possível ler o arquivo de configuração");

        toml::from_str(&arquivo).expect("Não foi possível carregar dados do arquivo")
    }
}
