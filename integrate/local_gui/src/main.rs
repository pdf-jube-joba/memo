use infra::TestRepository;
use ownlinkmemo_domain as domain;
use ownlinkmemo_gui as gui;
use ownlinkmemo_in_memory as infra;
use ownlinkmemo_domain::repository;
use ownlinkmemo_domain::repository::MemoRepository;
use eframe::egui;

pub struct State {
    repo: TestRepository,
    req: Option<gui::data::AppStateChangeRequest>,
    ui_state: gui::app::AppState,
}

impl Default for State {
    fn default() -> Self {
        Self{ 
            repo:TestRepository::init(),
            req: None,
            ui_state: gui::app::AppState::new(domain::shared::User { number: 0 }),
        }
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            if let Some(mes) = self.ui_state.step(ui, self.req.take()) {
                match mes {
                    gui::data::MessageResponse::Search => {
                        let ids = self.repo.search();
                        let mut vec = Vec::new();
                        for id in ids {
                            let memo = self.repo.pick(id.clone()).unwrap();
                            let thumbnail = gui::data::Thumbnail::Default;
                            vec.push((id, memo, thumbnail));
                        }
                        self.req = Some(gui::data::AppStateChangeRequest::Reload(vec));
                        
                    }
                    gui::data::MessageResponse::Pick(id) => {
                        let memo = self.repo.pick(id.clone()).unwrap();
                        self.req = Some(gui::data::AppStateChangeRequest::ViewMemo(id, memo));
                    }
                    gui::data::MessageResponse::Post(constructor) => {
                        self.repo.post(constructor).unwrap();
                    }
                    gui::data::MessageResponse::Modify(id,modify) => {
                        self.repo.modify(id,modify).unwrap();
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