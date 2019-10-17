# [Project Cube Collection](https://wiryls.github.io/ProjectCubeCollection/)

This is a simple HTML5 puzzle game based on [Egret](https://www.egret.com/).

**Try to move cubes to all target points, and then press `enter` to the next level!**

![Level - Palace](./screenshots/cc-level-palace.gif?raw=true "Merge cubes to  U-shaped and move them to end points!")

## Tutorial

Use:

- `Arrow Keys` to move,
- `Enter` next level,
- `R` as restart,
- `N` to skip this level,
- `ESC` to reset the game.

## Rules

- Players control all blue cubes.
- Move cubes to endpoints to enter the next level.
- Cubes with higher priority will absorb lower. (Priority: Red > Blue > Green)
- Cubes with the same priority need to hit to merge.

## Known issues

- As Egret suggests me using 44100Hz 96kbps mp3 files, it takes some time to load all music.
- Frame rate becomes weird on `Chrome beta (> 72.0)` or newer. Not sure if it's a bug of the browser or engine.
