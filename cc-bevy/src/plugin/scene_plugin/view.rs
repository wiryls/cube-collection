use bevy::prelude::*;
use bevy::window::WindowResized;
use cc_core::cube::Point;
use iyes_loopless::prelude::*;

pub fn setup(appx: &mut App, stage: impl StageLabel) {
    appx.init_resource::<GridView>()
        .add_event::<ViewUpdated>()
        .add_startup_system(setup_camera)
        .add_system_to_stage(stage, update_gridview.run_on_event::<WindowResized>());
}

pub struct ViewUpdated {
    pub mapper: ViewMapper,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn update_gridview(
    windows: ResMut<Windows>,
    mut view: ResMut<GridView>,
    mut window_resized: EventReader<WindowResized>,
    mut mapper_updated: EventWriter<ViewUpdated>,
) {
    if let Some(window) = windows.get_primary() {
        // multiple windows are not supported, we just watch the primary one.
        let id = window.id();

        for event in window_resized.iter().filter(|x| x.id == id).last() {
            // rect is defined by camera's ScalingMode::WindowSize
            let w = event.width / 2.;
            let h = event.height / 2.;
            let r = UiRect {
                left: -w,
                right: w,
                top: h,
                bottom: -h,
            };

            if view.set_target(r) && view.available() {
                let mapper = view.mapping().clone();
                mapper_updated.send(ViewUpdated { mapper });
            }
        }
    }
}

#[derive(Default)]
pub struct GridView {
    // input
    source: Option<UiRect<i32>>,
    target: Option<UiRect<f32>>,
    // output
    mapper: ViewMapper,
}

impl GridView {
    pub fn set_source(&mut self, source: UiRect<i32>) -> bool {
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

    pub fn set_target(&mut self, target: UiRect<f32>) -> bool {
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

    pub fn mapping(&self) -> &ViewMapper {
        &self.mapper
    }

    fn remap(&mut self) {
        if let (Some(source), Some(target)) = (self.source, self.target) {
            let tw = target.right - target.left;
            let th = target.top - target.bottom;
            let sw = (source.right - source.left) as f32;
            let sh = (source.bottom - source.top) as f32;
            let unit = f32::min(tw / sw, th / sh);

            self.mapper = ViewMapper {
                source: (source.left, source.top),
                target: Vec2 {
                    x: target.left + (tw - sw * unit) / 2.,
                    y: target.top - (th - sh * unit) / 2.,
                },
                unit,
            };
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct ViewMapper {
    source: (i32, i32),
    target: Vec2,
    unit: f32,
}

impl ViewMapper {
    pub const fn unit(&self) -> f32 {
        self.unit
    }

    pub fn flip(&self, o: &impl Mappable) -> Vec2 {
        o.delta(self.source).into()
    }

    pub fn scale(&self, o: &impl Mappable) -> Vec2 {
        o.scale(self.unit).into()
    }

    pub fn locate(&self, o: &impl Mappable) -> Vec2 {
        let mut output = Vec2::from(o.delta(self.source));
        output *= self.unit;
        output += self.target;
        output
    }
}

pub trait Mappable {
    fn scale(&self, factor: f32) -> (f32, f32);
    fn delta(&self, source: (i32, i32)) -> (f32, f32);
}

impl Mappable for (i32, i32) {
    fn scale(&self, factor: f32) -> (f32, f32) {
        (self.0 as f32 * factor, self.1 as f32 * -factor)
    }

    fn delta(&self, source: (i32, i32)) -> (f32, f32) {
        ((self.0 - source.0) as f32, (source.1 - self.1) as f32)
    }
}

impl Mappable for (f32, f32) {
    fn scale(&self, factor: f32) -> (f32, f32) {
        (self.0 * factor, self.1 * -factor)
    }

    fn delta(&self, source: (i32, i32)) -> (f32, f32) {
        (self.0 - source.0 as f32, source.1 as f32 - self.1)
    }
}

impl Mappable for (usize, usize) {
    fn scale(&self, factor: f32) -> (f32, f32) {
        (self.0 as f32 * factor, self.1 as f32 * -factor)
    }

    fn delta(&self, source: (i32, i32)) -> (f32, f32) {
        (
            (self.0 as i64 - source.0 as i64) as f32,
            (source.1 as i64 - self.1 as i64) as f32,
        )
    }
}

impl Mappable for Point {
    fn scale(&self, factor: f32) -> (f32, f32) {
        (self.x as f32 * factor, self.y as f32 * -factor)
    }

    fn delta(&self, source: (i32, i32)) -> (f32, f32) {
        ((self.x - source.0) as f32, (source.1 - self.y) as f32)
    }
}
