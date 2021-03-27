use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigurationFile {
    pub connection: String,
    pub server_address: String,
}

impl ConfigurationFile {
    pub fn get_configuration_file() -> Self {
        let caminho_executavel_full = env::current_exe().unwrap();
        let caminho = caminho_executavel_full.parent().unwrap().to_str().unwrap();
        let nome_executavel = caminho_executavel_full
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let caminho_configuracao = format!("{}/{}.json", caminho, nome_executavel);
        let arquivo = fs::read_to_string(&caminho_configuracao)
            .expect("Não foi possível ler o arquivo de configuração");

        serde_json::from_str(&arquivo).expect("Não foi possível carregar dados do arquivo")
    }
}
