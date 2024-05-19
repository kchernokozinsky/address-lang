use super::*;
use serde_json;
use std::fs::File;
use std::io::{self, Read, Write};

pub fn serialize_ast(ast: &Algorithm) -> Result<String, serde_json::Error> {
    serde_json::to_string(ast)
}

pub fn deserialize_ast(json: &str) -> Result<Algorithm, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn serialize_ast_to_file(ast: &Algorithm, file_path: &str) -> io::Result<()> {
    let serialized = serialize_ast(ast).expect("Serialization failed");
    let mut file = File::create(file_path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn deserialize_ast_from_file(file_path: &str) -> io::Result<Algorithm> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let deserialized = deserialize_ast(&contents).expect("Deserialization failed");
    Ok(deserialized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::location::Location;
    use std::fs;

    #[test]
    fn test_serialize_deserialize() {
        let expr = Located {
            l_location: Location::default(),
            r_location: Location::default(),
            node: ExpressionKind::Int { value: 42 },
        };

        let stmt = OneLineStatement {
            l_location: Location::default(),
            r_location: Location::default(),
            node: OneLineStatementKind::Return,
        };

        let file_line = FileLine::Line {
            labels: vec!["label1".to_string()],
            statements: Statements::OneLineStatement(stmt.clone()),
        };

        let algorithm = Algorithm::Body(vec![file_line]);

        // Serialize
        let serialized = serialize_ast(&algorithm).expect("Serialization failed");

        // Deserialize
        let deserialized: Algorithm = deserialize_ast(&serialized).expect("Deserialization failed");

        // Ensure the original and deserialized objects are the same
        assert_eq!(algorithm, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_file() {
        let expr = Located {
            l_location: Location::default(),
            r_location: Location::default(),
            node: ExpressionKind::Int { value: 42 },
        };

        let stmt = OneLineStatement {
            l_location: Location::default(),
            r_location: Location::default(),
            node: OneLineStatementKind::Return,
        };

        let file_line = FileLine::Line {
            labels: vec!["label1".to_string()],
            statements: Statements::OneLineStatement(stmt.clone()),
        };

        let algorithm = Algorithm::Body(vec![file_line]);

        // File path
        let file_path = "test_algorithm.json";

        // Serialize to file
        serialize_ast_to_file(&algorithm, file_path).expect("Serialization to file failed");

        // Deserialize from file
        let deserialized =
            deserialize_ast_from_file(file_path).expect("Deserialization from file failed");

        // Ensure the original and deserialized objects are the same
        assert_eq!(algorithm, deserialized);

        // Clean up
        fs::remove_file(file_path).expect("Failed to remove test file");
    }
}
