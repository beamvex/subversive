import React, { useState } from 'react';
import { Card, Button, Stack, Accordion } from 'react-bootstrap';

export const PostForm: React.FC = () => {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Handle post submission
  };

  return (
    <Card className="mb-3">
      <Card.Body>
        <Stack direction="horizontal" gap={2} className="mb-2">
          <Button variant="outline-success">▲</Button>
          <Button variant="outline-danger">▼</Button>
        </Stack>
        <form onSubmit={handleSubmit}>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Post title"
            className="w-full p-2 mb-2 border rounded"
            required
          />
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            placeholder="Post content"
            className="w-full p-2 mb-2 border rounded"
            rows={4}
            required
          />
          <Button type="submit" variant="primary">
            Create Post
          </Button>
        </form>
        <Accordion>
          <Accordion.Item eventKey="0">
            <Accordion.Header>Comments</Accordion.Header>
            <Accordion.Body>Comments will go here...</Accordion.Body>
          </Accordion.Item>
        </Accordion>
      </Card.Body>
    </Card>
  );
};
