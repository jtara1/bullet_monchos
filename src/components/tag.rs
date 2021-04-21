use crate::Owner;

/* information about the entity it's attached to */
pub struct Tag {
    owner: Owner
}

impl Tag {
    pub fn new(owner: Owner) -> Self {
        Tag { owner }
    }

    pub fn owner(&self) -> &Owner {
        &self.owner
    }
}