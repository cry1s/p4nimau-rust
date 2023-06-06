#[test]
fn test_command_trait() {
    use crate::answers::CommandAnswers;
    struct TestCommand {
        answers: Vec<String>,
    }
    impl CommandAnswers for TestCommand {
        fn get_possible_answers(&self) -> &Vec<String> {
            &self.answers
        }
        fn get_mut_possible_answers(&mut self) -> &mut Vec<String> {
            &mut self.answers
        }
    }
    let command = TestCommand {
        answers: vec!["a", "b", "c"].iter().map(|x| x.to_string()).collect(),
    };
    for _ in 0..100 {
        let answer = command.get_answer();
        assert!(command.get_possible_answers().contains(&answer.to_string()));
    }
}
