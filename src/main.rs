extern crate rss;
use rss::Channel;

fn main() {
    let channel = Channel::from_url("http://feeds.washingtonpost.com/rss/rss_morning-mix").unwrap();
    println!("{:#?}",channel);
}
