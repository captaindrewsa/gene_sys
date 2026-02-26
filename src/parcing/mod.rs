use std::time::{Duration, Instant};
use tokio::time::sleep;
use reqwest;

const REQUEST_INTERVAL: u64 = 334;               // ограничение на запросы (1000 мс / 3 < 334 мс)

pub struct Parser{
    base_url: String,
    last_request_time: Option<Instant>,
}

impl Parser{
    
    /// Новый экземпляр Parser
    pub fn new() -> Self {

        Self {
            base_url: String::from("https://rest.kegg.jp"),                 // начальный адрес сайта
            last_request_time: None,                                        // сколько времени прошло с последнего запроса (мс)
        }
    }


    /// Ограничение запросов по времени
    async fn rate_limiter(&mut self) {
                                              
        if let Some(last) = self.last_request_time {                        // проверяем, был ли до этого запрос (!= None)

            let elapsed = last.elapsed();                                   // записываем, сколько прошло времени
            if elapsed < Duration::from_millis(REQUEST_INTERVAL) {          // прошло ли достаточно времени (1000 мс / 3 < 334 мс) между запросами

                let wait_time = Duration::from_millis(REQUEST_INTERVAL) - elapsed;     // cколько нужно подождать
                sleep(wait_time).await;                                     // ждем
            }
        }
        self.last_request_time = Some(Instant::now());                      // засекаем время
    }


    /// Загрузка HTML по пути
    pub async fn fetch_html(&mut self, path: &str) -> Result<String, String> {

        // Проверяем можно ли отправить запрос
        self.rate_limiter().await;  
        
        // Собираем полный URL, например  = https://www.kegg.jp + /entry/ec:5.4.2.2
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


/// Тесты
#[cfg(test)]
mod tests {

    use super::*;
    
    #[tokio::test]
    /// Проверка получения данных
    async fn test_fetch_correct_url() {
        let path = "ec:5.4.2.2";                                            // phosphoglucomutase
        let mut parser = Parser::new();

        let result = parser.fetch_html(path).await;
    
        assert!(result.is_ok(), "Тест провален! Не удалось получить страницу HTML!");
        
        let html = result.unwrap();
        
        assert!(!html.is_empty(), "Тест провален! HTML пришел пустым!");

        assert!(html.contains("phosphoglycerate") || html.contains("phosphoglucomutase"), 
                "Тест провален! Полученная страница не сожержит название фермента");
        
        println!("Тест пройден! Получено {} байт", html.len());
        println!();
    }

    
    #[tokio::test]
    /// Проверка получения данных с несуществующей страницы
    async fn test_fetch_incorrect_url() {
        let path = "non_existent_12345";                                   // несуществующая страница
        let mut parser = Parser::new();

        let result = parser.fetch_html(path).await;
        
        assert!(result.is_err(), "Тест провален: Функция должна была вернуть ошибку, но получила данные");
        
        if let Err(e) = result {
            println!("Тест пройден! Получена ожидаемая ошибка: {}", e);
        }
        println!();
    }


    #[tokio::test]
    // Проверка rate limiter (делаем 5 запросов подряд)
    async fn test_rate_limiter() {
        let mut parser = Parser::new();
        let mut timestamps = Vec::new();                                   // вектор для хранения времени каждого запроса
        
        // Делаем 5 запросов и запоминаем время каждого
        for i in 1..=5 {
            let result = parser.fetch_html("ec:5.4.2.2").await;
            let after = Instant::now();                                     // время после запроса
            
            assert!(result.is_ok(), "Тест провален! Запрос №{} не удался", i);
            
            // Сохраняем время, когда запрос был выполнен
            timestamps.push(after);

            if i > 1 {
                let interval = after - timestamps[i-2];
                println!("Время от предыдущего запроса: {:?}", interval);
                
                assert!(interval >= Duration::from_millis(REQUEST_INTERVAL), 
                        "Тест провален! Запросы идут слишком часто: {:?} < {}ms", interval, REQUEST_INTERVAL);
            }
        }

        println!("Тест пройден! Запросы не превышают заданный лимит");
        println!();
    }
}
