use actix::{Handler, Message};
use chrono::{Duration, Local};
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use diesel::{self, prelude::*};
use crate::errors::ServiceError;
use crate::models::{DbExecutor, Invitation};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateInvitation {
    pub email: String,
}

impl Message for CreateInvitation {
    type Result = Result<Invitation, ServiceError>;
}

impl Handler<CreateInvitation> for DbExecutor {
    type Result = Result<Invitation, ServiceError>;

    fn handle(&mut self, msg: CreateInvitation, _: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();

        let new_invitation = Invitation {
            id: Uuid::new_v4(),
            email: msg.email.clone(),
            expires_at: Local::now().naive_local() + Duration::hours(24),
        };

        diesel::insert_into(invitations)
            .values(&new_invitation)
            .execute(conn)
            .map_err(|error| {
                println!("{:#?}", error);
                ServiceError::InternalServerError
            })?;

        let mut items = invitations
            .filter(email.eq(&new_invitation.email))
            .load::<Invitation>(conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(items.pop().unwrap())
    }
}