import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Container from "react-bootstrap/Container";
import Alert from "react-bootstrap/Alert";

import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Button from "react-bootstrap/Button";
import Modal from "react-bootstrap/Modal";

function App() {
  const [greetMsg, setGreetMsg] = useState<string | null>("");
  const [name, setName] = useState<string | null>(null);
  const [file, setFile] = useState<string | null>(null);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }
  async function openFile() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await invoke("open_file");
  }

  return (
    <Container>
      <Row>
        <Col>
          <Modal.Dialog>
            <Modal.Header
              closeButton
              onHide={() => {
                setGreetMsg(null);
              }}
            >
              <Modal.Title>Greet</Modal.Title>
            </Modal.Header>

            <Modal.Body>
              <input
                id="greet-input"
                onChange={(e) => setName(e.currentTarget.value)}
                placeholder="Enter a name..."
              />
              <Button
                onClick={(e) => {
                  e.preventDefault();
                  greet();
                }}
                type="button"
              >
                Greet
              </Button>
              <Button
                variant="primary"
                onClick={(e) => {
                  e.preventDefault();
                  openFile();
                }}
                type="button"
              >
                Open File
              </Button>
            </Modal.Body>

            {greetMsg && (
              <Modal.Footer>
                <Alert key="light" variant="light">
                  {greetMsg}
                </Alert>
              </Modal.Footer>
            )}
          </Modal.Dialog>
        </Col>
      </Row>
    </Container>
  );
}

export default App;
