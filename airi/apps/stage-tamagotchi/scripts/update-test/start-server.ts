/* eslint-disable no-console */
import process, { exit } from 'node:process'

import { readFile } from 'node:fs/promises'
import { createServer } from 'node:http'
import { extname, join, normalize } from 'node:path'

import { cac } from 'cac'

const CONTENT_TYPES: Record<string, string> = {
  '.exe': 'application/vnd.microsoft.portable-executable',
  '.yml': 'text/yaml; charset=utf-8',
  '.yaml': 'text/yaml; charset=utf-8',
  '.zip': 'application/zip',
  '.dmg': 'application/octet-stream',
  '.deb': 'application/vnd.debian.binary-package',
  '.rpm': 'application/x-rpm',
  '.txt': 'text/plain; charset=utf-8',
}

function getContentType(pathname: string) {
  return CONTENT_TYPES[extname(pathname)] ?? 'application/octet-stream'
}

export async function startUpdateTestServer(options: { port: number, rootDir: string }) {
  const server = createServer(async (request, response) => {
    const pathname = request.url?.split('?')[0] || '/'
    // eslint-disable-next-line e18e/prefer-static-regex
    const safePath = normalize(pathname).replace(/^(\.\.(\/|\\|$))+/, '')
    const filePath = join(options.rootDir, safePath === '/' ? '/index.html' : safePath)

    try {
      const body = await readFile(filePath)
      response.writeHead(200, { 'content-type': getContentType(filePath) })
      response.end(body)
    }
    catch {
      response.writeHead(404, { 'content-type': 'text/plain; charset=utf-8' })
      response.end(`Not found: ${pathname}`)
    }
  })

  await new Promise<void>((resolve, reject) => {
    server.once('error', reject)
    server.listen(options.port, 'localhost', () => resolve())
  })

  return server
}

async function main() {
  const cli = cac('update-test-server')
    .option('--port <port>', 'Port to listen on', { default: '8787' })
    .option('--root <path>', 'Server root directory', { default: 'scripts/update-test/fixtures/server' })

  const parsed = cli.parse()
  const port = Number(parsed.options.port)
  const rootDir = String(parsed.options.root)

  const server = await startUpdateTestServer({ port, rootDir })

  console.log(`Update test server listening on http://localhost:${port}`)
  console.log(`stable:  http://localhost:${port}/stable`)
  console.log(`nightly: http://localhost:${port}/nightly`)
  console.log(`canary:  http://localhost:${port}/canary`)

  const close = async () => {
    await new Promise<void>((resolve, reject) => {
      server.close(error => error ? reject(error) : resolve())
    })
    exit(0)
  }

  process.on('SIGINT', () => {
    void close()
  })
  process.on('SIGTERM', () => {
    void close()
  })
}

if (import.meta.main) {
  main().catch((error) => {
    console.error(error)
    exit(1)
  })
}
