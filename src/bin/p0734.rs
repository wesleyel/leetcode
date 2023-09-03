/**
 * [734] Sentence Similarity
 *
 * We can represent a sentence as an array of words, for example, the sentence "I am happy with leetcode" can be represented as arr = ["I","am",happy","with","leetcode"].
 *
 * Given two sentences sentence1 and sentence2 each represented as a string array and given an array of string pairs similarPairs where similarPairs[i] = [xi, yi] indicates that the two words xi and yi are similar.
 *
 * Return true if sentence1 and sentence2 are similar, or false if they are not similar.
 *
 * Two sentences are similar if:
 *
 *
 * 	They have the same length (i.e., the same number of words)
 * 	sentence1[i] and sentence2[i] are similar.
 *
 *
 * Notice that a word is always similar to itself, also notice that the similarity relation is not transitive. For example, if the words a and b are similar, and the words b and c are similar, a and c are not necessarily similar.
 *
 *
 * Example 1:
 *
 * Input: sentence1 = ["great","acting","skills"], sentence2 = ["fine","drama","talent"], similarPairs = [["great","fine"],["drama","acting"],["skills","talent"]]
 * Output: true
 * Explanation: The two sentences have the same length and each word i of sentence1 is also similar to the corresponding word in sentence2.
 *
 *
 * Example 2:
 *
 * Input: sentence1 = ["great"], sentence2 = ["great"], similarPairs = []
 * Output: true
 * Explanation: A word is similar to itself.
 *
 *
 * Example 3:
 *
 * Input: sentence1 = ["great"], sentence2 = ["doubleplus","good"], similarPairs = [["great","doubleplus"]]
 * Output: false
 * Explanation: As they don't have the same length, we return false.
 *
 *
 *
 * Constraints:
 *
 *
 * 	1 <= sentence1.length, sentence2.length <= 1000
 * 	1 <= sentence1[i].length, sentence2[i].length <= 20
 * 	sentence1[i] and sentence2[i] consist of English letters.
 * 	0 <= similarPairs.length <= 1000
 * 	similarPairs[i].length == 2
 * 	1 <= xi.length, yi.length <= 20
 * 	xi and yi consist of lower-case and upper-case English letters.
 * 	All the pairs (xi, yi) are distinct.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/sentence-similarity/
// discussion: https://leetcode.cn/problems/sentence-similarity/discussion/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn are_sentences_similar(
        sentence1: Vec<String>,
        sentence2: Vec<String>,
        similar_pairs: Vec<Vec<String>>,
    ) -> bool {
        if sentence1.len() != sentence2.len() {
            return false;
        };
        let s = similar_pairs
            .iter()
            .map(|x| (&x[0], &x[1]))
            .collect::<std::collections::HashSet<_>>();
        sentence1
            .iter()
            .zip(sentence2.iter())
            .all(|(w1, w2)| s.contains(&(w1, w2)) || s.contains(&(w2, w1)) || w1.len() == w2.len())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_734() {
        assert_eq!(
            Solution::are_sentences_similar(
                vec![
                    "great".to_string(),
                    "acting".to_string(),
                    "skills".to_string()
                ],
                vec![
                    "fine".to_string(),
                    "drama".to_string(),
                    "talent".to_string()
                ],
                vec![
                    vec!["great".to_string(), "fine".to_string()],
                    vec!["drama".to_string(), "acting".to_string()],
                    vec!["skills".to_string(), "talent".to_string()]
                ]
            ),
            true
        );
        assert_eq!(
            Solution::are_sentences_similar(
                vec!["great".to_string()],
                vec!["great".to_string()],
                vec![]
            ),
            true
        );
        assert_eq!(
            Solution::are_sentences_similar(
                vec!["great".to_string()],
                vec!["doubleplus".to_string(), "good".to_string()],
                vec![vec!["great".to_string(), "doubleplus".to_string()]]
            ),
            false
        );
    }
}
