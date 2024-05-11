use crate::BotCommands;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Restart the dialogue")]
    Restart,
    #[command(description = "Start new session")]
    New,
    #[command(description = "Get your data")]
    SendUserData,
    #[command(description = "Delete all your data")]
    DeleteAllData,
    #[command(description = "Bot go to sleep")]
    Sleep,
    #[command(description = "Make Notion integration")]
    Notion,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveToNotion,
    GetNotionCode,
    GetDBID,
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
}