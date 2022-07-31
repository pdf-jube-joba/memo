use ownlinkmemo_domain as domain;
use std::{collections::HashMap};
use chrono::prelude::*;

//#[derive(Clone)]
pub struct TestRepository {
    next: u64,
    repository: HashMap<domain::repository::Id, domain::repository::Memo>
}

impl TestRepository {
    pub fn init() -> Self {
        Self {next: 0, repository: HashMap::new()}
    }
}

impl domain::repository::MemoRepository for TestRepository {
    fn search(&self) -> Vec<domain::repository::Id> {
        self.repository.iter().map(|(x,_)| x.clone()).collect()
    }
    fn pick(&self, id: domain::repository::Id) -> Result<domain::repository::Memo, domain::repository::AccessError> {
        match self.repository.get(&id) {
            Some(ctn) => Ok(ctn.clone()),
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn post(&mut self, constructor: domain::repository::Constructor) -> Result<(), domain::repository::AccessError> {
        match constructor {
            domain::repository::Constructor::Link(cst) => {
                let info_system: domain::link::InfoSystem = domain::link::InfoSystem::from(Local::now(), cst.registrant);
                let info_user: domain::link::InfoUser = domain::link::InfoUser::from(cst.origin, cst.kind, cst.content_type);
                let body: domain::link::Body = domain::link::Body::from(cst.link);
                let obj: domain::repository::LinkObject = domain::repository::LinkObject::from(info_system, info_user, body);
                let id: domain::repository::Id = domain::repository::Id::Link(domain::shared::Id::from(self.next));
                self.repository.insert(id, domain::repository::Memo::Link(obj));
                self.next +=1;
            }
        }
        Ok(())
    }
    fn modify(&mut self, id: domain::repository::Id, modifier: domain::repository::Modifier) -> Result<(), domain::repository::AccessError> {
        match self.repository.get_mut(&id) {
            Some(obj) => {
                match (obj, modifier) {
                    (domain::repository::Memo::Link(link), domain::repository::Modifier::Link(modifier)) => match modifier {
                        domain::link::Modifier::Origin(origin) => {
                            link.info_user_mut().origin = origin;
                            Ok(())
                        }
                    }
                    _ => Err(domain::repository::AccessError::NotFound)
                }
            }
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
}
