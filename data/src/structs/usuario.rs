use serde::{Deserialize, Serialize};

pub type IdTypeUsuario = i64;

// Struct que recebe usuário do banco
#[derive(Serialize, Deserialize)]
pub struct Usuario {
    pub id: i64,
    pub nome: String,
}
