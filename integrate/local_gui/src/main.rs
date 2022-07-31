use gui::Thumbnail;
use ownlinkmemo_domain as domain;
use ownlinkmemo_gui as gui;
use ownlinkmemo_in_memory as infra;
use ownlinkmemo_domain::repository;
use ownlinkmemo_domain::repository::MemoRepository;
use eframe::egui;

pub enum State {
    Home,
    InMemo(repository::Id, repository::Memo),
    CreateLink(repository::Memo),
}

pub struct InRepo {
    repo: infra::TestRepository,
    cache: Vec<(repository::Id, repository::Memo, gui::Thumbnail)>,
    view: State
}

impl InRepo {
    pub fn reload(&mut self) {
        let ids = self.repo.search();
        let mut cache = Vec::new();
        for id in ids.into_iter() {
            let memo = self.repo.pick(id.clone()).unwrap();
            let thumbnail = gui::Thumbnail::Default(memo.clone());
            cache.push((id, memo, thumbnail));
        }
        self.cache = cache;
    }
}

impl Default for InRepo {
    fn default () -> Self {
        let repo = infra::TestRepository::init();
        let cache = Vec::new();
        let view = State::Home;
        Self{repo, cache, view}
    }
}

impl eframe::App for InRepo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
/*         match self.0 {
            None => {
                egui::CentralPanel::default().show(ctx, |ui|{
                    ui.label("home");
                    if ui.button("start").clicked() {
                        *self = Window(Some(InRepo::default()));
                    }
                });
            }
            Some(ref mut repo) => {
*/
        let mut mes = None;
        egui::CentralPanel::default().show(ctx, |ui|{
            match self.view {
                State::Home => {
                    self.reload();
                    let g =
                        self.cache.iter_mut().map(
                            |(id, _memo, thumbnail)|{(id, thumbnail)}
                        ).collect();
                    let view = gui::View::Home(g);
                    mes = view.default_view(ui);
                }
                State::InMemo(ref mut id, ref mut memo) => {
                    
                }
                State::CreateLink(ref mut memo) => {

                }
            }
        });
    }
}
/*     }
} */
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(InRepo::default())),
    );
}