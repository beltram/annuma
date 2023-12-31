use crate::job::Job;
use crate::model::Farmer;
use anyhow::anyhow;
use scraper::{ElementRef, Selector};

pub fn scrap_farmer(html: String) -> anyhow::Result<Vec<Farmer>> {
    let document = scraper::html::Html::parse_document(&html);
    let html = document.root_element();

    let mut farmers = vec![];

    let category_selector = Selector::parse("div.h2div").map_err(|_| anyhow!("Parsing error"))?;
    let job_h3_selector = Selector::parse("h3").map_err(|_| anyhow!("Parsing error"))?;
    let job_h4_selector = Selector::parse("h4").map_err(|_| anyhow!("Parsing error"))?;
    let annonce_mh = Selector::parse("ul.annonce_mh").map_err(|_| anyhow!("Parsing error"))?;
    let annonce = Selector::parse("div.annonce_content").map_err(|_| anyhow!("Parsing error"))?;

    for category in html.select(&category_selector) {
        let jobs_h3 = category.select(&job_h3_selector);
        let jobs_h4 = category.select(&job_h4_selector);
        let annonce_mh = category.select(&annonce_mh);

        for (job, annonce_mh) in jobs_h3.zip(annonce_mh.clone()) {
            for annonce in annonce_mh.select(&annonce) {
                if let Ok(j) = job.inner_html().parse::<Job>() {
                    let farmer = parse_annonce(&annonce, &j)?;
                    farmers.push(farmer);
                }
            }
        }
        for (job, annonce_mh) in jobs_h4.zip(annonce_mh) {
            for annonce in annonce_mh.select(&annonce) {
                if let Ok(j) = job.inner_html().parse::<Job>() {
                    let farmer = parse_annonce(&annonce, &j)?;
                    farmers.push(farmer);
                }
            }
        }
    }

    Ok(farmers)
}

fn parse_annonce(element: &ElementRef<'_>, job: &Job) -> anyhow::Result<Farmer> {
    let title = Selector::parse("div.annonce_titre").unwrap();
    let title = element
        .select(&title)
        .next()
        .ok_or(anyhow!("No title"))?
        .inner_html();

    let mut label = String::default();
    let mut address = String::default();

    let label_selector = Selector::parse("span.fa-bookmark").unwrap();
    let address_selector = Selector::parse("span.fa-map-marker").unwrap();
    let ohwsnw = Selector::parse("div.ohwsnw").unwrap();

    for el in element.select(&ohwsnw) {
        let text = el.text().next();
        if el.select(&label_selector).next().is_some() {
            label = text
                .ok_or(anyhow!("invalid label node"))?
                .trim()
                .to_string();
        } else if el.select(&address_selector).next().is_some() {
            address = text
                .ok_or(anyhow!("invalid address node"))?
                .trim()
                .to_string();
        }
    }

    Ok(Farmer {
        title,
        label,
        address,
        job: *job,
        coord: None,
    })
}
