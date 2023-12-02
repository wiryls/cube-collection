![sacrifice](./docs/images/level-preview-sacrifice.gif)

# [Project Cube Collection](https://wiryls.github.io/cube-collection/)

A dead simple puzzle game based on [Bevy Engine 0.11.2](https://github.com/bevyengine/bevy), move cubes to **cover all targets** as shown in the picture below.

![a-moth-to-flame](./docs/images/level-preview-a-moth-to-flame.gif)

Try the online version at my [GitHub Pages](https://wiryls.github.io/cube-collection/)!

## Tutorial

- Move: `Arrow Keys` or `W`/`A`/`S`/`D`.
- Restart: `R`
- Skip current level: `N`.
- Return to the previous level: `L`.
- Reset the game: `ESC`.

## Rules

- You move ALL green cubes.
- Make cubes to cover all target points to enter the next level.
- Cubes may absorb each others.
  - Red + Green -> **Red**
  - Green + Blue -> **Green**
  - Blue + Red -> **Blue**
  - Red + Green + Blue -> Nothing happends
- Cubes with the same kind (except white) merge when hitting each other.

## Known issues

- Color pattern may not friendly to some color blindness.

## About this repository

### Run

1. Clone this repository: `git clone https://github.com/wiryls/cube-collection.git`
2. Compile and run: `cargo run --release cube-collection`

### Add custom levels

Levels are saved as TOML files:

1. Add you custom level files to `.\cube-collection\assets\level\`.
2. Add the file name into `name_list` of `.\cube-collection\assets\level\index.toml`.
