
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

### Setup basic game loop
In order to play the game the user should able to 

1.  See the current game state
2.  See what the user may input 
3.  Have their inputs be read by the game
4.  See the game respond to their inputs

This list is probably too high-level and non-exhaustive, but it should provide a basis to work from. I've broken down these problems into their own seperate issues

### Basic read user input
The user should be able to input their commands from the CLI which can be easily accomplished using the in-built std::io crate.

