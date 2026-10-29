pub mod entity {
    use slotmap::{new_key_type, Key};
    use std::fmt;
    use crate::character::BaseCharacter;

    new_key_type! {
        pub struct EntityID;
    }

    impl fmt::Display for EntityID {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "EntityID[{}]", self.data().as_ffi())
        }
    }

    pub enum EntityType {
        Wall,
        PlayerCharacter(BaseCharacter),
        NPC(BaseCharacter),
        Empty,
        Other,
    }

    pub struct Entity {
        pub id: Option<EntityID>,
        pub name: String,
        pub e_type: EntityType,
        location: Option<(usize, usize)>,
        display: Option<char>,
    }

    impl Entity {
        pub fn new(
            id: Option<EntityID>,
            name: Option<String>,
            e_type: Option<EntityType>,
            location: Option<(usize, usize)>,
            display: Option<char>,
        ) -> Entity {
            Entity {
                id: id,
                name: name.unwrap_or("unnamed_entity".to_string()),
                e_type: e_type.unwrap_or(EntityType::Other),
                location: location,
                display: display,
            }
        }

        /// Get an Entity's char representation
        pub fn render(&self) -> char {
            match self.e_type {
                EntityType::PlayerCharacter(_) | EntityType::NPC(_) => {
                    self.display.clone().unwrap_or('%')
                }
                EntityType::Empty => '_',
                EntityType::Wall => '#',
                EntityType::Other => '?',
            }
        }

        /// Get an entity's location as (x,y) coordinates
        pub fn get_location(&self) -> Option<(usize, usize)> {
            self.location.clone()
        }

        pub fn set_location(&mut self, x: usize, y: usize) {
            self.location = Some((x, y));
        }
    }
}
