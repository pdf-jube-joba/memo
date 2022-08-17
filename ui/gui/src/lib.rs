use chrono::prelude::*;
use ownlinkmemo_domain as domain;
use domain::{repository as repository, link::Constructor};

const LEFTPANEL_LENGTH: f32 = 300.0;
const BUTTONPANEL_LENGTH: f32 = 50.0;
const TILE_LENGTH: f32 = 150.0;

pub struct State {
    user: domain::shared::User,
    cache: Vec<(repository::Id, repository::Memo, Thumbnail)>,
    search: String,
//    pub temp: Option<repository::Memo>,
//    load: Option<u8>,
    view: View,
}

pub enum MessageResponse {
    Search,
    Pick(repository::Id),
    Post(repository::Constructor),
}

pub enum ChangeRequest {
    BackHome,
    Reload(Vec<(repository::Id, repository::Memo, Thumbnail)>),
    ViewMemo(repository::Id, repository::Memo),
}

pub enum InnerMessage {
    BackHome,
    MoveTo(domain::repository::Id),
    CreateLink(CreateLink),
    MakingView,
}

fn or_op(o: &mut Option<InnerMessage>, r: Option<InnerMessage>) {
    match o {
        None => {*o = r;}
        Some(_) => {}
    }
}

pub enum View {
    Home,
    InMemoLoad,
    InMemo(repository::Id, MemoTemp),
    CreateLink(CreateLink),
}

impl Default for View {
    fn default() -> Self {
        View::Home
    }
}

pub enum Thumbnail {
    Loading(Option<u8>),
    Default,
}

pub struct MemoTempLink {
    time: chrono::DateTime<Local>,
    owner: Option<domain::shared::User>,
    link: String,
    link_changeable: bool,
}

pub struct CreateLink {
    link: String,
}

pub enum MemoTemp {
    Link(MemoTempLink),
}

impl State {
    pub fn new(user: domain::shared::User) -> State {
        Self {
            user,
            cache: Vec::new(),
            search: String::new(),
            view: View::Home,
        }
    }

    pub fn request_process(&mut self, mes: ChangeRequest) {
        match mes {
            ChangeRequest::BackHome => {
                self.view = View::Home;
            }
            ChangeRequest::Reload(vec) => {
                self.cache = vec;
            }
            ChangeRequest::ViewMemo(id, memo) => {
                let temp = match memo {
                    repository::Memo::Link(link) => {
                        MemoTemp::Link(MemoTempLink{
                            time: link.info_system().created.clone(),
                            owner: Some(link.info_system().owner.clone()),
                            link: link.body().link().clone(),
                            link_changeable: false,
                        })
                    }
                    _ => unimplemented!()
                };
                self.view = View::InMemo(id, temp);
            }
        }
    }

    pub fn step(&mut self, ui: &mut egui::Ui) -> Option<MessageResponse> {
        let mes = self.default_view(ui);
        if let Some(mes) = mes {
            self.inner_message_process(mes)
        } else {
            None
        }
    }

