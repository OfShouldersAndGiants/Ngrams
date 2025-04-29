use std::collections::HashMap;

pub struct NGramModel {
    pub unigram: HashMap<String, usize>,
    pub bigram: HashMap<(String, String), usize>,
    pub trigram: HashMap<(String, String, String), usize>,
}

impl NGramModel {
    pub fn train(text: &str) -> Self {
        let tokens = Self::tokenize(text);
        let mut unigram: HashMap<String, usize> = HashMap::new();
        let mut bigram: HashMap<(String, String), usize> = HashMap::new();
        let mut trigram: HashMap<(String, String, String), usize> = HashMap::new();

        // Walk through every token, keeping track of its index so we can refer to
        // the word that came immediately before it (needed for bigram counts).
        for (i, token) in tokens.iter().enumerate() {
            // ---------------------------- UNIGRAM ---------------------------------
            // Bump the occurrence count for the current word. If the word hasn’t
            // been seen yet, `entry(...).or_insert(0)` inserts it with count 0,
            // then we increment it to 1.
            *unigram.entry(token.clone()).or_insert(0) += 1;

            // ----------------------------- BIGRAM ---------------------------------
            // Only once we’re past the very first word do we have a valid “previous
            // token”. At that point we form the pair (prev, current) and update
            // its frequency. This lets us answer questions like “after ‘mountain’
            // how often does ‘climb’ appear?”
            if i > 0 {
                let prev = tokens[i - 1].clone();
                *bigram.entry((prev, token.clone())).or_insert(0) += 1;
            }

            // ----------------------------- TRIGRAM ---------------------------------
            // Only once we’re past the second word do we have a valid “previous
            // two words”. At that point we form the pair (prev1, prev2, current) and update
            // its frequency. This lets us answer questions like “after ‘rock climbing’
            // how often does ‘trip’ appear?”
            if i > 1 {
                let ante_prev = tokens[i - 2].clone();
                let prev = tokens[i - 1].clone();
                *trigram.entry((ante_prev, prev, token.clone())).or_insert(0) += 1;
            }
        }

        NGramModel {
            unigram,
            bigram,
            trigram,
        }
    }

    // ----------------------------- H E L P E R S  ---------------------------------
    /// A simple whitespace tokenizer that also strips basic punctuation and lowercases tokens.
    pub fn tokenize(text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|t| {
                t.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase()
            })
            .filter(|t| !t.is_empty())
            .collect()
    }
}

impl NGramModel {
    /// Suggest completions using unigram counts.
    ///
    /// Steps:
    /// 1. Normalise the incoming prefix to lowercase so search is case-insensitive.
    /// 2. Iterate over every (word, count) pair in the unigram table.
    /// 3. Keep only those words whose *full* string starts with the given prefix.
    /// 4. Map the filtered iterator to an owned `(String, usize)` tuple because
    ///    the original keys & values are borrowed from the HashMap.
    /// 5. Collect into a `Vec`, then sort
    /// 6.
    pub fn suggest_unigram(&self, input: &str) -> (String, usize) {
        // 1. Lower‑case the input once so we don’t repeat this inside the filter.
        let input: String = input
            .to_lowercase()
            .split_whitespace()
            .last()
            .unwrap_or("")
            .to_string();
        // 2‑4. Filter on prefix match, clone the key, copy the count; collect to Vec.
        let mut candidates: Vec<(String, usize)> = self
            .unigram
            .iter()
            .filter(|(word, _)| word.starts_with(&input))
            .map(|(word, count)| (word.clone(), *count))
            .collect();

        // 5. Sort by count descending (`b.1.cmp(&a.1)` is reverse order because `.sort_by` is ascending by default).
        candidates.sort_by(|a, b| b.1.cmp(&a.1));

        // 6. Return
        let best_candidate = candidates.first().cloned().unwrap_or((String::new(), 0));

        return best_candidate;
    }

    pub fn suggest_bigram(&self, input: &str) -> (String, usize) {
        let tokenized_input = Self::tokenize(input);

        if tokenized_input.len() < 2 {
            return (String::new(), 0);
        };

        let current: String = tokenized_input[tokenized_input.len() - 1].clone();
        let previous: String = tokenized_input[tokenized_input.len() - 2].clone();

        // 2‑4. Filter on prefix match, clone the key, copy the count; collect to Vec.
        let mut candidates: Vec<(String, usize)> = self
            .bigram
            .iter()
            .filter(|((previus_word, current_word), _)| {
                previus_word == &previous && current_word.starts_with(&current)
            })
            .map(|((_, current_word), count)| (current_word.clone(), *count))
            .collect();

        candidates.sort_by(|a, b| b.1.cmp(&a.1));

        let best_candidate = candidates.first().cloned().unwrap_or((String::new(), 0));

        return best_candidate;
    }

    pub fn suggest_trigram(&self, input: &str) -> (String, usize) {
        let tokenized_input = Self::tokenize(input);

        if tokenized_input.len() < 3 {
            return (String::new(), 0);
        };

        let current: String = tokenized_input[tokenized_input.len() - 1].clone();
        let previous: String = tokenized_input[tokenized_input.len() - 2].clone();
        let ante_previous: String = tokenized_input[tokenized_input.len() - 3].clone();

        // 2‑4. Filter on prefix match, clone the key, copy the count; collect to Vec.
        let mut candidates: Vec<(String, usize)> = self
            .trigram
            .iter()
            .filter(|((ante_prev_word, prev_word, current_word), _)| {
                ante_prev_word == &ante_previous
                    && prev_word == &previous
                    && current_word.starts_with(&current)
            })
            .map(|((_, _, current), count)| (current.clone(), *count))
            .collect();

        candidates.sort_by(|a, b| b.1.cmp(&a.1));

        let best_candidate = candidates.first().cloned().unwrap_or((String::new(), 0));

        return best_candidate;
    }
}
