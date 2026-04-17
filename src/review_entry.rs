use crate::{MangaData, RatingEntry};

pub struct ReviewEntry {
    pub manga : MangaData,
    pub ratings : Vec<RatingEntry>
}