pub type Tokens = Vec<Token>;

pub enum Token {
    IncreaseValue,
    DecreaseValue,
    MovePointerToRight,
    MovePointerToLeft,
    StartLoop,
    EndLoop,
    Read,
    Write

}

impl Token {
    pub fn tokenize(input: &str) -> Vec<Self> {
        let mut tokens: Vec<Self> = Vec::new();
        let characters: Vec<char> = input.chars().collect();

        for character in characters {
            let token = match character {
                '+' => Self::IncreaseValue,
                '-' => Self::DecreaseValue,
                '>' => Self::MovePointerToRight,
                '<' => Self::MovePointerToLeft,
                '[' => Self::StartLoop,
                ']' => Self::EndLoop,
                ',' => Self::Read,
                '.' => Self::Write,
                _ => continue
            };

            tokens.push(token);
        }

        tokens
    }
}