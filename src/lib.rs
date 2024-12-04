/// Walks the current directory recursively and prints the path of each file.
/// This function is used for testing.
///
/// # Examples
///
/// ```
///
/// use nutek_code_new_project::walk_dir_recursively_with_ignore;
///
/// walk_dir_recursively_with_ignore("");
/// ```
pub fn walk_dir_recursively_with_ignore(path: &str) -> String {
    use ignore::WalkBuilder;

    let mut walk_result = String::new();
    if path.is_empty() {
        for result in WalkBuilder::new("./").hidden(false).build() {
            match result {
                Ok(entry) => {
                    String::push_str(
                        &mut walk_result,
                        format!("{}\n", entry.path().display()).as_str(),
                    );
                }
                Err(err) => {
                    panic!("ERROR: {}", err)
                }
            }
        }
        return walk_result;
    } else {
        for result in WalkBuilder::new(std::path::Path::new(&path))
            .hidden(false)
            .build()
        {
            match result {
                Ok(entry) => {
                    String::push_str(
                        &mut walk_result,
                        format!("{}\n", entry.path().display()).as_str(),
                    );
                }
                Err(err) => {
                    panic!("ERROR: {}", err)
                }
            }
        }
        return walk_result;
    }
}

/// Checks if the given path is a text file.
///
/// # Examples
///
/// ```
/// use nutek_code_new_project::is_text_file;
///
/// assert_eq!(is_text_file("./Cargo.toml"), true);
/// ```
pub fn is_text_file(path: &str) -> bool {
    use mime_guess;
    use std::io::Read;

    // the file doesn't have to exist, it just looks at the path
    let guess = mime_guess::from_path(path);
    if guess.is_empty() == false {
        if guess.first().unwrap().to_string().starts_with("text/") == true {
            return true;
        }
        return false;
    } else {
        let file = std::fs::File::open(path).expect("Failed to open file while checking it's type");
        let mut reader = std::io::BufReader::new(file);

        let mut buffer = [0u8; 8000]; // Allocate a buffer of 8000 bytes
        let bytes_read = reader.read(&mut buffer).expect("Failed to read file");

        // Check for null bytes within the first 8000 bytes
        if !buffer[..bytes_read].iter().any(|&byte| byte == 0) {
            return true;
        } else {
            return false;
        }
    }
}

/// Returns the exact mime type of the file at the given path.
/// This function is not guaranteed to be accurate.
///
/// # Examples
///
/// ```
/// use nutek_code_new_project::get_exact_mime_type;
///
/// assert_eq!(get_exact_mime_type("Cargo.toml"), "text/x-toml");
/// ```
pub fn get_exact_mime_type(path: &str) -> String {
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    mime.to_string()
}

