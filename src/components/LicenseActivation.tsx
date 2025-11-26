import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface LicenseStatus {
  isValid: boolean;
  isActivated: boolean;
  planType: "monthly" | "yearly" | "lifetime" | null;
  daysRemaining: number | null;
  inGracePeriod: boolean;
  error: string | null;
  requiresActivation: boolean;
}

interface LicenseActivationProps {
  onActivated: (status: LicenseStatus) => void;
  initialError?: string;
}

export function LicenseActivation({
  onActivated,
  initialError,
}: LicenseActivationProps) {
  const [productKey, setProductKey] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(initialError || "");

  // Format product key as user types (XXXXX-XXXXX-XXXXX-XXXXX)
  const handleKeyChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let value = e.target.value.toUpperCase().replace(/[^A-Z0-9]/g, "");

    // Add dashes every 5 characters
    const parts = [];
    for (let i = 0; i < value.length && i < 20; i += 5) {
      parts.push(value.slice(i, i + 5));
    }
    setProductKey(parts.join("-"));
    setError("");
  };

  const handleActivate = async () => {
    if (!productKey || productKey.replace(/-/g, "").length !== 20) {
      setError("Please enter a valid product key");
      return;
    }

    setIsLoading(true);
    setError("");

    try {
      // Get device name (hostname)
      let deviceName = "Unknown Device";
      try {
        deviceName = await invoke<string>("get_device_id");
        deviceName = deviceName.slice(0, 20); // Truncate for display
      } catch {
        // Use default
      }

      const status = await invoke<LicenseStatus>("activate_license", {
        productKey: productKey,
        deviceName: deviceName,
      });

      if (status.isValid) {
        onActivated(status);
      } else {
        setError(status.error || "Activation failed");
      }
    } catch (err: any) {
      console.error("Activation error:", err);
      setError(err.toString() || "Failed to activate license");
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !isLoading) {
      handleActivate();
    }
  };

  return (
    <div className="h-screen bg-light-bg flex items-center justify-center p-8">
      <div className="max-w-md w-full">
        {/* Logo / Header */}
        <div className="text-center mb-8">
          <img
            src="/logo.png"
            alt="ConvertSave"
            className="w-20 h-20 mx-auto mb-4"
          />
          <h1 className="text-3xl font-bold text-dark-purple mb-2">
            Welcome to ConvertSave
          </h1>
          <p className="text-secondary">
            Enter your product key to activate the app
          </p>
        </div>

        {/* Activation Form */}
        <div className="bg-white rounded-2xl border-2 border-dark-purple p-6 space-y-6">
          <div className="space-y-2">
            <label
              htmlFor="product-key"
              className="block text-sm font-bold text-dark-purple"
            >
              Product Key
            </label>
            <input
              id="product-key"
              type="text"
              value={productKey}
              onChange={handleKeyChange}
              onKeyDown={handleKeyDown}
              placeholder="XXXXX-XXXXX-XXXXX-XXXXX"
              className="w-full px-4 py-3 text-lg font-mono tracking-wider border-2 border-dark-purple rounded-xl focus:outline-none focus:ring-2 focus:ring-mint-accent text-center uppercase"
              disabled={isLoading}
              maxLength={23}
              autoFocus
            />
            <p className="text-xs text-secondary text-center">
              You can find your product key in your purchase confirmation email
            </p>
          </div>

          {error && (
            <div className="bg-pink-accent text-dark-purple px-4 py-3 rounded-xl text-sm font-medium">
              {error}
            </div>
          )}

          <button
            onClick={handleActivate}
            disabled={isLoading || productKey.replace(/-/g, "").length !== 20}
            className={`w-full py-3 rounded-xl font-bold text-lg border-2 border-dark-purple transition-all ${
              isLoading || productKey.replace(/-/g, "").length !== 20
                ? "bg-lighter-bg text-secondary cursor-not-allowed"
                : "bg-mint-accent text-dark-purple hover:bg-opacity-80"
            }`}
          >
            {isLoading ? (
              <span className="flex items-center justify-center gap-2">
                <svg
                  className="animate-spin h-5 w-5"
                  viewBox="0 0 24 24"
                  fill="none"
                >
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  />
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
                  />
                </svg>
                Activating...
              </span>
            ) : (
              "Activate"
            )}
          </button>
        </div>

        {/* Help Links */}
        <div className="mt-6 text-center space-y-2">
          <p className="text-sm text-secondary">
            Don't have a product key?{" "}
            <a
              href="https://convertsave.com/#pricing"
              target="_blank"
              rel="noopener noreferrer"
              className="text-dark-purple font-bold hover:underline"
            >
              Purchase a license
            </a>
          </p>
          <p className="text-sm text-secondary">
            Need help?{" "}
            <a
              href="mailto:support@convertsave.com"
              className="text-dark-purple font-bold hover:underline"
            >
              Contact support
            </a>
          </p>
        </div>
      </div>
    </div>
  );
}

export default LicenseActivation;
