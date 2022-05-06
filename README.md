# CS128h Final Project
## Group Name
Name.unwrap_or(group_num.to_string())
## Group Members
Tianyue Cao (tc37)
## Project Introduction
In this project I will make a 2D platformer game with the Amethyst game engine. The platformer game will include features such as movement, enemies, maps, and interactions with the environment. If there is enough time there may also be a multiplayer mode. I choose to work on this project because I like playing and creating games and a 2D platformer game is not hard to make.
## System Overview
- main: Initializes and starts the game.
- states: Defines all game states and their behaviors. This includes the init state (where the envionment is set up), the main menu state, the level selection state, game state, and pause state (in-game menu or the menu after the game concludes).
- components: Contains definition of all user-defined components. The components will be given to appropriate entities when they are initialized.
- entities: Used for initializing entities in the game. Note that they are not the actual entities. Instead, they only store data and encapsulate the process of initializing entities.
- utils: contains useful constants, functions, and structs which can be accessed by the entire project.
## Possible Challenges
This is my first time using Amethyst, so there will be some trouble understanding how it works. However, I've made games with other game engines so I am confident that I can eventually learn how ti use Amethyst.
## References
- [Amethyst](https://github.com/amethyst/amethyst)
