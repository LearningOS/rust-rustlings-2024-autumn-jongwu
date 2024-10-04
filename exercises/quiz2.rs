// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let o: String = match command {
                Command::Uppercase => make_upper(string.to_string()),
                Command::Trim => make_trim(string.to_string()),
                Command::Append(sz) => make_append(string.to_string(), *sz),
            };
            output.push(o);
        }
        output
    }

    fn make_upper(s:String) -> String {
        let mut out = String::new();
        let mut ch: char;
        for c in s.as_bytes() {
            if *c as u8 >= 'a' as u8 && *c as u8 <= 'z' as u8 {
                ch = (*c as u8 + 'A' as u8 - 'a' as u8) as char;
            } else {
                ch = *c as char;
            }
            out.push(ch);
        }
        out
    }

    fn make_trim(s: String) -> String {
        let mut flag = 0;
        let mut out = String::new();
        let mut i = 0;
        for c in s.chars() {
            i += 1;
            if c == ' ' && flag == 0 {
                continue;
            }
            flag = 1;
            if c == ' ' && (i == s.len() || out.clone().into_bytes()[out.len() - 1] == b' ') {
                break;
            }
            out.push(c);
        }
        out
    }

    fn make_append(s: String, sz: usize) -> String {
        let mut out = String::new();
        let mut i: usize = 0;
        out.push_str(s.as_str());
        while i < sz {
            out.push_str("bar");
            i += 1;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
