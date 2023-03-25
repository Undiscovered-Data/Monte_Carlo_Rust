use rand::Rng;
use std::io;
use statrs::statistics::Statistics;

fn main() -> io::Result<()> {

    let mut bin_vec: Vec<String> = Vec::new();
    let mut item_vec: Vec<String> = Vec::new();
    println!("Welcome to MC");
    println!("Type calc to exit\n or press enter to continue");

    loop {
        let mut name = String::new();
        let mut name2 = String::new();
        let mut str_number = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut name)?;
        let str_stdin = name.trim();
        if str_stdin == "calc" { break; }

        println!("Enter the name of the item");
        let stdin = io::stdin();
        stdin.read_line(&mut name2)?;
        let name3 = name2.trim();
        let clone_name = name3.clone();
        item_vec.push(clone_name.to_string());
        println!("Enter the quantity");
        let stdin2 = io::stdin();
        stdin2.read_line(&mut str_number)?;
        let quantity: usize = str_number.trim().parse().unwrap();
        //let temp_vec = vec![&name3; quantity.try_into().unwrap()]; needed if not usize
        let temp_vec = vec![&name3; quantity];
        for a in temp_vec.iter() {
            bin_vec.push(a.to_string());
        }

        println!("\nType calc to calculate\n or enter to add another item");
    }
    for the_item in item_vec {
        let clone_item = the_item.clone();
        let the_tupl = calculate_it(&bin_vec, the_item);
        let the_mean = the_tupl.0;
        let the_std_d = the_tupl.1;
        println!("You get {clone_item} on average at {the_mean} picks std dev {the_std_d}");
    }

    Ok(())
}

fn calculate_it(bin_vec: &Vec<String>, calc_item: String) -> (f64, f64) {
    let mut eval_vec = Vec::new();
    for _ in 0..1000 {
        let mut m_vec = bin_vec.clone();
        let mut the_count: f64 = 0.0;
        let vec_length1 = m_vec.len();

        for _ in 0..vec_length1 {
            the_count += 1.0;
            let vec_length = m_vec.len();
            let mut rng = rand::thread_rng();
            let rand_number = rng.gen_range(0..vec_length);
            let the_thing = m_vec.swap_remove(rand_number);
            if the_thing == calc_item { break; }
        }
        eval_vec.push(the_count);
    }

    let eval2 = eval_vec.clone();
    let the_mean = eval_vec.mean();
    let std_d = eval2.std_dev();
    (the_mean, std_d)
}

