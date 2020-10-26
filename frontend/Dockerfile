FROM node:15

# Setting working directory. All the path will be relative to WORKDIR
WORKDIR /usr/src/
COPY event-models ./event-models
COPY workspace-service ./workspace-service
WORKDIR /usr/src/frontend

# Installing dependencies
COPY frontend/package.json frontend/yarn.lock .
RUN yarn install --frozen-lockfile

# Copying source files
COPY frontend .

# Building app
RUN yarn test
RUN ORIGIN=__FNHS_ORIGIN__ \
    NEXT_PUBLIC_INSTRUMENTATION_KEY=__FNHS_INSTRUMENTATION_KEY__ \
    WORKSPACE_SERVICE_GRAPHQL_ENDPOINT=http://workspace-service.workspace-service/graphql \
    yarn build

# Running the app
COPY frontend/docker-entrypoint.sh /usr/local/bin/
ENTRYPOINT ["docker-entrypoint.sh"]
CMD [ "yarn", "start" ]
