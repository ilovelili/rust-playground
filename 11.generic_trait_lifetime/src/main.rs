fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);

  let article = NewArticle {
    title: String::from("today's news"),
    content: String::from("stock price"),
    author: String::from("min"),
  };

  println!("{}", article.summarize());
  notify(&article);
}

// The largest function has a parameter called list,
// which represents any concrete slice of i32 values that we might pass into the function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

pub trait Summary {
  fn summarize(&self) -> String {
    String::from("read more ...")
  }
}

struct NewArticle {
  title: String,
  content: String,
  author: String,
}

impl Summary for NewArticle {
  fn summarize(&self) -> String {
    format!("{}:{}-{}", self.author, self.title, self.content)
  }
}

fn notify(item: &impl Summary) {
  println!("{}", item.summarize())
}

// return types implements a trait
fn summarizable_item() -> impl Summary {
  NewArticle {
    title: String::from("today's news"),
    content: String::from("stock price"),
    author: String::from("min"),
  }
}

// fn lifetime_v1() {
//   {
//     let r;

//     {
//       let x = 5;
//       r = &x; // error
//     }

//     println!("r: {}", r);
//   }
// }

// fn lifetime_v2(a: &str, b: &str) -> &str {
//   if a.len() > b.len() {
//     a
//   }
//   b
// }
