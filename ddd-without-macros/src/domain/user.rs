use ddd_core::*;

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[derive(Clone, Debug)]
pub struct UserEntity {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub created_by: String,
    pub updated_on: DateTime<Utc>,
    pub updated_by: String,
    pub name: String,
    pub age: u8,
}

impl From<UserEntity> for User {
    fn from(entity: UserEntity) -> Self {
        Self {
            name: entity.name,
            age: entity.age,
        }
    }
}

impl Entity<Uuid> for UserEntity {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn created_on(&self) -> &DateTime<Utc> {
        &self.created_on
    }

    fn created_by(&self) -> &str {
        &self.created_by
    }

    fn updated_on(&self) -> &DateTime<Utc> {
        &self.updated_on
    }

    fn updated_by(&self) -> &str {
        &self.updated_by
    }
}

impl UpdateableWith<User> for UserEntity {
    fn update(&mut self, data: User) -> &mut Self {
        self.name = data.name;
        self.age = data.age;
        self
    }
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
