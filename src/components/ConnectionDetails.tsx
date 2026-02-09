/**
 * @file ConnectionDetails.tsx
 *
 * @created 2026-02-01
 * @author Christian Blank <christianblank91@gmail.com>
 *
 * @copyright 2026 Christian Blank
 *
 * @description Component displaying detailed VPN connection information
 */

import { Globe, MapPin, Server, Network } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from './ui/card';

interface MullvadStatus {
  connected: boolean;
  ip?: string;
  country?: string;
  city?: string;
  hostname?: string;
  server_type?: string;
}

interface ConnectionDetailsProps {
  status: MullvadStatus;
}

/**
 * Displays detailed information about the current VPN connection
 * Shows IP address, location, server hostname, and connection type
 *
 * @param status - The current VPN status including connection details
 */
export function ConnectionDetails({ status }: ConnectionDetailsProps) {
  if (!status.connected) {
    return null;
  }

  const details = [
    {
      icon: Globe,
      label: 'IP Address',
      value: status.ip || 'Unknown',
    },
    {
      icon: MapPin,
      label: 'Location',
      value:
        status.city && status.country
          ? `${status.city}, ${status.country}`
          : status.country || 'Unknown',
    },
    {
      icon: Server,
      label: 'Server',
      value: status.hostname || 'Unknown',
    },
    {
      icon: Network,
      label: 'Protocol',
      value: status.server_type?.toUpperCase() || 'Unknown',
    },
  ];

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">Connection Details</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {details.map((detail) => (
          <div key={detail.label} className="flex items-start gap-3">
            <detail.icon className="h-5 w-5 text-muted-foreground mt-0.5" />
            <div className="flex-1 space-y-1">
              <p className="text-sm font-medium leading-none">{detail.label}</p>
              <p className="text-sm text-muted-foreground">{detail.value}</p>
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
