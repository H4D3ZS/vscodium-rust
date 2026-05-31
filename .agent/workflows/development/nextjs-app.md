---
description: Create a new Next.js application with TypeScript, Tailwind CSS, ESLint, and modern best practices
---

# New Next.js Application

I will help you create a new Next.js application with the latest best practices and a solid foundation.

## Guardrails
- Always use TypeScript for type safety
- Use App Router (not Pages Router) unless explicitly requested otherwise
- Prefer server components where possible
- Keep dependencies minimal and purposeful

## Steps

### 1. Initialize Project
// turbo
```bash
npx create-next-app@latest . --typescript --tailwind --eslint --app --src-dir --import-alias "@/*"
```

If the project already exists, skip this step.

### 2. Install Essential Dependencies
// turbo
```bash
npm install clsx tailwind-merge lucide-react
```

Optional but recommended:
```bash
npm install @tanstack/react-query zod
```

### 3. Set Up Utilities

Create `src/lib/utils.ts`:
```typescript
import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
```

### 4. Configure Tailwind (Optional Enhancements)

Update `tailwind.config.ts` with custom theme extensions:
```typescript
import type { Config } from "tailwindcss"

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["var(--font-geist-sans)"],
        mono: ["var(--font-geist-mono)"],
      },
    },
  },
  plugins: [],
}
export default config
```

### 5. Clean Up Default Page

Replace `src/app/page.tsx` content with:
```tsx
export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <h1 className="text-4xl font-bold">Hello, World!</h1>
    </main>
  )
}
```

### 6. Create Folder Structure

```
src/
├── app/
│   ├── layout.tsx
│   ├── page.tsx
│   └── globals.css
├── components/
│   └── ui/           # Reusable UI components
├── lib/
│   └── utils.ts      # Utility functions
├── hooks/            # Custom React hooks
└── types/            # TypeScript type definitions
```

### 7. Verify Setup
// turbo
```bash
npm run dev
```

Open http://localhost:3000 to verify the app is running.

## Guidelines
- Use `@/` import alias for clean imports
- Keep components small and focused
- Use TypeScript strict mode
- Prefer CSS Modules or Tailwind over CSS-in-JS
- Use Next.js built-in features (Image, Link, Font optimization)

## Reference
- [Next.js Docs](https://nextjs.org/docs)
- [Tailwind CSS Docs](https://tailwindcss.com/docs)
