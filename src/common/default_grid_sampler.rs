use crate::common::bitmatrix::BitMatrix;
use crate::common::grid_sampler::GridSampler;
use crate::common::perspective_transform::PerspectiveTransform;
use crate::error::Error;

pub struct DefaultGridSampler {
}

impl GridSampler for DefaultGridSampler {
    fn sample_grid(&self, image: &BitMatrix, dimension_x: isize, dimension_y: isize, transform: &PerspectiveTransform) -> Result<BitMatrix, Error> {
        if dimension_x <= 0 || dimension_y <= 0 {
            return Err(Error::NotFoundError);
        }

        let bits = BitMatrix::new(dimension_x, dimension_y);
        let mut points: Vec<f64> = Vec::<f64>::with_capacity(2 * dimension_x as usize);
        for y in 0..dimension_y {
            let max = points.len();
            let i_value = y as f64 + 0.5;
            for x in (0..max).step_by(2) {
                points[x] = (x / 2) as f64 + 0.5;
                points[x + 1] = i_value;
            }
            transform.transform_points(&mut points);
            // TODO GridSampler::check_and_nudge_points(image, points.as_mut_slice())?;

            unimplemented!();
        }

        return Ok(bits);
    }
}

impl DefaultGridSampler {
    pub const fn new() -> DefaultGridSampler where Self: Sized {
        return DefaultGridSampler {};
    }
}