use serde::{Serialize, Deserialize};
use crate::database::DataBase;
use sqlx::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub entry: String,
    pub name: Option<String>,
    pub definition: Option<String>,
    pub enzymes: Vec<String>,
    pub left_compounds: Vec<String>,
    pub right_compounds: Vec<String>,
}

impl DataBase {
    /// Вставка Reaction и связанных с ним данных
    pub async fn post_reaction(&self, reaction: Reaction) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO reaction (entry, name, definition) 
             VALUES (?, ?, ?)"
        )
        .bind(&reaction.entry)
        .bind(&reaction.name)
        .bind(&reaction.definition)
        .execute(&mut *tx)
        .await?;

        for enzyme_entry in &reaction.enzymes {
            sqlx::query(
                "INSERT INTO reaction_enzyme (react_entry, enzyme_entry) VALUES (?, ?)"
            )
            .bind(&reaction.entry)
            .bind(enzyme_entry)
            .execute(&mut *tx)
            .await?;
        }

        for comp_entry in &reaction.left_compounds {
            sqlx::query(
                "INSERT INTO equation_left (react_entry, comp_entry) VALUES (?, ?)"
            )
            .bind(&reaction.entry)
            .bind(comp_entry)
            .execute(&mut *tx)
            .await?;
        }

        for comp_entry in &reaction.right_compounds {
            sqlx::query(
                "INSERT INTO equation_right (react_entry, comp_entry) VALUES (?, ?)"
            )
            .bind(&reaction.entry)
            .bind(comp_entry)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        println!("Reaction {} inserted successfully", reaction.entry);
        Ok(())
    }

     /// Обновление Reaction и связанных данных
    pub async fn update_reaction(&self, reaction: Reaction) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        // Проверка существования
        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM reaction WHERE entry = ?")
            .bind(&reaction.entry)
            .fetch_one(&mut *tx)
            .await?;
        if exists.0 == 0 {
            return Ok(());
        }

        // Обновление основных полей
        sqlx::query("UPDATE reaction SET name = ?, definition = ? WHERE entry = ?")
            .bind(&reaction.name)
            .bind(&reaction.definition)
            .bind(&reaction.entry)
            .execute(&mut *tx)
            .await?;

        // Удаляем старые связи
        sqlx::query("DELETE FROM reaction_enzyme WHERE react_entry = ?")
            .bind(&reaction.entry)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM equation_left WHERE react_entry = ?")
            .bind(&reaction.entry)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM equation_right WHERE react_entry = ?")
            .bind(&reaction.entry)
            .execute(&mut *tx)
            .await?;

        // Вставляем ферменты
        for enzyme_entry in &reaction.enzymes {
            sqlx::query("INSERT INTO reaction_enzyme (react_entry, enzyme_entry) VALUES (?, ?)")
                .bind(&reaction.entry)
                .bind(enzyme_entry)
                .execute(&mut *tx)
                .await?;
        }

        // Вставляем левые соединения
        for comp_entry in &reaction.left_compounds {
            sqlx::query("INSERT INTO equation_left (react_entry, comp_entry) VALUES (?, ?)")
                .bind(&reaction.entry)
                .bind(comp_entry)
                .execute(&mut *tx)
                .await?;
        }

        // Вставляем правые соединения
        for comp_entry in &reaction.right_compounds {
            sqlx::query("INSERT INTO equation_right (react_entry, comp_entry) VALUES (?, ?)")
                .bind(&reaction.entry)
                .bind(comp_entry)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        println!("Reaction {} updated successfully", reaction.entry);
        Ok(())
    }
}