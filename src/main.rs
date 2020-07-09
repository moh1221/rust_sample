mod order;

fn main(){
    let d: order::Order = order::Order::new(String::from("first"), String::from("order # 1"), 20.0);
    println!("{}", d);
}
