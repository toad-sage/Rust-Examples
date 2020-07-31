use lazy_static::lazy_static;
use mongodb::Client;

lazy_static! {
    static ref DATABASE_CONNECTION: Client = Client::with_uri_str("mongodb+srv://ruffle:10qpalzm@cluster0-fw7ue.mongodb.net/avenger?retryWrites=true&w=majority").unwrap();
}

pub struct Connection;

pub trait IConnnection {
    fn init(&self) -> &'static Client;
}

impl IConnnection for Connection {
    fn init(&self) -> &'static Client {
        lazy_static::initialize(&DATABASE_CONNECTION);
        &*DATABASE_CONNECTION
    }
}
