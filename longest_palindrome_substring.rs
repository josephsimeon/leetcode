#[allow(dead_code)]
fn time_limit_exceeded (s: String) -> String
{
    let mut starting_position: usize = 0;
    let mut longest_length: usize = 1;

    if s.len() <= 1
    {
        return s;
    }

    for i in 0..s.len()
    {
        let character: char = s.chars().nth(i).expect("No char found");

        for j in (i + 1)..s.len()
        {
            let next_character: char = s.chars().nth(j).expect("No char found");
            if character == next_character
            {
                let length: usize = j - i + 1;
                let mut check_string: bool = true;

                if length > 2
                {
                    let middle: usize;

                    match length % 2
                    {
                        0 => {
                            middle = length / 2;
                        },
                        _ => {
                            middle = (length - 1) / 2;
                        },
                    }

                    let mut offset: usize = 0;
                    'checking_string: for index in i..(middle + i)
                    {
                        if s.chars().nth(index) != s.chars().nth(j - offset)
                        {
                            check_string = false;
                            break 'checking_string;
                        }

                        offset += 1;
                    }
                }

                if check_string
                {
                    if length > longest_length
                    {
                        starting_position = i;
                        longest_length = length;
                    }
                }
            }
        }
    }

    (&s[starting_position..(longest_length + starting_position)]).to_string()
}

fn longest_palindrome (s: String) -> String
{
    let mut starting_position: usize = 0;
    let mut longest_length: usize = 1;

    if s.len() <= 1
    {
        return s;
    }

    let chars = s.chars().collect::<Vec<char>>();
    let mut positions: Vec<(usize, usize)> = Vec::new();
    
    for i in 0..(chars.len() - 1)
    {
        if chars[i] == chars[i + 1] { positions.push((i, i + 1)) };
    }

    for i in 1..(chars.len() - 1)
    {
        if chars[i - 1] == chars[i + 1] { positions.push((i - 1, i + 1)) };
    }

    for p in positions
    {
        let mut lower: usize = p.0;
        let mut upper: usize = p.1;

        'finding_limits: while chars[lower] == chars[upper]
        {
            if lower == 0 || upper == (chars.len() - 1)
            {
                break 'finding_limits;
            }

            if chars[lower - 1] != chars[upper + 1]
            {
                break 'finding_limits;
            }

            lower -= 1;
            upper += 1;
        }

        let length: usize = upper - lower + 1;
        if length > longest_length
        {
            starting_position = lower;
            longest_length = length;
        }
    }

    (&s[starting_position..(longest_length + starting_position)]).to_string()
}

#[test]
fn babad ()
{
    assert_eq!(longest_palindrome("babad".to_string()), "bab");
}

#[test]
fn cbbd ()
{
    assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
}

#[test]
fn a ()
{
    assert_eq!(longest_palindrome("a".to_string()), "a");
}

#[test]
fn aaaa ()
{
    assert_eq!(longest_palindrome("aaaa".to_string()), "aaaa");
}

#[test]
fn abcba ()
{
    assert_eq!(longest_palindrome("abcba".to_string()), "abcba");
}

#[test]
fn aacabdkacaa ()
{
    assert_eq!(longest_palindrome("aacabdkacaa".to_string()), "aca");
}
