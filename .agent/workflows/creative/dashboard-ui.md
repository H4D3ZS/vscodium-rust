---
description: Create professional admin dashboard interfaces for any tech stack
---

# Dashboard UI Creation

I will help you build a professional, feature-rich admin dashboard interface that adapts to your project's tech stack.

## Guardrails
- Never assume a specific framework (React, Vue, Angular, etc.)
- Detect project stack before suggesting implementations
- Use consistent layout patterns (sidebar + header)
- Ensure responsive design for tablet and desktop
- Always include loading and empty states

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What data will the dashboard display?
- What actions can users perform?
- What navigation structure is needed?
- Any specific metrics, charts, or KPIs required?
- What role/permission levels exist?
- Is there an existing design system or component library?

### 2. Analyze Project Stack
Detect the existing setup:
- **Framework**: Check for React, Vue, Angular, Svelte, etc.
- **Styling**: Check for Tailwind, CSS Modules, Sass, or CSS-in-JS
- **Component library**: Check for MUI, Shadcn, Chakra, Vuetify, etc.
- **Chart library**: Check for Recharts, Chart.js, ApexCharts, etc.
- **Existing patterns**: Look at existing components for conventions

If no existing setup or unclear, ask the user which stack they prefer.

### 3. Plan Dashboard Structure
A typical admin dashboard includes:

**Layout Components:**
- Sidebar navigation (collapsible)
- Top header with search, notifications, user menu
- Main content area
- Footer (optional)

**Common Page Types:**
- Overview/Home with stats cards and charts
- Data tables with sorting, filtering, pagination
- Detail views for individual records
- Forms for create/edit operations
- Settings pages

### 4. Create Layout Structure
Build the core layout components:
- **Sidebar**: Navigation links, active state indicators, collapse toggle
- **Header**: Breadcrumbs, search, user dropdown
- **Content wrapper**: Consistent padding and max-width

### 5. Create Dashboard Components
Build reusable components for:

**Stats Cards:**
- Title, value, change indicator
- Icon or sparkline chart
- Comparison to previous period

**Data Tables:**
- Column headers with sort controls
- Row data with actions
- Pagination controls
- Empty state when no data

**Charts:**
- Line/area charts for trends
- Bar charts for comparisons
- Pie/donut charts for distributions

### 6. Assemble Dashboard Pages
Compose components into pages:
- Follow consistent grid layouts
- Add appropriate spacing
- Include loading skeletons
- Handle empty states gracefully

### 7. Verify
- Test responsive behavior on different screen sizes
- Verify navigation works correctly
- Check loading and empty states appear correctly
- Test with sample data

## Principles

### Layout
- Use consistent sidebar width (240-280px collapsed to 64-80px)
- Content area should have max-width for readability
- Consistent padding (24px) in content areas

### Color & Theme
- Neutral base colors (gray/slate) for background
- Accent color for primary actions and active states
- Semantic colors: green (success), red (error), yellow (warning)
- Support dark mode if project requires it

### Typography
- Clear hierarchy: page title → section headers → card titles → body
- Use tabular/monospace numbers for data display

### Data Display
- Show loading skeletons while fetching data
- Provide helpful empty states with next action suggestions
- Make tables sortable and filterable for large datasets
- Use appropriate number formatting (currency, percentages)

## Reference
- Check for existing layout components in the project
- Look at the project's routing structure
- Review any existing design tokens or theme configuration
