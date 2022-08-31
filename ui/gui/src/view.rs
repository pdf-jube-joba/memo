use chrono::prelude::*;
use ownlinkmemo_domain as domain;
use domain::{repository as repository};
use std::borrow::Borrow;
use crate::data::*;

#[derive(Hash)]
pub struct MemoViewLink<'a, T: Borrow<chrono::DateTime<Local>>> {
    pub time: T,
    pub owner: Option<&'a domain::shared::User>,
    pub link: &'a mut String,
    pub link_changeable: bool,
}

#[derive(Hash)]
pub enum MemoView<'a, T: Borrow<chrono::DateTime<Local>>> {
    Link(MemoViewLink<'a, T>),
}

pub enum MemoViewMessage {
}

pub fn memo_view<T: Borrow<chrono::DateTime<Local>>>(memo: MemoView<T>, ui: &mut egui::Ui) -> Option<MemoViewMessage> {
    egui::Grid::new("test")
    .show(ui, |ui|{
        match memo {
            MemoView::Link(ob) => {
                ui.label("link");
                ui.add_enabled(ob.link_changeable, egui::TextEdit::singleline(ob.link));
                ui.end_row();

                ui.label("owner");
                ui.label({
                    match ob.owner {
                        None => "you".to_string(),
                        Some(owner) => owner.number.to_string()
                    }
                });
                ui.end_row();
                
                ui.label("time");
                ui.label(ob.time.borrow().format("%Y/%m/%d %H:%M:%S").to_string());
                ui.end_row();
            }
        }
    });
    None
}

impl<'a> From<&'a mut MemoTemp> for MemoView<'a, &'a chrono::DateTime<Local>> {
    fn from(item: &'a mut MemoTemp) -> Self {
        match item {
            MemoTemp::Link(temp) => {
                MemoView::Link(MemoViewLink {
                    time : &temp.time,
                    owner : temp.owner.as_ref(),
                    link : &mut temp.link,
                    link_changeable : temp.link_changeable,
                })
            }
        }
    }
}

impl<'a> From<&'a mut CreateLink> for MemoView<'a, chrono::DateTime<Local>> {
    fn from(item: &'a mut CreateLink) -> Self {
        let time = Local::now();
        MemoView::Link(MemoViewLink {
            time : time ,
            owner : None,
            link : &mut item.link,
            link_changeable : true,
        })
    }
}

pub enum ThumbnailView<'a> {
    Loading(Option<u8>),
    Default(&'a repository::Memo),
}

pub enum ThumbnailMessage {
    Clicked,
}

pub fn thumbnail_view(ob: ThumbnailView, ui: &mut egui::Ui) -> Option<ThumbnailMessage> {
    let mut click = false; 
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
                        click = ui.add_sized(
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
    if click {Some(ThumbnailMessage::Clicked)} else {None}
}

pub struct ThumbnailListView<'a> {
    pub list: Vec<(repository::Id, ThumbnailView<'a>)>,
}

pub enum ThumbnailListMessage {
    Clicked(repository::Id),
}

pub fn thumbnail_list_view(ob: ThumbnailListView, ui: &mut egui::Ui) -> Option<ThumbnailListMessage> {
    let const_num = ((ui.available_width()) / TILE_LENGTH).floor() as i32;
    let mut target = None;
    let mut num = const_num;
    egui::Grid::new("main")
    .show(ui, |ui|{
        for (ref mut id, thumbnailview) in ob.list {
            if thumbnail_view(thumbnailview, ui).is_some() {
                target = Some(ThumbnailListMessage::Clicked(id.clone()));
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

pub enum SideMenuState {
    Home,
    InMemoLoad,
    InMemo(repository::Id),
    CreateLink,
}

pub struct SideMenuView<'a> {
    pub state: SideMenuState,
    pub search: &'a mut String,
}

pub enum SideMenuMessage {
    BackHome,
    MakeView,
}

pub fn side_menu_view(ob: SideMenuView, ui: &mut egui::Ui) -> Option<SideMenuMessage> {
    let mut mes = None;
    ui.separator();
    match ob.state {
        SideMenuState::Home => {ui.label("in Repo");}
        SideMenuState::InMemoLoad => {ui.label("memo loading");}
        SideMenuState::InMemo(id) => {ui.label("in Memo");}
        SideMenuState::CreateLink => {ui.label("creating link");}
    }
    ui.separator();
    ui.text_edit_singleline(ob.search);
    if ui.button("Back Home or Reload").clicked() {
        mes = Some(SideMenuMessage::BackHome)
    }
    if ui.button("make link").clicked() {
        mes = Some(SideMenuMessage::MakeView)
    }
    mes
}

pub enum StatusBarState {
    Home,
    Loading,
    InMemo,
    CreateLink,
}

pub struct StatusBarView {
    pub state: StatusBarState,
}

pub enum StatusBarMessege {
    CreateLink,
}

pub fn status_bar_view(ob: StatusBarView, ui: &mut egui::Ui) -> Option<StatusBarMessege>{
    let mut mes = None;
    ui.with_layout(egui::Layout::right_to_left(), |ui|{
        match ob.state {
            StatusBarState::Home => {ui.label("none");}
            StatusBarState::Loading => {ui.label("memo loading");}
            StatusBarState::InMemo => { ui.label("change?"); }
            StatusBarState::CreateLink => {
                if ui.button("create").clicked() {
                    mes = Some(StatusBarMessege::CreateLink);
                }
            }
        };
    });
    mes
}