#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("book: {} {} ", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("movie: {} {} ", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("audio book: {}", title)
        } else {
            String::from("Media description")
        }
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

    println!("{}", auidobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());

    print_media(auidobook);
    print_media(good_movie);
    print_media(bad_book);
}
