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
        d="M13.9943 1.38782C14.7799 0.597519 16.0462 0.605146 16.8227 1.40486C17.5582 2.16248 17.5902 3.35922 16.9235 4.15506L16.8059 4.28385L6.9083 14.24L1.20262 8.56734C0.412437 7.78173 0.39754 6.49283 1.16935 5.6885C1.90053 4.92651 3.07548 4.8728 3.869 5.53724L3.99758 5.65463L6.8911 8.53197L13.9943 1.38782Z"
        fill="#007F3B"
      />
    </svg>
  </div>
);

export default TickIcon;
