mod add_functions;
mod enums;
mod comands_handlers;
mod handler_functions;
mod yagpt_apis;
mod notion_apis;
mod shemas {
    pub mod notion_shemas;
    pub mod ya_gpt_shemas;
}

use std::{
    env,
    fs::File,
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
            serializer::{Bincode, Json},
            ErasedStorage, RedisStorage, SqliteStorage, Storage,
        },
        UpdateHandler,
    },
    types::{
        InputFile,
        ParseMode
    },
    prelude::*,
    utils::command::BotCommands,
};
use reqwest::{
    header,
    Client,
    Response
};
use chrono::{
    Datelike,
    Local,
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
use enums::*;
use std::fs::OpenOptions;
use dotenvy::dotenv;
use notion_apis::*;


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

    let storage: MyStorage = if std::env::var("DB_REMEMBER_REDIS").is_ok() {
        RedisStorage::open("redis://127.0.0.1:6379", Bincode).await.unwrap().erase()
    } else {
        SqliteStorage::open("db.sqlite", Json).await.unwrap().erase()
    };

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
        .branch(case![Command::Help].endpoint(help_handler))
        .branch(case![Command::New].endpoint(restart_handler))
        .branch(case![Command::Restart].endpoint(restart_handler))
        .branch(case![Command::SendUserData].endpoint(send_user_data))
        .branch(case![Command::DeleteAllData].endpoint(delete_all_data))
        .branch(case![Command::Sleep].endpoint(sleep_handler))
        .branch(case![Command::Notion].endpoint(notion_command));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Start].endpoint(start))
        .branch(case![State::ReceiveEnergy].endpoint(receive_energy))
        .branch(case![State::ReceiveEmotions { energy }].endpoint(receive_emotions))
        .branch(case![State::ReceiveReflection { energy, emotions }].endpoint(receive_reflection))
        .branch(case![State::ReceiveRate { energy, emotions, reflection }].endpoint(receive_rate))
        .branch(case![State::IsAllOk { energy, emotions, reflection, rate }].endpoint(is_all_ok))
        .branch(case![State::DeleteAllUserData].endpoint(delete_handler))
        .branch(case![State::Waiting].endpoint(waiting_handler))
        .branch(case![State::ReceiveToNotion].endpoint(receive_to_notion))
        .branch(case![State::GetNotionCode].endpoint(write_down_notion_token))
        .branch(case![State::GetDBID].endpoint(get_db_id));
        
    dialogue::enter::<Update, ErasedStorage<State>, State, _>()
        .branch(message_handler)
    
}
