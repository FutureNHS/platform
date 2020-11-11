import React, { ReactNode, useState } from "react";

import styled from "styled-components";

import { Expander } from "../Expander";

const ListItem = styled.li`
  align-items: flex-start;
  display: flex;
  padding-top: 16px;
  padding-bottom: 26px;
  border-bottom: 1px solid ${({ theme }) => theme.colorNhsukGrey4};
`;

const DetailsContainer = styled.div`
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  padding-left: 12px;
  > div {
    padding-bottom: 10px;
  }
  p,
  h4 {
    margin-bottom: 0;

    font-size: 16px;
  }
  p {
    color: ${({ theme }) => theme.colorNhsukGrey1};
  }
  a {
    font-size: 16px;
  }
`;

const List = styled.ul`
  padding-top: 20px;
  padding-left: 12px;
  padding-right: 16px;
  ${({ theme }) => `
    background: ${theme.colorNhsukGrey5};
    @media (min-width: ${theme.mqBreakpoints.tablet}) {
      display: none;
    }
  `}
`;

const Heading = styled.h3`
  background-color: #005eb8;
  color: ${({ theme }) => theme.colorNhsukWhite};
  display: inline-block;
  font-size: 1.5rem;
  line-height: 1.33333;
  padding: 8px 32px;
  position: relative;
  top: 34px;
  ${({ theme }) => `
    @media (min-width: ${theme.mqBreakpoints.tablet}) {
      display: none;
    }
  `}
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

export const MobileList = <ItemType extends Item>({
  tableHeading,
  icon,
  columns,
  data,
  extraDetails,
}: Props<ItemType>) => {
  const [expandedId, setExpandedId] = useState<string | null>(null);

  return (
    <>
      {tableHeading && <Heading>{tableHeading}</Heading>}
      <List>
        {data.map((x) => {
          const expanded = x.id === expandedId;
          return (
            <ListItem key={x.id}>
              {icon && icon(x)}
              <DetailsContainer>
                {columns.map((c, i) => (
                  <React.Fragment key={i}>
                    {c.heading && <h4>{c.heading}</h4>}
                    <div>{c.content(x)}</div>
                  </React.Fragment>
                ))}
                {extraDetails &&
                  expanded &&
                  extraDetails.map((c, i) => (
                    <React.Fragment key={i}>
                      <h4>{c.heading || " "}</h4>
                      <div>{c.content(x)}</div>
                    </React.Fragment>
                  ))}
              </DetailsContainer>
              {extraDetails && (
                <Expander
                  expanded={expanded}
                  onClick={() => setExpandedId(expanded ? null : x.id)}
                />
              )}
            </ListItem>
          );
        })}
      </List>
    </>
  );
};
