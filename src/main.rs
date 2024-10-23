struct Product {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}
impl Product {}

fn main() {
    std::println!("[SIMPLE WEB SCRAPER]");

    // prompt user for url
    // https://www.scrapingcourse.com/ecommerce/
    let mut url_entry: String;
    loop {
        std::print!("Enter your URL: ");
        url_entry = String::new();
        std::io::stdin()
            .read_line(&mut url_entry)
            .expect("error: unable to read user input");
        break;
    }
    // todo: add check for valid URL
    std::println!("URL Accepted.");

    let target_url: String = String::from(url_entry);
    let response = reqwest::blocking::get(&target_url);
    let html_content: String = response.unwrap().text().unwrap();
    // println!("{html_content}");
    let document: scraper::Html = scraper::Html::parse_document(&html_content);
    let html_product_selector: scraper::Selector = scraper::Selector::parse("li.product").unwrap();
    let html_products = document.select(&html_product_selector);

    let mut products: Vec<Product> = Vec::new();

    for html_product in html_products {
        // scraping logic to retrieve info of interest
        let url: Option<String> = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a: scraper::ElementRef<'_>| a.value().attr("href"))
            .map(str::to_owned);

        let image: Option<String> = html_product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img: scraper::ElementRef<'_>| img.value().attr("src"))
            .map(str::to_owned);

        let name: Option<String> = html_product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2: scraper::ElementRef<'_>| h2.text().collect::<String>());

        let price: Option<String> = html_product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price: scraper::ElementRef<'_>| price.text().collect::<String>());

        // instantiate a new product with data and add to vec
        let product = Product {
            url,
            image,
            name,
            price,
        };
        products.push(product);
    }

    // create csv output file
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // append header to csv file
    writer
        .write_record(&["url", "image", "name", "price"])
        .unwrap();
    // populate output file
    for product in products {
        let url = product.url.unwrap();
        let image = product.image.unwrap();
        let name = product.name.unwrap();
        let price = product.price.unwrap();
        writer.write_record(&[url, image, name, price]).unwrap();
    }
    writer.flush().unwrap();


    let mut shutdown_entry: String;
    std::println!("Program completed. Check products.csv for output. (Press enter to exit)");
    shutdown_entry = String::new();
    std::io::stdin()
        .read_line(&mut shutdown_entry)
        .expect("error: unable to read user input");
}
