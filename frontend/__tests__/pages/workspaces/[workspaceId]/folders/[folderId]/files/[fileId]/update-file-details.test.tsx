import React from "react";

import { ThemeProvider } from "styled-components";

import theme from "../../../../../../../../lib/fixtures/theme.json";
import {
  GetWorkspaceByIdDocument,
  GetFileByIdDocument,
} from "../../../../../../../../lib/generated/graphql";
import { render } from "../../../../../../../../lib/test-helpers/render";
import { mockUrqlClient } from "../../../../../../../../lib/test-helpers/urql";
import UpdateFile from "../../../../../../../../pages/workspaces/[workspaceId]/folders/[folderId]/files/[fileId]/update-file";
import UpdateFileDetails from "../../../../../../../../pages/workspaces/[workspaceId]/folders/[folderId]/files/[fileId]/update-file-details";

describe(UpdateFile, () => {
  const client = mockUrqlClient([
    [
      GetWorkspaceByIdDocument,
      {
        workspace: {
          id: "w1",
          title: "hospital",
          description: "hospital",
        },
      },
    ],
    [
      GetFileByIdDocument,
      {
        file: {
          id: "f1",
          title: "file title",
          description: "file description",
          folder: "file folder",
          fileName: "file name",
          fileType: "file type",
          latestVersion: "file version",
          createdAt: "2020-10-06T17:53:45.089829+00:00",
          modifiedAt: "2020-10-06T17:53:45.089829+00:00",
        },
      },
    ],
  ]);

  test("renders", () => {
    const container = render(
      <ThemeProvider theme={theme}>
        <UpdateFileDetails urqlClient={client} />
      </ThemeProvider>,
      {
        router: {
          query: {
            fileId: "file-1",
            workspaceId: "workspace-1",
            folderId: "folder-1",
          },
        },
      }
    );
    expect(container.asFragment()).toBeTruthy();
  });
});
