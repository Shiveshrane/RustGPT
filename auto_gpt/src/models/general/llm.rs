use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Clone)]

pub struct Message {
    pub content: String,
    pub role: String,
}

//Chat Completion
#[derive(Debug, Serialize, Clone)]

pub struct ChatCompletion {
    //chat completion is a struct with the following fields: model, messages, and temperature and used to store the chat completion
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Debug, Deserialize)]
pub struct APIMessage {
    //APIMessage is a struct with the following fields: pub content and is used for deserializing the message from the API
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct APIChoice {
    //APIChoice is a struct with the following fields: pub message and is used for deserializing the choice from the API
    pub message: APIMessage,
}

#[derive(Debug, Deserialize)]
pub struct APIResponse {
    //APIResponse is a struct with the following fields: pub choices and is used for deserializing the response from the API
    pub choices: Vec<APIChoice>,
}

//Api choice means the choice of the response, like foe example if we have 3 choices, then we will have 3 APIChoices and we store it in a vector for better handling
