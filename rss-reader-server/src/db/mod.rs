pub enum Category {
    Code,
    Tech,
    Ocean,
}

pub struct Feed {
    pub title: String,
    pub url: String,
    pub category: Category,
}

pub fn get_feeds() -> Vec<Feed> {
    vec![Feed {
        title: String::from("Node.js Blog"),
        url: String::from("https://nodejs.org/en/feed/blog.xml"),
        category: Category::Code,
    }]
}
