use crate::effect::StateEffect;
use crate::{Action,Event,EventId,EventKind,Person,PersonId,SimTime};
use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use thiserror::Error;
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct Location{pub name:String}
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct WorldState{pub time:SimTime,pub people:HashMap<PersonId,Person>,pub locations:HashMap<String,Location>,pub events:Vec<Event>,next_event_id:u64}
#[derive(Debug,Error)] pub enum WorldError{#[error("person {0:?} does not exist")] UnknownPerson(PersonId)}
impl WorldState{
 pub fn new()->Self{Self{time:SimTime(0),people:HashMap::new(),locations:HashMap::new(),events:Vec::new(),next_event_id:0}}
 pub fn add_person(&mut self,person:Person){self.people.insert(person.id,person);}
 /// Apply a recorded effect. This is the single replayable mutation boundary.
 pub fn apply_effect(&mut self,effect:&StateEffect){match effect{
  StateEffect::PersonEnteredRoom{..}|StateEffect::PersonLeftRoom{..}=>{},
  StateEffect::MemoryAdded{person,timestamp,description,salience}=>{if let Some(p)=self.people.get_mut(person){p.remember(*timestamp,description.clone(),*salience);}},
  StateEffect::RelationshipChanged{observer,target,trust_delta,conflict_delta,uncertainty_delta}=>{if let Some(p)=self.people.get_mut(observer){if let Some(r)=p.relationships.get_mut(target){r.trust+=trust_delta;r.conflict+=conflict_delta;r.uncertainty+=uncertainty_delta;r.clamp();}}}
 }}
 pub fn emit(&mut self,kind:EventKind,effects:Vec<StateEffect>)->EventId{let id=EventId(self.next_event_id);self.next_event_id+=1;let parent=self.events.last().map(|e|e.id);for effect in &effects{self.apply_effect(effect);}self.events.push(Event{id,timestamp:self.time,causal_parents:parent.into_iter().collect(),kind,effects});id}
 pub fn apply_agent_action(&mut self,actor:PersonId,action:&Action)->Result<(),WorldError>{if !self.people.contains_key(&actor){return Err(WorldError::UnknownPerson(actor));}match action{Action::Say(text)=>{let name=self.people[&actor].identity.name.clone();self.emit(EventKind::Custom{description:format!("{} said: {}",name,text)},Vec::new());},Action::DoNothing=>{}}Ok(())}
}
impl Default for WorldState{fn default()->Self{Self::new()}}
