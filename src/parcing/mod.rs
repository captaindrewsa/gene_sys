use reqwest;

pub struct Parser{
    base_url: String,
}

impl Parser{
    
    /// Новый экземпляр Parser
    pub fn new() -> Self {
        Self {
            base_url: String::from("https://rest.kegg.jp"),
        }
    }

    /// Загрузка HTML по пути
    pub async fn fetch_html(&self, path: &str) -> Result<String, String> {
        
        // Собираем полный URL, например  = https://rest.kegg.jp/get/ + ec:5.4.2.2
        let url = format!("{}/get/{}", self.base_url, path); 
        // println!("Пытаюсь запросить: {}", url);

        // Создаём клиент
        let client = reqwest::Client::new();
        
        // GET-запрос
        let response = client
            .get(&url)                                                      // создаём запрос (не отправляем)
            .send()                                                         // отправляем
            .await                                                          // ждем ответ
            .map_err(|e| format!("Ошибка при GET-запросе: {}", e))?;        // если ошибка, преобразуем ее в строку
        
        // Проверяем статус
        if !response.status().is_success() {                                // если Ok, забираем значение,
            return Err(format!("HTTP ошибка: {}", response.status()));      // иначе - Err, выходим из функции с ошибкой
        }
        
        // Читаем тело ответа
        let html = response.text()                                          // хочу получить тело ответа как текст
            .await                                                          // ждём пока данные прочитаются
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))?;      // если ошибка, преобразуем ее в строку
        
        if html.contains("No such data was found") {
            return Err(format!("Ошибка! Для указанного запроса '{}' данные не найдены", path));
        }

        // println!("Успешно загружено {} байт", html.len());
        Ok(html)
    }
}


// Тесты
#[cfg(test)]
mod tests {

    use super::*;
    
    #[tokio::test]
    // Проверка получения данных
    async fn test_fetch_correct_url() {
        let path = "ec:5.4.2.2"; // phosphoglucomutase
        let parser = Parser::new();

        let result = parser.fetch_html(path).await;
    
        assert!(result.is_ok(), "Тест провален! Не удалось получить страницу HTML!");
        
        let html = result.unwrap();
        
        assert!(!html.is_empty(), "Тест провален! HTML пришел пустым!");

        assert!(html.contains("phosphoglycerate") || html.contains("phosphoglucomutase"), 
                "Тест провален! Полученная страница не сожержит название фермента");
        
        println!("Тест пройден! Получено {} байт", html.len());
        println!()
    }
    
    #[tokio::test]
    async fn test_fetch_incorrect_url() {
        let path = "non_existent_12345"; // несуществующая страница
        let parser = Parser::new();

        let result = parser.fetch_html(path).await;
        
        assert!(result.is_err(), "Тест провален: Функция должна была вернуть ошибку, но получила данные");
        
        if let Err(e) = result {
            println!("Тест пройден! Получена ожидаемая ошибка: {}", e);
        }
    }
}
