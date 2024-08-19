mod types;

pub use types::*;

#[cfg(test)]
mod tests {
    use rust_xlsxwriter::*;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_all() {
        let json = json!([]);

        //println!("{:?}", json.is_array());

        let mut workbook = Workbook::new();

        let worksheet = workbook.add_worksheet();

        array(worksheet, json, "name").unwrap();

        workbook.save("test.xlsx").unwrap();
    }
}
