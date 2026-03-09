# Task Forge — Tasks Page Functional Specification

## Overview

The Tasks page is the primary view for managing tasks. It displays all tasks in a table format with the ability to mark tasks as complete, delete tasks, filter by status or subject, and add new tasks via a popup modal.

---

## Navigation Bar Update

The **Add Task** link is removed from the navbar. The navbar now contains:

| Nav Item  | Destination        |
|----------|--------------------|
| **Home**  | Home page          |
| **Tasks** | Tasks page (current) |

Adding tasks is handled exclusively through a popup modal accessible from the Tasks page.

---

## Page Layout

### Add Task Button

- A persistent **"Add Task"** button is always visible on the Tasks page regardless of the current filter state.
- Positioned at the top-right of the page, above the table.
- Clicking it opens the Add Task popup modal.
- Styled with the orange accent color to stand out.

### Filter Sidebar

Filters are displayed in a **sidebar** on the left side of the page. The sidebar is a sticky card that stays visible while scrolling.

#### Sidebar Styling
- Fixed width (~200px), positioned to the left of the main content area.
- Background uses `var(--card-bg)` with a border and rounded corners.
- A "Filters" title at the top with a bottom border separator.
- Each filter has an uppercase label above its dropdown.
- The sidebar is sticky (stays in view when scrolling the task list).

#### Filter Options:

| Filter         | Type     | Options                                      |
|---------------|----------|----------------------------------------------|
| **Status**     | Dropdown or toggle | All (default) / Incomplete / Completed |
| **Subject**    | Dropdown | No Grouping (default) / All Subjects / Individual subject names |

#### Filter Behavior — Status
- **All** — Shows all tasks in the default layout (described below).
- **Incomplete** — Shows only tasks where completed is false.
- **Completed** — Shows only tasks where completed is true.

#### Filter Behavior — Subject
- **No Grouping** (default) — Standard view split by due date (no subject grouping).
- **All Subjects** — The table splits into **sub-tables grouped by subject**. Each sub-table has a **subject heading** displayed above it. **Within each subject group, the same two-section layout is maintained**: tasks without due dates (top, collapsed to 3 with Show More) and tasks with due dates (bottom, sorted ascending). Tasks without a subject are grouped under an "Uncategorized" heading.
- **Individual subject** — When a specific subject is selected, **only tasks belonging to that subject** are displayed. No other subjects are shown. **The two-section due date layout is still maintained**: tasks without due dates on top (collapsed to 3 with Show More) and tasks with due dates below (sorted ascending).

#### Filter Dropdown Styling
- Dropdown menus must use theme-aware colors: `var(--card-bg)` for background and `var(--text-primary)` for text.
- The dropdown options list (native `<option>` elements) must also inherit the correct background and text colors so they remain readable in both dark and light mode.
- Use `color-scheme: dark` / `color-scheme: light` on the root element to ensure native form controls (dropdowns, date pickers, checkboxes) respect the current theme.
- Nothing should appear pure white in dark mode except text.

---

## Default Table Layout (No Filters)

The page displays two sections:

### 1. Tasks Without Due Dates (Top Section)

- Displays tasks that have **no due date assigned**.
- Shows only the **first 3 tasks** by default.
- A **"Show More"** button appears at the bottom of this section if there are more than 3 tasks.
- Clicking "Show More" expands to reveal all tasks without due dates.
- Once expanded, the button changes to **"Show Less"** to collapse back to 3.

### 2. Tasks With Due Dates (Bottom Section)

- Displays all tasks that **have a due date**.
- Sorted by due date in **ascending order** (soonest first).
- No collapsing — all tasks are visible.

---

## Table Structure

Each table has the following columns:

| Column       | Description                                      | Width    |
|-------------|--------------------------------------------------|----------|
| **Checkbox** | Toggles task completion status                   | Narrow   |
| **Name**     | The task name                                    | Flexible |
| **Subject**  | The subject/category (or empty if none)          | Medium   |
| **Due Date** | The due date in readable format (or empty)       | Medium   |
| **Delete**   | A delete button/icon to remove the task          | Narrow   |

### Table Styling

- **Alternating row colors** — Subtle difference (e.g., dark grey and slightly lighter dark grey in dark mode; white and light grey in light mode).
- **Thin borders** between rows to accentuate the alternating colors.
- Table header row is slightly bolder/darker than data rows.
- Rounded corners on the overall table container.
- Consistent with the app's modern/sleek design.

---

## Interactions

### Checkbox (Mark Complete)

- Clicking the checkbox toggles the task's completed status.
- When a task is marked as complete, the **entire row becomes slightly greyed out** (reduced opacity or muted colors).
- The checkbox shows as checked/filled.
- Clicking again unchecks and restores the normal row appearance.

### Delete Button

- Each row has a delete button/icon (e.g., trash icon or "X").
- Clicking the delete button opens a **confirmation modal**:
  - Message: "Are you sure you want to delete this task?"
  - Displays the task name for clarity.
  - Two buttons: **"Delete"** (destructive, red-styled) and **"Cancel"**.
