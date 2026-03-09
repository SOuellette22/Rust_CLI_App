# Task Forge — Home Page Functional Specification

## Overview

The home page serves as the landing page for Task Forge. It introduces the application, describes its purpose, and highlights key features. This page is purely informational — no interactive task management occurs here.

---

## Technology Stack

| Layer    | Technology       |
|----------|-----------------|
| Desktop  | Tauri            |
| Frontend | React            |
| Backend  | Rust             |
| Data     | JSON persistence |

---

## Navigation Bar

A fixed navbar sits at the top of every page. It contains the following links:

| Nav Item     | Destination         |
|-------------|---------------------|
| **Home**     | Home page (current) |
| **Tasks**    | Task list page      |

### Navbar Behavior
- The currently active page should be visually indicated (e.g., highlighted or underlined nav item).
- The navbar should remain consistent across all pages.
- Smooth transition animations when navigating between pages.

### Navbar Styling
- Background: dark grey / black.
- Text: white with orange accent on the active nav item.
- Clean, minimal design with adequate spacing.
- App name "Task Forge" displayed on the left side of the navbar as a logo/brand element.

---

## Page Layout

The home page body is structured top-to-bottom in the following sections:

### 1. Hero Section

| Element     | Description                                                                                          |
|------------|------------------------------------------------------------------------------------------------------|
| **Title**   | "Task Forge" — large, bold heading centered on the page.                                             |
| **Image**   | A relevant image that relates to task management / productivity (e.g., a forge, anvil, or workflow). Centered below the title. |
| **Description** | A short paragraph (2-3 sentences) below the image explaining what Task Forge is and what it does. Centered text. |

### 2. Key Features Section

A section below the hero that highlights the core features of Task Forge. Displayed as individual feature cards or blocks arranged in a row (or responsive grid).

| Feature            | Description                                                  |
|-------------------|--------------------------------------------------------------|
| **Add Tasks**      | Create tasks with a name, subject category, and due date.    |
| **Track Progress** | View all your tasks in one place and monitor their status.   |
| **Mark Complete**  | Check off tasks as you finish them to stay organized.        |

#### Feature Card Styling
- Each feature should be displayed in its own card/block.
- Cards should have a subtle border or background to distinguish them.
- An icon or small graphic above each feature title is optional but recommended.

---

## Theme & Styling

### Color Scheme

| Role             | Light Mode          | Dark Mode            |
|-----------------|---------------------|----------------------|
| Background       | Light grey / white  | Dark grey / black    |
| Primary Accent   | Orange              | Orange               |
| Text (primary)   | Dark grey / black   | White / light grey   |
| Text (secondary) | Medium grey         | Medium grey          |
| Navbar           | Dark grey / black   | Black                |
| Feature Cards    | White with shadow   | Dark grey with border|

### Dark / Light Mode Toggle
- A toggle switch should be accessible from the navbar (e.g., a sun/moon icon).
- Switching modes should apply a smooth transition to all elements on the page.
- The user's preference should be persisted (saved to local storage).

### General Styling Guidelines
- Modern, clean, and minimal design.
- Smooth CSS transitions on all interactive elements (hover states, page navigation, theme toggle).
- Rounded corners on cards and buttons.
- Consistent spacing and padding throughout.
- Responsive design — should look good on various window sizes.
- Font: a clean sans-serif font (e.g., Inter, Poppins, or system default).

---

## Interactions

| Element              | Interaction                                      |
|---------------------|--------------------------------------------------|
| Navbar links         | Navigate to the corresponding page with smooth transition. |
| Theme toggle         | Switches between dark and light mode with a smooth transition. |
| Feature cards        | Subtle hover effect (e.g., slight lift or glow). No click action. |

---

## Wireframe (Text-Based)

```
+------------------------------------------------------------------+
| [Task Forge]          Home    Tasks    Add Task       [sun/moon]  |
+------------------------------------------------------------------+
|                                                                    |
|                         TASK FORGE                                 |
|                                                                    |
|                     [ Relevant Image ]                             |
|                                                                    |
|          A modern task management app built to help you            |
|          organize, track, and complete your tasks with ease.       |
|                                                                    |
+------------------------------------------------------------------+
|                                                                    |
|                       KEY FEATURES                                 |
|                                                                    |
|   +------------------+  +------------------+  +------------------+ |
|   |    Add Tasks     |  |  Track Progress  |  |  Mark Complete   | |
|   |                  |  |                  |  |                  | |
|   | Create tasks     |  | View all your    |  | Check off tasks  | |
|   | with a name,     |  | tasks in one     |  | as you finish    | |
|   | subject, and     |  | place and        |  | them to stay     | |
|   | due date.        |  | monitor status.  |  | organized.       | |
|   +------------------+  +------------------+  +------------------+ |
|                                                                    |
+------------------------------------------------------------------+
```

---

## Backend Connection

The home page is **purely static** — it does not require any Tauri backend commands or data from the Rust backend. All content is hardcoded in the React component.

However, the home page shares the following global concerns that involve the Tauri layer:

### Theme Persistence
- The dark/light mode preference is stored in the browser's `localStorage` via the frontend.
- No Tauri command is needed for theme persistence — `localStorage` is available in Tauri's WebView.

### Navigation
- Page routing is handled entirely on the frontend using **React Router**.
- No Tauri commands are needed for navigation.

### Tauri Window Configuration
- The Tauri config (`tauri.conf.json`) defines the window properties for the app:
  - **Window title**: "Task Forge"
  - **Default size**: 1024x720
  - **Resizable**: true
- The navbar and all pages render inside this single native window.

---

## Notes

- All content on this page is static/informational — no forms, no task data displayed.
- The hero image will need to be sourced or created. A placeholder can be used during development.
- Page navigation between Home and Tasks should use React Router with smooth transitions.
