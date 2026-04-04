use crate::{
    entity::{Entity, EntityBuilder},
    event::{Event, EventBuilder},
};
use async_from::{AsyncFrom, async_trait};
use serde::Deserialize;

#[derive(Clone)]
pub struct Enemy {
    entity: Entity,
    on_death: Vec<Event>,
}

impl Enemy {
    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    pub fn get_on_death(&self) -> &Vec<Event> {
        &self.on_death
    }
}

#[derive(Deserialize)]
pub struct EnemyBuilder {
    entity: EntityBuilder,
    on_death: Vec<EventBuilder>,
}

#[async_trait]
impl AsyncFrom<EnemyBuilder> for Enemy {
    async fn async_from(value: EnemyBuilder) -> Self {
        let mut on_death = Vec::new();
        for event_builder in value.on_death {
            on_death.push(Event::async_from(event_builder).await)
        }
        Enemy {
            entity: Entity::async_from(value.entity).await,
            on_death,
        }
    }
}
