fn main() {
    let n1 = "Eletrical".to_string();
    let n2 = " Eletronic".to_string();
    let n3 = " Engineeering".to_string();
    let n4 = n1 +&n2 +&n3; // n2 & n3 reference is passed

    // about eletrical/eletronic
    println!("\nThe {} is informed by the aspiration to 
        train eletrical/eletronic engineering professionals 
        in the areas of design, buildiung and maintenance of 
        eletrical control systems,", n4);
    let w1 = "Computer".to_string();
    let w2 = " Science".to_string();
    let w3 = w1 + &w2;   // w2 reference passed
    println!();
    println!("{} is aimed at developing competent, creative,
        innovative, enterpreneurial and ethnically-minded persons,
        capable of creating value in the diverse fields of computer science. ", w3);


}
