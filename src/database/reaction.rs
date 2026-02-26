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
}