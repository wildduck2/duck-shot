To build a desktop-only app in Rust that captures and manipulates screenshots similar to Flameshot, here's a detailed plan and step-by-step **To-Do List**:

### 1. **Project Setup**

- **Install Rust**: Ensure Rust is installed on your system. You can install it by following the instructions at [rustup.rs](https://rustup.rs/).
- **Create a new Rust project**:
  ```bash
  cargo new screenshot_app
  cd screenshot_app
  ```
- **Tooling**: Set up your environment with any necessary tools (like `cargo` and `cargo-edit`) to easily manage dependencies.

### 2. **Choosing a GUI Framework**

**Goal**: Build the user interface (UI) for the app.

- **Select GUI Library**: Use a native Rust library to create the desktop interface.

  - **Iced** (recommended): Modern and simple-to-use GUI library with cross-platform support.
  - **GTK-RS**: If you want a more mature toolkit, especially for Linux.

- **To-Do**:
  1.  Research Iced or GTK-RS and decide which suits your needs.
  2.  Install the chosen library by adding it to your `Cargo.toml`.
  3.  Create a simple GUI with a window and basic buttons like “Take Screenshot” and “Save Image.”
  4.  Handle basic UI events like button clicks and file save dialogs.

### 3. **Screenshot Capture**

**Goal**: Capture the screen (full screen, selected area, or specific window).

- **Select a Screenshot Library**:

  - **`screenshot-rs`**: Cross-platform screenshot capture library for Rust (works on Windows, macOS, and Linux).

- **To-Do**:
  1.  Add the `screenshot` library to your `Cargo.toml`.
  2.  Implement screen capture functionality for:
      - Fullscreen capture.
      - Custom region capture (you may need to integrate with the GUI to allow user selection).
  3.  Ensure that the screenshots are captured in an image format (e.g., PNG, JPEG).

### 4. **Image Manipulation**

**Goal**: Allow basic image editing such as cropping, resizing, and adding annotations (text, arrows, highlights).

- **Select an Image Processing Library**:

  - **`image` crate**: Popular Rust library for image manipulation.
  - **Annotation Options**: You can use `rusttype` for text rendering or directly manipulate pixel data for shapes.

- **To-Do**:
  1.  Install the `image` crate for loading, editing, and saving screenshots.
  2.  Implement basic image manipulation features:
      - **Cropping**: Allow users to crop specific areas.
      - **Resizing**: Add functionality to resize the image.
      - **Annotations**: Support drawing arrows, rectangles, lines, and text on the screenshot.
      - **Color Picking**: Add a color picker to customize annotations.
  3.  Add undo/redo functionality for image edits (optional).

### 5. **Interactive GUI for Annotations**

**Goal**: Build interactive tools for drawing and annotating the screenshot (arrows, lines, shapes, etc.).

- **To-Do**:
  1.  Set up drawing modes (e.g., arrow, rectangle, text) in the GUI.
  2.  Implement user interactions such as:
      - Click-and-drag for drawing shapes.
      - Text input mode for adding text.
  3.  Add mouse and keyboard input handling for more precision.
  4.  Display a preview of the annotations before saving.

### 6. **Saving Screenshots**

**Goal**: Allow users to save screenshots to their desired location after editing.

- **To-Do**:
  1.  Use a file dialog to let users choose the save location (integrate with your GUI library).
  2.  Allow saving in different formats (PNG, JPEG, etc.).
  3.  Handle file overwrites and file naming logic (ask the user for confirmation if the file exists).

### 7. **Clipboard Integration**

**Goal**: Copy the screenshot or edited image directly to the system clipboard.

- **To-Do**:
  1.  Research Rust clipboard libraries like `copypasta` or `clipboard`.
  2.  Add functionality to copy screenshots/edits to the clipboard with a single button click.

### 8. **Delay Capture**

**Goal**: Implement a feature to capture the screen after a short delay (useful for capturing menus or tooltips).

- **To-Do**:
  1.  Add a timer functionality that waits before capturing the screenshot (use `std::thread::sleep` or GUI timer functions).
  2.  Add a countdown UI in your app to let users know when the screenshot will be taken.

### 9. **Custom Hotkeys**

**Goal**: Set up global keyboard shortcuts for taking screenshots, even when the app is minimized or in the background.

- **To-Do**:
  1.  Research and integrate a global hotkey library like `hotkey` or `keyboard`.
  2.  Allow users to configure custom hotkeys for actions like capturing the full screen or a region.
  3.  Ensure hotkeys work across platforms (Windows, macOS, Linux).

### 10. **Cross-Platform Support**

**Goal**: Ensure the app works on Windows, macOS, and Linux.

- **To-Do**:
  1.  Test the app on multiple platforms and handle platform-specific behaviors (e.g., different file paths or UI quirks).
  2.  Handle platform-specific features (e.g., clipboard integration or screenshot mechanics might differ).
  3.  Use CI/CD tools like GitHub Actions to automate cross-platform builds.

### 11. **App Packaging**

**Goal**: Package the app into a native installer for different platforms.

- **To-Do**:
  1.  **Windows**: Use `cargo-bundle` to package the app as an `.exe` installer.
  2.  **macOS**: Package the app as a `.dmg` file using `cargo-bundle` or `cargo-tauri-bundle`.
  3.  **Linux**: Package the app as a `.deb` or `.AppImage` using tools like `cargo-deb`.

### 12. **Optional Features**

- **Cloud Integration**: Allow users to upload screenshots to cloud storage services (e.g., Dropbox, Google Drive) using APIs.
- **Image Format Conversion**: Let users convert screenshots into different formats (PNG, JPEG, etc.) before saving.
- **Advanced Editing Tools**: Add more advanced editing tools such as blurring parts of the screenshot, color filters, or pixelation (useful for sensitive information).

---

### Summary of Tasks:

1. **Set Up Project**: Initialize Rust project and decide on a GUI library.
2. **Screenshot Capture**: Implement screen capture functionality using `screenshot-rs`.
3. **Image Manipulation**: Implement cropping, resizing, annotations, and text features.
4. **GUI for Editing**: Build interactive GUI for drawing, annotating, and selecting areas.
5. **Save & Export**: Enable saving screenshots in different formats and handling file dialogues.
6. **Clipboard Support**: Integrate with the system clipboard for easy pasting.
7. **Delayed Capture**: Add functionality to delay screenshot capture by a user-defined time.
8. **Custom Hotkeys**: Implement global hotkeys for quick screenshotting.
9. **Cross-Platform Compatibility**: Ensure your app runs smoothly on Windows, macOS, and Linux.
10. **Packaging & Distribution**: Package your app for distribution across different platforms.

This should give you a solid roadmap for developing the app. Each step can be broken down into smaller tasks as you dive into the implementation details.
