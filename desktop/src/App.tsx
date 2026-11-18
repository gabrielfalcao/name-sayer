import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Container from "react-bootstrap/Container";
import Alert from "react-bootstrap/Alert";

import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Button from "react-bootstrap/Button";
import Modal from "react-bootstrap/Modal";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [file, setFile] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }
  async function openFile() {
    const selectedPath = await invoke<string | null>("open_file");
    if (selectedPath) {
      console.log("Selected file:", selectedPath);
      setFile(selectedPath)
    }
  }

  return (
    <Container>
      <Row>
        <Col>
          <Modal.Dialog>
            <Modal.Header
              closeButton
              onHide={() => {
                setGreetMsg("");
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
            </Modal.Body>

            {greetMsg?.length > 0 && (
              <Modal.Footer>
                <Alert key="light" variant="light">
                  {greetMsg}
                </Alert>
              </Modal.Footer>
            )}
          </Modal.Dialog>
        </Col>
      </Row>
      <Row>
        <Col>
        <br />
        <hr />
        <br />
        </Col>
      </Row>
      <Row>
        <Col>
          <Modal.Dialog>
            <Modal.Header
              closeButton
              onHide={() => {
                setFile("");
              }}
            >
              <Modal.Title>Open File</Modal.Title>
            </Modal.Header>

            <Modal.Body>
              <Button
                variant="danger"
                onClick={(e) => {
                  e.preventDefault();
                  openFile();
                }}
                type="button"
              >
                Open File
              </Button>

            </Modal.Body>

            {file?.length > 0 && (
              <Modal.Footer>
                <Alert key="light" variant="light">
                  {file}
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
