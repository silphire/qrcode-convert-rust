pub struct Version {
    version_number: isize,
    alignment_pattern_centers: Vec<isize>,
}

impl Version {
    pub const fn get_version_number(&self) -> isize {
        return self.version_number;
    }

    pub const fn get_alignment_pattern_centers(&self) -> Vec<isize> {
        return self.alignment_pattern_centers;
    }

    pub fn get_provisional_version_for_dimension(dimension: isize) -> &'static Version {
        unimplemented!();
    }
    
    pub const fn get_dimension_for_version(&self) -> isize {
        return 17 + 4 * self.get_version_number();
    }
}