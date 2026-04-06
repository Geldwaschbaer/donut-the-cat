pub mod icon;
pub mod node;

use crate::map::{
    icon::MapIcon,
    node::{MapNode, MapNodeBuilder},
};
use async_from::{AsyncFrom, async_trait};
use macroquad::prelude::*;
use serde::Deserialize;

pub struct Map {
    map_nodes: Vec<MapNode>,
    background: Texture2D,
    icon_endboss: Texture2D,
    icon_boss: Texture2D,
    icon_enemy: Texture2D,
    icon_mystery: Texture2D,
    icon_shop: Texture2D,
    icon_start: Texture2D,
}

impl Map {
    pub async fn new() -> Map {
        let builder = {
            let serialized = load_string("assets/layout/level-1.json")
                .await
                .expect("file exists");
            serde_json::from_str(&serialized).expect("could not parse event")
        };

        Map::async_from(builder).await
    }

    pub fn get_map_nodes(&self) -> &Vec<MapNode> {
        &self.map_nodes
    }

    pub fn get_map_nodes_mut(&mut self) -> &mut Vec<MapNode> {
        &mut self.map_nodes
    }

    pub fn get_map_node(&self, map_node: usize) -> &MapNode {
        &self.map_nodes.get(map_node).expect("map_node exists")
    }

    pub fn get_background(&self) -> &Texture2D {
        &self.background
    }

    pub fn get_icon(&self, icon: &MapIcon) -> &Texture2D {
        match icon {
            MapIcon::Boss => &self.icon_boss,
            MapIcon::Endboss => &self.icon_endboss,
            MapIcon::Enemy => &self.icon_enemy,
            MapIcon::Mystery => &self.icon_mystery,
            MapIcon::Shop => &self.icon_shop,
            MapIcon::Start => &self.icon_start,
        }
    }
}

#[derive(Deserialize)]
pub struct MapBuilder(Vec<MapNodeBuilder>);

#[async_trait]
impl AsyncFrom<MapBuilder> for Map {
    async fn async_from(builder: MapBuilder) -> Map {
        let mut map_nodes = Vec::new();
        for builder in builder.0.into_iter() {
            map_nodes.push(MapNode::async_from(builder).await);
        }
        let background = load_texture("assets/backgrounds/map-bg.png")
            .await
            .expect("map background exists");

        let icon_boss = load_texture("assets/icon/boss.png")
            .await
            .expect("map background exists");
        let icon_endboss = load_texture("assets/icon/endboss.png")
            .await
            .expect("map background exists");
        let icon_enemy = load_texture("assets/icon/enemy.png")
            .await
            .expect("map background exists");
        let icon_mystery = load_texture("assets/icon/mystery.png")
            .await
            .expect("map background exists");
        let icon_shop = load_texture("assets/icon/shop.png")
            .await
            .expect("map background exists");
        let icon_start = load_texture("assets/icon/start.png")
            .await
            .expect("map background exists");
        Map {
            map_nodes,
            background,
            icon_boss,
            icon_endboss,
            icon_enemy,
            icon_mystery,
            icon_shop,
            icon_start,
        }
    }
}
