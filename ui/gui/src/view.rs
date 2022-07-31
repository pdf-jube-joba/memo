use egui::Link;
use ownlinkmemo_domain as domain;

const LEFTPANEL_LENGTH: f32 = 300.0;
const BUTTONPANEL_LENGTH: f32 = 50.0;
const TILE_LENGTH: f32 = 150.0;

pub enum View<'a> {
    Home(Box<dyn Iterator<Item = (&'a mut domain::repository::Id, &'a mut Thumbnail)>>),
    InMemo(MemoView),
    CreateLink(MemoView),
}

pub enum Message {
    BackHome,
    MoveTo(domain::repository::Id),
    CreateLink,
}

fn or_op(o: &mut Option<Message>, r: Option<Message>) {
    match o {
        None => {*o = r;}
        Some(_) => {}
    }
}

impl View<'_> {
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
            View::Home(_) => {ui.label("in Repo");}
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
                View::Home(_) => {ui.label("none");}
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
            View::Home(vec) => {
                let const_num = ((ui.available_width()) / TILE_LENGTH).floor() as i32;
                let mut target = None;
                let mut num = const_num;
                egui::Grid::new("main")
                .show(ui, |ui|{
                    for (id, memo) in vec {
                        if thumbnail_tile(memo, ui) {
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

#[derive(Hash)]
pub enum MemoView {
    Changeable(domain::repository::Memo),
    Unchangeable(domain::repository::Memo),
}

fn panel(mv: &mut MemoView, ui: &mut egui::Ui) -> Option<Message> {
    let (memo, b) =
    match mv {
        MemoView::Changeable(memo) => (memo, true),
        MemoView::Unchangeable(memo) => (memo, false)
    };
    egui::Grid::new(memo.clone())
    .show(ui, |ui|{
        match memo {
            domain::repository::Memo::Link(ob) => {
                let origin = format!("{:?}", ob.info_user().origin.clone());
                let kind = format!("{:?}", ob.info_user().kind.clone());
                ui.label("link");
                ui.add_enabled(b, egui::TextEdit::singleline(ob.body_mut().link_mut()));
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

pub enum Thumbnail {
    Loading(Option<u8>),
    Default(domain::repository::Memo),
}

fn thumbnail_tile(ob: &mut Thumbnail, ui: &mut egui::Ui) -> bool {
    let mut b = false; 
    egui::Frame::default()
    .fill(egui::Color32::GRAY)
    .show(ui, |ui|{
        match ob {
            Thumbnail::Loading(_) => {
                ui.add_sized(
                    [TILE_LENGTH, TILE_LENGTH],
                    egui::Spinner::default()
                );
            }
            Thumbnail::Default(memo) => {
                match memo {
                    domain::repository::Memo::Link(obj) => {
                        b = ui.add_sized(
                            [TILE_LENGTH, TILE_LENGTH],
                            egui::Label::new(obj.body().link().clone())
                                .sense(egui::Sense::click())
                        ).clicked();
                    }
                    _ => {}
                }
            }
        }
    });
    b
}