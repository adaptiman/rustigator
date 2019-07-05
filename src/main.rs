extern crate rss;
use rss::{Channel, Item};

mod article;


fn search_terms() -> Vec<&'static str> {
    let search_terms: Vec<&str> = vec![
        "TRUMP",
        "KOREA"
    ];
    search_terms
}

pub fn main() {
    // reads the rss from URL and creates an rss::Channel
    let search_terms = search_terms();


    let channel = match Channel::from_url("http://feeds.washingtonpost.com/rss/rss_morning-mix") {
        Ok(val) => val,
        Err(_) => panic!("No data"),
    };

    let mut items: Vec<&Item> = vec![];

    for item in channel.items().iter() {
        if let Some(description) = item.description() {
            for term in &search_terms {
                if description.to_uppercase().as_str().contains(term) {
                    items.push(&item);
                }
            }
        }
    }

    // println!("{:?}", items);
    for item in items {
        println!("Title: {:?}", item.title());
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let search_terms = search_terms();
        assert!(search_terms.len() > 0);
    }
}