# Resume Builder / Manual Editor Implementation Plan

Currently, users upload a PDF, which our AI parses into structured JSON (`cvProfile.structured_data`) to generate tailored proposals. However, as you rightly pointed out, users need a way to manually verify, correct, or build their resume from scratch directly in a web form if the AI missed something or if they want to update their skills without re-uploading a new PDF.

## Goal
Build an **Interactive Resume Builder** tab under "My Resume" so candidates can directly edit their structured data (Experience, Education, Skills, Profile) via manual forms. This data will automatically serve as the source of truth for the AI Autopilot and Tailoring Engine.

## Proposed Changes

### 1. Backend: Update CV Profile Endpoint
#### [MODIFY] `backend/app/main.py`
- Add a new `PUT` or `POST` endpoint `/api/cv-profile/update-structured` to accept JSON payloads representing the updated resume data.
- Update the `CVProfile` model's `structured_data` column in the database securely based on user input.

#### [MODIFY] `backend/app/schemas.py`
- Define a `StructuredCVUpdateRequest` Pydantic model containing:
  - `name`, `email`, `phone`, `location`
  - `skills` (Array of strings)
  - `experience` (Array of objects with `title`, `company`, `duration`, `description`)
  - `education` (Array of objects with `degree`, `school`, `year`)

### 2. Frontend: New ResumeBuilder Component
#### [NEW] `frontend/src/components/ResumeBuilder.jsx`
- Create a dedicated React component for the Manual Resume Editor.
- The UI will include:
  - **Basic Info Form**: Inputs for Name, Email, Phone, Location.
  - **Skills Editor**: A tag-based input to easily add/remove skills.
  - **Experience & Education Editor**: Dynamic forms to add, edit, or remove past roles and degrees.
- Ensure the design feels premium (dark mode, glassmorphism, smooth animations) matching the rest of your app.

### 3. Frontend: App.jsx Integration
#### [MODIFY] `frontend/src/App.jsx`
- Under the `ProfileView` (My Resume) component, add a toggle to switch between **"View PDF Sandbox"** and **"Manual Resume Editor"**.
- Pass the parsed `cvProfile.structured_data` to `ResumeBuilder.jsx` so it pre-fills the form with what the AI already extracted from their PDF.
- When the user hits "Save", it will push the updated JSON to the new `/api/cv-profile/update-structured` endpoint and refresh the profile.

## Verification Plan
1. **API Testing**: Verify `PUT /api/cv-profile/update-structured` correctly updates the `structured_data` string in SQLite.
2. **UI Testing**: Ensure users can seamlessly edit experience, add a skill, save, and see the changes persist across reloads.
3. **AI Testing**: Ensure that when the Autopilot tailors a resume for a job, it pulls from the newly updated manual form data rather than stale parsed data.

## User Review Required
> [!IMPORTANT]
> 1. Do you want this manual form to **completely replace** the original uploaded PDF when we send out cold outreach emails? (i.e. we dynamically generate a sleek PDF from this form data), OR do we just use this form data to **inform the AI**, but still send out their original PDF? 
> *(Recommendation: Use this manual data to inform the AI to tailor a new PDF, which is how your aggressive tailoring pipeline already works!)*
> 
> 2. Are there any specific fields you want in the manual form that standard resumes usually lack? (e.g., Portfolio Links, GitHub, LinkedIn).

Let me know if this manual builder approach looks good to you!
