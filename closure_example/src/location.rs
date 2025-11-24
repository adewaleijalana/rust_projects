pub struct Location {
    name: String,
    treasure: u32,
}

impl Location {
    pub fn new(name: String, treasure: u32) -> Self {
        Self { name, treasure }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn treasure(&self) -> u32 {
        self.treasure
    }
}

pub struct Map<'a> {
    locations: &'a [Location],
}

impl<'a> Map<'a> {
    pub fn new(locations: &'a [Location]) -> Self {
        Self { locations }
    }

    pub fn explore<F>(&self, process: F)
    where
        F: FnMut(&Location),
    {
        let _ = &self.locations.iter().for_each(process);
    }
}
