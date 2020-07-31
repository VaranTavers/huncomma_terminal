use huncomma::model::Mistake;

pub fn combine_mistakes(m1: &Mistake, m2: &Mistake) -> Mistake {
    Mistake::new_dyn(
        format!("{}\n\t{}", m1.get_str(), m2.get_str()),
        m1.prob + (1.0 - m1.prob) * m2.prob
    )
}

// I'm sorry for this. I hope I can make it more pretty.
pub fn merge_mistakes(mistake_vec: Vec<Vec<(usize, usize, Mistake)>>) -> Vec<(usize, usize, Mistake)> {
    if mistake_vec.is_empty() {
        return Vec::new();
    }
    let mut iter = mistake_vec.iter();
    let mut first = iter.next().unwrap().clone();
    let mut result = Vec::new();
    for second in iter {
        let mut first_iter = first.iter();
        let mut second_iter = second.iter();

        let mut first_obj = first_iter.next();
        let mut second_obj = second_iter.next();

        while first_obj.is_some() || second_obj.is_some() {
            if first_obj.is_none() {
                result.push(second_obj.unwrap().clone());
                second_obj = second_iter.next();
            } else if second_obj.is_none() {
                result.push(first_obj.unwrap().clone());
                first_obj = first_iter.next();
            } else {
                let (r1, c1, m1) = first_obj.unwrap();
                let (r2, c2, m2) = second_obj.unwrap();

                if r1 == r2 && c1 == c2 {
                    result.push((*r1, *c1, combine_mistakes(m1, m2)));
                    first_obj = first_iter.next();
                    second_obj = second_iter.next();
                } else if r1 < r2 || (r1 == r2 && c1 < c2) {
                    result.push(first_obj.unwrap().clone());
                    first_obj = first_iter.next();
                } else {
                    result.push(second_obj.unwrap().clone());
                    second_obj = second_iter.next();
                }
            }
        }

        first = result;
        result = Vec::new();
    }

    first
}
