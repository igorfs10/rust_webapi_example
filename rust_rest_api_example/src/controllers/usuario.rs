use crate::structs::response::Response;

use actix_web::{delete, get, post, put, web, Responder};
use rust_rest_api_example_data::structs::usuario::Usuario;
use rust_rest_api_example_data::traits::crud::Crud;

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
    match Usuario::get_all().await {
        Ok(result) => {
            response = Response::new_response("Sucesso".to_string(), Some(result));
        }
        Err(erro) => {
            response = Response::new_response(format!("{}", erro), None);
        }
    }
    web::Json(response)
}

#[get("/usuarios/{id}")]
async fn get_by_id(path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let response;
    match Usuario::get_by_id(id).await {
        Ok(result) => {
            response = Response::new_response("Sucesso".to_string(), Some(result));
        }
        Err(erro) => {
            response = Response::new_response(format!("{}", erro), None);
        }
    }
    web::Json(response)
}

#[post("/usuarios")]
async fn add(usuario: web::Json<Usuario>) -> impl Responder {
    let response: Response<bool>;
    match Usuario::add(usuario.into_inner()).await {
        Ok(_) => {
            response = Response::new_response("Criado com sucesso".to_string(), None);
        }
        Err(erro) => {
            response = Response::new_response(format!("{}", erro), None);
        }
    }
    web::Json(response)
}

#[put("/usuarios")]
async fn update(usuario: web::Json<Usuario>) -> impl Responder {
    let response: Response<bool>;
    match Usuario::update(usuario.into_inner()).await {
        Ok(_) => {
            response = Response::new_response("Atualizado com sucesso".to_string(), None);
        }
        Err(erro) => {
            response = Response::new_response(format!("{}", erro), None);
        }
    }
    web::Json(response)
}

#[delete("/usuarios/{id}")]
async fn remove(path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let response: Response<bool>;
    match Usuario::remove(id).await {
        Ok(_) => {
            response = Response::new_response("Removido com sucesso".to_string(), None);
        }
        Err(erro) => {
            response = Response::new_response(format!("{}", erro), None);
        }
    }
    web::Json(response)
}
