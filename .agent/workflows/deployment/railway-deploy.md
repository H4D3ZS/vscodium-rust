---
description: Deploy applications to Railway with database and environment configuration
---

# Railway Deployment

I will help you deploy your application to Railway with proper configuration.

## Guardrails
- Never commit secrets to the repository
- Use Railway's environment variables for all sensitive data
- Set up proper health checks
- Configure resource limits appropriately

## Steps

### 1. Pre-Deployment Checklist
- Ensure your app has a `package.json` with a `start` script
- Add a `Procfile` if needed for custom start commands
- Verify `.gitignore` includes sensitive files

### 2. Install Railway CLI (Optional)
// turbo
```bash
npm install -g @railway/cli
```

Login to Railway:
```bash
railway login
```

### 3. Initialize Railway Project

**Option A: Via CLI**
```bash
railway init
```

**Option B: Via Dashboard**
1. Go to [railway.app](https://railway.app)
2. Click "New Project"
3. Select "Deploy from GitHub repo"
4. Authorize and select your repository

### 4. Configure Environment Variables

Via CLI:
```bash
railway variables set DATABASE_URL="your-database-url"
railway variables set NODE_ENV="production"
railway variables set SECRET_KEY="your-secret-key"
```

Via Dashboard:
1. Select your project
2. Go to "Variables" tab
3. Add key-value pairs

### 5. Add Database (if needed)

**PostgreSQL:**
```bash
railway add --plugin postgresql
```

**Redis:**
```bash
railway add --plugin redis
```

**MongoDB:**
```bash
railway add --plugin mongodb
```

Railway automatically injects database connection URLs as environment variables.

### 6. Configure Build Settings

Create `railway.toml` in project root:
```toml
[build]
builder = "nixpacks"
buildCommand = "npm run build"

[deploy]
startCommand = "npm start"
healthcheckPath = "/api/health"
healthcheckTimeout = 300
restartPolicyType = "on_failure"
restartPolicyMaxRetries = 3
```

### 7. Configure for Specific Frameworks

**Next.js** - Add to `package.json`:
```json
{
  "scripts": {
    "build": "next build",
    "start": "next start -p $PORT"
  }
}
```

**Express.js** - Use PORT environment variable:
```javascript
const port = process.env.PORT || 3000
app.listen(port, () => {
  console.log(`Server running on port ${port}`)
})
```

### 8. Deploy
// turbo
```bash
railway up
```

Or push to your connected GitHub branch for automatic deployment.

### 9. Set Up Custom Domain

```bash
railway domain
```

Or via Dashboard:
1. Go to project settings
2. Click "Add Custom Domain"
3. Add your domain and configure DNS

### 10. View Logs

```bash
railway logs
```

Or view in the Railway dashboard under "Deployments" → "View Logs"

## Health Check Endpoint

Create `pages/api/health.ts` (Next.js) or equivalent:
```typescript
import type { NextApiRequest, NextApiResponse } from 'next'

export default function handler(req: NextApiRequest, res: NextApiResponse) {
  res.status(200).json({ status: 'ok', timestamp: new Date().toISOString() })
}
```

## Troubleshooting

### Build Failures
```bash
# Check build logs
railway logs --build

# Rebuild with clear cache
railway up --clear-build-cache
```

### Memory Issues
Update `railway.toml`:
```toml
[deploy]
numReplicas = 1
```

Or upgrade your Railway plan for more resources.

### Database Connection Issues
- Verify DATABASE_URL is set correctly
- Check if database plugin is running
- Ensure connection pooling for serverless environments

## Guidelines
- Use preview environments for PRs
- Set up notifications for failed deployments
- Monitor resource usage in dashboard
- Enable automatic deploys from main branch

## Reference
- [Railway Docs](https://docs.railway.app)
- [Railway Templates](https://railway.app/templates)
- [Nixpacks](https://nixpacks.com/docs) - Build system documentation