/// Returns the size of the file at the given path.
///
/// # Examples
///
/// ```
/// use nutek_code_new_project::file_size;
/// std::fs::write("test.txt", "hi");
/// assert_eq!(file_size(&std::path::Path::new("test.txt")), 2);
/// std::fs::remove_file("test.txt").unwrap();
/// ```
///
/// # Why? Gemini spoke...
///
/// **Limiting File Size and Offloading Large Files to Cloud Storage**
///
/// **Understanding the Problem:**
///
/// You want to:
///
/// 1. **Limit file uploads:** Prevent users from uploading files larger than 1GB directly to your database.
/// 2. **Offload large files:** Store large files in a cloud storage service like Google Cloud Storage.
/// 3. **Reference files in the database:** Store a reference (e.g., a URL or file ID) to the large file in the database.
/// 4. **Retrieve files on demand:** Fetch the large file from the cloud storage service when needed.
///
/// **Implementing the Solution:**
///
/// Here's a general approach:
///
/// 1. **Frontend Validation:**
///    - Implement client-side validation to prevent users from selecting files larger than 1GB.
///    - Use JavaScript or other frontend technologies to check file sizes before uploading.
///
/// 2. **Backend Validation:**
///    - Before processing the file upload, check the file size on the server.
///    - If the file is larger than 1GB, reject the upload or redirect it to a different endpoint.
///
/// 3. **Offloading to Cloud Storage:**
///    - If the file is too large, upload it to Google Cloud Storage using the appropriate libraries (e.g., `google-cloud-storage` crate for Rust).
///    - Store the uploaded file's URL or a unique identifier in your database.
///
/// 4. **Database Storage:**
///    - Store the following information in your database:
///      - User ID
///      - File name
///      - File size
///      - File type (MIME type)
///      - URL or identifier of the file in cloud storage
///
/// 5. **Retrieving Files on Demand:**
///    - When you need to access a large file:
///      - Retrieve the file's URL or identifier from the database.
///      - Use the appropriate library to fetch the file from Google Cloud Storage.
///      - Stream the file to the user or process it as needed.
///
/// **Rust Implementation:**
///
/// Here's a simplified example using the `google-cloud-storage` crate to upload files to Google Cloud Storage:
///
/// // ```rust
/// // use google_cloud_storage::Client;
/// //
/// // ...
/// //
/// // fn upload_to_gcs(file_path: &str, gcs_client: &Client, bucket_name: &str) -> Result<String, Box<dyn std::error::Error>> {
/// //    let object_id = gcs_client.bucket(bucket_name).object(file_path);
/// //    object_id.upload_from_path(file_path)?;
/// //
/// //    let public_url = object_id.signed_url(google_cloud_storage::SignedUrlExpiration::OneHour)?;
/// //    Ok(public_url.to_string())
/// // }
/// /// ```
///
/// **Key Considerations:**
///
/// - **Error Handling:** Implement robust error handling to handle potential issues during file uploads, downloads, and database operations.
/// - **Security:** Ensure proper security measures to protect user data and prevent unauthorized access to files in cloud storage.
/// - **Performance:** Optimize file transfers and database queries to minimize latency and improve performance.
/// - **Cost:** Consider the costs associated with cloud storage and data transfer, especially for large-scale applications.
/// - **Scalability:** Design your system to handle increasing numbers of files and users.
///
/// By following these guidelines and considering the specific requirements of your application, you can effectively limit file sizes, offload large files to cloud storage, and manage file retrieval efficiently.
///
pub fn file_size(path: &std::path::Path) -> u64 {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer.len() as u64
}

/// Returns the size of the file at the given path.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use std::fs::write;
/// use nutek_code_new_project::human_readable_size;
/// let mut file = std::fs::File::open("test.txt").unwrap();
/// write(Path::new("test.txt"), b"test").unwrap();
/// assert_eq!(human_readable_size(Path::new("test.txt")), "4.0B");
/// std::fs::remove_file("test.txt").unwrap();
/// ```
pub fn human_readable_size(path: &std::path::Path) -> String {
    // let mut file = std::fs::File::open("test.txt").unwrap();
    // file.write_all(b"test").unwrap();
    // assert_eq!(
    //     human_readable_size(&Path::new("test.txt")),
    //     "4.0B"
    // );
    let num_bytes = std::fs::File::open(path).unwrap().metadata().unwrap().len();
    // my implementation, might use num_bigint::BigUint
    use std::fmt::Write;
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut i = 0;
    let mut size_f = num_bytes as f64;

    while size_f >= 1024.0 {
        size_f /= 1024.0;
        i += 1;
    }

    let mut result = String::new();
    write!(result, "{:.1}{}", size_f, units[i]).unwrap();
    result
}

/// Returns a string containing the difference between two text strings.
/// The difference is calculated using the `diff` crate.
/// The result is a string containing the difference between two text strings.
/// # Examples
/// ```
/// use nutek_code_new_project::diff_in_text_string;
/// diff_in_text_string("Hello World", "Hello Nutek!");
/// // Output:
/// // 1  -Hello World
/// // 1  +Hello Nutek!
/// ```
pub fn diff_in_text_string(left: &str, right: &str) -> String {
    use diff;

    let mut result = String::new();
    let mut count: u64 = 0;
    for difference in diff::lines(left, right) {
        count = count + 1;
        match difference {
            diff::Result::Left(l) => result.push_str(&format!("{} -{}\n", count, l)),
            diff::Result::Both(l, _) => result.push_str(&format!("{}  {}\n", count, l)),
            diff::Result::Right(r) => {
                count = count - 1;
                result.push_str(&format!("{} +{}\n", count, r))
            }
        }
    }
    result
}

