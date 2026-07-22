pub struct Movie {
    id: i32,
    title: String,
    description: String,
    author: String,
    published_date: String,
}

impl Movie {
    /// Constructor
    pub fn new(
        title: &str,
        description: &str,
        author: &str,
        published_date: &str,
    ) -> Self {
        Self {
            id: 0,
            title: title.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            published_date: published_date.to_string(),
        }
    }

    // ===== Getters =====

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn published_date(&self) -> &str {
        &self.published_date
    }

    // ===== Setters =====

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn set_author(&mut self, author: &str) {
        self.author = author.to_string();
    }

    pub fn set_published_date(&mut self, published_date: &str) {
        self.published_date = published_date.to_string();
    }
}