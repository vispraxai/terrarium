use crate::{PersonId, SimTime};
use serde::{Deserialize, Serialize};
/// Observations are the deliberately lossy boundary between latent world truth and an agent.
#[derive(Debug,Clone,Serialize,Deserialize)] pub enum Observation{Text{timestamp:SimTime,text:String}}
#[derive(Debug,Clone,Serialize,Deserialize)] pub enum Action{Say(String),DoNothing}
pub trait Agent{fn observe(&mut self,observation:Observation)->Action;}
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct AgentAction{pub actor:PersonId,pub action:Action}
