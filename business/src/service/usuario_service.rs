use data::structs::usuario::Usuario;
use db::structs::base::Base;

use crate::structs::service::Service;
use crate::traits::base_service::BaseService;

impl BaseService<Base<Usuario>> for Service {}

pub struct UsuarioService;

impl UsuarioService {
    pub async fn get_all() -> Result<Vec<Usuario>, String> {
        match Service::get_all().await {
            Ok(result) => Ok(result),
            Err(error) => Err(error),
        }
    }

    pub async fn get_by_id(id: i64) -> Result<Usuario, String> {
        match Service::get_by_id(id).await {
            Ok(result) => Ok(result),
            Err(error) => Err(error),
        }
    }

    pub async fn insert(usuario: Usuario) -> Result<String, String> {
        match Service::insert(usuario).await {
            Ok(_) => Ok("Usuário criado com sucesso.".to_string()),
            Err(error) => Err(error),
        }
    }

    pub async fn update(usuario: Usuario) -> Result<String, String> {
        match Service::update(usuario).await {
            Ok(_) => Ok("Usuário atualizado com sucesso.".to_string()),
            Err(error) => Err(error),
        }
    }

    pub async fn delete(id: i64) -> Result<String, String> {
        match Service::delete(id).await {
            Ok(_) => Ok("Usuário excluído com sucesso.".to_string()),
            Err(error) => Err(error),
        }
    }
}
