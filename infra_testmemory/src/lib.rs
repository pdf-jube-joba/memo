use ownlinkmemo_domain as domain;
use std::collections::HashMap;

#[derive(PartialEq , Eq , Clone)]
pub struct TestLinkRepository {
    next: u64,
    linkrepository: HashMap<domain::setting::Id , domain::link::Content>
}

impl domain::link::LinkRepository for TestLinkRepository {
    fn listup(&self) -> Vec<domain::setting::Id> {
        let obj: TestLinkRepository = (*self).clone();
        obj.linkrepository.into_keys().collect()
    }
    fn search(&self, id: domain::setting::Id) -> Result<domain::link::LinkObject, domain::link::LinkError> {
        match self.linkrepository.get(&id) {
            Some(ctn) => Ok(domain::link::LinkObject::from(id, ctn.clone())) ,
            None => Err(domain::link::LinkError::NotFound)
        }
    }
    fn modify(&mut self, id: domain::setting::Id, content: domain::link::Content) -> Result<(), domain::link::LinkError> {
        if self.linkrepository.contains_key(&id) {
            self.linkrepository.insert(id, content);
            Ok(())
        } else {
            Err(domain::link::LinkError::NotFound)
        }
    }
    fn post(&mut self, content: domain::link::Content) -> Result<(), domain::link::LinkError> {
        self.linkrepository.entry(domain::setting::Id::from(self.next)).or_insert(content);
        self.next += 1;
        Ok(())
    }
}

impl TestLinkRepository {
    pub fn init() -> Self {
        Self {next: 0 , linkrepository: HashMap::new()}
    }
}