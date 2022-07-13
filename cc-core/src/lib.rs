mod refactor;

mod common;
mod model;
pub mod seed;
pub use self::{
    common::{Adjacence, Neighborhood, Point},
    model::*,
};

#[cfg(test)]
mod tests {
    use crate::seed::*;
    use crate::*;

    #[test]
    fn it_works() {
        /*****
         *BW *
         *G  *
         *x  *
         *****/

        // STEP 00
        let seed = Seed {
            info: Info {
                title: "test".into(),
                author: "test".into(),
            },
            size: Size {
                width: 3,
                height: 3,
            },
            cubes: vec![
                Cube {
                    kind: Kind::Blue,
                    body: vec![Point::new(0, 0)],
                    command: None,
                },
                Cube {
                    kind: Kind::Green,
                    body: vec![Point::new(0, 1)],
                    command: None,
                },
                Cube {
                    kind: Kind::White,
                    body: vec![Point::new(1, 0)],
                    command: None,
                },
            ],
            destnations: vec![Point::new(1, 0), Point::new(0, 2)],
        };
        let mut game = World::new(&seed);
        let stat = [
            Item {
                id: 0,
                kind: Kind::Blue,
                action: None,
                position: Point::new(0, 0),
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Item {
                id: 1,
                kind: Kind::Blue,
                action: None,
                position: Point::new(0, 1),
                neighborhood: Neighborhood::from([Adjacence::TOP].into_iter()),
            },
            Item {
                id: 2,
                kind: Kind::White,
                action: None,
                position: Point::new(1, 0),
                neighborhood: Neighborhood::new(),
            },
        ];
        assert!(game.iter().eq(stat.into_iter()));
        assert_eq!(game.progress(), (1, 2));

        // STEP 01
        let todo = Some(Some(Action {
            movement: Movement::Right,
            restriction: Restriction::Stop,
        }));
        let diff = [
            Diff {
                id: 0,
                action: todo.clone(),
                ..Default::default()
            },
            Diff {
                id: 1,
                action: todo.clone(),
                ..Default::default()
            },
        ];
        assert!(game.input(Some(Movement::Right)).eq(diff.into_iter()));
        assert_eq!(game.progress(), (1, 2));
        assert_eq!(game.commit().count(), 0);
        assert_eq!(game.progress(), (1, 2));

        // STEP 02
        let todo = Some(Some(Action {
            movement: Movement::Down,
            restriction: Restriction::Free,
        }));
        let diff = [
            Diff {
                id: 0,
                action: todo.clone(),
                position: Some(Point::new(0, 1)),
                ..Default::default()
            },
            Diff {
                id: 1,
                action: todo.clone(),
                position: Some(Point::new(0, 2)),
                ..Default::default()
            },
        ];
        assert!(game.input(Some(Movement::Down)).eq(diff.into_iter()));
        let stat = [
            Item {
                id: 0,
                kind: Kind::Blue,
                action: todo.clone().unwrap(),
                position: Point::new(0, 1),
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Item {
                id: 1,
                kind: Kind::Blue,
                action: todo.clone().unwrap(),
                position: Point::new(0, 2),
                neighborhood: Neighborhood::from([Adjacence::TOP].into_iter()),
            },
            Item {
                id: 2,
                kind: Kind::White,
                action: None,
                position: Point::new(1, 0),
                neighborhood: Neighborhood::new(),
            },
        ];
        assert!(game.iter().eq(stat.into_iter()));
        assert_eq!(game.progress(), (2, 2));
        assert_eq!(game.commit().count(), 0);
        assert_eq!(game.progress(), (2, 2));
    }
}
