use data::structs::usuario::Usuario;
use db::structs::base::Base;

use crate::structs::service::Service;
use crate::traits::base_service::BaseService;

impl BaseService<Base<Usuario>> for Service {}

pub struct UsuarioService;

impl UsuarioService {
    pub async fn get_all() -> Result<Vec<Usuario>, String> {
        Service::get_all().await
    }

    pub async fn get_by_id(id: i64) -> Result<Usuario, String> {
        Service::get_by_id(id).await
    }

    pub async fn insert(usuario: Usuario) -> Result<String, String> {
        Service::insert(usuario).await
    }

    pub async fn update(usuario: Usuario) -> Result<String, String> {
        Service::update(usuario).await
    }

    pub async fn delete(id: i64) -> Result<String, String> {
        Service::delete(id).await
    }
}
