#[cfg(test)]
mod test {

    use std::collections::BTreeMap;

    use crate::models::Location;
    use crate::models::Reference;
    use crate::models::Requirement;

    pub struct RequirementBuilder {
        req: Requirement,
    }

    impl RequirementBuilder {
        pub fn new(id: &str) -> Self {
            Self {
                req: Requirement {
                    id: id.into(),
                    title: None,
                    location: Location::from_str("no Location").unwrap(),
                    covers: vec![],
                    depends: vec![],
                    tags: vec![],
                    attributes: BTreeMap::new(),
                },
            }
        }

        pub fn title(mut self, s: &str) -> Self {
            self.req.title = Some(s.to_owned());
            self
        }

        pub fn location(mut self, location: &str) -> Result<Self, String> {
            let l = Location::from_str(location)?;
            self.req.location = l;
            Ok(self)
        }

        pub fn covers(
            mut self,
            id: &str,
            title: Option<&str>,
            location: &str,
        ) -> Result<Self, String> {
            let id = id.into();
            let title = title.map(std::borrow::ToOwned::to_owned);
            let location = Location::from_str(location)?;

            self.req.covers.push(Reference {
                id,
                title,
                location,
            });
            Ok(self)
        }

        pub fn depends(
            mut self,
            id: &str,
            title: Option<&str>,
            location: &str,
        ) -> Result<Self, String> {
            let id = id.into();
            let title = title.map(std::borrow::ToOwned::to_owned);
            let location = Location::from_str(location)?;

            self.req.covers.push(Reference {
                id,
                title,
                location,
            });
            Ok(self)
        }

        pub fn tag(mut self, tag: &str) -> Self {
            self.req.tags.push(tag.to_owned());
            self
        }

        pub fn attribute(mut self, key: &str, value: &str) -> Self {
            self.req.attributes.insert(key.to_owned(), value.to_owned());
            self
        }

        pub fn build(self) -> Requirement {
            self.req
        }
    }
}
#[cfg(test)]
pub use test::*;