- If confirmed, the task is removed from the list and the JSON file.
- If cancelled, the modal closes with no changes.

### Show More / Show Less

- Only appears in the "Tasks Without Due Dates" section.
- Toggles between showing 5 tasks and showing all tasks.
- Smooth expand/collapse transition.

---

## Add Task Popup Modal

### Trigger
- Clicking the **"Add Task"** button on the Tasks page.
- Available at all times on the Tasks page regardless of filters.

### Modal Layout

A centered modal overlay with a semi-transparent backdrop. Contains a form with the following fields:

| Field        | Type       | Required | Description                              |
|-------------|------------|----------|------------------------------------------|
| **Name**     | Text input | Yes      | The name of the task                     |
| **Subject**  | Text input | No       | The subject or category of the task      |
| **Due Date** | Date picker| No       | The due date in YYYY-MM-DD format        |

### Date Picker Behavior
- The date picker popup should **close automatically** when a date is selected (double-click or value change).
- Pressing **Enter** while the date picker is focused should also close the picker.
- Implemented by blurring the input on value change and on Enter keypress.

### Modal Actions

| Button     | Behavior                                                    |
|-----------|-------------------------------------------------------------|
| **Add**    | Validates the form, creates the task, saves to JSON, closes the modal, and refreshes the table. |
| **Cancel** | Closes the modal with no changes.                           |

### Validation
- The **Name** field is required. If empty on submit, show an inline error message.
- Subject and Due Date are optional.

### Modal Styling
- Smooth fade-in animation on open.
- Semi-transparent dark backdrop behind the modal.
- Modal card styled consistently with the app theme (card background, rounded corners, border).
- Close on backdrop click or Cancel button.

---

## Empty State

When there are no tasks at all:

- The table is replaced with a centered message: **"No tasks yet"**.
- Below the message, an **"Add Task"** button is displayed.
- Clicking this button opens the same Add Task popup modal.
- Styled cleanly with muted text and the accent-colored button.

---

## Delete Confirmation Modal

A centered modal overlay, similar in style to the Add Task modal:

- **Title**: "Delete Task"
- **Message**: "Are you sure you want to delete '[task name]'?"
- **Buttons**:
  - **"Delete"** — Red/destructive styling. Deletes the task and closes the modal.
  - **"Cancel"** — Neutral styling. Closes the modal with no changes.
- Smooth fade-in/out animation.
- Closes on backdrop click or Cancel button.

---

## Theme Styling

### Color Scheme (extends global theme)

