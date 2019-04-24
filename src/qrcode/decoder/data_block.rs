pub struct DataBlock {
    num_data_codewards: isize,
}

impl DataBlock {
    pub const fn get_num_data_codewards(&self) -> isize {
        return self.num_data_codewards;
    }
}