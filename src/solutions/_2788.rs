pub struct Solution {}

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        words.iter().for_each(|ws| {
            ws.split(separator).filter(|x| !x.is_empty()).for_each(|w| {
                result.push(w.to_string());
            })
        });

        result
    }
}
