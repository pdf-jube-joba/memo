use egui::Link;
use ownlinkmemo_domain as domain;

const LEFTPANEL_LENGTH: f32 = 300.0;
const BUTTONPANEL_LENGTH: f32 = 50.0;
const TILE_LENGTH: f32 = 150.0;

pub enum View {
    InRepo(Vec<(domain::repository::Id ,domain::repository::Memo)>),
    InMemo(domain::repository::Memo),
    CreateLink(domain::repository::Memo)
}

pub enum Message {
    BackHome,
    MoveTo(domain::repository::Id),
    CreateLink
}

fn or_op(o: &mut Option<Message>, r: Option<Message>) {
    match o {
        None => {*o = r;}
        Some(_) => {}
    }
}

impl View {
    pub fn default_view(&mut self, ui: &mut egui::Ui) -> Option<Message> {
        let mut mes = None;
        egui::SidePanel::left("left")
        .resizable(false)
        .default_width(LEFTPANEL_LENGTH)
        .show_inside(ui, |ui|{
            or_op(&mut mes, self.left(ui));
        });
        egui::TopBottomPanel::bottom("buttom")
        .resizable(false)
        .default_height(BUTTONPANEL_LENGTH)
        .show_inside(ui, |ui|{
            or_op(&mut mes, self.bottom(ui));
        });
        egui::CentralPanel::default()
        .show_inside(ui, |ui|{
            or_op(&mut mes, self.center(ui));
        });
        mes
    }

    fn left(&mut self, ui: &mut egui::Ui) -> Option<Message> {
        let mut mes = None;
        if ui.button("Back Home").clicked() {
            mes = Some(Message::BackHome)
        }
        ui.separator();
        match self {
            View::InRepo(_) => {ui.label("in Repo");}
            View::InMemo(_) => {ui.label("in Memo");}
            View::CreateLink(_) => {ui.label("creating link");}
        }
        ui.separator();
        if ui.button("make link").clicked() {
            mes = Some(Message::CreateLink);
        }
        mes
    }
    fn bottom(&mut self, ui: &mut egui::Ui) -> Option<Message>{
        let mut mes = None;
        ui.with_layout(egui::Layout::right_to_left(), |ui|{
            match self {
                View::InRepo(_) => {ui.label("none");}
                View::InMemo(_) => {ui.label("none");}
                View::CreateLink(_) => {
                    if ui.button("create").clicked() {
                        mes = Some(Message::CreateLink);
                    }
                }
            };
        });
        mes
    }
    fn center(&mut self, ui: &mut egui::Ui) -> Option<Message> {
        match self {
            View::InRepo(vec) => {
                let const_num = ((ui.available_width()) / TILE_LENGTH).floor() as i32;
                let mut target = None;
                let mut num = const_num;
                egui::Grid::new("main")
                .show(ui, |ui|{
                    for (id, memo) in vec {
                        if tile(memo, ui).inner {
                            target = Some(Message::MoveTo(id.clone()));
                        }
                        if num <= 0 {
                            ui.end_row();
                            num = const_num;
                        } else {
                            num = num - 1;
                        }
                    }
                });
                target
            }
            View::InMemo(memo) => {
                panel(memo, ui)
            }
            View::CreateLink(memo) => {
                panel(memo, ui)
            }
        }
    }
}

fn panel(memo: &mut domain::repository::Memo, ui: &mut egui::Ui) -> Option<Message> {
    egui::Grid::new(memo.clone())
    .show(ui, |ui|{
    match memo {
        domain::repository::Memo::Link(ob) => {
            let link = (*ob).body().link().clone();
            let origin = format!("{:?}", ob.info_user().origin.clone());
            let kind = format!("{:?}", ob.info_user().kind.clone());
            ui.label("link");
            ui.label(link);
            ui.end_row();
            ui.label("origin");
            ui.label(origin);
            ui.end_row();
            ui.label("kind");
            ui.label(kind);
        }
        _ => {
            ui.label("not implemented");
        }
    }
   });
    None
}

fn tile(ob: &domain::repository::Memo, ui: &mut egui::Ui) -> egui::InnerResponse<bool> {
    let str: String = match ob {
        domain::repository::Memo::Link(obj) => {
            obj.body().clone().link().clone()
        }
        _ => "not implemented".to_string()
    };
    egui::Frame::default()
    .fill(egui::Color32::GRAY)
    .show(ui, |ui|{
        let r =
            ui.add_sized(
                [TILE_LENGTH, TILE_LENGTH],
                egui::Label ::new(str).sense(egui::Sense::click()))
            .clicked();
        if r {println!("clicked");}
        r
    })
}