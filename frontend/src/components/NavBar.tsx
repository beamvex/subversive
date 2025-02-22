import { Navbar, Container, Nav, Form, Image, NavDropdown } from 'react-bootstrap';

const NavBar = () => {
  return (
    <Navbar bg="dark" variant="dark" expand="lg">
      <Container fluid> 
        <Navbar.Brand href="/">Subversive</Navbar.Brand>
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
        <Navbar.Collapse id="basic-navbar-nav">
          <Form className="d-flex flex-grow-1 mx-4">
            <Form.Control
              type="search"
              placeholder="Search"
              className="w-100"
              aria-label="Search"
            />
          </Form>
          <Nav>
            <Nav.Link href="/inbox" className="me-2 d-flex align-items-center">
              <svg 
                width="24" 
                height="24" 
                viewBox="0 0 16 16" 
                className="bi bi-inbox"
                fill="currentColor"
              >
                <path d="M4.98 4a.5.5 0 0 0-.39.188L1.54 8H6a.5.5 0 0 1 .5.5 1.5 1.5 0 1 0 3 0A.5.5 0 0 1 10 8h4.46l-3.05-3.812A.5.5 0 0 0 11.02 4H4.98zm-1.17-.437A1.5 1.5 0 0 1 4.98 3h6.04a1.5 1.5 0 0 1 1.17.563l3.7 4.625a.5.5 0 0 1 .106.374l-.39 3.124A1.5 1.5 0 0 1 14.117 13H1.883a1.5 1.5 0 0 1-1.489-1.314l-.39-3.124a.5.5 0 0 1 .106-.374l3.7-4.625z"/>
              </svg>
            </Nav.Link>
            <NavDropdown 
              title={
                <Image
                  src="https://ui-avatars.com/api/?name=User"
                  roundedCircle
                  width={32}
                  height={32}
                  className="d-inline-block align-middle"
                  alt="User avatar"
                />
              }
              id="basic-nav-dropdown"
              align="end"
            >
              <NavDropdown.Item href="/profile">Profile</NavDropdown.Item>
              <NavDropdown.Item href="/settings">Settings</NavDropdown.Item>
              <NavDropdown.Divider />
              <NavDropdown.Item href="/logout">Logout</NavDropdown.Item>
            </NavDropdown>
          </Nav>
        </Navbar.Collapse>
      </Container>
    </Navbar>
  );
};

export default NavBar;
