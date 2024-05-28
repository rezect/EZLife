use crate::BotCommands;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display all commands")]
    Help,
    #[command(description = "Record your impressions of the day")]
    Day,
    #[command(description = "Make fast note")]
    Note,
    #[command(description = "Change the Notion data")]
    Notion,
    #[command(description = "Bot go to sleep")]
    Sleep,
    #[command(description = "Check your if your notion is valid")]
    Checker,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    GetNotionCode,
    GetDBID,
    EnergyError,
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
    NoteHelper,
}