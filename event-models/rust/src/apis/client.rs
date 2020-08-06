use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
        }
    }

}
