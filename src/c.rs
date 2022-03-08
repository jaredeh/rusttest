// config files with ini format

use confy;

use serde_derive::Serialize;
use serde_derive::Deserialize;
//use std::error::Error;


#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    api_key: String,
}

fn lconftest() -> Result<(), confy::ConfyError> {
    let cfg: MyConfig = confy::load("jared_tests")?;
    dbg!(cfg);
    Ok(())
}

pub fn jrconftest() -> () {
    let cfg = MyConfig {version:3, api_key:"adddaf".to_string()};
    confy::store("jared_tests", cfg);
    match lconftest() {
        Ok(cfg) => cfg,
        Err(e) => (),
    }
}