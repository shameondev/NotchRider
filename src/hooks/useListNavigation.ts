import { useState, useCallback } from 'react';
import { useKeyboard } from './useKeyboard';

interface UseListNavigationOptions<T> {
  items: T[];
  onSelect: (item: T, index: number) => void;
  onCancel?: () => void;
}

export function useListNavigation<T>({ items, onSelect, onCancel }: UseListNavigationOptions<T>) {
  const [selectedIndex, setSelectedIndex] = useState(0);

  const moveUp = useCallback(() => {
    setSelectedIndex(i => Math.max(0, i - 1));
  }, []);

  const moveDown = useCallback(() => {
    setSelectedIndex(i => Math.min(items.length - 1, i + 1));
  }, [items.length]);

  const select = useCallback(() => {
    if (items[selectedIndex]) {
      onSelect(items[selectedIndex], selectedIndex);
    }
  }, [items, selectedIndex, onSelect]);

  useKeyboard({
    'ArrowUp': moveUp,
    'ArrowDown': moveDown,
    'Enter': select,
    'Escape': onCancel ?? (() => {}),
  });

  return { selectedIndex, setSelectedIndex };
}
