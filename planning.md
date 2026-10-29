
The goal of this project is to learn the basics of rust by developing a 2d top-down RPG played from the CLI
In this file I document my thought processes, goals, and issues encountered.

## Issue list

| Issue name | Issue description? | Issue done? |
|-----------:|--------------------|-------------|
| Consider basic game elements | What 'classes'/'structs'/'modules'/etc. should my game have? | True |
| Determine functionality of Entity| | True |
| Determine functionality of Location | | True |
| Determine funcitonality of GameState | | False |
| Setup basic game loop | | False |
| Basic read user input | read user inputs to the CLI | True |
| Basic interpreting of user inputs | convert user inputs to UserCommand Enums | True |
| Basic updating of the gamestate | GameState should validate the UserCommand | False |
| Basic rendering of the gamestate | Setup a way to convert the GameState into a ASCII 'art' | False |
 
### Consider basic game elements
A game should have things in it. Considering this will be a 2d top-down RPG, it should have some sort of terrain/**map** through which the player will eventually move.
A map consists out of a number of **locations**.
Each location may contain a zero, one or more **entities**
An entity may be a **wall**, **character**, or **other**. 
From this description (and some either enlightened or foolish anticipatory decisions) we can create the following list of classes (structs?) to add:

1. `GameState {map: Array2D<Locations>}`
2. `Location {x: usize, y: usize, entity: Option<Entity>}`
3. `Entity {entity_type: EntityType}`
4. `EntityType := {WALL, CHARACTER, OTHER}`


### Determine basic functionality of Entity
The most basic functionality an entity can have it showing it exists, i.e. it should be able to 'render' itself. Having an entity's location accessible through the entity may be useful in some cases. However, trying to refer to the 'Location' type in Entity naively results in a "Missing lifetime specifier" error. I'm too inexperienced to know the proper way to deal with this, therefore we'll settle for using a (x,y) tuple. We accept that in order to get an Entity's Location's properties we'll have to build some additional code around that.
Therefore we define Entity as follows:

```rust
enum EntityType {
    Wall,
    PlayerCharacter(BaseCharacter), // Suppose we have BaseCharacter struct which contains the information for a PlayerCharacter
    NPC(BaseCharacter), // Same idea
    Empty,
    Other
}

// an entity's type may be public, but for the other two fields we'll define getters to enable us to modify the get behaviour. (Suppose an entity may lie to the player about it's location?)
struct Entity {
    pub e_type: EntityType, 
    location: (usize, usize),
    display: Option<char>, // Possibly there are entities which should not be displayed
}

impl Entity {
    pub fn render(&self) -> char;
    pub fn get_location(&self) -> (usize, usize);
}
```

### Determine basic functionality of a Location
A location may be moved to or over by entities, therefore it seems sensibly to add a field which optionally contains an entity. Additionally, not all locations may be moved over, therfore we can assign the responsibility for determining a location's traversabiltiy to the location itself.
Therefore we define the location as: 

```rust
struct Location {
    pub x: usize,
    pub y: usize,
    entity: Option<&Entity>,
}

impl Location {
    pub fn is_traversable(&self) -> Result<bool, &str>;
    pub fn get_entity(&self) -> Option<&Entity>;
}
```
However, when a struct contains **a reference** to another struct (here Location contains an Entity), we have to introduce a lifetime specifier.

Suppose we do not, there exists a posibility that the 'Entity', the object which is being referenced, becomes invalid while the location containing the reference remains valid. This would result in a dangling pointer (the reference in the 'entity' field points to freed-up memory).

Therefore, rust requires us to specify that the container struct (Location) may only be valid for as long as the refrenced struct (Entity) remains valid. This is accomplished through the lifetime specifier:

```rust
struct Location<'a> {
    ...
    entity: Option<&'a Entity>
}
```
Alternatively we could allow Location to own the entity object, but this is undesirable if an entity may move between locations. Alternatively still we may come up with a way in which we do not have to contain a reference to Entity, which I will get back to later. 

### Determine basic functionality of a GameState
Instead of declaring a 'map' class, it may be good do declare a 'GameState' class which contains all high-level information, including a map. A GameState should keep track of at least two things. It should contain (a reference to) the map on which the player can move around. And a flag that can break the game loop.
Additionally, it may be useful for the GameState object to know which Characters are currently alive, and it may be good to keep track of what the user is seeing (the UI-state).

```rust
struct GameState {
    pub characters: Vec<BaseCharacter>,
    pub terrain_map: Array2D<char>,
    pub terminate: bool,
    pub ui_state: String,
}

impl GameState {
    pub fn render_map(&self) -> (),
    pub fn initalize_map(max_x, max_y) -> Array2D<char>,
    pub fn terminate(&self) -> bool,
}
```

### A location map and the problem of lifetimes
Instead of storing chars in a map and inferring what type of entity is on a location based on its char, it seems prudent to create a system in which the entity may be accessed directly (if it exists) at a location. Therefore we change 
```rust 
Array2D<char>
``` 
to 
```rust
Array2D<Location>
```

However this introduces a problem, because we defined Location to contain a reference to an Entity, and now because we define GameState to contain a Location, we need to pass a life time specifier all the way from GameState, through Location, to Entity, which is quite a hassle.
Luckily, people before me have found a way to circumvent this issue. Instead of passing an Entity directly to Location, we may define an Collection of EntityIDs and use those to access Entities. One benefit of this is that an numerical is much cheaper to copy/clone than a struct depending on what the struct contains. Because an Entity may in the future become a complexer struct, I choose the EntityID route.

### Tracking EntityIDs
For now we'll define an EntityID as:
```rust
pub struct EntityID(u32)
```
We could use a Vector to store Entities and use an Entity's ID to index it. However, Entities in our game may frequently be added and removed (e.g. enemies, items, neutral mobs), so we should take into account how we deal with insertion and deletion from our collection. One issue with a Vec is that if an entity were to  be 'despawned' and removed from the Vec, we'd leave behind a 'None' at the index it was at. We can not remove the entity entirely because other structs that were pointing at the removed entity would then be pointing at a completely different entity (remember that the EntityIDs serve as indices).
Thankfully again, brighter minds have come up with a 'SlotMap' which support O(1) insertion and deletion and it provides some desirable safety features.

```rust
struct GameState {
    entity_sm = SlotMap<EntityID, Entity> // Mapping of EntityID -> Entity
    ...
}
```

### SlotMap, EntityID, and implementing the 'Key' trait 
https://docs.rs/slotmap/latest/slotmap/

A SlotMap is similar to a HasMap, however, a SlotMap generates and retuns the key when inserting a new value. Additionally there is no hashing involved, it functions ona  direct indexing basis. One major feature of the SlotMap is that it can "reclaim memory" from deleted elements. Ordinarily in a Vec, if a certain item is deleted from index '1', that index is set to 'None', it represents a piece of memory which is now lost. (Alternatively all the items in the vec can be shifted, but that comes with its own problems.)

Because EntityID is not natively supported as a key type for a SlotMap, we have to implement the SlotMap Key trait for EntityID. Fortunately the SlotMap crate comes with an macro that can handle this for us.

```rust
new_key_type! {struct }
```

### Setup basic game loop
In order to play the game the user should able to 

1.  See the current game state
2.  See what the user may input 
3.  Have their inputs be read by the game
4.  See the game respond to their inputs

This list is probably too high-level and non-exhaustive, but it should provide a basis to work from. I've broken down these problems into their own seperate issues

### Basic read user input
The user should be able to input their commands from the CLI which can be easily accomplished using the in-built std::io crate.

