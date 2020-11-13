import React, { ReactNode, useState } from "react";

import classNames from "classnames";
import { Table as NHSTable } from "nhsuk-react-components";
import styled from "styled-components";

import { Expander } from "../Expander";

const TableContainer = styled.div`
  display: none;
  width: 100%;
  ${({ theme }) => `
  @media (min-width: ${theme.mqBreakpoints.tablet}) {
      display: block;
    }
  `}
  > div {
    ${({ theme }) => `
      background: ${theme.colorNhsukGrey5};
  `}
  }
`;

const IconWrapper = styled.div`
  display: flex;
  .file-icon-wrapper {
    flex-shrink: 0;
    padding-right: 8px;
  }
`;

const StyledTable = styled(NHSTable)`
  tbody tr:hover:not(&.withoutBottomBorder) {
    background: ${({ theme }) => theme.colorNhsukWhite};
  }
  td {
    font-size: 16px;
  }
`;

const ContentCell = styled(NHSTable.Cell)`
  &.withoutBottomBorder {
    border-bottom: none;
  }
`;

const ExpanderCell = styled(ContentCell)`
  padding: 0px 0;
  vertical-align: middle;
`;

const ExtraDetailCell = styled(NHSTable.Cell)`
  padding-top: 8px;
  padding-bottom: 16px;
  &.withoutBottomBorder {
    border-bottom: none;
    padding-bottom: 12px;
  }
`;

const StyledHeading = styled.h4`
  margin-bottom: 0;
  font-size: 16px;
`;

interface Item {
  id: string;
}

interface Props<ItemType extends Item> {
  tableHeading?: string;
  icon?: (x: ItemType) => ReactNode;
  columns: Array<{
    heading?: string;
    content: (x: ItemType) => ReactNode;
  }>;
  data: ItemType[];
  extraDetails?: Array<{
    heading?: string;
    content: (x: ItemType) => ReactNode;
  }>;
}

const TableComponent = <ItemType extends Item>({
  tableHeading,
  icon,
  columns,
  data,
  extraDetails,
}: Props<ItemType>) => {
  const [expandedId, setExpandedId] = useState<string | null>(null);
  return (
    <TableContainer>
      <NHSTable.Panel heading={tableHeading}>
        <StyledTable>
          <NHSTable.Head>
            <NHSTable.Row>
              {columns.map((c, i) => (
                <NHSTable.Cell key={i}>{c.heading}</NHSTable.Cell>
              ))}
              {extraDetails && <NHSTable.Cell></NHSTable.Cell>}
            </NHSTable.Row>
          </NHSTable.Head>
          <NHSTable.Body>
            {data.map((x) => {
              const expanded = x.id === expandedId;
              return (
                <React.Fragment key={x.id}>
                  <NHSTable.Row>
                    {columns.map(({ content }, i) => (
                      <ContentCell
                        key={i}
                        className={classNames({
                          withoutBottomBorder: expanded,
                        })}
                      >
                        {i === 0 && icon ? (
                          <IconWrapper>
                            {icon(x)}
                            {content(x)}
                          </IconWrapper>
                        ) : (
                          content(x)
                        )}
                      </ContentCell>
                    ))}
                    {extraDetails && (
                      <ExpanderCell
                        className={classNames({
                          withoutBottomBorder: expanded,
                        })}
                      >
                        <Expander
                          expanded={expanded}
                          onClick={() => setExpandedId(expanded ? null : x.id)}
                        />
                      </ExpanderCell>
                    )}
                  </NHSTable.Row>

                  {extraDetails &&
                    expanded &&
                    extraDetails.map((c, i) => {
                      const withoutBottomBorder = i < extraDetails.length - 1;
                      return (
                        <NHSTable.Row key={i}>
                          <ExtraDetailCell
                            className={classNames({
                              withoutBottomBorder,
                            })}
                          >
                            <StyledHeading>{c.heading || " "}</StyledHeading>
                          </ExtraDetailCell>
                          <ExtraDetailCell
                            className={classNames({
                              withoutBottomBorder,
                            })}
                          >
                            {c.content(x)}
                          </ExtraDetailCell>
                          <ExtraDetailCell
                            className={classNames({
                              withoutBottomBorder,
                            })}
                          />
                        </NHSTable.Row>
                      );
                    })}
                </React.Fragment>
              );
            })}
          </NHSTable.Body>
        </StyledTable>
      </NHSTable.Panel>
    </TableContainer>
  );
};

export { TableComponent as Table };
