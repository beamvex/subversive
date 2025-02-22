import { Card, ListGroup } from 'react-bootstrap';

const SideBar = () => {
  return (
    <Card className="mb-3">
      <Card.Body>
        <Card.Title>Communities</Card.Title>
        <ListGroup variant="flush">
          <ListGroup.Item action href="/r/reactjs">
            ReactJS
          </ListGroup.Item>
          <ListGroup.Item action href="/r/webdev">
            Web Development
          </ListGroup.Item>
          <ListGroup.Item action href="/r/programming">
            Programming
          </ListGroup.Item>
        </ListGroup>
      </Card.Body>
    </Card>
  );
};

export default SideBar;
