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
        return [...prev, ...newItems];
      });
  }, [messages]);


  useEffect(() => {
    // For each message, schedule removal
    const timers = queue.map((msg) => {
    return setTimeout(() => {
      setQueue((prev) => prev.filter((m) => m.id !== msg.id));
    }, msg.duration || 3000);
  });


  return () => timers.forEach((t) => clearTimeout(t));
  }, [queue]);


  return (
  
    <div className="fixed top-4 right-4 space-y-2 w-64 z-50">
      <AnimatePresence>
        {queue.map((msg) => (
          <motion.div
            key={msg.id}
            initial={{ opacity: 1, y: 0 }}
            animate={{ opacity: 0, y: -10 }}
            exit={{ opacity: 0, y: -10 }}
            transition={{ duration: msg.duration }}
            className="rounded-2xl shadow-lg p-4 bg-white text-gray-800 border
	               border-gray-200">
          {msg.text}
         </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}