pub trait Stage {
    type Input;
    type Output;

    fn process(&mut self, input: Self::Input) -> Self::Output;
}

// dummies for testing
struct StringToChars;
impl Stage for StringToChars {
    type Input = String;
    type Output = Vec<char>;

    fn process(&mut self, input: Self::Input) -> Self::Output {
        input.chars().collect()
    }
}

struct CharsToInts;
impl Stage for CharsToInts {
    type Input = Vec<char>;
    type Output = Vec<i32>;

    fn process(&mut self, input: Self::Input) -> Self::Output {
        input.iter().map(|c| *c as i32).collect()
    }
}