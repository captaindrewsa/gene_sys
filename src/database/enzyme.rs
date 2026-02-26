use serde::{Serialize, Deserialize};
use crate::database::DataBase;
use sqlx::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enzyme {
    pub entry: String,
    pub sysname: Option<String>,
    pub reaction_iubmb: Option<String>,
    pub names: Vec<String>,
    pub substrates: Vec<String>,
    pub products: Vec<String>,
}

impl DataBase {
    /// Вставка Enzyme и связанных с ним данных
    pub async fn post_enzyme(&self, enzyme: Enzyme) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO enzyme (entry, sysname, reaction_iubmb) 
             VALUES (?, ?, ?)"
        )
        .bind(&enzyme.entry)
        .bind(&enzyme.sysname)
        .bind(&enzyme.reaction_iubmb)
        .execute(&mut *tx)
        .await?;

        for name in &enzyme.names {
            sqlx::query(
                "INSERT INTO enzyme_names (entry, name) VALUES (?, ?)"
            )
            .bind(&enzyme.entry)
            .bind(name)
            .execute(&mut *tx)
            .await?;
        }

        for substrate_entry in &enzyme.substrates {
            sqlx::query(
                "INSERT INTO substrate (comp_entry, enzyme_entry) VALUES (?, ?)"
            )
            .bind(substrate_entry)
            .bind(&enzyme.entry)
            .execute(&mut *tx)
            .await?;
        }

        for product_entry in &enzyme.products {
            sqlx::query(
                "INSERT INTO product (comp_entry, enzyme_entry) VALUES (?, ?)"
            )
            .bind(product_entry)
            .bind(&enzyme.entry)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        println!("Enzyme {} inserted successfully", enzyme.entry);
        Ok(())
    }
}