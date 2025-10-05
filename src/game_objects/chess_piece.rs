pub struct Chess_piece<const N: usize> {
    pub name: String,
    pub sprite_sector: [u8; 2],
    pub movement: [[u8; 2]; N],
    pub repeating_movement: bool,
    pub only_front_movement: bool,
    pub position: [u8; 2],
    pub color: String,
    pub is_king: bool,
}

impl<const N: usize> Chess_piece<N> {
    fn get_move_options (&self) -> Vec<u8> {
        return vec![0, 1, 2];
    }
}
