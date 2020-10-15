import React from "react";

interface Props {
  className?: string;
}

const CrossIcon = ({ className }: Props) => {
  return (
    <div className={`icon-wrapper cross-icon-wrapper ${className || ""}`}>
      <svg
        width="16"
        height="16"
        viewBox="0 0 16 16"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M13.5937 15.2C13.1506 15.2 12.7075 15.0889 12.3752 14.7553L1.29827 3.63557C0.633651 2.96839 0.633651 1.85642 1.29827 1.30043C1.96288 0.633253 2.9598 0.633253 3.62442 1.30043L14.7013 12.4201C15.366 13.0873 15.366 14.0881 14.7013 14.7553C14.369 15.0889 14.0367 15.2 13.5937 15.2Z"
          fill="#DA291C"
        />
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M2.51673 15.2C2.07365 15.2 1.63057 15.0889 1.29827 14.7553C0.633651 14.0881 0.633651 13.0873 1.29827 12.4201L12.3752 1.30043C13.0398 0.633253 14.0367 0.633253 14.7013 1.30043C15.366 1.96762 15.366 2.96839 14.7013 3.63557L3.62442 14.7553C3.29211 15.0889 2.9598 15.2 2.51673 15.2Z"
          fill="#DA291C"
        />
      </svg>
    </div>
  );
};

export default CrossIcon;
