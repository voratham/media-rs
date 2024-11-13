use std::default;

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Audiobook2 { title: String },
}

impl Media {
    fn description(&self) -> String {
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
            _ => String::from("Media description"),
        }
    }
}

#[derive(Debug)]
struct Catelog {
    items: Vec<Media>,
}

impl Catelog {
    fn new() -> Self {
        Catelog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let auidobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    let auidobook2 = Media::Audiobook2 {
        title: String::from("An Audiobook 2"),
    };

    // println!("{}", auidobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catelog = Catelog::new();
    catelog.add(auidobook);
    catelog.add(auidobook2);
    catelog.add(good_movie);
    catelog.add(bad_book);

    println!("{:#?}", catelog)
}
