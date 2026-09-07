#!/usr/bin/env node
/**
 * Minimal logging reverse proxy for local debugging (no dependencies).
 * Forwards everything to TARGET (default http://localhost:3000) and logs
 * request/response metadata.
 *
 * Usage: node tools/mitmserver/server.ts [--port 8080] [--target http://localhost:3000]
 * Runs directly under Node >=23.6 (native TypeScript type stripping).
 */
import http from 'node:http';

function argValue(flag: string, fallback: string): string {
    const i = process.argv.indexOf(flag);
    return i !== -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

const PORT = Number(argValue('--port', '8080'));
const TARGET = new URL(argValue('--target', 'http://localhost:3000'));

const server = http.createServer((req, res) => {
    console.log('Request received:');
    console.log('  Method:', req.method);
    console.log('  URL:', req.url);
    console.log('  Headers:', req.headers);

    const upstream = http.request(
        {
            hostname: TARGET.hostname,
            port: TARGET.port || 80,
            path: req.url,
            method: req.method,
            headers: { ...req.headers, host: TARGET.host },
        },
        (upstreamRes) => {
            console.log('Response received:');
            console.log('  Status:', upstreamRes.statusCode);
            console.log('  Headers:', upstreamRes.headers);
            res.writeHead(upstreamRes.statusCode ?? 502, upstreamRes.headers);
            upstreamRes.pipe(res);
        },
    );

    upstream.on('error', (e) => {
        console.error('Proxy error:', e);
        res.writeHead(502);
        res.end('There was an error proxying the request.');
    });

    req.pipe(upstream);
});

server.listen(PORT, () => {
    console.log(`MITM proxy listening on :${PORT} → ${TARGET.origin}`);
});
