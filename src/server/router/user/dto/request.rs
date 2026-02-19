use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct UserRequest{
    pub id:i32
}