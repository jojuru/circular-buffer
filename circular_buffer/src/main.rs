use std::io;
use std::io::Write;
fn main() {
    let mut myarray: [i32; 10] = [0; 10];
    let mut head: usize = 0;
    let mut tail: usize = 0;
    let mut size: i32 = 0;
    

    println!("Luku lisätään joukkoon, 'e' poistu");
    println!("Muokkaus tila paina 'm'");
    println!("Näytä taulukko 'ls'");
    loop {
        print!("anna luku: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("failed to read");

        if s.trim() == String::from("e") {
            break;
        }

        //muokkaus tila
        if s.trim() == String::from("m"){
            println!("Poista tail 'rt'");
            println!("Poista head 'rh'");

            print!("anna käsky: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut s2= String::new();
            io::stdin()
                .read_line(&mut s2)
                .expect("failed to read");

            if s2.trim() == String::from("rt") {
                remove_tail(&mut myarray, &mut tail, &mut size);
                println!("{:?}", myarray);
            }
            else if s2.trim() == String::from("rh") {
                remove_head(&mut myarray, &mut head, &mut size);
                println!("{:?}", myarray);
            }

        }
        else if s.trim() == String::from("ls"){
            println!("{:?}", myarray);
        }
        else {
            //lisäys tila
            if !s.trim().is_empty() {
                if let Ok(n) = s.trim().parse::<i32>() {
                    add_element(&mut myarray, n, &mut head, &mut tail, &mut size);
                } 
                else {
                    println!("Invalid input, not a number. Please try again.");
                }
            }
        }
    }

    
    println!("Tulos");
    println!("{:?}", myarray);
}

fn add_element(myarray: &mut [i32; 10],  n: i32, head: &mut usize,  tail: &mut usize, size:&mut i32) {
    if head == tail && *size > 0{
        println!("Saving over old elements addition canceled, remove old elements first");
    }
    else {
        myarray[*tail] = n;
        *tail = (*tail + 1) % myarray.len();
        *size += 1;
    }
}


fn remove_tail(myarray: &mut [i32; 10], tail: &mut usize, size:&mut i32){
    if *size == 0 {
        println!("No elements to remove");
    } else {
        *size -= 1;
        if *tail == 0{
            *tail = myarray.len() - 1;
        }
        else{
            *tail -= 1;
        }
        myarray[*tail] = 0;
    }
}

fn remove_head(myarray: &mut [i32; 10], head: &mut usize, size:&mut i32){
    if *size == 0 {
        println!("No elements to remove");
    } else {
        *size -= 1;
        myarray[*head] = 0;
        *head = (*head + 1) % myarray.len();
    }
}


