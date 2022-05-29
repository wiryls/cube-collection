use bevy::{
    math::{Rect, Vec3, XY},
    prelude::*,
    window::WindowResized,
};
use iyes_loopless::prelude::*;

// GridPlugin adds a GridView resource and a GridUpdated event.
pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GridView>()
            .add_event::<GridUpdated>()
            .add_startup_system(setup_camera)
            .add_system(window_resized.run_on_event::<WindowResized>());
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn window_resized(
    windows: ResMut<Windows>,
    mut view: ResMut<GridView>,
    mut window_resized: EventReader<WindowResized>,
    mut mapper_updated: EventWriter<GridUpdated>,
) {
    if let Some(window) = windows.get_primary() {
        // multiple windows are not supported, we just watch the primary one.
        let id = window.id();

        for event in window_resized.iter().filter(|x| x.id == id).last() {
            // rect is defined by camera's ScalingMode::WindowSize
            let w = event.width / 2.;
            let h = event.height / 2.;
            let r = Rect {
                left: -w,
                right: w,
                top: h,
                bottom: -h,
            };

            if view.set_target(r) && view.available() {
                let mapper = view.mapping().clone();
                mapper_updated.send(GridUpdated { mapper });
            }
        }
    }
}

pub struct GridUpdated {
    pub mapper: GridMapper,
}

#[derive(Default)]
pub struct GridView {
    // input
    source: Option<Rect<i32>>,
    target: Option<Rect<f32>>,
    // output
    mapper: GridMapper,
}

impl GridView {
    pub fn set_source(&mut self, source: Rect<i32>) -> bool {
        let update = match self.source {
            None => true,
            Some(rect) => rect != source,
        };
        if update {
            self.source = Some(source);
            self.remap();
        }
        update
    }

    pub fn set_target(&mut self, target: Rect<f32>) -> bool {
        let update = match self.target {
            None => true,
            Some(rect) => rect != target,
        };
        if update {
            self.target = Some(target);
            self.remap();
        }
        update
    }

    pub fn available(&self) -> bool {
        self.source.is_some() && self.target.is_some()
    }

    pub fn mapping(&self) -> &GridMapper {
        &self.mapper
    }

    fn remap(&mut self) {
        if let (Some(source), Some(target)) = (self.source, self.target) {
            let tw = target.right - target.left;
            let th = target.top - target.bottom;
            let sw = (source.right - source.left) as f32;
            let sh = (source.bottom - source.top) as f32;
            let unit = f32::min(tw / sw, th / sh);

            self.mapper = GridMapper {
                source: XY {
                    x: source.left,
                    y: source.top,
                },
                target: XY {
                    x: target.left + (tw - sw * unit) / 2.,
                    y: target.top - (th - sh * unit) / 2.,
                },
                unit,
            };
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct GridMapper {
    source: XY<i32>,
    target: XY<f32>,
    unit: f32,
}

impl GridMapper {
    pub fn unit(&self) -> f32 {
        self.unit
    }

    pub fn scale<T>(&self, x: T) -> f32
    where
        T: num_traits::AsPrimitive<f32>,
    {
        x.as_() * self.unit
    }

    pub fn locate<T, U, V>(&self, x: T, y: U, z: V) -> Vec3
    where
        T: num_traits::AsPrimitive<i32>,
        U: num_traits::AsPrimitive<i32>,
        V: num_traits::AsPrimitive<f32>,
    {
        let delta = self.unit / 2.;
        Vec3::new(
            self.target.x + delta + (x.as_() - self.source.x) as f32 * self.unit,
            self.target.y - delta - (y.as_() - self.source.y) as f32 * self.unit,
            z.as_(),
        )
    }
}
