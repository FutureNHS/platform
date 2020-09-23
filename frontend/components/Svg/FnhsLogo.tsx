import React from "react";

import styled from "styled-components";

import Svg from "./Svg";

const StyledSvg = styled(Svg)`
  display: block;
  // width: 138px;
  height: 40px;
  ${({ theme }) => `
    color: ${theme.colorNhsukWhite};

    @media (min-width: ${theme.mqBreakpoints.largeDesktop}) {
      color: ${theme.colorNhsukBlue};
    }
    @media (max-width: ${theme.mqBreakpoints.tablet}) {
      height: 32px;
    }
    @media (max-width: ${theme.mqBreakpoints.mobile}) {
      height: 28px;
    }
  `}
`;

const FnhsLogo = () => {
  return (
    <StyledSvg viewBox="0 0 138 29">
      <rect width="138" height="29" fill="none" />
      <mask
        id="mask0"
        mask-type="alpha"
        maskUnits="userSpaceOnUse"
        x="13"
        y="3"
        width="13"
        height="24"
      >
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M13.4502 3.00006H25.198V26.8382H13.4502V3.00006Z"
          fill="white"
        />
      </mask>
      <g mask="url(#mask0)">
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M13.464 26.8129C13.464 26.7985 15.4417 24.1256 17.8581 20.8726C20.2755 17.62 22.2526 14.9314 22.2526 14.8986C22.2526 14.8548 14.7228 4.67131 13.5923 3.18647L13.4502 3.00006L14.9364 3.00145L16.4222 3.00237L20.8366 8.94818C23.9031 13.0792 25.2343 14.9152 25.1974 14.9637C25.1683 15.002 23.1732 17.6897 20.7641 20.9358L16.3834 26.8382H14.924C14.1206 26.8382 13.464 26.8262 13.464 26.8119V26.8129Z"
          fill="#EC8A00"
        />
      </g>
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M7.35102 26.7676C7.75338 26.2665 16.1147 14.9595 16.1189 14.911C16.1221 14.8746 14.1994 12.2528 11.8457 9.08519C9.492 5.91756 7.51806 3.25288 7.45992 3.16337L7.35379 3.00049H8.81971H10.2856L14.67 8.90201C17.0814 12.1476 19.0544 14.8469 19.0544 14.9004C19.0544 14.9535 17.0772 17.6611 14.6603 20.9164L10.2658 26.8359L8.77957 26.8377C7.59834 26.8391 7.30534 26.8248 7.35102 26.7676Z"
        fill="#F0A133"
      />
      <mask
        id="mask1"
        mask-type="alpha"
        maskUnits="userSpaceOnUse"
        x="1"
        y="3"
        width="12"
        height="24"
      >
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M1 3H12.7391V26.8385H1V3Z"
          fill="white"
        />
      </mask>
      <g mask="url(#mask1)">
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M1.01406 26.7918C1.02421 26.766 3.00415 24.093 5.41321 20.8525C7.82226 17.6115 9.79297 14.932 9.79297 14.8984C9.79297 14.8647 7.95377 12.3615 5.70528 9.33507C3.45634 6.30864 1.47824 3.64534 1.30844 3.41648L0.999756 2.99982L2.48136 3.0012L3.96297 3.00259L8.37735 8.9484C11.4624 13.1035 12.7756 14.915 12.7382 14.9639C12.7091 15.0026 10.714 17.6899 8.30537 20.9365L3.9256 26.8384H2.46014C1.65451 26.8384 1.00345 26.8177 1.01406 26.7918Z"
          fill="#F3B55C"
        />
      </g>
      <path
        d="M28.2 22.7909H29.8531V14.9033H35.8667V13.4648H29.8531V7.49518H36.1542V6.0567H28.2V22.7909Z"
        fill="currentColor"
      />
      <path
        d="M48.4291 10.5639H46.9197V17.3247C46.9197 17.8362 46.8638 18.3477 46.752 18.8591C46.6402 19.3706 46.4405 19.8501 46.153 20.2976C45.8974 20.7291 45.5301 21.0808 45.0509 21.3525C44.5717 21.6402 43.9887 21.784 43.3019 21.784C42.2797 21.784 41.545 21.4324 41.0977 20.7291C40.6345 20.0738 40.4029 19.1628 40.4029 17.996V10.5639H38.8935V18.3796C38.8776 19.7542 39.197 20.873 39.8519 21.7361C40.5068 22.6151 41.5769 23.0627 43.0623 23.0786C43.7012 23.0786 44.2443 22.9907 44.6915 22.8149C45.1388 22.6711 45.5141 22.4793 45.8176 22.2395C46.1211 21.9838 46.3686 21.7121 46.5603 21.4244C46.752 21.1527 46.9037 20.881 47.0155 20.6093H47.0634V22.7909H48.5249C48.445 21.8 48.4131 20.841 48.4291 19.914V10.5639Z"
        fill="currentColor"
      />
      <path
        d="M57.4774 10.5639H54.6982V7.08761L53.1888 7.61505V10.5639H50.7929V11.8585H53.1888V19.003C53.1888 19.5624 53.2127 20.0818 53.2607 20.5613C53.3086 21.0408 53.4124 21.4723 53.5721 21.8559C53.7638 22.2236 54.0433 22.5192 54.4107 22.743C54.794 22.9668 55.3131 23.0786 55.968 23.0786C56.3673 23.0786 56.7347 23.0387 57.0701 22.9588C57.3895 22.8789 57.6371 22.8069 57.8128 22.743L57.717 21.4963C57.3336 21.6881 56.8704 21.784 56.3274 21.784C55.7364 21.784 55.3211 21.5842 55.0815 21.1847C54.826 20.8011 54.6982 20.3296 54.6982 19.7702V11.8585H57.4774V10.5639Z"
        fill="currentColor"
      />
      <path
        d="M69.7204 10.5639H68.211V17.3247C68.211 17.8362 68.1551 18.3477 68.0433 18.8591C67.9315 19.3706 67.7319 19.8501 67.4444 20.2976C67.1888 20.7291 66.8214 21.0808 66.3423 21.3525C65.8631 21.6402 65.2801 21.784 64.5933 21.784C63.571 21.784 62.8363 21.4324 62.3891 20.7291C61.9259 20.0738 61.6943 19.1628 61.6943 17.996V10.5639H60.1849V18.3796C60.1689 19.7542 60.4884 20.873 61.1432 21.7361C61.7981 22.6151 62.8683 23.0627 64.3537 23.0786C64.9926 23.0786 65.5356 22.9907 65.9829 22.8149C66.4301 22.6711 66.8055 22.4793 67.1089 22.2395C67.4124 21.9838 67.66 21.7121 67.8516 21.4244C68.0433 21.1527 68.1951 20.881 68.3069 20.6093H68.3548V22.7909H69.8163C69.7364 21.8 69.7044 20.841 69.7204 19.914V10.5639Z"
        fill="currentColor"
      />
      <path
        d="M73.4978 22.7909H75.0072V16.7494C75.0072 16.19 75.0631 15.6146 75.1749 15.0232C75.2708 14.4638 75.4305 13.9443 75.6541 13.4648C75.8937 12.9694 76.2052 12.5778 76.5885 12.2901C76.9718 12.0184 77.459 11.8745 78.05 11.8585C78.4493 11.8585 78.8166 11.9065 79.1521 12.0024V10.4201C78.8486 10.3402 78.4972 10.2922 78.0979 10.2762C77.3312 10.2922 76.6923 10.5559 76.1812 11.0674C75.6541 11.5788 75.2468 12.1942 74.9593 12.9134H74.9114V10.5639H73.402C73.4659 11.2512 73.4978 12.1622 73.4978 13.297V22.7909Z"
        fill="currentColor"
      />
      <path
        d="M89.6701 20.873C89.2867 21.0967 88.7676 21.2965 88.1128 21.4723C87.4579 21.6801 86.835 21.784 86.244 21.784C84.9183 21.768 83.92 21.3045 83.2492 20.3935C82.5624 19.5304 82.219 18.4356 82.219 17.109H90.5805V16.3418C90.5805 14.6476 90.2052 13.2251 89.4545 12.0743C88.6718 10.9076 87.426 10.3082 85.7169 10.2762C84.1516 10.2922 82.9138 10.8836 82.0033 12.0503C81.061 13.2331 80.5818 14.7755 80.5658 16.6774C80.5498 18.5475 80.9731 20.0658 81.8356 21.2326C82.6981 22.4313 84.1277 23.0467 86.1242 23.0786C87.3381 23.0627 88.5201 22.8389 89.6701 22.4074V20.873ZM82.219 15.8144C82.219 14.7435 82.5464 13.7765 83.2013 12.9134C83.8402 12.0503 84.7107 11.6028 85.8127 11.5709C86.9308 11.6028 87.7374 12.0424 88.2326 12.8895C88.6958 13.7206 88.9274 14.6955 88.9274 15.8144H82.219Z"
        fill="currentColor"
      />
      <path
        d="M106.631 22.7909H102.747L97.5057 13.1332C97.1703 12.5245 96.7297 11.5802 96.1837 10.3003C96.4255 11.9626 96.5464 13.3361 96.5464 14.4209V22.7909H93.4814V6.8001H97.2834L102.653 16.4812C102.84 16.8246 102.961 17.0705 103.016 17.2187C103.078 17.367 103.191 17.648 103.355 18.0616C103.519 18.4752 103.686 18.8927 103.858 19.3142C103.663 18.1982 103.566 16.8949 103.566 15.4043V6.8001H106.631V22.7909Z"
        fill="currentColor"
      />
      <path
        d="M123.289 22.7909H120.072V15.9076H113.696V22.7909H110.468V6.8001H113.696V13.2971H120.072V6.8001H123.289V22.7909Z"
        fill="currentColor"
      />
      <path
        d="M126.881 22.4281L127.173 19.56C128.437 20.2468 129.642 20.5902 130.788 20.5902C131.529 20.5902 132.176 20.4068 132.73 20.04C133.292 19.6732 133.572 19.1347 133.572 18.4245C133.572 17.7065 133.334 17.1953 132.859 16.891C132.391 16.5866 131.537 16.1691 130.297 15.6384C129.057 15.0999 128.125 14.5029 127.501 13.8473C126.885 13.184 126.577 12.2396 126.577 11.0144C126.577 9.99203 126.834 9.14137 127.349 8.4624C127.871 7.77563 128.558 7.27226 129.408 6.95228C130.265 6.62451 131.197 6.46062 132.204 6.46062C133.553 6.46062 134.824 6.67133 136.017 7.09276L135.748 9.72668C134.586 9.196 133.483 8.93065 132.438 8.93065C131.751 8.93065 131.151 9.08674 130.636 9.39891C130.129 9.70327 129.876 10.1871 129.876 10.8505C129.876 11.4826 130.067 11.9158 130.449 12.1499C130.831 12.384 131.697 12.8015 133.046 13.4025C134.403 13.9956 135.401 14.6316 136.041 15.3106C136.68 15.9818 137 16.9261 137 18.1435C137 19.0098 136.84 19.759 136.52 20.3912C136.201 21.0233 135.756 21.5462 135.187 21.9598C134.625 22.3656 133.978 22.6661 133.245 22.8612C132.519 23.0563 131.747 23.1538 130.928 23.1538C129.634 23.1538 128.285 22.9119 126.881 22.4281Z"
        fill="currentColor"
      />
    </StyledSvg>
  );
};

export default FnhsLogo;