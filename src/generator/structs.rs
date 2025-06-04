use std::fmt;

pub enum CreationError {
    NegativeValue,
    Zero,
    EmptyString,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match *self {
            CreationError::NegativeValue => "Value must be greater than zero.",
            CreationError::Zero => "Value must be greater than zero.",
            CreationError::EmptyString => "String must not be empty.",
        };
        f.write_str(desc)
    }
}

fn check_u_value(n: i8) -> Result<(), CreationError> {
    match n {
        x if x < 0 => Err(CreationError::NegativeValue),
        0 => Err(CreationError::Zero),
        _ => Ok(()),
    }
}

pub struct Maze<'a> {
    width: u8,
    height: u8,
    wall: &'a str,
    path: &'a str,
}

impl<'a> Maze<'a> {
    pub fn new(width: i8, height: i8, wall: &'a str, path: &'a str) -> Result<Self, CreationError> {
        check_u_value(width)?;
        check_u_value(height)?;
        if wall.is_empty() || path.is_empty() {
            Err(CreationError::EmptyString)
        } else {
            Ok(Self {
                width: width as u8,
                height: height as u8,
                wall,
                path,
            })
        }
    }
    pub fn get_dimensions(&self) -> (u8, u8) {
        (self.width, self.height)
    }
    pub fn get_wall(&self) -> &'a str {
        self.wall
    }
    pub fn get_path(&self) -> &'a str {
        self.path
    }
}

impl<'a> fmt::Display for Maze<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = format!(
            "({}, {}, {:?}, {:?})",
            self.width, self.height, self.wall, self.path
        );
        f.write_str(&desc)
    }
}
