use fake::{faker::lorem::en::Sentence, Fake};
use std::ops::Range;

pub fn get_random_enum(set: Vec<String>) -> String {
    let rand: String = Sentence(Range {
        start: 0,
        end: set.len() + 1,
    })
    .fake();
    let result = &set[rand.split(" ").collect::<Vec<&str>>().len() - 1];
    return result.to_owned();
}
