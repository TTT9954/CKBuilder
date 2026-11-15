
"use client";

import { useState, type FormEvent } from "react";
import { ccc } from "@ckb-ccc/connector-react";

type TransferStatus =
  | "idle"
  | "sending"
  | "confirming"
  | "committed"
  | "pending"
  | "error";

export default function CKBTransfer() {
  const signer = ccc.useSigner();

  const [recipient, setRecipient] = useState("");
  const [amount, setAmount] = useState("");
  const [status, setStatus] = useState<TransferStatus>("idle");
  const [txHash, setTxHash] = useState("");
  const [message, setMessage] = useState("");

  const handleTransfer = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (!signer) {
      setStatus("error");
      setMessage("Please connect your JoyID wallet first.");
      return;
    }

    const toAddress = recipient.trim();
    const transferAmount = amount.trim();

    if (!/^(?:0|[1-9]\d*)(?:\.\d{1,8})?$/.test(transferAmount) ||
        Number(transferAmount) <= 0) {
      setStatus("error");
      setMessage("Enter a positive CKB amount with up to 8 decimal places.");
      return;
    }

    setStatus("sending");
    setTxHash("");
    setMessage("Preparing transaction...");

    try {
      const { script: toLock } = await ccc.Address.fromString(
        toAddress,
        signer.client
      );

      const tx = ccc.Transaction.from({
        outputs: [
          {
            lock: toLock,
            capacity: ccc.fixedPointFrom(transferAmount),
          },
        ],
      });

      await tx.completeInputsByCapacity(signer);
      await tx.completeFeeBy(signer);

      setMessage("Please approve the transaction in JoyID.");

      const hash = await signer.sendTransaction(tx);

      setTxHash(hash);
      setStatus("confirming");
      setMessage("Transaction submitted. Waiting for confirmation...");

      try {
        const confirmed = await signer.client.waitTransaction(hash, 1);

        if (confirmed) {
          setStatus("committed");
          setMessage(
            confirmed.blockNumber !== undefined
              ? `Transaction committed in block ${confirmed.blockNumber.toString()}.`
              : "Transaction committed successfully."
          );

          window.dispatchEvent(new Event("ckb-balance-refresh"));
        } else {
          setStatus("pending");
          setMessage(
            "Transaction submitted, but confirmation has not been observed yet."
          );
        }
      } catch {
        setStatus("pending");
        setMessage(
          "Transaction submitted. Confirmation checking failed; use the transaction hash to check its status."
        );
      }
    } catch (error) {
      setStatus("error");
      setMessage(
        error instanceof Error
          ? error.message
          : "Transaction failed."
      );
    }
  };

  return (
    <section className="w-full rounded-lg border border-gray-300 p-5 dark:border-gray-700">
      <h2 className="text-xl font-semibold">Send CKB</h2>

      {!signer && (
        <p className="mt-3 text-sm text-yellow-600">
          Connect your JoyID wallet to send CKB.
        </p>
      )}

      <form onSubmit={handleTransfer} className="mt-4 flex flex-col gap-4">
        <div>
          <label htmlFor="recipient" className="mb-1 block text-sm font-medium">
            Recipient CKB Address
          </label>
          <input
            id="recipient"
            type="text"
            value={recipient}
            onChange={(event) => setRecipient(event.target.value)}
            placeholder="Enter recipient CKB address"
            required
            disabled={!signer || status === "sending" || status === "confirming"}
            className="w-full rounded-md border border-gray-300 bg-transparent px-3 py-2 text-sm dark:border-gray-600"
          />
        </div>

        <div>
          <label htmlFor="amount" className="mb-1 block text-sm font-medium">
            Amount (CKB)
          </label>
          <input
            id="amount"
            type="number"
            min="0.00000001"
            step="0.00000001"
            value={amount}
            onChange={(event) => setAmount(event.target.value)}
            placeholder="Enter amount"
            required
            disabled={!signer || status === "sending" || status === "confirming"}
            className="w-full rounded-md border border-gray-300 bg-transparent px-3 py-2 text-sm dark:border-gray-600"
          />
        </div>

        <button
          type="submit"
          disabled={!signer || status === "sending" || status === "confirming"}
          className="rounded-md bg-blue-600 px-4 py-3 font-semibold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {status === "sending"
            ? "Preparing..."
            : status === "confirming"
              ? "Waiting for confirmation..."
              : "Send CKB"}
        </button>
      </form>

      {message && (
        <div
          role="status"
          className={`mt-4 rounded-md p-3 text-sm ${
            status === "error"
              ? "bg-red-100 text-red-700"
              : status === "committed"
                ? "bg-green-100 text-green-700"
                : "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-200"
          }`}
        >
          {message}
        </div>
      )}

      {txHash && (
        <div className="mt-4">
          <p className="text-sm font-semibold">Transaction Hash</p>
          <code className="mt-1 block break-all rounded-md bg-gray-100 p-3 text-xs text-gray-800 dark:bg-gray-800 dark:text-gray-200">
            {txHash}
          </code>
          <p className="mt-2 text-sm">
            Status: <strong>{status}</strong>
          </p>
        </div>
      )}
    </section>
  );
}
