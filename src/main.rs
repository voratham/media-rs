mod content;

use content::media::Media;
use content::catalog::Catalog;

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

    let item = catalog.get_by_index(40);

    let placeholder = Media::Placeholder;
    // println!("🟢 > {:#?}", item.unwrap());

    // println!(
    //     "🟢 > {:#?}",
    //     item.expect("🔴 expected there to be an item here !")
    // );

    println!("🟢 > {:#?}", item.unwrap_or(&placeholder)); // fallback value
    println!("🔥 end program");
}

