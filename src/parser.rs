pub fn parse_command(command: &str) -> Vec<String> {
    // TODO: improve this to a good state machine
    let mut parts = Vec::new();
    let mut in_quotes = false;
    let mut in_word = false;
    let mut start_idx = 0;
    let mut next_char_escaped = false;
    let mut buffer = String::new();

    for (i, c) in command.chars().enumerate() {
        if in_quotes {
            if c == '"' {
                parts.push(String::from(&command[start_idx..i]));
                in_quotes = false;
                start_idx = i + 1;
            }
        } else {
            if c == '\\' && !next_char_escaped {
                buffer.push_str(&command[start_idx..i]);
                buffer.push_str(&command[(i+1)..(i+2)]);
                next_char_escaped = true;
                start_idx = i + 2;
            } else if c == '"' && !next_char_escaped {
                in_quotes = true;
                start_idx = i + 1;
                next_char_escaped = false;
            } else if !next_char_escaped && c != ' ' && !in_word {
                start_idx = i;
                in_word = true;
                next_char_escaped = false;
            } else if !next_char_escaped && c == ' ' && in_word {
                parts.push(format!("{}{}", buffer, String::from(&command[start_idx..i])));
                buffer = String::new();
                in_word = false;
                next_char_escaped = false;
            }
        }
    }

    let end = command.len();
    if start_idx != end {
        parts.push(format!("{}{}", buffer, String::from(&command[start_idx..end])));
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let result = parse_command("program execute something");

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "program");
        assert_eq!(result[1], "execute");
        assert_eq!(result[2], "something");
    }

    #[test]
    fn test_parse_command_with_quoted_args() {
        let result = parse_command("program \"execute something\" and \"something else\"");

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "program");
        assert_eq!(result[1], "execute something");
        assert_eq!(result[2], "and");
        assert_eq!(result[3], "something else");
    }

    #[test]
    fn test_parse_command_with_spaced_arg() {
        let result = parse_command("program execute\\ something");

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "program");
        assert_eq!(result[1], "execute something");
    }

    #[test]
    fn test_bash_command() {
        let result = parse_command("bash -c \"exit 1\"");

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "bash");
        assert_eq!(result[1], "-c");
        assert_eq!(result[2], "exit 1");
    }

    #[test]
    fn test_parse_virtualenv_tox_command() {
        let result = parse_command("bash -c \"virtualenv env && source env/bin/activate && tox\"");

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "bash");
        assert_eq!(result[1], "-c");
        assert_eq!(result[2], "virtualenv env && source env/bin/activate && tox");
    }
}
