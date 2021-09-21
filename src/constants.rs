pub const PROTOCOL: &str = "HTTP1/1";

pub struct HttpStatus {
    pub code: u32,
    msg: &'static str,
}

impl HttpStatus {
    pub fn full_msg(&self) -> String {
        format!("{} {}", self.code, self.msg)
    }
}

pub const HTTP_OK: HttpStatus = HttpStatus {
    code: 200,
    msg: "OK",
};
pub const HTTP_NOT_FOUND: HttpStatus = HttpStatus {
    code: 404,
    msg: "NOT FOUND",
};
pub const HTTP_NOT_ALLOWED: HttpStatus = HttpStatus {
    code: 405,
    msg: "NOT ALLOWED",
};
