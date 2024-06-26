#[macro_export]
macro_rules! get_function_string {
    ($func:ident) => {
        stringify!($func)
    };
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;


use helpers::command_line::get_user_response;
use models::agents_manager::managing_agent::ManagingAgent;
#[tokio::main]
async fn main() {
    let user_request: String = get_user_response("What website you'd like to ask the AI?");
    let mut  managing_agent:ManagingAgent=ManagingAgent::new(user_request)
    .await
    .expect("error to create Managing Agent");
managing_agent.execute_project().await;
    dbg!(managing_agent);
}
