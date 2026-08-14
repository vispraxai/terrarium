use terrarium_core::agent::{Agent, Observation};
use terrarium_core::{Action};

/// Minimal deterministic agent used to prove the Terrarium loop.
/// Replace this with the Vixir adapter later.
pub struct EchoAgent;

impl Agent for EchoAgent {
    fn observe(&mut self, observation: Observation) -> Action {
        match observation {
            Observation::Text { text, .. } => Action::Say(format!("I observed: {text}")),
        }
    }
}
