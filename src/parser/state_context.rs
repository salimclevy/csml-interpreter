use lazy_static::*;
use std::collections::*;
use std::sync::*;
use std::thread::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref CONTEXT: Mutex<HashMap<ThreadId, StateContext>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExitCondition {
    Goto,
    Error,
    Break,
    Hold,
}

#[derive(Debug, Copy, Clone)]
pub enum ExecutionState {
    Normal,
    Loop,
}

#[derive(Debug)]
pub struct StateContext {
    state: Vec<ExecutionState>,
    rip: usize,
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for StateContext {
    fn default() -> Self {
        Self {
            state: Vec::default(),
            rip: 0,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl StateContext {
    pub fn clear_rip() {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        if let Some(state_context) = hashmap.get_mut(&thread_id) {
            state_context.rip = 0;
        }
    }

    pub fn inc_rip() {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| StateContext::default());

        if let Some(state_context) = hashmap.get_mut(&thread_id) {
            state_context.rip += 1;
        }
    }

    pub fn get_rip() -> usize {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| StateContext::default());

        if let Some(state_context) = hashmap.get(&thread_id) {
            state_context.rip
        } else {
            unreachable!();
        }
    }
}

impl StateContext {
    pub fn clear_state() {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| StateContext::default());

        if let Some(state_context) = hashmap.get_mut(&thread_id) {
            state_context.state.clear();
        }
    }

    pub fn set_state(state: ExecutionState) {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| StateContext::default());

        if let Some(state_context) = hashmap.get_mut(&thread_id) {
            match state {
                ExecutionState::Loop => {
                    state_context.state.push(state);
                }
                ExecutionState::Normal => {
                    state_context.state.pop();
                }
            }
        }
    }

    pub fn get_state() -> ExecutionState {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| StateContext::default());

        if let Some(state_context) = hashmap.get(&thread_id) {
            if state_context.state.is_empty() {
                return ExecutionState::Normal;
            }
        }

        ExecutionState::Loop
    }
}
