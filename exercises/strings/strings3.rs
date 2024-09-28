// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let tmp: Vec<char> = input.chars().collect();
    let mut ret = String::new();
    let mut flag = 0;
    for i in 0..tmp.len() {
	if flag == 0 && tmp[i] == ' ' {
		continue;
	}
	flag = 1;
	if tmp[i] != ' ' || i < tmp.len() - 1 && tmp[i + 1] != ' ' {
		ret.push(tmp[i]);
	}
    }
    ret.to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's mdultiple ways to do this!
    let mut ret = input.to_string();
    let s = " world!".to_string();
    ret += &s;
    ret
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let tmp: Vec<char> = input.chars().collect();
    let mut ret = String::new();
    let mut j = 0;
    let mut i = 0;
    let cars = ['c', 'a', 'r', 's'];
    let balloons: Vec<char> = "balloons".to_string().chars().collect();
    while i < tmp.len() {
	if tmp[i] != 'c' {
		ret.push(tmp[i]);
		i += 1;
		continue;
	}
        while j < 4 && i < tmp.len() && tmp[i] == cars[j] {
		i += 1;
		j += 1;
	}
	if j == 4 {
		j = 0;
        	for c in &balloons {
			ret.push(*c);
		}
	} else {
		i -= j;
		ret.push(tmp[i]);
		i += 1;
	}
    }
    ret.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