| Element                  | Light Mode                  | Dark Mode                    |
|-------------------------|-----------------------------|------------------------------|
| Table row (even)         | White (#ffffff)             | Dark grey (#2a2a2a)          |
| Table row (odd)          | Light grey (#f0f0f0)       | Slightly lighter grey (#303030) |
| Table row border         | Subtle grey (#e0e0e0)      | Subtle grey (#3a3a3a)        |
| Completed row            | Greyed out (opacity ~0.5)  | Greyed out (opacity ~0.5)    |
| Table header             | Slightly darker background  | Slightly darker background   |
| Delete button            | Red accent on hover         | Red accent on hover          |
| Add Task button          | Orange accent               | Orange accent                |
| Modal backdrop           | rgba(0, 0, 0, 0.5)        | rgba(0, 0, 0, 0.7)          |

---

## Wireframe (Text-Based)

```
+----------------------------------------------------------------------+
| [Task Forge]              Home    Tasks                   [sun/moon]  |
+----------------------------------------------------------------------+
|            |                                                          |
| FILTERS    |  Tasks                                   [ + Add Task ]  |
| ---------- |                                                          |
|            |  Tasks Without Due Dates                                 |
| Status     |  +------+---------------+-----------+--------+--------+  |
| [All    v] |  |  [ ] | Name          | Subject   | Due    | Delete |  |
|            |  +------+---------------+-----------+--------+--------+  |
| Subject    |  |  [ ] | Buy groceries | Personal  |   —    |  [x]   |  |
| [None   v] |  |  [x] | Read ch. 5    | School    |   —    |  [x]   |  |
|            |  |  [ ] | Call dentist  |    —       |   —    |  [x]   |  |
|            |  +------+---------------+-----------+--------+--------+  |
|            |                    [ Show More ]                         |
|            |                                                          |
|            |  Tasks With Due Dates                                    |
|            |  +------+---------------+-----------+--------+--------+  |
|            |  |  [ ] | Name          | Subject   | Due    | Delete |  |
|            |  +------+---------------+-----------+--------+--------+  |
|            |  |  [ ] | Submit report | Work      | 03-15  |  [x]   |  |
|            |  |  [ ] | Final exam    | School    | 04-01  |  [x]   |  |
|            |  +------+---------------+-----------+--------+--------+  |
|            |                                                          |
+----------------------------------------------------------------------+
```

---

## Backend Connection

The Tasks page requires communication between the React frontend and the Rust backend via **Tauri commands** (using `@tauri-apps/api/core`'s `invoke` function). The existing Rust modules (`Task`, `TaskMap`) are reused as the data layer.

### Existing Rust Data Structures

The backend already has these structures in place:

- **`Task`** (`src/tasks/task.rs`) — Struct with fields: `name: String`, `subject: Option<String>`, `due_date: Option<NaiveDate>`, `completed: bool`. Has methods: `new()`, getters, and `mark_completed()` (toggles).
- **`TaskMap`** (`src/tasks/task_map.rs`) — Wraps a `HashMap<String, Task>`. Has methods: `add_task()`, `get_task()`, `load(filename)`, `save(filename)`. Persistence is via `db/task.json`.

### Tauri Commands to Implement

Define these as `#[tauri::command]` functions in the Tauri backend (`src-tauri/src/main.rs` or a dedicated commands module):

#### `get_tasks` — Load all tasks

```rust
#[tauri::command]
fn get_tasks() -> Result<Vec<TaskResponse>, String>
```

- Loads `TaskMap` from `db/task.json`.
- Returns all tasks as a JSON-serializable vector.
- Frontend calls this on page mount and after any mutation.

**Response format:**
```json
[
  {
    "name": "Submit report",
    "subject": "Work",
    "due_date": "2026-03-15",
    "completed": false
  }
]
```

#### `add_task` — Create a new task

```rust
#[tauri::command]
fn add_task(name: String, subject: Option<String>, due_date: Option<String>) -> Result<(), String>
```

- Creates a new `Task` and adds it to the `TaskMap`.
- Saves the updated map to `db/task.json`.
- Returns an error if a task with the same name already exists (or overwrites — match existing behavior).
- Frontend calls this from the Add Task modal on form submit.

#### `toggle_task` — Toggle completion status

```rust
#[tauri::command]
fn toggle_task(name: String) -> Result<(), String>
```

- Loads the `TaskMap`, finds the task by name, calls `mark_completed()`.
- Saves the updated map to `db/task.json`.
- Frontend calls this when a checkbox is clicked.

#### `delete_task` — Remove a task

```rust
#[tauri::command]
fn delete_task(name: String) -> Result<(), String>
```

- Loads the `TaskMap`, removes the task by name from the `HashMap`.
- Saves the updated map to `db/task.json`.
- Frontend calls this after the delete confirmation modal is confirmed.
- Note: A `remove_task()` method will need to be added to `TaskMap`.

### Required Backend Changes

The following changes to the existing Rust code are needed to support the GUI:

1. **Add `remove_task()` to `TaskMap`:**
   ```rust
   pub fn remove_task(&mut self, name: &str) -> bool {
       self.map.remove(name).is_some()
   }
   ```

2. **Make `Task` fields serializable for Tauri responses:**
   - `Task` already derives `Serialize` and `Deserialize`, so it can be returned directly from Tauri commands.
   - Ensure `NaiveDate` serializes as a string (`"YYYY-MM-DD"`) — this is already handled by the `chrono` serde feature.

3. **Ensure `db/` directory exists:**
   - `TaskMap::load` creates the file but not the directory.
   - Add directory creation logic (e.g., `fs::create_dir_all("db")`) before loading/saving.

4. **Register commands in Tauri:**
   ```rust
   fn main() {
       tauri::Builder::default()
           .invoke_handler(tauri::generate_handler![
               get_tasks,
               add_task,
               toggle_task,
               delete_task
           ])
           .run(tauri::generate_context!())
           .expect("error while running tauri application");
   }
   ```

### Frontend Integration

On the React side, replace `localStorage` calls with Tauri `invoke` calls:

```javascript
import { invoke } from "@tauri-apps/api/core";

// Load tasks
const tasks = await invoke("get_tasks");

// Add a task
await invoke("add_task", { name, subject, dueDate });

// Toggle completion
await invoke("toggle_task", { name });

// Delete a task
await invoke("delete_task", { name });
```

### Data Flow

```
User Action (React)
  → invoke("command_name", { args })
    → Tauri IPC bridge
      → Rust #[tauri::command] function
        → TaskMap::load("db/task.json")
        → Mutate data
        → TaskMap::save("db/task.json")
      → Return Result to frontend
    → Frontend re-fetches tasks via invoke("get_tasks")
  → React state updates → UI re-renders
```

### Error Handling

- All Tauri commands return `Result<T, String>`.
- Frontend should catch errors from `invoke` and display user-friendly messages (e.g., toast notification or inline error).
- Common errors: file not found (first run), duplicate task name, task not found for toggle/delete.

---

## Notes

- All task mutations (add, complete, delete) must persist to `db/task.json` via Tauri backend commands.
- The table should reactively update after any mutation without a full page reload.
- Smooth transitions for modal open/close, row completion toggle, and show more/less.
- The subject filter dropdown should dynamically populate from the existing tasks' subjects.
