#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: String,
}

// Constructor.
impl Ctx {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> String {
        self.user_id.clone()
    }
}
