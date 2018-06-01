#[macro_use]
extern crate failure;
extern crate rand;

use failure::Error;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Word(pub Vec<String>);

impl Word {
    fn new(orgline: &str) -> Word {
        let line = &orgline[1..orgline.len() - 1];
        let forms: Vec<_> = line.split('|')
            .map(|raw_form| raw_form.trim().to_string())
            .collect();
        Word(forms)
    }
}

pub struct Context {
    pub header: Word,
    words: Vec<Word>,
    init_size: usize,
    errors: Vec<Word>,
    randomizer: rand::ThreadRng,
    current_word: Word,
    guess_form: usize,
    check_form: usize,
}

impl Context {
    pub fn new(raw_table: String) -> Result<Context, Error> {
        let mut lines = raw_table.lines();
        let header = match lines.next() {
            Some(line) => Word::new(line),
            None => return Err(format_err!("Empty file")),
        };
        let words: Vec<_> = lines.skip(1).map(Word::new).collect();
        validate_words(&header, &words)?;
        let init_size = words.len();
        let mut context = Context {
            header,
            words,
            init_size,
            errors: Vec::new(),
            randomizer: rand::thread_rng(),
            current_word: Word(Vec::new()),
            guess_form: 0,
            check_form: 0,
        };
        context.next_word();
        Ok(context)
    }

    fn next_word(&mut self) {
        let words_count = self.words.len();
        if words_count < 1 {
            return;
        }
        let num = self.randomizer.gen_range(0, words_count);
        self.current_word = self.words.swap_remove(num);
        let formcnt = self.header.0.len();
        self.guess_form = self.randomizer.gen_range(0, formcnt);
        self.check_form = if self.guess_form == formcnt - 1 {
            self.randomizer.gen_range(0, formcnt - 1)
        } else {
            formcnt - 1
        };
    }

    pub fn get_check_form(&self) -> String {
        self.header.0[self.check_form].clone()
    }

    pub fn get_guess_form(&self) -> String {
        self.header.0[self.guess_form].clone()
    }

    pub fn get_guess(&self) -> String {
        self.current_word.0[self.guess_form].clone()
    }

    pub fn check(&mut self, try: &str) -> bool {
        let correct = self.current_word.0[self.check_form].contains(try);
        if !correct {
            self.words.push(self.current_word.clone());
            self.errors.push(self.current_word.clone());
        }
        self.next_word();
        correct
    }

    pub fn last_error(&self) -> Option<&Word> {
        self.errors.last()
    }

    pub fn get_errors(&self) -> &[Word] {
        &self.errors
    }

    pub fn has_next(&self) -> bool {
        self.words.len() > 0
    }

    pub fn words_count(&self) -> usize {
        self.init_size
    }

    pub fn correct_count(&self) -> usize {
        self.init_size - self.words.len() - 1
    }

    pub fn answers_count(&self) -> usize {
        self.correct_count() + self.errors.len()
    }
}

fn validate_words(header: &Word, words: &Vec<Word>) -> Result<(), Error> {
    let headlen = header.0.len();

    if headlen < 2 {
        return Err(format_err!("Invalid header"));
    }
    if words.len() < 1 {
        return Err(format_err!("Empty table"));
    }
    for (n, words) in words.iter().enumerate() {
        if words.0.len() != headlen {
            return Err(format_err!("Invalid line #{}", n + 3));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Who watches the watchmen?
    #[test]
    fn invalid_header() {
        assert_eq!(
            "Invalid header",
            validate_words(&Word::new("Bad header"), &vec![])
                .err()
                .unwrap()
                .cause()
                .to_string()
        );
    }

    #[test]
    fn empty_table() {
        assert_eq!(
            "Empty table",
            validate_words(&Word::new("|1|2|"), &vec![])
                .err()
                .unwrap()
                .cause()
                .to_string()
        );
    }

    #[test]
    fn invalid_line() {
        let head = Word::new("|One|Two|Three|");
        let words = vec![
            Word::new("|1st|2nd|3rd|"),
            Word::new("|1st|2nd|"),
            Word::new("|1st|"),
        ];

        assert_eq!(
            "Invalid line #4",
            validate_words(&head, &words)
                .err()
                .unwrap()
                .cause()
                .to_string()
        );
    }

    #[test]
    fn empty_file() {
        assert_eq!(
            "Empty file",
            Context::new(String::new())
                .err()
                .unwrap()
                .cause()
                .to_string()
        );
    }
}
