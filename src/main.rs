use List::*;

// Определяем перечисление для связанного списка
enum List {
    Cons(u32, Box<List>),
    Nil,
}

// Реализация методов для перечисления List
impl List {
    // Создаём новый пустой список
    fn new() -> List {
        Nil
    }

    // Добавляем элемент в начало списка
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // Получаем длину списка
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // Представляем список в виде строки
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    // Создаём пустой связанный список
    let mut list = List::new();

    // Добавляем несколько элементов в список
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Выводим длину списка и его содержимое
    println!("Длина связанного списка: {}", list.len());
    println!("{}", list.stringify());
}