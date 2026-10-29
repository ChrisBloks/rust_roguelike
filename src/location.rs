pub mod location {

    use slotmap::SlotMap;

use crate::entity::entity::{Entity, EntityID, EntityType};

    pub struct Location {
        pub x: usize,
        pub y: usize,
        entity: Option<EntityID>, // Should change this to a HashMap to support multiple entities ata single location
    }

    impl Location {

        /// Check if this location contains an entity may move into this location
        pub fn is_traversable(&self, entity_sm: &SlotMap<EntityID, Entity>) -> Result<bool, &str> {
            // If this location is empty, we can move into it
            if self.entity.is_none() {
                return Ok(true)
            }

            // Retrieve the entity type from the Entity SlotMap
            let entity: Option<&Entity> = entity_sm.get(self.entity.unwrap());
            if let Some(e) = entity {
                match e.e_type {
                    EntityType::Wall => Ok(false),
                    _ => Ok(true),
                }
            } else {
                Err("Could not determine the traversability of {self.x}, {self.y}")
            }
        }

        pub fn get_entity(&self) -> Option<EntityID> {
            self.entity
        }

        pub fn create_wall(x: usize, y: usize, wall_entity_id: EntityID) -> Location {
            Location {
                x,
                y,
                entity: Some(wall_entity_id),
            }
        }
    }
}
