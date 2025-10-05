pub struct Chess_piece<const N: usize> {
    pub name: String,
    pub sprite_sector: [u8; 2],
    pub movement: [[u8; 2]; N],
    pub repeating_movement: bool,
    pub only_front_movement: bool,
}
