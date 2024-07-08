/*
THis code defines the function that generates a map, where 0 is accessible, 1 is inaccessible
*/
use rand::Rng;

pub fn map_rand_gen(col:usize,row:usize)->Vec<Vec<i8>>
{
    let mut map=vec![vec![0;col];row];//the return thing
    let bar_num=col*row/10;//the number of barriers to set

    //generating barriers
    let mut rng=rand::thread_rng();
    
    for _i in 0..bar_num
        {
            
            map[rng.gen_range(0..col)][rng.gen_range(0..row)]=1;
        }
    map


}
