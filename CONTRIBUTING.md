# Contributing to DotKit

Thank you for considering contributing to **DotKit**! We welcome contributions from the community to help improve the tool, expand features, and fix any issues.

Please take a moment to review the guidelines below to ensure your contribution aligns with the project's goals and standards.

## Code of Conduct

We are committed to creating a welcoming and inclusive environment for all contributors. By participating in this project, you agree to adhere to our [Code of Conduct](https://github.com/nocoffeecraft/dotkit/blob/main/CODE_OF_CONDUCT.md).

## How to Contribute

We love contributions, whether it's a bug fix, a feature enhancement, or a documentation improvement. Here’s how you can get involved:

### Reporting Bugs

If you encounter a bug, we’d love to hear about it. Please provide as much detail as possible:
1. **Check for existing reports**: Before creating a new issue, please search through the [existing GitHub Issues](https://github.com/nocoffeecraft/dotkit/issues) to see if the bug has already been reported.
2. **Create a new issue**: If your bug hasn't been reported yet, open a new issue with the following information:
   - A clear and descriptive title.
   - Steps to reproduce the bug.
   - Expected and actual behaviour.
   - Your environment (OS, version of DotKit, etc.).
   - Any relevant logs or screenshots.

### Suggesting Enhancements

We’re open to feature suggestions and ideas to improve DotKit. If you have an idea:
1. **Search for similar suggestions**: Check the [open issues](https://github.com/nocoffeecraft/dotkit/issues) to see if someone else has already suggested it.
2. **Create a new issue**: If it’s a new idea, open an issue describing the enhancement, use cases, and why it would be valuable to DotKit.
3. **Discuss your suggestion**: We encourage discussion on proposed features to make sure they fit with the tool’s vision.

### Submitting Pull Requests

We accept pull requests (PRs) for bug fixes, features, and improvements. Here’s how to submit a PR:

1. **Fork the repository**: Click the "Fork" button in the top-right corner of the repository to create your own copy.
   
2. **Clone your fork**: Clone the repository to your local machine:
   ```bash
   git clone https://github.com/YOUR_USERNAME/dotkit.git
   cd dotkit
   ```

3. **Create a new branch**: Create a new branch for your changes:
   ```bash
   git checkout -b my-feature-branch
   ```

4. **Make your changes**: Implement the changes, bug fixes, or features in your branch.

5. **Write tests (if applicable)**: If your changes include new features or bug fixes, please write tests to verify your changes. For Rust projects, make sure to write unit tests in the appropriate files.

6. **Commit your changes**: After you’ve made your changes, commit them with a meaningful commit message:
   ```bash
   git commit -m "Brief description of changes"
   ```

7. **Push your changes**: Push your changes to your fork:
   ```bash
   git push origin my-feature-branch
   ```

8. **Create a pull request**: Open a pull request against the `main` branch of the original repository. Provide a detailed description of your changes, including any context and why the change is necessary.

### PR Review and Feedback

Once you submit a PR, the project maintainers will review it. We might suggest some changes or ask for clarification. Please be patient during this process and feel free to engage in the review to ensure the PR is merged.

## Development Setup

To contribute effectively to **DotKit**, you'll need to set up a local development environment. Here are the steps to get started:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/nocoffeecraft/dotkit.git
   cd dotkit
   ```

2. **Install Rust**: Ensure that you have the Rust toolchain installed. If not, install it by following the instructions on [rust-lang.org](https://www.rust-lang.org/).

3. **Build the project**: After cloning the repo, run the following command to build the project:
   ```bash
   cargo build
   ```

4. **Run the project**: To test DotKit locally, you can use:
   ```bash
   cargo run
   ```

5. **Install dependencies**: If your changes require new dependencies, add them to `Cargo.toml` and run:
   ```bash
   cargo update
   ```

## Testing

Testing is crucial to ensure DotKit works as expected. Please write unit tests for new features or bug fixes.

1. **Run tests**: To run the existing tests, execute:
   ```bash
   cargo test
   ```
2. **Add new tests**: If you add a new feature or bug fix, make sure to write tests in the appropriate module and verify they pass.

## Style Guide

Please adhere to the following guidelines when contributing to DotKit:

1. **Rust Code Style**: Follow the [Rust style guide](https://doc.rust-lang.org/1.0.0/style/). You can format your code using `cargo fmt` to ensure it follows the project's style.
   
2. **Commit Messages**: Write clear, concise commit messages in the imperative mood (e.g., "Fix bug with token balance" instead of "Fixed bug with token balance").

3. **Documentation**: Ensure that your code is well-documented. Add doc comments for public functions and modules using the Rust documentation syntax.

4. **Avoid Breaking Changes**: When making changes, try to ensure backward compatibility, especially for users who may already be relying on the current functionality.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project. See the [LICENSE](LICENSE) file for more details.