/// Returns a HashMap of the differences between two strings.
/// The keys are the number of lines that are different, count od  not changed
/// characters also removed and added characters from left and right strings.
/// # Examples
/// ```
/// use nutek_code_new_project::diff_in_text_summary_hashmap;
/// println!("{:?}", diff_in_text_summary_hashmap("Hello World", "Hello Nutek!"))
/// // Output: HashMap { "added": 6, "removed": 5, "not_changed": 6, "lines": 1 }
/// ```
pub fn diff_in_text_summary_hashmap(
    left: &str,
    right: &str,
) -> std::collections::HashMap<String, u64> {
    use diff;
    use std::collections::HashMap;

    let mut key_map: HashMap<String, u64> = HashMap::new();
    let mut added: u64 = 0;
    let mut removed: u64 = 0;
    let mut not_changed: u64 = 0;
    let mut lines: u64 = 0;

    for line in diff::lines(left, right) {
        lines = lines + 1;
        match line {
            diff::Result::Left(l) => {
                removed = removed + l.chars().count() as u64;
            }
            diff::Result::Both(b, _) => {
                not_changed = not_changed + b.chars().count() as u64;
            }
            diff::Result::Right(r) => {
                lines = lines - 1;
                added = added + r.chars().count() as u64;
            }
        }
    }
    key_map.insert("removed".to_string(), removed);
    key_map.insert("not changed".to_string(), not_changed);
    key_map.insert("added".to_string(), added);
    key_map.insert("lines".to_string(), lines);
    key_map
}