    pub fn default_view(&mut self, ui: &mut egui::Ui) -> Option<InnerMessage> {
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

    pub fn inner_message_process(&mut self, mes: InnerMessage) -> Option<MessageResponse> {
        match mes {
            InnerMessage::BackHome => {
                self.view = View::Home;
                return Some(MessageResponse::Search);
            },
            InnerMessage::MoveTo(id) => {
                self.view = View::InMemoLoad;
                return Some(MessageResponse::Pick(id));
            }
            InnerMessage::CreateLink(CreateLink{link}) => {
                let mes = MessageResponse::Post(repository::Constructor::Link(
                    domain::link::Constructor {
                        registrant: self.user.clone() ,
                        origin: domain::link::Origin::Nothing ,
                        kind: domain::link::Kind::Local ,
                        content_type: "".to_string() ,
                        link: link ,
                    }
                    ));
                self.view = View::Home;
                Some(mes)
            }
            InnerMessage::MakingView => {
                self.view = View::CreateLink(CreateLink{link: "".to_string()});
                None
            }
        }
    }

    fn left(&mut self, ui: &mut egui::Ui) -> Option<InnerMessage> {
        let mut mes = None;
        ui.separator();
        match self.view {
            View::Home => {ui.label("in Repo");}
            View::InMemoLoad => {ui.label("memo loading");}
            View::InMemo(_,_) => {ui.label("in Memo");}
            View::CreateLink(_) => {ui.label("creating link");}
        }
        ui.separator();
        ui.text_edit_singleline(&mut self.search);
        if ui.button("Back Home").clicked() {
            mes = Some(InnerMessage::BackHome)
        }
        if ui.button("make link").clicked() {
            mes = Some(InnerMessage::MakingView)
        }
        mes
    }

    fn bottom(&mut self, ui: &mut egui::Ui) -> Option<InnerMessage>{
        let mut mes = None;
        ui.with_layout(egui::Layout::right_to_left(), |ui|{
            match self.view {
                View::Home => {ui.label("none");}
                View::InMemoLoad => {ui.label("memo loading");}
                View::InMemo(_,_) => {
                    ui.label("change?");
                }
                View::CreateLink(CreateLink{ref link}) => {
                    if ui.button("create").clicked() {
                        mes = Some(InnerMessage::CreateLink(CreateLink{link: link.clone()}));
                    }
                }
            };
        });
        mes
    }

    fn center(&mut self, ui: &mut egui::Ui) -> Option<InnerMessage> {
        match self.view {
            View::Home => {
                let const_num = ((ui.available_width()) / TILE_LENGTH).floor() as i32;
                let mut target = None;
                let mut num = const_num;
                egui::Grid::new("main")
                .show(ui, |ui|{
                    for (ref mut id,ref memo, thumbnail) in &mut self.cache {
                        let thumbnailview =
                        match thumbnail {
                            Thumbnail::Loading(u) => ThumbnailView::Loading(u.clone()),
                            Thumbnail::Default => ThumbnailView::Default(&memo)
                        };
                        if thumbnail_view(thumbnailview, ui) {
                            target = Some(InnerMessage::MoveTo(id.clone()));
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
            View::InMemoLoad => {
                None
            }
            View::InMemo(ref mut _id, ref mut memo) => {
                    match memo {
                        MemoTemp::Link(temp) => {
                            let view = MemoView::Link(MemoViewLink {
                                time : &temp.time,
                                owner : temp.owner.as_ref(),
                                link : &mut temp.link,
                                link_changeable : temp.link_changeable,
                            });
                        memo_view(view, ui);
                        }
                    }
                None
            }
            View::CreateLink(ref mut creatememo) => {
                let time = Local::now();
                let view = MemoView::Link(MemoViewLink {
                    time : &time ,
                    owner : None,
                    link : &mut creatememo.link,
                    link_changeable : true,
                });
                memo_view(view, ui);
                None
            }
        }
    }
}

#[derive(Hash)]
pub struct MemoViewLink<'a> {
    time: &'a chrono::DateTime<Local>,
    owner: Option<&'a domain::shared::User>,
    link: &'a mut String,
    link_changeable: bool,
}

#[derive(Hash)]
pub enum MemoView<'a> {
    Link(MemoViewLink<'a>),
}

fn memo_view(memo: MemoView, ui: &mut egui::Ui) -> Option<InnerMessage> {
    egui::Grid::new(&memo)
    .show(ui, |ui|{
        match memo {
            MemoView::Link(ob) => {
                ui.label("link");
                ui.add_enabled(ob.link_changeable, egui::TextEdit::singleline(ob.link));
                ui.end_row();
            }
        }
    });
    None
}

pub enum ThumbnailView<'a> {
    Loading(Option<u8>),
    Default(&'a repository::Memo),
}

fn thumbnail_view(ob: ThumbnailView, ui: &mut egui::Ui) -> bool {
    let mut b = false; 
    egui::Frame::default()
    .fill(egui::Color32::GRAY)
    .show(ui, |ui|{
        match ob {
            ThumbnailView::Loading(_) => {
                ui.add_sized(
                    [TILE_LENGTH, TILE_LENGTH],
                    egui::Spinner::default()
                );
            }
            ThumbnailView::Default(memo) => {
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