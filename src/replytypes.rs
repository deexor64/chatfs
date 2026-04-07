use serde::Serialize;


#[derive(Debug, Serialize)]
pub enum ReplyType {
    #[serde(rename = "code_context")]
    CodeContext,
}




#[derive(Serialize)]
pub struct CodeContext<'a> {
    pub status: bool,
    pub reply_type: ReplyType,
    pub context: &'a str,
}
