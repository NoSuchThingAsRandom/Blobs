use std::io;
use std::cmp::Ordering;
struct Blob {
    identity:String,
    x_coord: isize,
    y_coord: isize,
    x_move: isize,
    y_move: isize,
    size: usize,
    target: String,
}
impl Blob {
    fn output(&self) {
        println!(
            "Blob {} has position ({},{}) size {} and is moving towards {}", self.identity,
            self.x_coord, self.y_coord, self.size,self.target
        );
    }
    fn new() -> Blob {
        //Get x coordinate
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        let x_coord = text.parse().unwrap();

        //Get y coordinate
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        let y_coord = text.parse().unwrap();

        //Get size
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        let size = text.parse().unwrap();
        Blob {
            identity:"Z".to_string(),
            x_coord: x_coord,
            y_coord: y_coord,
            x_move: 0,
            y_move: 0,
            size: size,
            target: " ".to_string(),
        }
    }
}
fn generate() -> Vec<Blob> {
    let mut blobs: Vec<Blob> = Vec::new();
    blobs.push(Blob {
        identity:"A".to_string(),
        x_coord: 1,
        y_coord: 1,
        x_move: 0,
        y_move: 0,
        size: 1,
        target: " ".to_string(),
    });
    blobs.push(Blob {
        identity:"B".to_string(),
        x_coord: 8,
        y_coord: 4,
        x_move: 0,
        y_move: 0,
        size: 2,
        target: " ".to_string(),
    });
    blobs.push(Blob {
        identity:"C".to_string(),
        x_coord: 1,
        y_coord: 7,
        x_move: 0,
        y_move: 0,
        size: 1,
        target: " ".to_string(),
    });
    blobs.push(Blob {
        identity:"D".to_string(),
        x_coord: 1,
        y_coord: 2,
        x_move: 0,
        y_move: 0,
        size: 2,
        target: " ".to_string(),
    });
    blobs.push(Blob {
        identity:"E".to_string(),
        x_coord: 3,
        y_coord: 9,
        x_move: 0,
        y_move: 0,
        size: 3,
        target: " ".to_string(),
    });
    blobs
}


fn get_new(){
    let mut blobs: Vec<Blob>=Vec::new();
    let mut input = String::new();
    println!("Do you wish to create a new blob? ");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    while input != "yes"{
        blobs.push(Blob::new());
        println!("Do you wish to create a new blob? ");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    }
}

fn display_grid(width:usize,height:usize,blobs:&[Blob]){
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
        let y=match b.y_coord.cmp(&0){
            Ordering::Equal => 0,
            Ordering::Less => b.y_coord.abs() as usize,
            Ordering::Greater => width+b.y_coord as usize,
        };
        let x=match b.x_coord.cmp(&0){
            Ordering::Equal => 0,
            Ordering::Less => b.x_coord.abs() as usize,
            Ordering::Greater => width+b.x_coord as usize,
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
    let mut blobs = generate();

    let mut width=0 as usize;
    let mut height=0 as usize;
    for blob in blobs.iter(){
        if blob.x_coord.abs()>width as isize{
            width=blob.x_coord.abs() as usize;
        }
        if blob.y_coord.abs()>height as isize{
            height=blob.y_coord.abs() as usize;
        }
    }
    println!("Width is: {}",width);
    display_grid(width, height, blobs.as_slice());

    let mut pass=1;
    while blobs.len()>1{
        //Calculate Blob Movement
        for current in 0..blobs.len() {
            let mut closest =std::usize::MAX;
            let mut target=0;
            for index in 0..blobs.len(){
                if current!=index{                
                    let distance= (((blobs[index].x_coord-blobs[current].x_coord).pow(2)+(blobs[index].y_coord-blobs[current].y_coord).pow(2)) as f64).sqrt() as usize;
                    if distance<closest{
                        closest=distance;
                        target=index;
                        blobs[current].target=blobs[index].identity.clone();
                    }
                }
            }
            blobs[current].x_move= match (blobs[target].x_coord-blobs[current].x_coord).cmp(&0){
                Ordering::Less => -1,
                Ordering::Greater => 1,
                _ => 0,
            };
            blobs[current].y_move= match (blobs[target].y_coord-blobs[current].y_coord).cmp(&0){
                Ordering::Less => -1,
                Ordering::Greater => 1,
                _ => 0,
            };
            //blobs[current].output();
        }

        //Move Blobs
        let mut current=0;
        let mut end=blobs.len();
        while current <end {
            blobs[current].x_coord+=blobs[current].x_move;
            blobs[current].y_coord+=blobs[current].y_move;
            let mut check=current+1;
            while check <end{
                if blobs[current].x_coord==blobs[check].x_coord && blobs[current].y_coord==blobs[check].y_coord{
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
        display_grid(width, height, blobs.as_slice());
        for x in blobs.iter(){
            x.output();
        }
        println!("Number of Blobs: {}",blobs.len());
        pass+=1;



    }

}
