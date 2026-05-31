---
description: Deploy applications to Vercel
---

# Vercel Deployment

I will help you deploy this project to Vercel.

## Steps

1.  Ensure you have the Vercel CLI installed.
    ```bash
    npm i -g vercel
    ```

2.  Login to Vercel.
    ```bash
    vercel login
    ```

3.  Deploy to preview.
    ```bash
    vercel
    ```

4.  Deploy to production.
    ```bash
    vercel --prod
    ```

## Guidelines
- Check `vercel.json` if you need to configure build settings or routes.
- Ensure all environment variables are set in the Vercel dashboard or via CLI.
