export function VenusIcon({ className }: { className?: string }) {
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
      <circle cx="12" cy="8" r="5" />
      {/* Vertical line (cross stem) */}
      <line x1="12" y1="13" x2="12" y2="21" />
      {/* Horizontal line (cross bar) */}
      <line x1="9" y1="18" x2="15" y2="18" />
    </svg>
  );
}
