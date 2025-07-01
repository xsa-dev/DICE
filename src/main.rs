use teloxide::prelude::*;
use log::info;

mod bot;
mod game;
mod state;

use bot::BotHandler;

#[tokio::main]
async fn main() {
    // Инициализация логгера
    env_logger::init();
    
    info!("Запуск Telegram бота для игры в кубики");

    // Получение токена бота из переменных окружения
    let bot_token = std::env::var("BOT_TOKEN")
        .expect("BOT_TOKEN должен быть установлен в переменных окружения");

    info!("Подключение к Telegram API...");
    let bot = Bot::new(bot_token);
    
    // Создание обработчика бота
    let handler = BotHandler::new();
    
    // Запуск бота
    Dispatcher::builder(bot, handler.schema())
        .dependencies(dptree::deps![])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
