// 1-
// fn main() {
//    check_password(123);
//    check_password("qwerty");
//    let pass1 = String::from("lahore");
//    check_password(pass1);
// }

// fn check_password<T: std::fmt::Display>(x:T){
//     println!("{}",x);
// }

// 2- generic type

// fn main(){
//     let v = vec![1,2,3,5];
//     println!("largest number is : {}", find_largest(&v));
// }
// fn find_largest<T>(x:&[T])->T{
//     let mut largest = x[0];
//     for &item in x.iter(){
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }

// // 3- Generic in struct defination
// #[derive(Debug)]

// struct NewStruct<T,U>{
//     x: T,
//     y: U,
// }

// fn main(){
//     let s1 = NewStruct{x:10 , y:32.4 };
//     println!("{:#?}",s1);
// }

// 4- generic in enum defination
// nothing to show

//5- generic in method defination


// #[derive(Debug)]
// struct Point <T>{
//     x:T,
//     y:T,
// }
// impl <T> Point<T>{
//     fn x(&self) -> &T{
//         &self.x
//     }
// }

// impl Point <f64>{
//     fn distance(&self)->f64{
//         ((&self.x.powi(2)) + (&self.y.powi(2))).sqrt()   
//     }
// }

// fn main(){
//     let p1 = Point{x:8, y:9};
//     let p2 = Point{x:10.0, y:20.0};
//     println!("{:#?}",p1.x());
//     println!("{:#?}",p2.distance());
// }

//6- generic in method defination 2

// #[derive(Debug)]
// struct Point<U,V>{
//     x:U,
//     y:V,
// }
// impl <U,V> Point <U,V>{
//     fn mixup<W,X>(self, other: Point<W,X>) -> Point<U,X>{
//         Point{
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
// fn main(){
//     let p1 = Point{x:12,y:110.12};
//     let p2 = Point{x:"Aslamualekum",y:'@'};

//     println!("{:#?}",p1.mixup(p2) );

// }

// 8 trait 
// struct Superman{
//   name: String,
// }
// struct Batman{
//   name: String,
// }
// struct Hulk{
//   name: String,
// }
// struct Spiderman{
//   name: String,
// }

// pub trait Power {
//   // fn power_score(&self)->u8; // if you will use separate power for all characters. this is the syntax
//   fn power_score(&self)->u8{
//     50
//   }
// }
// impl Power for Superman{
//   fn power_score(&self) ->u8 {
//       100
//   }
// }
// impl Power for Batman{
//   fn power_score(&self) ->u8 {
//       80
//   }
// }
// impl Power for Hulk{}//will use default power defined in traits
// impl Power for Spiderman{}//will use default power defined in traits
// fn main(){
//  // making instances of above structs
//  let super_man = Superman{
//    name:String::from("Superman")
//  };
//  let bat_man = Batman{
//    name:String::from("Batman")
//  };
//  let hulk = Hulk{
//    name:String::from("Hulk")
//  };
//  let spider_man = Spiderman{
//    name:String::from("Spiderman")
//  };
//  println!("{}",super_man.power_score());
//  println!("{}",bat_man.power_score());
//  println!("{}",hulk.power_score());
//  println!("{}",spider_man.power_score());
// }

// // 9 trait ex and 10 different method in traits 
// struct News{
//   author: String,
//   news: String,
// }
// struct Tweets{
//   author: String,
//   tweet: String,
// }

// pub trait Summery {
//   fn summrize_author(&self) -> String;
//   fn summrize(&self) -> String{
//     format!("{}",self.summrize_author())
//   }
// }

// impl Summery for News{
//   fn summrize_author(&self) -> String {
//       format!("{}", self.author)
//   }
// }

// // impl Summery for Tweets{
// //   fn summrize(&self) -> String {
// //       format!("{} by {}",self.tweet, self.author,)
// //   }
// // }
// fn main(){ 
//   let new_news = News{
//     author: String::from("John"),
//     news: String::from("Jobiden will win the election."),
//   };
//   let new_tweet = Tweets{
//     author: String::from("Deborah"),
//     tweet: String::from("Orya keep trying! Will you?"),
//   };
//     println!("{}",new_news.summrize()); 
//     // println!("{}",new_tweet.summrize()); 


// 11 trait bound syntax 
// #[derive(Debug)]
// struct Tweet{
//   author: String,
//   content: String,
// }
// #[derive(Debug)]
// struct News{
//   author: String,
//   content: String,
// }

// trait Summery{
//   fn summerize(&self)->String;
// }

// impl Summery for Tweet{
//   fn summerize(&self) ->String {
//       format!("@{} Posted this: {}",self.author,self.content)
//   }
// }
// impl Summery for News{
//   fn summerize(&self) ->String {
//       format!("@{} Posted this: {}", self.author, self.content)
//   }
// }
// fn notify <T: Summery> (item: T) -> String{
//   format!("{}",item.summerize())
// }
// fn main(){
//   let new_tweet =  Tweet{
//     author: String::from("John"),
//     content: String::from("It will be all ok Orya :)")
//   };
//   println!("{}",notify(new_tweet));
// }

