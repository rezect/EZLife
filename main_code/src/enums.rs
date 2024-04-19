use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
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
    IsAllOk {
        energy: String,
        emotions: String,
        reflection: String,
    },
    Waiting,
    OneHourOk,
    TwoHourOk,
}