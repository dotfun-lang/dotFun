use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
	Eof,

	// Identifiers & literals
	Identifier,
	NumberLiteral,
	StringLiteral,

	// Keywords: OOP / Structures
	Class,
	Interface,
	Import,
	Package,
	Enum,
	Struct,
	Protected,
	Private,
	Override,
	This,
	New,
	Super,
	Constructor,
	Data,
	Typeof,
	Annotation,

	// Control Flow
	If,
	Else,
	Elif,
	While,
	For,
	Loop,
	Break,
	Continue,

	// Functions
	Async,
	Await,
	Function,
	Return,

	// Literals
	True,
	False,
	Null,

	// Variables
	Mut,
	Val,

	// Operators (word versions)
	And,
	Or,
	Not,
	Is,
	In,
	Of,

	// Error Handling
	Try,
	Catch,
	Finally,
	Throw,

	// Switch/Case
	Switch,
	Case,
	Default,

	// Symbols for operators
	Plus,          // +
	Minus,         // -
	Star,          // *
	Slash,         // /
	Percent,       // %
	AndAnd,        // &&
	OrOr,          // ||
	NotBang,       // !
	NotEqual,      // !=
	EqualEqual,    // ==
	Colon,         // :
	Greater,       // >
	Less,          // <
	GreaterEqual,  // >=
	LessEqual,     // <=
	MinusMinus,    // --
	PlusPlus,      // ++
	Dollar,        // $
	BangBang,      // !!

	// Assignment
	Equal, // =

	// Brackets
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	LeftBracket,
	RightBracket,

	// Lambdas
	Arrow,       // ->
	FatArrow,    // =>
	ColonColon,  // ::
	Question,    // ?
	Ellipsis,    // ...

	BitAnd,
	BitOr,
	BitXor,

	ShiftLeft,
	ShiftRight,

	AT, //@

	// Punctuation
	Comma,
	Dot,
	Semicolon
}

pub struct Token {
	pub token_type: TokenType,
	pub lexeme: String,
	pub line: i64,
	pub column: i64
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}('{}') at {}:{}",
            self.token_type, self.lexeme, self.line, self.column
        )
    }
}
