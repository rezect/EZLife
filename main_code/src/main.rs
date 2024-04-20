mod add_functions;
mod enums;
mod comands_handlers;
mod handler_functions;

use teloxide::{
    prelude::*,
    dispatching::{dialogue::{self, InMemStorage}, UpdateHandler},
};
use comands_handlers::*;
use handler_functions::*;
use add_functions::{add_str_to_file, sleep_next_day, one_hour_ok, two_hour_ok};
use enums::*;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("dotenv error");
    pretty_env_logger::init();
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

fn shema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help_handler))
        .branch(case![Command::Restart].endpoint(restart_handler))
        .branch(case![Command::AddEmotions].endpoint(add_emotions_handler));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Start].endpoint(start))
        .branch(case![State::Waiting].endpoint(waiting_handler))
        .branch(case![State::OneHourOk].endpoint(one_hour_ok))
        .branch(case![State::TwoHourOk].endpoint(two_hour_ok))
        .branch(case![State::ReceiveAgree].endpoint(receive_agree))
        .branch(case![State::ReceiveEnergy].endpoint(receive_energy))
        .branch(case![State::ReceiveEmotions { energy }].endpoint(receive_emotions))
        .branch(case![State::ReceiveReflection { energy, emotions }].endpoint(receive_reflection))
        .branch(case![State::IsAllOk { energy, emotions, reflection }].endpoint(is_all_ok));
        
    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        
}
