use domain::repository::MemoRepository;
use ownlinkmemo_gui as gui;
use ownlinkmemo_in_memory as infra;
use ownlinkmemo_domain as domain;
use eframe::egui;

pub enum Window {
    Home,
    Window(infra::TestRepository, gui::View),
}

impl Window {
    pub fn init() -> Self {
        let repo = infra::TestRepository::init();
        Window::Window(infra::TestRepository::init(), gui::View::InRepo(Vec::new()))
    }
}

fn reload(repo: &infra::TestRepository) -> Vec<(domain::repository::Id, domain::repository::Memo)> {
    let mut vec = Vec::new();
    let f = |id: &domain::repository::Id|{
        vec.push((id.clone(), repo.pick(id.clone()).unwrap()));
    };
    repo.search().iter().for_each(f);
    vec
}

impl Default for Window {
    fn default() -> Self {
        Self::Home
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self {
            Self::Home => {
                egui::CentralPanel::default().show(ctx, |ui|{
                    ui.label("home");
                    if ui.button("start").clicked() {
                        *self = Window::init();
                    }
                });
            }
            Self::Window(repo, view) => {
                egui::CentralPanel::default().show(ctx, |ui|{
                    if let Some(res) = view.default_view(ui) {
                        match res {
                            gui::Message::BackHome => {
                                *view = gui::View::InRepo(reload(repo));
                            }
                            gui::Message::MoveTo(id) => {
                                *view = gui::View::InMemo(repo.pick(id).unwrap());
                            }
                            gui::Message::CreateLink => {
                                let constructor = domain::link::Constructor {
                                    registrant: domain::shared::User{number: 0},
                                    origin: domain::link::Origin::Nothing,
                                    kind: domain::link::Kind::Local,
                                    content_type: "nothing".to_string(),
                                    link: "go".to_string(),
                                };
                                repo.post(domain::repository::Constructor::Link(constructor)).unwrap();
                                *view = gui::View::InRepo(reload(repo));
                            }
                        }
                }
                });
            }
        }
    }
}
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(Window::default())),
    );
}