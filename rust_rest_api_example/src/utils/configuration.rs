use std::env;
use std::fs;

use rust_rest_api_example_data::serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct ConfigurationFile {
    pub connection: String,
    pub server_address: String,
}

impl ConfigurationFile {
    pub fn get_configuration_file() -> Self {
        let caminho_executavel_full = env::current_exe().unwrap();
        let caminho = caminho_executavel_full.parent().unwrap().to_str().unwrap();
        let caminho_configuracao = format!("{}/config.json", caminho);
        let arquivo = fs::read_to_string(&caminho_configuracao)
            .expect("Não foi possível ler o arquivo de configuração");

        serde_json::from_str(&arquivo).expect("Não foi possível carregar dados do arquivo")
    }
}
