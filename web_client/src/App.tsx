import { FormEvent, useEffect, useState } from "react";

function App() {
  const [messages, setMessages] = useState<string[]>([]);
  const [socket, setSocket] = useState<WebSocket | undefined>(undefined);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:3000/");
    socket.onmessage = (e: MessageEvent<string>) =>
      setMessages((prev) => [...prev, e.data]);
    setSocket(socket);
    return () => socket.close();
  }, []);

  const submit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!socket) return;
    const form = e.target as typeof e.target & {
      input: { value: string };
    };
    socket.send(form.input.value);
    form.input.value = "";
  };

  return (
    <>
      <h1>WebSocket Chat App</h1>
      <ul>
        {messages.map((body, index) => (
          <li key={index}>{body}</li>
        ))}
      </ul>
      <form onSubmit={submit}>
        <input type="text" name="input" />
        <button type="submit">Send</button>
      </form>
    </>
  );
}
export default App
