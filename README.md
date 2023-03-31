![sacrifice](./docs/images/level-preview-sacrifice.gif)

# [Project Cube Collection](https://wiryls.github.io/cube-collection/)

A dead simple puzzle game based on [Bevy Engine 0.10.0](https://github.com/bevyengine/bevy), move cubes to **cover all targets** as shown in the picture below.

![a-moth-to-flame](./docs/images/level-preview-a-moth-to-flame.gif)

Try the online version at my [GitHub Pages](https://wiryls.github.io/cube-collection/)!

## Tutorial

- Move: `Arrow Keys` or `W`/`A`/`S`/`D`.
- Restart: `R`
- Skip current level: `N`.
- Return to the previous level: `L`.
- Reset the game: `ESC`.

## Rules

- You could move all GREEN cubes.
- Move cubes to translucent targets to enter the next level.
- Cubes may absorb each others.
  - Red + Green -> Red + **Red**
  - Green + Blue -> Green + **Green**
  - Blue + Red -> Blue + **Blue**
  - Red + Green + Blue -> Red + Green + Blue
- Cubes with the same kind (except white) merge when hitting each other.

## Known issues

- [Jitters](https://github.com/bevyengine/bevy/issues/4669) happend while moving cubes.
- Color pattern may not friendly to some color blindness.

## About this repository

### Run

1. Clone this repository: `git clone https://github.com/wiryls/cube-collection.git`
2. Compile and run: `cargo run --release cube-collection`

### Add custom level

Levels are saved as TOML files:

1. Add you custom level file to `.\cube-collection\assets\level\`.
2. Add the file name into `name_list` of `.\cube-collection\assets\level\index.toml`.
