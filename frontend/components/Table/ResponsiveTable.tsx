import React, { ReactNode } from "react";

import { MobileList } from "./MobileList";
import { Table } from "./Table";

// TODO: extract these into a components/Table/types.tsx and share them.
interface Item {
  id: string;
}

interface Props<ItemType extends Item> {
  columns: Array<{
    heading?: string;
    content: (x: ItemType) => ReactNode;
  }>;
  extraDetails: Array<{
    heading?: string;
    content: (x: ItemType) => ReactNode;
  }>;
  data: ItemType[];
  tableHeading?: string;
  icon?: (x: ItemType) => ReactNode;
}

export const ResponsiveTable = <ItemType extends Item>({
  tableHeading,
  icon,
  columns,
  data,
  extraDetails,
}: Props<ItemType>) => (
  <>
    <MobileList
      tableHeading={tableHeading}
      icon={icon}
      columns={columns}
      data={data}
      extraDetails={extraDetails}
    />
    <Table
      tableHeading={tableHeading}
      icon={icon}
      columns={columns}
      data={data}
      extraDetails={extraDetails}
    />
  </>
);
