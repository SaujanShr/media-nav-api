const { test, before, after } = require('node:test');
const assert = require('node:assert/strict');
const { spawn } = require('node:child_process');
const path = require('node:path');

const PORT = 4100;
const BASE_URL = `http://localhost:${PORT}`;

let serverProcess;

before(async () => {
  serverProcess = spawn('node', ['server.js'], {
    cwd: path.join(__dirname, '..'),
    env: { ...process.env, PORT: String(PORT) },
  });

  for (let i = 0; i < 50; i++) {
    try {
      const res = await fetch(`${BASE_URL}/health`);
      if (res.ok) return;
    } catch {
      // server not up yet
    }
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  throw new Error('server did not start in time');
});

after(() => {
  serverProcess.kill();
});

test('GET /health returns ok', async () => {
  const res = await fetch(`${BASE_URL}/health`);
  const body = await res.json();

  assert.equal(res.status, 200);
  assert.equal(body.status, 'ok');
  assert.equal(body.provider, 'example');
});

test('GET /items returns all fixtures paginated by default', async () => {
  const res = await fetch(`${BASE_URL}/items`);
  const body = await res.json();

  assert.equal(res.status, 200);
  assert.equal(body.total, 10);
  assert.equal(body.page, 1);
  assert.equal(body.limit, 10);
  assert.equal(body.items.length, 10);
});

test('GET /items respects page and limit', async () => {
  const res = await fetch(`${BASE_URL}/items?page=2&limit=3`);
  const body = await res.json();

  assert.equal(body.page, 2);
  assert.equal(body.limit, 3);
  assert.equal(body.items.length, 3);
});

test('GET /items filters by search', async () => {
  const res = await fetch(`${BASE_URL}/items?search=comic`);
  const body = await res.json();

  assert.ok(body.items.length > 0);
  assert.ok(body.items.every(item => item.title.toLowerCase().includes('comic')));
});

test('GET /items sorts by title ascending and descending', async () => {
  const asc = await (await fetch(`${BASE_URL}/items?sort=title&direction=asc&limit=10`)).json();
  const desc = await (await fetch(`${BASE_URL}/items?sort=title&direction=desc&limit=10`)).json();

  const titles = asc.items.map(item => item.title);
  const sorted = [...titles].sort((a, b) => a.localeCompare(b));
  assert.deepEqual(titles, sorted);
  assert.deepEqual(desc.items.map(item => item.title), [...sorted].reverse());
});

test('GET /items/:id returns the matching detail', async () => {
  const res = await fetch(`${BASE_URL}/items/comic-1`);
  const body = await res.json();

  assert.equal(res.status, 200);
  assert.equal(body.id, 'comic-1');
});

test('GET /items/:id returns 404 for an unknown id', async () => {
  const res = await fetch(`${BASE_URL}/items/does-not-exist`);
  const body = await res.json();

  assert.equal(res.status, 404);
  assert.equal(body.id, 'does-not-exist');
});

test('unknown routes return 404', async () => {
  const res = await fetch(`${BASE_URL}/nope`);
  const body = await res.json();

  assert.equal(res.status, 404);
  assert.equal(body.path, '/nope');
});
