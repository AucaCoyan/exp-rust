struct Employee {
    emp_id: u32,
    emp_name: String,
}

impl Employee {
    fn show_details(self: &Self) {
        println!("The employee ID is {}", self.emp_id);
        println!("The employee name is {}", self.emp_name);
    }

    fn mutate_and_show_details(&mut self, new_name: String) {
        self.emp_name = new_name;
        println!(
            "The new name for the ID {}, is {}",
            self.emp_id, self.emp_name
        );
    }

    fn create_employee(id: u32, name: String) -> Employee {
        Employee {
            emp_id: id,
            emp_name: name,
        }
    }
}

fn main() {
    let employee1 = Employee {
        emp_id: 1,
        emp_name: "Anmol".to_string(),
    };
    employee1.show_details();

    let employee2 = &mut Employee {
        emp_id: 1,
        emp_name: "Anmol".to_string(),
    };
    employee2.mutate_and_show_details("Bhavya".to_string());
    employee2.show_details();

    let employee3 = Employee::create_employee(2, "Ayush".to_string());
    employee3.show_details();
}
