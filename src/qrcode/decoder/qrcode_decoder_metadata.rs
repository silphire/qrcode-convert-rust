use crate::result_point::ResultPointTrait;

pub struct QRCodeDecoderMetaData {
    mirrored: bool,
}

impl QRCodeDecoderMetaData {
    pub const fn is_mirrored(&self) -> bool {
        return self.mirrored;
    }

    pub fn apply_mirrored_correction(&self, points: &[Box<ResultPointTrait>]) {
        if !self.is_mirrored() || points.len() < 3 {
            return;
        }

        let bottom_left = points[0];
        points[0] = points[2];
        points[2] = bottom_left;
    }
}