//12 the impl trait syntax is convenient 
// 
//13 specifying multiple trait bound with + syntax 
// see notes

//14 clearer trait bound with where clause 
// see notes

//15 returning types that implement traits 
// struct Tweet{
//   author: String,
//   content: String,
// }

// trait Summery{
//   fn summerize(&self)-> String;
// }

// impl Summery for Tweet{
//   fn summerize(&self) -> String {
//       format!("@{} posted this : {}",self.author, self.content)
//   }
// }

// fn value_ret()->impl Summery{
//   Tweet{
//     author: String::from("Ali"),
//     content: String::from("Hi how are you?")
//   }
// }
// fn main(){
//   value_ret();
// }

//16 using trait bounds to conditional implement methods 
#[derive(Debug)]
struct Pair<T>{
    x: T,
    y: T,
} 
impl <T> Pair<T>{
    fn new(x: T, y: T)->Self{
        Self{
            x,
            y,
        }
    }
}
impl <T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x > self.y {
            println!("X:{} is greater than Y: {}", self.x,self.y);
        }
        else{
            println!("Y:{} is greater than X: {}", self.y,self.x);
        }

    }
}
fn main(){
    let mut h1 = HashMap::new();
    h1.insert(1, 2);
    let mut h2 = HashMap::new();
    h2.insert(3, 4);
    let p = Pair::new(h1, h2);

    
    let p1 = Pair::new(2, 3);
    println!("{:#?}",p);
    p1.cmp_display();
}

// //17 correct code of largest function

// fn main(){
//     let num_list = vec![11,2,3,5,3,6];
//     let largest_num = largest(&num_list);
//     println!("Largest number is = {}", largest_num);
//     let char_list = vec!['y','r','h','d','g','m'];
//     let largest_char = largest(&char_list);
//     println!("Largest character is = {}", largest_char);
// }

// fn largest<T: PartialOrd + Copy> (list: &[T])->T{
//     let mut largest_item = list[0];
//     for &item in list.iter(){
//         if item > largest_item{
//             largest_item = item;
//         }
//     }
//     largest_item
// }



























// ===================================<Assignment # 2>===================================== //

// a. Define a struct IOT_student with attributes (name, age, education).
// #[derive(Debug)]
// struct IotStudent{
//   name: String,
//   age: i32,
//   education: String,
// }

// // b. Define another struct IOT_instructor (name, age).

// #[derive(Debug)]
// struct IotInstructor{
//   name: String,
//   age: i32,
// }

// // c. Define a trait Questions with method ask_Questions with a default
// // implementation which prints (“​Zoom session will be LIVE, Zoom recording will
// // not be available. Quarter 2 studio recorded videos are available on Portal.​”).
// trait Questions {
//   fn ask_questions(&self, student_name: String)->String{
//     String::from("Zoom session will be LIVE, Zoom recording will not be available. Quarter 2 studio recorded videos are available on Portal.")
//   }
// }
// // d. Impl trait Questions for IOT_instructor which overrides the default implementation
// // of method ask_question, takes student name as a parameter and prints on
// // screen (“{} ​In case of any issue email to education@piaic.org​”).

// impl Questions for IotInstructor{
//   fn ask_questions(&self, student_name: String) ->String {
//     format!("{} ​In case of any issue email to education@piaic.org", student_name)
//   }
// }

// // e. Create instances of both the structs and call Method ask_question.

// fn main(){
// let student = IotStudent{
//   name: String::from("Orya"),
//   age: 25,
//   education: String::from("Electronics Engineering"),
// };
// let instructor = IotInstructor{
//   name: String::from("Sir Hanzala"),
//   age: 23,
// };
// println!("{}",instructor.ask_questions(student.name));
// }

// use std::fmt::Display;

// struct Pair<T>{
//   x:T,
//   y:T,
// }

// impl <T> Pair<T>{
//   fn new(x: T, y: T) -> Self {
//     Self{x, y}
//   }
// }

// impl <T: Display + PartialOrd> Pair<T>{
//   fn cmp_display(&self){
//     if self.x <= self.y{
//       println!("The smallest number is x = {}", self.x);
//     }else{
//       println!("The smallest number is y = {}", self.y);
//     }
//   }
// }
// fn main(){
//   let nums = Pair::new(3,1);
//   nums.cmp_display();
// }

// fn smallest <T: PartialOrd + Copy> (list: &[T]) -> T {
//   let mut smallest = list[0];

//   for &item in list {
//     if item < smallest {
//       smallest = item;
//     } 
//   }
//   smallest
// }
// fn main(){
//   let number_list = vec![32,50,25,100,65];
//   let result = smallest(&number_list);
//   println!("the smallest number is {}", result);
// }




