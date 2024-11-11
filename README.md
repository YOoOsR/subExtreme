# subExtreme
<p align="center">
    <img src="https://github.com/user-attachments/assets/89b783e8-231b-4ee1-994b-4ee241e0150f" alt="subExtreme Screenshot"/>
</p>

`subExtreme` is a subdomain discovery tool written in Rust, designed to perform brute-force attacks to discover subdomains.



---

## Features

- **Wordlist Support**: You can specify a wordlist file for subdomain enumeration.
- **Variable Number of Parallel Requests**: Define the number of parallel requests for optimal performance.
- **Output to File**: Save the discovered subdomains to a file.
- **Exception Handling**: The tool handles errors related to connections and servers.
- **Easy Command-Line Interface**: User-friendly CLI for smooth interaction.



---

## Installation on Linux

### Step 1: Install Rust

If you don't have Rust installed, you can install it using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



After installation, reload the environment by running:

```bash
source $HOME/.cargo/env
```



### Step 2: Clone the Repository

Clone the repository to your local machine:

```bash
git clone https://github.com/ahmedhamdy0x/subextreme.git
cd subExtreme
```



### Step 3: Build the Tool

Use `cargo` to build the tool:

```bash
cargo build --release
```



### Step 4: Install Dependencies

Ensure that all dependencies are correctly installed by running:

```bash
cargo build
```



### Step 5. Add subExtreme to Your System PATH

To make the tool globally accessible from any directory, move it to `/usr/local/bin/`:

```bash
sudo cp subExtreme/target/release/subextreme /usr/local/bin/
```



### Step 6. Make subExtreme Executable

Ensure that the tool is executable by running the following command:

```bash
sudo chmod +x /usr/local/bin/subextreme
```




---

## How to Use

### 1. Specify the Wordlist

- `-w`: Use this flag to specify the wordlist file for subdomain enumeration.

### 2. Define the Target Domain

- `-d`: Use this flag to define the target domain (e.g., `example.com`).

### 3. Specify the Output File

- `-o`: Define the file path to save the discovered subdomains.

### 4. Set the Number of Parallel Requests

- `-c`: Set the number of parallel requests (default is 20).




### Usage Example

#### Subdomain Enumeration with a Custom Wordlist & Parallel Requests & Output file

```bash
subextreme -w /path/to/wordlist.txt -d example.com -c 100 -o output.txt
```

---


### Contact

Developed by [Ahmed Hamdy](https://github.com/ahmedhamdy0x)

Youtube Channel [Gentil Security](https://www.youtube.com/@gentil.security)

For inquiries or support, feel free to contact me at: **info.gentil.academy@gmail.com**
