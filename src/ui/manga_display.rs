use crate::MangaData;

pub struct MangaView<'a> {
    manga: &'a MangaData,
}

// FIXME
impl MangaView<'a> {
    pub fn new(manga: &'a MangaData) -> Self {
        Self { manga }
    }
}

impl egui::Widget for MangaView<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.heading("todo")
    }
}