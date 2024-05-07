use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Restart the dialogue")]
    Restart,
    #[command(description = "Start new session")]
    New,
    #[command(description = "Add your reflection")]
    AddReflection,
    #[command(description = "Get your data")]
    SendUserData,
    #[command(description = "Delete all your data")]
    DeleteAllData,
    #[command(description = "Bot go to sleep")]
    Sleep,
    #[command(description = "Change your DB id")]
    ChangeDBId,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveToNotion,
    ReceiveNotionInfo,
    ReceiveAgree,
    ReceiveEnergy,
    ReceiveEmotions {
        energy: String,
    },
    ReceiveReflection {
        energy: String,
        emotions: String,
    },
    ReceiveRate {
        energy: String,
        emotions: String,
        reflection: String,
    },
    IsAllOk {
        energy: String,
        emotions: String,
        reflection: String,
        rate: u32
    },
    Waiting,
    DeleteAllUserData,
    OneHourOk,
    AddNewReflection,
}