#[derive(Debug, Clone)]
pub enum ASTBody {
    String(String),
    Node(Box<ASTNode>),
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub name: String,
    pub children: Vec<ASTBody>,
    pub self_closing: bool,

    /// <tag param />
    pub extra_param: String,
}
