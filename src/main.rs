use std::io;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// fn operation_type_selection(){}

fn main() {
    
    let mut input_string = String::new();
    let question_array = vec!["type out the first 3 octets of the ip address of the webpage you want to open eg 192.168.1: ","type out the 4th octet, begining the range: ", "how many controllers on the network are there?: ","Input the Number coresponding to the Browser you want to use:\n0.Chrome\n1.Edge\n2.Firefox","use 'https://' instead of 'http://' ?[y=1/n=0]:"];
    let mut _answer_array = Vec::new();
    let mut _total_ip_array : Vec<String> = Vec::new();
    for text in 0..question_array.len(){
        println!("{}",question_array[text]);
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        _answer_array.push(input_string.clone());
    }
    let o4 = _answer_array[1].trim().parse::<i32>().unwrap();
    
    let mult = _answer_array[2].trim().parse::<i32>().unwrap();
    let mut _ip1 = Vec::new();
    for i in 0..mult{
        _ip1.push(o4+i);
        // println!("{}",o4+i);
    }
    for i in _ip1.iter(){
        let mut ownedstr = String::new();
        let sitetype = &_answer_array[4];
        println!("{}",sitetype.eq(&no));
        print_type_of(&sitetype);
        print_type_of(&no);
        match sitetype == 0.to_string(){
            true => ownedstr.push_str(&"https://".to_owned()),
            _ => ownedstr.push_str(&"http://".to_owned())
        }
        println!("{}",_answer_array[4]);
        let baseipstr: &str = &_answer_array[0].trim();
        let point: String = ".".to_owned();
        let ippart: &str = &i.to_string();
        let end : String = "/commissioning/index.html".to_owned();
        ownedstr.push_str(baseipstr);
        ownedstr.push_str(&point);
        ownedstr.push_str(ippart);
        ownedstr.push_str(&end);
        if _answer_array[3] >= 1.to_string(){
            open::with(ownedstr,"msedge");
        }else{
            open::that(ownedstr);
        }
        
        
        // println!("{}",ownedstr);
        // total_ip_array.push_str(&ownedstr);
    }
    // for i in total_ip_array.iter(){
    //     println!("{}",i)
    // }

}