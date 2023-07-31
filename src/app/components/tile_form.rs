use std::fmt::Display;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileForm {
    Empty,
    Two,
    Four,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
    OneTwentyEight,
    TwoFiftySix,
    FiveTwelve,
    OneThousandTwentyFour,
    TwoThousandFortyEight,
}

impl Display for TileForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileForm::Empty => write!(f, ""),
            TileForm::Two => write!(f, "2"),
            TileForm::Four => write!(f, "4"),
            TileForm::Eight => write!(f, "8"),
            TileForm::Sixteen => write!(f, "16"),
            TileForm::ThirtyTwo => write!(f, "32"),
            TileForm::SixtyFour => write!(f, "64"),
            TileForm::OneTwentyEight => write!(f, "128"),
            TileForm::TwoFiftySix => write!(f, "256"),
            TileForm::FiveTwelve => write!(f, "512"),
            TileForm::OneThousandTwentyFour => write!(f, "1024"),
            TileForm::TwoThousandFortyEight => write!(f, "2048"),
        }
    }
}

impl TryFrom<u32> for TileForm {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TileForm::Empty),
            2 => Ok(TileForm::Two),
            4 => Ok(TileForm::Four),
            8 => Ok(TileForm::Eight),
            16 => Ok(TileForm::Sixteen),
            32 => Ok(TileForm::ThirtyTwo),
            64 => Ok(TileForm::SixtyFour),
            128 => Ok(TileForm::OneTwentyEight),
            256 => Ok(TileForm::TwoFiftySix),
            512 => Ok(TileForm::FiveTwelve),
            1024 => Ok(TileForm::OneThousandTwentyFour),
            2048 => Ok(TileForm::TwoThousandFortyEight),
            _ => Err(()),
        }
    }
}

impl From<TileForm> for u32 {
    fn from(item: TileForm) -> Self {
        match item {
            TileForm::Empty => 0,
            TileForm::Two => 2,
            TileForm::Four => 4,
            TileForm::Eight => 8,
            TileForm::Sixteen => 16,
            TileForm::ThirtyTwo => 32,
            TileForm::SixtyFour => 64,
            TileForm::OneTwentyEight => 128,
            TileForm::TwoFiftySix => 256,
            TileForm::FiveTwelve => 512,
            TileForm::OneThousandTwentyFour => 1024,
            TileForm::TwoThousandFortyEight => 2048,
        }
    }
}

impl TileForm {
    pub fn get_class(&self) -> &'static str {
        match self {
            TileForm::Empty => "empty",
            TileForm::Two => "tile-2",
            TileForm::Four => "tile-4",
            TileForm::Eight => "tile-8",
            TileForm::Sixteen => "tile-16",
            TileForm::ThirtyTwo => "tile-32",
            TileForm::SixtyFour => "tile-64",
            TileForm::OneTwentyEight => "tile-128",
            TileForm::TwoFiftySix => "tile-256",
            TileForm::FiveTwelve => "tile-512",
            TileForm::OneThousandTwentyFour => "tile-1024",
            TileForm::TwoThousandFortyEight => "tile-2048",
        }
    }
}
