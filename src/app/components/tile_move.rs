#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileMove {
    Still,
    Left3,
    Right3,
    Down3,
    Up3,
}

impl TileMove {
    pub fn get_class(&self) -> &'static str {
        match self {
            TileMove::Still => "",
            TileMove::Right3 => "slide-3-right",
            TileMove::Down3 => "slide-3-down",
            TileMove::Left3 => "slide-3-left",
            TileMove::Up3 => "slide-3-up",
        }
    }

    pub fn get_direction(&self) -> &'static str {
        match self {
            TileMove::Still => "",
            TileMove::Right3 => "right",
            TileMove::Down3 => "down",
            TileMove::Left3 => "left",
            TileMove::Up3 => "up",
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            TileMove::Still => 0,
            TileMove::Right3 => 3,
            TileMove::Down3 => 3,
            TileMove::Left3 => 3,
            TileMove::Up3 => 3,
        }
    }
}

impl TryFrom<&str> for TileMove {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "still" => Ok(TileMove::Still),
            "slide-3-right" => Ok(TileMove::Right3),
            "slide-3-down" => Ok(TileMove::Down3),
            "slide-3-left" => Ok(TileMove::Left3),
            "slide-3-up" => Ok(TileMove::Up3),
            _ => Err(()),
        }
    }
}
