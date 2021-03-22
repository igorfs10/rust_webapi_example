use actix_web::{delete, get, post, put, web, Responder};
use postgre_api_data::structs::response::Response;
use postgre_api_data::structs::table::Table;
use postgre_api_data::structs::usuario::Usuario;
use postgre_api_data::traits::crud::CRUD;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_by_id);
    cfg.service(update);
    cfg.service(add);
    cfg.service(remove);
}

#[get("/usuarios")]
async fn get_all() -> impl Responder {
    let response;
    match Table::<Usuario>::get_all() {
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
async fn get_by_id(web::Path(id): web::Path<i64>) -> impl Responder {
    let response;
    match Table::<Usuario>::get_by_id(id) {
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
    match Table::<Usuario>::add(usuario.into_inner()) {
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
    match Table::<Usuario>::update(usuario.into_inner()) {
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
async fn remove(web::Path(id): web::Path<i64>) -> impl Responder {
    let response: Response<bool>;
    match Table::<Usuario>::remove(id) {
        Ok(_) => {
            response = Response::new_response("Removido com sucesso".to_string(), None);
        }
        Err(erro) => {
            response = Response::new_response(format!("{}", erro), None);
        }
    }
    web::Json(response)
}
