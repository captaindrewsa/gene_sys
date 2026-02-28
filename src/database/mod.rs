use sqlx::{migrate::MigrateDatabase, Connection, Sqlite, SqliteConnection, SqlitePool};
use std::fs;

// URL БД
const DB_URL: &str = "sqlite://my_database.db";

/// Путь к SQL-скрипту для инициализации схемы
const SCHEMA_SQL_PATH: &str = "src/database/sql/create_DB.sql";

// Объявляем подмодули, содержащие структуры и их методы
mod compound;
mod enzyme;
mod reaction;

// Реэкспортируем структуры, чтобы они были доступны извне как database::Compound и т.д.
pub use compound::Compound;
pub use enzyme::Enzyme;
pub use reaction::Reaction;

/// Просто функция для демонстарции архитектуры тестирования 
pub fn first_fn()-> i32{
    0
}

/// Главная структура, которая несет в себе весь функционал бд
/// и на которую навешиваются все функции
pub struct DataBase{
    pool: SqlitePool, // пул для соединений для выполнения запросов к БД
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

    #[tokio::test]
    async fn test_update_compound() {
        let db = DataBase::new().await.unwrap();

        let compound = Compound {
            entry: "C99999".to_string(),
            formula: Some("H2O".to_string()),
            exact_mass: Some(18.0106),
            mol_weight: Some(18.0153),
            names: vec!["Water".to_string()],
        };
        db.post_compound(compound.clone()).await.unwrap();


        let updated = Compound {
            entry: "C99999".to_string(),
            formula: Some("H2O2026".to_string()),
            exact_mass: Some(29.0106),
            mol_weight: Some(38.0153),
            names: vec!["Water".to_string(), "Dihydrogen oxide".to_string()],
        };
        let result = db.update_compound(updated).await;
        assert!(matches!(result, Ok(())));
    }

    #[tokio::test]
    async fn test_update_enzyme() {
        let db = DataBase::new().await.unwrap();

        // Сначала вставим необходимые соединения
        let comp1 = Compound {
            entry: "C10001".to_string(),
            formula: Some("C6H12O6".to_string()),
            exact_mass: Some(180.0634),
            mol_weight: Some(180.156),
            names: vec!["Glucose".to_string()],
        };
        db.post_compound(comp1).await.unwrap();

        let comp2 = Compound {
            entry: "C10002".to_string(),
            formula: Some("C6H12O6".to_string()),
            exact_mass: Some(180.0634),
            mol_weight: Some(180.156),
            names: vec!["Glucose-6-phosphate".to_string()],
        };
        db.post_compound(comp2).await.unwrap();

        // Вставляем исходный enzyme
        let enzyme = Enzyme {
            entry: "EC:2.7.1.2".to_string(),
            sysname: Some("ATP:glucose 6-phosphotransferase".to_string()),
            reaction_iubmb: Some("ATP + glucose = ADP + glucose-6-phosphate".to_string()),
            names: vec!["Glucokinase".to_string()],
            substrates: vec!["C10001".to_string()],
            products: vec!["C10002".to_string()],
        };
        db.post_enzyme(enzyme).await.unwrap();

        let comp3 = Compound {
            entry: "C10003".to_string(),
            formula: Some("C6H12O6".to_string()),
            exact_mass: Some(180.0634),
            mol_weight: Some(180.156),
            names: vec!["Mannose".to_string()],
        };
        db.post_compound(comp3).await.unwrap();

        let comp4 = Compound {
            entry: "C10004".to_string(),
            formula: Some("C6H13O9P".to_string()),
            exact_mass: Some(260.0297),
            mol_weight: Some(260.136),
            names: vec!["Mannose-6-phosphate".to_string()],
        };
        db.post_compound(comp4).await.unwrap();

        let updated_enzyme = Enzyme {
            entry: "EC:2.7.1.2".to_string(),
            sysname: Some("ATP:glucose 6-phosphotransferase".to_string()),
            reaction_iubmb: Some("ATP + glucose = ADP + glucose-6-phosphate".to_string()),
            names: vec!["Glucokinase".to_string(), "Hexokinase type IV".to_string()],
            substrates: vec!["C10001".to_string(), "C10003".to_string()],
            products: vec!["C10002".to_string(), "C10004".to_string()],
        };

        let result = db.update_enzyme(updated_enzyme).await;
        assert!(matches!(result, Ok(())));
    }

