import React from "react";

interface Props {
  className?: string;
}

const CrossIcon = ({ className }: Props) => {
  return (
    <div className={`icon-wrapper cross-icon-wrapper ${className || ""}`}>
      <svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M17.5937 19.2C17.1506 19.2 16.7075 19.0889 16.3752 18.7553L5.29827 7.63557C4.63365 6.96839 4.63365 5.85642 5.29827 5.30043C5.96288 4.63325 6.9598 4.63325 7.62442 5.30043L18.7013 16.4201C19.366 17.0873 19.366 18.0881 18.7013 18.7553C18.369 19.0889 18.0367 19.2 17.5937 19.2Z"
          fill="#DA291C"
        />
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M6.51673 19.2C6.07365 19.2 5.63057 19.0889 5.29827 18.7553C4.63365 18.0881 4.63365 17.0873 5.29827 16.4201L16.3752 5.30043C17.0398 4.63325 18.0367 4.63325 18.7013 5.30043C19.366 5.96762 19.366 6.96839 18.7013 7.63557L7.62442 18.7553C7.29211 19.0889 6.9598 19.2 6.51673 19.2Z"
          fill="#DA291C"
        />
      </svg>
    </div>
  );
};

export default CrossIcon;
