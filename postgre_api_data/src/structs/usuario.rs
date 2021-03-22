use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize)]
pub struct Usuario {
    pub id: i64,
    pub nome: String,
}

impl<'c> FromRow<'c, PgRow> for Usuario {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Usuario {
            id: row.get(0),
            nome: row.get(1),
        })
    }
}
