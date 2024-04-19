use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    dispatching::{dialogue::{self, InMemStorage}, UpdateHandler},
};
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::time::Duration;
use chrono::{Timelike, Local};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().expect("505");
    log::info!("Starting bot...");
    let bot = Bot::from_env();
    let my_id = ChatId(821961326);
    bot.send_message(my_id, "I`ve been started...").await.unwrap();

    Dispatcher::builder(bot, shema())
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Restart the dialogue")]
    Restart,
    #[command(description = "Add your emotions")]
    AddEmotions,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveAgree,
    ReceiveEnergy,
    ReceiveEmotions {
        energy: String,
    },
    ReceiveReflection {
        energy: String,
        emotions: String,
    },
    OneHourOk,
    TwoHourOk,
}

fn shema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help_handler))
        .branch(case![Command::Restart].endpoint(restart_handler))
        .branch(case![Command::AddEmotions].endpoint(add_emotions_handler));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::OneHourOk].endpoint(one_hour_ok))
        .branch(case![State::TwoHourOk].endpoint(two_hour_ok))
        .branch(case![State::Start].endpoint(start))
        .branch(case![State::ReceiveAgree].endpoint(receive_agree))
        .branch(case![State::ReceiveEnergy].endpoint(receive_energy))
        .branch(case![State::ReceiveEmotions { energy }].endpoint(receive_emotions))
        .branch(case![State::ReceiveReflection { energy, emotions }].endpoint(receive_reflection));
        
    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        
}

// Функции-обработчики команд
async fn help_handler(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn restart_handler(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Привет, готов поговорить о прошедшем дне? ;)").await?;
    dialogue.update(State::ReceiveAgree).await?;
    Ok(())
}

async fn add_emotions_handler(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Какие эмоции ты хочешь добавить?").await?;
    // Реализация добавления эмоций в файлик
    dialogue.update(State::ReceiveAgree).await?;
    Ok(())
}

// Функции-обработчики состояний
async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Привет, готов поговорить о прошедшем дне? ;)").await?;
    dialogue.update(State::ReceiveAgree).await?;
    Ok(())
}

async fn receive_agree(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Да") => {
            bot.send_message(msg.chat.id, "Хорошо, начнем с энергии").await?;
            bot.send_message(msg.chat.id, "Какая она была сегодня?").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
        Some("Нет") => {
            bot.send_message(msg.chat.id, "Хорошо, а через час удобно будет поговорить?").await?;
            dialogue.update(State::OneHourOk).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши только Да или Нет, зайка ;)").await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
    }
    Ok(())
}

async fn receive_energy(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Низкая") => {
            bot.send_message(msg.chat.id, "Ничего страшного, это нормально бро").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            // Реализация записи energy в файл
            dialogue.update(State::ReceiveEmotions { energy: String::from("Низкая энергия") }).await?;
        }
        Some("Средняя") => {
            bot.send_message(msg.chat.id, "Главное во всем держать золотую середину ;)").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            // Реализация записи energy в файл
            dialogue.update(State::ReceiveEmotions { energy: String::from("Средняя энергия") }).await?;
        }
        Some("Высокая") => {
            bot.send_message(msg.chat.id, "Сегодня позитивненький день, получается что-ли :)").await?;
            bot.send_message(msg.chat.id, "Теперь расскажи о своих чувствах за сегодня").await?;
            // Реализация записи energy в файл
            dialogue.update(State::ReceiveEmotions { energy: String::from("Высокая энергия") }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши одну из трех категорий: низкая, средняя, высокая").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
    }
    Ok(())
}

async fn receive_emotions(
    bot: Bot,
    dialogue: MyDialogue,
    energy: String,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Теперь можешь поделиться впечатлениями о дне").await?;
            // Реализация записи emotions в файл
            dialogue.update(State::ReceiveReflection { energy, emotions: (String::from(text)) }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши что-нибудь, котик ;)").await?;
            dialogue.update(State::ReceiveEmotions { energy }).await?;
        }
    }
    Ok(())
}

async fn receive_reflection(
    bot: Bot,
    dialogue: MyDialogue,
    (energy, emotions): (String, String),
    msg: Message
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, format!("Отлично, день закончен, поздравляю!\nВот краткий итог:")).await?;
            bot.send_message(msg.chat.id, format!("Energy: {}\nEmotions: {}\nReflection: {}", energy, emotions, text)).await?;
            let user_name = msg.from().unwrap().username.to_owned().unwrap();
            bot.send_message(ChatId(821961326), format!("User: {}\nEnergy: {}\nEmotions: {}\nReflection: {}", user_name, energy, emotions, text)).await?;
            // Реализация записи reflection в файл
            sleep_next_day().await;
            bot.send_message(dialogue.chat_id(), "Привет, готов поговорить о прошедшем дне? ;)").await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши что-нибудь, котик ;)").await?;
            dialogue.update(State::ReceiveReflection { energy, emotions }).await?;
        }
    }
    Ok(())
}

// Функции для обработки ожидания пользователя (левая ветка в схеме)
async fn one_hour_ok(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Да") => {
            bot.send_message(msg.chat.id, "Хорошо, напишу через часик").await?;
            sleep_n_hours(1).await;
            bot.send_message(msg.chat.id, "Час прошел, давай начнем с энергии").await?;
            bot.send_message(msg.chat.id, "Какая она была сегодня?").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
        Some("Нет") => {
            bot.send_message(msg.chat.id, "Хорошо, а через два часа удобно будет поговорить?").await?;
            dialogue.update(State::TwoHourOk).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши только Да или Нет, зайка ;)").await?;
            dialogue.update(State::OneHourOk).await?;
        }
    }
    Ok(())
}

async fn two_hour_ok(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some("Да") => {
            bot.send_message(msg.chat.id, "Хорошо, напишу через два часика").await?;
            sleep_n_hours(2).await;
            bot.send_message(msg.chat.id, "Время пришлоо, давай начнем с энергии").await?;
            bot.send_message(msg.chat.id, "Какая она была сегодня?").await?;
            dialogue.update(State::ReceiveEnergy).await?;
        }
        Some("Нет") => {
            sleep_next_day().await;
            bot.send_message(dialogue.chat_id(), "Привет, готов поговорить о прошедшем дне? ;)").await?;
            dialogue.update(State::ReceiveAgree).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Напиши только Да или Нет, солнышко ;)").await?;
            dialogue.update(State::TwoHourOk).await?;
        }
    }
    Ok(())
}

// Доп функции, не являющиеся обработчиками
async fn sleep_n_hours(n: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(n * 3600)).await;
}

async fn sleep_next_day() {
    let since_midnight = {
        let now = Local::now().time();
        let seconds = now.num_seconds_from_midnight() as u64;
        seconds
    };
    let due: Duration;
    if since_midnight > 75600 {
        due = std::time::Duration::from_secs(86400 - since_midnight + 75600);
    } else {
        due = std::time::Duration::from_secs(75600 - since_midnight);
    }
    tokio::time::sleep(due).await;
}