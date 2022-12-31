use rand::{Rng, thread_rng};

fn throwing_a_die()
{
    use rand::distributions::{Distribution, Uniform};

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6
        {
            break;
        }
    }
}

use rand_distr::{Distribution, Normal, NormalError};
fn normal_dist() -> Result<(), NormalError>
{
    let mut rng = thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);

    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}

fn main()
{
    let mut rng = rand::thread_rng();

    // Generate random values
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    print!("Hello, world!");
    println!(" I'm learning Rust!");
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    // Generate random values in a range
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));

    throwing_a_die();

    let norm = normal_dist();
    println!("This is my normal, I think? {:?}", norm);
}
