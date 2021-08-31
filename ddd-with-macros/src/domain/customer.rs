use ddd_derive::Entity;

type EmailAddress = String;

#[derive(Entity, Clone, Debug)]
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
