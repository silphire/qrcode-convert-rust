use crate::common::bitmatrix::BitMatrix;
use crate::common::perspective_transform::PerspectiveTransform;

pub trait GridSampler {
    fn sample_grid(&self, image: BitMatrix, dimension_x: isize, dimension_y: isize, transform: PerspectiveTransform);

    fn check_and_nudge_points(image: &BitMatrix, points: &[f64]) {
        let max_offset = points.len() - 1;

        let nudged = true;
        for offset in (0..max_offset).step_by(2) {
            if !nudged {
                break;
            }
            let x = points[offset] as isize;
            let y = points[offset + 1] as isize;
            if x < -1 || x > image.width || y < -1 || y > image.height {
                // throw NotFound
                unimplemented!();
            }
            nudged = false;
            if x == -1 {
                points[offset] = 0.0;
                nudged = true;
            } else if x == image.width {
                points[offset] = (image.width - 1) as f64;
                nudged = true;
            }
            if y == -1 {
                points[offset + 1] = 0.0;
                nudged = true;
            } else if y == image.height {
                points[offset + 1] = (image.height - 1) as f64;
                nudged = true;
            }
        }

        nudged = true;
        for offset in (0..max_offset).rev().step_by(2) {
            let x = points[offset] as isize;
            let y = points[offset + 1] as isize;
            if x < -1 || x > image.width || y < -1 || y > image.height {
                // throw NotFound
                unimplemented!();
            }
            nudged = false;
            if x == -1 {
                points[offset] = 0.0;
                nudged = true;
            } else if x == image.width {
                points[offset] = (image.width - 1) as f64;
                nudged = true;
            }
            if y == -1 {
                points[offset + 1] = 0.0;
                nudged = true;
            } else if y == image.height {
                points[offset + 1] = (image.height - 1) as f64;
                nudged = true;
            }
        }
    }
}