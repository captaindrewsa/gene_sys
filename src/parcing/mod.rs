use reqwest;

pub struct Parser{
    base_url: String,
}

impl Parser{
    
    /// Новый экземпляр Parser
    pub fn new() -> Self {
        Self {
            base_url: String::from("https://www.genome.jp"),
        }
    }

    /// Загрузка HTML по пути
    pub async fn fetch_html(&self, path: &str) -> Result<String, String> {
        
        // Собираем полный URL, например  = https://www.genome.jp + /entry/ec:5.4.2.2
        let url = format!("{}{}", self.base_url, path); 
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
            return Err(format!("Ошибка: Для указанного запроса '{}' данные не найдены", path));
        }

        // println!("Успешно загружено {} байт", html.len());
        Ok(html)
    }
}