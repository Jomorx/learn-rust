fn main(){
    let num1 = 100;
    let num2 = 991;

    let res = max(& num1,&num2);
    println!("{res}")

}
fn max<'a>(num1:&'a i32,num2:&'a i32)->&'a i32{
    if num1>num2 {
        num1
    }else {
        num2
    }
}