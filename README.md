# [Project Cube Collection](https://wiryls.github.io/cube-collection/)

This is a simple puzzle game based on [Bevy Engine](https://github.com/bevyengine/bevy).

**Try to move cubes to all target points!**

![level-preview](./docs/images/level-preview.gif)

## Tutorial

Use:

- `Arrow Keys` or `W/A/S/D` to move,
- `R` to restart,
- `N` to skip this level,
- `L` to return the previous level,
- `ESC` to reset the game.

## Rules

- You could move all GREEN cubes.
- Move cubes to translucent targets to enter the next level.
- Cubes may absorb each others.
   - Red + Green -> Red + Red
   - Green + Blue -> Green + Green
   - Blue + Red -> Blue + Blue
   - Red + Green + Blue -> Red + Green + Blue
- Cubes with the same kind (except white) merge when hitting each other.

## Known issues

- [Jitters](https://github.com/bevyengine/bevy/issues/4669) happend while moving cubes.
