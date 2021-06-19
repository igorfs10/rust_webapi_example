use crate::structs::response::Response;

use actix_web::{delete, get, http::StatusCode, post, put, web, Responder};
use business::service::usuario_service::UsuarioService;
use data::structs::usuario::Usuario;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_by_id);
    cfg.service(update);
    cfg.service(add);
    cfg.service(remove);
    cfg.service(get);
}

#[get("/")]
async fn get() -> impl Responder {
    web::Json("response")
}

#[get("/usuarios")]
async fn get_all() -> impl Responder {
    let response;
    match UsuarioService::get_all().await {
        Ok(result) => {
            response = Response::new_response("".to_string(), Some(result));
            web::Json(response).with_status(StatusCode::OK)
        }
        Err(error) => {
            response = Response::new_response(error, None);
            web::Json(response).with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[get("/usuarios/{id}")]
async fn get_by_id(path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let response;
    match UsuarioService::get_by_id(id).await {
        Ok(result) => {
            response = Response::new_response("".to_string(), Some(result));
            web::Json(response).with_status(StatusCode::OK)
        }
        Err(error) => {
            response = Response::new_response(error, None);
            web::Json(response).with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[post("/usuarios")]
async fn add(usuario: web::Json<Usuario>) -> impl Responder {
    let response: Response<bool>;
    match UsuarioService::insert(usuario.into_inner()).await {
        Ok(_) => {
            response = Response::new_response("Criado com sucesso".to_string(), None);
            web::Json(response).with_status(StatusCode::OK)
        }
        Err(error) => {
            response = Response::new_response(error, None);
            web::Json(response).with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[put("/usuarios")]
async fn update(usuario: web::Json<Usuario>) -> impl Responder {
    let response: Response<bool>;
    match UsuarioService::update(usuario.into_inner()).await {
        Ok(_) => {
            response = Response::new_response("Atualizado com sucesso".to_string(), None);
            web::Json(response).with_status(StatusCode::OK)
        }
        Err(error) => {
            response = Response::new_response(error, None);
            web::Json(response).with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[delete("/usuarios/{id}")]
async fn remove(path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let response: Response<bool>;
    match UsuarioService::delete(id).await {
        Ok(_) => {
            response = Response::new_response("Removido com sucesso".to_string(), None);
            web::Json(response).with_status(StatusCode::OK)
        }
        Err(error) => {
            response = Response::new_response(error, None);
            web::Json(response).with_status(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
