
pub fn linear_fib(n: usize) -> usize {
    let mut data: Vec<Option<usize>>  = Vec::with_capacity(n + 1);

    for _ in 0..n + 1 {
        data.push(None);
    }

    data[0] = Some(0);
    data[1] = Some(1);

    let mut index = 2;
    while index <= n {
        data[index] = Some(data[index - 1].expect("prior indices for linear fib should be filled") + data[index - 2].expect("prior indices for linear fib should be filled"));
        index += 1;
    }

    data[n].unwrap()
}

pub fn recursive_fib(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return recursive_fib(n - 1) + recursive_fib(n - 2);
    }
}

pub fn recursive_memoized_fib(n: usize) -> usize {
    let mut map = std::collections::HashMap::new();
    let res = recursive_memoized_fib_helper(n, &mut map);

    fn recursive_memoized_fib_helper(n: usize, map: &mut std::collections::HashMap<usize, usize>) -> usize {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            if map.contains_key(&n) {
                return map[&n];
            } else {
                let result = recursive_memoized_fib_helper(n - 1, map) + recursive_memoized_fib_helper(n - 2, map);
                map.insert(n, result);
                return result;
            }
        }
    }
    res
}