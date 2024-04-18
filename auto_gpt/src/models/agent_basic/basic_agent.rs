use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::general::llm::Message;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    // AgentState is an enum with the following variants for the state of the agent
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
pub struct BasicAgent {
    // BasicAgent is a struct with the following fields: objective, position, state, and memory
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl BasicTraits for BasicAgent {
    //Implementations for basic agent. We set state to Discovery as default and memory as empty
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::from([]),
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }
}
