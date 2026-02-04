import { useEffect } from 'react';

type KeyHandler = () => void;
type KeyBindings = Record<string, KeyHandler>;

export function useKeyboard(bindings: KeyBindings): void {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const handler = bindings[event.key];
      if (handler) {
        event.preventDefault();
        handler();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [bindings]);
}
