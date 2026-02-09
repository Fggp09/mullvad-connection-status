/**
 * @file StatusCard.tsx
 *
 * @created 2026-02-01
 * @author Christian Blank <christianblank91@gmail.com>
 *
 * @copyright 2026 Christian Blank
 *
 * @description Main status card component showing VPN connection state
 */

import { Shield, ShieldAlert } from 'lucide-react';
import { Card, CardContent } from './ui/card';
import { Badge } from './ui/badge';
import { cn } from '@/lib/utils';

interface StatusCardProps {
  connected: boolean;
}

/**
 * Displays a prominent card showing whether the VPN is connected
 * Features a shield icon and color-coded badge
 *
 * @param connected - Whether the VPN is currently connected
 */
export function StatusCard({ connected }: StatusCardProps) {
  return (
    <Card className="border-2">
      <CardContent className="pt-6">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            {connected ? (
              <Shield className={cn('h-12 w-12 text-green-500')} strokeWidth={2} />
            ) : (
              <ShieldAlert className={cn('h-12 w-12 text-destructive')} strokeWidth={2} />
            )}
            <div>
              <h2 className="text-2xl font-bold">{connected ? 'Connected' : 'Disconnected'}</h2>
              <p className="text-sm text-muted-foreground">
                {connected ? 'Your traffic is protected' : 'No VPN connection detected'}
              </p>
            </div>
          </div>
          <Badge variant={connected ? 'success' : 'destructive'}>
            {connected ? 'ACTIVE' : 'INACTIVE'}
          </Badge>
        </div>
      </CardContent>
    </Card>
  );
}
