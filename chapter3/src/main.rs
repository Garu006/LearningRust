fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("El valor de x dentro del bloque es: {x}");
    }
    println!("El valor de x fuera del bloque es: {x}");

    let spaces = "   ";
    let _spaces = spaces.len();

    // float number types
    let _x = 2.0; //f64
    let _y: f32 = 3.0; //f32

    println!("El valor de _x es: {_x}, el valor de _y es: {_y}");

    // numeric operations
    let sum = 5 + 10;

    let resta = 95.5 - 4.3;

    let multi = 4 * 32;

    let div = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;

    println!("La respuesta a todas las operaciones son: 
      suma = {sum}
    , resta = {resta}
    , multiplicación = {multi}
    , división = {div}
    , truncado = {truncated}
    , resto = {remainder}");

    // boolean type
    let t = true;
    let f: bool = false; //con tipo explícito
    println!("{t}, {f}");

    // char type
    let c = 'z';
    let z: char = 'z'; //con tipo explicito
    let beaver = '🦫';
    println!("{c}, {z}, {beaver}");

    // compound types
    //tuples type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("el valor de y es: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("Los valores de la tupla son: {five_hundred}, {six_point_four}, {one}");
    
}