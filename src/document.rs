use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

imp Document {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::from("Hello, world!"));
        Self { rows }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}
