use std::fmt::Debug; //used for printing the array or type

pub fn pivot<T:PartialOrd+Debug>(v:&mut [T])->usize{
    let mut p=0;
    for i in 1..v.len(){
        if v[i]<v[p]{
            v.swap(p+1, i);
            v.swap(p,p+1);
            p+=1;
        }
    }
    p
}



pub fn merge_sort<T:PartialOrd+Debug>(mut v:Vec<T>)->Vec<T>{
    //sort the left half
    //sort the right half
    //bring the sorted halves together
    if v.len()<=1{
        return v;
    }
    let mut res=Vec::with_capacity(v.len());
    let b =v.split_off(v.len()/2);
    let a=merge_sort(v);
    let b=merge_sort(b);
    
    let mut a_it=a.into_iter();
    let mut b_it=b.into_iter(); 
    let mut a_peak=a_it.next();
    let mut b_peak=b_it.next();
    loop{
        match a_peak{
            Some(ref a_val)=>match b_peak{
                Some(ref b_val)=>{
                    if b_val<a_val{
                        res.push(b_peak.take().unwrap());
                        b_peak=b_it.next();
                    }
                    else{
                        res.push(a_peak.take().unwrap());
                        a_peak=a_it.next();
                    }
                    

                }
                None=>{
                    res.push(a_peak.take().unwrap());
                    res.extend(a_it);
                    return res;

                }
            }
            None=>{
                if let Some(b_val)=b_peak{
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }

}


pub fn quick_sort<T:PartialOrd+Debug>(v:&mut [T]){
    if v.len()<=1{
        return;
    }
    let p= pivot(v);
    println!("{:?}",v);
    let (a,b)=v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
    //
    // 
}

pub fn bub_sort<X:PartialOrd+Debug+Clone>(v:&mut [X]){
    for p in 0..v.len(){
        
        let mut sorted=true;
        for i in 0..v.len()-1{
            if v[i]> v[i+1]{
                v.swap(i+1, i);
                sorted=false;
                println!("{:?}",v);
                
            }
        }
        // println!("print karo be {:?}",v);
        if sorted{
            return;
        }
    }

}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_buble_sort() {
        let mut v=vec![4,6,1,8,11,13,3];
        bub_sort(&mut v);   
        assert_eq!(v, vec![1,3,4,6,8,11,13]); //panic if two values are not equal
    }
    #[test]
    fn pivot_test(){
        let mut v=vec![3,10,6,2,18,1];
        let p=pivot(&mut v);
        for x in 0..v.len(){
            assert!((v[x]>v[p])==(x>p));
        }
    }

    #[test]
    fn test_quick_sort(){
        let mut v=vec![3,10,6,2,18,1];
        quick_sort(&mut v);
        assert_eq!(v,vec![1,2,3,6,10,1]);
    }
}
