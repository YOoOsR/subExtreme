# subExtreme

<p align="center">
    <img src="https://github.com/user-attachments/assets/89b783e8-231b-4ee1-994b-4ee241e0150f" alt="subExtreme Screenshot" width="80%"/>
</p>

`subExtreme` is a subdomain discovery tool written in Rust, designed to perform brute-force attacks to discover subdomains.

---

## Features

subExtreme offers a variety of useful features:

- **Wordlist Support**: Specify a wordlist file for subdomain enumeration.
- **Variable Number of Parallel Requests**: Adjust the number of parallel requests for optimal performance.
- **Output to File**: Save discovered subdomains to a file.
- **Exception Handling**: Handles connection errors and server issues gracefully.
- **Easy Command-Line Interface**: User-friendly CLI for smooth interaction.

---

## Installation on Linux

Follow these steps to install `subExtreme` on your Linux system:

### Step 1: Install Rust

If you don't have Rust installed, you can install it using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, reload the environment:

```bash
source $HOME/.cargo/env
```

---

### Step 2: Clone the Repository

Clone the repository to your local machine:

```bash
git clone https://github.com/ahmedhamdy0x/subextreme.git
cd subextreme
```

---

### Step 3: Build the Tool

Use `cargo` to build the tool:

```bash
sudo apt install pkg-config -y
sudo apt install libssl-dev -y
cargo build --release
```

---

### Step 4: Install Dependencies

Ensure that all dependencies are correctly installed by running:

```bash
cargo build
```

---

### Step 5: Add subExtreme to Your System PATH

Move it to `/usr/local/bin/` to make the tool globally accessible:

```bash
sudo cp target/release/subextreme /usr/local/bin/
```

---

### Step 6: Make subExtreme Executable

Ensure that the tool is executable:

```bash
sudo chmod +x /usr/local/bin/subextreme
```

---

## How to Use

To start using `subExtreme`, follow the steps below:

### 1. Specify the Wordlist

Use the `-w` flag to specify the wordlist file for subdomain enumeration.

### 2. Define the Target Domain

Use the `-d` flag to define the target domain (e.g., `example.com`).

### 3. Specify the Output File

Use the `-o` flag to define the file path where discovered subdomains will be saved.

### 4. Set the Number of Parallel Requests

Use the `-c` flag to set the number of parallel requests (default is 20).

---

### Usage Example

#### Subdomain Enumeration with a Custom Wordlist & Parallel Requests & Output File

```bash
subextreme -w /path/to/wordlist.txt -d example.com -c 100 -o output.txt
```

---

## Contact

Developed by [Ahmed Hamdy](https://github.com/ahmedhamdy0x)

YouTube Channel: [Gentil Security](https://www.youtube.com/@gentil.security)

For inquiries or support, feel free to contact me at: **info.gentil.academy@gmail.com**

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for more details.
