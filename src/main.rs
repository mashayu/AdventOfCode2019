use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
	println!("Hello, world!");
    let f = File::open("foo.txt")?;
    /*let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };*/

    let file_reader = BufReader::new(f);
    let mut v: Vec<i32> = Vec::new();
    //let mut v: Vec<isize 9> = vec![];

    for line in file_reader.lines() {
    	//println!("{}", line.unwrap());
        match line {
            Err(why)   => panic!("{:?}", why),
            Ok(string) => match string.trim().parse::<i32>() {
                Err(why)   => panic!("Not a number!"),
                Ok(number) => v.push(number)
            }
        }
    }

    let mut res2: i32;
    let mut res3: i32 = 0;
    loop{
        //println!("Vector: {:?}", v);
        for i in v.iter_mut() {
    	    *i = *i/3 - 2;
        }

        //res2 = v.iter().sum();

        res2 = v.iter().map(|&x: &i32|
        if x < 0 { 0 }
        else { x }
	    ).sum();

	    println!("{:?}", res2);

        res3 = res3+res2;

        println!("{:?}", res3);
        if(res2 == 0){
        	break;
        }
    }

    //println!("Vector: {:?}", v);

    /*let res: Result<i32, &'static str> = v.iter().map(|&x: &i32|
    if x < 0 { Err("Negative element found") }
    else { Ok(x) }
	).sum();*/

	//let res2: i32 = v.iter().sum();


	println!("{:?}", res2);

	println!("{:?}", res3);
    Ok(())
}