/// Get sha256 hash of a file
///
/// # Examples
/// ```
/// use std::io::Write;
/// use nutek_code_new_project::hash_of_file;
/// let mut file = std::fs::File::create("test.txt").unwrap();
/// file.write_all(b"Hello Nutek!").unwrap();
/// let result = hash_of_file("test.txt");
/// assert_eq!(
///    result,
///    "ad5fe770e5f0a3c389ac1ec390d3acb3f430af6f7dee53a76ca64121f715f8a6"
/// );
/// std::fs::remove_file("test.txt").unwrap();
/// ```
pub fn hash_of_file(file: &str) -> String {
    let result = sha256::try_digest(file);
    match result {
        Ok(hash) => hash.to_string(),
        Err(e) => format!("Error hashing {} file: {}", file, e),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile::{tempdir, NamedTempFile};
    use walkdir::WalkDir;

    #[tokio::test]
    async fn it_walks_dir_recursively_with_ignore() {
        walk_dir_recursively_with_ignore("");
        walk_dir_recursively_with_ignore("target");
    }

    #[tokio::test]
    async fn it_is_text_file() {
        use tokio::fs::write;

        assert!(is_text_file("./Cargo.toml"));
        assert!(is_text_file("./src/lib.rs"));
        let file = NamedTempFile::new().unwrap();
        assert!(is_text_file("./.gitignore"));
        write(file.path(), "text_plain").await.unwrap();
        assert!(is_text_file(file.path().to_str().unwrap()));
        assert!(is_text_file("./Cargo.lock"));
        for entry in WalkDir::new("target/debug/deps/") {
            if let Some(entry) = entry.ok() {
                if entry.path().starts_with("nutek_code_new_project-")
                    && !entry.path().to_str().unwrap().contains(".")
                {
                    assert!(!is_text_file(entry.path().to_str().unwrap()));
                }
            }
        }
    }

    #[tokio::test]
    async fn it_is_binary_file() {
        for entry in WalkDir::new("target/debug/deps/") {
            if let Some(entry) = entry.ok() {
                if entry.path().starts_with("nutek_code_new_project-")
                    && !entry.path().to_str().unwrap().contains(".")
                {
                    assert!(!is_text_file(entry.path().to_str().unwrap()));
                }
            }
        }
    }

    #[tokio::test]
    async fn it_is_exact_mime_type() {
        use tokio::fs::{remove_file, write};
        write("test_get_exact_mime_type.rs", "test").await.unwrap();
        assert!(get_exact_mime_type("test_get_exact_mime_type.rs").contains("rust"));
        remove_file("test_get_exact_mime_type.rs").await.unwrap();
    }

    #[tokio::test]
    async fn it_is_file_size() {
        use tokio::fs::write;
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        write(file_path, "test").await.unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        assert_eq!(file_size(file_path.as_path()), 4);
        let file_path = tmp_dir.path().join("test.txt");
        write(file_path, "🎶").await.unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        assert_eq!(file_size(file_path.as_path()), 4);
        tmp_dir.close().unwrap();
    }

    #[tokio::test]
    async fn it_is_human_readable_size() {
        use tokio::fs::write;

        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        write(file_path, "test").await.unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        assert_eq!(human_readable_size(file_path.as_path()), "4.0B");
        let file_path = tmp_dir.path().join("test.txt");
        write(file_path, "🎶").await.unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        assert_eq!(human_readable_size(file_path.as_path()), "4.0B");
        tmp_dir.close().unwrap();
    }

    #[tokio::test]
    async fn it_is_diff_in_text_string() {
        use tokio::fs::{read_to_string, write};

        let file1 = tempfile::NamedTempFile::new().unwrap();
        let file2 = tempfile::NamedTempFile::new().unwrap();
        write(file1.path(), "test\n4321\nHello Nutek!\n")
            .await
            .unwrap();
        write(file2.path(), "test\nHello World!\nHello Nutek!\n")
            .await
            .unwrap();
        let test1 = read_to_string(file1.path()).await.unwrap();
        let test2 = read_to_string(file2.path()).await.unwrap();
        let result = diff_in_text_string(&test1, &test2);
        assert_eq!(
            result,
            "1  test\n2 -4321\n2 +Hello World!\n3  Hello Nutek!\n4  \n"
        );
    }

    #[tokio::test]
    async fn test_diff_in_text_summary_json() {
        use tokio::fs::{read_to_string, write};

        let tmp_dir = tempdir().unwrap();
        let file_path1 = tmp_dir.path().join("left.txt");
        write(file_path1, b"test\n4321\nHello Nutek!\n")
            .await
            .unwrap();
        let file_path2 = tmp_dir.path().join("right");
        write(file_path2, b"test\nHello World!\nHello Nutek!\n")
            .await
            .unwrap();
        let file_path1 = tmp_dir.path().join("left.txt");
        assert!(is_text_file(file_path1.to_str().unwrap()));
        let file_path2 = tmp_dir.path().join("right");
        assert!(is_text_file(file_path2.to_str().unwrap()));
        let left = read_to_string(tmp_dir.path().join("left.txt"))
            .await
            .unwrap();
        let right = read_to_string(tmp_dir.path().join("right")).await.unwrap();
        let result = diff_in_text_summary_hashmap(&left, &right);
        // eprintln!("{:?}", result);
        let expected = std::collections::HashMap::from([
            ("added".to_string(), 12),
            ("removed".to_string(), 4),
            ("not changed".to_string(), 16),
            ("lines".to_string(), 4),
        ]);
        // eprintln!("{:?}", expected);
        assert_eq!(expected, result);
        // assert_eq!(
        //     result,
        //     "1  test\n2 -4321\n2 +Hello World!\n3  Hello Nutek!\n4  \n"
        // );
        eprintln!(
            "{:?}",
            diff_in_text_summary_hashmap("Hello World", "Hello Nutek!")
        );
        tmp_dir.close().unwrap();
    }

    #[tokio::test]
    async fn it_is_hash_of_file() {
        use tokio::fs::write;

        let file = NamedTempFile::new().unwrap();
        write(file.path(), "Hello Nutek!").await.unwrap();
        let result = hash_of_file(file.path().to_str().unwrap());
        assert_eq!(
            result,
            "ad5fe770e5f0a3c389ac1ec390d3acb3f430af6f7dee53a76ca64121f715f8a6"
        );
        let non_existent_file = std::path::Path::new("test_non_existent_file.rs");
        let result = hash_of_file(non_existent_file.to_str().unwrap());
        // eprintln!("{}", result);
        assert_eq!(
            result,
            "Error hashing test_non_existent_file.rs file: No such file or directory (os error 2)"
        );
        let should_fail_result_is_dir = hash_of_file("./target");
        assert_eq!(
            should_fail_result_is_dir,
            "Error hashing ./target file: Is a directory (os error 21)"
        );
    }

    #[test]
    fn test_std_fs_write() {
        std::fs::write("test.txt", "Hello World!").unwrap();
    }
}
