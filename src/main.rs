extern crate rss;
use rss::{Channel, Item};

fn main() {
    //reads the rss from URL and creates an rss::Channel
    let mut channel = Channel::from_url("http://feeds.washingtonpost.com/rss/rss_morning-mix").unwrap();
    //println!("{:#?}",channel);

    //consumes the channel and returns a vector of Items
    channel.set_items(vec![rss::Item::default()]);
    assert_eq!(channel.into_items().len(), 1);
    println!("{:#?}", channel);

}
