pub struct Minutes(i32);
pub struct Hours(i32);
pub struct Days(i32);

impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Hours {
        // Implement the minute to hour conversion here
    }
}

// TODO: implement from hours to days

// TODO: implement from minutes to days

// TODO: implement from days to hours
