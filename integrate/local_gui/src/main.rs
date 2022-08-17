use gui::Thumbnail;
use infra::TestRepository;
use ownlinkmemo_domain as domain;
use ownlinkmemo_gui as gui;
use ownlinkmemo_in_memory as infra;
use ownlinkmemo_domain::repository;
use ownlinkmemo_domain::repository::MemoRepository;
use eframe::egui;

pub struct State {
    repo: TestRepository,
    s: gui::State,
}

impl Default for State {
    fn default() -> Self {
        let repo = TestRepository::init();
        let s = gui::State::new(domain::shared::User { number: 0 });
        Self{ repo, s }
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            if let Some(mes) = self.s.step(ui) {
                match mes {
                    gui::MessageResponse::Search => {
                        let ids = self.repo.search();
                        let mut vec = Vec::new();
                        for id in ids {
                            let memo = self.repo.pick(id.clone()).unwrap();
                            let thumbnail = gui::Thumbnail::Default;
                            vec.push((id, memo, thumbnail));
                        }
                        let req = gui::ChangeRequest::Reload(vec);
                        self.s.request_process(req);
                    }
                    gui::MessageResponse::Pick(id) => {
                        let memo = self.repo.pick(id.clone()).unwrap();
                        let req = gui::ChangeRequest::ViewMemo(id, memo);
                        self.s.request_process(req);
                    }
                    gui::MessageResponse::Post(constructor) => {
                        self.repo.post(constructor).unwrap();
                    }
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
        Box::new(|_cc| Box::new(State::default())),
    );
}