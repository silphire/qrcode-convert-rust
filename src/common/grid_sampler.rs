use crate::error::Error;
use crate::common::bitmatrix::BitMatrix;
use crate::common::perspective_transform::PerspectiveTransform;
use crate::common::default_grid_sampler::DefaultGridSampler;

static grid_sampler: &GridSampler = &DefaultGridSampler::new();

pub trait GridSampler: Sync {
    fn get_instance() -> &'static GridSampler where Self: Sized {
        return grid_sampler;
    }

    fn sample_grid(&self, image: &BitMatrix, dimension_x: isize, dimension_y: isize, transform: &PerspectiveTransform) -> Result<BitMatrix, Error>;

    fn check_and_nudge_points(image: &BitMatrix, points: &mut [f64]) -> Result<(), Error> where Self: Sized {
        let max_offset = points.len() - 1;

        let mut nudged = true;
        for offset in (0..max_offset).step_by(2) {
            if !nudged {
                break;
            }
            let x = points[offset] as isize;
            let y = points[offset + 1] as isize;
            if x < -1 || x > image.get_width() || y < -1 || y > image.get_height() {
                return Err(Error::NotFoundError);
            }
            nudged = false;
            if x == -1 {
                points[offset] = 0.0;
                nudged = true;
            } else if x == image.get_width() {
                points[offset] = (image.get_width() - 1) as f64;
                nudged = true;
            }
            if y == -1 {
                points[offset + 1] = 0.0;
                nudged = true;
            } else if y == image.get_height() {
                points[offset + 1] = (image.get_height() - 1) as f64;
                nudged = true;
            }
        }

        nudged = true;
        for offset in (0..max_offset).rev().step_by(2) {
            let x = points[offset] as isize;
            let y = points[offset + 1] as isize;
            if x < -1 || x > image.get_width() || y < -1 || y > image.get_height() {
                return Err(Error::NotFoundError);
            }
            nudged = false;
            if x == -1 {
                points[offset] = 0.0;
                nudged = true;
            } else if x == image.get_width() {
                points[offset] = (image.get_width() - 1) as f64;
                nudged = true;
            }
            if y == -1 {
                points[offset + 1] = 0.0;
                nudged = true;
            } else if y == image.get_height() {
                points[offset + 1] = (image.get_height() - 1) as f64;
                nudged = true;
            }
        }

        return Ok(());
    }
}