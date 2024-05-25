mod add_functions;
mod enums;
mod comands_handlers;
mod handler_functions;
mod callback_handlers;
mod shemas {
    pub mod notion_shemas;
    pub mod ya_gpt_shemas;
}

use std::{
    env,
    fs::File,
    fs::OpenOptions,
    path::{
        Path,
        PathBuf
    },
    io::Read,
    io::Write,
    time::Duration,
};
use teloxide::{
    dispatching::{
        dialogue::{
            self,
            serializer::Json,
            ErasedStorage, SqliteStorage, Storage,
        }, UpdateFilterExt, UpdateHandler
    }, dptree::endpoint, payloads::SendMessageSetters, prelude::*, types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ParseMode
    }, utils::command::BotCommands
};
use reqwest::{
    header,
    Client,
    Response
};
use chrono::{
    Datelike,
    Local,
    Utc,
    Timelike,
};
use shemas::{
    notion_shemas::*,
    ya_gpt_shemas::*,
};
use tokio::time::sleep;
use serde_json::json;
use comands_handlers::*;
use handler_functions::*;
use add_functions::*;
use callback_handlers::*;
use enums::*;
use dotenvy::dotenv;


type MyDialogue = Dialogue<State, ErasedStorage<State>>;
type MyStorage = std::sync::Arc<ErasedStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("dotenv error");
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();
    let my_id = ChatId(821961326);
    bot.send_message(my_id, "I ||started||\\.\\.\\.")
        .parse_mode(ParseMode::MarkdownV2)
        .await.unwrap();

    check_or_create_file("db.sqlite").await;

    let storage: MyStorage = SqliteStorage::open("db.sqlite", Json).await.unwrap().erase();

    Dispatcher::builder(bot, shema())
    .dependencies(dptree::deps![storage])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

fn shema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help_command))
        .branch(case![Command::Day].endpoint(new_day_command))
        .branch(case![Command::Sleep].endpoint(sleep_command))
        .branch(case![Command::Notion].endpoint(notion_command))
        .branch(case![Command::Note].endpoint(note_command));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Start].endpoint(start_handler))
        .branch(case![State::GetNotionCode].endpoint(get_notion_code_handler))
        .branch(case![State::GetDBID].endpoint(get_db_id_handler))
        .branch(case![State::ReceiveEmotions { energy }].endpoint(receive_emotions_handler))
        .branch(case![State::ReceiveReflection { energy, emotions }].endpoint(receive_reflection_handler))
        .branch(case![State::ReceiveRate { energy, emotions, reflection }].endpoint(receive_rate_handler))
        .branch(case![State::IsAllOk { energy, emotions, reflection, rate }].endpoint(is_all_ok_handler))
        .branch(case![State::Waiting].endpoint(waiting_handler))
        .branch(case![State::EnergyError].endpoint(energy_error_handler))
        .branch(case![State::NoteHelper].endpoint(note_helper));

    let callback_handler = Update::filter_callback_query()
        .branch(case![State::EnergyError].endpoint(callback_handler))
        .branch(endpoint(callback_handler_not_correct_state));

    dialogue::enter::<Update, ErasedStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_handler)

}
