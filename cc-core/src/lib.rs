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
        let game = State::new(&Seed {
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
        });
        assert_eq!(game.progress(), (1, 2));
        assert!(game.current().eq([
            Item {
                id: 0,
                kind: Kind::Blue,
                action: None,
                position: Point::new(0, 0),
                neighborhood: Neighborhood::new(),
            },
            Item {
                id: 1,
                kind: Kind::Green,
                action: None,
                position: Point::new(0, 1),
                neighborhood: Neighborhood::new(),
            },
            Item {
                id: 2,
                kind: Kind::White,
                action: None,
                position: Point::new(1, 0),
                neighborhood: Neighborhood::new(),
            },
        ]
        .into_iter()));

        // STEP 01
        let next = game.link();
        assert_eq!(next.progress(), (1, 2));
        assert!(game.differ(&next).eq([
            Diff {
                id: 0,
                neighborhood: Some(Neighborhood::from([Adjacence::BOTTOM].into_iter())),
                ..Default::default()
            },
            Diff {
                id: 1,
                kind: Some(Kind::Blue),
                neighborhood: Some(Neighborhood::from([Adjacence::TOP].into_iter())),
                ..Default::default()
            },
        ]
        .into_iter()));
        assert!(next.current().eq([
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
        ]
        .into_iter()));

        // STEP 02
        let game = next;
        let next = game.next(Some(Movement::Right));
        assert_eq!(next.progress(), (1, 2));

        let action = Some(Some(Action {
            movement: Movement::Right,
            restriction: Restriction::Stop,
        }));

        assert!(game.differ(&next).eq([
            Diff {
                id: 0,
                action: action.clone(),
                ..Default::default()
            },
            Diff {
                id: 1,
                action: action.clone(),
                ..Default::default()
            },
        ]
        .into_iter()));
        assert!(next.current().eq([
            Item {
                id: 0,
                kind: Kind::Blue,
                action: action.clone().unwrap(),
                position: Point::new(0, 0),
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Item {
                id: 1,
                kind: Kind::Blue,
                action: action.clone().unwrap(),
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
        ]
        .into_iter()));

        // STEP 03
        let game = next;
        let next = game.next(Some(Movement::Down));
        assert_eq!(next.progress(), (2, 2));

        let action = Some(Some(Action {
            movement: Movement::Down,
            restriction: Restriction::Free,
        }));
        assert!(game.differ(&next).eq([
            Diff {
                id: 0,
                action: action.clone(),
                position: Some(Point::new(0, 1)),
                ..Default::default()
            },
            Diff {
                id: 1,
                action: action.clone(),
                position: Some(Point::new(0, 2)),
                ..Default::default()
            },
        ]
        .into_iter()));
        assert!(next.current().eq([
            Item {
                id: 0,
                kind: Kind::Blue,
                action: action.clone().unwrap(),
                position: Point::new(0, 1),
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Item {
                id: 1,
                kind: Kind::Blue,
                action: action.clone().unwrap(),
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
        ]
        .into_iter()));
    }
}
