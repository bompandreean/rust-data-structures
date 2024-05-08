use my_pseudo_random::rand;
use std::{fmt::Debug, sync::Mutex, vec};

mod quick_sort_threaded;

fn main() {
    println!("Hello, world!");
}
//bublle sort ,
// [5,9, 78, 63, 1 ,0 ,58 , 4]
//[5, 9, 63, 1, 0, 58, 4 ,78]
//[5, 9, 1, 0, 58, 4, 63, 78]
//[5, 1, 0, 9, 4, 58, 63, 78]
//[1, 0 , 5, 4, 9, 58, 63, 78]
// [0, 1, 4, 5, 9, 58, 63, 78]

/*
[5, 9, 63, 1, 0, 58, 4, 78]
[5, 9, 1, 0, 58, 4, 63, 78]
[5, 1, 0, 9, 4, 58, 63, 78]
[1, 0, 5, 4, 9, 58, 63, 78]
[0, 1, 4, 5, 9, 58, 63, 78]
[0, 1, 4, 5, 9, 58, 63, 78]
*/

fn bubble_sort_clasic<T: PartialOrd + std::fmt::Debug>(v: &mut [T]) {
    let mut swaped = true;
    while swaped {
        swaped = false;
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                swaped = true;
            }
        }
        println!("{:?}", v);
    }
}

fn divide_and_conquer<T: PartialOrd + Debug>(mut vec: Vec<T>) -> Vec<T> {
    println!("{:?}", vec);

    if vec.len() <= 1 {
        return vec;
    }

    //initialize result
    let mut res = Vec::with_capacity(vec.len());
    //divede
    let b: Vec<T> = vec.split_off(vec.len() / 2);
    let a: Vec<T> = divide_and_conquer(vec);
    let b: Vec<T> = divide_and_conquer(b);

    //sort
    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();
    let mut a_peek = a_iter.next();
    let mut b_peek = b_iter.next();

    loop {
        match (&a_peek, &b_peek) {
            (Some(a_value), Some(b_value)) => {
                if a_value < b_value {
                    res.push(a_peek.take().unwrap());
                    println!("resulted in a<b: {:?}", res);
                    a_peek = a_iter.next();
                } else {
                    res.push(b_peek.take().unwrap());
                    println!("resulted in a>b: {:?}", res);
                    b_peek = b_iter.next();
                }
            }
            (None, Some(_)) => {
                res.push(b_peek.take().unwrap());
                res.extend(b_iter);

                return res;
            }
            (Some(_), None) => {
                res.push(a_peek.take().unwrap());
                res.extend(a_iter);

                return res;
            }
            _ => {
                return res;
            }
        }
    }
}

//Quiqk sort
fn pivot<T: PartialOrd + Debug>(vec: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..vec.len() {
        if vec[i] < vec[p] {
            vec.swap(p + 1, i);
            // println!("after swap p+1 and i: {:?}", vec);
            vec.swap(p, p + 1);
            // println!("after swap p and p+1: {:?}", vec);
            p += 1;
        }
    }

    p
}

fn quick_sort<T: PartialOrd + Debug>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }

    let pivot = pivot(vec);
    println!("after pivot = {} : {:?}", pivot, vec);

    let (a, b) = vec.split_at_mut(pivot);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

fn pivot_randomly<T: PartialOrd + Debug>(vec: &mut [T]) -> usize {
    let mut p = rand(vec.len() - 1);
    println!("random pivot = {}", p);
    for i in 1..vec.len() {
        if vec[i] < vec[p] {
            vec.swap(p + 1, i);
            // println!("after swap p+1 and i: {:?}", vec);
            vec.swap(p, p + 1);
            // println!("after swap p and p+1: {:?}", vec);
            p += 1;
        }
    }

    p
}

fn quick_sort_randomly<T: PartialOrd + Debug>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }

    let pivot = pivot_randomly(vec);
    println!("after random pivot = {} : {:?}", pivot, vec);

    let (a, b) = vec.split_at_mut(pivot);
    quick_sort_randomly(a);
    quick_sort_randomly(&mut b[1..]);
}

struct RawSend<T>(*mut [T]);
unsafe impl<T> Send for RawSend<T> {}

fn threaded_quick_sort<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    println!("{:?}", v);
    if v.len() <= 1 {
        return;
    }

    let pivot = pivot(v);

    let (a, b) = v.split_at_mut(pivot);

    let raw_pointer_to_a: *mut [T] = a as *mut [T];
    let raw_pointer_wrapped = RawSend(raw_pointer_to_a);

    unsafe {
        let handle = std::thread::spawn(move || {
            let raw_s = raw_pointer_wrapped; // moves the full RawSend into threaded scope
            threaded_quick_sort(&mut *raw_s.0);
        });

        threaded_quick_sort(&mut b[1..]);

        handle.join().ok();
    }
}

#[cfg(test)]
mod test {
    use crate::{
        bubble_sort_clasic, divide_and_conquer, quick_sort, quick_sort_randomly,
        threaded_quick_sort,
    };

    #[test]
    fn test_bubble_sort() {
        let mut vec = vec![5, 9, 78, 63, 1, 0, 58, 4];
        bubble_sort_clasic(&mut vec);
    }

    #[test]
    fn test_divide_and_conquer() {
        let vec = vec![5, 9, 78, 63, 1, 0, 58, 4];
        let result = divide_and_conquer(vec);
        println!("result {:?}", result);
    }

    #[test]
    fn test_quick_sort() {
        let mut vec = vec![4, 9, 0, 5, 3, 1, 7];
        // pivot(&mut vec);
        quick_sort(&mut vec);
        // quick_sort_randomly(&mut vec);
    }

    #[test]
    fn test_quick_sort_randomly() {
        let mut vec = vec![4, 9, 0, 5, 3, 1, 7];
        quick_sort_randomly(&mut vec);
    }

    #[test]
    fn test_threaded_quick_sort() {
        let mut vec = vec![4, 9, 0, 5, 3, 1, 7];
        threaded_quick_sort(&mut vec);
        println!("{:?}", vec);
    }
}
