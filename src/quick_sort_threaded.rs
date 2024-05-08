use std::fmt::Debug;
use std::sync::Mutex;
use std::thread;
use rayon::prelude::*;

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

struct RawWrapper<T>(*mut [T]);
unsafe impl<T> Send for RawWrapper<T>{}

fn threaded_quick_sort_2<T: PartialOrd + Debug + Send + 'static >(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }

    let pivot = pivot(vec);
    println!("{:?}", vec);

    let (left, right) = vec.split_at_mut(pivot);
    let left_wrapped = RawWrapper(left);

    unsafe{
        let left_thread = thread::spawn(move || {
            let raw_left = left_wrapped; //move the whole raw pointer
            threaded_quick_sort_2(&mut *raw_left.0); //unsafe operation
        });

        threaded_quick_sort_2(&mut right[1..]);

        left_thread.join().unwrap();
    }
}


fn quick_sort_rayon<T: PartialOrd + Debug + Send>(vec: &mut [T]){
    if vec.len() <= 1 {
        return;
    }

    let pivot = pivot(vec);
    println!("{:?}", vec);

    let (a, b) = vec.split_at_mut(pivot);

    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double_threaded_sort() {
        let mut vec = vec![4, 9, 0, 5, 3, 1, 7];
        threaded_quick_sort_2(&mut vec);
        println!("{:?}", vec);
    }  
    
    #[test]
    fn test_rayon_quick_sort() {
        let mut vec = vec![4, 9, 0, 5, 3, 1, 7];
        quick_sort_rayon(&mut vec);
        println!("{:?}", vec);
    }
}
