/**
 * @file App.tsx
 *
 * @created 2026-02-01
 * @author Christian Blank <christianblank91@gmail.com>
 *
 * @copyright 2026 Christian Blank
 *
 * @description Main application component that displays VPN connection status
 */

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { StatusCard } from './components/StatusCard';
import { ConnectionDetails } from './components/ConnectionDetails';
import { cn } from './lib/utils';

/**
 * Represents the Mullvad VPN connection status response
 */
interface MullvadStatus {
  connected: boolean;
  ip?: string;
  country?: string;
  city?: string;
  hostname?: string;
  server_type?: string;
}

/**
 * Main application component
 * Listens for VPN status updates from the Rust backend and displays them
 */
function App() {
  const [status, setStatus] = useState<MullvadStatus | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Get initial status
    const fetchStatus = async () => {
      try {
        const result = await invoke<MullvadStatus>('get_vpn_status');
        setStatus(result);
      } catch (error) {
        console.error('Failed to fetch VPN status:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchStatus();

    // Listen for status updates from the backend
    const unlisten = listen<MullvadStatus>('vpn-status-changed', (event) => {
      setStatus(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  if (loading) {
    return (
      <div className="flex h-screen w-full items-center justify-center bg-background">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    );
  }

  return (
    <div className={cn('min-h-screen w-full p-4', 'bg-background')}>
      <div className="mx-auto max-w-md space-y-4">
        <StatusCard connected={status?.connected ?? false} />
        {status && <ConnectionDetails status={status} />}
      </div>
    </div>
  );
}

export default App;
