fn main() {
    controlFlow();
    print_labeled_measurement(5, 'h');

    //another way to call the function
    let y = { //definimos una variable y le asignamos el valor de una expresión que tiene un codigo que devuelve un valor
        let x = 5;
        x + 1
    };
    println!("El valor de y es: {y}");

    let x = five(); //mandamos a llamar la funcion y le asignamos el valor que devuelve a la variable x
    println!("El valor de x es: {x}");

    let x = plus_one(5); //mandamos a llamar la funcion y le pasamos un parametro tipo entero y la funcion devuelve ese valor mas 1
    println!("El valor de x es: {x}");

}

fn print_labeled_measurement(value: i32, unit_label: char) { //definimos una funcion que recibe dos parametros, un entero y un label como char, y luego los manda a imprimir
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 { //funcion que no recive ningun parametro pero devuelve un entero
    5
}
fn plus_one(x: i32) -> i32 {//funcion que recibe un entero y devuelve el valor del parametro mas 1
    x + 1
}

// Control Flow

fn controlFlow() {
    let number = 7;
    if number < 5 {
        println!("La condición es verdadera");
    } else {
        println!("La condición es falsa");
    }
    let num = 8k;
    if num % 4 == 0{
        println!("el numero es divisible por 4");
    } else if num % 3 == 0 {
        println!("El numero es divisible por 3");
    } else if num % 2 == 0 {
        println!("El numero es divisible por 2");
    } else {
        println!("El numero no es divisible por nada");
    }
    
}
