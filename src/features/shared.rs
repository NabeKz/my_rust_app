pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(self) -> String {
        self.message
    }
}
