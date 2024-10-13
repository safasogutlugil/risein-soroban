# PayBan

![PayBan](payban.png)

## About Me

My name is Safa Sogutlugil. I graduated as an M.D. from Marmara University in 2020 and with a CS degree from Sabanci University in 2024. My journey into the world of cryptocurrency began in 2017, and I recently developed a keen interest in Web3 in 2024, participating in the Rise In - Stellar Fullstack Bootcamp. This marks my first coding experience in Soroban and Web3. Outside of my academic pursuits, I enjoy reading and exploring psychology, always seeking to understand the intricacies of the human mind.

## Description

PayBan is a simple payment and messaging system designed for the Stellar Blockchain, allowing users to send payments using Stellar's native token, XLM. With the ability to transfer XLM to multiple recipients in a single transaction, users can streamline group payments while attaching short messages to their transfers for added context. The platform also features a transaction history view, enabling users to track past transfers and review received messages easily. The project focuses on developing a robust backend contract and thorough testing to ensure a secure and reliable experience for all users, making payments and communication seamless and efficient.


## Vision

At PayBan, we envision a world where payments are as simple and fast as sending a message. By harnessing the power of the Stellar Blockchain, we aim to create a user-friendly platform that facilitates seamless transactions and enhances communication between individuals. Our goal is to empower users with efficient, secure, and transparent payment solutions, bridging the gap between traditional finance and the decentralized future. By enabling multi-recipient transfers and instant messaging, PayBan seeks to transform how people transact, fostering greater connectivity and collaboration in a digital economy. Together, we can redefine the way we handle payments.

## Project Ideas

1. **Define Smart Contract Structure:**
   - Identify key variables, including user balances, transaction history, and message storage.
   - Develop functions for sending payments, including multi-recipient transfers and message attachment.

2. **Smart Contract Development:**
   - Implement the payment function to handle single and multi-recipient XLM transfers.
   - Create a function to store messages alongside transactions for easy retrieval.
   - Develop a function to retrieve transaction history for users.

3. **Testing and Validation:**
   - Write unit tests for all smart contract functions to ensure correctness and security.
   - Conduct integration tests to confirm that the payment and messaging features work seamlessly together.

4. **Front-End Development:**
   - Design a user-friendly interface for sending payments and viewing transaction history.
   - Implement features for inputting recipient addresses, amounts, and messages.
   - Integrate the front-end with the smart contract to enable real-time transactions.

5. **User Feedback and Iteration:**
   - Conduct user testing sessions to gather feedback on usability and functionality.
   - Make necessary adjustments based on user input to enhance the overall experience.

6. **Deployment:**
   - Deploy the smart contract on the Stellar Blockchain.
   - Launch the front-end application, making it accessible to users for payment and messaging.

## Tech Stack

- Rust & Web3

## Setup

### Prerequisites

Before you begin, ensure you have the following installed:

- Node.js (version 14 or higher)
- npm (Node package manager)
- Stellar SDK for JavaScript
- Soroban CLI

### Installation Steps

#### 1. Clone the Repository:
Open your terminal and run:
```bash
git clone https://github.com/safasogutlugil/risein-soroban.git
cd risein-soroban
```

#### 2. Install Dependencies:
Inside the project directory run:
```bash
npm install
```

#### 3. Test Existing or New Functionality:
Run all tests:
```bash
cargo test
```
Run a specific test:
```bash
cargo test enter_test_name_to_run
```

#### 4. Compile the Smart Contract:
Use the Soroban CLI to compile the smart contract:
```bash
soroban contract build
```

#### 5. Deploy the Smart Contract:
Deploy the contract to the Stellar Blockchain:
```bash
soroban contract deploy --network testnet
```





## Contributing

If you'd like to contribute, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License.