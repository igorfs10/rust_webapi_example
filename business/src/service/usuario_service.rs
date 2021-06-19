use data::structs::usuario::Usuario;
use data::traits::base_repo::BaseRepo;

pub struct UsuarioService;

impl UsuarioService {
    pub async fn get_all() -> Result<Vec<Usuario>, String> {
        match Usuario::get_all().await {
            Ok(result) => Ok(result),
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn get_by_id(id: i64) -> Result<Usuario, String> {
        match Usuario::get_by_id(id).await {
            Ok(result) => Ok(result),
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn insert(usuario: Usuario) -> Result<String, String> {
        match Usuario::insert(usuario).await {
            Ok(_) => Ok("Usuário criado com sucesso.".to_string()),
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn update(usuario: Usuario) -> Result<String, String> {
        match Usuario::update(usuario).await {
            Ok(_) => Ok("Usuário atualizado com sucesso.".to_string()),
            Err(error) => Err(error.to_string()),
        }
    }

    pub async fn delete(id: i64) -> Result<String, String> {
        match Usuario::delete(id).await {
            Ok(_) => Ok("Usuário excluído com sucesso.".to_string()),
            Err(error) => Err(error.to_string()),
        }
    }
}
