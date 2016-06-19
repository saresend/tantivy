use super::*;
    

///
/// Document are really just a list of field values.
///
#[derive(Debug)]
pub struct Document {
    field_values: Vec<FieldValue>,
}

impl Document {

    pub fn new() -> Document {
        Document {
            field_values: Vec::new(),
        }
    }
    
    pub fn len(&self,) -> usize {
        self.field_values.len()
    }

    pub fn add_text(&mut self, field: &Field, text: &str) {
        self.add(FieldValue::Text(field.clone(), String::from(text)));
    }

    pub fn add_u32(&mut self, field: &Field, value: u32) {
        self.add(FieldValue::U32(field.clone(), value));
    }

    pub fn add(&mut self, field_value: FieldValue) {
        self.field_values.push(field_value);
    }
           
    pub fn get_fields(&self) -> &Vec<FieldValue> {
        &self.field_values
    }
    
    pub fn get_all<'a>(&'a self, field: Field) -> Vec<&'a FieldValue> {
        self.field_values
            .iter()
            .filter(|field_value| field_value.field() == field)
            .collect()
    }

    pub fn get_first<'a>(&'a self, field: Field) -> Option<&'a FieldValue> {
        self.field_values
            .iter()
            .filter(|field_value| field_value.field() == field)
            .next()
    }
}

impl From<Vec<FieldValue>> for Document {
    fn from(field_values: Vec<FieldValue>) -> Document {
        Document {
            field_values: field_values
        }
    }
}