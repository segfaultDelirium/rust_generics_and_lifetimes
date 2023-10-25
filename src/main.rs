use std::collections::linked_list::Iter;
use std::convert;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
struct AgeAndHeight<'a> {
    age: i32,
    height: i32,
    name: &'a str,
}

fn find_largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for el in list {
        if el > largest {
            largest = el
        }
    }
    largest
}

// fn head_tail<T>(list: &mut Iter<T>) -> (Option<&T>, Iter<T>) {
//     (None, vec![].into_iter())
// }

fn find_largest(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];
    for el in list {
        if el > largest {
            largest = el;
        }
    }
    largest
}

fn find_largest2(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for el in list {
        if el > largest {
            largest = el
        }
    }
    largest
}

fn head_tail<T>(list: &Vec<T>) -> (Option<&T>, Vec<&T>) {
    let head = list.get(0);
    let tail: Vec<&T> = list.into_iter().skip(1).collect();
    (head, tail)
}

fn head_tail_can_error<T>(list: &Vec<T>) -> (&T, Vec<&T>) {
    let head = list.get(0).unwrap();
    let tail: Vec<&T> = list.into_iter().skip(1).collect();
    (head, tail)
}

fn skip_double_reference<'a, T>(list: Vec<&&'a T>) -> Vec<&'a T> {
    list.into_iter().map(|x| *x).collect::<Vec<&'a T>>()
}

fn find_largest_generic_rec<'a, T: std::cmp::PartialOrd>(list: &Vec<&'a T>) -> &'a T {
    fn find_largest_generic_rec_inner<'a, T: std::cmp::PartialOrd>(
        list: &Vec<&'a T>,
        largest: &'a T,
    ) -> &'a T {
        if list.is_empty() {
            return largest;
        };
        let (head, tail) = head_tail_can_error(&list);
        let new_largest = if *head > largest { *head } else { largest };
        find_largest_generic_rec_inner(&skip_double_reference(tail), new_largest)
    }
    find_largest_generic_rec_inner(list, list[0])
}

fn print_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list2 = [34, 50, 25, 100, 65];
    let largest = find_largest(&number_list);
    println!("The largest number is {}", largest);

    // following will work since &number_list is a &[i32]
    let largest2 = find_largest2(&number_list);
    println!("The largest2 number is {}", largest2);

    let largest3 = find_largest2(&number_list2);
    println!("The largest3 number is {}", largest3);

    // following will not work since &number_list2 is not a vector,
    // let largest4 = find_largest(&number_list2);
    // println!("The largest4 number is {}", largest4);

    let number_list3 = [34.0, 50.0, 25.0, 100.0, 65.0];
    let largest4 = find_largest_generic(&number_list3);
    println!("largest is {}", largest4);
    let largest5 = find_largest_generic(&number_list2);
    println!("largest is {}", largest5);

    let number_list_ref = get_vector_with_references_generic(&number_list);

    let largest6 = find_largest_generic_rec(&number_list_ref);
    println!("largest6: {}", largest6);
}

fn get_vector_with_references_generic<T>(list: &Vec<T>) -> Vec<&T> {
    let ref_vector: Vec<&T> = list.into_iter().collect();
    ref_vector
}

fn find_oldest<'a>(list: Vec<&'a AgeAndHeight<'a>>) -> &'a AgeAndHeight<'a> {
    let mut oldest = list[0];
    for el in list {
        if el.age > oldest.age {
            oldest = &el
        }
    }
    oldest
}

fn find_oldest_in_iterator<'a>(iterator: &mut IntoIter<AgeAndHeight<'a>>) -> AgeAndHeight<'a> {
    let mut oldest = iterator.next().unwrap();
    iterator.for_each(|x| {
        if x.age > oldest.age {
            oldest = x
        };
    });

    oldest
}

fn get_vector_with_references<'a>(list: &'a Vec<AgeAndHeight<'a>>) -> Vec<&'a AgeAndHeight> {
    let ref_vector: Vec<&AgeAndHeight> = list.into_iter().collect();
    ref_vector
}

fn main() {
    print_largest();

    let ages_and_heights = vec![
        AgeAndHeight {
            age: 65,
            height: 113,
            name: "adam",
        },
        AgeAndHeight {
            age: 20,
            height: 193,
            name: "bob",
        },
        AgeAndHeight {
            age: 30,
            height: 143,
            name: "michael",
        },
        AgeAndHeight {
            age: 90,
            height: 203,
            name: "julia",
        },
        AgeAndHeight {
            age: 13,
            height: 183,
            name: "jacob",
        },
    ];

    // let ref_vector: Vec<&AgeAndHeight> = ages_and_heights.into_iter().map(|x| &x).collect();
    let ref_vector = get_vector_with_references(&ages_and_heights);

    let oldest = find_oldest(ref_vector);
    println!("oldest: {:?}", oldest);

    let mut iterator = ages_and_heights.into_iter();
    let oldest_from_iterator = find_oldest_in_iterator(&mut iterator);
    println!("oldest_from_iterator: {:?}", oldest_from_iterator);
    println!("bye!");
}
