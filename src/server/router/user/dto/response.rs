use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct UserResponse{
    pub name:String,
    pub id:i32
}