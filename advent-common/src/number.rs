use crate::AdventError;

pub fn num_digits(mut n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut cnt = 0;
    while n > 0 {
        n /= 10;
        cnt += 1;
    }

    cnt
}

#[derive(Debug, Clone, Copy)]
pub struct XYZCoord {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl XYZCoord {
    pub fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    pub fn dist_sq(&self, p: &XYZCoord) -> u64 {
        let dx = u64::abs_diff(self.x, p.x).pow(2);
        let dy = u64::abs_diff(self.y, p.y).pow(2);
        let dz = u64::abs_diff(self.z, p.z).pow(2);

        dx + dy + dz
    }
}

impl TryFrom<&str> for XYZCoord {
    type Error = AdventError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .filter(|x| !x.is_empty())
            .ok_or_else(|| AdventError::InputParseError("triple: missing first component".into()))?
            .parse::<u64>()?;
        let y = parts
            .next()
            .filter(|y| !y.is_empty())
            .ok_or_else(|| AdventError::InputParseError("triple: missing second component".into()))?
            .parse::<u64>()?;
        let z = parts
            .next()
            .filter(|z| !z.is_empty())
            .ok_or_else(|| AdventError::InputParseError("triple: missing third component".into()))?
            .parse::<u64>()?;

        if parts.next().is_some() {
            return Err(AdventError::InputParseError(
                "triple: expected exactly three comma-separated numbers".into(),
            ));
        }
        Ok(XYZCoord { x, y, z })
    }
}

impl TryFrom<String> for XYZCoord {
    type Error = AdventError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}
