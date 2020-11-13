import { Input, Button } from "nhsuk-react-components";
import styled from "styled-components";

export const StyledInput = styled(Input)`
  border: none;
  margin-bottom: 16px;
`;

export const StyledTag = styled.p`
  margin-bottom: 40px;
`;

export const StyledButton = styled(Button)`
  margin-left: 12px;
`;

export const StyledFileInfoBox = styled.div`
  ${({ theme }) => `
background-color: ${theme.colorNhsukGrey5};
`}
  margin-bottom: 24px;
`;

export const FormField = styled.div`
  padding: 16px 20px;

  #text {
    padding-bottom: 60px;
  }
`;

export const FormFieldNoSidePadding = styled.div`
  padding-top: 16px;
  padding-bottom: 16px;
`;

export const StyledHeadingSection = styled.div`
  display: flex;
`;

export const StyledFileName = styled.h4`
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: 0px;
`;
