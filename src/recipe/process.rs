// Process Recipe Under Recipe(Process Map)

use crate::recipe::Recipe;

pub struct Process {
    pub id: String,
    pub recipe: Recipe,
    pub name: String,
    pub note: String,
    pub output_of: Option<Process>,
}

impl Process {
    pub fn new(
        id: &str,
        recipe: Recipe,
        name: &str,
        note: &str,
        output_of: Option<Process>,
    ) -> Process {
        Process {
            id: id.to_string(),
            recipe,
            name: name.to_string(),
            note: note.to_string(),
            output_of
        }
    }
}
