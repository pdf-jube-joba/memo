use chrono::prelude::*;
use ownlinkmemo_domain as domain;
use domain::{repository as repository};
use std::borrow::Borrow;
use crate::data::*;

#[derive(Hash)]
pub enum MemoView<'a, T: Borrow<chrono::DateTime<Local>>> {
    Link(MemoViewLink<'a, T>),
}

#[derive(Hash)]
pub struct MemoViewLink<'a, T: Borrow<chrono::DateTime<Local>>> {
    pub time: T,
    pub owner: Option<&'a domain::shared::User>,
    pub link: &'a mut String,
    pub link_changeable: bool,
}

pub fn memo_view<T: Borrow<chrono::DateTime<Local>>>(memo: MemoView<T>, ui: &mut egui::Ui) -> Option<crate::data::InnerMessage> {
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

pub fn thumbnail_view(ob: ThumbnailView, ui: &mut egui::Ui) -> bool {
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
    click
}