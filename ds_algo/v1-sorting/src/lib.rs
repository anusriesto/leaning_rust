use std::fmt::Debug; //used for printing the array or type

pub fn bubble_sort<T:PartialOrd + Debug>(v:&mut [T]){
    for p in 0..v.len(){
        let mut sorted =true;
        for i in 0..(v.len()-1)-p{
            if v[i]>v[i+1]{
                v.swap(i,i+1);
                sorted=false;
            }
        }
        println!("{:?}",v); //we give question mark to print the debug method
        if sorted{
            return;
        }
    }
}

//lets do a bonus challenge to improve complexity 
pub fn bubble_sort_comp<T: PartialOrd + Debug>(v: &mut [T]) {
    let mut n = v.len();
    while n > 0 {
        let mut new_n = 0;
        for i in 1..n {
            if v[i - 1] > v[i] {
                v.swap(i - 1, i);
                new_n = i;
            }
        }
        println!("{:?}", v); // Print the array after each pass
        n = new_n; // Limit the next pass to the unsorted portion
    }
}

pub fn merge_sort<T:PartialOrd+Debug>(mut v:Vec<T>)->Vec<T>{
    //sort the left half
    //sort the right half
    //bring the sorted halves together
    if v.len()<=1{
        return v;
    }
    let b =v.split_off(v.len()/2);
    let a=merge_sort(v);
    b=merge_sort(b);
    let a_it=a.into_iter();
    let b_it=b.into_iter();


}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_buble_sort() {
        let mut v=vec![4,6,1,8,11,13,3];
        bubble_sort_comp(&mut v);   
        assert_eq!(v, vec![4,6,8,11,13]);
    }
}
