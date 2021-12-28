impl Solution {
    pub fn remove_vowels(mut s: String) -> String {
        s.replace(&['a', 'e', 'i', 'o', 'u'][..], "")
    }
}
