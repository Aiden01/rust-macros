macro_rules! m_vec {
    ( $($item:expr),+ ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($item); )+
            temp_vec
        }
    };
}

macro_rules! m_path {
    () => { "/" };
    ( $($segments:expr),+ ) => { concat!($(concat!("/", $segments)),+) };
}

macro_rules! m_math {
    ($num:tt) => { println!("{}", $num); };
    ($left:tt $right:tt) => { println!("Missing two arguments"); };
    ($($num1:tt $sym:tt $num2:tt)+) => {
        println!("Result: {}", $($num1 $sym $num2)+);
    }
}

macro_rules! avg {
    ($($data:tt),+) => {
        {
            let mut data_vec = Vec::new();
            $( data_vec.push($data); )+
            let length = data_vec.len();
            let sum: i32 = data_vec.into_iter().sum();
            println!("{}", sum / length as i32);
        }
    };
}

macro_rules! cast {
    () => {};
    (($cast_to:ty)) => {
        println!("Missing the expression that you want to cast to {}", $var);
    };
    ($var:expr) => {
        println!("Missing the type that you want {} to be casted to", $var);
    };
    (($cast_to:ty) $var:expr) => {
        $var as $cast_to
    };
}

macro_rules! does {
    ($expr1:tt is $expr2:expr) => { $expr1 == $expr2 };
}

macro_rules! do_that {
    ([$job:expr] <- $times:tt) => {
        for _ in 1..$times {
            println!("{}", $job);
        }
    }
}

macro_rules! build_fn {
    (name $name:ident body $body:expr) => {
        fn $name() {
            $body
        }
    };
    (name $name:ident ($rty:ty) body $body:expr) => {
        fn $name() -> $rty {
            $body
        }
    };
}

fn main() {
    let v = m_vec!("hello", "world");
    println!("{:?}", v);
    let p = m_path!("home", "foo", ".config");
    println!("{}", p);
    m_math!(1 * 2);
    avg!(14, 45, 65, 32, 45);
    println!("{}", cast!( (u32) 32.2));
    println!("{}", does!(14 is 13 + 1));
    do_that!([1 + 1] <- 5);
    build_fn!(name hello_world body println!("hey"));
    hello_world();
    build_fn!(name foo (i32) body 1 + 3);
    println!("{}", foo());
}
