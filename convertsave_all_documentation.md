# ConvertSave

## Project Description
This will be a native application using Tauri. Frontend will be React/Tailwind/shadcn. The app will be a local file conversion utility that will take one file and convert it into a number of other file types. We will be using ffmpeg, pandoc, libreoffice, imagemagick and more to do the conversions. Whatever tools we can use we should use. The app will be cross-platform for Windows, Mac, and Linux. We want the look of it to be similar to how Gumroad has its UI with chunky colorful buttons with muted tones. There will be a very simple UI where the user can just drop a file into it or the user can click on the drop zone and it opens a file explorer. Then there's an option for whatever file types we can convert to. We should allow for literally any file type that our tools can convert

## Product Requirements Document
Product Requirements Document (PRD)

1.  Introduction
    1.1. Project Name: ConvertSave
    1.2. Document Version: 1.0
    1.3. Date: October 26, 2023
    1.4. Author: [Your Name/Team Name]
    1.5. Purpose: This document outlines the product requirements for ConvertSave, a native, cross-platform file conversion utility. It details the product's goals, features, functional and non-functional requirements, and design specifications.

2.  Product Overview
    ConvertSave is designed to be a comprehensive local file conversion utility. It will operate as a native desktop application, leveraging the Tauri framework for cross-platform compatibility (Windows, macOS, Linux). The application's core functionality is to take a single input file and convert it into various other file types, utilizing a suite of powerful, pre-bundled command-line tools such as FFmpeg, Pandoc, LibreOffice, and ImageMagick. The user interface will be simple, intuitive, and visually distinct, aiming for an aesthetic similar to Gumroad's UI with chunky, colorful buttons and muted tones. The primary goal is to provide a robust, easy-to-use tool for any user requiring local file conversions, without relying on cloud services.

3.  Goals & Objectives
    3.1. Primary Goal: To provide a highly versatile, local file conversion utility capable of converting "literally any file type that our tools can convert" using bundled third-party command-line utilities.
    3.2. Secondary Objectives:
        *   Deliver a native, cross-platform application for Windows, macOS, and Linux.
        *   Ensure a simple, intuitive, and visually appealing user interface.
        *   Allow users granular control over conversion settings through advanced options.
        *   Bundle all necessary conversion tools within the application package for offline functionality and ease of use.
        *   Ensure the application "works properly" across various hardware configurations.
4.  Target Audience & Use Cases
    4.1. Target Audience: Any user looking to convert files locally, from casual users to those requiring specific, advanced conversion options. Marketing will be conducted through internal YouTube channels.
    4.2. Use Cases:
        *   **Basic File Conversion**: A user has a `.mov` file and needs to convert it to `.mp4` for sharing. They drag and drop the `.mov` file into ConvertSave, select `.mp4` from the list of output options, and click "Convert".
        *   **Specific Document Conversion**: A student needs to convert a `.docx` file into a `.pdf` or `.epub`. They open ConvertSave, click the drop zone to open a file explorer, select the `.docx` file, choose the desired output format, and initiate conversion.
        *   **Advanced Image Conversion**: A designer wants to convert a `.png` to a `.jpg` with specific quality settings or apply a resize operation not available in basic options. They use the advanced options section to input specific ImageMagick flags.
        *   **Batch Conversion (Future Consideration/Implicit)**: While not explicitly requested, the architecture should ideally allow for future expansion into simple batch processing, even if not implemented initially. The current scope is for single file conversion.
        *   **Custom Output Location**: A user wants to save their converted file directly to a specific project folder instead of the default output directory. They select a custom destination folder before conversion.

5.  Scope
    5.1. In Scope for Initial Release:
        *   Native desktop application built with Tauri (Rust backend).
        *   Frontend built with React, Tailwind CSS, and shadcn/ui.
        *   Core functionality: Single file input (drag-and-drop or file explorer selection).
        *   Dynamic display of available output file types based on input.
        *   Execution of conversions using bundled external tools (FFmpeg, Pandoc, LibreOffice, ImageMagick, etc.).
        *   Cross-platform compatibility for Windows, macOS, and Linux.
        *   User-selectable output directory, in addition to a configurable default output directory.
        *   Hidden "Advanced Options" section allowing users to input raw command-line flags/parameters for the underlying conversion tools.
        *   UI design adhering to the Gumroad-like aesthetic (chunky, colorful buttons, muted tones, drop shadows, Mabry Pro-like font).
    5.2. Out of Scope for Initial Release:
        *   Cloud-based conversion services.
        *   Complex file editing features (e.g., video trimming, image resizing via GUI controls, beyond what advanced options allow).
        *   Right-click context menu integration for file types (this is a future scope item).
        *   Batch conversion (initial focus is on single-file conversions).
        *   In-app purchase or monetization features.
        *   Automatic updates for bundled external tools (manual updates via app updates).
