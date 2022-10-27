use rocket::request::{Request, FromRequest, Outcome};

#[derive(Debug)]
pub struct Token<'r> {
    pub tkn: &'r str
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Token<'r>, ()> {

        match req.headers().get_one("Authorization") {
            Some(token) => {
                let token_raw = Token{tkn: token};
                Outcome::Success(token_raw)
            },
            None => Outcome::Forward(()),
        }
    }
}