use std::fmt;

#[derive(Clone, Debug)]
pub struct FormField {
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub required: bool,
    pub masked: bool,
    pub options: Vec<String>,
    pub option_index: usize,
    pub readonly: bool,
}

impl FormField {
    pub fn new(label: &str, placeholder: &str, required: bool) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            placeholder: placeholder.into(),
            required,
            masked: false,
            options: Vec::new(),
            option_index: 0,
            readonly: false,
        }
    }

    pub fn select(label: &str, options: Vec<String>, required: bool) -> Self {
        let value = options.first().cloned().unwrap_or_default();
        Self {
            label: label.into(),
            value,
            placeholder: String::new(),
            required,
            masked: false,
            options,
            option_index: 0,
            readonly: false,
        }
    }

    pub fn readonly(label: &str, value: &str) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            placeholder: String::new(),
            required: true,
            masked: false,
            options: Vec::new(),
            option_index: 0,
            readonly: true,
        }
    }

    pub fn masked(label: &str, placeholder: &str, required: bool) -> Self {
        let mut field = Self::new(label, placeholder, required);
        field.masked = true;
        field
    }

    pub fn cycle_next(&mut self) {
        if !self.options.is_empty() {
            self.option_index = (self.option_index + 1) % self.options.len();
            self.value = self.options[self.option_index].clone();
        }
    }

    pub fn cycle_prev(&mut self) {
        if !self.options.is_empty() {
            if self.option_index == 0 {
                self.option_index = self.options.len() - 1;
            } else {
                self.option_index -= 1;
            }
            self.value = self.options[self.option_index].clone();
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.required {
            !self.value.trim().is_empty()
        } else {
            true
        }
    }
}

impl fmt::Display for FormField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.masked {
            write!(f, "{}", "*".repeat(self.value.len()))
        } else {
            write!(f, "{}", self.value)
        }
    }
}
