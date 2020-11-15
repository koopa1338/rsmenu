/*
 * Example how to get all binaries from PATH in bash:
 *
    #!/bin/bash
    IFS=: read -ra dirs_in_path <<< "$PATH"

    for dir in "${dirs_in_path[@]}"; do
        for file in "$dir"/*; do
            [[ -x $file && -f $file ]] && printf '%s\n' "${file##*/}"
        done
    done
 */

use std::collections::BTreeMap;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;


fn main() {
    let input = vec!["test", "test2", "not a test", "help!", "#+*@€"];
    let matcher = SkimMatcherV2::default();
    let mut score_map = BTreeMap::new();
    /*
     * need a structure or type that we can sort based on the score of the entries and let us add
     * entries with the same score. BTreeMap can't use the score as a key, as entries with the same
     * score get overwritten.
     */
    for entry in input {
        score_map.insert(matcher.fuzzy_match(entry, "test").unwrap_or(0), entry);
    }
    print!("{:?}", score_map);
}
