pub enum Sex {
    Male,
    Female,
    Other,
}
impl From<Sex> for String {
    fn from(val: Sex) -> Self {
        match val {
            Sex::Male => "Male".into(),
            Sex::Female => "Female".into(),
            Sex::Other => "Other".into(),
        }
    }
}

pub struct Person {
    pub name: String,
    pub age: u8,
    pub sex: Sex,
}
