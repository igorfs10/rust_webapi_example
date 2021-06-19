use std::env;
use std::fs;

pub fn get_config(config_name: &str) -> String {
    let full_exe_path = env::current_exe().unwrap();
    let path = full_exe_path.parent().unwrap().to_str().unwrap();
    let config_path = format!("{}/config.json", path);
    let config_file =
        fs::read_to_string(&config_path).expect("Não foi possível ler o arquivo de configuração");
    gjson::get(&config_file, config_name).to_string()
}
