use std::io;
use rand::Rng;

fn main() {
    let mut l =
        [[0,0,0,0],
        [0,0,0,0],
        [0,2,0,0],
        [0,0,0,0]];

    let mut sign:Option<char>;

    loop{
        l = generate_oneblock(l);
        show_block(l);
        sign = get_sign();
        l = build_block(l,sign);
        if is_win(l) {
            println!("YOU WIN");
            show_block(l);
            break;
        }
        else if is_lose(l) {
            println!("YOU LOSE");
            show_block(l);
            break;
        }
    }

}

fn generate_oneblock(mut l:[[i32;4];4])->[[i32;4];4]{
    let mut zero_bloc_list: Vec<(i32,i32)> = Vec::new();

    for i in 0..4 {
        for j in 0..4 {
            if l[i][j]==0 {
                zero_bloc_list.push((i.try_into().unwrap(),j.try_into().unwrap()));
            }
        }
    }

    if zero_bloc_list.is_empty(){return l};

    let zero_block_index = rand::thread_rng().gen_range(0..zero_bloc_list.len());
    let target = zero_bloc_list[zero_block_index];
    l[target.0 as usize][target.1 as usize] = 2;
    return l;
}

fn show_block(l:[[i32;4];4]){

    for i in 0..4 {
        println!("=========================");
        println!("| {}| {}| {}| {}|",parse(l[i][0]),parse(l[i][1]),parse(l[i][2]),parse(l[i][3]));
    }
    println!("=========================");

}

fn get_sign()->Option<char>{
    loop{
        println!("plz input wasd");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("invalid input");
        println!("your sign is {}",buffer);
        let b = buffer.chars().nth(0);
        if b == Some('w') || b == Some('a') || b == Some('s') || b == Some('d') {
            return b;
        }
        else{
            println!("invalid input");
        }
    }
}

fn build_block(mut l:[[i32;4];4],sign:Option<char>)->[[i32;4];4]{
    //matchとかenumでいい感じにしたかったかも
    if sign == Some('w'){
        for i in 0..4 {
            let tmp = tokoroten(l[3][i], l[2][i], l[1][i], l[0][i]);
            l[3][i] = tmp.0;
            l[2][i] = tmp.1;
            l[1][i] = tmp.2;
            l[0][i] = tmp.3;
        }
    }
    else if sign == Some('a'){
        for i in 0..4 {
            let tmp = tokoroten(l[i][3], l[i][2], l[i][1], l[i][0]);
            l[i][3] = tmp.0;
            l[i][2] = tmp.1;
            l[i][1] = tmp.2;
            l[i][0] = tmp.3;
        }
    }
    else if sign == Some('s'){
        for i in 0..4 {
            let tmp = tokoroten(l[0][i], l[1][i], l[2][i], l[3][i]);
            l[0][i] = tmp.0;
            l[1][i] = tmp.1;
            l[2][i] = tmp.2;
            l[3][i] = tmp.3;
        }
    }
    else if sign == Some('d'){
        for i in 0..4 {
            let tmp = tokoroten(l[i][0], l[i][1], l[i][2], l[i][3]);
            l[i][0] = tmp.0;
            l[i][1] = tmp.1;
            l[i][2] = tmp.2;
            l[i][3] = tmp.3;
        }
    }
    else {
        println!("cannot build block");
    }

    return l;
}

fn is_win(l:[[i32;4];4])->bool {
    let mut is_win:bool = false; 
    for i in 0..4 {
        for j in 0..4 {
            if l[i][j] == 2048 {
                is_win = true;
            }
        }
    }
    return is_win;
}    

fn is_lose(l:[[i32;4];4])->bool{

    let mut is_nozero:bool = true;
    let mut is_stuck:bool = true;
    
    for i in 0..4 {
        for j in 0..4 {
            if l[i][j] == 0 {
                is_nozero = false;
                break;
            }
        }
    }

    for i in 0..3{
        for j in 0..3{
            if l[i][j]==l[i][j+1] || l[i][j]==l[i+1][j]{
                is_stuck=false;
                break;
            }

        }
    }

    return is_nozero && is_stuck;

}

fn tokoroten(mut a:i32,mut b:i32,mut c:i32,mut d:i32)->(i32,i32,i32,i32){

    //if zero, push it
    let mut tmp :Vec<i32> = Vec::new();
    for i in [d,c,b,a]{
        if i!=0{tmp.push(i)};
    }
    while tmp.len() < 4 {
        tmp.push(0);
    }
    d = if tmp[0]!=0 {tmp[0]}else{0};
    c = if tmp[1]!=0 {tmp[1]}else{0};
    b = if tmp[2]!=0 {tmp[2]}else{0};
    a = if tmp[3]!=0 {tmp[3]}else{0};

    //if same number, combaine and push it
    if d==c && d != 0 {
        d = d*2;
        c = b;
        b = a;
        a = 0;
    }
    if c == b && c != 0 {
        c = c*2;
        b = a;
        a = 0;
    }
    if b == a && b != 0 {
        b = b*2;
        a = 0;
    }
    // println!("a {},b {},c {},d {}",a,b,c,d);
    return (a,b,c,d);
}

fn parse(x:i32)->String{
    let mut ret = x.to_string();
    while ret.len() < 4 {
        ret.insert(0, ' ');
    }
    return ret;
}        
        // println!("============");
        // println!("| 0| 0| 0| 0|",);
        // println!("============");
        // println!("| 0| 0| 0| 0|");
        // println!("============");
        // println!("| 0| 0| 0| 0|");
        // println!("============");
        // println!("| 0| 0| 0| 0|");
        // println!("============");

        // println!("=========================");
        // println!("| 2048| 2048| 2048| 2048|");
        // println!("=========================");
        // println!("| 128 | 128 |  128|  128|");
        // println!("=========================");
        // println!("| 16  | 18  | 2048| 2048|");
        // println!("=========================");
        // println!("| 0   | 0   | 2048| 2048|");
        // println!("=========================");  