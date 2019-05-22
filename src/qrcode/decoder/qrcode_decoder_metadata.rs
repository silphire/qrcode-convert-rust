use crate::result_point::ResultPointTrait;

pub struct QRCodeDecoderMetaData {
    mirrored: bool,
}

impl QRCodeDecoderMetaData {
    pub const fn is_mirrored(&self) -> bool {
        return self.mirrored;
    }

    pub fn apply_mirrored_correction(&self, points: &mut [Box<ResultPointTrait>]) {
        if !self.is_mirrored() || points.len() < 3 {
            return;
        }

        points.swap(0, 2);
    }
}