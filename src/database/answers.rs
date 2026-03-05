/// Типы ответов для операций с базой данных

/// Результат операций с базой данных
#[derive(Debug, PartialEq)]
pub enum DBAnswer {
    /// Успешное добавление (POST)
    PostOk,
    
    /// Успешное обновление (UPDATE)
    UpdateOk {
        /// Какие поля были обновлены
        updated_fields: Vec<String>,
        /// Сколько записей затронуто
        rows_affected: u64,
    },
    
    /// Успешное удаление (DELETE)
    DeleteOk(u64),
    
    /// Запись не найдена
    NotFound,
    
    /// Ошибка
    Error(String),
}

impl DBAnswer {
    /// Создать ответ об успешном POST
    pub fn post_ok() -> Self {
        DBAnswer::PostOk
    }
    
    /// Создать ответ об успешном UPDATE
    pub fn update_ok(updated_fields: Vec<String>, rows_affected: u64) -> Self {
        DBAnswer::UpdateOk {
            updated_fields,
            rows_affected,
        }
    }
    
    /// Создать ответ об успешном DELETE
    pub fn delete_ok(rows_affected: u64) -> Self {
        DBAnswer::DeleteOk(rows_affected)
    }
    
    /// Создать ответ "не найдено"
    pub fn not_found() -> Self {
        DBAnswer::NotFound
    }
    
    /// Создать ответ с ошибкой
    pub fn error(msg: impl Into<String>) -> Self {
        DBAnswer::Error(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_ok() {
        let result = DBAnswer::post_ok();
        assert_eq!(result, DBAnswer::PostOk);
    }

    #[test]
    fn test_update_ok() {
        let fields = vec!["name".to_string(), "formula".to_string()];
        let result = DBAnswer::update_ok(fields.clone(), 1);
        
        match result {
            DBAnswer::UpdateOk { updated_fields, rows_affected } => {
                assert_eq!(updated_fields, fields);
                assert_eq!(rows_affected, 1);
            }
            _ => panic!("Expected UpdateOk"),
        }
    }

    #[test]
    fn test_delete_ok() {
        let result = DBAnswer::delete_ok(5);
        assert_eq!(result, DBAnswer::DeleteOk(5));
    }

    #[test]
    fn test_not_found() {
        let result = DBAnswer::not_found();
        assert_eq!(result, DBAnswer::NotFound);
    }

    #[test]
    fn test_error() {
        let result = DBAnswer::error("Database error");
        assert!(matches!(result, DBAnswer::Error(_)));
    }
}