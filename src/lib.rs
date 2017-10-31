// compute a Knuth-Morris-Pratt shift table for each element of the pattern `x`.
// !0 is a sentinel value.
fn prepare_kmp<T>(x: &[T], next: &mut [usize])
    where T: PartialEq
{
    let mut i = 0;
    let mut j = !0;
    next[0] = !0;
    while i < x.len() {
        while let Some(&next_j) = next.get(j) { // .get(!0) -> None
            if x[i] == x[j] {
                break;
            }
            j = next_j;
        }
        i += 1;
        j = j.wrapping_add(1);
        if i != x.len() && x[i] == x[j] {
            next[i] = next[j];
        } else {
            next[i] = j;
        }
    }
}

const STACK_NEXT_SIZE: usize = 16;

/// Search for the first occurence of `pattern` as a substring of `text`,
/// if any. Return the start of the substring as an offset from the start of
/// the text inside a `Some`. If the patter is not found, return `None`.
pub fn knuth_morris_pratt<T>(text: &[T], pattern: &[T]) -> Option<usize>
    where T: PartialEq
{
    // empty pattern is a trivial match
    if pattern.len() == 0 {
        return Some(0);
    }

    if pattern.len() >= text.len() {
        return if pattern == text {
            Some(0)
        } else {
            None
        };
    }

    // use the stack for short patterns
    let mut next_vec;
    let mut next_stack = [0; STACK_NEXT_SIZE];
    let next;
    if pattern.len() > STACK_NEXT_SIZE - 1 {
        next_vec = vec![0; pattern.len() + 1];
        next = &mut next_vec[..];
    } else {
        next = &mut next_stack[..];
    }
    prepare_kmp(pattern, next);
    
    let mut i = 0;
    let mut j = 0;
    while j < text.len() {
        while let Some(&next_i) = next.get(i) { // sentinel .get(!0) -> None
            if pattern[i] == text[j] {
                break;
            }
            i = next_i;
        }
        i = i.wrapping_add(1);
        j += 1;
        if i >= pattern.len() {
            return Some(j - i);
            // i = next[i]; to continue searching after first match
        }
    }
    None
}


#[test]
fn test_stuff() {
    let body = "G";
    let pattern = "GCAGAGAG";
    knuth_morris_pratt(body.as_bytes(), pattern.as_bytes());
    macro_rules! test {
        ($body:expr, $pattern:expr) => {
            assert_eq!($body.find($pattern),
                       knuth_morris_pratt($body.as_bytes(), $pattern.as_bytes()),
                       "assertion failed for body={}, pattern={}",
                       $body, $pattern)
        }
    }
    test!("xyz", "a");
    test!("xyz", "x");
    test!("xyz", "y");
    test!("xyz", "z");
    test!("substrinstring", "string");
    test!("abcαaαβγ", "αβ");

    let result = knuth_morris_pratt(&[1729, 1, 1729, 3, 4], &[1729, 3]);
    println!("Found = {:?}", result);
    assert_eq!(result, Some(2));
}
