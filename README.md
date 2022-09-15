# [Project Cube Collection](https://wiryls.github.io/cc/)

This is a simple puzzle game based on [Bevy Engine](https://github.com/bevyengine/bevy).

**Try to move cubes to all target points!**

## Tutorial

Use:

- `Arrow Keys` to move,
- `R` as restart,
- `N` to skip this level,
- `ESC` to reset the game.

## Rules

- You control all green cubes.
- Move cubes to endpoints to enter the next level.
- Cubes with higher priority will absorb lower.
   - Red > Green
   - Green > Blue
   - Blue > Red
- Cubes with the same color merged when hitted.

## Known issues

- [Jitters](https://github.com/bevyengine/bevy/issues/4669) happend while moving cubes.
