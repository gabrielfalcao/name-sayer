import { useState, useEffect } from "react";
import { invoke, Channel } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import Container from "react-bootstrap/Container";
import Alert from "react-bootstrap/Alert";

import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import Button from "react-bootstrap/Button";
import Modal from "react-bootstrap/Modal";

type SelectedFile = string;

function App() {
  const [file, setFile] = useState("");
  const onEvent = new Channel<SelectedFile>();
  onEvent.onmessage = (message) => {
    setFile(message.event);
  };
  async function openFile() {
    await invoke<string | null>("open_file");
  }
  useEffect(() => {
  listen<string>("file-selected", (event) => {
    setFile(event.payload);
  });
  })
  return (
    <Container>
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
