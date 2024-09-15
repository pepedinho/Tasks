# Task ✅

**Task** is a command-line task manager developed in Rust. It allows you to manage your tasks and folders in a simple and persistent manner.

## 📋 Features

- **Task Management**: Add, remove, and organize your tasks directly from the command line.
- **Folder Management**: Create folders to organize your tasks into subcategories.
- **Data Persistence**: Tasks are saved in a `.tsk` file to be preserved between sessions.
- **Simple Input**: Create tasks and folders with easy command-line input.

## 📂 How to Use

1. **Setup**: Create a `.tsk` file in the directory where you run the program. This will allow Task to store and load your tasks.

2. **Create a Task Folder**:
   - Press `a` to start input.
   - Type the folder name followed by a `/`, for example: `my_folder/`.

3. **Create a Task**:
   - Press `a` to start input.
   - Type the task name, for example: `my_task`.

4. **Create a Folder with a Task**:
   - Press `a` to start input.
   - Type the folder name followed by the task name, for example: `my_folder/my_first_task`.

## 🚀 Coming Soon

- **Automatic `.tsk` File Creation**: Run Task with an argument to automatically create a `.tsk` file.
- **More Features**: Additional features will be added to enhance task management.

## 📥 Installation

1. **Ensure Rust is Installed**: You can install Rust from [rust-lang.org](https://www.rust-lang.org/).
2. **Clone the Project**:
   ```sh
   git clone https://github.com/pepedihno/task.git
   ```
3. **Navigate to the Project Directory:**:
   ```s
   cd task
   ```
4. **Build the Project**:
   ```sh
    cargo build --release
   ```
5. **Run the Program**:
   ```sh 
   ./target/release/task
   ```
