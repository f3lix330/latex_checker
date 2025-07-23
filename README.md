# Latex Checker

Checks every .tex file in a directory (incl. sub-directories) for capitalized words that should be in the glossary.

## Usage
```
latex_checker [directory]
```
To exclude files or directories, create a file "exclude_files.txt" in [directory] and enter one file or directory name per row.
To allow certain words, create a file "allow_words.txt" in [directory] and enter one word per row.
