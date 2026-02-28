use serde::{Serialize, Deserialize};
use crate::database::DataBase;
use sqlx::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compound {
    pub entry: String,
    pub formula: Option<String>,
    pub exact_mass: Option<f64>,
    pub mol_weight: Option<f64>,
    pub names: Vec<String>,
}

impl DataBase {
    /// Вставка Compound и связанных с ним имен
    pub async fn post_compound(&self, compound: Compound) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO compound (entry, formula, exact_mass, mol_weight) 
             VALUES (?, ?, ?, ?)"
        )
        .bind(&compound.entry)
        .bind(&compound.formula)
        .bind(compound.exact_mass)
        .bind(compound.mol_weight)
        .execute(&mut *tx)
        .await?;

        for name in &compound.names {
            sqlx::query(
                "INSERT INTO compound_names (entry, name) VALUES (?, ?)"
            )
            .bind(&compound.entry)
            .bind(name)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        println!("Compound {} inserted successfully", compound.entry);
        Ok(())
    }

    /// Обновление Compound и связанных имён
    pub async fn update_compound(&self, compound: Compound) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        // Проверяем, существует ли запись
        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM compound WHERE entry = ?")
            .bind(&compound.entry)
            .fetch_one(&mut *tx)
            .await?;
        if exists.0 == 0 {
            return Ok(());
        }

        // Обновляем основные поля
        sqlx::query(
            "UPDATE compound SET formula = ?, exact_mass = ?, mol_weight = ? WHERE entry = ?"
        )
        .bind(&compound.formula)
        .bind(compound.exact_mass)
        .bind(compound.mol_weight)
        .bind(&compound.entry)
        .execute(&mut *tx)
        .await?;

        // Удаляем старые имена
        sqlx::query("DELETE FROM compound_names WHERE entry = ?")
            .bind(&compound.entry)
            .execute(&mut *tx)
            .await?;

        // Вставляем новые имена
        for name in &compound.names {
            sqlx::query("INSERT INTO compound_names (entry, name) VALUES (?, ?)")
                .bind(&compound.entry)
                .bind(name)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        println!("Compound {} updated successfully", compound.entry);
        Ok(())
    }
}