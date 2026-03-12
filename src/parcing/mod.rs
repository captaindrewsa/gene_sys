use std::time::{Duration, Instant};
use tokio::time::sleep;
use reqwest;

const REQUEST_INTERVAL: Duration = Duration::from_millis(334);               // ограничение на запросы (1000 мс / 3 < 334 мс)

pub struct Parser{
    client: reqwest::Client,
    last_request_time: Option<Instant>,
}

impl Parser{
    
    /// Новый экземпляр Parser
    pub fn new() -> Self {

        Self {     
            client: reqwest::Client::new(),                                 // создаем клиент
            last_request_time: None,                                        // сколько времени прошло с последнего запроса (мс)
        }
    }


    /// Ограничение запросов по времени
    async fn rate_limiter(&mut self) {
                                              
        if let Some(last) = self.last_request_time {                        // проверяем, был ли до этого запрос (!= None)

            let elapsed = last.elapsed();                                   // записываем, сколько прошло времени
            if elapsed < REQUEST_INTERVAL {                                 // прошло ли достаточно времени (1000 мс / 3 < 334 мс) между запросами

                let wait_time = REQUEST_INTERVAL - elapsed;                 // cколько нужно подождать
                sleep(wait_time).await;                                     // ждем
            }
        }
    }


    /// Получение страницы
    pub async fn fetch_kegg(&mut self, url: &str) -> Result<String, String> {

        // Проверяем можно ли отправить запрос
        self.rate_limiter().await;  
        
        // println!("Пытаюсь запросить: {}", url);
        
        // GET-запрос
        let response = self.client
            .get(url)                                                       // создаём запрос (не отправляем)
            .send()                                                         // отправляем
            .await                                                          // ждем ответ
            .map_err(|e| format!("Ошибка при GET-запросе: {}", e))?;        // если ошибка, преобразуем ее в строку

        self.last_request_time = Some(Instant::now());                      // засекаем время с последнего запроса
        
        // Проверяем статус
        if !response.status().is_success() {                                // если Ok, забираем значение,
            return Err(format!("HTTP ошибка: {}", response.status()));      // иначе - Err, выходим из функции с ошибкой
        }
        
        // Читаем тело ответа
        let result = response.text()                                        // хочу получить тело ответа как текст
            .await                                                          // ждём пока данные прочитаются
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))?;      // если ошибка, преобразуем ее в строку
        
        if result.contains("No such data was found") {
            return Err(format!("Ошибка! Для указанного запроса '{}' данные не найдены", url));
        }

        // println!("Успешно получено {} байт", result.len());
        println!();
        Ok(result)
    }
}


/// Тесты
#[cfg(test)]
mod tests {

    use super::*;
    
    #[tokio::test]
    /// Проверка получения данных
    async fn test_fetch_correct_url() {
        let url = "https://www.kegg.jp/entry/ec:5.4.2.2";                   // phosphoglucomutase
        let mut parser = Parser::new();

        let result = parser.fetch_kegg(url).await;
        
        assert!(result.is_ok(), "Тест провален! Не удалось получить страницу HTML!");
        
        let html = result.unwrap();
        
        assert!(!html.is_empty(), "Тест провален! Ответ пришел пустым!");

        assert!(html.contains("EC 5.4.2.2"), 
                "Тест провален! Ответ не сожержит искомого фермента");
        
        println!("Тест пройден! Получено {} байт", html.len());
        println!();
    }

    
    #[tokio::test]
    /// Проверка получения данных с несуществующей страницы
    async fn test_fetch_incorrect_url() {
        let path = "https://www.kegg.jp/entry/ec:123";                      // несуществующая страница
        let mut parser = Parser::new();

        let result = parser.fetch_kegg(path).await;
        
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
        let mut timestamps = Vec::new();                                    // вектор для хранения времени каждого запроса
        
        // Делаем 5 запросов и запоминаем время каждого
        for i in 1..=5 {
            let result = parser.fetch_kegg("https://www.kegg.jp/entry/ec:5.4.2.2").await;
            let after = Instant::now();                                     // время после запроса
            
            assert!(result.is_ok(), "Тест провален! Запрос №{} не удался", i);
            
            // Сохраняем время, когда запрос был выполнен
            timestamps.push(after);

            if i > 1 {
                let interval = timestamps[i-1] - timestamps[i-2];
                println!("Время от предыдущего запроса: {:?}", interval);
                
                assert!(interval >= REQUEST_INTERVAL, 
                        "Тест провален! Запросы идут слишком часто!");
            }
        }

        println!("Тест пройден! Запросы не превышают заданный лимит");
        println!();
    }
}

pub mod compound;