use prettytable::{Cell, Row, Table};
use serde_json::{json, Map, Value};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let report_path = "report.json";
    let mut report = if let Ok(mut file) = File::open(report_path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            serde_json::from_str(&contents).unwrap_or(json!({}))
        } else {
            json!({})
        }
    } else {
        json!({})
    };
    let mut report = report.as_object_mut().unwrap();

    let args: Vec<String> = env::args().collect();

    // Re-run test with label
    if args.len() > 1 {
        let label = &args[1];
        test(&mut report, label);
        let mut file = File::create(report_path).expect("Failed to create file");
        file.write_all(serde_json::to_string(&report).unwrap().as_bytes())
            .expect("Write failed");
    }

    // generate table
    let mut table = Table::new();
    let labels = report.keys().cloned().collect::<Vec<_>>();

    // header
    let slash_cell = "Test Cases \\ Label";
    let header: Vec<Cell> = std::iter::once(Cell::new(slash_cell))
        .chain(labels.iter().map(|k| Cell::new(k.as_str())))
        .collect();
    table.add_row(Row::new(header));

    // content
    if let Some(first_value) = report.values().next() {
        if let Some(dict) = first_value.as_object() {
            for key in dict.keys() {
                let case: Vec<Cell> = std::iter::once(Cell::new(key))
                    .chain(labels.iter().map(|label| {
                        Cell::new(
                            &report
                                .get(label)
                                .unwrap()
                                .get(key)
                                .unwrap()
                                .to_string()
                                .trim_matches('"'),
                        )
                    }))
                    .collect();
                table.add_row(Row::new(case));
            }
        }
    }
    println!("{}", table);
}

fn handle_test_result<T>(
    name: &str,
    result: std::io::Result<T>,
    report: &mut Map<String, Value>,
    label: &str,
) {
    let v = report.entry(label.to_owned()).or_insert(json!({}));
    match result {
        Ok(_) => {
            v[name] = json!("o");
        }
        Err(error) => {
            eprintln!("Failed to '{}': {}", name, error);
            v[name] = json!("x");
        }
    }
}

fn test(report: &mut Map<String, Value>, label: &str) {
    println!("Re-run test for: {}", label);
    let root = env::current_dir().expect("Failed to get current directory");
    let readonly_folder = root.join("readonlyFolder");

    // Create a new folder
    let new_folder = readonly_folder.join("new_folder");
    handle_test_result(
        "1. create a new sub folder",
        fs::create_dir(&new_folder),
        report,
        label,
    );

    // Rename sub folder
    let sub_folder = readonly_folder.join("subFolder");
    let renamed_folder = readonly_folder.join("renamed_folder");
    handle_test_result(
        "2. rename the sub folder",
        fs::rename(&sub_folder, &renamed_folder),
        report,
        label,
    );

    // Delete sub folder
    let del_folder = readonly_folder.join("subFolder2");
    handle_test_result(
        "3. delete the sub folder",
        fs::remove_dir_all(del_folder),
        report,
        label,
    );

    // Reading folder contents
    handle_test_result(
        "4. read folder",
        fs::read_dir(&readonly_folder),
        report,
        label,
    );

    // Writing to the folder
    let new_file = readonly_folder.join("new.txt");
    handle_test_result(
        "5. write into folder",
        fs::File::create(&new_file),
        report,
        label,
    );

    let mut permissions = fs::metadata(&readonly_folder).unwrap().permissions();
    report[label]["6. readonly (metadata) before changed"] = json!(permissions.readonly());
    // Modifying folder permissions
    permissions.set_readonly(!permissions.readonly());
    handle_test_result(
        "7. set permissions",
        fs::set_permissions(&readonly_folder, permissions),
        report,
        label,
    );
    let permissions = fs::metadata(&readonly_folder).unwrap().permissions();
    report[label]["8. readonly (metadata) after changed"] = json!(permissions.readonly());
}
