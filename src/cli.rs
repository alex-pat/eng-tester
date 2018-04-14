use eng_tester::{Context, Word};
use std::cmp::max;
use std::io;
use std::io::prelude::*;

pub fn run(mut context: Context) {
    let stdin = io::stdin();
    let mut input = String::new();
    print_intro(&context);
    while context.has_next() {
        wait_enter();
        clear_screen();
        print_question(&context);
        input.clear();
        if let Err(e) = stdin.read_line(&mut input) {
            eprintln!("Input error! {}", e);
            break;
        }
        let try = input.trim();
        if try.is_empty() {
            break;
        }
        if context.check(&try) {
            println!("Yes!");
        } else {
            println!("No!");
            if let Some(error) = context.last_error() {
                print_words(&context, &[error]);
            }
        }
    }
    clear_screen();
    print_words(
        &context,
        context
            .get_errors()
            .iter()
            .map(|x| x)
            .collect::<Vec<_>>()
            .as_slice(),
    );
}

fn print_question(context: &Context) {
    print!(
        "ENG: {}/{} right answers

    {} : {}

    {} ? ",
        context.correct_count(),
        context.answers_count(),
        context.get_guess_form(),
        context.get_guess(),
        context.get_check_form()
    );
    ::std::io::stdout().flush().expect("Cannot flush stdout!");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn wait_enter() {
    let mut stdin = ::std::io::stdin();
    let mut buff = [0; 1];
    while buff[0] != b'\n' {
        stdin.read(&mut buff).expect("Stdin error");
    }
}

fn print_intro(context: &Context) {
    clear_screen();
    println!(
        "
                E N G L I S H
    Written by Alexander Pateenok, 2017
    From 450501 with love,

         {} words in {}

    Press Enter to start (empty string to exit)
    ",
        context.words_count(),
        ::env::args().nth(1).expect("Cannot find first arg")
    );
}

fn print_words(context: &Context, words: &[&Word]) {
    let lengths: Vec<usize> = context
        .header
        .0
        .iter()
        .enumerate()
        .map(|(i, f)| {
            max(
                f.chars().count(),
                words
                    .iter()
                    .map(|w| w.0[i].chars().count())
                    .max()
                    .expect("Empty table"),
            )
        })
        .collect();
    let print_word_line = |word: &Word| {
        println!(
            "{}",
            word.0
                .iter()
                .enumerate()
                .map(|(i, w)| format!("{:1$}", w, lengths[i]))
                .collect::<Vec<String>>()
                .join(" | ")
        );
    };
    print_word_line(&context.header);
    println!(
        "{:-<1$}",
        "",
        lengths.iter().sum::<usize>() + (lengths.len() - 1) * 3
    );
    for word in words {
        print_word_line(&word);
    }
}
