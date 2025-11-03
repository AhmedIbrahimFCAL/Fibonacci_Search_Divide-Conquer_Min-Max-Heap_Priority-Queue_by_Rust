pub fn sequential_search<T: PartialEq>(arr: &[T], value: &T) -> i32 {
    for i in 0..arr.len() {
        if arr[i] == *value {
            return i as i32;
        }
    }
    return -1;
}

pub fn recursive_sequential_search<T: PartialEq>(arr: &[T], value: &T, index: usize) -> i32 {
    if index >= arr.len() {
        return -1;
    }
    if arr[index] == *value {
        return index as i32;
    }
    recursive_sequential_search(arr, value, index + 1)
}

pub fn binary_search<T: PartialEq + PartialOrd>(arr: &[T], value: &T) -> i32 {
    let mut l = 0 as usize;
    let mut r = arr.len() - 1;
    let mut mid;
    while l <= r {
        mid = (l + r) >> 1;
        // println!("l:{}  mid:{}  r:{}",l,mid,r);
        if arr[mid] == *value {
            return mid as i32;
        }
        if *value < arr[mid] {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    return -1;
}

pub fn recursive_binary_search<T: PartialEq + PartialOrd>(arr:&[T], value:&T, l:usize, r:usize) -> i32{
    if l > r {
        return -1;
    }
    let mid = (l + r) >> 1;
    if arr[mid] == *value {
        return mid as i32;
    }
    if(*value < arr[mid]){
        return recursive_binary_search(arr, value, l, mid-1);
    }
    recursive_binary_search(arr, value, mid+1, r)
}

pub fn test_search(){
    let arr = vec![1,2,4,5,6,8,9,10,12];
    let (elem1, elem2, elem3, elem4) = (3,7,2,9);
    println!("Data: {:?}",arr);
    println!("the element {elem1} at {} using sequention search",sequential_search(&arr, &elem1));
    println!("the element {elem2} at {} using sequention search",sequential_search(&arr, &elem2));
    println!("the element {elem3} at {} using sequention search",sequential_search(&arr, &elem3));
    println!("the element {elem4} at {} using sequention search",sequential_search(&arr, &elem4));
    println!();
    println!("the element {elem1} at {} using recursion sequention search",recursive_sequential_search(&arr, &elem1,0));
    println!("the element {elem2} at {} using recursion sequention search",recursive_sequential_search(&arr, &elem2,0));
    println!("the element {elem3} at {} using recursion sequention search",recursive_sequential_search(&arr, &elem3,0));
    println!("the element {elem4} at {} using recursion sequention search",recursive_sequential_search(&arr, &elem4,0));
    println!();
    println!("the element {elem1} at {} using binary search",binary_search(&arr, &elem1));
    println!("the element {elem2} at {} using binary search",binary_search(&arr, &elem2));
    println!("the element {elem3} at {} using binary search",binary_search(&arr, &elem3));
    println!("the element {elem4} at {} using binary search",binary_search(&arr, &elem4));
    println!();
    println!("the element {elem1} at {} using recursion binary search",recursive_binary_search(&arr, &elem1,0,arr.len()-1));
    println!("the element {elem2} at {} using recursion binary search",recursive_binary_search(&arr, &elem2,0,arr.len()-1));
    println!("the element {elem3} at {} using recursion binary search",recursive_binary_search(&arr, &elem3,0,arr.len()-1));
    println!("the element {elem4} at {} using recursion binary search",recursive_binary_search(&arr, &elem4,0,arr.len()-1));
    println!();
    
}