#[derive(Debug)]
pub struct Student {
    id: u32,
    first_name: String,
    last_name: String,
    class: String,
    age: u8,
    state_of_origin: String,
}

impl Student {
    pub fn new(
        id: u32,
        first_name: String,
        last_name: String,
        class: String,
        age: u8,
        state_of_origin: String,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            class,
            age,
            state_of_origin,
        }
    }

    pub fn parse_student(student: String) -> Option<Self> {
        let mut student_record = student.split(",");
        let id = student_record.next()?.parse::<u32>().ok()?;
        let first_name = student_record.next()?.to_string();
        let last_name = student_record.next()?.to_string();
        let class = student_record.next()?.to_string();
        let age = student_record.next()?.parse::<u8>().ok()?;
        let state_of_origin = student_record.next()?.to_string();

        Some(Self {
            id,
            first_name,
            last_name,
            class,
            age,
            state_of_origin,
        })
    }
}
