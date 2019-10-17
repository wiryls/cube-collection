# [Project Cube Collection](https://wiryls.github.io/ProjectCubeCollection/)

A simple HTML5 puzzle game (still work in progress) based on [Egret](https://www.egret.com/).

Try to move cubes to all target points, and then press `enter` to next level!

![Level - Palace](./screenshots/cc-level-palace.gif?raw=true "Merge cubes to  U-shaped and move them to end points!")

## Tutorial

Use:

- `Arrow Keys` to move,
- `Enter` next level,
- `R` as restart,
- `N` to skip this level,
- `ESC` to reset the game.

## Rules

- Players controls all blue cubes.
- Move any cubes to end points to enter next level.
- Cubes with higher priority will absorb lower. (Priority: Red > Blue > Green)
- Cubes with same priority need to hit to be merged.

## Known issues

- As Egret suggests me using 44100Hz 96kbps mp3 files, it takes some time to load all music.
- Frame rate becomes weird on `Chrome beta (> 72.0)` or later. Not sure if it's a bug of the browser or engine.
