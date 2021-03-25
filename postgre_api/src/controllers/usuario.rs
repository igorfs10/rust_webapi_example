use actix_web::{delete, get, post, put, web, Responder};
use postgre_api_data::structs::table::Table;
use postgre_api_data::structs::usuario::Usuario;
use postgre_api_data::structs::{response::Response, state::State};
use postgre_api_data::traits::crud::CRUD;

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
async fn get_all(app_state: web::Data<State>) -> impl Responder {
    let response;
    match Table::<Usuario>::get_all(app_state.db_pool.clone()).await {
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
async fn get_by_id(app_state: web::Data<State>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let response;
    match Table::<Usuario>::get_by_id(app_state.db_pool.clone(), id).await {
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
async fn add(app_state: web::Data<State>, usuario: web::Json<Usuario>) -> impl Responder {
    let response: Response<bool>;
    match Table::<Usuario>::add(app_state.db_pool.clone(), usuario.into_inner()).await {
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
async fn update(app_state: web::Data<State>, usuario: web::Json<Usuario>) -> impl Responder {
    let response: Response<bool>;
    match Table::<Usuario>::update(app_state.db_pool.clone(), usuario.into_inner()).await {
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
async fn remove(app_state: web::Data<State>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let response: Response<bool>;
    match Table::<Usuario>::remove(app_state.db_pool.clone(), id).await {
        Ok(_) => {
            response = Response::new_response("Removido com sucesso".to_string(), None);
        }
        Err(erro) => {
            response = Response::new_response(format!("{}", erro), None);
        }
    }
    web::Json(response)
}
