use std::process::Command;
use std::fs;

// Hardcoded credentials (CWE-798)
const DB_PASSWORD: &str = "admin123";
const API_KEY: &str = "sk-hardcoded-secret-key-12345";

// SQL Injection via string formatting (CWE-89)
fn get_user(username: &str) -> String {
    let query = format!("SELECT * FROM users WHERE username = '{}'", username);
    println!("Executing: {}", query);
    query
}

// Command Injection via unsanitized user input (CWE-78)
fn run_ping(host: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("ping -c 1 {}", host))
        .output()
        .expect("Failed to execute");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

// Path Traversal (CWE-22)
fn read_user_file(filename: &str) -> String {
    let path = format!("/var/app/uploads/{}", filename);
    fs::read_to_string(path).unwrap_or_default()
}

// Unsafe memory operations (CWE-119)
fn unsafe_pointer_ops() {
    let data: Vec<u8> = vec![1, 2, 3];
    unsafe {
        let ptr = data.as_ptr();
        // Out-of-bounds read
        let val = *ptr.offset(100);
        println!("Value: {}", val);
    }
}

// Integer overflow (CWE-190)
fn calculate_buffer_size(user_input: u32) -> u32 {
    user_input * 1024 * 1024  // Can overflow for large inputs
}

// Panic on unwrap without error handling (CWE-248)
fn parse_input(input: &str) -> i32 {
    input.parse::<i32>().unwrap()  // Panics on invalid input
}

// Insecure random number generation for security-sensitive use
fn generate_token() -> u32 {
    // Using non-cryptographic randomness for a security token
    let seed: u32 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    seed ^ 0xDEADBEEF
}

// Sensitive data in logs (CWE-532)
fn authenticate(user: &str, password: &str) -> bool {
    println!("Authenticating user: {} with password: {}", user, password);
    password == DB_PASSWORD
}

fn main() {
    // Command injection example
    run_ping("google.com; cat /etc/passwd");

    // SQL injection example
    get_user("admin' OR '1'='1");

    // Path traversal example
    read_user_file("../../etc/passwd");

    // Unsafe memory
    unsafe_pointer_ops();

    // Integer overflow
    let size = calculate_buffer_size(4096);
    println!("Buffer size: {}", size);

    // Token generation
    println!("Token: {}", generate_token());

    // Credential leak in logs
    authenticate("admin", DB_PASSWORD);

    println!("API_KEY in use: {}", API_KEY);
}
