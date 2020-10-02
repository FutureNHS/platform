const fs = require("fs");
const path = require("path");

const createFolderResponse = require("../cypress/fixtures/create-folder-graphql-response.json");
const folderByWorkspaceResponse = require("../cypress/fixtures/folder-by-workspace-graphql-response.json");
const folderResponse = require("../cypress/fixtures/folder-graphql-response.json");
const workspaceResponse = require("../cypress/fixtures/workspace-graphql-response.json");

const workspacesResolver = {
  workspaces: async () => workspaceResponse.data.workspaces,
  workspace: async () => workspaceResponse.data.workspaces[0],
};

const folderResolver = {
  foldersByWorkspace: async () =>
    folderByWorkspaceResponse.data.foldersByWorkspace,
  folder: async () => folderResponse.data.folder,
};

const folderMutation = {
  createFolder: async () => createFolderResponse.data.createFolder,
};

const schema = fs.readFileSync(
  path.join(__dirname, "schema", "../../schema.graphql"),
  "utf8"
);

const federationResolver = {
  _service: async () => ({
    sdl: schema,
  }),
};

module.exports = {
  Query: { ...workspacesResolver, ...folderResolver, ...federationResolver },
  Mutation: { ...folderMutation },
};
