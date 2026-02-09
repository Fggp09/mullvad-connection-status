/**
 * @file useDarkMode.tsx
 *
 * @created 2026-02-09
 * @author Christian Blank <christianblank91@gmail.com>
 *
 * @copyright 2026 Christian Blank
 *
 * @description Hook for managing dark mode state with localStorage persistence
 */

import { useEffect, useState } from 'react';

export function useDarkMode() {
  const [isDark, setIsDark] = useState<boolean>(() => {
    const saved = localStorage.getItem('darkMode');
    return saved === 'true';
  });

  useEffect(() => {
    const root = document.documentElement;
    if (isDark) {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
    localStorage.setItem('darkMode', String(isDark));
  }, [isDark]);

  const toggleDarkMode = () => setIsDark(!isDark);

  return { isDark, toggleDarkMode };
}
