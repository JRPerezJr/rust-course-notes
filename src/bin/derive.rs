#[derive(Debug, Clone, Copy)] // Use only (Clone, Copy) for struct that are small in size
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)] // Use only (Clone, Copy) for struct that are small in size
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let slave = Employee {
        position: Position::Worker,
        work_hours: 120,
    };

    println!("{:?}", slave);
    print_employee(slave);
    print_employee(slave); // a copy
}
