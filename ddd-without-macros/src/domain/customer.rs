use ddd_core::*;

type EmailAddress = String;

#[derive(Clone, Debug)]
pub struct Customer {
    pub name: String,
    pub membership_status: MembershipStatus,
    pub email: EmailAddress,
}

#[derive(Clone, Debug)]
pub enum MembershipStatus {
    GOLD,
    SILVER,
}

#[derive(Clone, Debug)]
pub struct CustomerEntity {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub created_by: String,
    pub updated_on: DateTime<Utc>,
    pub updated_by: String,
    pub name: String,
    pub membership_status: MembershipStatus,
    pub email: EmailAddress,
}

impl From<CustomerEntity> for Customer {
    fn from(entity: CustomerEntity) -> Self {
        Self {
            name: entity.name,
            membership_status: entity.membership_status,
            email: entity.email,
        }
    }
}

impl Entity<Uuid> for CustomerEntity {
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

impl UpdateableWith<Customer> for CustomerEntity {
    fn update(&mut self, data: Customer) -> &mut Self {
        self.name = data.name;
        self.membership_status = data.membership_status;
        self.email = data.email;
        self
    }
}

#[cfg(test)]
mod tests {
    use ddd_core::UpdateableWith;
    use ddd_core::{Utc, Uuid};

    use crate::domain::{Customer, CustomerEntity, MembershipStatus};

    #[test]
    fn customer_entity_created() {
        let mut ce = CustomerEntity {
            id: Uuid::new_v4(),
            created_by: "admin@meetup.com".into(),
            created_on: Utc::now(),
            updated_by: "john.smith@rustmeetup.com".into(),
            updated_on: Utc::now(),
            name: "Kim".into(),
            membership_status: MembershipStatus::GOLD,
            email: "kim@rustmeetup.com".into(),
        };
        println!("{:?}", &ce);
        &ce.update(Customer {
            name: "Alex".into(),
            membership_status: MembershipStatus::SILVER,
            email: "alex@rustmeetup.com".into(),
        });
        println!("{:?}", &ce);

        let c: Customer = ce.into();

        println!("{:?}", c);
    }
}
