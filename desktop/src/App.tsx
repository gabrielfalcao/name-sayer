import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import Container from "react-bootstrap/Container";
import Alert from 'react-bootstrap/Alert';

import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';
import Modal from 'react-bootstrap/Modal';

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState(null);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <Container>
      <Row>
        <Col>
          <Modal.Dialog>
            <Modal.Header closeButton onHide={() => {
              setGreetMsg(null)
            }}>
              <Modal.Title>Greet</Modal.Title>
            </Modal.Header>

            <Modal.Body>
              <form
                className="row"
                onSubmit={(e) => {
                  e.preventDefault();
                  greet();
                }}
              >
                <input
                  id="greet-input"
                  onChange={(e) => setName(e.currentTarget.value)}
                  placeholder="Enter a name..."
                />
                <Button type="submit">Greet</Button>
              </form>

            </Modal.Body>

            {greetMsg &&
            <Modal.Footer>
              <Alert key="light" variant="light">
                {greetMsg}
              </Alert>
            </Modal.Footer>
            }
          </Modal.Dialog>

        </Col>
      </Row>
    </Container>
  );
}

export default App;
