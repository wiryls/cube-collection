# [Project Cube Collection](https://wiryls.github.io/cube-collection/)

A dead simple puzzle game based on [Bevy Engine](https://github.com/bevyengine/bevy), move cubes to **cover all targets** as shown in the picture below.

![level-preview](./docs/images/level-preview.gif)

Try the online version at my [GitHub Pages](https://wiryls.github.io/cube-collection/)!

## Tutorial

Use:

- `Arrow Keys` or `W/A/S/D` to move,
- `R` to restart,
- `N` to skip current level,
- `L` to return to the previous level,
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
- Color pattern may not friendly to some color blindness.
