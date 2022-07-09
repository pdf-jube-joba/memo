use ownlinkmemo_domain as domain;
use std::collections::HashMap;
use chrono::prelude::*;

//#[derive(Clone)]
pub struct TestRepository {
    next: u64,
    repository: HashMap<domain::repository::Id, domain::repository::Memo>
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
                let id: domain::repository::Id = domain::repository::Id::Link(domain::link::Id::from(self.next));
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
                            link.info_user().origin = origin;
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


/* 
impl domain::repository::LinkRepository for TestLinkRepository {
    fn search(&self) -> Vec<domain::setting::Id> {
        self.repository.iter().map(|(x,_)| x.clone()).collect()
    }
    fn pick(&self, id: domain::setting::Id) -> Result<domain::repository::LinkContent, domain::repository::AccessError> {
        match self.repository.get(&id) {
            Some(ctn) => Ok(ctn.clone()),
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn modify(&mut self, id: domain::setting::Id, info: domain::link::InfoUser, body: domain::link::Body) -> Result<(), domain::repository::AccessError> {
        match self.repository.get_mut(&id) {
            Some(v) => {
                *v = domain::repository::LinkContent::from((*v.info_system()).clone(), info, body);
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn post(&mut self, user: domain::setting::User, info: domain::link::InfoUser, body: domain::link::Body) -> Result<(), domain::repository::AccessError> {
        let info_system: domain::link::InfoSystem = domain::link::InfoSystem::from(chrono::Local::now(), user);
        let content: domain::repository::LinkContent = domain::repository::LinkContent::from(info_system, info, body); 
        self.repository.insert(domain::setting::Id::from(self.next), content) ;
        self.next += 1;
        Ok(())
    }
    fn delete(&mut self, id: domain::setting::Id) -> Result<(), domain::repository::AccessError> {
        self.repository.remove(&id);
        Ok(())
    }
}

#[derive(Clone)]
pub struct TestWordRepository {
    next: u64,
    repository: HashMap<domain::setting::Id, domain::repository::WordContent>
}

impl domain::repository::WordRepository for TestWordRepository {
    fn search(&self) -> Vec<domain::setting::Id> {
        self.repository.iter().map(|(x,_)| x.clone()).collect()
    }
    fn pick(&self, id: domain::setting::Id) -> Result<domain::repository::WordContent, domain::repository::AccessError> {
        match self.repository.get(&id) {
            Some(ctn) => Ok(ctn.clone()),
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn modify(&mut self, id: domain::setting::Id, info: domain::word::InfoUser, body: domain::word::Body) -> Result<(), domain::repository::AccessError> {
        match self.repository.get_mut(&id) {
            Some(v) => {
                *v = domain::repository::WordContent::from((*v.info_system()).clone(), info, body);
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn post(&mut self, user: domain::setting::User, info: domain::word::InfoUser, body: domain::word::Body) -> Result<(), domain::repository::AccessError> {
        let info_system: domain::word::InfoSystem = domain::word::InfoSystem::from(chrono::Local::now(), user);
        let content: domain::repository::WordContent = domain::repository::WordContent::from(info_system, info, body); 
        self.repository.insert(domain::setting::Id::from(self.next), content) ;
        self.next += 1;
        Ok(())
    }
    fn delete(&mut self, id: domain::setting::Id) -> Result<(), domain::repository::AccessError> {
        self.repository.remove(&id);
        Ok(())
    }
}

#[derive(Clone)]
pub struct TestArticleRepository {
    next: u64,
    repository: HashMap<domain::setting::Id, domain::repository::ArticleContent>
}

impl domain::repository::ArticleRepository for TestArticleRepository {
    fn search(&self) -> Vec<domain::setting::Id> {
        self.repository.iter().map(|(x,_)| x.clone()).collect()
    }
    fn pick(&self, id: domain::setting::Id) -> Result<domain::repository::ArticleContent, domain::repository::AccessError> {
        match self.repository.get(&id) {
            Some(ctn) => Ok(ctn.clone()),
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn modify(&mut self, id: domain::setting::Id, info: domain::article::InfoUser, body: domain::article::Body) -> Result<(), domain::repository::AccessError> {
        match self.repository.get_mut(&id) {
            Some(v) => {
                *v = domain::repository::ArticleContent::from((*v.info_system()).clone(), info, body);
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn post(&mut self, user: domain::setting::User, info: domain::article::InfoUser, body: domain::article::Body) -> Result<(), domain::repository::AccessError> {
        let info_system: domain::article::InfoSystem = domain::article::InfoSystem::from(chrono::Local::now(), user);
        let content: domain::repository::ArticleContent = domain::repository::ArticleContent::from(info_system, info, body); 
        self.repository.insert(domain::setting::Id::from(self.next), content) ;
        self.next += 1;
        Ok(())
    }
    fn delete(&mut self, id: domain::setting::Id) -> Result<(), domain::repository::AccessError> {
        self.repository.remove(&id);
        Ok(())
    }
}

#[derive(Clone)]
pub struct TestTempRepository {
    next: u64,
    repository: HashMap<domain::setting::Id, domain::repository::TempContent>
}

impl domain::repository::TempRepository for TestTempRepository {
    fn search(&self) -> Vec<domain::setting::Id> {
        self.repository.iter().map(|(x,_)| x.clone()).collect()
    }
    fn pick(&self, id: domain::setting::Id) -> Result<domain::repository::TempContent, domain::repository::AccessError> {
        match self.repository.get(&id) {
            Some(ctn) => Ok(ctn.clone()),
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn modify(&mut self, id: domain::setting::Id, info: domain::temp::InfoUser, body: domain::temp::Body) -> Result<(), domain::repository::AccessError> {
        match self.repository.get_mut(&id) {
            Some(v) => {
                *v = domain::repository::TempContent::from((*v.info_system()).clone(), info, body);
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn post(&mut self, user: domain::setting::User, info: domain::temp::InfoUser, body: domain::temp::Body) -> Result<(), domain::repository::AccessError> {
        let info_system: domain::temp::InfoSystem = domain::temp::InfoSystem::from(chrono::Local::now(), user);
        let content: domain::repository::TempContent = domain::repository::TempContent::from(info_system, info, body); 
        self.repository.insert(domain::setting::Id::from(self.next), content) ;
        self.next += 1;
        Ok(())
    }
    fn delete(&mut self, id: domain::setting::Id) -> Result<(), domain::repository::AccessError> {
        self.repository.remove(&id);
        Ok(())
    }
}

pub fn init() -> domain::repository::Repository {
    domain::repository::Repository {
        link_repository: Box::new(TestLinkRepository{next: 0, repository: HashMap::new()}),
        word_repository: Box::new(TestWordRepository{next: 0, repository: HashMap::new()}),
        article_repository: Box::new(TestArticleRepository{next: 0, repository: HashMap::new()}),
        temp_repository: Box::new(TestTempRepository{next: 0, repository: HashMap::new()})
    }
}

pub fn testinit() -> domain::repository::Repository {
    let mut repo = init();
    let user: domain::setting::User = domain::setting::User{number: 0};
    let info_user: domain::link::InfoUser = domain::link::InfoUser::from(
        domain::link::Origin::Nothing,
        domain::link::Kind::Global
    );
    let body: domain::link::Body = domain::link::Body::from(String::new(), String::new(), String::new());
    repo.link_repository.post(user, info_user, body).unwrap();
    repo
}

#[derive(PartialEq , Eq , Clone)]
pub struct TestRepository<InfoUser, Body> {
    next: u64,
    repository: HashMap<domain::setting::Id , domain::repository::Content<InfoUser, Body>>
}

impl <InfoUser, Body> domain::repository::Repository for TestRepository<InfoUser , Body>
    where InfoUser : Clone {
    fn listup(&self) -> Vec<domain::setting::Id> {
        self.repository.into_keys().collect()
    }
    fn search(&self, id: domain::setting::Id) -> Result<domain::repository::Object<InfoUser , Body>, domain::repository::AccessError> {
        match self.repository.get(&id) {
            Some(ctn) => Ok(domain::link::LinkObject::from(id, ctn.clone())) ,
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
}

#[derive(PartialEq , Eq , Clone)]
pub struct TestLinkRepository {
    next: u64,
    link_repository: HashMap<domain::setting::Id , domain::link::Content>,
    word_repository: HashMap<domain::setting::Id , domain::word::Content>,
    article_repository: HashMap<domain::setting::Id , domain::article::Content>
}

impl domain::repository::Repository for TestLinkRepository {
    fn link_listup(&self) -> Vec<domain::setting::Id> {
        let obj: TestLinkRepository = (*self).clone();
        obj.link_repository.into_keys().collect()
    }
    fn word_listup(&self) -> Vec<domain::setting::Id> {
        let obj: TestLinkRepository = (*self).clone();
        obj.word_repository.into_keys().collect()
    }
    fn article_listup(&self) -> Vec<domain::setting::Id> {
        let obj: TestLinkRepository = (*self).clone();
        obj.article_repository.into_keys().collect()
    }
    fn link_search(&self, id: domain::setting::Id) -> Result<domain::link::LinkObject, domain::repository::AccessError> {
        match self.link_repository.get(&id) {
            Some(ctn) => Ok(domain::link::LinkObject::from(id, ctn.clone())) ,
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn word_search(&self, id: domain::setting::Id) -> Result<domain::word::WordObject, domain::repository::AccessError> {
        match self.word_repository.get(&id) {
            Some(ctn) => Ok(domain::word::WordObject::from(id, ctn.clone())) ,
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn article_search(&self, id: domain::setting::Id) -> Result<domain::article::ArticleObject, domain::repository::AccessError> {
        match self.article_repository.get(&id) {
            Some(ctn) => Ok(domain::article::ArticleObject::from(id, ctn.clone())) ,
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn link_modify_info(&mut self, id: domain::setting::Id, info: domain::link::InfoUser) -> Result<(), domain::repository::AccessError> {
        match self.link_repository.get_mut(&id) {
            Some(v) => {
                *v = domain::link::Content::from((*v.info_system()).clone(), info, (*v.body()).clone());
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn word_modify_info(&mut self, id: domain::setting::Id, info: domain::word::InfoUser) -> Result<(), domain::repository::AccessError> {
        match self.word_repository.get_mut(&id) {
            Some(v) => {
                *v = domain::word::Content::from((*v.info_system()).clone(), info, (*v.body()).clone());
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn article_modify_info(&mut self, id: domain::setting::Id, info: domain::article::InfoUser) -> Result<(), domain::repository::AccessError> {
        match self.article_repository.get_mut(&id) {
            Some(v) => {
                *v = domain::article::Content::from((*v.info_system()).clone(), info, (*v.body()).clone());
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn link_modify_body(&mut self, id: domain::setting::Id, body: domain::link::Body) -> Result<(), domain::repository::AccessError> {
        match self.link_repository.get_mut(&id) {
            Some(v) => {
                *v = domain::link::Content::from((*v.info_system()).clone(), (*v.info_user()).clone(), body);
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn link_modify_body(&mut self, id: domain::setting::Id, body: domain::link::Body) -> Result<(), domain::repository::AccessError> {
        match self.link_repository.get_mut(&id) {
            Some(v) => {
                *v = domain::link::Content::from((*v.info_system()).clone(), (*v.info_user()).clone(), body);
                Ok(())
            },
            None => Err(domain::repository::AccessError::NotFound)
        }
    }
    fn post(&mut self, info_usr: domain::link::InfoUser, body: domain::link::Body) -> Result<(), domain::link::LinkAccessError> {
        let info_sys = domain::setting::InfoSystem::from(chrono::Local::now(), domain::setting::User{number: 0});
        let info = domain::link::Info::from(info_sys, info_usr);
        let content = domain::link::Content::from(info, body);
        self.linkrepository.entry(domain::setting::Id::from(self.next)).or_insert(content);
        self.next += 1;
        Ok(())
    }
    fn delete(&mut self, id: domain::setting::Id) -> Result<(), domain::link::LinkAccessError> {
        Ok(())
    }
}

impl TestLinkRepository {
    pub fn init() -> Self {
        Self {next: 0 , linkrepository: HashMap::new()}
    }
}
*/