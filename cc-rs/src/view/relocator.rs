use bevy::math::{Rect, Size, Vec3};

pub struct RelocatorUpdated {
    pub mapper: Mapper
}

#[derive(Default)]
pub struct Relocator {
    // input
    view: Option<Rect<f32>>,
    grid: Option<Size<i32>>,
    // output
    mapper: Mapper
}

impl Relocator {

    pub fn set_view(&mut self, view: Rect<f32>) -> bool {
        let update = self.view.map_or(true, |rect| rect != view);
        if update {
            self.view = Some(view);
            self.update_mapper();
        }
        update
    }

    pub fn set_grid(&mut self, grid: Size<i32>) -> bool {
        let update = self.grid.map_or(true, |rect| rect != grid);
        if update {
            self.grid = Some(grid);
            self.update_mapper();
        }
        update
    }

    pub fn mapping(&self) -> &Mapper {
        &self.mapper
    }

    pub fn is_ready(&self) -> bool {
        self.grid.is_some() && self.view.is_some()
    }

    fn update_mapper(&mut self) {
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

    pub fn scale(&self, x: f32) -> f32 {
        x * self.unit
    }

    pub fn locate3(&self, x: i32, y: i32, z: f32) -> Vec3 {
        let delta = self.unit / 2.;
        Vec3::new(
            self.left + x as f32 * self.unit + delta,
            self.top - y as f32 * self.unit - delta,
            z)
    }
}
