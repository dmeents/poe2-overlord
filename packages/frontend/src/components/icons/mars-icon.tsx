export function MarsIcon({ className }: { className?: string }) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={className}
      aria-hidden="true"
    >
      {/* Circle */}
      <circle cx="10" cy="14" r="6" />
      {/* Arrow shaft */}
      <line x1="14" y1="10" x2="20" y2="4" />
      {/* Arrow head horizontal */}
      <line x1="15" y1="4" x2="20" y2="4" />
      {/* Arrow head vertical */}
      <line x1="20" y1="4" x2="20" y2="9" />
    </svg>
  );
}
