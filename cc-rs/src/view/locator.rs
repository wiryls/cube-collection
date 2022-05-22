use bevy::math::{Rect, Size, Vec3};

pub struct LocatorUpdated {
    pub mapper: Mapper
}

#[derive(Default)]
pub struct GridLocator {
    // input
    view: Option<Rect<f32>>,
    grid: Option<Size<i32>>,
    // output
    mapper: Mapper
}

impl GridLocator {
    pub fn set_view(&mut self, view: Rect<f32>) -> bool {
        let update = match self.view {
            None => true,
            Some(rect) => rect != view
        };
        if update {
            self.view = Some(view);
            self.remap();
        }
        update
    }

    pub fn set_grid(&mut self, grid: Size<i32>) -> bool {
        let update = match self.grid {
            None => true,
            Some(size) => size != grid
        };
        if update {
            self.grid = Some(grid);
            self.remap();
        }
        update
    }

    pub fn available(&self) -> bool {
        self.grid.is_some() && self.view.is_some()
    }

    pub fn mapping(&self) -> &Mapper {
        &self.mapper
    }

    fn remap(&mut self) {
        if let (Some(view), Some(grid)) = (self.view, self.grid) {
            let vw = view.right - view.left;
            let vh = view.top - view.bottom;
            let gw = grid.width as f32;
            let gh = grid.height as f32;
            let unit = f32::min(vw / gw, vh / gh);
    
            self.mapper = Mapper::new(
                view.top - (vh - gh * unit) / 2.,
                view.left + (vw - gw * unit) / 2., 
                unit);
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Mapper {
    top: f32,
    left: f32,
    unit: f32,
}

impl Mapper {
    pub fn new(top: f32, left: f32, unit: f32) -> Self {
        Self{top, left, unit}
    }

    pub fn scale<T: num_traits::AsPrimitive<f32>>(&self, x: T) -> f32 {
        x.as_() * self.unit
    }

    pub fn locate<T, U, V>(&self, x: T, y: U, z: V) -> Vec3 where
        T: num_traits::AsPrimitive<f32>,
        U: num_traits::AsPrimitive<f32>,
        V: num_traits::AsPrimitive<f32>
    {
        let delta = self.unit / 2.;
        Vec3::new(
            self.left + delta + x.as_() * self.unit,
            self.top - delta - y.as_() * self.unit,
            z.as_())
    }
}
