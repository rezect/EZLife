use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn receive_age(
    bot: Bot,
    dialogue: MyDialogue,
    full_name: String,
    msg: Message
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(age)) => {
            bot.send_message(msg.chat.id, "What`s your location?").await?;
            dialogue.update(State::ReceiveLocation { full_name: (full_name), age: (age) }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a number.").await?;
        }
    }

    Ok(())
}

async fn receive_location(
    bot: Bot,
    dialogue: MyDialogue,
    (full_name, age): (String, u8),
    msg: Message,
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<String>()) {
        Some(Ok(location)) => {
            let message = 
                format!("Full name {full_name}\nAge: {age}\nLocation: {location}");
            bot.send_message(msg.chat.id, message).await?;
            dialogue.exit().await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a text message.").await?;
        }
    }

    Ok(())
}

async fn start(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! What's your full name?").await?;
    dialogue.update(State::ReceiveFullName).await?;

    Ok(())
}

async fn receive_full_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<String>()) {
        Some(Ok(text)) => {
            bot.send_message(msg.chat.id, "What`s your age?").await?;
            dialogue.update(State::ReceiveAge { full_name: (text) }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge {
        full_name: String,
    },
    ReceiveLocation {
        full_name: String,
        age: u8,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    pretty_env_logger::init();
    dotenvy::dotenv().expect("505");
    log::info!("Starting my bot...");
    let bot = Bot::from_env();
    let my_id = ChatId(821961326);
    
    bot.send_message(my_id, "Hi").await?;
    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveFullName].endpoint(receive_full_name))
            .branch(dptree::case![State::ReceiveAge { full_name }].endpoint(receive_age))
            .branch(
                dptree::case![State::ReceiveLocation { full_name, age }].endpoint(receive_location),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    Ok(())
}
