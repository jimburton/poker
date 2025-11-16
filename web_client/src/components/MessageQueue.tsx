/**
Component for displaying messages with feedback and status updates.
**/
import { useEffect, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";


// Example usage:
// <MessageQueue messages={messagesArray} />
// Push new messages to the parent-state array. This component fades them out automatically.


export default function MessageQueue ( { messages } ) {

  const [queue, setQueue] = useState([]);
  console.log(`MessageQueue messages:`);
  console.log(messages);

  useEffect(() => {
    if (!messages || messages.length === 0) return;

      // Add new messages to internal queue
      setQueue((prev) => {
        const newItems = messages.filter((m) => !prev.find((p) => p.id === m.id));
        return [...newItems, ...prev];
      });
  }, [messages]);


  useEffect(() => {
    // For each message, schedule removal
    const timers = queue.map((msg) => {
      return setTimeout(() => {
        setQueue((prev) => prev.filter((m) => m.id !== msg.id));
      }, msg.duration || 30000);
  });


  return () => timers.forEach((t) => clearTimeout(t));
  }, [queue]);


  return (
  
    <div className="d-flex flex-column mx-auto messages m-2">
        {queue.map((msg) => (
          <div key={msg.id} className="row ps-5">
          {msg.text}
         </div>
        ))}
    </div>
  );
}