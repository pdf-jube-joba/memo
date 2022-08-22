use chrono::prelude::*;
use ownlinkmemo_domain as domain;
use domain::{repository as repository};
use crate::{data::*, view::*};

pub struct AppState {
  user: domain::shared::User,
  cache: Vec<(repository::Id, repository::Memo, Thumbnail)>,
  search: String,
  view: View,
}

fn or_op(o: &mut Option<InnerMessage>, r: Option<InnerMessage>) {
  match o {
    None => {*o = r;}
    Some(_) => {}
  }
}

impl AppState {
    pub fn new(user: domain::shared::User) -> AppState {
        Self {
            user,
            cache: Vec::new(),
            search: String::new(),
            view: View::Home,
        }
    }

    pub fn request_process(&mut self, mes: AppStateChangeRequest) {
        match mes {
            AppStateChangeRequest::BackHome => {
                self.view = View::Home;
            }
            AppStateChangeRequest::Reload(vec) => {
                self.cache = vec;
            }
            AppStateChangeRequest::ViewMemo(id, memo) => {
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

    pub fn step(&mut self, ui: &mut egui::Ui, req: Option<AppStateChangeRequest>) -> Option<MessageResponse> {
        if let Some(req) = req {self.request_process(req);}
        let mes = self.view(ui);
        if let Some(mes) = mes {
            self.inner_message_process(mes)
        } else {
            None
        }
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

    pub fn view(&mut self, ui: &mut egui::Ui) -> Option<InnerMessage> {
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
                memo_view(memo.into(), ui);
                None
            }
            View::CreateLink(ref mut creatememo) => {
                memo_view(creatememo.into(), ui);
                None
            }
        }
    }
}
