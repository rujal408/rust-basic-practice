fn main() {
    // let mut new_vec = Vec::new();
    // let mut counted = 1;

    // while counter <11{
    //     new_vec.push(counted);
    //     counted+=1;
    // }

    // println!("{:?}", new_vec)
    //
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

    // Like Slice or Splice in JS
    let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();

    println!("{:?}", new_vec)
}
