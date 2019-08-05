use itertools::equal;
use itertools::sorted;
use itertools::Itertools;
use quicli::prelude::*;
use structopt::StructOpt;

/// Find anagrams for a word
#[derive(Debug, StructOpt)]
struct Cli {
    /// Word to find anagrams for
    #[structopt(long = "word", short = "w")]
    word: String,
    /// Dictionary to find anagrams in
    #[structopt(long = "dictionary", short = "d")]
    dict: String,
    /// Number of anagrams to return
    #[structopt(long = "number", short = "n", default_value = "0")]
    num: usize,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("anagrams")?;

    let num = args.num;
    let num_string = if num == 0 {
        "all".to_string()
    } else {
        num.to_string()
    };

    let word = args.word;
    let dict = args.dict;

    info!(
        "Finding {} anagrams for {:?} in {:?}",
        num_string, word, dict
    );

    let dict = read_file(dict)?;
    let sorted_word = sorted(word.chars());
    let anagrams = dict
        .lines()
        .map(|line| line.to_ascii_lowercase())
        .unique()
        .filter(|candidate| equal(sorted(candidate.chars()), sorted_word.clone()))
        .filter(|anagram| *anagram != word);

    let num_anagrams = anagrams.clone().count();
    if num_anagrams > 0usize {
        let num = if num > 0 { num } else { num_anagrams };
        anagrams
            .take(num)
            .for_each(|anagram| println!("{}", anagram));
    } else {
        println!("No anagrams found.")
    }

    Ok(())
}
