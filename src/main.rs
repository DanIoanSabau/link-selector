extern crate error_chain;
extern crate reqwest;
extern crate select;
extern crate tokio;

error_chain::error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let response_body = reqwest::get("https://www.rust-lang.org/en-US")
        .await?
        .text()
        .await?;

    select::document::Document::from(response_body.as_str())
        .find(select::predicate::Name("a"))
        .filter_map(|node| node.attr("href"))
        .for_each(|link_url| println!("{}", link_url));

    Ok(())
}
