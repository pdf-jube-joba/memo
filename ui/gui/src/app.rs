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

fn insert_opt<T>(value: &mut Option<T>, subst: Option<T> ) {
    match value {
        None => {*value = subst;}
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

    pub fn step(&mut self, ui: &mut egui::Ui, req: Option<AppStateChangeRequest>) -> Option<MessageResponse> {
        if let Some(req) = req {self.request_process(req);}
        let mes = self.view(ui);
        if let Some(mes) = mes {
            self.inner_message_process(mes)
        } else {
            None
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
        let mut mes: Option<InnerMessage> = None;
        egui::SidePanel::left("left")
        .resizable(false)
        .default_width(LEFTPANEL_LENGTH)
        .show_inside(ui, |ui|{
            insert_opt(&mut mes, {
                let state = match &self.view {
                    View::Home => {SideMenuState::Home}
                    View::InMemoLoad => {SideMenuState::InMemoLoad}
                    View::InMemo(ref id,_) => {SideMenuState::InMemo(id.clone())}
                    View::CreateLink(_) => {SideMenuState::CreateLink}
                };
                let view = SideMenuView {state, search: &mut self.search};
                side_menu_view(view, ui).map(|mes|{
                    match mes {
                        SideMenuMessage::BackHome => InnerMessage::BackHome,
                        SideMenuMessage::MakeView => InnerMessage::MakingView,
                    }
                })
            }
            );
        });
        egui::TopBottomPanel::bottom("buttom")
        .resizable(false)
        .default_height(BUTTONPANEL_LENGTH)
        .show_inside(ui, |ui|{
            insert_opt(&mut mes, {
                let mut str = "".to_string();
                let state = match &self.view {
                    View::Home => {StatusBarState::Home}
                    View::InMemoLoad => {StatusBarState::Loading}
                    View::InMemo(ref id,_) => {StatusBarState::InMemo}
                    View::CreateLink(link) => {
                        str = link.link.clone();
                        StatusBarState::CreateLink
                    }
                };
                let view = StatusBarView {state};
                status_bar_view(view, ui).map(|mes|{
                    match mes {
                        StatusBarMessege::CreateLink => InnerMessage::CreateLink(CreateLink{link: str})
                    }
                })
            });
        });
        egui::CentralPanel::default()
        .show_inside(ui, |ui|{
            insert_opt(&mut mes, {
                let view_list = ThumbnailListView{
                    list: self.cache.iter().map(|(id,memo,thumbnail)|{
                        (
                            id.clone(),
                            match thumbnail {
                                Thumbnail::Loading(_) => ThumbnailView::Loading(None),
                                Thumbnail::Default => ThumbnailView::Default(memo),
                            }
                        )
                }).collect()};
                match self.view {
                    View::Home => {
                        thumbnail_list_view(view_list, ui).map(|mes|{
                            match mes {
                                ThumbnailListMessage::Clicked(id) => InnerMessage::MoveTo(id)
                            }
                        })
                    }
                    View::InMemoLoad => {
                        None
                    }
                    View::InMemo(ref mut id, ref mut memo) => {
                        memo_view(memo.into() , ui);
                        None
                    }
                    View::CreateLink(ref mut memo) => {
                        memo_view(memo.into(), ui);
                        None
                    }
                }
            });
        });
        mes
    }
}