6.  Functional Requirements
    6.1. File Input (FR-001)
        *   FR-001.01: The application MUST provide a visible "drop zone" area for users to drag and drop a single file for conversion.
        *   FR-001.02: Clicking on the "drop zone" area MUST open a native file explorer dialog for users to select a single input file.
        *   FR-001.03: The application MUST validate the selected input file to ensure it exists and is accessible.
    6.2. Conversion Type Selection (FR-002)
        *   FR-002.01: Upon successful input file selection, the application MUST dynamically display a list of possible output file types.
        *   FR-002.02: The list of output types MUST be determined by the capabilities of the bundled conversion tools for the given input file type.
        *   FR-002.03: The user MUST be able to select one desired output file type from the displayed list.
    6.3. Output Management (FR-003)
        *   FR-003.01: The application MUST have a default output directory (e.g., "Converted Files" within user's Documents or a configurable location).
        *   FR-003.02: The user MUST be able to specify a custom destination folder for the current conversion, overriding the default.
        *   FR-003.03: The application MUST ensure that the output file name defaults to the input file name with the new extension, but also allow for a simple rename (if desired, though not explicitly asked, it's good UX).
        *   FR-003.04: The application MUST handle filename conflicts gracefully (e.g., appending a number if a file with the same name already exists in the destination, or prompting user).
    6.4. Conversion Execution (FR-004)
        *   FR-004.01: The application MUST execute the appropriate bundled command-line tool (FFmpeg, Pandoc, LibreOffice, ImageMagick, etc.) for the chosen conversion.
        *   FR-004.02: The application MUST display a progress indicator during the conversion process.
        *   FR-004.03: Upon successful conversion, the application MUST notify the user and potentially offer to open the output directory.
        *   FR-004.04: In case of conversion failure, the application MUST display an error message and provide relevant diagnostic information (e.g., tool error output, if available).
    6.5. Advanced Options (FR-005)
        *   FR-005.01: The application MUST include a section for "Advanced Options" that is initially tucked away/hidden to maintain a clean UI.
        *   FR-005.02: The advanced options section MUST provide a text input area where users can write their own advanced options and feature flags specific to the chosen conversion tool (e.g., FFmpeg parameters, Pandoc arguments).
        *   FR-005.03: The application MAY provide GUI options for some important, commonly used settings (e.g., video bitrate, image quality), but the primary mechanism for granularity will be the raw command-line input.
        *   FR-005.04: The application MUST correctly pass these user-defined options to the invoked conversion tool.    6.6. Bundled Tools (FR-006)
        *   FR-006.01: The application installation MUST include all necessary third-party conversion tools (FFmpeg, Pandoc, LibreOffice, ImageMagick, and others as identified) directly within the application package.
        *   FR-006.02: The application MUST be able to locate and execute these bundled tools reliably across all supported operating systems.

7.  Non-Functional Requirements
    7.1. Performance (NFR-001)
        *   NFR-001.01: The application MUST function properly without significant crashes or freezes.
        *   NFR-001.02: Performance is not a key metric; the application's speed will largely depend on the underlying conversion tools and user hardware. The focus is on stability and correctness.
    7.2. Usability & UX (NFR-002)
        *   NFR-002.01: The UI MUST be "very simple," providing a clear flow from file input to conversion.
        *   NFR-002.02: The visual design MUST resemble Gumroad's UI, featuring chunky, colorful buttons with drop shadows and muted pastel tones.
        *   NFR-002.03: The primary typeface should be Ubuntu or a visually similar font.
        *   NFR-002.04: The "Advanced Options" section MUST be initially hidden or tucked away to avoid cluttering the main interface.
    7.3. Technical Stack (NFR-003)
        *   NFR-003.01: The application MUST be built using Tauri for the native desktop framework.
        *   NFR-003.02: The frontend MUST be developed with React, styled using Tailwind CSS, and utilize shadcn/ui components.
        *   NFR-003.03: The backend logic for interacting with external tools MUST be implemented in Rust (via Tauri).
    7.4. Cross-Platform Compatibility (NFR-004)
        *   NFR-004.01: The application MUST function seamlessly on Windows, macOS, and Linux operating systems.
    7.5. Reliability (NFR-005)
        *   NFR-005.01: The application MUST robustly handle edge cases such as invalid file paths, corrupted input files (gracefully fail), and external tool errors.
    7.6. Security (NFR-006)
        *   NFR-006.01: As a local-only utility, the application SHOULD minimize any network communication.
        *   NFR-006.02: The application SHOULD avoid storing sensitive user data.
8.  High-Level Architecture
    *   **Frontend (UI)**: React, Tailwind CSS, shadcn/ui. Handles user interaction, file selection, display of conversion options, progress indicators, and input for advanced options.
    *   **Backend (Core Logic)**: Rust (Tauri). Responsible for:
        *   Interfacing with the OS for file system access.
        *   Managing the execution of bundled external conversion tools.
        *   Parsing and passing user-defined advanced options to the tools.
        *   Handling conversion status and error reporting back to the frontend.
    *   **External Conversion Tools**: FFmpeg, Pandoc, LibreOffice, ImageMagick, etc. These are standalone executables/binaries that will be bundled with the application and invoked by the Rust backend.

9.  Future Scope
    9.1. Right-Click Context Menu Integration:
        *   Allow users to right-click on supported file types in their operating system's file explorer.
        *   Provide an option to "Convert with ConvertSave" which automatically opens the ConvertSave application with the selected file already loaded for conversion.

## Technology Stack
This document outlines the recommended technology stack for the "ConvertSave" project, a cross-platform native desktop application designed for local file conversions. The choices prioritize robustness, cross-platform compatibility, ease of development, and the ability to seamlessly integrate and bundle external conversion utilities.

CORE APPLICATION FRAMEWORKS

*   **Tauri (with Rust)**
    *   **Description:** A framework for building cross-platform native applications using web technologies for the UI. It leverages Rust for the backend and native OS interactions.
    *   **Justification:**
        *   **Cross-Platform Compatibility:** Directly addresses the requirement for Windows, macOS, and Linux support.
        *   **Performance & Security:** Rust's native performance ensures efficient handling of local file operations and interactions with external executables. Its focus on memory safety contributes to a more secure application.
        *   **Small Bundle Sizes:** Tauri produces significantly smaller executable sizes compared to alternatives like Electron, beneficial for distribution.
        *   **Native Integration:** Provides robust APIs for system-level operations, which will be crucial for bundling external tools, facilitating future right-click menu integrations, and efficient file system access.        *   **Web Technologies for UI:** Allows leveraging familiar web development tools for the frontend, aligning with the project's frontend choice.

*   **React**
    *   **Description:** A JavaScript library for building user interfaces.
    *   **Justification:**
        *   **Component-Based Architecture:** Facilitates modular and reusable UI components, ideal for building a flexible and extensible user interface.
        *   **Declarative UI:** Simplifies UI development and state management, leading to more predictable code.
        *   **Large Ecosystem & Community:** Abundant resources, libraries, and community support are available for rapid development and problem-solving.
        *   **Familiarity:** A common and well-supported choice for modern web UIs, seamlessly integrating with Tauri's web-based frontend approach.

*   **Tailwind CSS**
    *   **Description:** A utility-first CSS framework for rapidly building custom designs.
    *   **Justification:**
        *   **Rapid UI Development:** Speeds up UI creation, especially for achieving the specific "chunky colorful buttons with muted tones" and "drop shadows" aesthetic without writing extensive custom CSS.
        *   **Consistency:** Encourages consistent styling across the application through a predefined set of utility classes.
        *   **Theming & Customization:** Easily configurable to match the specific "Gumroad-like" visual style, including custom color palettes and typography (e.g., "Ubuntu font").
        *   **Optimized Output:** Purges unused CSS, leading to smaller final bundle sizes for the application.

*   **shadcn/ui**
    *   **Description:** A collection of reusable components built using Radix UI and Tailwind CSS.
    *   **Justification:**
        *   **Pre-built Components:** Provides a solid foundation of accessible, customizable UI components (e.g., buttons, input fields, dialogs) that can be styled with Tailwind to achieve the desired "chunky" and "colorful pastels" aesthetic.
        *   **Headless Components:** Built on Radix UI, offering unstyled, accessible primitives that maximize customization flexibility while handling complex accessibility concerns out-of-the-box.
        *   **Tailwind Integration:** Seamlessly integrates with Tailwind CSS, leveraging the chosen styling framework for consistent and efficient styling.
## Styling Guidelines
STYLING GUIDELINES: CONVERTSAVE

1. DESIGN PHILOSOPHY & VISION
   The "ConvertSave" application will adopt a design philosophy rooted in clarity, approachability, and efficiency. Our visual style draws significant inspiration from Gumroad's UI, aiming for a professional yet friendly aesthetic.
   
   Core Aesthetic Principles:
   - Chunky Colorful Buttons: Primary actions and interactive elements will feature prominent, generously padded buttons with vibrant pastel backgrounds.
   - Muted Tones: Backgrounds, secondary elements, and general UI will utilize soft, understated colors to ensure the colorful buttons stand out without overwhelming the user.
   - Simplicity & Directness: The user interface will be minimalistic, focusing on essential functions. Key interactions, such as file dropping and conversion selection, will be intuitive and immediately visible.
   - Modern & Clean: A contemporary feel with ample whitespace, clean lines, and a focus on readability.

2. COLOR PALETTE
   The color palette features a sophisticated mix of muted neutrals and vibrant accent colors.

   A. Primary Palette:
      - Dark Text: #24262e (Primary text and strong emphasis)
      - Secondary Text: #919296 (Secondary text and muted elements)
      - White: #ffffff (Primary background)
      - Light Background: #f4f1ed (Surface/card backgrounds)
      - Lighter Background: #e7e3df (Borders and dividers)
      - Muted Background: #dbd7d5 (Subtle backgrounds)

   B. Accent Colors (For buttons and interactive elements):
      - Blue: #3562e3 (Primary actions, warning states)
      - Mint Green: #91f4c2 (Success states, secondary actions)
      - Pink: #ef87ad (Error states, tertiary actions)

   C. Usage Guidelines:
      - Use Dark Text (#24262e) for all primary text and headings
      - Use Secondary Text (#919296) for secondary text, labels, and less prominent UI elements
      - White (#ffffff) serves as the main application background
      - Light Background (#f4f1ed) is used for cards and elevated surfaces
      - Lighter Background (#e7e3df) is used for borders, dividers, and subtle separations
      - Accent colors are reserved for interactive elements and status indicators

3. TYPOGRAPHY
   The chosen typeface contributes to the modern, approachable aesthetic.

   A. Primary Font:
      - Font Family: Ubuntu (Google Fonts)
      - Font Weights: 
        - Medium (500) for body text and regular UI elements
        - Bold (700) for headings, buttons, and emphasis

   B. Font Sizing System:
      - Display (H1): 2.25rem (36px) - Bold
      - Section Titles (H2): 1.875rem (30px) - Bold
      - Component Titles (H3): 1.5rem (24px) - Bold
      - Body Text/Labels: 1rem (16px) - Medium
      - Button Text: 1.125rem (18px) - Bold
      - Small Text/Helper: 0.875rem (14px) - Medium

   C. Line Height & Letter Spacing:
      - Line Height: 1.5 for body text; 1.2 for headings
      - Letter Spacing: Standard for body; slightly tighter for headings
4. UI COMPONENTS & ELEMENTS
   Consistent application of styling across all UI elements is crucial for a cohesive user experience.

   A. Buttons:
      - Appearance: Chunky and prominent with generous padding (py-4 px-8)
      - Background: Use accent colors (Blue #3562e3, Mint Green #91f4c2, Pink #ef87ad) for primary actions
      - Border Radius: Softly rounded corners (rounded-xl)
      - Drop Shadows: Subtle shadow using Dark Text (#24262e) at low opacity
      - States:
         - Default: Full color with shadow
         - Hover: Slightly lifted with increased shadow
         - Active: Pressed appearance (no shadow)
         - Disabled: Reduced opacity with Lighter Background (#e7e3df)

   B. Drop Zone:
      - Border: 2px dashed border in Secondary Text (#919296)
      - Background: Transparent default, Mint Green/10 on hover
      - Padding: Generous (p-12) to create a large target area
      - Border Radius: rounded-xl for consistency

   C. Conversion Option Cards:
      - Background: Lighter Background (#e7e3df) when unselected, accent color when selected
      - Text: Secondary Text (#919296) when unselected, Dark Text (#24262e) when selected
      - Padding: p-4 for comfortable touch targets
      - Include format icon or abbreviation prominently

   D. Input Fields:
      - Background: Light Background (#f4f1ed)
      - Border: 1px solid Lighter Background (#e7e3df)
      - Focus: Mint Green (#91f4c2) border with subtle glow
      - Text: Dark Text (#24262e)
      - Placeholder: Secondary Text (#919296)

   E. Advanced Options Section:
      - Background: Lighter Background (#e7e3df) to indicate secondary importance
      - Collapsible with smooth animation
      - Monospace font for command-line inputs

5. ICONOGRAPHY
   - Style: Simple line icons with 2px stroke weight
   - Color: Secondary Text (#919296) for inactive, Dark Text (#24262e) for active
   - Size: 20px for standard UI, 16px for compact areas

6. LAYOUT & SPACING
   - Use an 8px grid system for consistent spacing
   - Generous whitespace between sections (space-y-8)
   - Max width container (max-w-4xl) for optimal readability
   - Center alignment for main content

7. ANIMATIONS & TRANSITIONS
   - Duration: 200ms for most transitions
   - Easing: ease-in-out for smooth feel
   - Button hover: subtle y-axis movement (-translate-y-0.5)
   - Focus states: smooth color transitions