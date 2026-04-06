use crate::{event::Event, map::icon::MapIcon};
use async_from::{AsyncFrom, async_trait};
use macroquad::prelude::*;
use serde::Deserialize;

pub struct MapNode {
    event: Event,
    position: Vec2,
    neighbours: Vec<usize>,
    visited: bool,
    icon: MapIcon,
}

impl MapNode {
    pub fn get_event(&self) -> &Event {
        &self.event
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_neighbours(&self) -> &Vec<usize> {
        &self.neighbours
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn mark_visited(&mut self) {
        self.visited = true;
    }

    pub fn get_icon(&self) -> &MapIcon {
        &self.icon
    }
}

#[derive(Deserialize)]
pub struct MapNodeBuilder {
    event_options: Vec<String>,
    position: (f32, f32),
    neighbours: Vec<usize>,
    icon: MapIcon,
}

#[async_trait]
impl AsyncFrom<MapNodeBuilder> for MapNode {
    async fn async_from(builder: MapNodeBuilder) -> MapNode {
        let event = {
            let len = builder.event_options.len();
            if len > 0 {
                let element = rand::gen_range(0, builder.event_options.len());
                let file = builder
                    .event_options
                    .get(element)
                    .expect("event option exists");
                let serialized = load_string(file).await.expect("file exists");
                Event::async_from(
                    serde_json::from_str(&serialized)
                        .expect(&format!("could not parse event from file '{}'", file)),
                )
                .await
            } else {
                Event::ReturnToMap
            }
        };
        MapNode {
            event,
            position: Vec2::new(builder.position.0, builder.position.1),
            neighbours: builder.neighbours,
            visited: false,
            icon: builder.icon,
        }
    }
}
