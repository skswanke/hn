use colored::Colorize;
use serde_json::Value;
use structopt::StructOpt;

use std::io::Read;
use std::process::Command;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value)]
    page: i32,
    #[structopt(short, long)]
    article: Option<i32>,
    #[structopt(short, long)]
    comments: Option<i32>,
}

const PAGE_SIZE: i32 = 10;

fn get_page(page: i32) -> Vec<String> {
    let mut res = reqwest::blocking::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .expect("Failed to fetch");

    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Failed to read");
    let page_articles = buf
        .trim()
        .replace("[", "")
        .replace("]", "")
        .split(",")
        .map(String::from)
        .collect::<Vec<String>>();

    let slice_start: usize = (page * PAGE_SIZE) as usize;
    let slice_end: usize = slice_start + PAGE_SIZE as usize;
    let current_page = page_articles[slice_start..slice_end].to_vec();

    return current_page;
}

fn get_post_info(post: String) -> Value {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", post);
    let mut res = reqwest::blocking::get(&url).expect("Failed to fetch");

    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Failed to read");

    return serde_json::from_str(&buf).unwrap();
}

fn display_title(post_info: Value, post_number: i32, page: i32) -> String {
    format!(
        "{:>3}: {}\n     {}p / {}c - by {}",
        ((page * PAGE_SIZE) + post_number).to_string().bright_blue(),
        post_info["title"].to_string().bold(),
        post_info["score"].to_string().bright_red(),
        post_info["descendants"].to_string().bright_blue(),
        post_info["by"].to_string().green()
    )
}

struct Link {
    article_id: String,
    url: String,
}

fn get_article_links(article_no: i32) -> Link {
    let page_posts = get_page(article_no / 10);
    let post_idx: usize = (article_no % 10) as usize;
    let post = &page_posts[post_idx];
    let details = get_post_info(post.to_string());

    Link {
        article_id: post.to_string(),
        url: details["url"].to_string(),
    }
}

fn open_in_tab(link: String) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &format!("explorer \"{}\"", link)])
            .output()
            .expect("failed to open link")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("open \"{}\"", link))
            .output()
            .expect("failed to open link")
    };
    return;
}

fn print_article(article: i32) {
    let link = get_article_links(article);
    println!("{}: {}", "Article".bright_blue(), link.url);
    println!(
        "{}: https://news.ycombinator.com/item?id={}",
        "Comments".bright_red(),
        link.article_id
    );
    open_in_tab(link.url);
    return;
}

fn open_comments(comments: i32) {
    let link = get_article_links(comments);
    println!("{}: {}", "Article".bright_blue(), link.url);
    let comments_link = format!("https://news.ycombinator.com/item?id={}", link.article_id);
    println!("{}: {}", "Comments".bright_red(), comments_link);
    open_in_tab(comments_link);
    return;
}

fn print_posts(page: i32) {
    let page_posts = get_page(page);
    let mut current_post = 0;
    let mut display_posts: Vec<String> = Vec::new();
    for post in page_posts {
        let post_info = get_post_info(post);
        display_posts.push(display_title(post_info, current_post, page));
        current_post = current_post + 1;
    }
    for display_post in display_posts {
        println!("{}", display_post);
    }
    return;
}

fn main() {
    let args = Cli::from_args();
    let mut is_article = false;
    let mut is_comments = false;

    match args.article {
        Option::Some(article) => print_article(article),
        Option::None => is_article = true,
    }
    match args.comments {
        Option::Some(comments) => open_comments(comments),
        Option::None => is_comments = true,
    }

    if !is_article && !is_comments {
        print_posts(args.page);
    }
}
