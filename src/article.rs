use url::Url;
use rss::Item;

#[derive(Debug)]
struct Article {
    title: String,
    author: String,
    date: String,
    summary: String,
    body: String,
    url: Url,
}

impl Article {
    pub fn new(item: &Item) -> Article {
        Article {
            title: item.title().unwrap().to_owned(),
            author: item.author().unwrap().to_owned(),
            date: item.pub_date().unwrap().to_owned(),
            summary: item.description().unwrap().to_owned(),
            body: item.content().unwrap().to_owned(),
            url: Url::parse(item.link().unwrap()).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use rss::Channel;


    #[test]
    fn test_new_article() {
        let channel = match Channel::from_url("http://feeds.washingtonpost.com/rss/rss_morning-mix") {
            Ok(val) => val,
            Err(_) => panic!("No data"),
        };
        let item = &channel.items()[0];
        let a = Article::new(&item);
        println!("{:?}", a);
        assert!(false);
    }
}