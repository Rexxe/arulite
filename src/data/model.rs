use std::fmt;

#[derive(Debug)]
pub struct CR {
    pub id: i32,
    pub customer_id: i32,
    pub summary: String,
}

impl fmt::Display for CR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}: {}", self.id, self.customer_id, self.summary)
    }
}
