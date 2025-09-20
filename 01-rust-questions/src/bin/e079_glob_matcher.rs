// 79. How do you write a simple glob matcher supporting * and ?
// using dynamic programming, without using the regex crate? What are the limitations of glob matching?
/// Glob matcher supporting `*` and `?`
/// `*` → matches any sequence (including empty)
/// `?` → matches any single character
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let p = pattern.as_bytes();
    let t = text.as_bytes();
    let m = p.len();
    let n = t.len();

    // dp[i][j] = does pattern[0..i] match text[0..j]
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    // handle leading '*'s (they can match empty)
    for i in 1..=m {
        if p[i - 1] == b'*' {
            dp[i][0] = dp[i - 1][0];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            match p[i - 1] {
                b'?' => dp[i][j] = dp[i - 1][j - 1],
                b'*' => {
                    // * matches empty (dp[i-1][j]) or one more char (dp[i][j-1])
                    dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
                }
                _ => {
                    dp[i][j] = dp[i - 1][j - 1] && p[i - 1] == t[j - 1];
                }
            }
        }
    }

    dp[m][n]
}

fn main() {
    let tests = vec![
        ("he?lo", "hello", true),
        ("he*o", "hello", true),
        ("*", "anything", true),
        ("a*b?c", "axxxbyc", true),
        ("a*b?c", "axxbc", false),
        ("abc", "abc", true),
        ("abc", "abcd", false),
    ];

    for (pat, txt, expected) in tests {
        let result = glob_match(pat, txt);
        println!(
            "{:?} vs {:?} -> {} (expected {})",
            pat, txt, result, expected
        );
        assert_eq!(result, expected);
    }
}
