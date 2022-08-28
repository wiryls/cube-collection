use super::Location;
use bevy::prelude::*;
use num_traits::AsPrimitive;

#[derive(Default)]
pub struct GridView {
    // input
    source: Option<UiRect<i32>>,
    target: Option<UiRect<f32>>,
    // output
    mapper: GridMapper,
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

    pub fn mapping(&self) -> &GridMapper {
        &self.mapper
    }

    fn remap(&mut self) {
        if let (Some(source), Some(target)) = (self.source, self.target) {
            let tw = target.right - target.left;
            let th = target.top - target.bottom;
            let sw = (source.right - source.left) as f32;
            let sh = (source.bottom - source.top) as f32;
            let scale = f32::min(tw / sw, th / sh);

            self.mapper = GridMapper {
                source: IVec2 {
                    x: source.left,
                    y: source.top,
                },
                target: Vec2 {
                    x: target.left + (tw - sw * scale) / 2.,
                    y: target.top - (th - sh * scale) / 2.,
                },
                scale,
            };
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct GridMapper {
    source: IVec2,
    target: Vec2,
    scale: f32,
}

impl GridMapper {
    pub fn scale<T>(&self, o: T) -> f32
    where
        T: AsPrimitive<f32>,
    {
        self.scale * o.as_()
    }

    #[allow(unused)]
    pub fn flip<T, U>(&self, o: &T) -> Vec2
    where
        T: Location<U>,
        U: AsPrimitive<f32>,
    {
        Vec2::new(o.x().as_(), -o.y().as_())
    }

    pub fn absolute<T, U>(&self, o: &T) -> Vec2
    where
        T: Location<U>,
        U: AsPrimitive<i32>,
    {
        let delta = self.scale * 0.5;
        let x = self.target.x + delta + (o.x().as_() - self.source.x) as f32 * self.scale;
        let y = self.target.y - delta - (o.y().as_() - self.source.y) as f32 * self.scale;
        Vec2::new(x, y)
    }
}
