# macOS Stealer - Ethical Research Tool

**Disclaimer:** This tool is provided for educational and ethical research purposes only. The use of this software for any malicious or illegal activities is strictly prohibited. The developers are not responsible for any misuse of this software. By using this software, you agree to use it responsibly and ethically.

## Overview

This Rust-based application is designed to collect various types of data from a macOS system. It is intended to be used for security research, penetration testing (with explicit permission), and educational purposes to understand how such tools operate. The application gathers system information, browser data (including extensions), and cryptocurrency wallet data. It then encrypts and sends this data to a remote server.

**Important Note:** This tool is a proof-of-concept and should not be used in production environments or on systems without explicit consent.

## Functionality

The application performs the following actions:

1.  **Anti-VM and Anti-Debugger Checks:**
    *   Checks if the program is running inside a virtual machine.
    *   Detects if a debugger is attached to the process.
    *   These checks are implemented in `src/antivm.rs` (startLine: 1, endLine: 53).

2.  **System Information Gathering:**
    *   Collects the OS version, CPU information, and memory details.
    *   This information is gathered using system commands and is implemented in `src/system_info.rs` (startLine: 1, endLine: 8).

3.  **Data Collection:**
    *   **Browser Data:** Collects data from various browsers, including Chrome, Firefox, Brave, Edge, Vivaldi, Yandex, Opera, and Opera GX. This includes:
        *   Browser profiles
        *   Autofill data
        *   Browsing history
        *   Cookies
        *   Login credentials
        *   Local extension settings for specific cryptocurrency wallets.
        *   The browser data collection logic is implemented in `src/browsers.rs` (startLine: 1, endLine: 200).
    *   **Wallet Data:** Collects data from various cryptocurrency wallets, including Exodus, Electrum, Coinomi, Guarda, Wasabi, Atomic, and Ledger Live.
        *   The wallet data collection logic is implemented in `src/wallets.rs` (startLine: 1, endLine: 39).

4.  **Data Packaging:**
    *   Creates a temporary directory to store collected data.
    *   Copies all collected data into this temporary directory.
    *   Creates a ZIP archive of the temporary directory.
    *   The temporary directory creation, file copying, and zip archive creation logic is implemented in `src/tools.rs` (startLine: 1, endLine: 124).

5.  **Data Encryption:**
    *   Encrypts the ZIP archive using a simple XOR cipher with a predefined key.
    *   The encryption logic is implemented in `src/sender.rs` (startLine: 22, endLine: 28).

6.  **Data Transmission:**
    *   Sends the encrypted data to a remote server via a TCP connection.
    *   The data sending logic is implemented in `src/sender.rs` (startLine: 30, endLine: 37).

7.  **Controller Mode:**
    *   The application can also run in a "controller" mode, which currently only prints a message with the encryption key. This mode is triggered by passing `run_controller` as a command-line argument.
    *   The controller logic is implemented in `src/controller.rs` (startLine: 1, endLine: 16).

## Code Structure

The project is organized into several modules:

*   `src/main.rs`: The main entry point of the application. It handles command-line arguments, spawns the controller process, and orchestrates the data collection and sending process. (startLine: 1, endLine: 55)
*   `src/config.rs`: Contains configuration constants such as the encryption key and remote server IP. (startLine: 1, endLine: 2)
*   `src/tools.rs`: Provides utility functions for executing shell commands, creating directories, copying files, creating ZIP archives, and managing temporary files. (startLine: 1, endLine: 124)
*   `src/antivm.rs`: Implements anti-virtual machine and anti-debugger checks. (startLine: 1, endLine: 53)
*   `src/system_info.rs`: Collects system information. (startLine: 1, endLine: 8)
*   `src/browsers.rs`: Collects data from various web browsers. (startLine: 1, endLine: 200)
*   `src/wallets.rs`: Collects data from various cryptocurrency wallets. (startLine: 1, endLine: 39)
*   `src/sender.rs`: Handles data encryption and sending to the remote server. (startLine: 1, endLine: 37)
*   `src/controller.rs`: Implements the controller mode of the application. (startLine: 1, endLine: 16)

## How to Build and Run

1.  **Prerequisites:**
    *   Rust toolchain installed (`rustc`, `cargo`).
    *   Basic understanding of Rust programming.

2.  **Build:**
    ```bash
    cargo build --release
    ```

