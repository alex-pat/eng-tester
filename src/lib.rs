extern crate rand;

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
    errors: Vec<Word>,
    randomizer: rand::ThreadRng,
    current_word: Word,
    guess_form: usize,
    check_form: usize,
}

impl Context {
    pub fn new(raw_table: String) -> Result<Context, &'static str> {
        let mut lines = raw_table.lines();
        let header = match lines.next() {
            Some(line) => Word::new(line),
            None => return Err("Header not found"),
        };
        let words = lines.skip(1).map(Word::new).collect();
        let mut context = Context {
            header,
            words,
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

    pub fn last_error(&self) -> Option<Word> {
        self.errors.last().map(|w| w.clone())
    }

    pub fn get_errors(&self) -> Vec<Word> {
        self.errors.clone()
    }

    pub fn has_next(&self) -> bool {
        self.words.len() > 0
    }
}

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub is_gui: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Filename required!"),
        };
        let is_gui = match args.next() {
            Some(ref s) if s == "--gui" => true,
            Some(_) => return Err("Wrong param"),
            None => false,
        };
        Ok(Config { file_path, is_gui })
    }
}
