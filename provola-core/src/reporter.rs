use crate::TestResult;

pub trait Reporter {
    fn report(&self, result: TestResult);
}

pub struct SimpleReporter;

impl SimpleReporter {
    pub fn new() -> Self {
        SimpleReporter {}
    }
}

impl Reporter for SimpleReporter {
    fn report(&self, result: TestResult) {
        if let TestResult::Fail(_) = result {
            println!("FAIL");
        } else {
            println!("PASS");
        }
    }
}