3.  **Run:**
    ```bash
    ./target/release/mac-os-stealer
    ```
    This will execute the program, collect data, and send it to the configured remote server.

    To run the controller mode:
    ```bash
    ./target/release/mac-os-stealer run_controller <encryption_key>
    ```
    Replace `<encryption_key>` with the actual encryption key.

## Configuration

The following configurations can be found in `src/config.rs`:

*   `ENCRYPTION_KEY`: The key used for XOR encryption. **Important:** This is a simple encryption method and should not be used for sensitive data in a real-world scenario.
*   `REMOTE_IP`: The IP address and port of the remote server to send the data to.

**Note:** The `config::ENCRYPTION_KEY` is used in `src/main.rs` (startLine: 23) and `src/sender.rs` (startLine: 17). The `config::REMOTE_IP` is used in `src/sender.rs` (startLine: 18).

## Dependencies

The project uses the following crates:

*   `anyhow`: For error handling.
*   `reqwest`: For making HTTP requests (not currently used, but included in `Cargo.toml` (startLine: 8)).
*   `serde` and `serde_json`: For serialization and deserialization (not currently used, but included in `Cargo.toml` (startLine: 9, 10)).
*   `rand`: For generating random numbers (not currently used, but included in `Cargo.toml` (startLine: 11)).
*   `base64`: For base64 encoding (not currently used, but included in `Cargo.toml` (startLine: 12)).
*   `zip`: For creating ZIP archives.
*   `futures`: For asynchronous programming (not currently used, but included in `Cargo.toml` (startLine: 14)).
*   `uuid`: For generating UUIDs for temporary directories.

## Disclaimers and Ethical Considerations

*   **This tool is for educational and ethical research purposes only.**
*   **Do not use this tool for any illegal or malicious activities.**
*   **The developers are not responsible for any misuse of this software.**
*   **Always obtain explicit consent before running this tool on any system.**
*   **The encryption used is basic and should not be relied upon for securing sensitive data.**
*   **This tool is a proof-of-concept and may contain bugs or vulnerabilities.**
*   **Use this tool responsibly and ethically.**
*   **Be aware of the legal and ethical implications of collecting and transmitting data.**
*   **This tool is provided "as is" without any warranty.**

## Future Improvements

*   Implement more robust encryption methods.
*   Add support for more browsers and wallets.
*   Improve error handling and logging.
*   Add more sophisticated anti-VM and anti-debugger techniques.
*   Implement a more flexible configuration system.
*   Add support for different data transmission protocols.
*   Add more comprehensive testing.
*   Refactor code for better maintainability and readability.
*   Add a proper command-line interface.
*   Implement asynchronous operations for better performance.
*   **Implement DNS Tunneling:**
    *   Modify the `sender.rs` module to include DNS tunneling capabilities.
    *   Use a DNS library like `trust-dns` to craft and send DNS queries.
    *   Encode data into DNS query names and send them to a controlled DNS server.
    *   Set up a DNS server to receive and decode these queries.
*   **Data Fragmentation and Timing:**
    *   Create a scheduler in `sender.rs` to send data fragments at random intervals.
    *   Implement a data fragmentation function to break data into smaller pieces.
    *   Use both DNS and HTTP(S) requests to send data, alternating between them to avoid detection.
*   **Obfuscation:**
    *   Integrate the `obfstr` crate to obfuscate string literals throughout the codebase.
    *   Implement control flow obfuscation in critical functions, especially in `main.rs` and `tools.rs`.
*   **Environment Checks:**
    *   Add VM and sandbox detection in `antivm.rs` and `system_info.rs`.
    *   Check for known VM artifacts like MAC addresses and registry keys.
    *   Use system calls to detect sandboxing tools and terminate if detected.
*   **Debugger Detection:**
    *   Enhance the `is_debugger_attached` function in `antivm.rs` to include more robust checks.
    *   Implement timing checks to detect breakpoints by measuring execution time deviations.
*   **Self-Modification:**
    *   Use Rust's `mmap` to modify executable code sections at runtime.
    *   Implement polymorphic code in `main.rs` that changes its structure during execution.
*   **Code Encryption:**
    *   Encrypt sensitive code sections using symmetric encryption.
    *   Decrypt code at runtime only when needed, using a secure key management strategy.
*   **Behavioral Analysis Evasion:**
    *   Implement checks for common analysis tools in `system_info.rs`.
    *   Use misleading behavior to confuse analysis tools, such as fake error messages or delays.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for any bugs or feature requests.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
