#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("book: {} {} ", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("movie: {} {} ", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("audio book: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        match self {
            Media::Book { title, author } => {
                format!("book: {} {} ", title, author)
            }
            Media::Movie { title, director } => {
                format!("movie: {} {} ", title, director)
            }
            Media::Audiobook { title } => {
                format!("audio book: {}", title)
            }
            Media::Podcast(id) => {
                format!("podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}
