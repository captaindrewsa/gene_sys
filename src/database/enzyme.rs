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

    /// Обновление Enzyme и связанных данных
    pub async fn update_enzyme(&self, enzyme: Enzyme) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        // Проверка существования
        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM enzyme WHERE entry = ?")
            .bind(&enzyme.entry)
            .fetch_one(&mut *tx)
            .await?;
        if exists.0 == 0 {
            return Ok(());
        }

        // Обновление основных полей
        sqlx::query("UPDATE enzyme SET sysname = ?, reaction_iubmb = ? WHERE entry = ?")
            .bind(&enzyme.sysname)
            .bind(&enzyme.reaction_iubmb)
            .bind(&enzyme.entry)
            .execute(&mut *tx)
            .await?;

        // Удаляем старые имена
        sqlx::query("DELETE FROM enzyme_names WHERE entry = ?")
            .bind(&enzyme.entry)
            .execute(&mut *tx)
            .await?;
        // Удаляем старые субстраты
        sqlx::query("DELETE FROM substrate WHERE enzyme_entry = ?")
            .bind(&enzyme.entry)
            .execute(&mut *tx)
            .await?;
        // Удаляем старые продукты
        sqlx::query("DELETE FROM product WHERE enzyme_entry = ?")
            .bind(&enzyme.entry)
            .execute(&mut *tx)
            .await?;

        // Вставляем новые имена
        for name in &enzyme.names {
            sqlx::query("INSERT INTO enzyme_names (entry, name) VALUES (?, ?)")
                .bind(&enzyme.entry)
                .bind(name)
                .execute(&mut *tx)
                .await?;
        }

        // Вставляем субстраты
        for substrate_entry in &enzyme.substrates {
            sqlx::query("INSERT INTO substrate (comp_entry, enzyme_entry) VALUES (?, ?)")
                .bind(substrate_entry)
                .bind(&enzyme.entry)
                .execute(&mut *tx)
                .await?;
        }

        // Вставляем продукты
        for product_entry in &enzyme.products {
            sqlx::query("INSERT INTO product (comp_entry, enzyme_entry) VALUES (?, ?)")
                .bind(product_entry)
                .bind(&enzyme.entry)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        println!("Enzyme {} updated successfully", enzyme.entry);
        Ok(())
    }

    /// GET запрос
    /// Загружает фермент по entry
    /// Возвращает `None`, если запись не найдена.
    pub async fn get_enzyme_by_entry(&self, entry: &str) -> Result<Option<Enzyme>, sqlx::Error> {
        let row: Option<(String, Option<String>, Option<String>)> = sqlx::query_as(
            "SELECT entry, sysname, reaction_iubmb FROM enzyme WHERE entry = ?"
        )
        .bind(entry)
        .fetch_optional(&self.pool)
        .await?;

        if let Some((entry, sysname, reaction_iubmb)) = row {
            let names: Vec<String> = sqlx::query_scalar(
                "SELECT name FROM enzyme_names WHERE entry = ?"
            )
            .bind(&entry)
            .fetch_all(&self.pool)
            .await?;
            let substrates: Vec<String> = sqlx::query_scalar(
                "SELECT comp_entry FROM substrate WHERE enzyme_entry = ?"
            )
            .bind(&entry)
            .fetch_all(&self.pool)
            .await?;
            let products: Vec<String> = sqlx::query_scalar(
                "SELECT comp_entry FROM product WHERE enzyme_entry = ?"
            )
            .bind(&entry)
            .fetch_all(&self.pool)
            .await?;
            Ok(Some(Enzyme {
                entry,
                sysname,
                reaction_iubmb,
                names,
                substrates,
                products,
            }))
        } else {
            Ok(None)
        }
    }
}