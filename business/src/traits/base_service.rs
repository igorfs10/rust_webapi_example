use db::{async_trait::async_trait, traits::base_repo::BaseRepo};

#[async_trait]
pub trait BaseService<T: BaseRepo> {
    async fn get_all() -> Result<Vec<T::ModelType>, String> {
        match T::get_all().await {
            Ok(result) => Ok(result),
            Err(error) => Err(error.to_string()),
        }
    }

    async fn get_by_id(id: T::IdType) -> Result<T::ModelType, String>
    where
        T::IdType: Send + 'static,
    {
        match T::get_by_id(id).await {
            Ok(result) => Ok(result),
            Err(error) => Err(error.to_string()),
        }
    }

    async fn insert(data: T::ModelType) -> Result<String, String>
    where
        T::ModelType: Send + 'static,
    {
        match T::insert(data).await {
            Ok(_) => Ok("Item criado com sucesso.".to_string()),
            Err(error) => Err(error.to_string()),
        }
    }

    async fn update(data: T::ModelType) -> Result<String, String>
    where
        T::ModelType: Send + 'static,
    {
        match T::update(data).await {
            Ok(_) => Ok("Item atualizado com sucesso.".to_string()),
            Err(error) => Err(error.to_string()),
        }
    }

    async fn delete(id: T::IdType) -> Result<String, String>
    where
        T::IdType: Send + 'static,
    {
        match T::delete(id).await {
            Ok(_) => Ok("Item excluído com sucesso.".to_string()),
            Err(error) => Err(error.to_string()),
        }
    }
}
