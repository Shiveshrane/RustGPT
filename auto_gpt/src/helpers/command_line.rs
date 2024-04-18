use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]

pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        //Decide on color
        let statement_color: Color = match self {
            Self::AICall => Color::Green,
            Self::UnitTest => Color::Blue,
            Self::Issue => Color::Red,
        };
        //print agent statement in color

        stdout.execute(SetForegroundColor(Color::Yellow)).unwrap();
        print!("Agent:{} ", agent_pos);

        //reset the color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        //reset color again
        stdout.execute(ResetColor).unwrap();
    }
}
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();
    //print question in a specific color
    stdout.execute(SetForegroundColor(Color::Red)).unwrap();
    println!("");
    println!("{}", question);
    //reset color
    stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
    //read user input
    let mut response: String = String::new();
    stdin().read_line(&mut response).unwrap();

    return response.trim().to_string();
}

//get user response to see if the code is safe
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        //print question in color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        println!("WARNING!REVIEW THE CODE BEFORE EXECUTING IT!");
        println!("Reviw the code to ensure it is safe to run");

        //reset color
        stdout.execute(ResetColor).unwrap();
        //present options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1]All good!!");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] Stop the project!");

        stdout.execute(ResetColor).unwrap();

        let mut human_response: String = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read line");

        let human_response: String = human_response.trim().to_lowercase();

        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "stop" | "2" | "n" => return false,
            _ => {
                println!("Invalid response! Please select 1 or 2");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_user_response() {
        let response = get_user_response("What is your name?");
        assert_eq!(response, "test");
    }
    #[test]
    fn test_print_agent_message() {
        PrintCommand::AICall.print_agent_message("AI", "This is a test");
    }
}