    #[tokio::test]
    async fn test_update_reaction() {
        let db = DataBase::new().await.unwrap();

        let comp1 = Compound {
            entry: "C20001".to_string(),
            formula: Some("ATP".to_string()),
            exact_mass: None,
            mol_weight: None,
            names: vec!["ATP".to_string()],
        };
        db.post_compound(comp1).await.unwrap();

        let comp2 = Compound {
            entry: "C20002".to_string(),
            formula: Some("ADP".to_string()),
            exact_mass: None,
            mol_weight: None,
            names: vec!["ADP".to_string()],
        };
        db.post_compound(comp2).await.unwrap();

        let comp3 = Compound {
            entry: "C20003".to_string(),
            formula: Some("Glucose".to_string()),
            exact_mass: None,
            mol_weight: None,
            names: vec!["Glucose".to_string()],
        };
        db.post_compound(comp3).await.unwrap();

        let comp4 = Compound {
            entry: "C20004".to_string(),
            formula: Some("Glucose-6-phosphate".to_string()),
            exact_mass: None,
            mol_weight: None,
            names: vec!["Glucose-6-phosphate".to_string()],
        };
        db.post_compound(comp4).await.unwrap();

        let enzyme = Enzyme {
            entry: "EC:2.7.1.3".to_string(),
            sysname: Some("ATP:glucose 6-phosphotransferase".to_string()),
            reaction_iubmb: None,
            names: vec!["Hexokinase".to_string()],
            substrates: vec!["C20001".to_string(), "C20003".to_string()],
            products: vec!["C20002".to_string(), "C20004".to_string()],
        };
        db.post_enzyme(enzyme).await.unwrap();

        let reaction = Reaction {
            entry: "R10001".to_string(),
            name: Some("Hexokinase reaction".to_string()),
            definition: Some("ATP + D-glucose = ADP + D-glucose 6-phosphate".to_string()),
            enzymes: vec!["EC:2.7.1.3".to_string()],
            left_compounds: vec!["C20001".to_string(), "C20003".to_string()],
            right_compounds: vec!["C20002".to_string(), "C20004".to_string()],
        };
        db.post_reaction(reaction).await.unwrap();

        let enzyme2 = Enzyme {
            entry: "EC:2.7.1.4".to_string(),
            sysname: Some("ATP:glucose 6-phosphotransferase (alternative)".to_string()),
            reaction_iubmb: None,
            names: vec!["Glucokinase".to_string()],
            substrates: vec!["C20001".to_string(), "C20003".to_string()],
            products: vec!["C20002".to_string(), "C20004".to_string()],
        };
        db.post_enzyme(enzyme2).await.unwrap();

        let water = Compound {
            entry: "C20005".to_string(),
            formula: Some("H2O".to_string()),
            exact_mass: Some(18.0106),
            mol_weight: Some(18.0153),
            names: vec!["Water".to_string()],
        };
        db.post_compound(water).await.unwrap();

        let updated_reaction = Reaction {
            entry: "R10001".to_string(),
            name: Some("Hexokinase reaction (updated)".to_string()),
            definition: Some("ATP + D-glucose = ADP + D-glucose 6-phosphate".to_string()),
            enzymes: vec!["EC:2.7.1.3".to_string(), "EC:2.7.1.4".to_string()],
            left_compounds: vec!["C20001".to_string(), "C20003".to_string(), "C20005".to_string()],
            right_compounds: vec!["C20004".to_string(), "C20005".to_string()],
        };

        let result = db.update_reaction(updated_reaction).await;
        assert!(matches!(result, Ok(())));
    }
}