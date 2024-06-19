fn main() {
    let mut i = 0;
    loop{
        if i % 3 == 0 || i % 5 == 0{
            if i % 3 == 0 {
                print!("Fizz");
            }
            if i % 5 == 0{
                print!("Buzz")
            }
            println!("")
        }

        else{
            println!("{i}");
        }

        if i>99{
            break
        }
        i += 1;
    }
}
