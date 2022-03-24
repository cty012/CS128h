# CS128h Final Project
## Group Members
Tianyue Cao (tc37)
## Project Introduction
In this project I will make a 2D platformer game with the Amethyst game engine. The platformer game will include features such as movement, enemies, maps, and interactions with the environment. If there is enough time there may also be a multiplayer mode. I choose to work on this project because I like playing and creating games and a 2D platformer game is not hard to make.
## System Overview
- game server: The part that runs the game and process the logic (such as the physics, player status, etc.). Not really a server unless I finish the multiplayer part. Will be run on a separate thread.
- game client: Get the current state from the game server and render the ui.
- game menu: Controls the scene switching and eveything outside of the actual game.
- data: saves temperary data which can be accessed anywhere except for the game server thread.
## Possible Challenges
This is my first time using Amethyst, so there will be some trouble understanding how it works. However, I've made games with other game engines so I am confident that I can eventually learn how ti use Amethyst.
## References
- [Amethyst](https://github.com/amethyst/amethyst)
