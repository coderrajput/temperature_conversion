use std::io;

fn main() {
    println!("choose your option");
    println!("1.Fahreheit to celcius");
    println!("2.celcius to Fahreheit ");
    let calculated_temperature=0.0;
    let mut option= String::new();
    let mut temperature= String:: new();
    io::stdin().read_line(&mut option).expect("failed to read line");
    let option : i32= option.trim().parse().expect("please type a number");
    println!("enter temperature");
    io::stdin().read_line(&mut temperature).expect("failed to read line");
    let temperature : f64=temperature.trim().parse().expect("please type a number");

    let calculated_temperature =if option==1{
        to_celcius(temperature)
    } else if option == 2 {
        to_fahrenheit(temperature)
    } else {
        0.0
    };
    
    println!("calculated temperature: {}",calculated_temperature);
}


/* to convert fahrenheit temperature to celcius  */
fn to_celcius (x: f64) -> f64{
    let mut temp =0.0;  
    temp=(x-32.0)*(5.0/9.0);
    temp
}


/* from celcius to fahreheit*/
fn to_fahrenheit (x: f64) -> f64{
    let mut temp= 0.0;
    temp= x*(9.0/5.0)+32.0;
    temp
   }