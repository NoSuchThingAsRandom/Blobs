use std::cmp::Ordering;
use std::io;
struct Blob {
    identity: String,
    coords: Vec<isize>,
    move_coords: Vec<isize>,
    size: usize,
    target: String,
}
impl Blob {
    fn output(&self) {
        println!(
            "Blob {} has position {:?} size {} and is moving towards {}",
            self.identity, self.coords, self.size, self.target
        );
    }
    fn new(dims:usize) -> Blob {
        //Get x coordinate
        println!("Enter the {} coordinates in the form x,y,z",dims);
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        let pos: Vec<isize> = text
            .trim()
            .split(",")
            .filter_map(|x| x.parse::<isize>().ok())
            .collect();
        if pos.len()!=dims{
            println!("Invalid number of coordinates, try again!");
            return Blob::new(dims)
        }
        println!("Enter the size of the Blob: ");
        //Get size
        text=String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        println!("{}",text.trim());
        let size = text.trim().parse().unwrap();

        Blob {
            identity: "A".to_string(),
            coords: pos,
            move_coords: vec![],
            size: size,
            target: " ".to_string(),
        }
    }
}

fn template() -> Vec<Blob> {
    let mut blobs: Vec<Blob> = Vec::new();
    blobs.push(Blob {
        identity:"A".to_string(),
        coords: vec![47,-6,1,46],
        move_coords: vec![0,0,0,0],
        size: 1,
        target: " ".to_string(),
    });
    blobs.push(Blob {
        identity:"B".to_string(),
        coords: vec![29,28,13,-11],
		move_coords: vec![0,0,0,0],
        size: 1,
        target: " ".to_string(),
    });
    blobs.push(Blob {
        identity:"C".to_string(),
        coords: vec![-17,40,45,9],
		move_coords: vec![0,0,0,0],
        size: 1,
        target: " ".to_string(),
    });
    blobs.push(Blob {
        identity:"D".to_string(),
        coords: vec![0,12,-18,16],
		move_coords: vec![0,0,0,0],
        size: 1,
        target: " ".to_string(),
    });
    blobs.push(Blob {
        identity:"E".to_string(),
        coords: vec![-6,-31,-40,35],
		move_coords: vec![0,0,0,0],
        size: 1,
        target: " ".to_string(),
    });
    blobs
}


fn generate(dims:usize) -> Vec<Blob>{
    let mut blobs: Vec<Blob> = Vec::new();
    let mut input = String::new();
    println!("Do you wish to create a new blob? ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    while input.trim() == "yes" {
        blobs.push(Blob::new(dims));
        println!("Do you wish to create a new blob? ");
        input=String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
    blobs
}

fn display_grid(width:usize,height:usize,blobs:&[Blob]){
    return;
    let mut grid=vec![vec![String::from("   ");width*2+2];height*2+2];
    let mut index=1;
    for x in -(width as isize)..width as isize+1{
        grid[0][index]=format!("{:<3}",x.to_string());
        index+=1;
    }
    index=1;
    for y in -(height as isize)..height as isize+1{
        grid[index][0]=format!("{:<3}",y.to_string());
        index+=1;
    }
    for b in blobs.iter(){
        let y=match b.coords[1].cmp(&0){
            Ordering::Equal => 0,
            Ordering::Less => b.coords[1].abs() as usize,
            Ordering::Greater => height+b.coords[1] as usize,
        };
        let x=match b.coords[0].cmp(&0){
            Ordering::Equal => 0,
            Ordering::Less => b.coords[0].abs() as usize,
            Ordering::Greater => width+b.coords[0] as usize,
        };
        grid[y+1][x+1]=format!("{:<3}",b.identity);
    }
    for y in grid.iter(){
        for x in y.iter(){
            print!("|{}",x);
        }
        print!("\n");
    }
}

fn main() {
    /*
	let mut input = String::new();
    println!("How many dimensions do you want? ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
	*/
	let mut input=String::from("4");
    let dims =input.trim().parse::<usize>().unwrap();
    let mut blobs = template();
    
    let mut width=0 as usize;
    let mut height=0 as usize;
    for blob in blobs.iter(){
        if blob.coords[0].abs()>width as isize{
            width=blob.coords[0].abs() as usize;
        }
        if blob.coords[1].abs()>height as isize{
            height=blob.coords[1].abs() as usize;
        }
    }
    println!("Width is: {}",width);
    if dims==2{
        display_grid(width, height, blobs.as_slice());
    }
    let mut pass=1;
    while blobs.len()>1{
        //Calculate Blob Movement
        for current in 0..blobs.len() {
            let mut closest =std::usize::MAX;
            let mut target=0;
            for index in 0..blobs.len(){
                if current!=index{
                    let mut distance=0 as usize;
                    for d in 0..dims{
                        distance+=(blobs[index].coords[0]-blobs[current].coords[0]).pow(2)as usize;
                    }
                    distance=((distance as f64).sqrt())as usize;

                    if distance < closest {
                        closest=distance;
                        target=index;
                        blobs[current].target=blobs[index].identity.clone();
                    }
                }
            }
            for i in 0..blobs[current].move_coords.len(){
                blobs[current].move_coords[i]= match (blobs[target].coords[i]-blobs[current].coords[i]).cmp(&0){
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                    _ => 0,
                };
            }
        }

        //Move Blobs
        let mut current=0;
        let mut end=blobs.len();
        while current <end {
            for d in 0..dims{
                blobs[current].coords[d]+=blobs[current].move_coords[d];
            }
            let mut check=current+1;
            while check <end{
                let mut collide=true;
                for d in 0..dims{
                   if blobs[current].coords[d]!=blobs[check].coords[d]{
                       collide=false;
                       break;
                   }
                }
                if collide{
                    blobs[current].size+=blobs[check].size;
                    blobs.remove(check);
                    end-=1;
                    check-=1;
                }
                check+=1;
            }
            current+=1;
        }

        //Output Blobs
        println!("\n\nCurrent Pass: {}",pass);
        if dims==2{
            display_grid(width, height, blobs.as_slice());
        }
        for x in blobs.iter(){
            x.output();
        }
        println!("Number of Blobs: {}",blobs.len());
        pass+=1;
    }
}
