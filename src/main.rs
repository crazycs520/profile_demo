fn main() {
    println!("begin");
    for i in 1..100 {
        query(i);
    }
    println!("end");
}

fn query(load :u64){
    let _ = read_data(load);
}

fn read_data(n :u64) -> u64 {
    let n = n* 1000_000_00;
    let mut sum = 0;
    for i in 0..n {
        sum+=i;
    }
    return sum;
}
