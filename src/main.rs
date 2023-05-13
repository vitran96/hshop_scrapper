use regex::Regex;
use scraper::{Html, Selector};

fn main() {
    let host = "https://hshop.erista.me";
    let region = "north-america";
    let site_count = 100;

    let url = format!("{host}/c/games/s/{region}?sb=name&sd=ascending&count={site_count}");
    let response = reqwest::blocking::get(&url).expect("Could not load url.");
    let body = response.text().expect("No response body found.");

    let document = Html::parse_document(&body);
    let nums_of_item_selector =
        Selector::parse("div.next-container:nth-child(1) > span:nth-child(1)")
            .expect("Cannot find 'showing ... of ...");

    let nums_of_item_full_string = document
        .select(&nums_of_item_selector)
        .next()
        .expect("Cannot find 'showing ... of ...")
        .text()
        .next()
        .unwrap();

    let nums_of_item_regex = Regex::new(r"(\d+$)").unwrap();
    let nums_of_item = nums_of_item_regex
        .captures(nums_of_item_full_string)
        .expect("Failed capture")
        .get(0)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();

    println!("Nums of 3DS game: {nums_of_item}");

    let nums_of_page = nums_of_item / site_count;

    println!("Nums of page to scrap {nums_of_page}");

    let game_info_cards_selector =
        Selector::parse("div.elements > a").expect("Cannot parse CSS selector");

    let game_download_link_selector = Selector::parse(".btn").expect("Cannot parse CSS selector");

    // div.elements:nth-child(2) > a:nth-child(1) > div:nth-child(1) > h3:nth-child(1)
    let game_name_selector =
        Selector::parse(".base-info > h3:nth-child(1)").expect("Cannot parse CSS selector");

    // div.elements:nth-child(2) > a:nth-child(1) > div:nth-child(2) > div:nth-child(1) > span:nth-child(1)
    let game_id_selector =
        Selector::parse(".base-info > .meta > div:nth-child(1) > span:nth-child(1)")
            .expect("Cannot parse CSS selector");

    // div.elements:nth-child(2) > a:nth-child(1) > div:nth-child(2) > div:nth-child(3)
    // let game_title_id_selector = Selector::parse(".base-info > .meta > div:nth-child(3) > span:nth-child(1)").expect("Cannot parse CSS selector");

    // div.elements:nth-child(2) > a:nth-child(1) > div:nth-child(2) > div:nth-child(4) > span:nth-child(1)
    let game_size_selector: Selector =
        Selector::parse(".base-info > .meta > div:nth-child(4) > span:nth-child(1)")
            .expect("Cannot parse CSS selector");

    // div.elements:nth-child(2) > a:nth-child(1) > div:nth-child(2) > div:nth-child(5) > span:nth-child(1)
    let game_version_selector: Selector =
        Selector::parse(".base-info > .meta > div:nth-child(5) > span:nth-child(1)")
            .expect("Cannot parse CSS selector");

    // div.elements:nth-child(2) > a:nth-child(1) > div:nth-child(2) > div:nth-child(6) > span:nth-child(1)
    // let game_product_code: Selector = Selector::parse(".base-info > .meta > div:nth-child(6) > span:nth-child(1)").expect("Cannot parse CSS selector");

    let mut wtr = csv::Writer::from_path("hshop_na_db.csv").expect("Could not create file.");
    wtr.write_record(&["ID", "Title", "Version", "Size", "Download link"])
        .expect("Could not write header.");

    let mut current_page = 0;
    while current_page < nums_of_page {
        println!("Page {current_page}");

        let offset = current_page * 100;
        let page_url = format!("{url}&offset={offset}");

        let page_body = reqwest::blocking::get(page_url)
            .expect("Could not load url.")
            .text()
            .expect("No response body found.");

        let page_document = Html::parse_document(&page_body);

        let game_info_card_elements = page_document.select(&game_info_cards_selector);

        let mut game_page_count = 0;
        println!("Get Game info");
        for game_info_card_element_ref in game_info_card_elements {
            println!("Game in page to process: {game_page_count}");

            let game_element = game_info_card_element_ref.value();
            let href = game_element.attr("href").unwrap();
            let game_page_body = reqwest::blocking::get(format!("{host}{href}"))
                .expect("Could not load game page url.")
                .text()
                .expect("No response body found for game page.");

            let game_page_document = Html::parse_document(&game_page_body);
            let download_link = game_page_document
                .select(&game_download_link_selector)
                .next()
                .expect("Cannot get game download link")
                .value()
                .attr("href")
                .expect("No href");

            let game_name = game_info_card_element_ref
                .select(&game_name_selector)
                .next()
                .expect("Cannot find game name")
                .text()
                .next()
                .unwrap();

            let game_id = game_info_card_element_ref
                .select(&game_id_selector)
                .next()
                .expect("Cannot find game name")
                .text()
                .next()
                .unwrap();

            let game_size = game_info_card_element_ref
                .select(&game_size_selector)
                .next()
                .expect("Cannot find game size")
                .text()
                .next()
                .unwrap();

            let game_version = game_info_card_element_ref
                .select(&game_version_selector)
                .next()
                .expect("Cannot find game version")
                .text()
                .next()
                .unwrap();

            let game_info_array = [game_id, game_name, game_version, game_size, download_link];
            println!("Scrapped game info: {:?}", game_info_array);

            wtr.write_record(&game_info_array)
                .expect("Could not write data");

            game_page_count += 1;
        }

        current_page += 1;

        wtr.flush().expect("Failed to flush");
    }

    wtr.flush().expect("Could not close file");
    println!("Done");
}
