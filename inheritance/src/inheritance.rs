pub trait Printable {
    fn print(&self);
}

pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub id: usize,
}

impl Printable for Person {
    fn print(&self) {
        println!("Name: {}, {}", self.last_name, self.first_name);
        println!("ID: {}", self.id);
    }
}

pub struct Student {
    person: Person,
    pub scores: Vec<u8>,
}

impl Student {
    pub fn new(first_name: String, last_name: String, id: usize, scores: Vec<u8>) -> Student {
        Student {
            person: Person { first_name, last_name, id },
            scores: scores
        }
    }

    fn average(&self) -> u8 {
        let sum: usize = self.scores.iter().map(|u| *u as usize).sum();
        return (sum/self.scores.len()) as u8;
    }

    fn score(&self) -> &'static str {
        return match self.average() {
            90..=100 => "O",
            80...90   => "E",
            70...80   => "A",
            55...70   => "P",
            40...55   => "D",
            0...40    => "T",
            _        => panic!("Tried matching integer not in [0,100]")
        }
    }
}

impl Printable for Student {
    fn print(&self) {
        self.person.print();
        println!("Grade: {}", self.score())
    }
}