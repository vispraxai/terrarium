use crate::effect::StateEffect;
use crate::event::EventKind;
use crate::{Duration,Person,PersonId,Run,SimTime,WorldState};
use serde::{Deserialize,Serialize};
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct Simulation{pub world:WorldState,pub run:Run}
impl Simulation{
 pub fn new()->Self{let world=WorldState::new();let mut run=Run::new("main");run.capture(&world);Self{world,run}}
 pub fn add_person(&mut self,person:Person){self.world.add_person(person);}
 pub fn advance(&mut self,duration:Duration){self.world.time+=duration;}
 pub fn promise_made(&mut self,from:PersonId,to:PersonId,content:impl Into<String>){self.emit(EventKind::PromiseMade{from,to,content:content.into()},Vec::new());}
 /// Psychological consequences are represented explicitly as effects. This is
 /// longer than mutating Bob directly, but now replay can reproduce it exactly.
 pub fn promise_broken(&mut self,from:PersonId,to:PersonId,content:impl Into<String>){let content=content.into();let description=format!("{} broke a promise: {}",from.0,content);let effects=vec![StateEffect::MemoryAdded{person:to,timestamp:self.world.time,description,salience:0.8},StateEffect::RelationshipChanged{observer:to,target:from,trust_delta:-0.15,conflict_delta:0.10,uncertainty_delta:0.10}];self.emit(EventKind::PromiseBroken{from,to,content},effects);}
 fn emit(&mut self,kind:EventKind,effects:Vec<StateEffect>){let id=self.world.emit(kind,effects);let event=self.world.events.last().cloned().expect("emit just appended event");debug_assert_eq!(event.id,id);self.run.record_event(event);}
 pub fn time(&self)->SimTime{self.world.time}
 /// A clone is still useful for cheap experimentation, while Run now makes
 /// the fork point explicit for the future Observatory.
 pub fn branch(&self,branch_id:impl Into<String>,event_cursor:usize)->Self{let mut child=self.clone();child.run=self.run.branch(branch_id,event_cursor);child}
}
impl Default for Simulation{fn default()->Self{Self::new()}}
