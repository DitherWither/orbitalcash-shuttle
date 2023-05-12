use rocket::{
    outcome::IntoOutcome,
    request::{FromRequest, Outcome},
    Request,
};

pub struct CurrentUser(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<CurrentUser, ()> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| {
                let user_id = cookie.value().parse::<i32>().ok()?;
                Some(CurrentUser(user_id))
            })
            .or_forward(())
    }
}
