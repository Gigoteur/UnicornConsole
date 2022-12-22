mod input_context;

pub struct Contexts {
    pub(crate) input_context: input_context::InputContext,
}

impl Contexts {
    pub fn new(num_players: usize) -> Self {
        Self {
            input_context: input_context::InputContext::new(num_players),
        }
    }
}