pub mod model;
pub mod seed;
pub mod state;

pub use self::state::*;

#[cfg(test)]
mod tests {
    use crate::model::*;
    use crate::seed::*;
    use crate::*;

    #[test]
    fn it_works() {
        /*****
         *GW *
         *B  *
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
                    kind: Kind::Green,
                    body: vec![Point::new(0, 0)],
                    command: None,
                },
                Cube {
                    kind: Kind::Blue,
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
        let mut game = State::new(&seed);
        let stat = [
            Item {
                id: 0,
                kind: Kind::Green,
                position: Point::new(0, 0),
                movement: None,
                constraint: Constraint::Free,
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Item {
                id: 1,
                kind: Kind::Green,
                position: Point::new(0, 1),
                movement: None,
                constraint: Constraint::Free,
                neighborhood: Neighborhood::from([Adjacence::TOP].into_iter()),
            },
            Item {
                id: 2,
                kind: Kind::White,
                position: Point::new(1, 0),
                movement: None,
                constraint: Constraint::Free,
                neighborhood: Neighborhood::new(),
            },
        ];
        assert_eq!(game.iter().collect::<Vec<_>>(), stat);
        assert_eq!(game.progress(), (1, 2));

        // STEP 01
        let diff = [
            Diff {
                id: 0,
                movement: Some(Some(Movement::Right)),
                constraint: Some(Constraint::Stop),
                ..Default::default()
            },
            Diff {
                id: 1,
                movement: Some(Some(Movement::Right)),
                constraint: Some(Constraint::Stop),
                ..Default::default()
            },
        ];
        assert_eq!(game.input(Some(Movement::Right)).collect::<Vec<_>>(), diff);
        let diff = [
            Diff {
                id: 0,
                movement: Some(None),
                constraint: Some(Constraint::Free),
                ..Default::default()
            },
            Diff {
                id: 1,
                movement: Some(None),
                constraint: Some(Constraint::Free),
                ..Default::default()
            },
        ];
        assert_eq!(game.commit().collect::<Vec<_>>(), diff);
        assert_eq!(game.progress(), (1, 2));

        // STEP 02
        let diff = [
            Diff {
                id: 0,
                movement: Some(Some(Movement::Down)),
                ..Default::default()
            },
            Diff {
                id: 1,
                movement: Some(Some(Movement::Down)),
                ..Default::default()
            },
        ];
        assert_eq!(game.input(Some(Movement::Down)).collect::<Vec<_>>(), diff);
        let diff = [
            Diff {
                id: 0,
                movement: Some(None),
                position: Some(Point::new(0, 1)),
                ..Default::default()
            },
            Diff {
                id: 1,
                movement: Some(None),
                position: Some(Point::new(0, 2)),
                ..Default::default()
            },
        ];
        assert_eq!(game.commit().collect::<Vec<_>>(), diff);
        let stat = [
            Item {
                id: 0,
                kind: Kind::Green,
                position: Point::new(0, 1),
                movement: None,
                constraint: Constraint::Free,
                neighborhood: Neighborhood::from([Adjacence::BOTTOM].into_iter()),
            },
            Item {
                id: 1,
                kind: Kind::Green,
                position: Point::new(0, 2),
                movement: None,
                constraint: Constraint::Free,
                neighborhood: Neighborhood::from([Adjacence::TOP].into_iter()),
            },
            Item {
                id: 2,
                kind: Kind::White,
                position: Point::new(1, 0),
                movement: None,
                constraint: Constraint::Free,
                neighborhood: Neighborhood::new(),
            },
        ];
        assert_eq!(game.iter().collect::<Vec<_>>(), stat);
        assert_eq!(game.progress(), (2, 2));
    }
}
