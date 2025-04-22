#![feature(inherent_str_constructors)]

mod templates;
mod read;
mod optimize;
mod member;
mod post;
mod featured_work;
mod die_linky;

pub const FRONT_MATTER_SPLIT: &'static str = "=====";

fn main() {
    println!("Hello, world!");
}
