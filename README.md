---
title: Project Setup
---

## Project Overview

This Rust project captures the screen using the X11 library, processes the image data, and saves it as a PNG file. The project uses the `image` crate for image handling and `dirs` crate to manage directory paths.

## Dependencies

This project relies on the following Rust crates:

- `image` (for image processing and saving)
- `dirs` (for managing directory paths)
- `x11` (for interfacing with the X11 display server)

## Project Setup

### 1. Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/): The Rust programming language.
- [X11 Development Libraries](https://xorg.freedesktop.org/wiki/): Required for interfacing with the X11 server.

### 2. Install Dependencies

Add the necessary dependencies to your `Cargo.toml` file:

```toml
[dependencies]
image = "0.24"
dirs = "5.0"
x11 = "2.19"
```

### 3. Project Structure

Ensure your project directory has the following structure:

```
your_project/
├── Cargo.toml
├── src/
│   └── main.rs
```

### 4. Code Explanation

- **`main.rs`**: Contains the main logic to capture the screen, process the image data, and save the image.

#### Key Code Sections

- **Connecting to X11 Server**:

  ```rust
  let display: *mut _XDisplay = unsafe { XOpenDisplay(ptr::null()) };
  ```

- **Capturing Screen Image**:

  ```rust
  let image: *mut XImage = unsafe {
      XGetImage(
          display,
          root_window,
          0,
          0,
          width as u32,
          height as u32,
          XAllPlanes(),
          ZPixmap,
      )
  };
  ```

- **Processing Image Data**:

  ```rust
  let img = extract_image_buffer(image);
  ```

- **Saving the Image**:
  ```rust
  img.save(&filepath).expect("failed to save image");
  ```

### 5. Running the Project

1. **Build the Project**: Compile the project using Cargo.

   ```sh
   cargo build
   ```

2. **Run the Project**: Execute the compiled binary.

   ```sh
   cargo run
   ```

3. **Check Output**: The screenshot will be saved in the `~/Pictures/Screenshots` directory with a timestamped filename.

### 6. Error Handling

Ensure you handle errors effectively:

- **Failed to Connect to X11 Server**:

  ```rust
  if display.is_null() {
      eprintln!("failed to connect to X11 server with the default display");
      return Ok(());
  }
  ```

- **Failed to Capture Image**:

  ```rust
  if image.is_null() {
      eprintln!("failed to get image from X11 server");
      unsafe { XCloseDisplay(display) };
      return Ok(());
  }
  ```

- **Failed to Save Image**:
  ```rust
  img.save(&filepath).expect("failed to save image");
  ```

### 7. Directory Management

Ensure the `Pictures/Screenshots` directory exists or create it:

```rust
let screenshots_dir = home.join("Pictures").join("Screenshots");
if !screenshots_dir.exists() {
    fs::create_dir_all(&screenshots_dir)?;
}
```

## Conclusion

This project demonstrates capturing the screen on X11, processing the image data, and saving it in PNG format using Rust. Ensure you have all dependencies installed and configured correctly for smooth execution.
