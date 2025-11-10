import { useState, useRef, useEffect } from "react";
import { ChevronDown } from "lucide-react";

interface CustomSelectProps {
  value: string;
  onChange: (value: string) => void;
  options: string[];
  disabled?: boolean;
  placeholder?: string;
}

export function CustomSelect({
  value,
  onChange,
  options,
  disabled = false,
  placeholder = "Select an option",
}: CustomSelectProps) {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleSelect = (option: string) => {
    onChange(option);
    setIsOpen(false);
  };

  // Calculate minimum width based on longest option
  const longestOption = options.reduce((longest, current) => {
    return current.length > longest.length ? current : longest;
  }, placeholder);

  // Approximate width: ~7px per character + padding (32px) + icon space (36px)
  const minWidth = Math.max(longestOption.length * 7 + 68, 110);

  const displayValue = value ? value.toUpperCase() : placeholder;

  return (
    <div
      ref={dropdownRef}
      className="relative"
      style={{ minWidth: `${minWidth}px` }}
    >
      <button
        type="button"
        onClick={() => !disabled && setIsOpen(!isOpen)}
        disabled={disabled}
        className={`btn-chunky px-4 py-2 pr-10 w-full text-center ${
          disabled
            ? "bg-lighter-bg border-2 border-secondary text-secondary cursor-not-allowed"
            : "bg-white border-2 border-dark-purple text-dark-purple cursor-pointer hover:bg-light-bg"
        }`}
      >
        <span>{displayValue}</span>
        <ChevronDown
          className={`absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 pointer-events-none transition-transform ${
            isOpen ? "rotate-180" : ""
          } ${disabled ? "text-secondary" : "text-dark-purple"}`}
        />
      </button>

      {isOpen && !disabled && (
        <div className="absolute z-50 mt-2 w-full bg-white border-2 border-dark-purple rounded-xl shadow-chunky-hover max-h-80 overflow-hidden">
          <div className="overflow-y-auto max-h-80 p-2">
            {options.map((option, index) => (
              <button
                key={option}
                type="button"
                onClick={() => handleSelect(option)}
                className={`w-full px-4 py-2 text-center font-bold hover:bg-light-bg transition-colors whitespace-nowrap ${
                  index === 0 ? "rounded-t-lg" : ""
                } ${index === options.length - 1 ? "rounded-b-lg" : ""} ${
                  value === option ? "text-dark-purple" : "text-secondary"
                }`}
              >
                {option.toUpperCase()}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
