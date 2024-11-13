#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
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
            Media::Podcast(id) => {
                format!("podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            MightHaveAValue::ThereIsAValue(&self.items[index])
        } else {
            MightHaveAValue::NoValueAvailable
        }
    }
}

// TODO: craft owner Options
#[derive(Debug)]
enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook {
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

    let podcast = Media::Podcast(99);

    let placeholder = Media::Placeholder;

    // println!("{}", auidobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // println!("{:#?}", catalog.items.get(100))
    // match catalog.items.get(0) {
    //     Option::Some(value) => {
    //         println!("Item: {:#?}", value)
    //     }
    //     None => println!("Nothing at that index !"),
    // }

    // let item = catalog.get_by_index(40);
    // print!("{:#?}", item)

    match catalog.get_by_index(40) {
        MightHaveAValue::ThereIsAValue(value) => println!("Item: {:#?}", value),
        MightHaveAValue::NoValueAvailable => println!("No value here "),
    }
}
