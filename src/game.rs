use rand::Rng;
use crate::state::{EvenOddChoice, HighLowChoice, GuessOneChoice};

/// Структура для управления игровой логикой
pub struct DiceGame;

impl DiceGame {
    /// Бросок кубика - возвращает число от 1 до 6
    pub fn roll_dice() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=6)
    }

    /// Проверка результата для игры "Четное/Нечетное"
    pub fn check_even_odd(dice_result: u8, user_choice: EvenOddChoice) -> bool {
        let is_even = dice_result % 2 == 0;
        match user_choice {
            EvenOddChoice::Even => is_even,
            EvenOddChoice::Odd => !is_even,
        }
    }

    /// Проверка результата для игры "Больше/Меньше 3.5"
    pub fn check_high_low(dice_result: u8, user_choice: HighLowChoice) -> bool {
        match user_choice {
            HighLowChoice::High => dice_result >= 4,
            HighLowChoice::Low => dice_result <= 3,
        }
    }

    /// Проверка результата для игры "Точное число"
    pub fn check_exact_number(dice_result: u8, user_guess: u8) -> bool {
        dice_result == user_guess
    }

    /// Проверка результата для игры "Угадать единицу"
    pub fn check_guess_one(dice_result: u8, user_choice: GuessOneChoice) -> bool {
        let is_one = dice_result == 1;
        match user_choice {
            GuessOneChoice::Yes => is_one,
            GuessOneChoice::No => !is_one,
        }
    }

    /// Получение эмодзи кубика по числу
    pub fn dice_emoji(number: u8) -> &'static str {
        match number {
            1 => "⚀",
            2 => "⚁",
            3 => "⚂",
            4 => "⚃",
            5 => "⚄",
            6 => "⚅",
            _ => "🎲",
        }
    }

    /// Получение сообщения о выигрыше
    pub fn win_message() -> &'static str {
        let messages = [
            "🎉 Поздравляю! Вы угадали!",
            "🎊 Отлично! Правильный ответ!",
            "✨ Великолепно! Вы победили!",
            "🏆 Браво! Точное попадание!",
            "🎯 Превосходно! Вы угадали!",
        ];
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..messages.len());
        messages[index]
    }

    /// Получение ободряющего сообщения при проигрыше
    pub fn lose_message() -> &'static str {
        let messages = [
            "😔 Не угадали, но не расстраивайтесь!",
            "🎲 В этот раз не повезло, попробуйте еще!",
            "💪 Ничего страшного, удача улыбнется в следующий раз!",
            "🌟 Не переживайте, у вас все получится!",
            "🎮 Попытка не пытка, играем еще!",
        ];
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..messages.len());
        messages[index]
    }
}
