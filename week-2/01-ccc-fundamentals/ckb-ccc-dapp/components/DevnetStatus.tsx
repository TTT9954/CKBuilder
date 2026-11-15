
"use client";

import { useCallback, useEffect, useState } from "react";
import { ccc } from "@ckb-ccc/connector-react";

export default function DevnetStatus() {
  const { client } = ccc.useCcc();
  const [blockNumber, setBlockNumber] = useState<string>("Loading...");
  const [error, setError] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);

  const loadTip = useCallback(async () => {
    setLoading(true);

    try {
      const tip = await client.getTip();
      setBlockNumber(tip.toString());
      setError("");
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to query Devnet"
      );
    } finally {
      setLoading(false);
    }
  }, [client]);

  useEffect(() => {
    loadTip();
  }, [loadTip]);

  return (
    <div className="w-full rounded-lg border border-gray-300 p-4 dark:border-gray-700">
      <h2 className="text-lg font-semibold">CKB Testnet</h2>

      {error ? (
        <p className="mt-2 text-red-500">Connection error: {error}</p>
      ) : (
        <p className="mt-2">
          Current block height: <strong>{blockNumber}</strong>
        </p>
      )}

      <p className="mt-2 text-sm text-gray-500">
        RPC: {client.url}
      </p>

      <button
        onClick={loadTip}
        disabled={loading}
        className="mt-4 rounded-md bg-blue-600 px-4 py-2 text-white disabled:opacity-50"
      >
        {loading ? "Refreshing..." : "Refresh Block Height"}
      </button>
    </div>
  );
}
