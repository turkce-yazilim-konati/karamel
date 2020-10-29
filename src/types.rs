#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum BramaKeywordType {
    None=0,
    Do,
    If,
    In,
    For,
    Let,
    New,
    Try,
    Var,
    Case,
    Else,
    Enum,
    Eval,
    Null,
    This,
    True,
    Void,
    With,
    Break,
    Catch,
    Class,
    Const,
    False,
    Super,
    Throw,
    While,
    Yield,
    Delete,
    Export,
    Import,
    Public,
    Return,
    Static,
    Switch,
    Typeof,
    Default,
    Extends,
    Finally,
    Package,
    Private,
    Continue,
    Debugger,
    Function,
    Arguments,
    Interface,
    Protected,
    Implements,
    Instanceof,
    Undefined
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum BramaOperatorType {
    None,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Increment,
    Deccrement,
    Assign,
    AssignAddition,
    AssignSubtraction,
    AssignMultiplication,
    AssignDivision,
    AssignModulus,
    Equal,
    EqualValue,
    NotEqual,
    NotEqualValue,
    Not,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseUnsignedRightShift,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    QuestionMark,
    ColonMark,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftParentheses,
    RightParentheses,
    SquareBracketStart,
    SquareBracketEnd,
    Comma,
    Semicolon,
    Dot,
    CommentLine,
    CommentMultilineStart,
    CommentMultilineEnd,
    CurveBracketStart,
    CurveBracketEnd,
    NewLine
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum BramaTokenType {
    None=0,
    Integer,
    Double,
    Symbol,
    Operator,
    Text,
    Keyword,
    EndOfFile
}

pub trait TokenTrait {
    fn get_line(&self) -> i32;
    fn get_column(&self) -> i32;
    fn get_type(&self) -> BramaTokenType;
}

#[warn(dead_code)]
pub struct OperatorToken {
    pub line      : i32,
    pub column    : i32,
    pub operator  : BramaOperatorType
}

impl TokenTrait for OperatorToken {
    fn get_line(&self) -> i32 {
        self.line
    }

    fn get_column(&self) -> i32 {
        self.column
    }

    fn get_type(&self) -> BramaTokenType {
        BramaTokenType::Operator
    }
}

#[warn(dead_code)]
pub struct Tokinizer {
    pub data  : &'static str,
    pub length: i32,
    pub line  : i32,
    pub column: i32,
    pub index : i32,
    pub tokens: Vec<Box<dyn TokenTrait>>
}

impl Tokinizer {
    pub fn is_end(&self) -> bool {
        self.length <= self.index
    }

    pub fn get_char(&self) -> char {
        if !self.is_end() {
            return self.data.chars().nth(self.index as usize).unwrap();
        }
        return '\0';
    }

    pub fn get_next_char(&self) -> char {
        if self.length > self.index + 1 {
            return self.data.chars().nth((self.index + 1) as usize).unwrap();
        }
        return '\0';
    }

    pub fn add_token(&mut self, token: Box<dyn TokenTrait>) {
        self.column = 0;
        self.tokens.push(token);
    }

    pub fn increase_index(&mut self) {
        self.index += 1;
    }

    pub fn increate_line(& mut self) {
        self.line += 1;
    }

    pub fn reset_column(& mut self) {
        self.column = 0;
    }
}


pub trait CharTraits {
    fn is_new_line(&self) -> bool;
    fn is_whitespace(&self) -> bool;
    fn is_symbol(&self) -> bool;
}

impl CharTraits for char {
    fn is_new_line(&self) -> bool {
        *self == '\n'
    }

    fn is_whitespace(&self) -> bool {
        *self == ' ' || *self == '\r' || *self == '\t'
    }

    fn is_symbol(&self) -> bool {
        (*self >= 'a' && *self <= 'z')     ||
            (*self >= 'A' && *self <= 'Z') ||
            *self == '_'                   ||
            *self == '$'
    }
}