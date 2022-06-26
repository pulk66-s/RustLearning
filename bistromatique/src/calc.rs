fn convert_nb(a: &str, b: &str) -> Option<(i128, i128)>
{
    let nb1 = a.parse::<i128>();
    let nb2 = b.parse::<i128>();
    return match nb1.is_err() || nb2.is_err() {
        true => None,
        false => Some((nb1.unwrap(), nb2.unwrap()))
    };
}

fn base_calc(a: &str, b: &str, func: fn(i128, i128) -> i128) -> Option<i128>
{
    return match convert_nb(a, b) {
        None => None,
        Some((x, y)) => Some(func(x, y))
    }
}

fn add_calc(a: i128, b: i128) -> i128
{
    return a + b
}

fn sub_calc(a: i128, b: i128) -> i128
{
    return a - b
}

fn mult_calc(a: i128, b: i128) -> i128
{
    return a * b
}

pub fn add(a: &str, b: &str) -> Option<i128>
{
    return base_calc(a, b, add_calc)
}

pub fn sub(a: &str, b: &str) -> Option<i128>
{
    return base_calc(a, b, sub_calc)
}

pub fn mult(a: &str, b: &str) -> Option<i128>
{
    return base_calc(a, b, mult_calc)
}
