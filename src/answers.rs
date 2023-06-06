pub(crate) trait CommandAnswers {
    fn get_possible_answers(&self) -> &Vec<String>;
    fn get_mut_possible_answers(&mut self) -> &mut Vec<String>;
    fn get_answer(&self) -> &str {
        let answers = self.get_possible_answers();
        let index = rand::random::<usize>() % answers.len();
        answers[index].as_str()
    }
    fn add_possible_answer(&mut self, answer: String) {
        self.get_mut_possible_answers().push(answer);
    }
    fn delete_possible_answer(&mut self, answer: String) {
        self.get_mut_possible_answers().retain(|x| x != &answer);
    }
}

struct Anecdote {
    pub(crate) answers: Vec<String>,
    pub(crate) min_length: usize,
}

impl CommandAnswers for Anecdote {
    fn get_possible_answers(&self) -> &Vec<String> {
        &self.answers
    }
    fn get_mut_possible_answers(&mut self) -> &mut Vec<String> {
        &mut self.answers
    }
}

pub(crate) struct CheckOk {
    pub(crate) answers: Vec<String>,
}

