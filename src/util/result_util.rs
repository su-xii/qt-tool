use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use bytes::{BufMut, BytesMut};

#[derive(Serialize,Deserialize,Debug)]
pub struct ResultUtil<T = ()>{
    code:i32,
    message:String,
    data:Option<T>
}

impl <T> IntoResponse for ResultUtil<T>
where T:Serialize
{
    fn into_response(self) -> Response {
        fn make_response(buf: BytesMut, ser_result: serde_json::Result<()>) -> Response {
            match ser_result {
                Ok(()) => (
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                    )],
                    buf.freeze(),
                ).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )],
                    err.to_string(),
                ).into_response(),
            }
        }

        let mut buf = BytesMut::with_capacity(128).writer();
        let res = serde_json::to_writer(&mut buf, &self);
        make_response(buf.into_inner(), res)
    }
}

impl <T:Serialize> ResultUtil<T>{
    pub fn success_with_data(message:String ,data: T)-> ResultUtil<T>{
        ResultUtil{
            code: 0,
            message,
            data:Some(data),
        }
    }

    pub fn fail_with_data(message:String ,data: T)-> ResultUtil<T>{
        ResultUtil{
            code: 1,
            message,
            data:Some(data),
        }
    }
}

impl ResultUtil<()>{
    pub fn success(message:String)-> ResultUtil<()>{
        ResultUtil{
            code: 0,
            message,
            data: None,
        }
    }

    pub fn fail(message:String)-> ResultUtil<()>{
        ResultUtil{
            code: 1,
            message,
            data:None
        }
    }
}
