use ddd_derive::Entity;

#[derive(Entity, Clone, Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[cfg(test)]
mod tests {
    use ddd_core::{UpdateableWith, Utc, Uuid};

    use super::*;

    #[test]
    fn user_entity_created() {
        let mut ue = UserEntity {
            id: Uuid::new_v4(),
            created_by: "admin@meetup.com".into(),
            created_on: Utc::now(),
            updated_by: "john.smith@rustmeetup.com".into(),
            updated_on: Utc::now(),
            name: "Kim".into(),
            age: 32,
        };
        println!("{:?}", &ue);
        &ue.update(User {
            age: 30,
            name: "Alex".into(),
        });
        println!("{:?}", &ue);

        let u: User = ue.into();

        println!("{:?}", u);
    }
}
