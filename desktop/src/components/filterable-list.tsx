import React, { useState } from 'react';
import { Container, Form, ListGroup, Badge } from 'react-bootstrap';

const INITIAL_ITEMS = [
  { id: 1, name: 'Wireless Headphones', category: 'Electronics' },
  { id: 2, name: 'Ergonomic Office Chair', category: 'Furniture' },
  { id: 3, name: 'Mechanical Keyboard', category: 'Electronics' },
  { id: 4, name: 'Stainless Steel Water Bottle', category: 'Accessories' },
  { id: 5, name: 'Standing Desk Converter', category: 'Furniture' },
];

export default function FilterableList() {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedCategory, setSelectedCategory] = useState('All');

  // Extract unique categories for a dropdown filter
  const categories = ['All', ...new Set(INITIAL_ITEMS.map((item) => item.category))];

  // Filter items based on both search text and selected category
  const filteredItems = INITIAL_ITEMS.filter((item) => {
    const matchesSearch = item.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesCategory = selectedCategory === 'All' || item.category === selectedCategory;
    return matchesSearch && matchesCategory;
  });

  return (
    <Container className="mt-4" style={{ maxWidth: '600px' }}>
      <h3 className="mb-3">Product Directory</h3>

      {/* Search & Category Controls */}
      <Form className="d-flex gap-2 mb-3">
        <Form.Control
          type="search"
          placeholder="Search items..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
        <Form.Select
          value={selectedCategory}
          onChange={(e) => setSelectedCategory(e.target.value)}
          style={{ width: '180px' }}
        >
          {categories.map((cat) => (
            <option key={cat} value={cat}>
              {cat}
            </option>
          ))}
        </Form.Select>
      </Form>

      {/* Filtered List Output */}
      <ListGroup>
        {filteredItems.length > 0 ? (
          filteredItems.map((item) => (
            <ListGroup.Item
              key={item.id}
              className="d-flex justify-content-between align-items-center"
            >
              <span>{item.name}</span>
              <Badge bg="secondary" pill>
                {item.category}
              </Badge>
            </ListGroup.Item>
          ))
        ) : (
          <ListGroup.Item className="text-muted text-center">
            No matching items found.
          </ListGroup.Item>
        )}
      </ListGroup>
    </Container>
  );
}
