pub struct DentistAppointment {
    doctor: String,
}

impl DentistAppointment {
    pub fn new(doctor: String) -> Self {
        Self { doctor }
    }
    pub fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
        println!(
            "You are booked between {check_in_time} and {check_out_time} with doctor: {}",
            self.doctor
        );
        &self.doctor
    }
}
