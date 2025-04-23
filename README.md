# rust-practice

So far I have covered:

- Basic syntax and concepts (variables, functions, control flow)
- Ownership, borrowing, and references
- Structs and methods
- Enums and pattern matching
---

### 🛠️ Rust Projects for Chapters 1–6

#### 1. **Guessing Game Enhancer (Upgrade from Chapter 2)**
   - Add a difficulty level using an enum.
   - Keep score across multiple rounds.
   - Use structs to store player info (name, score, difficulty).

#### 2. **Simple Contact Manager**
   - Define a `Contact` struct with fields like name, phone, and email.
   - Use a vector to store multiple contacts.
   - Allow adding, removing, listing, and searching contacts with match patterns on user input.

#### 3. **Bank Account Simulator**
   - Use an enum for account actions: `Deposit`, `Withdraw`, `Balance`.
   - Define a `BankAccount` struct with balance and owner.
   - Use pattern matching to handle transactions.

#### 4. **Basic Quiz App**
   - Store questions and answers using a `Question` struct.
   - Store answer status using an enum: `Correct`, `Incorrect`, `Unanswered`.
   - Match user input to expected answers and keep score.

#### 5. **Shape Area Calculator (Struct + Enum Combined)**
   - Define an enum for shape types and a struct for each shape's properties.
   - Use pattern matching to compute area for each shape.

#### 6. **To-Do List App**
   - Use an enum to define task status: `Pending`, `InProgress`, `Completed`.
   - Use a struct for a `Task` with title, description, and status.
   - Pattern match on status for actions like listing tasks by state or updating status.

#### 7. **Simple Command-Line RPG Battle System**
   - Define a `Character` struct with HP, name, and attack value.
   - Use an enum for actions: `Attack`, `Defend`, `Heal`.
   - Implement a simple turn-based system using pattern matching.

---

as for chapter 6 alone i can have this 
---

### 🔧 **Mini Projects for Rust Chapter 6 (Enums & Pattern Matching)**

1. **Simple ATM Simulation**
   - Define an enum for different types of transactions (`Deposit`, `Withdraw`, `CheckBalance`).
   - Use pattern matching to simulate the transaction behavior and update balance accordingly.

2. **Basic Command Line Todo App**
   - Use an enum to represent different commands (`Add`, `Remove`, `List`, `Complete`).
   - Use pattern matching to match on commands and act accordingly.

3. **Dice Game**
   - Create an enum for different game outcomes (`Win`, `Lose`, `Draw`).
   - Roll two virtual dice and determine the outcome based on pattern matching.

4. **Basic State Machine**
   - Model a traffic light or a vending machine using an enum for states.
   - Write a function that uses pattern matching to transition between states based on input.

5. **Simple HTTP Status Code Parser**
   - Create an enum for a few common HTTP statuses (`Ok`, `NotFound`, `InternalServerError`, etc.).
   - Write a function that takes a number (e.g., `200`, `404`) and returns the appropriate enum variant.

6. **Shape Area Calculator**
   - Use an enum to define different shapes (`Circle`, `Rectangle`, `Square`).
   - Use pattern matching to calculate area for each shape.

---

