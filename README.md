<div align="center">
  <img src="assets/dotkit_logo_no_bg.png" alt="dotkit logo" height="150px"/>
  <h1>DotKit</h1>
  <h2>A Scaffolding Tool for Ink Smart Contracts</h2>
</div>

**DotKit** is a powerful scaffolding tool designed to help you quickly generate basic templates for **ink! smart contracts**. Whether you're building a Counter, NFT, Token, or Multisig contract, DotKit provides a streamlined way to kickstart your project, allowing you to focus on the specifics of your use case.

With DotKit, you can:
- Select a contract type based on your needs.
- Customize basic details such as project name, author name, and email.
- Generate a ready-to-use ink! contract with minimal effort.

## Demo

https://github.com/user-attachments/assets/892bf249-e5f2-4087-b4d1-e9a9a3f81f16

## Features
- **Interactive CLI**: DotKit guides you through a simple, interactive process to gather required details.
- **Supported Contract Types**:
  - **Counter Contract**: A basic increment/decrement counter.
  - **NFT Contract**: A template for non-fungible token standards.
  - **Token Contract**: A starting point for creating fungible tokens.
  - **Multisig Contract**: A contract for managing multisignature wallets.
- **Sensible Defaults**: Defaults for inputs like project name and author details ensure you can get started even faster.
- **Validated Inputs**: Input validation ensures your data is accurate before generating the contract.

## Installation

1. Clone the repository:
   ```bash
   cargo install dotkit
   ```

2. Build the project:
   ```bash
   cargo contract build --release
   ```

## Usage

1. Run DotKit:
   ```bash
   dotkit
   ```

2. Follow the interactive prompts to:
   - Enter your project name.
   - Enter your name and email.
   - Select a contract type.

3. Once complete, DotKit will generate a new folder with the scaffolded contract files.

## Example

Here’s an example of using DotKit to generate a Counter contract:

1. Run the tool:
   ```bash
   dotkit
   ```
2. Input the details when prompted:
   ```text
   Enter your project name: counter_project
   Enter your name: Alice
   Enter your email: alice@example.com
   Pick a project type:
   - [c] Counter Contract
   - [t] Token Contract
   - [n] NFT Contract
   - [m] Multisig Contract
   Select an option: c
   ```

3. DotKit will generate the contract and display:
   ```text
   Let's cook!🚀

   1. Open `lib.rs`
   2. Make some required changes
   3. Run `cargo contract build` to build the contract"
   ```

4. Navigate to the generated folder:
   ```bash
   cd counter_project
   ```
5. Open `llib.rs` file and adjust the generated contract file as needed.

## Project Structure

The scaffolded project will have the following structure:
```
project_name/
├── lib.rs          # Main ink! contract file
├── Cargo.toml      # Rust project file
└── README.md       # Project README file
```

## Roadmap
- [X] MVP with basic contract types
- [ ] Add support for additional contract types.
- [ ] Audit contracts
- [ ] Provide more customization options for generated contracts.
- [ ] Integrate with popular blockchain development tools.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests. See the [CONTRIBUTING.md](CONTRIBUTING.md) file for more details.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
Special thanks to the ink! development community for their fantastic tools and support.

