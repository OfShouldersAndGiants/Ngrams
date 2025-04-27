# NLP Rust Implementations
This repository implements various NLP algorithms in Rust using the torch crate. The goal is to explore the algorithms and their mathematical foundations, starting with fundamental techniques and progressing to more advanced models.

## N-gram Language Model
The first step is implementing a statistical n-gram model, a foundational NLP technique for modeling sequences of words or characters. N-grams estimate the probability of a word given its preceding _n-1_ words, making them useful for tasks like text prediction and language modeling. This implementation will serve as a basis for understanding more complex models like RNNs and Transformers.

## Project Goal

Create a command-line application in Rust that suggests words based on a user-provided prefix. The application will:

- Start with a small, pre-defined vocabulary
- Implement two suggestion modes:
  - Unigram mode: Suggests words based on their frequency in the training data
  - Bigram mode: Suggests words based on the previous word (context-based)

This implementation will demonstrate the practical application of n-gram models for text prediction.
