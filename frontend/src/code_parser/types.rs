#[derive(Debug, Clone)]
pub enum ParseBody {
    StringContent(String),
    NodeContent(Box<ParseNode>),
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub name: String,
    pub children: Vec<ParseBody>,
    pub self_closing: bool,

    /// <tag param />
    pub extra_param: String,
}
