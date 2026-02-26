use sqlx::{migrate::MigrateDatabase, Connection, Sqlite, SqliteConnection, SqlitePool};
use serde::{Serialize, Deserialize};
use std::fs;

// URL БД
const DB_URL: &str = "sqlite://my_database.db";

/// Путь к SQL-скрипту для инициализации схемы
const SCHEMA_SQL_PATH: &str = "src/database/sql/create_DB.sql";

/// Просто функция для демонстарции архитектуры тестирования 
pub fn first_fn()-> i32{
    0
}

/// Главная структура, которая несет в себе весь функционал бд
/// и на которую навешиваются все функции
pub struct DataBase{
    pool: SqlitePool, // пул для соединений для выполнения запросов к БД
}

// Основные структуры даннных
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compound {
    pub entry: String,
    pub formula: Option<String>,
    pub exact_mass: Option<f64>,
    pub mol_weight: Option<f64>,
    pub names: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enzyme {
    pub entry: String,
    pub sysname: Option<String>,
    pub reaction_iubmb: Option<String>,
    pub names: Vec<String>,
    pub substrates: Vec<String>,
    pub products: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub entry: String,
    pub name: Option<String>,
    pub definition: Option<String>,
    pub enzymes: Vec<String>,
    pub left_compounds: Vec<String>, // entry соединений слева
    pub right_compounds: Vec<String>, // entry соединений справа
}

impl DataBase{
    // ==================== Инициализация ====================

    /// Создаёт новый экземпляр `DataBase`, инициализируя БД при необходимости.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        //Создаём файл БД, если его нет
        Self::ensure_database_exists().await?;

        //Выполняем SQL-скрипт для создания таблиц
        Self::run_schema_script().await?;

        // Создаём пул соединений
        let pool = SqlitePool::connect(DB_URL).await?;
        Ok(DataBase { pool })
    }

    /// Проверяет существование БД и создаёт её при отсутствии.
    async fn ensure_database_exists() -> Result<(), Box<dyn std::error::Error>> {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database at {}", DB_URL);
            Sqlite::create_database(DB_URL).await?;
            println!("Database created successfully.");
        } else {
            println!("Database already exists.");
        }
        Ok(())
    }

    /// Выполняет SQL-скрипт из файла.
    async fn run_schema_script() -> Result<(), Box<dyn std::error::Error>> {
        let script = fs::read_to_string(SCHEMA_SQL_PATH)?;
        let mut conn = SqliteConnection::connect(DB_URL).await?;
        sqlx::query("PRAGMA foreign_keys = ON;").execute(&mut conn).await?;
        sqlx::query(&script).execute(&mut conn).await?;
        println!("Schema script executed successfully.");
        Ok(())
    }

    // ==================== POST функции ====================

    /// Вставка Compound и связанных с ним имен
    pub async fn post_compound(&self, compound: Compound) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Вставка в таблицу compound
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

        // Вставка имен в таблицу compound_names
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

    /// Вставка Enzyme и связанных с ним данных (имена, субстраты, продукты)
    pub async fn post_enzyme(&self, enzyme: Enzyme) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Вставка в таблицу enzyme
        sqlx::query(
            "INSERT INTO enzyme (entry, sysname, reaction_iubmb) 
             VALUES (?, ?, ?)"
        )
        .bind(&enzyme.entry)
        .bind(&enzyme.sysname)
        .bind(&enzyme.reaction_iubmb)
        .execute(&mut *tx)
        .await?;

        // Вставка имен в таблицу enzyme_names
        for name in &enzyme.names {
            sqlx::query(
                "INSERT INTO enzyme_names (entry, name) VALUES (?, ?)"
            )
            .bind(&enzyme.entry)
            .bind(name)
            .execute(&mut *tx)
            .await?;
        }

        // Вставка субстратов в таблицу substrate
        for substrate_entry in &enzyme.substrates {
            sqlx::query(
                "INSERT INTO substrate (comp_entry, enzyme_entry) VALUES (?, ?)"
            )
            .bind(substrate_entry)
            .bind(&enzyme.entry)
            .execute(&mut *tx)
            .await?;
        }

        // Вставка продуктов в таблицу product
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

    /// Вставка Reaction и связанных с ним данных (ферменты, левые и правые части)
    pub async fn post_reaction(&self, reaction: Reaction) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Вставка в таблицу reaction
        sqlx::query(
            "INSERT INTO reaction (entry, name, definition) 
             VALUES (?, ?, ?)"
        )
        .bind(&reaction.entry)
        .bind(&reaction.name)
        .bind(&reaction.definition)
        .execute(&mut *tx)
        .await?;

        // Вставка связей с ферментами в таблицу reaction_enzyme
        for enzyme_entry in &reaction.enzymes {
            sqlx::query(
                "INSERT INTO reaction_enzyme (react_entry, enzyme_entry) VALUES (?, ?)"
            )
            .bind(&reaction.entry)
            .bind(enzyme_entry)
            .execute(&mut *tx)
            .await?;
        }

        // Вставка левых частей уравнения в таблицу equation_left
        for comp_entry in &reaction.left_compounds {
            sqlx::query(
                "INSERT INTO equation_left (react_entry, comp_entry) VALUES (?, ?)"
            )
            .bind(&reaction.entry)
            .bind(comp_entry)
            .execute(&mut *tx)
            .await?;
        }

        // Вставка правых частей уравнения в таблицу equation_right
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

    //Ваша функция, которая делает что-то полезное
    pub fn do_something(){
        //! В идеале, делает что-то полезное
    }
}

