import React, { useState } from "react";

// Main TodoApp component
const TodoApp = () => {
  // State for managing todos
  const [todos, setTodos] = useState([
    { id: 1, text: "Learn React", completed: true },
    { id: 2, text: "Build a todo app", completed: false },
    { id: 3, text: "Deploy to production", completed: false },
  ]);

  // State for input field
  const [inputValue, setInputValue] = useState("");

  // Add new todo
  const handleAddTodo = () => {
    if (inputValue.trim() !== "") {
      const newTodo = {
        id: Date.now(),
        text: inputValue,
        completed: false,
      };
      setTodos([...todos, newTodo]);
      setInputValue("");
    }
  };

  // Toggle todo completion status
  const handleToggle = (id) => {
    setTodos(
      todos.map((todo) =>
        todo.id === id ? { ...todo, completed: !todo.completed } : todo,
      ),
    );
  };

  // Delete a todo
  const handleDelete = (id) => {
    setTodos(todos.filter((todo) => todo.id !== id));
  };

  // Get counts for summary
  const completedCount = todos.filter((todo) => todo.completed).length;
  const totalCount = todos.length;

  return (
    <div className="todo-app">
      <h1>Todo List</h1>

      {/* Input form */}
      <div className="todo-form">
        <input
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          placeholder="Add a new task"
        />
        <button onClick={handleAddTodo}>Add</button>
      </div>

      {/* Todo list */}
      <div className="todo-list">
        {todos.length === 0 ? (
          <div className="empty-state">No tasks yet! Add one above.</div>
        ) : (
          todos.map((todo) => (
            <TodoItem
              key={todo.id}
              todo={todo}
              onToggle={handleToggle}
              onDelete={handleDelete}
            />
          ))
        )}
      </div>

      {/* Summary footer */}
      <div className="todo-summary">
        <p>
          {completedCount} completed out of {totalCount} tasks (
          {Math.round((completedCount / totalCount) * 100) || 0}%)
        </p>
      </div>
    </div>
  );
};

// Individual todo item component
const TodoItem = ({ todo, onToggle, onDelete }) => {
  return (
    <div className={`todo-item ${todo.completed ? "completed" : ""}`}>
      <input
        type="checkbox"
        checked={todo.completed}
        onChange={() => onToggle(todo.id)}
      />
      <span className="todo-text">{todo.text}</span>
      <button className="delete-btn" onClick={() => onDelete(todo.id)}>
        Delete
      </button>
    </div>
  );
};

export default TodoApp;
