use actix_session::Session;
use actix_session::SessionExt;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::future::{ready, Ready};
use uuid::Uuid;

pub struct TypedSession(Session);

// this TypedSession wrapper will create a safe API around on redis operations
// this is a similar approach to how we used domain specific parsers when dealing
// with user input earlier.
//
// as an added bonus we are going to implement actix_web::FromRequest for this
// type so we can use it as an extractor in the same way we used the bare session
// type previously
impl TypedSession {
    // include all of your keys that you'll use here
    // to prevent messing this up later
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    // wrap your insert calls in functions to prevent bad calls
    pub fn insert_user(&self, u: Uuid) -> Result<(), serde_json::Error> {
        self.0.insert(TypedSession::USER_ID_KEY, u)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, serde_json::Error> {
        self.0.get(TypedSession::USER_ID_KEY)
    }
}

impl FromRequest for TypedSession {
    type Error = <Session as FromRequest>::Error;
    // this looks like this because traits dont support async
    // so we need to impliment Future ourselves.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
