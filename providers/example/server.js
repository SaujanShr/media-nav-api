const express = require('express');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 4000;

// Load fixtures
const itemsPath = path.join(__dirname, 'fixtures', 'items.json');
const detailsPath = path.join(__dirname, 'fixtures', 'details.json');

let items = [];
let details = {};

try {
  items = JSON.parse(fs.readFileSync(itemsPath, 'utf8'));
  details = JSON.parse(fs.readFileSync(detailsPath, 'utf8'));
  console.log(`Loaded ${items.length} items from fixtures`);
} catch (err) {
  console.error('Error loading fixtures:', err);
  process.exit(1);
}

// Middleware
app.use(express.json());

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ status: 'ok', provider: 'example' });
});

// Get all items (with optional pagination, search, and sort)
app.get('/items', (req, res) => {
  const page = parseInt(req.query.page) || 1;
  const limit = parseInt(req.query.limit) || 10;
  const { search, sort, direction } = req.query;

  let filtered = items;
  if (search) {
    const needle = search.toLowerCase();
    filtered = filtered.filter(item => item.title.toLowerCase().includes(needle));
  }

  if (sort === 'title') {
    filtered = [...filtered].sort((a, b) => a.title.localeCompare(b.title));
    if (direction === 'desc') filtered.reverse();
  }

  const offset = (page - 1) * limit;
  const paginatedItems = filtered.slice(offset, offset + limit);

  res.json({
    items: paginatedItems,
    total: filtered.length,
    page,
    limit
  });
});

// Get item details by ID
app.get('/items/:id', (req, res) => {
  const { id } = req.params;

  if (!details[id]) {
    return res.status(404).json({
      error: 'Item not found',
      id
    });
  }

  res.json(details[id]);
});

// 404 handler
app.use((req, res) => {
  res.status(404).json({
    error: 'Endpoint not found',
    path: req.path
  });
});

// Error handler
app.use((err, req, res, next) => {
  console.error('Server error:', err);
  res.status(500).json({
    error: 'Internal server error',
    message: err.message
  });
});

// Start server
app.listen(PORT, () => {
  console.log(`Example provider server running on port ${PORT}`);
  console.log(`Available endpoints:`);
  console.log(`  GET /health`);
  console.log(`  GET /items`);
  console.log(`  GET /items/:id`);
});
