#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index(usize);

impl Index {
    pub fn get(self) -> usize {
        self.0
    }

    pub fn from_field_name(value: &str) -> Option<Self> {
        value
            .strip_prefix("db_bind_")
            .and_then(|suffix| suffix.parse::<usize>().ok())
            .map(Self)
    }

    pub fn from_pill_text(value: &str) -> Option<Self> {
        let rest = value.strip_prefix('$')?;
        let (index, _) = rest.split_once('=')?;
        index.parse::<usize>().ok().map(Self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reference {
    start: usize,
    end: usize,
    index: Index,
}

impl Reference {
    pub fn start(self) -> usize {
        self.start
    }

    pub fn end(self) -> usize {
        self.end
    }

    pub fn index(self) -> Index {
        self.index
    }
}

pub fn summary_references(summary: &str) -> Vec<Reference> {
    let bytes = summary.as_bytes();
    let mut cursor = 0usize;
    let mut references = Vec::new();

    while cursor < bytes.len() {
        if bytes[cursor] != b'$' {
            cursor += 1;
            continue;
        }

        let mut end = cursor + 1;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }

        let Some(index) = summary[cursor + 1..end].parse::<usize>().ok().map(Index) else {
            cursor += 1;
            continue;
        };

        references.push(Reference {
            start: cursor,
            end,
            index,
        });
        cursor = end;
    }

    references
}

#[cfg(test)]
mod tests {
    use super::{Index, summary_references};

    #[test]
    fn parses_field_name_indexes() {
        assert_eq!(Index::from_field_name("db_bind_2").map(Index::get), Some(2));
        assert_eq!(Index::from_field_name("status").map(Index::get), None);
    }

    #[test]
    fn parses_pill_indexes() {
        assert_eq!(Index::from_pill_text("$4=room-1").map(Index::get), Some(4));
        assert_eq!(Index::from_pill_text("room-1").map(Index::get), None);
    }

    #[test]
    fn finds_summary_references() {
        let references =
            summary_references("SELECT * FROM rooms WHERE id = $1 AND user_id = $2");
        let indexes: Vec<usize> = references
            .into_iter()
            .map(|value| value.index().get())
            .collect();
        assert_eq!(indexes, vec![1, 2]);
    }
}