/// Здесь мы можем писать внутренние тесты, которые коротенькие и 
/// работают на проверку каких-то локальных проблем. 
/// Оформляется все следующим образом:
#[cfg(test)]
mod db_test{
    //! Макрос cfg(test) применяется только к целым модулям,
    //! поэтому после него обязательно пишем mod *название*. 
    //! Это своего рода просто блок, к которому применяется тестирование \
    
    //!Далее все так, как и в интеграционных тестах.
    //! Каждую функцию сопровождаем #[тест] и внутри пишем что хотим проверить
    #[test]
    fn another_unit_test(){

    }

    use super::*;
    use tokio;
    //Проверка создания БД
    #[tokio::test]
    async fn test_database_creation() {
        let db = DataBase::new().await;
        assert!(db.is_ok(), "Database should be created successfully");
    }


    //Проверка POST-запросов
    #[tokio::test]
    async fn test_post_reaction() {
        let db = DataBase::new().await.unwrap();
        
        // Вставим нужные соединения и фермент
        let water = Compound {
            entry: "C00001".to_string(),
            formula: Some("H2O".to_string()),
            exact_mass: Some(18.0106),
            mol_weight: Some(18.0153),
            names: vec!["Water".to_string()],
        };
        db.post_compound(water).await.unwrap();
        
        let atp = Compound {
            entry: "C00002".to_string(),
            formula: Some("C10H16N5O13P3".to_string()),
            exact_mass: Some(506.9957),
            mol_weight: Some(507.181),
            names: vec!["ATP".to_string()],
        };
        db.post_compound(atp).await.unwrap();
        
        let adp = Compound {
            entry: "C00003".to_string(),
            formula: Some("C10H15N5O10P2".to_string()),
            exact_mass: Some(427.0294),
            mol_weight: Some(427.201),
            names: vec!["ADP".to_string()],
        };
        db.post_compound(adp).await.unwrap();
        
        let enzyme = Enzyme {
            entry: "EC:2.7.1.1".to_string(),
            sysname: Some("ATP:hexose 6-phosphotransferase".to_string()),
            reaction_iubmb: Some("ATP + D-hexose = ADP + D-hexose 6-phosphate".to_string()),
            names: vec!["Hexokinase".to_string()],
            substrates: vec!["C00002".to_string()],
            products: vec!["C00003".to_string()],
        };
        db.post_enzyme(enzyme).await.unwrap();
        
        let reaction = Reaction {
            entry: "R00001".to_string(),
            name: Some("Hexokinase reaction".to_string()),
            definition: Some("ATP + D-glucose = ADP + D-glucose 6-phosphate".to_string()),
            enzymes: vec!["EC:2.7.1.1".to_string()],
            left_compounds: vec!["C00002".to_string()], // ATP слева
            right_compounds: vec!["C00003".to_string()], // ADP справа
        };
        
        let result = db.post_reaction(reaction).await;
        assert!(result.is_ok(), "Reaction insertion failed: {:?}", result.err());
    }

}