import React from "react";

interface Props {
  className?: string;
}

const TickIcon = ({ className }: Props) => (
  <div className={`icon-wrapper tick-icon wrapper ${className || ""}`}>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M16.9943 5.38782C17.7799 4.59752 19.0462 4.60515 19.8227 5.40486C20.5582 6.16248 20.5902 7.35922 19.9235 8.15506L19.8059 8.28385L9.9083 18.24L4.20262 12.5673C3.41244 11.7817 3.39754 10.4928 4.16935 9.6885C4.90053 8.92651 6.07548 8.8728 6.869 9.53724L6.99758 9.65463L9.8911 12.532L16.9943 5.38782Z"
        fill="#007F3B"
      />
    </svg>
  </div>
);

export default TickIcon;
