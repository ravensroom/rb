pub mod employee {
    pub fn add_employee() {
        use std::collections::HashMap;
        let mut company: HashMap<String, Vec<String>> = HashMap::new();
        loop {
            // retrieve user input: "Add {name} to {department}"
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.trim() == "/print" {
                // print people in each department sorted alphabetically
                let departments = company.keys().collect::<Vec<&String>>();
                let mut departments = departments.clone();
                departments.sort();
                for department in departments {
                    let people = company.get(department).unwrap();
                    println!("{}: {:?}", department, people);
                }
                break;
            }
            match get_new_employee(&input) {
                Some(employee) => {
                    let department = company.entry(employee.department.clone()).or_insert(vec![]);
                    department.push(employee.name.clone());
                }
                None => {
                    println!("Invalid input. Please try again.");
                }
            }
        }
    }

    struct Employee {
        name: String,
        department: String,
    }

    fn get_new_employee(input: &String) -> Option<Employee> {
        let splitted = &input.split_whitespace().collect::<Vec<&str>>();
        if splitted.len() != 4 || splitted[0] != "Add" || splitted[2] != "to" {
            return None;
        }
        return Some(Employee {
            name: splitted[1].to_string(),
            department: splitted[3].to_string(),
        });
    }
}

pub mod pig_latin {
    pub fn test_pig_latin() {
        let empty = to_pig_latin("");
        assert_eq!(empty, "");

        let pig_latin = to_pig_latin("apple");
        assert_eq!(pig_latin, "apple-hay");

        let pig_latin = to_pig_latin("banana");
        assert_eq!(pig_latin, "anana-bay");
    }

    fn to_pig_latin(word: &str) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let first_char = match word.chars().next() {
            Some(c) => c,
            None => return String::new(),
        };
        if vowels.contains(&first_char) {
            format!("{}-hay", word)
        } else {
            let mut chars = word.chars();
            chars.next();
            let rest = chars.collect::<String>();
            format!("{}-{}ay", rest, first_char)
        }
    }
}

pub mod mode {
    pub fn test_find_mode() {
        let mode = find_mode(&vec![5, 3, 6, 9, 7, 6, 6, 6, 6, 2]);
        assert_eq!(mode, 6);
    }

    fn find_mode(numbers: &Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for &num in numbers {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }

        let mut mode = 0;
        let mut max_count = 0;
        for (&num, &count) in map.iter() {
            if count > max_count {
                mode = num;
                max_count = count;
            }
        }

        mode
    }
}

pub mod median {
    pub fn test_find_median() {
        let median = find_median(&vec![5.0, 3.0, 6.0, 9.0, 7.0]);
        assert_eq!(median, 6.0);

        let median = find_median(&vec![5.0, 3.0, 6.0, 9.0, 7.0, 8.0]);
        assert_eq!(median, 6.5);
    }

    fn find_median(numbers: &Vec<f64>) -> f64 {
        let mut numbers = numbers.clone();
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = numbers.len();
        if len % 2 == 0 {
            let mid = len / 2;
            (numbers[mid - 1] + numbers[mid]) / 2.0
        } else {
            numbers[len / 2]
        }
    }
}
